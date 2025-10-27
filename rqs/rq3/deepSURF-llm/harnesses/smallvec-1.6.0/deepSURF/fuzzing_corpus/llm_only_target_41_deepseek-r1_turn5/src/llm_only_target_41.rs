#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 105 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let mut vec: SmallVec<[i32; 16]> = SmallVec::new();
        let ops = _to_usize(GLOBAL_DATA, 0) % 8;
        let mut data_idx = 1;

        for _ in 0..ops {
            match _to_u8(GLOBAL_DATA, data_idx) % 6 {
                0 => {
                    let val = _to_i32(GLOBAL_DATA, data_idx);
                    vec.push(val);
                    data_idx += 4;
                }
                1 => {
                    if !vec.is_empty() {
                        let idx = _to_usize(GLOBAL_DATA, data_idx) % vec.len();
                        println!("{:?}", *vec.index(idx));
                        vec.swap_remove(idx);
                        data_idx += 8;
                    }
                }
                2 => {
                    let new_len = _to_usize(GLOBAL_DATA, data_idx);
                    vec.truncate(new_len);
                    data_idx += 8;
                }
                3 => {
                    vec.dedup();
                    let _ = vec.as_ptr();
                }
                4 => {
                    vec.dedup_by(|a, b| {
                        let choice = _to_bool(GLOBAL_DATA, data_idx);
                        data_idx += 1;
                        if choice { a == b } else { false }
                    });
                }
                5 => {
                    let cap = _to_usize(GLOBAL_DATA, data_idx);
                    vec.reserve(cap);
                    data_idx += 8;
                }
                _ => vec.clear(),
            }

            if data_idx + 16 >= GLOBAL_DATA.len() {
                data_idx = 1;
            }
        }

        let mut alt_vec: SmallVec<[i32; 16]> = SmallVec::from_vec(vec![
            _to_i32(GLOBAL_DATA, 95),
            _to_i32(GLOBAL_DATA, 99),
            _to_i32(GLOBAL_DATA, 103)
        ]);
        alt_vec.insert_from_slice(1, &vec.as_slice());
        alt_vec.dedup_by_key(|x| (*x) % 2);

        let mut hybrid = SmallVec::<[String; 24]>::new();
        for chunk in GLOBAL_DATA.chunks(8) {
            if let Ok(s) = std::str::from_utf8(chunk) {
                hybrid.push(s.to_string());
            }
        }
        hybrid.dedup();
        println!("{:?}", hybrid.as_slice());

        let mut dyn_vec = SmallVec::<[i128; 32]>::from_slice(&[
            _to_i128(GLOBAL_DATA, 15), 
            _to_i128(GLOBAL_DATA, 31)
        ]);
        let _ = dyn_vec.as_mut_slice();
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