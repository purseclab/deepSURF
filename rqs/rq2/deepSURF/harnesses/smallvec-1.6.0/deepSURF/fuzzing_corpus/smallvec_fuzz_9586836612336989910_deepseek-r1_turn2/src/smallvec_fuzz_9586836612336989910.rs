#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 4096 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let mut vecs = Vec::new();
        let op_count = _to_u8(GLOBAL_DATA, 0) % 10;

        for i in 0..op_count {
            match _to_u8(GLOBAL_DATA, 1 + i as usize) % 6 {
                0 => {
                    let capacity = _to_usize(GLOBAL_DATA, 100 + (i as usize) * 8);
                    vecs.push(SmallVec::<[String; 64]>::with_capacity(capacity));
                }
                1 => {
                    let elem = String::from(_to_str(GLOBAL_DATA, 200 + (i as usize) * 16, 250 + (i as usize) * 16));
                    vecs.push(SmallVec::from_elem(elem, _to_usize(GLOBAL_DATA, 300 + (i as usize) * 8) % 65));
                }
                2 => {
                    let slice_len = _to_usize(GLOBAL_DATA, 400 + (i as usize) * 8) % 65;
                    let elements = (0..slice_len).map(|j| {
                        String::from(_to_str(GLOBAL_DATA, 500 + j * 10, 510 + j * 10))
                    }).collect::<Vec<_>>();
                    vecs.push(SmallVec::from(elements));
                }
                3 => {
                    vecs.push(SmallVec::<[String; 64]>::new());
                }
                4 => {
                    let mut vec = SmallVec::<[String; 64]>::new();
                    let push_count = _to_usize(GLOBAL_DATA, 600 + (i as usize) * 8) % 65;
                    for j in 0..push_count {
                        vec.push(String::from(_to_str(GLOBAL_DATA, 700 + j * 10, 710 + j * 10)));
                    }
                    vecs.push(vec);
                }
                _ => {
                    let offset = _to_usize(GLOBAL_DATA, 800 + (i as usize) * 8);
                    let len = _to_usize(GLOBAL_DATA, 808 + (i as usize) * 8);
                    vecs.push(SmallVec::from_buf_and_len([(); 64].map(|_| String::new()), len));
                }
            };
        }

        for vec in &mut vecs {
            let ops = _to_u8(GLOBAL_DATA, 900) % 8;
            for i in 0..ops {
                match _to_u8(GLOBAL_DATA, 901 + i as usize) % 6 {
                    0 => {
                        let new_cap = _to_usize(GLOBAL_DATA, 1000 + (i as usize) * 16);
                        vec.grow(new_cap);
                    }
                    1 => vec.truncate(_to_usize(GLOBAL_DATA, 1100 + (i as usize) * 16)),
                    2 => {
                        let idx = _to_usize(GLOBAL_DATA, 1200 + (i as usize) * 16);
                        let s = String::from(_to_str(GLOBAL_DATA, 1300 + (i as usize) * 16, 1350 + (i as usize) * 16));
                        if !vec.is_empty() {
                            vec.insert(idx % vec.len(), s);
                        }
                    }
                    3 => {
                        _ = vec.pop();
                    }
                    4 => {
                        let s = String::from(_to_str(GLOBAL_DATA, 1400 + (i as usize) * 16, 1450 + (i as usize) * 16));
                        vec.push(s);
                    }
                    _ => {
                        let new_cap = _to_usize(GLOBAL_DATA, 1500 + (i as usize) * 16);
                        let _capacity_before = vec.capacity();
                        vec.grow(new_cap);
                        let _ = vec.as_slice().get(0);
                        if let Some(last) = vec.as_mut_slice().last_mut() {
                            println!("{:?}", *last);
                        }
                    }
                }
            }
        }

        let target_idx = _to_usize(GLOBAL_DATA, 2000) % vecs.len();
        let new_cap = _to_usize(GLOBAL_DATA, 2008);
        if let Some(v) = vecs.get_mut(target_idx) {
            v.grow(new_cap);
            let _ = v.as_ptr();
            let _ = v.capacity();
            let slice = v.as_slice();
            if !slice.is_empty() {
                println!("{:?}", &slice[0]);
            }
        }
    });
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