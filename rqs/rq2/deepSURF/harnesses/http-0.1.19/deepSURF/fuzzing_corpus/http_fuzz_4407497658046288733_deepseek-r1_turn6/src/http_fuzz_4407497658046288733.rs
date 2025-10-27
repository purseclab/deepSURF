#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use http::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 128 {return;}
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        
        let op_count = _to_u8(GLOBAL_DATA, 0) % 8;
        let mut offset = 1;
        
        let mut header_map = header::HeaderMap::with_capacity(32);
        let mut uri_builder = uri::Builder::new();
        let mut request_builder = request::Request::builder();
        
        for _ in 0..op_count {
            let op_type = _to_u8(GLOBAL_DATA, offset) % 5;
            offset = (offset + 1) % 64;
            
            match op_type {
                0 => {
                    let name_len = _to_u8(GLOBAL_DATA, offset) % 32;
                    offset = (offset + 1) % 64;
                    let name_data = _to_str(GLOBAL_DATA, offset as usize, offset as usize + name_len as usize);
                    if let Ok(name) = header::HeaderName::from_bytes(name_data.as_bytes()) {
                        let value_len = _to_u8(GLOBAL_DATA, offset + name_len as usize) % 32;
                        let value_data = _to_str(GLOBAL_DATA, (offset + name_len as usize + 1) as usize, (offset + name_len as usize + 1 + value_len as usize) as usize);
                        if let Ok(value) = header::HeaderValue::from_str(value_data) {
                            header_map.append(&name, value);
                        }
                    }
                },
                1 => {
                    let scheme_data = _to_str(GLOBAL_DATA, offset, offset + 8);
                    if let Ok(scheme) = uri::Scheme::from_str(scheme_data) {
                        uri_builder.scheme(scheme.clone());
                        println!("{:?}", scheme.as_str());
                    }
                    offset = (offset + 8) % 64;
                },
                2 => {
                    let auth_data = _to_str(GLOBAL_DATA, offset, offset + 16);
                    uri_builder.authority(auth_data);
                    offset = (offset + 16) % 64;
                },
                3 => {
                    let path_data = _to_str(GLOBAL_DATA, offset, offset + 24);
                    uri_builder.path_and_query(path_data);
                    offset = (offset + 24) % 64;
                },
                4 => {
                    let method_data = _to_str(GLOBAL_DATA, offset, offset + 8);
                    if let Ok(method) = method::Method::from_bytes(method_data.as_bytes()) {
                        request_builder.method(method.clone());
                        println!("{:?}", method.as_str());
                    }
                    offset = (offset + 8) % 64;
                },
                _ => {}
            }
        }
        
        let final_uri = _unwrap_result(uri_builder.build());
        let final_request = request_builder.uri(final_uri).body(()).ok();
        
        let t_0 = _to_u8(GLOBAL_DATA, offset) % 32;
        let t_1 = _to_str(GLOBAL_DATA, offset + 1, offset + 1 + t_0 as usize);
        let t_2 = String::from(t_1);
        let t_3 = &t_2;
        let _ = header::HeaderValue::from_str(t_3);
        
        let mut response_builder = response::Response::builder();
        let status_code = _to_u16(GLOBAL_DATA, offset + t_0 as usize + 2) % 500 + 100;
        if let Ok(status) = status::StatusCode::from_u16(status_code) {
            response_builder.status(status);
        }
        
        if let Some(host_header) = header_map.get("host") {
            println!("Host header value: {:?}", host_header.to_str());
        }
        
        let response = response_builder.body(()).ok();
        println!("{:?}", header_map.get("content-type"));
        println!("{:?}", final_request.as_ref().map(|r| r.method()));
        println!("{:?}", response.as_ref().map(|r| r.status()));
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