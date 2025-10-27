#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use http::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

fn main (){
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 256 {return;}
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        
        let mut offset = 0;
        let op_count = _to_u8(GLOBAL_DATA, offset) % 4;
        offset += 1;

        let mut headers = http::HeaderMap::new();
        let mut uri_builder = uri::Builder::new();
        let mut req_builder = request::Request::builder();
        let mut res_builder = response::Response::builder();

        for _ in 0..op_count {
            let op_type = _to_u8(GLOBAL_DATA, offset) % 7;
            offset += 1;

            match op_type {
                0 => {
                    let scheme = _to_str(GLOBAL_DATA, offset, offset + (_to_u8(GLOBAL_DATA, offset + 1) % 16) as usize);
                    let _ = uri_builder.scheme(scheme);
                }
                1 => {
                    let path = _to_str(GLOBAL_DATA, offset, offset + (_to_u8(GLOBAL_DATA, offset + 1) % 32) as usize);
                    let _ = uri_builder.path_and_query(path);
                }
                2 => {
                    let name_len = _to_u8(GLOBAL_DATA, offset) as usize % 16;
                    let val_len = _to_u8(GLOBAL_DATA, offset + 1) as usize % 32;
                    let name_start = offset + 2;
                    let val_start = name_start + name_len;

                    let name_str = _to_str(GLOBAL_DATA, name_start, name_start + name_len);
                    let val_str = _to_str(GLOBAL_DATA, val_start, val_start + val_len);

                    match header::HeaderName::from_bytes(name_str.as_bytes()) {
                        Ok(name) => {
                            match header::HeaderValue::from_bytes(val_str.as_bytes()) {
                                Ok(val) => { headers.append(&name, val); }
                                _ => ()
                            }
                        }
                        _ => ()
                    }
                }
                3 => {
                    let method_data = &GLOBAL_DATA[offset..offset + 8];
                    if let Ok(method) = method::Method::from_bytes(method_data) {
                        let _ = req_builder.method(method);
                    }
                }
                4 => {
                    let status = _to_u16(GLOBAL_DATA, offset);
                    if let Ok(code) = status::StatusCode::from_u16(status) {
                        let _ = res_builder.status(code);
                    }
                }
                5 => {
                    let construct_type = _to_u8(GLOBAL_DATA, offset) % 3;
                    offset += 1;

                    let len1 = _to_u8(GLOBAL_DATA, offset) as usize % 32;
                    offset += 1;
                    let s1 = _to_str(GLOBAL_DATA, offset, offset + len1);
                    offset += len1;

                    let len2 = _to_u8(GLOBAL_DATA, offset) as usize % 32;
                    offset += 1;
                    let s2 = _to_str(GLOBAL_DATA, offset, offset + len2);
                    offset += len2;

                    let hn1 = match construct_type {
                        0 => header::HeaderName::from_static(s1),
                        1 => header::HeaderName::from_bytes(s1.as_bytes()).unwrap_or_else(|_| header::HeaderName::from_static("")),
                        2 => header::HeaderName::from_str(s1).unwrap_or_else(|_| header::HeaderName::from_static("")),
                        _ => unreachable!()
                    };

                    let hn2 = header::HeaderName::from_static(s2);
                    let _ = s1.eq(&hn2);
                    let _ = (&hn1).eq(s2);
                }
                6 => {
                    let authority_start = offset;
                    let authority_len = _to_u8(GLOBAL_DATA, offset) as usize % 64;
                    offset += 1;
                    let authority_str = _to_str(GLOBAL_DATA, authority_start, authority_start + authority_len);
                    if let Ok(auth) = uri::Authority::from_str(authority_str) {
                        let _ = uri_builder.authority(auth);
                    }
                }
                _ => unreachable!()
            }

            offset = (offset + _to_u8(GLOBAL_DATA, offset) as usize) % GLOBAL_DATA.len();
        }

        let _uri = uri_builder.build();
        let _ = headers.entry(header::ACCEPT).unwrap().or_insert(header::HeaderValue::from_static("*/*"));
        let _req = req_builder.uri("http://example.com").body::<&[u8]>(&[]);
        let _res = res_builder.version(version::Version::HTTP_11).body::<&[u8]>(&[]);

        if let Some(entry) = headers.get(header::USER_AGENT) {
            let _ = entry.to_str();
        }

        let _ = headers.insert(header::CONTENT_TYPE, header::HeaderValue::from_static("text/plain"));
        let _ = headers.remove(header::HOST);
        let _ = headers.keys().next().map(|k| println!("{:?}", k));
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