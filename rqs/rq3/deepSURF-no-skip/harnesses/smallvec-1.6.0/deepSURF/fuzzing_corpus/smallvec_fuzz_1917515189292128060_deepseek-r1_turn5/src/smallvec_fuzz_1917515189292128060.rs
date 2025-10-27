#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 16384 {return;}
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let mut pos = 0;
        let num_strings = _to_u8(GLOBAL_DATA, pos) % 50;
        pos += 1;

        let mut elements = Vec::new();
        for _ in 0..num_strings {
            if pos + 1 >= GLOBAL_DATA.len() { break; }
            let len = _to_u8(GLOBAL_DATA, pos) as usize % 65;
            pos += 1;
            let end = pos + len;
            if end > GLOBAL_DATA.len() { break; }
            let s = _to_str(GLOBAL_DATA, pos, end);
            elements.push(String::from(s));
            pos = end;
        }

        if pos + 8 >= GLOBAL_DATA.len() { return; }
        let constructor_choice = _to_u8(GLOBAL_DATA, pos) % 4;
        pos += 1;
        
        let mut sv = match constructor_choice {
            0 => SmallVec::<[String; 16]>::with_capacity(_to_usize(GLOBAL_DATA, pos) % 65),
            1 => SmallVec::<[String; 16]>::from_vec(elements),
            2 => {
                let elem = elements.get(0).cloned().unwrap_or_default();
                SmallVec::<[String; 16]>::from_elem(elem, _to_usize(GLOBAL_DATA, pos) % 65)
            }
            3 => SmallVec::<[String; 16]>::from_iter(elements.into_iter()),
            _ => SmallVec::<[String; 16]>::new()
        };

        pos += 8;
        let num_ops = _to_u8(GLOBAL_DATA, pos) % 100;
        pos += 1;

        for _ in 0..num_ops {
            if pos >= GLOBAL_DATA.len() { break; }
            let op = _to_u8(GLOBAL_DATA, pos) % 9;
            pos += 1;

            match op {
                0 => {
                    sv.push(String::from(""));
                }
                1 => {
                    sv.pop();
                }
                2 => {
                    if !sv.is_empty() {
                        let idx = _to_usize(GLOBAL_DATA, pos) % sv.len();
                        sv.remove(idx);
                    }
                }
                3 => {
                    sv.truncate(_to_usize(GLOBAL_DATA, pos) % 65);
                }
                4 => {
                    if !sv.is_empty() {
                        let idx = _to_usize(GLOBAL_DATA, pos) % sv.len();
                        let elem = sv.get(idx).map(|s: &String| s.as_str()).unwrap_or("");
                        println!("Element {}: {}", idx, elem);
                    }
                }
                5 => {
                    sv.clear();
                }
                6 => {
                    let len = sv.len();
                    if len > 0 {
                        sv.swap_remove(_to_usize(GLOBAL_DATA, pos) % len);
                    }
                }
                7 => {
                    let new_len = _to_usize(GLOBAL_DATA, pos) % 65;
                    sv.resize_with(new_len, || String::new());
                }
                8 => {
                    let cap = _to_usize(GLOBAL_DATA, pos);
                    sv.reserve(cap);
                }
                _ => {}
            }
            pos = pos.wrapping_add(8);
        }

        let _ = sv.clone().into_vec();
        let _ = sv.clone().into_boxed_slice();
        let _ = sv.into_iter();
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