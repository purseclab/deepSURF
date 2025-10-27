#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use stackvector::*;
use global_data::*;
use std::ops::{Deref, DerefMut};

#[derive(Clone)]
struct CustomType1(String);
struct CustomType2(String);
struct CustomType3(String);

impl std::iter::IntoIterator for CustomType2 {
    type Item = CustomType1;
    type IntoIter = CustomType3;

    fn into_iter(self) -> Self::IntoIter {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let selector = _to_usize(GLOBAL_DATA, 0) % 3;
        if selector == 0 { panic!("INTENTIONAL PANIC!"); }
        let len = _to_u8(GLOBAL_DATA, 1) % 17;
        let s = _to_str(GLOBAL_DATA, 2, 2 + len as usize);
        CustomType3(String::from(s))
    }
}

impl std::iter::Iterator for CustomType3 {
    type Item = CustomType1;

    fn next(&mut self) -> Option<Self::Item> {
        let global_data = get_global_data();
        let len = _to_u8(global_data.second_half, 0) % 17;
        let s = _to_str(global_data.second_half, 1, 1 + len as usize);
        Some(CustomType1(String::from(s)))
    }
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 1024 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let first_half = global_data.first_half;
        let op_count = _to_u8(first_half, 0) % 4 + 1;

        let mut idx = 1;
        for _ in 0..op_count {
            let ctor_selector = _to_u8(first_half, idx) % 3;
            idx += 1;

            match ctor_selector {
                0 => {
                    let str_len = _to_u8(first_half, idx) % 64;
                    idx += 1;
                    let s = _to_str(first_half, idx, idx + str_len as usize);
                    idx += str_len as usize;
                    let mut vec = StackVec::<[CustomType1; 64]>::from_iter(CustomType2(s.to_string()));
                    let _ = vec.drain();
                    if !vec.is_empty() {
                        vec.swap_remove(0);
                    }
                    vec.into_vec();
                },
                1 => {
                    let elem_count = _to_u8(first_half, idx) % 64;
                    idx += 1;
                    let s = _to_str(first_half, idx, idx + 8);
                    idx += 8;
                    let elem = CustomType1(s.to_string());
                    let mut vec = StackVec::<[CustomType1; 64]>::from_elem(elem, elem_count as usize);
                    if vec.len() > 0 {
                        println!("{:?}", &vec[0].0);
                        vec.swap_remove(0);
                        vec.push(CustomType1(String::new()));
                    }
                    vec.into_vec();
                },
                2 => {
                    let slice_len = _to_u8(first_half, idx) % 64;
                    idx += 1;
                    let mut elements = Vec::new();
                    for _ in 0..slice_len {
                        let str_len = _to_u8(first_half, idx) % 16;
                        idx += 1;
                        let s = _to_str(first_half, idx, idx + str_len as usize);
                        idx += str_len as usize;
                        elements.push(CustomType1(s.to_string()));
                    }
                    let mut vec = StackVec::<[CustomType1; 64]>::from_vec(elements);
                    vec.push(CustomType1(String::new()));
                    let _ = vec.pop();
                    vec.into_vec();
                },
                _ => {
                    let mut vec = StackVec::<[CustomType1; 64]>::new();
                    let ops = _to_u8(first_half, idx) % 4;
                    idx += 1;
                    for _ in 0..ops {
                        let str_len = _to_u8(first_half, idx) % 16;
                        idx += 1;
                        let s = _to_str(first_half, idx, idx + str_len as usize);
                        idx += str_len as usize;
                        vec.insert(0, CustomType1(s.to_string()));
                    }
                    if !vec.is_empty() {
                        let _ = vec.get(0);
                    }
                    vec.into_vec();
                }
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