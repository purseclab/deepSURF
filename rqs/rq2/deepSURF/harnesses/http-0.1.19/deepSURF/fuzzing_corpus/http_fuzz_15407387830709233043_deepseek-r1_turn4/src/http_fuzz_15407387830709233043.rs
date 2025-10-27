#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use http::*;
use global_data::*;
use std::str::FromStr;
use bytes::*;

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 1 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let mut headers = header::HeaderMap::new();
        let num_ops = _to_usize(GLOBAL_DATA, 0) % 65;
        let mut offset = 1;

        for _ in 0..num_ops {
            if offset + 1 > GLOBAL_DATA.len() { break; }

            let op_choice = _to_u8(GLOBAL_DATA, offset) % 7;
            offset += 1;

            match op_choice {
                0 => {
                    let bytes = Bytes::new();
                    let _ = <header::HeaderName as HttpTryFrom<_>>::try_from(bytes);
                }
                1 => {
                    let static_str = _to_str(GLOBAL_DATA, offset, (offset + 5) % GLOBAL_DATA.len());
                    let bytes = Bytes::from_static(static_str.as_bytes());
                    let name = _unwrap_result(header::HeaderName::from_bytes(&bytes));
                    headers.entry(name.clone()).unwrap().or_insert(header::HeaderValue::from_static("value"));
                    let name_clone = name.clone();
                    let _val = header::HeaderValue::from_name(name_clone);
                    println!("{:?}", headers.get(name));
                }
                2 => {
                    let auth_bytes = Bytes::from_static(b"example.com");
                    let authority = _unwrap_result(uri::Authority::from_shared(auth_bytes));
                    let bytes = Bytes::from(authority.clone());
                    let name = _unwrap_result(<header::HeaderName as HttpTryFrom<_>>::try_from(bytes));
                    let _ = headers.entry(name.clone()).unwrap().or_insert(name.clone().into());
                    println!("{:?}", authority.as_str());
                }
                3 => {
                    let path_bytes = Bytes::from_static(b"/path?query");
                    let path = _unwrap_result(uri::PathAndQuery::from_shared(path_bytes));
                    let bytes = Bytes::from(path.clone());
                    let _ = <header::HeaderName as HttpTryFrom<_>>::try_from(bytes);
                    println!("{:?}", path.as_str());
                }
                4 => {
                    let mut builder = request::Request::builder();
                    let method = _unwrap_result(header::HeaderName::from_bytes(&GLOBAL_DATA[offset..offset+4]));
                    builder.header(method.clone(), "value");
                    let _ = builder.body(());
                    let _ = uri::Uri::from_static("http://example.com");
                }
                5 => {
                    let key = headers.keys().next().cloned();
                    if let Some(name) = key {
                        headers.remove(&name);
                        let _ = headers.contains_key(&name);
                    }
                }
                6 => {
                    let mut response_builder = response::Response::builder();
                    let status = _unwrap_result(status::StatusCode::from_u16(_to_u16(GLOBAL_DATA, offset)));
                    let _ = response_builder.status(status).body(());
                }
                _ => {}
            }

            offset = (offset + 4) % GLOBAL_DATA.len();
        }

        let final_bytes = Bytes::from_static(&GLOBAL_DATA[offset..(offset + 8) % GLOBAL_DATA.len()]);
        let _ = <header::HeaderName as HttpTryFrom<_>>::try_from(final_bytes);
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