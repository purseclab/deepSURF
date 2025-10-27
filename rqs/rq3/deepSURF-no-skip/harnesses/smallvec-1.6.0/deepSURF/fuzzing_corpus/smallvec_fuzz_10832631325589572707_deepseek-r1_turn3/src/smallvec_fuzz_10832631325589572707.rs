#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::SmallVec;
use global_data::*;
use std::fmt::Debug;

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 1024 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let mut ops = _to_u8(GLOBAL_DATA, 0) % 64;
        let mut sv1 = SmallVec::<[u8; 64]>::new();
        let mut sv2 = SmallVec::<[u8; 64]>::with_capacity(32);

        for idx in 1..ops as usize * 3 {
            match _to_u8(GLOBAL_DATA, idx) % 21 {
                0 => {
                    let cap = _to_usize(GLOBAL_DATA, idx + 10);
                    sv1 = SmallVec::with_capacity(cap % 65);
                }
                1 => {
                    let buf = array_from_data(GLOBAL_DATA, idx * 5);
                    sv1 = SmallVec::from_buf_and_len(buf, _to_usize(GLOBAL_DATA, idx + 42) % 65);
                }
                2 => {
                    sv1.push(_to_u8(GLOBAL_DATA, idx + 8));
                }
                3 => {
                    if !sv1.is_empty() {
                        sv1.swap_remove(_to_usize(GLOBAL_DATA, idx + 7) % sv1.len());
                    }
                }
                4 => {
                    let slice = sv1.as_ref();
                    println!("{:?}", slice);
                    let cap = sv1.capacity();
                    sv1.reserve(cap.wrapping_add(_to_usize(GLOBAL_DATA, idx + 12)));
                }
                5 => {
                    sv2.extend_from_slice(sv1.as_ref());
                }
                6 => {
                    sv1.insert_from_slice(
                        _to_usize(GLOBAL_DATA, idx + 3) % (sv1.len() + 1),
                        &GLOBAL_DATA[_to_usize(GLOBAL_DATA, idx + 8) % GLOBAL_DATA.len()..]
                    );
                }
                7 => {
                    let val = _to_u8(GLOBAL_DATA, idx + 6);
                    sv1.resize(_to_usize(GLOBAL_DATA, idx + 9) % 65, val);
                }
                8 => {
                    let _ = sv1.drain(_to_usize(GLOBAL_DATA, idx + 4) % (sv1.len() + 1)..);
                }
                9 => {
                    let slice_ref = sv1.as_ref();
                    let mut_slice = sv2.as_mut_slice();
                    if !slice_ref.is_empty() && !mut_slice.is_empty() {
                        println!("{} {}", slice_ref[0], mut_slice[0]);
                    }
                }
                10 => {
                    let cloned = sv1.clone();
                    sv2.extend(cloned);
                }
                11 => {
                    let elem = _to_u8(GLOBAL_DATA, idx + 2);
                    sv1 = SmallVec::from_elem(elem, _to_usize(GLOBAL_DATA, idx + 10) % 65);
                }
                12 => {
                    if let Ok(arr) = sv1.clone().into_inner() {
                        let mut vec = Vec::from(arr);
                        vec.push(_to_u8(GLOBAL_DATA, idx));
                        sv1 = SmallVec::from_vec(vec);
                    }
                }
                13 => {
                    sv1.shrink_to_fit();
                }
                14 => {
                    let _ = sv1.pop();
                }
                15 => {
                    sv1.append(&mut sv2);
                }
                16 => {
                    let _val = sv1.as_ptr();
                }
                17 => {
                    let _val = sv1.as_mut_ptr();
                }
                18 => {
                    if !sv1.is_empty() {
                        let pos = _to_usize(GLOBAL_DATA, idx) % sv1.len();
                        sv1.insert(pos, _to_u8(GLOBAL_DATA, idx + 5));
                    }
                }
                19 => {
                    let _ = sv1.as_mut_slice().reverse();
                }
                _ => {
                    let _ordering = sv1.partial_cmp(&sv2);
                    let _fmt = format!("{:?}", sv1);
                }
            }
        }

        let final_ref = sv1.as_ref();
        sv2.extend(final_ref.iter().copied());
        println!("Final: {:?}", final_ref);
    });
}

