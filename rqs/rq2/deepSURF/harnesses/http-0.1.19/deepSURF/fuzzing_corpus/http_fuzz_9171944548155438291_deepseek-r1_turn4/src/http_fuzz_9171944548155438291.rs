#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use http::uri::Authority;
use http::header::Entry;
use http::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 128 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let mut offset = 0;
        let operations = _to_u8(GLOBAL_DATA, offset) % 8;
        offset += 1;

        let mut uri = None;
        let mut header_map = header::HeaderMap::new();
        let mut response_builder = response::Response::builder();

        for _ in 0..operations {
            let op_type = _to_u8(GLOBAL_DATA, offset) % 6;
            offset = (offset + 1) % GLOBAL_DATA.len();

            match op_type {
                0 => {
                    let t_str = _to_str(GLOBAL_DATA, offset, offset + 16);
                    uri = Some(_unwrap_result(Uri::from_str(t_str)));
                    let auth = uri.as_ref().unwrap().authority_part().map(|a| {
                        println!("{:?}", a.host());
                        println!("{:?}", a.port_u16());
                    });
                    let scheme = uri.as_ref().unwrap().scheme_str().map(|s| println!("{}", s));
                }
                1 => {
                    let t_str = _to_str(GLOBAL_DATA, offset, offset + 32);
                    let authority = _unwrap_result(Authority::from_str(t_str));
                    println!("Authority: {:?}", authority.host());
                }
                2 => {
                    let name = _to_str(GLOBAL_DATA, offset, offset + 8);
                    let value = _to_str(GLOBAL_DATA, offset + 8, offset + 16);
                    let hname = _unwrap_result(header::HeaderName::from_bytes(name.as_bytes()));
                    let hvalue = _unwrap_result(header::HeaderValue::from_bytes(value.as_bytes()));
                    header_map.append(hname, hvalue);
                }
                3 => {
                    let mut builder = request::Request::builder();
                    let t_str = _to_str(GLOBAL_DATA, offset, offset + 16);
                    let uri_part = _unwrap_result(Uri::from_str(t_str));
                    builder.uri(uri_part);
                    let method_val = _to_u8(GLOBAL_DATA, offset + 16) % 6;
                    let method = match method_val {
                        0 => Method::GET,
                        1 => Method::POST,
                        2 => Method::PUT,
                        3 => Method::DELETE,
                        4 => Method::HEAD,
                        _ => Method::OPTIONS,
                    };
                    builder.method(method);
                    let _req = builder.body(()).ok().map(|r| println!("{:?}", r.version()));
                }
                4 => {
                    let t_str = _to_str(GLOBAL_DATA, offset, offset + 16);
                    let status = _unwrap_result(StatusCode::from_str(t_str));
                    response_builder.status(status);
                    let version_val = _to_u8(GLOBAL_DATA, offset + 16) % 2;
                    let version = match version_val {
                        0 => Version::HTTP_11,
                        _ => Version::HTTP_10,
                    };
                    response_builder.version(version);
                    let _res = response_builder.body(()).ok().map(|r| println!("{:?}", r.status()));
                }
                5 => {
                    let name = _to_str(GLOBAL_DATA, offset, offset + 8);
                    let hname = _unwrap_result(header::HeaderName::from_bytes(name.as_bytes()));
                    if let Some(val) = header_map.get_mut(&hname) {
                        println!("Header value: {:?}", val.to_str().unwrap());
                    }
                    if let Entry::Occupied(mut entry) = header_map.entry(hname).unwrap() {
                        let drained = entry.insert_mult("drained".parse().unwrap());
                        println!("Drained values: {:?}", drained.count());
                    }
                }
                _ => {}
            }
            offset = (offset + 32) % GLOBAL_DATA.len();
        }

        if let Some(u) = uri {
            let pq = u.path_and_query();
            println!("{:?}", pq.map(|pq| pq.path()));
            let parts = u.into_parts();
            let _rebuilt_uri = _unwrap_result(Uri::from_parts(parts));
        }

        for header in header_map.iter() {
            println!("{:?} {:?}", header.0, header.1.to_str().unwrap());
        }

        let mut drain = header_map.drain();
        while let Some((name, value)) = drain.next() {
            println!("Drained: {:?} {:?}", name, value);
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