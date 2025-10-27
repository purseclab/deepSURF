#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use http::{header::{HeaderMap, HeaderName, HeaderValue, Entry}, method::Method, uri::Uri};
use global_data::*;
use std::str::FromStr;

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 2 {
            return;
        }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let mut header_map = if GLOBAL_DATA.len() > 0 && _to_u8(GLOBAL_DATA, 0) % 2 == 0 {
            let capacity = _to_usize(GLOBAL_DATA, 1) % 65;
            HeaderMap::with_capacity(capacity)
        } else {
            HeaderMap::new()
        };

        let mut idx = 2;
        let ops = (_to_u8(GLOBAL_DATA, 1) % 5) as usize;

        for _ in 0..ops {
            if idx + 3 > GLOBAL_DATA.len() {
                break;
            }

            match _to_u8(GLOBAL_DATA, idx) % 7 {
                0 => {
                    if idx + 25 > GLOBAL_DATA.len() {
                        idx += 1;
                        continue;
                    }
                    let name = _to_str(GLOBAL_DATA, idx + 1, idx + 11);
                    let value = _to_str(GLOBAL_DATA, idx + 11, idx + 21);
                    if let (Ok(n), Ok(v)) = (HeaderName::from_str(name), HeaderValue::from_str(value)) {
                        header_map.insert(n, v);
                    }
                    idx += 21;
                }
                1 => {
                    if idx + 11 > GLOBAL_DATA.len() {
                        idx += 1;
                        continue;
                    }
                    let name = _to_str(GLOBAL_DATA, idx + 1, idx + 11);
                    if let Ok(n) = HeaderName::from_str(name) {
                        let _ = header_map.get_mut(&n).map(|v| *v = HeaderValue::from_static("modified"));
                    }
                    idx += 11;
                }
                2 => {
                    let mut iter = header_map.iter_mut();
                    while let Some((_, v)) = iter.next() {
                        *v = HeaderValue::from_static("mutated");
                    }
                    idx += 1;
                }
                3 => {
                    if idx + 11 > GLOBAL_DATA.len() {
                        idx += 1;
                        continue;
                    }
                    let name = _to_str(GLOBAL_DATA, idx + 1, idx + 11);
                    if let Ok(entry) = header_map.entry(name) {
                        match entry {
                            Entry::Occupied(mut o) => {
                                o.insert(HeaderValue::from_static("occupied"));
                            }
                            Entry::Vacant(v) => {
                                v.insert(HeaderValue::from_static("vacant"));
                            }
                        }
                    }
                    idx += 11;
                }
                4 => {
                    let _ = header_map.append("x-fuzz-header", HeaderValue::from_static("appended"));
                    idx += 1;
                }
                5 => {
                    let mut drain = header_map.drain();
                    while let Some(_) = drain.next() {}
                    idx += 1;
                }
                _ => {
                    if idx + 11 > GLOBAL_DATA.len() {
                        idx += 1;
                        continue;
                    }
                    let name = _to_str(GLOBAL_DATA, idx + 1, idx + 11);
                    let _ = header_map.remove(name);
                    idx += 11;
                }
            }
        }

        let mut values = header_map.values();
        values.next();
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