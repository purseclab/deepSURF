#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use http::*;
use global_data::*;
use std::str::FromStr;
use http::header::{HeaderMap, HeaderName, HeaderValue};

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 1024 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let mut offset = 0;
        let num_ops = _to_u8(GLOBAL_DATA, offset) % 10;
        offset += 1;

        let mut paths = vec![];
        let mut authorities = vec![];
        let mut header_map = HeaderMap::new();
        let mut uri_builder = http::uri::Builder::new();

        for _ in 0..num_ops {
            let op_selector = _to_u8(GLOBAL_DATA, offset) % 6;
            offset += 1;

            match op_selector {
                0 => {
                    let len = _to_u8(GLOBAL_DATA, offset) % 65;
                    offset += 1;
                    let s = _to_str(GLOBAL_DATA, offset, offset + len as usize);
                    offset += len as usize;
                    let pq = http::uri::PathAndQuery::from_str(s);
                    let pq = _unwrap_result(pq);
                    paths.push(pq);
                }
                1 => {
                    let len = _to_u8(GLOBAL_DATA, offset) % 65;
                    offset += 1;
                    let s = _to_str(GLOBAL_DATA, offset, offset + len as usize);
                    offset += len as usize;
                    let auth = http::uri::Authority::from_str(s);
                    let auth = _unwrap_result(auth);
                    authorities.push(auth);
                }
                2 => {
                    let len = _to_u8(GLOBAL_DATA, offset) % 65;
                    offset += 1;
                    let s = _to_str(GLOBAL_DATA, offset, offset + len as usize);
                    offset += len as usize;
                    uri_builder.path_and_query(s);
                }
                3 => {
                    let name_len = _to_u8(GLOBAL_DATA, offset) % 65;
                    offset += 1;
                    let name = _to_str(GLOBAL_DATA, offset, offset + name_len as usize);
                    offset += name_len as usize;
                    let value_len = _to_u8(GLOBAL_DATA, offset) % 65;
                    offset += 1;
                    let value = _to_str(GLOBAL_DATA, offset, offset + value_len as usize);
                    offset += value_len as usize;

                    let header_name = HeaderName::from_static(name);
                    let header_value = HeaderValue::from_static(value);
                    header_map.append(header_name, header_value);
                }
                4 => {
                    let len = _to_u8(GLOBAL_DATA, offset) % 65;
                    offset += 1;
                    let s = _to_str(GLOBAL_DATA, offset, offset + len as usize);
                    offset += len as usize;
                    let auth = http::uri::Authority::from_str(s);
                    let auth = _unwrap_result(auth);
                    uri_builder.authority(auth.as_str());
                }
                5 => {
                    let len = _to_u8(GLOBAL_DATA, offset) % 65;
                    offset += 1;
                    let s = _to_str(GLOBAL_DATA, offset, offset + len as usize);
                    offset += len as usize;
                    let pq = http::uri::PathAndQuery::from_static(s);
                    paths.push(pq);
                }
                _ => unreachable!()
            }
        }

        let built_uri = uri_builder.build();
        let uri = _unwrap_result(built_uri);
        let uri_pq = uri.path_and_query().unwrap();

        let t_len1 = _to_u8(GLOBAL_DATA, offset) % 65;
        offset += 1;
        let t_str1 = _to_str(GLOBAL_DATA, offset, offset + t_len1 as usize);
        offset += t_len1 as usize;
        let t_len2 = _to_u8(GLOBAL_DATA, offset) % 65;
        offset += 1;
        let t_str2 = _to_str(GLOBAL_DATA, offset, offset + t_len2 as usize);

        let mut s1 = String::from(t_str1);
        let s2 = String::from(t_str2);

        for pq in &paths {
            s1.partial_cmp(pq);
            pq.partial_cmp(&s2);
        }

        uri_pq.partial_cmp(&s1);
        s2.partial_cmp(uri_pq);

        if let Some(auth) = authorities.first() {
            println!("{:?}", auth.as_str());
            let _ = auth.port_u16();
        }

        for (name, value) in header_map.iter() {
            println!("{:?}: {:?}", name.as_str(), value.to_str().unwrap());
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