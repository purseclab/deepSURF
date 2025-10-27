#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use http::*;
use global_data::*;
use std::str::FromStr;

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 1024 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        
        let op_count = _to_usize(global_data.first_half, 0) % 8 + 2;
        let mut map = header::HeaderMap::new();
        
        for i in 0..op_count {
            let selector = _to_u8(global_data.second_half, i as usize) % 7;
            match selector {
                0 => {
                    let name_start = 10 * i;
                    let name_len = _to_u8(global_data.first_half, name_start) % 50 + 1;
                    let val_start = name_start + name_len as usize + 1;
                    let val_len = _to_u8(global_data.first_half, val_start) % 50 + 1;
                    
                    if let (Ok(name), Ok(value)) = (
                        header::HeaderName::from_bytes(&global_data.first_half[name_start..name_start + name_len as usize]),
                        header::HeaderValue::from_bytes(&global_data.first_half[val_start..val_start + val_len as usize])
                    ) {
                        map.append(name, value);
                    }
                },
                1 => {
                    let iter_map: std::collections::HashMap<_, _> = map.iter()
                        .map(|(k, v)| (k.clone(), v.to_str().unwrap().to_owned()))
                        .collect();
                    println!("{:?}", iter_map);
                },
                2 => {
                    let drain = map.drain();
                    for item in drain {
                        println!("{:?}", item);
                    }
                },
                3 => {
                    let name_start = 20 * i;
                    let name_len = _to_u8(global_data.second_half, name_start) % 50 + 1;
                    if let Ok(name) = header::HeaderName::from_bytes(&global_data.second_half[name_start..name_start + name_len as usize]) {
                        map.remove(name);
                    }
                },
                4 => {
                    let mut entries = Vec::new();
                    for j in 0..3 {
                        let offset = 30 * i + 10 * j;
                        let k_len = _to_u8(global_data.first_half, offset) % 20 + 1;
                        let v_len = _to_u8(global_data.first_half, offset + k_len as usize) % 20 + 1;
                        
                        if let (Ok(k), Ok(v)) = (
                            header::HeaderName::from_bytes(&global_data.first_half[offset..offset + k_len as usize]),
                            header::HeaderValue::from_bytes(&global_data.first_half[offset + k_len as usize..offset + k_len as usize + v_len as usize])
                        ) {
                            entries.push((k, v));
                        }
                    }
                    map.extend(entries);
                },
                5 => {
                    let uri_data = &global_data.second_half[100..200];
                    if let Ok(s) = std::str::from_utf8(uri_data) {
                        if let Ok(uri) = http::Uri::from_str(s) {
                            let mut builder = http::request::Request::builder();
                            builder.uri(uri);
                            builder.method(http::Method::GET);
                            println!("{:?}", builder);
                        }
                    }
                },
                6 => {
                    if let Ok(status) = status::StatusCode::from_u16(_to_u16(global_data.second_half, i * 2)) {
                        if let Ok(resp) = response::Response::builder().status(status).body(()) {
                            println!("{:?}", resp.version());
                        }
                    }
                },
                _ => panic!("INTENTIONAL PANIC!"),
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