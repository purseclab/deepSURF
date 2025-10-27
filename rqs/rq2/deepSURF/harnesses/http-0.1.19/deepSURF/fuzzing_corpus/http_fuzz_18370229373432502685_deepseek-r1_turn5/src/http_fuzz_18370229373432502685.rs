#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use http::*;
use global_data::*;
use std::str::FromStr;

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 4 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        
        for i in 0..(GLOBAL_DATA.len() % 65) {
            match GLOBAL_DATA[i] % 7 {
                0 => {
                    let bytes = bytes::Bytes::from_static(b"http");
                    let scheme = http::uri::Scheme::from_shared(bytes.clone()).unwrap_or(http::uri::Scheme::HTTP);
                    let _ = <http::uri::Scheme as http::HttpTryFrom<_>>::try_from(scheme);
                },
                1 => {
                    let auth_data = &GLOBAL_DATA[i..(i+8).min(GLOBAL_DATA.len())];
                    let authority = http::uri::Authority::from_static("example.com");
                    let shared = authority.as_str().as_bytes();
                    let _ = <http::uri::Authority as http::HttpTryFrom<_>>::try_from(shared);
                    let mut header_map = http::header::HeaderMap::new();
                    let entry = header_map.entry(http::header::HOST).expect("entry").or_insert(http::header::HeaderValue::from_static("localhost"));
                    println!("{:?}", entry);
                },
                2 => {
                    let end = (i + 16).min(GLOBAL_DATA.len());
                    let path_str = _to_str(GLOBAL_DATA, i, end);
                    let path = http::uri::PathAndQuery::from_static("/path");
                    let _ = path.path().eq(path_str);
                    let path_from_bytes = http::uri::PathAndQuery::from_shared(bytes::Bytes::from(&GLOBAL_DATA[i..end])).unwrap_or_else(|_| path.clone());
                    println!("{:?} {:?}", path, path_from_bytes);
                },
                3 => {
                    let mut headers = http::header::HeaderMap::new();
                    let name = http::header::HeaderName::from_bytes(&GLOBAL_DATA[i..i+2]).unwrap_or(
                        http::header::HeaderName::from_static("host")
                    );
                    let value = http::header::HeaderValue::from_bytes(&GLOBAL_DATA[i..i+4]).unwrap_or(
                        http::header::HeaderValue::from_static("value")
                    );
                    headers.insert(name.clone(), value);
                    let _val = headers.get(&name).map(|v| println!("{:?}", v));
                    headers.append(name, http::header::HeaderValue::from(GLOBAL_DATA[i] as i64));
                },
                4 => {
                    let mut uri_builder = http::uri::Builder::new();
                    let scheme = http::uri::Scheme::HTTP;
                    let uri = uri_builder.scheme(scheme).build().unwrap();
                    let parts = uri.into_parts();
                    let rebuilt = http::uri::Uri::from_parts(parts).unwrap();
                    println!("{:?}", rebuilt);
                    let response_builder = http::response::Response::builder().status(200).header("x-fuzz", "test");
                },
                5 => {
                    let method = http::method::Method::from_bytes(&GLOBAL_DATA[i..i+4]).unwrap_or(
                        http::method::Method::GET
                    );
                    let mut builder = http::request::Request::builder();
                    builder.method(method).uri("http://example.com");
                    let _ = builder.body(());
                },
                6 => {
                    let scheme_data = &GLOBAL_DATA[i..(i+4).min(GLOBAL_DATA.len())];
                    let auth_bytes = bytes::Bytes::from(scheme_data);
                    let authority = http::uri::Authority::from_shared(auth_bytes.clone()).unwrap_or_else(|_| 
                        http::uri::Authority::from_static("localhost")
                    );
                    let header_val = http::header::HeaderValue::from_shared(auth_bytes)
                        .unwrap_or_else(|_| http::header::HeaderValue::from_static("value"));
                    println!("{:?} {:?}", authority.as_str(), header_val.to_str().unwrap());
                },
                _ => unreachable!(),
            }
        }
    });
}

// Type conversion functions remain unchanged below...

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