fn array_from_data(data: &[u8], start: usize) -> [u8; 64] {
    let mut arr = [0u8; 64];
    for i in 0..64 {
        arr[i] = data.get(start + i).copied().unwrap_or(0);
    }
    arr
}

fn _to_u8(data:&[u8], index:usize)->u8 {
    data[index]
}

fn _to_u16(data:&[u8], index:usize)->u16 {
    let data0 = _to_u8(data, index) as u16;
    let data1 = _to_u8(data, index+1) as u16;
    data0 << 8 | data1
}

fn _to_u32(data:&[u8], index:usize)->u32 {
    let data0 = _to_u16(data, index) as u32;
    let data1 = _to_u16(data, index+2) as u32;
    data0 << 16 | data1
}

fn _to_u64(data:&[u8], index:usize)->u64 {
    let data0 = _to_u32(data, index) as u64;
    let data1 = _to_u32(data, index+4) as u64;
    data0 << 32 | data1
}

fn _to_u128(data:&[u8], index:usize)->u128 {
    let data0 = _to_u64(data, index) as u128;
    let data1 = _to_u64(data, index+8) as u128;
    data0 << 64 | data1
}

fn _to_usize(data:&[u8], index:usize)->usize {
    _to_u64(data, index) as usize
}

fn _to_i8(data:&[u8], index:usize)->i8 {    
    data[index] as i8
}

fn _to_i16(data:&[u8], index:usize)->i16 {
    let data0 = _to_i8(data, index) as i16;
    let data1 = _to_i8(data, index+1) as i16;
    data0 << 8 | data1
}

fn _to_i32(data:&[u8], index:usize)->i32 {
    let data0 = _to_i16(data, index) as i32;
    let data1 = _to_i16(data, index+2) as i32;
    data0 << 16 | data1
}

fn _to_i64(data:&[u8], index:usize)->i64 {
    let data0 = _to_i32(data, index) as i64;
    let data1 = _to_i32(data, index+4) as i64;
    data0 << 32 | data1
}

fn _to_i128(data:&[u8], index:usize)->i128 {
    let data0 = _to_i64(data, index) as i128;
    let data1 = _to_i64(data, index+8) as i128;
    data0 << 64 | data1
}

fn _to_isize(data:&[u8], index:usize)->isize {
    _to_i64(data, index) as isize
}

fn _to_f32(data:&[u8], index: usize) -> f32 {
    let data_slice = &data[index..index+4];
    use std::convert::TryInto;
    let data_array:[u8;4] = data_slice.try_into().expect("slice with incorrect length");
    f32::from_le_bytes(data_array)
}

fn _to_f64(data:&[u8], index: usize) -> f64 {
    let data_slice = &data[index..index+8];
    use std::convert::TryInto;
    let data_array:[u8;8] = data_slice.try_into().expect("slice with incorrect length");
    f64::from_le_bytes(data_array)
}

fn _to_char(data:&[u8], index: usize)->char {
    let char_value = _to_u32(data,index);
    match char::from_u32(char_value) {
        Some(c)=>c,
        None=>{
            std::process::exit(0);
        }
    }
}

fn _to_bool(data:&[u8], index: usize)->bool {
    let bool_value = _to_u8(data, index);
    if bool_value %2 == 0 {
        true
    } else {
        false
    }
}

fn _to_str(data:&[u8], start_index: usize, end_index: usize)->&str {
    let data_slice = &data[start_index..end_index];
    use std::str;
    match str::from_utf8(data_slice) {
        Ok(s)=>s,
        Err(_)=>{
            std::process::exit(0);
        }
    }
}

fn _unwrap_option<T>(opt: Option<T>) -> T {
    match opt {
        Some(_t) => _t,
        None => {
            std::process::exit(0);
        }
    }
}

fn _unwrap_result<T, E>(_res: std::result::Result<T, E>) -> T {
    match _res {
        Ok(_t) => _t,
        Err(_) => {
            std::process::exit(0);
        },
    }
}