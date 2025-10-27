#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use http::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 1024 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let mut offset = 0;
        let num_ops = _to_u8(GLOBAL_DATA, offset) % 16;
        offset += 1;

        let mut schemes = Vec::new();
        let mut header_map = header::HeaderMap::new();
        let mut builder = request::Request::builder();
        let mut response_builder = response::Response::builder();

        for _ in 0..num_ops {
            if offset + 1 > GLOBAL_DATA.len() { break; }
            let op_type = _to_u8(GLOBAL_DATA, offset) % 8;
            offset += 1;

            match op_type {
                0 => {
                    let len = _to_u8(GLOBAL_DATA, offset) % 64;
                    offset += 1;
                    let s = _to_str(GLOBAL_DATA, offset, offset + len as usize);
                    offset += len as usize;
                    if let Ok(scheme) = uri::Scheme::from_str(s) {
                        schemes.push(scheme);
                    }
                }
                1 => {
                    let len = _to_u8(GLOBAL_DATA, offset) % 64;
                    offset += 1;
                    let s = _to_str(GLOBAL_DATA, offset, offset + len as usize);
                    offset += len as usize;
                    if let Ok(auth) = uri::Authority::from_str(s) {
                        let _ = auth.port_u16();
                        println!("Authority: {:?}", auth);
                    }
                }
                2 => {
                    let name_len = _to_u8(GLOBAL_DATA, offset) % 32;
                    offset += 1;
                    let val_len = _to_u8(GLOBAL_DATA, offset) % 32;
                    offset += 1;
                    let name = _to_str(GLOBAL_DATA, offset, offset + name_len as usize);
                    offset += name_len as usize;
                    let value = _to_str(GLOBAL_DATA, offset, offset + val_len as usize);
                    offset += val_len as usize;
                    let n = header::HeaderName::from_static(name);
                    let v = header::HeaderValue::from_static(value);
                    header_map.append(n, v);
                }
                3 => {
                    let len = _to_u8(GLOBAL_DATA, offset) % 64;
                    offset += 1;
                    let s = _to_str(GLOBAL_DATA, offset, offset + len as usize);
                    offset += len as usize;
                    if let Ok(uri) = Uri::from_str(s) {
                        let authority = uri.authority_part().map(|a| a.as_str());
                        println!("URI authority: {:?}", authority);
                        builder.uri(uri);
                    }
                }
                4 => {
                    let len = _to_u8(GLOBAL_DATA, offset) % 64;
                    offset += 1;
                    let s = _to_str(GLOBAL_DATA, offset, offset + len as usize);
                    offset += len as usize;
                    let method = method::Method::from_bytes(s.as_bytes());
                    if let Ok(m) = method {
                        builder.method(m);
                    }
                }
                5 => {
                    let len = _to_u8(GLOBAL_DATA, offset) % 64;
                    offset += 1;
                    let s = _to_str(GLOBAL_DATA, offset, offset + len as usize);
                    offset += len as usize;
                    if let Ok(pq) = uri::PathAndQuery::from_str(s) {
                        println!("PathAndQuery: {:?}", pq);
                    }
                }
                6 => {
                    for (name, val) in header_map.iter() {
                        println!("Header: {:?} => {:?}", name.as_str(), val.to_str());
                    }
                }
                7 => {
                    let status_code = _to_u16(GLOBAL_DATA, offset) % 600;
                    offset += 2;
                    if let Ok(code) = status::StatusCode::from_u16(status_code) {
                        response_builder.status(code);
                    }
                }
                _ => {}
            }
        }

        if schemes.len() >= 2 {
            for i in 1..schemes.len() {
                let a = &schemes[i-1];
                let b = &schemes[i];
                a.eq(b);
                println!("Comparing {:?} vs {:?}", a.as_str(), b.as_str());
            }
        }

        let _ = builder.body(());
        let _ = response_builder.body(());

        if let Some(val) = header_map.get("content-type") {
            println!("Content-Type: {:?}", val.to_str());
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