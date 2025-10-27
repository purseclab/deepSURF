#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use http::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

struct CustomType0(String);

fn main (){
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 150 {return;}
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let ops = _to_usize(GLOBAL_DATA, 0) % 8;
        
        let mut header_map = header::HeaderMap::with_capacity(32);
        let mut uri_builder = uri::Builder::new();
        let mut req_builder = request::Request::builder();
        
        for i in 0..ops {
            let offset = i * 8;
            let op_selector = _to_u8(GLOBAL_DATA, offset) % 6;
            
            match op_selector {
                0 => {
                    let name = _to_str(GLOBAL_DATA, offset + 1, offset + 9);
                    let header_name = header::HeaderName::from_static(name);
                    let header_value = header::HeaderValue::from_static(name);
                    header_map.insert(header_name, header_value);
                }
                1 => {
                    let num_val = _to_u64(GLOBAL_DATA, offset);
                    let header_value = match num_val % 3 {
                        0 => header::HeaderValue::from(num_val as u32),
                        1 => header::HeaderValue::from(num_val as i64),
                        _ => header::HeaderValue::from(num_val as usize),
                    };
                    let _ = header_value.to_str();
                }
                2 => {
                    let uri_data = _to_str(GLOBAL_DATA, offset, offset + 16);
                    if let Ok(uri) = uri::Uri::from_str(uri_data) {
                        let _ = uri.path();
                        let _ = uri::Uri::builder().path_and_query(uri_data);
                    }
                }
                3 => {
                    let method_bytes = &GLOBAL_DATA[offset..offset+8];
                    if let Ok(method) = method::Method::from_bytes(method_bytes) {
                        req_builder.method(method);
                    }
                }
                4 => {
                    let idx = _to_usize(GLOBAL_DATA, offset) % 32;
                    let hv = header::HeaderValue::from_static("value");
                    let hn = header::HeaderName::from_static("name");
                    println!("{:?}", header_map.get(&hn));
                    header_map.append(hn, hv);
                }
                5 => {
                    let t1 = header::HeaderValue::from(ops);
                    let t2 = header::HeaderValue::from_static("static_val");
                    let t3 = header::HeaderValue::from_bytes(b"bytes").unwrap();
                    t1.eq(&t2);
                    t2.eq(&t3);
                    t3.eq(&t1);
                }
                _ => {}
            }
        }
        
        let t_0 = _to_usize(GLOBAL_DATA, 80);
        let final_val = header::HeaderValue::from(t_0);
        let custom_str = _to_str(GLOBAL_DATA, 90, 100);
        let custom_val = CustomType0(custom_str.to_string());
        let _ = final_val.eq(&&custom_val.0);
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