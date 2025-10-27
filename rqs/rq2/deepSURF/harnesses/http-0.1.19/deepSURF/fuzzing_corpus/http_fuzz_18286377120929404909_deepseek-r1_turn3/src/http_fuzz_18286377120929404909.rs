#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use http::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

fn main (){
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 1024 {return;}
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        
        let op_count = _to_u8(GLOBAL_DATA, 0) % 10;
        let constructor_choice = _to_u8(GLOBAL_DATA, 1) % 2;
        let capacity = _to_usize(GLOBAL_DATA, 2) % 65;
        
        let mut header_map = match constructor_choice {
            0 => header::HeaderMap::with_capacity(0),
            _ => header::HeaderMap::with_capacity(capacity),
        };

        let mut offset = 3;
        for _ in 0..op_count {
            if offset >= GLOBAL_DATA.len() { break; }
            let op_byte = _to_u8(GLOBAL_DATA, offset);
            offset += 1;

            match op_byte % 7 {
                0 => {
                    if offset + 2 >= GLOBAL_DATA.len() { break; }
                    let name_len = _to_u8(GLOBAL_DATA, offset) as usize % 32;
                    offset += 1;
                    let name_bytes = &GLOBAL_DATA[offset..(offset + name_len).min(GLOBAL_DATA.len())];
                    offset += name_len;
                    let value_len = _to_u8(GLOBAL_DATA, offset) as usize % 32;
                    offset += 1;
                    let value_bytes = &GLOBAL_DATA[offset..(offset + value_len).min(GLOBAL_DATA.len())];
                    offset += value_len;
                    
                    if let Ok(name) = header::HeaderName::from_bytes(name_bytes) {
                        if let Ok(value) = header::HeaderValue::from_bytes(value_bytes) {
                            header_map.insert(name, value);
                        }
                    }
                }
                1 => {
                    if offset + 1 >= GLOBAL_DATA.len() { break; }
                    let name_len = _to_u8(GLOBAL_DATA, offset) as usize % 32;
                    offset += 1;
                    let name_bytes = &GLOBAL_DATA[offset..(offset + name_len).min(GLOBAL_DATA.len())];
                    offset += name_len;
                    
                    if let Ok(name) = header::HeaderName::from_bytes(name_bytes) {
                        header_map.remove(name);
                    }
                }
                2 => {
                    let mut iter = header_map.iter_mut();
                    let _ = iter.size_hint();
                    println!("{:?}", iter.next());
                }
                3 => {
                    let mut values = header_map.values_mut();
                    let _ = values.size_hint();
                    println!("{:?}", values.next());
                }
                4 => {
                    let mut drain = header_map.drain();
                    let _ = drain.size_hint();
                    for _ in drain {}
                }
                5 => {
                    if offset + 1 >= GLOBAL_DATA.len() { break; }
                    let name_len = _to_u8(GLOBAL_DATA, offset) as usize % 32;
                    offset += 1;
                    let name_bytes = &GLOBAL_DATA[offset..(offset + name_len).min(GLOBAL_DATA.len())];
                    offset += name_len;
                    
                    if let Ok(entry) = header_map.entry(header::HeaderName::from_bytes(name_bytes).unwrap_or_else(|_| std::process::exit(0))) {
                        match entry {
                            header::Entry::Occupied(mut o) => { o.remove(); },
                            header::Entry::Vacant(v) => {
                                let value_len = _to_u8(GLOBAL_DATA, offset) as usize % 32;
                                offset += 1;
                                let value_bytes = &GLOBAL_DATA[offset..(offset + value_len).min(GLOBAL_DATA.len())];
                                offset += value_len;
                                v.insert(header::HeaderValue::from_bytes(value_bytes).unwrap_or_else(|_| std::process::exit(0)));
                            }
                        }
                    }
                }
                6 => {
                    let hm: std::collections::HashMap<_, _> = header_map.iter()
                        .map(|(k, v)| (k.clone(), v.clone()))
                        .collect();
                    let _ = <header::HeaderMap<header::HeaderValue> as HttpTryFrom<_>>::try_from(&hm);
                }
                _ => {}
            }
        }

        let mut target_iter = header_map.iter_mut();
        target_iter.size_hint();

        let _ = Request::builder();
        let _ = Response::builder();
        match _to_u8(GLOBAL_DATA, offset) % 4 {
            0 => { let _ = header::HeaderValue::from_static("test"); }
            1 => { let _ = header::HeaderName::from_bytes(&GLOBAL_DATA[offset..(offset+4).min(GLOBAL_DATA.len())]); }
            2 => { let _ = Method::GET; }
            _ => { let _ = Version::HTTP_11; }
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