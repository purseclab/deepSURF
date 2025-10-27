#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use http::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 256 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let mut offset = 0;
        let ops = _to_u8(GLOBAL_DATA, offset) % 16;
        offset += 1;

        let mut uris = Vec::new();
        let mut builders = Vec::new();
        let mut headers = header::HeaderMap::new();

        for _ in 0..ops {
            let constructor_type = _to_u8(GLOBAL_DATA, offset) % 4;
            offset += 1;

            match constructor_type {
                0 => {
                    let len = _to_u8(GLOBAL_DATA, offset) % 64;
                    offset += 1;
                    let s = _to_str(GLOBAL_DATA, offset, offset + len as usize);
                    offset += len as usize;
                    if let Ok(u) = uri::Uri::from_str(s) {
                        uris.push(u);
                    }
                }
                1 => {
                    let len = _to_u8(GLOBAL_DATA, offset) % 64;
                    offset += 1;
                    let s = _to_str(GLOBAL_DATA, offset, offset + len as usize);
                    offset += len as usize;
                    uris.push(uri::Uri::from_static(s));
                }
                2 => {
                    let len = _to_u8(GLOBAL_DATA, offset) % 64;
                    offset += 1;
                    let s = _to_str(GLOBAL_DATA, offset, offset + len as usize);
                    offset += len as usize;
                    if let Ok(auth) = uri::Authority::from_str(s) {
                        let mut builder = uri::Builder::new();
                        builder.authority(auth);
                        builders.push(builder);
                    }
                }
                3 => {
                    let len = _to_u8(GLOBAL_DATA, offset) % 64;
                    offset += 1;
                    let s = _to_str(GLOBAL_DATA, offset, offset + len as usize);
                    offset += len as usize;
                    if let Ok(scheme) = uri::Scheme::from_str(s) {
                        let mut builder = uri::Builder::new();
                        builder.scheme(scheme);
                        builders.push(builder);
                    }
                }
                _ => {}
            }

            let header_name_len = _to_u8(GLOBAL_DATA, offset) % 32;
            offset += 1;
            let header_name = _to_str(GLOBAL_DATA, offset, offset + header_name_len as usize);
            offset += header_name_len as usize;

            let header_value_len = _to_u8(GLOBAL_DATA, offset) % 32;
            offset += 1;
            let header_value = _to_str(GLOBAL_DATA, offset, offset + header_value_len as usize);
            offset += header_value_len as usize;

            if let Ok(name) = header::HeaderName::from_bytes(header_name.as_bytes()) {
                if let Ok(value) = header::HeaderValue::from_str(header_value) {
                    headers.insert(name, value);
                }
            }
        }

        for mut builder in builders.drain(..) {
            if let Ok(u) = builder.build() {
                uris.push(u);
            }
        }

        for i in 0..uris.len() {
            for j in 0..uris.len() {
                let a = &uris[i];
                let b = &uris[j];
                a.eq(b);
                println!("{:?}", a);
                println!("{:?}", b);
            }
        }

        let mut req_builder = request::Request::builder();
        let mut res_builder = response::Response::builder();
        let _ = req_builder.uri(&uris[0]);
        let _ = res_builder.status(200);
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