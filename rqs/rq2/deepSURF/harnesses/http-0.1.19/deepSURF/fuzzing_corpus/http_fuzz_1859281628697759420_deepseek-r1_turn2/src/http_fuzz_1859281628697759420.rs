#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use http::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 512 {
            return;
        }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let mut pos = 0;
        let num_ops = _to_u8(GLOBAL_DATA, pos) % 5;
        pos += 1;

        let mut header_map = http::header::HeaderMap::new();
        let mut authority = None;
        let mut uri = None;
        let mut path_queries = vec![];

        for _ in 0..num_ops {
            let op = _to_u8(GLOBAL_DATA, pos) % 4;
            pos += 1;

            match op {
                0 => {
                    let len = _to_u8(GLOBAL_DATA, pos) % 17;
                    pos += 1;
                    let end = pos + len as usize;
                    if end >= GLOBAL_DATA.len() {
                        break;
                    }
                    let name = _to_str(GLOBAL_DATA, pos, end);
                    let name = http::header::HeaderName::from_static(name);
                    pos = end;

                    let v_len = _to_u8(GLOBAL_DATA, pos) % 17;
                    pos += 1;
                    let v_end = pos + v_len as usize;
                    if v_end >= GLOBAL_DATA.len() {
                        break;
                    }
                    let value = _to_str(GLOBAL_DATA, pos, v_end);
                    let value = http::header::HeaderValue::from_static(value);
                    pos = v_end;

                    header_map.append(&name, value);
                    println!("{:?}", header_map);
                }
                1 => {
                    let len = _to_u8(GLOBAL_DATA, pos) % 17;
                    pos += 1;
                    let end = pos + len as usize;
                    if end >= GLOBAL_DATA.len() {
                        break;
                    }
                    let auth_str = _to_str(GLOBAL_DATA, pos, end);
                    authority = Some(http::uri::Authority::from_static(auth_str));
                    pos = end;
                    println!("{:?}", authority.as_ref().map(|a: &http::uri::Authority| a.as_str()));
                }
                2 => {
                    let len = _to_u8(GLOBAL_DATA, pos) % 17;
                    pos += 1;
                    let end = pos + len as usize;
                    if end >= GLOBAL_DATA.len() {
                        break;
                    }
                    let uri_str = _to_str(GLOBAL_DATA, pos, end);
                    uri = Some(http::uri::Uri::from_static(uri_str));
                    pos = end;
                    println!("{:?}", uri.as_ref().map(|u: &http::uri::Uri| u.path()));
                }
                3 => {
                    let len = _to_u8(GLOBAL_DATA, pos) % 17;
                    pos += 1;
                    let end = pos + len as usize;
                    if end >= GLOBAL_DATA.len() {
                        break;
                    }
                    let path_str = _to_str(GLOBAL_DATA, pos, end);
                    let pq = http::uri::PathAndQuery::from_str(path_str);
                    if let Ok(pq) = pq {
                        path_queries.push(pq);
                    }
                    pos = end;
                }
                _ => {}
            }
        }

        let mut t0 = _to_u8(GLOBAL_DATA, pos) % 17;
        pos += 1;
        let t1_start = pos;
        let t1_end = t1_start + t0 as usize;
        if t1_end >= GLOBAL_DATA.len() {
            return;
        }
        let t1 = _to_str(GLOBAL_DATA, t1_start, t1_end);
        let t2 = &t1;

        pos = t1_end;
        t0 = _to_u8(GLOBAL_DATA, pos) % 17;
        pos += 1;
        let t3_start = pos;
        let t3_end = t3_start + t0 as usize;
        if t3_end >= GLOBAL_DATA.len() {
            return;
        }
        let t4 = _to_str(GLOBAL_DATA, t3_start, t3_end);
        let t5 = http::uri::PathAndQuery::from_str(t4);
        let t6 = _unwrap_result(t5);
        let t7 = &t6;

        t2.eq(t7);

        if let Some(auth) = authority {
            let _port = auth.port_u16();
        }
        if let Some(uri) = uri {
            let _path = uri.path();
        }
        for pq in path_queries {
            println!("{:?}", pq.as_str());
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