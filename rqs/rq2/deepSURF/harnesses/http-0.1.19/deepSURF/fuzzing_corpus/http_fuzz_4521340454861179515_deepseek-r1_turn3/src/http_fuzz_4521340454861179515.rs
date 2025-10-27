#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use http::*;
use global_data::*;
use std::str::FromStr;

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 512 { return; }
        set_global_data(data);
        
        let global_data = get_global_data();
        let g_data = global_data.first_half;
        let ops = _to_u8(g_data, 0) % 8;
        
        let mut path1 = String::new();
        let mut path2 = String::new();
        let mut builder = http::uri::Builder::new();
        let mut req_builder = http::request::Request::builder();
        let mut resp_builder = http::response::Response::builder();
        let mut header_map = http::header::HeaderMap::new();
        
        for i in 0..ops {
            let op_type = _to_u8(g_data, i as usize * 4) % 10;
            let offset = i as usize * 8;
            
            match op_type {
                0 => {
                    let len = _to_u8(g_data, offset) % 64;
                    path1 = _to_str(g_data, offset+1, offset+1 + len as usize).to_string();
                }
                1 => {
                    let len = _to_u8(g_data, offset) % 64;
                    path2 = _to_str(g_data, offset+1, offset+1 + len as usize).to_string();
                }
                2 => {
                    let auth_data = _to_str(g_data, offset, offset + 32);
                    if let Ok(auth) = http::uri::Authority::from_str(auth_data) {
                        let _ = builder.authority(auth);
                    }
                }
                3 => {
                    let scheme_data = _to_str(g_data, offset, offset + 8);
                    if let Ok(scheme) = http::uri::Scheme::from_str(scheme_data) {
                        let _ = builder.scheme(scheme);
                    }
                }
                4 => {
                    let path_data = _to_str(g_data, offset, offset + 32);
                    let _ = builder.path_and_query(path_data);
                }
                5 => {
                    let uri = builder.build().ok();
                    println!("{:?}", uri.as_ref().map(|u| u.path()));
                }
                6 => {
                    let method_data = _to_str(g_data, offset, offset + 16);
                    if let Ok(method) = http::method::Method::from_str(method_data) {
                        let _ = req_builder.method(method);
                    }
                }
                7 => {
                    let header_name = _to_str(g_data, offset, offset + 16);
                    let header_value = _to_str(g_data, offset+16, offset + 32);
                    if let Ok(name) = http::header::HeaderName::from_str(header_name) {
                        if let Ok(value) = http::header::HeaderValue::from_str(header_value) {
                            header_map.append(name, value);
                        }
                    }
                }
                8 => {
                    let status = _to_u16(g_data, offset);
                    if let Ok(status_code) = http::status::StatusCode::from_u16(status) {
                        let _ = resp_builder.status(status_code);
                    }
                }
                9 => {
                    let uri_data = _to_str(g_data, offset, offset + 64);
                    let _ = http::uri::Uri::from_str(uri_data);
                }
                _ => ()
            }
        }
        
        let pq1 = http::uri::PathAndQuery::from_str(&path1).ok();
        let pq2 = http::uri::PathAndQuery::from_shared(_to_str(g_data, 64, 128).as_bytes().into()).ok();
        
        if let (Some(p1), Some(p2)) = (&pq1, &pq2) {
            p1.partial_cmp(p2);
            p2.partial_cmp(&path1);
            println!("{:?}", p1);
        }
        
        let _req = req_builder.body(());
        let _resp = resp_builder.body(());
        
        let uri_from_parts = http::uri::Uri::from_parts(builder.build().unwrap().into_parts()).ok();
        println!("{:?}", uri_from_parts.as_ref().map(|u| u.authority()));
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