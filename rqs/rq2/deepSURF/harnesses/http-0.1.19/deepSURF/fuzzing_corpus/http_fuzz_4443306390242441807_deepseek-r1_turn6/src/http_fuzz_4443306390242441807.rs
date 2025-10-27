#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use http::*;
use global_data::*;
use std::str::FromStr;

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 128 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let op_count = _to_u8(GLOBAL_DATA, 0) % 10;
        let mut cursor = 1;

        for _ in 0..op_count {
            let op_selector = _to_u8(GLOBAL_DATA, cursor) % 7;
            cursor = cursor.wrapping_add(1);

            match op_selector {
                0 => {
                    let uri_len = _to_u8(GLOBAL_DATA, cursor) as usize % 65;
                    cursor = cursor.wrapping_add(1);
                    let uri_str = _to_str(GLOBAL_DATA, cursor, cursor + uri_len);
                    cursor = cursor.wrapping_add(uri_len);
                    let uri = http::uri::Uri::from_static(uri_str);
                    let parts = uri.into_parts();
                    if let Some(authority) = parts.authority {
                        let port = authority.port_part();
                        println!("{:?}", _unwrap_option(port).as_str());
                    }
                }
                1 => {
                    let auth_str = _to_str(GLOBAL_DATA, cursor, cursor + 16);
                    cursor = cursor.wrapping_add(16);
                    let authority = _unwrap_result(http::uri::Authority::from_str(auth_str));
                    let port = authority.port_part();
                    println!("{:?}", _unwrap_option(port).as_str());
                }
                2 => {
                    let mut builder = http::uri::Builder::new();
                    let uri = _unwrap_result(builder.build());
                    let port = uri.port_part();
                    let _ = _unwrap_option(port).as_str();
                }
                3 => {
                    let mut req_builder = http::request::Request::builder();
                    let uri_len = _to_u8(GLOBAL_DATA, cursor) as usize % 65;
                    cursor = cursor.wrapping_add(1);
                    let uri_str = _to_str(GLOBAL_DATA, cursor, cursor + uri_len);
                    cursor = cursor.wrapping_add(uri_len);
                    req_builder.uri(uri_str);
                    let req = _unwrap_result(req_builder.body(()));
                    let uri = req.uri();
                    let port = uri.port_part();
                    println!("{:?}", _unwrap_option(port).as_str());
                }
                4 => {
                    let auth_len = _to_u8(GLOBAL_DATA, cursor) as usize % 65;
                    cursor = cursor.wrapping_add(1);
                    let auth_str = _to_str(GLOBAL_DATA, cursor, cursor + auth_len);
                    cursor = cursor.wrapping_add(auth_len);
                    let auth = _unwrap_result(http::uri::Authority::from_str(auth_str));
                    let header_name = http::header::HeaderName::from_static("Host");
                    let header_value = http::header::HeaderValue::from_static("localhost");
                    let mut headers = http::header::HeaderMap::new();
                    headers.insert(header_name, header_value);
                    let port = auth.port_part();
                    let _ = _unwrap_option(port).as_str();
                }
                5 => {
                    let path_len = _to_u8(GLOBAL_DATA, cursor) as usize % 65;
                    cursor = cursor.wrapping_add(1);
                    let path_str = _to_str(GLOBAL_DATA, cursor, cursor + path_len);
                    cursor = cursor.wrapping_add(path_len);
                    let path = _unwrap_result(http::uri::PathAndQuery::from_str(path_str));
                    let mut uri_builder = http::uri::Builder::new();
                    uri_builder.path_and_query(path);
                    let uri = _unwrap_result(uri_builder.build());
                    let port = uri.port_part();
                    let _ = _unwrap_option(port).as_str();
                }
                6 => {
                    let method_bytes = &GLOBAL_DATA[cursor..cursor+8];
                    let method = _unwrap_result(http::method::Method::from_bytes(method_bytes));
                    let mut req = http::request::Request::new(());
                    *req.method_mut() = method;
                    let port = req.uri().port_part();
                    println!("{:?}", _unwrap_option(port).as_str());
                }
                _ => unreachable!()
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