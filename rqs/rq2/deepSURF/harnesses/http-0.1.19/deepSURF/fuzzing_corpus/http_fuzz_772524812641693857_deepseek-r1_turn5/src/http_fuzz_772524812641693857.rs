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
        let gdata = global_data.first_half;

        let op_count = _to_u8(gdata, 0) % 8;
        let mut map = header::HeaderMap::with_capacity(32);
        let mut response_builder = Response::builder();
        let mut uri_builder = Uri::builder();

        for i in 0..op_count {
            let offset = 1 + i as usize * 16;
            let selector = _to_u8(gdata, offset) % 12;

            match selector {
                0 => {
                    let len = _to_u8(gdata, offset+1) % 64;
                    let s = _to_str(gdata, offset+2, offset+2 + len as usize);
                    let hn = _unwrap_result(<header::HeaderName as HttpTryFrom<&str>>::try_from(s));
                    let hv = _unwrap_result(header::HeaderValue::from_bytes(s.as_bytes()));
                    map.insert(hn, hv);
                }
                1 => {
                    let len = _to_u8(gdata, offset+1) % 64;
                    let s = _to_str(gdata, offset+2, offset+2 + len as usize);
                    let uri = _unwrap_result(<Uri as HttpTryFrom<&str>>::try_from(s));
                    let mut builder = Request::builder();
                    builder.uri(uri);
                    let _ = builder.body(());
                }
                2 => {
                    let name_len = _to_u8(gdata, offset+1) % 64;
                    let name = _to_str(gdata, offset+2, offset+2 + name_len as usize);
                    let hn = _unwrap_result(<header::HeaderName as HttpTryFrom<&str>>::try_from(name));
                    let val_len = _to_u8(gdata, offset+2 + name_len as usize) % 64;
                    let val = _to_str(gdata, offset+3 + name_len as usize, offset+3 + name_len as usize + val_len as usize);
                    let hv = _unwrap_result(header::HeaderValue::from_str(val));
                    map.append(hn, hv);
                }
                3 => {
                    let len = _to_u8(gdata, offset+1) % 64;
                    let s = _to_str(gdata, offset+2, offset+2 + len as usize);
                    let auth = _unwrap_result(<uri::Authority as HttpTryFrom<&str>>::try_from(s));
                    let bytes = auth.as_str().as_bytes();
                    let hv = _unwrap_result(header::HeaderValue::from_bytes(bytes));
                    map.insert(header::HOST, hv);
                }
                4 => {
                    for (name, value) in &map {
                        println!("{:?} -> {:?}", name, value);
                    }
                }
                5 => {
                    let len = _to_u8(gdata, offset+1) % 64;
                    let s = _to_str(gdata, offset+2, offset+2 + len as usize);
                    let _ = _unwrap_result(<header::HeaderName as HttpTryFrom<&str>>::try_from(s));
                }
                6 => {
                    let status = _to_u16(gdata, offset+1);
                    response_builder.status(status);
                }
                7 => {
                    let method_byte = _to_u8(gdata, offset+1) % 6;
                    let method = match method_byte {
                        0 => Method::GET,
                        1 => Method::POST,
                        2 => Method::PUT,
                        3 => Method::DELETE,
                        4 => Method::HEAD,
                        _ => Method::OPTIONS,
                    };
                    let _ = Request::builder().method(method);
                }
                8 => {
                    let path_len = _to_u8(gdata, offset+1) % 64;
                    let path = _to_str(gdata, offset+2, offset+2 + path_len as usize);
                    uri_builder.path_and_query(path);
                }
                9 => {
                    let scheme_byte = _to_u8(gdata, offset+1) % 2;
                    let scheme = match scheme_byte {
                        0 => "http",
                        _ => "https",
                    };
                    uri_builder.scheme(scheme);
                }
                10 => {
                    let authority_len = _to_u8(gdata, offset+1) % 64;
                    let authority = _to_str(gdata, offset+2, offset+2 + authority_len as usize);
                    uri_builder.authority(authority);
                }
                11 => {
                    let uri = _unwrap_result(uri_builder.build());
                    let _ = uri.path_and_query().map(|pq| pq.path());
                }
                _ => {}
            }
        }

        let response = response_builder.body(());
        let _ = _unwrap_result(response);

        let final_name = _to_str(gdata, 512, 768);
        let _ = <header::HeaderName as HttpTryFrom<&str>>::try_from(final_name);
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