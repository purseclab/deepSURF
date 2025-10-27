#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use http::*;
use http::header::HeaderName;
use http::uri::Authority;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 256 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let ops_count = _to_u8(GLOBAL_DATA, 0) % 8 + 1;
        let mut header_map = http::header::HeaderMap::new();
        let mut response_builder = http::response::Builder::new();
        let mut request_builder = http::request::Builder::new();

        for i in 0..ops_count {
            match _to_u8(GLOBAL_DATA, i as usize + 1) % 14 {
                0 => {
                    let len = _to_u8(GLOBAL_DATA, 2 + i as usize) % 32;
                    let s = _to_str(GLOBAL_DATA, 3 + i as usize, 3 + i as usize + len as usize);
                    let uri = _unwrap_result(Uri::from_str(s));
                    let _ = uri.port_part();
                },
                1 => {
                    let len = _to_u8(GLOBAL_DATA, 4 + i as usize) % 32;
                    let auth_str = _to_str(GLOBAL_DATA, 5 + i as usize, 5 + i as usize + len as usize);
                    let authority = _unwrap_result(Authority::from_str(auth_str));
                    let _ = authority.port_part();
                },
                2 => {
                    let uri_parts = Uri::from_static("http://example.com:8080/foo?bar").into_parts();
                    let rebuilt_uri = _unwrap_result(Uri::from_parts(uri_parts));
                    println!("{:?}", rebuilt_uri.port_part());
                },
                3 => {
                    let key_len = _to_u8(GLOBAL_DATA, 6 + i as usize) % 16;
                    let key = _to_str(GLOBAL_DATA, 7 + i as usize, 7 + i as usize + key_len as usize);
                    let hname = _unwrap_result(HeaderName::from_bytes(key.as_bytes()));
                    let hval = HeaderValue::from_static("value");
                    header_map.insert(hname, hval);
                },
                4 => {
                    let key_len = _to_u8(GLOBAL_DATA, 8 + i as usize) % 16;
                    let key = _to_str(GLOBAL_DATA, 9 + i as usize, 9 + i as usize + key_len as usize);
                    let hname = _unwrap_result(HeaderName::from_bytes(key.as_bytes()));
                    let _entries = header_map.entry(hname).unwrap();
                },
                5 => {
                    let mut builder = Uri::builder();
                    let path_len = _to_u8(GLOBAL_DATA, 10 + i as usize) % 32;
                    let path = _to_str(GLOBAL_DATA, 11 + i as usize, 11 + i as usize + path_len as usize);
                    let uri = _unwrap_result(builder.path_and_query(path).build());
                    let _ = uri.port_part();
                },
                6 => {
                    let auth_len = _to_u8(GLOBAL_DATA, 12 + i as usize) % 32;
                    let auth_str = _to_str(GLOBAL_DATA, 13 + i as usize, 13 + i as usize + auth_len as usize);
                    let auth = _unwrap_result(Authority::from_str(auth_str));
                    let uri = Uri::builder()
                        .authority(auth)
                        .path_and_query("/")
                        .build()
                        .unwrap();
                    println!("{:?}", uri.port_part());
                },
                7 => {
                    let method_len = _to_u8(GLOBAL_DATA, 14 + i as usize) % 16;
                    let method_str = _to_str(GLOBAL_DATA, 15 + i as usize, 15 + i as usize + method_len as usize);
                    let method = _unwrap_result(http::method::Method::from_str(method_str));
                    request_builder.method(method);
                },
                8 => {
                    let status_code = _to_u16(GLOBAL_DATA, 16 + i as usize);
                    response_builder.status(status_code);
                },
                9 => {
                    let key_len = _to_u8(GLOBAL_DATA, 18 + i as usize) % 16;
                    let key = _to_str(GLOBAL_DATA, 19 + i as usize, 19 + i as usize + key_len as usize);
                    let hname = _unwrap_result(HeaderName::from_bytes(key.as_bytes()));
                    let hval = HeaderValue::from_static("value");
                    response_builder.header(hname, hval);
                },
                10 => {
                    let scheme_len = _to_u8(GLOBAL_DATA, 20 + i as usize) % 16;
                    let scheme_str = _to_str(GLOBAL_DATA, 21 + i as usize, 21 + i as usize + scheme_len as usize);
                    let scheme = _unwrap_result(http::uri::Scheme::from_str(scheme_str));
                    let _uri = Uri::builder().scheme(scheme).build().unwrap();
                },
                11 => {
                    for (name, value) in header_map.iter() {
                        println!("Header: {:?} => {:?}", name, value);
                    }
                },
                12 => {
                    let version_val = _to_u8(GLOBAL_DATA, 22 + i as usize) % 2;
                    let version = if version_val == 0 { http::Version::HTTP_11 } else { http::Version::HTTP_2 };
                    request_builder.version(version);
                },
                13 => {
                    if let Some(k) = header_map.keys().next().cloned() {
                        header_map.remove(&k);
                    }
                },
                _ => ()
            }
        }

        let final_len = _to_u8(GLOBAL_DATA, 127) % 64;
        let final_str = _to_str(GLOBAL_DATA, 128, 128 + final_len as usize);
        let final_uri = _unwrap_result(Uri::from_str(final_str));
        let port_part = final_uri.port_part();
        println!("{:?}", port_part);
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