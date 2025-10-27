#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use http::header::{HeaderName, HeaderMap, HeaderValue};
use http::uri::{Authority, Scheme, Uri, PathAndQuery};
use http::{Request, Response};
use http::status::StatusCode;
use http::method::Method;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 1024 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let iter_count = _to_u8(GLOBAL_DATA, 0) % 8;
        let header_name_data = &GLOBAL_DATA[1..33];
        
        let mut headers = HeaderMap::with_capacity(12);
        let mut req_builder = Request::builder();
        let mut res_builder = Response::builder();

        for i in 0..iter_count {
            let selector = _to_u8(GLOBAL_DATA, 34 + i as usize) % 5;
            
            match selector {
                0 => {
                    let hn = HeaderName::from_lowercase(header_name_data);
                    let hv = HeaderValue::from_name(_unwrap_result(hn));
                    headers.insert(_unwrap_result(HeaderName::from_bytes(&GLOBAL_DATA[50..80])), hv);
                }
                1 => {
                    let uri = Uri::from_static(_to_str(GLOBAL_DATA, 100, 200));
                    req_builder.uri(uri);
                }
                2 => {
                    let status = _unwrap_result(StatusCode::from_u16(_to_u16(GLOBAL_DATA, 300)));
                    res_builder.status(status);
                }
                3 => {
                    let auth = Authority::from_static(_to_str(GLOBAL_DATA, 400, 450));
                    let uri = Uri::builder().authority(auth).build().unwrap();
                    req_builder.uri(uri);
                }
                4 => {
                    let method = _unwrap_result(Method::from_bytes(&GLOBAL_DATA[500..520]));
                    req_builder.method(method);
                }
                _ => ()
            }
        }

        let header_name = _unwrap_result(HeaderName::from_bytes(header_name_data));
        println!("{:?}", header_name.as_str());
        
        let mut req = _unwrap_result(req_builder.body(()));
        *req.headers_mut() = headers;
        
        let res = _unwrap_result(res_builder.body(()));
        let res_headers = res.headers();
        
        if let Some(hn) = req.headers().keys().next() {
            let _ = res_headers.get(hn);
        }

        let uri_parts = req.uri().clone().into_parts();
        let rebuilt_uri = _unwrap_result(Uri::from_parts(uri_parts));
        let scheme = _unwrap_result(Scheme::from_str(_to_str(GLOBAL_DATA, 600, 620)));
        let path = PathAndQuery::from_static(_to_str(GLOBAL_DATA, 630, 700));
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