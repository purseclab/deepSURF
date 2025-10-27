#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use ordnung::*;
use global_data::*;
use std::ops::{Deref, DerefMut};

#[derive(Debug)]
struct CustomType0(String);

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 4096 {return;}
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let mut idx = 0;

        let constructor_choice = _to_u8(GLOBAL_DATA, idx) % 3;
        idx += 1;
        let mut cv = match constructor_choice {
            0 => ordnung::compact::Vec::new(),
            1 => ordnung::compact::Vec::with_capacity(_to_usize(GLOBAL_DATA, idx)),
            2 => {
                let mut temp = Vec::new();
                for _ in 0..(_to_u8(GLOBAL_DATA, idx) % 32) {
                    let l = _to_u8(GLOBAL_DATA, idx+1) % 17;
                    let s = _to_str(GLOBAL_DATA, idx+2, idx+2+l as usize);
                    temp.push(CustomType0(s.to_string()));
                    idx += 2 + l as usize;
                }
                ordnung::compact::Vec::from(temp)
            }
            _ => unreachable!()
        };

        let mut map = ordnung::Map::with_capacity(_to_usize(GLOBAL_DATA, idx));
        idx += 8;

        let ops = _to_u8(GLOBAL_DATA, idx) % 32;
        idx += 1;
        for _ in 0..ops {
            let op_type = _to_u8(GLOBAL_DATA, idx) % 5;
            idx += 1;

            match op_type {
                0 => {
                    let kl = _to_u8(GLOBAL_DATA, idx) % 17;
                    let ks = _to_str(GLOBAL_DATA, idx+1, idx+1+kl as usize);
                    let vl = _to_u8(GLOBAL_DATA, idx+1+kl as usize) % 17;
                    let vs = _to_str(GLOBAL_DATA, idx+2+kl as usize, idx+2+kl as usize+vl as usize);
                    map.insert(ks.to_string(), vs.to_string());
                    idx += 2 + kl as usize + vl as usize;
                }
                1 => {
                    let kl = _to_u8(GLOBAL_DATA, idx) % 17;
                    let ks = _to_str(GLOBAL_DATA, idx+1, idx+1+kl as usize);
                    let _ = map.get(&ks.to_string()).map(|v| println!("{:?}", v));
                    idx += 1 + kl as usize;
                }
                2 => {
                    cv.push(CustomType0(String::from("payload")));
                }
                3 => {
                    for (k, v) in map.iter() {
                        println!("{:?} => {:?}", k, v);
                    }
                }
                4 => {
                    if let Some(e) = cv.pop() {
                        println!("{:?}", e);
                    }
                }
                _ => ()
            }
        }

        let _iter = cv.into_iter();
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