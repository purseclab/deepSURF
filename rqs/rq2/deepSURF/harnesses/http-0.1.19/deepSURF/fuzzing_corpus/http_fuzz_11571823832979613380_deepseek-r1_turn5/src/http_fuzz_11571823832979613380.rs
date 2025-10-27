#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use http::*;
use http::header::{HeaderName, HeaderValue};
use http::HttpTryFrom;
use global_data::*;
use std::str::FromStr;
use std::collections::HashMap;

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 2 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        
        let ops = _to_u8(GLOBAL_DATA, 0) as usize % 5;
        let mut map = match _to_u8(GLOBAL_DATA, 1) % 3 {
            0 => HeaderMap::new(),
            1 => HeaderMap::with_capacity(_to_u8(GLOBAL_DATA, 2) as usize % 65),
            2 => {
                let mut headers = Vec::new();
                let max_headers = (GLOBAL_DATA.len().saturating_sub(3)) / 4;
                for i in 0..max_headers {
                    let idx = 3 + i * 4;
                    if idx + 4 > GLOBAL_DATA.len() { break; }
                    let name = _unwrap_result(HeaderName::from_bytes(&GLOBAL_DATA[idx..idx+2]));
                    let value = _unwrap_result(HeaderValue::from_bytes(&GLOBAL_DATA[idx+2..idx+4]));
                    headers.push((name, value));
                }
                HeaderMap::from_iter(headers)
            },
            _ => unreachable!()
        };

        for i in 0..ops {
            let idx = 3 + i * 4;
            match _to_u8(GLOBAL_DATA, idx) % 6 {
                0 => {
                    let name = _unwrap_result(HeaderName::from_bytes(&GLOBAL_DATA[idx..idx+2]));
                    let value = _unwrap_result(HeaderValue::from_bytes(&GLOBAL_DATA[idx+2..idx+4]));
                    map.insert(name, value);
                }
                1 => {
                    let name = _unwrap_result(HeaderName::from_bytes(&GLOBAL_DATA[idx..idx+2]));
                    let value = _unwrap_result(HeaderValue::from_bytes(&GLOBAL_DATA[idx+2..idx+4]));
                    map.append(name, value);
                }
                2 => {
                    let name = _unwrap_result(HeaderName::from_bytes(&GLOBAL_DATA[idx..idx+2]));
                    let _ = map.entry(name).map_err(|_| ()).ok();
                }
                3 => {
                    let name = _unwrap_result(HeaderName::from_bytes(&GLOBAL_DATA[idx..idx+2]));
                    let _ = map.get(name);
                }
                4 => {
                    let name = _unwrap_result(HeaderName::from_bytes(&GLOBAL_DATA[idx..idx+2]));
                    let mut values = map.values();
                    values.size_hint();
                    println!("{:?}", values.next().unwrap_or(&HeaderValue::from_static("")));
                }
                5 => {
                    let mut iter = map.iter();
                    iter.size_hint();
                    if let Some((name, value)) = iter.next() {
                        println!("{:?} {:?}", name, value);
                    }
                }
                _ => unreachable!()
            };
            
            let values = map.values();
            values.size_hint();
        }

        let mut drain = map.drain();
        while let Some((name, value)) = drain.next() {
            println!("Draining {:?} {:?}", name, value);
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