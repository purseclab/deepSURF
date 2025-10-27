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
        let ops = _to_u8(GLOBAL_DATA, offset) % 5;
        offset += 1;

        let mut paths: Vec<http::uri::PathAndQuery> = Vec::new();
        let mut authorities = Vec::new();
        let mut header_map = header::HeaderMap::new();

        for _ in 0..ops+1 {
            let constructor_choice = _to_u8(GLOBAL_DATA, offset) % 3;
            offset += 1;

            let str_len = _to_u8(GLOBAL_DATA, offset) % 65;
            offset += 1;
            let start = offset;
            offset += str_len as usize;
            let s = _to_str(GLOBAL_DATA, start, offset);

            let path = match constructor_choice {
                0 => http::uri::PathAndQuery::from_str(s),
                1 => Ok(http::uri::PathAndQuery::from_static(s)),
                2 => <http::uri::PathAndQuery as http::HttpTryFrom<&[u8]>>::try_from(s.as_bytes()),
                _ => unreachable!()
            };
            paths.push(_unwrap_result(path));

            let auth = http::uri::Authority::from_str(s);
            if let Ok(a) = auth {
                authorities.push(a);
            }

            let name = _to_str(GLOBAL_DATA, offset, offset + 5);
            offset += 5;
            let val = _to_str(GLOBAL_DATA, offset, offset + 10);
            offset += 10;

            if let Ok(hn) = header::HeaderName::from_bytes(name.as_bytes()) {
                if let Ok(hv) = header::HeaderValue::from_bytes(val.as_bytes()) {
                    header_map.append(hn, hv);
                }
            }
        }

        let mut builder = http::request::Request::builder();
        for (i, path) in paths.iter().enumerate() {
            if i == 0 {
                let _ = builder.uri(path.as_str());
            }
            println!("Path: {:?}", path);
        }

        let uri_from_parts = http::uri::Uri::from_parts({
            let base_uri = http::Uri::from_static("/");
            let mut parts = base_uri.into_parts();
            parts.path_and_query = Some(paths[0].clone());
            parts
        });
        if let Ok(uri) = uri_from_parts {
            let _ = builder.body(uri);
        }

        for auth in &authorities {
            let port = auth.port_u16();
            println!("Authority port: {:?}", port);
        }

        let cmp_choice = _to_u8(GLOBAL_DATA, offset) % 2;
        offset += 1;
        match cmp_choice {
            0 if paths.len() > 1 => paths[0].eq(paths[1].as_str()),
            1 if !authorities.is_empty() => paths[0].eq(authorities[0].as_str()),
            _ => paths[0].eq("")
        };
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