#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::SmallVec;
use global_data::*;

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 256 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        
        let num_ops = _to_u8(GLOBAL_DATA, 0) % 8 + 1;
        let mut offset = 1;
        
        for _ in 0..num_ops {
            let op = _to_u8(GLOBAL_DATA, offset) % 7;
            offset += 1;
            
            match op {
                0 => {
                    let sv = SmallVec::<[u8; 32]>::new();
                    println!("{:?}", sv.as_slice());
                    let _ = sv.len();
                }
                1 => {
                    let cap = _to_usize(GLOBAL_DATA, offset);
                    offset += 2;
                    let mut sv = SmallVec::<[u8; 32]>::with_capacity(cap);
                    let _ = sv.capacity();
                    println!("{:?}", sv.as_mut_slice());
                    sv.push(_to_u8(GLOBAL_DATA, offset));
                    offset += 1;
                }
                2 => {
                    let slice_start = offset % 64;
                    let slice_len = _to_usize(GLOBAL_DATA, offset + 1) % 65;
                    let slice = &GLOBAL_DATA[slice_start..(slice_start + slice_len).min(GLOBAL_DATA.len())];
                    let sv = SmallVec::<[u8; 32]>::from_slice(slice);
                    let _ = sv.len();
                    println!("{:?}", sv.as_slice());
                }
                3 => {
                    let elem = _to_u8(GLOBAL_DATA, offset);
                    let count = _to_usize(GLOBAL_DATA, offset + 1) % 65;
                    offset += 2;
                    let mut sv = SmallVec::<[u8; 32]>::from_elem(elem, count);
                    sv.truncate(_to_usize(GLOBAL_DATA, offset));
                    println!("{:?}", sv.as_ptr());
                }
                4 => {
                    let slice_start = offset % 64;
                    let slice_len = _to_usize(GLOBAL_DATA, offset + 1) % 65;
                    offset = (offset + slice_len + 2).min(GLOBAL_DATA.len());
                    let slice = &GLOBAL_DATA[slice_start..(slice_start + slice_len).min(GLOBAL_DATA.len())];
                    let mut sv = SmallVec::<[u8; 32]>::from_slice(slice);
                    let new_len = _to_usize(GLOBAL_DATA, offset);
                    sv.truncate(new_len);
                    let _ = sv.len();
                }
                5 => {
                    let mut sv1 = SmallVec::<[u8; 32]>::new();
                    let mut sv2 = SmallVec::<[u8; 32]>::new();
                    for _ in 0..3 {
                        sv1.push(_to_u8(GLOBAL_DATA, offset));
                        sv2.push(_to_u8(GLOBAL_DATA, offset + 1));
                        offset += 2;
                    }
                    sv1.append(&mut sv2);
                    println!("{:?}", sv1.capacity());
                }
                6 => {
                    let slice_start = offset % 64;
                    let slice_len = _to_usize(GLOBAL_DATA, offset + 1) % 65;
                    let slice = &GLOBAL_DATA[slice_start..(slice_start + slice_len).min(GLOBAL_DATA.len())];
                    let mut sv: SmallVec<[u8; 32]> = slice.iter().cloned().collect();
                    let _ = sv.insert_many(0, slice.iter().cloned());
                    println!("{:?}", sv.drain(..));
                }
                _ => unreachable!()
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