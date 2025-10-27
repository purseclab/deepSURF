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

        let ops_count = _to_u8(GLOBAL_DATA, 0) % 8;
        let mut offset = 1;

        for _ in 0..ops_count {
            let op_selector = _to_u8(GLOBAL_DATA, offset) % 7;
            offset = (offset + 1) % 64;

            match op_selector {
                0 => {
                    let len1 = _to_u8(GLOBAL_DATA, offset) % 32;
                    offset = (offset + 1) % 64;
                    let s1 = _to_str(GLOBAL_DATA, offset, offset + len1 as usize);
                    let pq1 = http::uri::PathAndQuery::from_str(s1);
                    let pq1 = _unwrap_result(pq1);
                    
                    let len2 = _to_u8(GLOBAL_DATA, offset + len1 as usize) % 32;
                    let s2 = _to_str(GLOBAL_DATA, offset + len1 as usize + 1, offset + len1 as usize + 1 + len2 as usize);
                    let pq2 = http::uri::PathAndQuery::from_str(s2);
                    let pq2 = _unwrap_result(pq2);
                    
                    pq1.partial_cmp(&pq2);
                    println!("{:?} {:?}", pq1.as_str(), pq2.as_str());
                }
                1 => {
                    let auth_len = _to_u8(GLOBAL_DATA, offset) % 32;
                    offset = (offset + 1) % 64;
                    let auth_str = _to_str(GLOBAL_DATA, offset, offset + auth_len as usize);
                    let authority = http::uri::Authority::from_static(auth_str);
                    let uri = http::uri::Uri::builder().authority(authority).build().unwrap();
                    let pq = uri.path_and_query().unwrap();
                    pq.partial_cmp(uri.path());
                }
                2 => {
                    let mut map = http::header::HeaderMap::new();
                    let name = http::header::HeaderName::from_static("X-Fuzzed");
                    let val = http::header::HeaderValue::from_static(_to_str(GLOBAL_DATA, offset, offset + 8));
                    map.insert(name.clone(), val);
                    println!("{:?}", map.get(&name).unwrap().to_str().unwrap());
                }
                3 => {
                    let builder = http::uri::Uri::builder()
                        .path_and_query(_to_str(GLOBAL_DATA, offset, offset + 16))
                        .build();
                    let uri = _unwrap_result(builder);
                    let uri_clone = uri.clone();
                    let parts = uri.into_parts();
                    let new_uri = http::uri::Uri::from_parts(parts).unwrap();
                    new_uri.to_string().partial_cmp(&uri_clone.to_string());
                }
                4 => {
                    let bytes = &GLOBAL_DATA[offset..offset + 16];
                    let pq = <http::uri::PathAndQuery as http::HttpTryFrom<&[u8]>>::try_from(bytes);
                    if let Ok(pq) = pq {
                        let s = pq.as_str();
                        println!("{}", s);
                        pq.partial_cmp(s);
                    }
                }
                5 => {
                    let auth = http::uri::Authority::from_static(_to_str(GLOBAL_DATA, offset, offset + 16));
                    let uri = http::uri::Uri::builder()
                        .authority(auth.as_str())
                        .build().unwrap();
                    uri.authority().unwrap().partial_cmp(&auth);
                }
                6 => {
                    let method_bytes = &GLOBAL_DATA[offset..offset + 8];
                    let method = http::method::Method::from_bytes(method_bytes);
                    if let Ok(m) = method {
                        let auth = http::uri::Authority::from_static(_to_str(GLOBAL_DATA, offset + 8, offset + 24));
                        let uri = http::uri::Uri::builder()
                            .authority(auth)
                            .path_and_query(m.as_ref())
                            .build()
                            .unwrap();
                        let req = http::Request::builder()
                            .method(m)
                            .uri(uri)
                            .body(())
                            .unwrap();
                        println!("{} {}", req.method(), req.uri());
                    }
                }
                _ => {}
            }
            offset = (offset + 16) % 64;
        }

        let t1_len = _to_u8(GLOBAL_DATA, 200) % 32;
        let t1 = _to_str(GLOBAL_DATA, 200, 200 + t1_len as usize);
        let t2 = _to_str(GLOBAL_DATA, 200 + t1_len as usize, 200 + t1_len as usize + 32);
        let pq_target = _unwrap_result(http::uri::PathAndQuery::from_str(t2));
        t1.partial_cmp(&pq_target);
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