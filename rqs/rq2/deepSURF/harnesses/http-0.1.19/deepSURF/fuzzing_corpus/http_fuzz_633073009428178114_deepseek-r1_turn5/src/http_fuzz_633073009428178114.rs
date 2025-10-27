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
        let num_operations = _to_u8(GLOBAL_DATA, offset) % 5;
        offset += 1;

        let mut headers = header::HeaderMap::new();
        let header_count = _to_u8(GLOBAL_DATA, offset) % 10;
        offset += 1;

        for _ in 0..header_count {
            let key_len = _to_u8(GLOBAL_DATA, offset) % 15 + 1;
            offset += 1;
            let key = _to_str(GLOBAL_DATA, offset, (offset + key_len as usize));
            offset += key_len as usize;

            let val_len = _to_u8(GLOBAL_DATA, offset) % 20 + 1;
            offset += 1;
            let val = _to_str(GLOBAL_DATA, offset, (offset + val_len as usize));
            offset += val_len as usize;

            match header::HeaderName::from_bytes(key.as_bytes()) {
                Ok(hdr) => {
                    headers.insert(hdr.clone(), val.parse().unwrap());
                    println!("Header: {:?} => {:?}", hdr, headers.get(&hdr).unwrap());
                }
                _ => ()
            }
        }

        let method_idx = _to_u8(GLOBAL_DATA, offset) % 7;
        offset += 1;
        let method = match method_idx {
            0 => method::Method::GET,
            1 => method::Method::POST,
            2 => method::Method::PUT,
            3 => method::Method::DELETE,
            4 => method::Method::HEAD,
            5 => method::Method::OPTIONS,
            _ => method::Method::CONNECT,
        };

        let uri_len = _to_u8(GLOBAL_DATA, offset) % 50 + 10;
        offset += 1;
        let uri_str = _to_str(GLOBAL_DATA, offset, (offset + uri_len as usize));
        offset += uri_len as usize;

        let uri = uri::Uri::from_str(uri_str).unwrap_or(uri::Uri::from_static(""));
        println!("URI: {:?}", uri);
        println!("URI Path: {:?}", uri.path());
        if let Some(scheme) = uri.scheme_str() {
            println!("URI Scheme: {}", scheme);
        }

        let mut builder = request::Request::builder();
        builder.method(method);
        builder.uri(uri);
        builder.version(version::Version::HTTP_11);

        for (key, value) in headers.iter() {
            builder.header(key, value);
        }

        let authority_idx = _to_u8(GLOBAL_DATA, offset) % 3;
        offset += 1;
        let authority = match authority_idx {
            0 => uri::Authority::from_static("localhost"),
            1 => uri::Authority::from_static("example.com"),
            _ => <uri::Authority as HttpTryFrom<&str>>::try_from(_to_str(GLOBAL_DATA, offset, offset + 10)).unwrap_or(uri::Authority::from_static("")),
        };
        println!("Authority: {:?}", authority);
        println!("Authority Host: {:?}", authority.host());
        if let Some(port) = authority.port_u16() {
            println!("Authority Port: {}", port);
        }

        let scheme_idx = _to_u8(GLOBAL_DATA, offset) % 4;
        offset += 1;
        let scheme_input = _to_str(GLOBAL_DATA, offset, (offset + 15));
        offset += 15;
        
        if let Ok(scheme) = uri::Scheme::from_str(scheme_input) {
            println!("Scheme: {:?}", scheme.as_str());
        }
        let _ = <uri::Scheme as HttpTryFrom<&[u8]>>::try_from(scheme_input.as_bytes());

        for (hdr, val) in headers.iter() {
            println!("Header Entry: {:?} => {:?}", hdr.as_str(), val.to_str().unwrap());
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