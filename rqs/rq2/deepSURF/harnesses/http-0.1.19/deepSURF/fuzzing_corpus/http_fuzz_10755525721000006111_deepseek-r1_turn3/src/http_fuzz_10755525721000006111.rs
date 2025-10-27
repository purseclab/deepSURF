#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use http::*;
use global_data::*;
use std::str::FromStr;
use http::header::{HeaderName, HeaderValue};
use http::method::Method;
use http::uri::Uri;
use http::status::StatusCode;

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 256 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let mut idx = 0;
        let op_count = _to_u8(GLOBAL_DATA, idx) % 4;
        idx += 1;

        let mut header_map = header::HeaderMap::new();
        let mut request_builder = Request::builder();
        let mut response_builder = Response::builder();

        for _ in 0..op_count {
            let selector = _to_u8(GLOBAL_DATA, idx) % 5;
            idx += 1;

            match selector {
                0 => {
                    let name_len = _to_u8(GLOBAL_DATA, idx) % 17;
                    idx += 1;
                    let name = _to_str(GLOBAL_DATA, idx, idx + name_len as usize);
                    idx += name_len as usize;
                    
                    let value_len = _to_u8(GLOBAL_DATA, idx) % 17;
                    idx += 1;
                    let value_str = _to_str(GLOBAL_DATA, idx, idx + value_len as usize);
                    idx += value_len as usize;
                    
                    let header_name = _unwrap_result(HeaderName::from_bytes(name.as_bytes()));
                    let header_value = _unwrap_result(HeaderValue::from_str(value_str));
                    header_map.append(header_name.clone(), header_value.clone());
                }
                1 => {
                    let uri_len = _to_u8(GLOBAL_DATA, idx) % 32;
                    idx += 1;
                    let uri_str = _to_str(GLOBAL_DATA, idx, idx + uri_len as usize);
                    idx += uri_len as usize;
                    
                    let uri = _unwrap_result(Uri::from_str(uri_str));
                    request_builder.uri(uri);
                }
                2 => {
                    let method_len = _to_u8(GLOBAL_DATA, idx) % 8;
                    idx += 1;
                    let method_bytes = &GLOBAL_DATA[idx..idx + method_len as usize];
                    idx += method_len as usize;
                    
                    let method = _unwrap_result(Method::from_bytes(method_bytes));
                    request_builder.method(method);
                }
                3 => {
                    let status_bytes = &GLOBAL_DATA[idx..idx + 3];
                    idx += 3;
                    let status = _unwrap_result(StatusCode::from_bytes(status_bytes));
                    response_builder.status(status);
                }
                4 => {
                    let name_len = _to_u8(GLOBAL_DATA, idx) % 17;
                    idx += 1;
                    let name = _to_str(GLOBAL_DATA, idx, idx + name_len as usize);
                    idx += name_len as usize;
                    
                    let name_key = _unwrap_result(HeaderName::from_bytes(name.as_bytes()));
                    if let Some(value) = header_map.get(&name_key) {
                        let cmp_len = _to_u8(GLOBAL_DATA, idx) % 17;
                        idx += 1;
                        let cmp_str = _to_str(GLOBAL_DATA, idx, idx + cmp_len as usize);
                        value.eq(cmp_str);
                    }
                }
                _ => unreachable!()
            }
        }

        let _request = request_builder.body(()).ok();
        let _response = response_builder.body(()).ok();

        if !header_map.is_empty() {
            let cmp_name = HeaderName::from_static("test");
            let header_value = HeaderValue::from_static("value");
            if let Some(entry) = header_map.get(&cmp_name) {
                entry.eq(&header_value);
            }
            
            for (name, value) in header_map.iter() {
                let _ = value.to_str();
                name.eq(name.as_str());
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