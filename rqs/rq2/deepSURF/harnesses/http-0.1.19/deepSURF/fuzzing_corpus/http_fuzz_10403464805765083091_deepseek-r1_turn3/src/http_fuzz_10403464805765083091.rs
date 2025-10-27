#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use http::*;
use http::header::HeaderName;
use http::uri::{Scheme, Authority, PathAndQuery};
use http::HttpTryFrom;
use global_data::*;
use std::str::FromStr;
use bytes::*;

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 32 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let operations = _to_usize(&global_data.first_half, 0) % 8 + 2;

        let mut headers = header::HeaderMap::new();
        let mut uri_builder = uri::Builder::new();
        let mut req_builder = request::Request::builder();
        let mut res_builder = response::Response::builder();

        for i in 0..operations {
            match _to_usize(&global_data.first_half, i*4) % 6 {
                0 => {
                    let name_idx = _to_usize(&global_data.first_half, i*4+1) % 128;
                    let value_idx = _to_usize(&global_data.first_half, i*4+2) % 128;
                    
                    let header_name = match name_idx % 2 {
                        0 => HeaderName::from_static(_to_str(&global_data.second_half, name_idx, name_idx + 8)),
                        _ => HeaderName::from_bytes(&global_data.second_half[name_idx..name_idx+8]).unwrap()
                    };
                    
                    let header_value = HeaderValue::from_bytes(&global_data.second_half[value_idx..value_idx+16]).unwrap();
                    headers.insert(header_name, header_value);
                }
                1 => {
                    let scheme = Scheme::from_str(_to_str(&global_data.second_half, 16, 24)).unwrap();
                    uri_builder.scheme(scheme);
                }
                2 => {
                    let authority = Authority::from_static(_to_str(&global_data.second_half, 24, 32));
                    uri_builder.authority(authority);
                }
                3 => {
                    let path_data = Bytes::from(&global_data.second_half[32..64]);
                    let path = <PathAndQuery as HttpTryFrom<_>>::try_from(path_data);
                    uri_builder.path_and_query(path.unwrap());
                }
                4 => {
                    let uri = uri_builder.build().unwrap();
                    req_builder.uri(uri);
                }
                5 => {
                    let method = Method::from_bytes(&global_data.second_half[64..72]).unwrap();
                    req_builder.method(method);
                }
                _ => {}
            }
        }

        let path_bytes = Bytes::from(&global_data.second_half[0..16]);
        let path = <PathAndQuery as HttpTryFrom<_>>::try_from(path_bytes);
        uri_builder.path_and_query(path.unwrap());
        
        if let Ok(uri) = uri_builder.build() {
            req_builder.uri(uri);
            res_builder.status(StatusCode::from_u16(_to_u16(&global_data.first_half, 8)).unwrap());
            
            if headers.len() > 0 {
                if let Some(h) = req_builder.headers_mut() {
                    *h = headers.clone();
                }
                if let Some(h) = res_builder.headers_mut() {
                    *h = headers.clone();
                }
            }
        }

        let val = headers.get("X-Test").map(|v| v.to_str().unwrap());
        println!("Header value: {:?}", val);
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