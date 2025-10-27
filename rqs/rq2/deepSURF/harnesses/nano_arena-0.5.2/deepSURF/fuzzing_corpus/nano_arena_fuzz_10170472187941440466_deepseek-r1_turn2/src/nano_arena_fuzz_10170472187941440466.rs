#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use nano_arena::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

#[derive(Debug)]
struct CustomType0(String);

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 1024 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        
        let constructor_selector = _to_u8(GLOBAL_DATA, 0) % 3;
        let mut arena = match constructor_selector {
            0 => Arena::new(),
            1 => Arena::with_capacity(_to_usize(GLOBAL_DATA, 1)),
            2 => {
                let count = _to_usize(GLOBAL_DATA, 1) % 65;
                let mut items = Vec::with_capacity(count);
                let mut offset = 9;
                for _ in 0..count {
                    if offset + 2 > GLOBAL_DATA.len() { break; }
                    let len = _to_u8(GLOBAL_DATA, offset) as usize % 65;
                    offset += 1;
                    let s = _to_str(GLOBAL_DATA, offset, offset + len);
                    items.push(CustomType0(s.to_string()));
                    offset += len;
                }
                Arena::from_iter(items)
            }
            _ => unreachable!(),
        };

        let mut indices = Vec::new();
        let mut offset = 33;
        let ops = _to_u8(GLOBAL_DATA, 32) % 65;

        for _ in 0..ops {
            if offset + 4 > GLOBAL_DATA.len() { break; }
            let op = _to_u8(GLOBAL_DATA, offset) % 6;
            offset += 1;

            match op {
                0 => {
                    let len = _to_u8(GLOBAL_DATA, offset) as usize % 65;
                    offset += 1;
                    let s = _to_str(GLOBAL_DATA, offset, offset + len);
                    let idx = arena.insert(CustomType0(s.to_string()));
                    indices.push(idx);
                    offset += len;
                }
                1 => {
                    if !indices.is_empty() {
                        let idx_pos = _to_usize(GLOBAL_DATA, offset) % indices.len();
                        let idx = &indices[idx_pos];
                        if let Some((val, split)) = arena.split_at(idx) {
                            println!("{:?}", val);
                            if !indices.is_empty() {
                                let idx = &indices[_to_usize(GLOBAL_DATA, offset) % indices.len()];
                                if let Some(sval) = split.get(idx) {
                                    println!("{:?}", sval);
                                }
                            }
                        }
                    }
                    offset += 8;
                }
                2 => {
                    if !indices.is_empty() {
                        let idx_pos = _to_usize(GLOBAL_DATA, offset) % indices.len();
                        let idx = &indices[idx_pos];
                        if let Some(val) = arena.get(idx) {
                            println!("{:?}", val);
                        }
                    }
                    offset += 8;
                }
                3 => {
                    if !indices.is_empty() {
                        let idx_pos = _to_usize(GLOBAL_DATA, offset) % indices.len();
                        let idx = indices.remove(idx_pos);
                        let _ = arena.remove(idx);
                    }
                    offset += 8;
                }
                4 => {
                    if indices.len() >= 2 {
                        let a = _to_usize(GLOBAL_DATA, offset) % indices.len();
                        let b = _to_usize(GLOBAL_DATA, offset + 8) % indices.len();
                        arena.swap(&indices[a], &indices[b]);
                    }
                    offset += 16;
                }
                5 => {
                    let new_cap = _to_usize(GLOBAL_DATA, offset);
                    arena.truncate(new_cap);
                    offset += 8;
                }
                _ => {}
            }
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