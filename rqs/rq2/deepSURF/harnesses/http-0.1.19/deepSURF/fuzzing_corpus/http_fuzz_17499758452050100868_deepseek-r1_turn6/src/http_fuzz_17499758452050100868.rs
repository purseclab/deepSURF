#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use http::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 512 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let op_count = _to_u8(GLOBAL_DATA, 0) % 8 + 1;
        let mut cursor = 1;

        let auth1_len = _to_u8(GLOBAL_DATA, cursor) % 64;
        cursor += 1;
        let auth1 = _to_str(GLOBAL_DATA, cursor as usize, cursor as usize + auth1_len as usize);
        cursor += auth1_len as usize;

        let auth2_len = _to_u8(GLOBAL_DATA, cursor) % 64;
        cursor += 1;
        let auth2 = _to_str(GLOBAL_DATA, cursor as usize, cursor as usize + auth2_len as usize);
        cursor += auth2_len as usize;

        let authority1 = http::uri::Authority::from_static(auth1);
        let authority2 = http::uri::Authority::from_static(auth2);

        let mut header_map = http::header::HeaderMap::new();
        for _ in 0..(_to_u8(GLOBAL_DATA, cursor) % 5) {
            cursor += 1;
            let name = _unwrap_result(http::header::HeaderName::from_bytes(&GLOBAL_DATA[cursor..cursor+4]));
            cursor += 4;
            let value = _unwrap_result(http::header::HeaderValue::from_bytes(&GLOBAL_DATA[cursor..cursor+4]));
            cursor += 4;
            header_map.append(name, value);
        }

        let uri = http::uri::Uri::from_static(auth1);
        let _ = uri.authority();
        println!("{:?}", uri.port_u16());

        let mut req_builder = http::request::Request::builder();
        req_builder.uri(uri);
        let _req = req_builder.body(()).unwrap();

        let mut response_builder = http::response::Response::builder();
        let method = _to_u8(GLOBAL_DATA, cursor);
        cursor += 1;
        let method_str = _to_str(GLOBAL_DATA, cursor, cursor + 4);
        cursor += 4;
        let method = _unwrap_result(http::method::Method::from_str(method_str));
        response_builder.status(_to_u16(GLOBAL_DATA, cursor));
        cursor += 2;
        
        for _ in 0..op_count {
            let op = _to_u8(GLOBAL_DATA, cursor) % 7;
            cursor += 1;
            
            match op {
                0 => {
                    let cmp_result = authority1.partial_cmp(&authority2);
                    println!("{:?}", cmp_result);
                },
                1 => {
                    let path = _to_str(GLOBAL_DATA, cursor, cursor + 8);
                    cursor += 8;
                    let path_query = _unwrap_result(http::uri::PathAndQuery::from_str(path));
                    let _ = path_query.partial_cmp("test/path");
                },
                2 => {
                    let val = _to_u16(GLOBAL_DATA, cursor);
                    cursor += 2;
                    let header_val = http::header::HeaderValue::from(val);
                    header_map.insert("X-Test", header_val);
                },
                3 => {
                    let scheme = _to_str(GLOBAL_DATA, cursor, cursor + 5);
                    cursor += 5;
                    let _ = _unwrap_result(http::uri::Scheme::from_str(scheme));
                },
                4 => {
                    let port = _to_u16(GLOBAL_DATA, cursor);
                    cursor += 2;
                    let _ = authority1.port_u16().partial_cmp(&Some(port));
                },
                5 => {
                    let status = _to_u16(GLOBAL_DATA, cursor);
                    cursor += 2;
                    let _ = http::status::StatusCode::from_u16(status).unwrap();
                },
                _ => {
                    let auth_str = _to_str(GLOBAL_DATA, cursor, cursor + 8);
                    cursor += 8;
                    let auth_bytes = http::uri::Authority::from_static(auth_str);
                    println!("{:?}", auth_bytes.as_str());
                }
            }
        }

        let ordering = authority1.partial_cmp(auth2);
        println!("{:?}", ordering);

        let path_and_query = http::uri::PathAndQuery::from_static(auth1);
        let _cmp = path_and_query.partial_cmp("test/path");
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