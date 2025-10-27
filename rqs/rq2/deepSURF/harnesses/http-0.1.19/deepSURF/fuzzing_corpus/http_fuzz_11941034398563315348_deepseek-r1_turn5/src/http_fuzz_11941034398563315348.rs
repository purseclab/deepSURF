#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use http::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};
use bytes::*;

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 2 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let mut header_map = http::header::HeaderMap::new();
        let mut builder = http::request::Request::builder();
        let bytes_selector = _to_u8(GLOBAL_DATA, 0) % 3;

        let bytes = match bytes_selector {
            0 => bytes::Bytes::new(),
            1 => bytes::Bytes::from_static(&[b'X'; 16]),
            _ => bytes::Bytes::from_static(b"test"),
        };

        for _ in 0..(_to_u8(GLOBAL_DATA, 1) % 4) {
            let op = _to_u8(GLOBAL_DATA, 2) % 5;
            
            match op {
                0 => {
                    let header_val = <http::header::HeaderValue as http::HttpTryFrom<_>>::try_from(bytes.clone());
                    let _ = _unwrap_result(header_val);
                }
                1 => {
                    let name = _unwrap_result(http::header::HeaderName::from_bytes(&GLOBAL_DATA[3..7]));
                    let value = _unwrap_result(http::header::HeaderValue::from_bytes(&GLOBAL_DATA[8..12]));
                    header_map.insert(name, value);
                }
                2 => {
                    let uri = _unwrap_result(http::uri::Uri::from_shared(bytes.clone()));
                    builder.uri(uri);
                }
                3 => {
                    let version = match _to_u8(GLOBAL_DATA, 13) % 2 {
                        0 => http::version::Version::HTTP_11,
                        _ => http::version::Version::HTTP_2,
                    };
                    builder.version(version);
                }
                _ => {
                    let response = _unwrap_result(http::response::Response::builder()
                        .status(_to_u16(GLOBAL_DATA, 14))
                        .body(bytes.clone()));
                    println!("{:?}", response.version());
                }
            };
        }

        let mut uri_builder = http::uri::Builder::new();
        let scheme = _unwrap_result(http::uri::Scheme::from_shared(bytes.clone()));
        uri_builder.scheme(scheme);
        let uri = _unwrap_result(uri_builder.build());

        let authority = _unwrap_result(http::uri::Authority::from_shared(bytes.clone()));
        println!("{}", authority.host());
        let path_query = _unwrap_result(http::uri::PathAndQuery::from_shared(bytes.clone()));
        println!("{}", path_query.path());

        let header_value = <http::header::HeaderValue as http::HttpTryFrom<_>>::try_from(bytes.clone());
        let _ = _unwrap_result(header_value);
        println!("{:?}", header_map.get(http::header::CONTENT_TYPE));

        let mut request_builder = http::request::Request::builder();
        for idx in 0..(_to_u8(GLOBAL_DATA, 15) % 3) {
            let name = _unwrap_result(http::header::HeaderName::from_str(&format!("Header-{}", idx)));
            let value = _unwrap_result(http::header::HeaderValue::from_str(&format!("Value{}", _to_u16(GLOBAL_DATA, 16 + idx as usize))));
            request_builder.header(name, value);
        }

        let status = _unwrap_result(http::status::StatusCode::from_u16(_to_u16(GLOBAL_DATA, 20)));
        let response = _unwrap_result(http::response::Response::builder().status(status).body(bytes.clone()));
        println!("{}", response.status());
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