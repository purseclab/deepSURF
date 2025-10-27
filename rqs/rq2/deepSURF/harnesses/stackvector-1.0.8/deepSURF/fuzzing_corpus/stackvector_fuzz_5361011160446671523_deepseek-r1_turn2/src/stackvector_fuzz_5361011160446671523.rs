#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use stackvector::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

#[derive(Debug, Clone)]
struct CustomType1(String);
struct CustomType2(String);
struct CustomType3(String);
type StackArray = [CustomType1; 128];

impl std::iter::IntoIterator for CustomType2 {
    type Item = CustomType1;
    type IntoIter = CustomType3;
    
    fn into_iter(self) -> Self::IntoIter {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let len = _to_usize(GLOBAL_DATA, 90) % 17;
        let start = _to_usize(GLOBAL_DATA, 98) % (GLOBAL_DATA.len().saturating_sub(len));
        let s = _to_str(&global_data.second_half, start, start + len);
        CustomType3(s.to_string())
    }
}

impl std::iter::Iterator for CustomType3 {
    type Item = CustomType1;
    
    fn next(&mut self) -> Option<Self::Item> {
        let global_data = get_global_data();
        let len = _to_usize(global_data.first_half, 110) % 11;
        let start = _to_usize(global_data.second_half, 120) % (global_data.second_half.len().saturating_sub(len));
        let s = _to_str(global_data.second_half, start, start + len);
        Some(CustomType1(s.to_string()))
    }
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 1024 {return;}
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        
        let mut stack_vec = match _to_u8(GLOBAL_DATA, 0) % 3 {
            0 => StackVec::<StackArray>::new(),
            1 => {
                let slice_len = _to_usize(GLOBAL_DATA, 10) % 16;
                let items: Vec<CustomType1> = (0..slice_len).map(|i| {
                    CustomType1(_to_str(GLOBAL_DATA, 20 + i*4, 24 + i*4).to_string())
                }).collect();
                StackVec::from_vec(items)
            },
            _ => StackVec::from_elem(
                CustomType1(_to_str(GLOBAL_DATA, 100, 110).to_string()),
                _to_usize(GLOBAL_DATA, 110) % 8
            ),
        };

        let ops = _to_usize(GLOBAL_DATA, 120) % 8 + 4;
        for i in 0..ops {
            match _to_u8(GLOBAL_DATA, 130 + i) % 6 {
                0 => stack_vec.push(CustomType1(_to_str(GLOBAL_DATA, 140 + i*8, 148 + i*8).to_string())),
                1 => {let _ = stack_vec.pop();},
                2 => stack_vec.truncate(_to_usize(GLOBAL_DATA, 160 + i*4) % 64),
                3 => {
                    let idx = _to_usize(GLOBAL_DATA, 180 + i*4) % (stack_vec.len() + 1);
                    stack_vec.insert(idx, CustomType1(_to_str(GLOBAL_DATA, 200 + i*8, 208 + i*8).to_string()));
                },
                4 => {
                    let idx = _to_usize(GLOBAL_DATA, 220 + i*4) % (stack_vec.len() + 1);
                    let iter = CustomType2(_to_str(GLOBAL_DATA, 240 + i*8, 248 + i*8).to_string());
                    stack_vec.insert_many(idx, iter);
                },
                _ => {
                    let _ = stack_vec.as_slice();
                    let _ = stack_vec.as_mut_slice();
                    println!("{:?}", stack_vec);
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