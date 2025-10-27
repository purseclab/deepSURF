#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use http::*;
use http::header::HeaderName;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 256 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let mut offset = 0;
        let uri_choice = _to_u8(GLOBAL_DATA, offset) % 2;
        offset += 1;

        let uri = match uri_choice {
            0 => {
                let len = _to_u8(GLOBAL_DATA, offset) as usize % 32;
                offset += 1;
                let s = _to_str(GLOBAL_DATA, offset, offset + len);
                offset += len;
                Uri::from_static(s)
            }
            _ => {
                let len = _to_u8(GLOBAL_DATA, offset) as usize % 32;
                offset += 1;
                let bytes = &GLOBAL_DATA[offset..offset + len];
                offset += len;
                _unwrap_result(Uri::from_shared(bytes.into()))
            }
        };

        let header_count = _to_u8(GLOBAL_DATA, offset) % 65;
        offset += 1;

        let mut headers = HeaderMap::new();
        for _ in 0..header_count {
            let name_len = _to_u8(GLOBAL_DATA, offset) as usize % 16;
            offset += 1;
            let name_str = _to_str(GLOBAL_DATA, offset, offset + name_len);
            offset += name_len;

            let value_len = _to_u8(GLOBAL_DATA, offset) as usize % 16;
            offset += 1;
            let value_str = _to_str(GLOBAL_DATA, offset, offset + value_len);
            offset += value_len;

            let name = HeaderName::from_static(name_str);
            let value = HeaderValue::from_static(value_str);
            headers.insert(name, value);
        }

        let method_choice = _to_u8(GLOBAL_DATA, offset) % 8;
        offset += 1;

        let mut builder = match method_choice {
            0 => Request::get(&uri),
            1 => Request::post(&uri),
            2 => Request::put(&uri),
            3 => Request::delete(&uri),
            4 => Request::head(&uri),
            5 => Request::connect(&uri),
            6 => Request::options(&uri),
            _ => Request::patch(&uri),
        };

        for (name, value) in headers.iter() {
            builder.header(name, value);
            println!("Header: {:?} => {:?}", name.as_str(), value.to_str());
        }

        let version_choice = _to_u8(GLOBAL_DATA, offset) % 2;
        offset += 1;
        let version = match version_choice {
            0 => Version::HTTP_11,
            _ => Version::HTTP_2,
        };
        builder.version(version);

        let request = _unwrap_result(builder.body(()));
        println!("Final request: {:?} {:?}", request.method(), request.uri());
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