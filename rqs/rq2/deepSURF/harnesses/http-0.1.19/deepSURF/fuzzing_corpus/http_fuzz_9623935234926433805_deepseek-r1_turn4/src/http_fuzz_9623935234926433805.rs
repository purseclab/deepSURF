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

        let op_count = _to_u8(GLOBAL_DATA, 0) % 8 + 1;
        let mut offset = 1;

        for _ in 0..op_count {
            let selector = _to_u8(GLOBAL_DATA, offset) % 4;
            offset = (offset + 1) % GLOBAL_DATA.len();

            match selector {
                0 => {
                    let header_vec = (0..(_to_u8(GLOBAL_DATA, offset) % 5 + 1))
                        .map(|i| _to_u8(GLOBAL_DATA, (offset + i as usize) % GLOBAL_DATA.len()))
                        .collect::<Vec<u8>>();
                    offset = (offset + header_vec.len()) % GLOBAL_DATA.len();

                    let t1 = <http::header::HeaderName as http::HttpTryFrom<&[u8]>>::try_from(&header_vec[..]);
                    let t2 = _unwrap_result(t1);
                    let t3: &str = t2.as_ref();
                    println!("{:?}", t3);
                }
                1 => {
                    let mut map = http::header::HeaderMap::with_capacity(5);
                    for i in 0..3 {
                        let name = http::header::HeaderName::from_static("X-Custom-Header");
                        let val = http::header::HeaderValue::from_static("value");
                        map.append(&name, val);
                    }
                    let entry = map.entry("Content-Type").expect("valid header");
                    match entry {
                        http::header::Entry::Occupied(mut e) => {
                            e.insert("text/html".parse().unwrap());
                        }
                        http::header::Entry::Vacant(e) => {
                            e.insert("text/plain".parse().unwrap());
                        }
                    }
                    println!("{:?}", map.get("Content-Type").unwrap());
                }
                2 => {
                    let uri_data = &GLOBAL_DATA[offset..offset+32];
                    let uri = http::uri::Uri::from_static(
                        _to_str(uri_data, 0, uri_data.len())
                    );
                    let parts = uri.into_parts();
                    let rebuilt_uri = http::uri::Uri::from_parts(parts).unwrap();
                    println!("{}", rebuilt_uri);
                    offset = (offset + 32) % GLOBAL_DATA.len();
                }
                3 => {
                    let method_bytes = &GLOBAL_DATA[offset..offset+8];
                    let method = http::method::Method::from_bytes(method_bytes).unwrap();
                    let req = http::request::Request::builder()
                        .method(&method)
                        .uri("http://example.com")
                        .body(())
                        .unwrap();
                    println!("{:?}", req.method());
                    offset = (offset + 8) % GLOBAL_DATA.len();
                }
                _ => {}
            }

            let hn_bytes = &GLOBAL_DATA[offset..offset+16];
            let header_name = <http::header::HeaderName as http::HttpTryFrom<&[u8]>>::try_from(hn_bytes).unwrap();
            let deref_name: &str = header_name.as_ref();
            let _ = deref_name.as_bytes();
            offset = (offset + 16) % GLOBAL_DATA.len();
        }

        let auth_data = &GLOBAL_DATA[offset..offset+64];
        let authority = http::uri::Authority::from_static(_to_str(auth_data, 0, 64));
        println!("Authority: {:?}", authority);
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