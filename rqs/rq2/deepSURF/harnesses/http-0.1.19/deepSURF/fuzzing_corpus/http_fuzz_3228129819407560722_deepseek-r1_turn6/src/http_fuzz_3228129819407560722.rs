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
        let op_count = _to_u8(GLOBAL_DATA, 0) % 5;
        
        let mut header_map = http::header::HeaderMap::new();
        let mut schemes = Vec::new();
        let mut uri_instances = Vec::new();

        for i in 0..op_count {
            let i_usize = i as usize;
            match _to_u8(GLOBAL_DATA, 1 + i_usize) % 6 {
                0 => {
                    let t_str = _to_str(GLOBAL_DATA, i_usize*8, (i_usize+1)*8);
                    let scheme = _unwrap_result(http::uri::Scheme::from_str(t_str));
                    schemes.push(scheme.clone());
                    println!("{:?}", scheme);
                }
                1 => {
                    let bytes = _to_str(GLOBAL_DATA, i_usize*10, (i_usize+1)*10).as_bytes();
                    let auth = _unwrap_result(http::uri::Authority::from_shared(bytes.into()));
                    let uri = _unwrap_result(
                        http::uri::Uri::builder()
                            .authority(auth)
                            .path_and_query("/")
                            .build()
                    );
                    uri_instances.push(uri);
                }
                2 => {
                    let hn = http::header::HeaderName::from_static(_to_str(GLOBAL_DATA, i_usize*5, i_usize*5+5));
                    let hv = http::header::HeaderValue::from_static(_to_str(GLOBAL_DATA, i_usize*6, i_usize*6+6));
                    header_map.insert(hn, hv);
                }
                3 => {
                    let key = http::header::HeaderName::from_static(_to_str(GLOBAL_DATA, i_usize*4, i_usize*4+4));
                    let values = header_map.get_all(&key);
                    let val_iter = values.iter();
                    for v in val_iter {
                        println!("Header: {:?}", v);
                    }
                }
                4 => {
                    let t_bytes = &GLOBAL_DATA[i_usize*12..i_usize*12+12];
                    let scheme = _unwrap_result(http::uri::Scheme::from_shared(t_bytes.into()));
                    let uri = _unwrap_result(http::uri::Uri::from_shared(scheme.into_bytes()));
                    println!("URI: {:?}", uri.scheme_str());
                }
                _ => {
                    let authority = http::uri::Authority::from_static("localhost");
                    let path = http::uri::PathAndQuery::from_static("/");
                    let mut uri_builder = http::uri::Builder::new();
                    uri_builder.authority(authority.as_str());
                    uri_builder.path_and_query(path.as_str());
                    let uri = _unwrap_result(uri_builder.build());
                    uri_instances.push(uri);
                }
            }
        }

        if !schemes.is_empty() {
            let _ = schemes[_to_u8(GLOBAL_DATA, 0) as usize % schemes.len()].as_ref();
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