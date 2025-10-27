#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

struct CustomType3(String);
struct CustomType2(String);

impl Iterator for CustomType3 {
    type Item = String;
    
    fn next(&mut self) -> Option<Self::Item> {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let selector = _to_u8(GLOBAL_DATA, 9) % 3;
        if selector == 0 {
            panic!("INTENTIONAL PANIC!");
        }
        let mut len = (_to_u8(GLOBAL_DATA, 17) as usize) % 65;
        len = len.min(GLOBAL_DATA.len().saturating_sub(18));
        let s = _to_str(GLOBAL_DATA, 18, 18 + len);
        Some(s.to_string())
    }
}

impl IntoIterator for CustomType2 {
    type Item = String;
    type IntoIter = CustomType3;
    
    fn into_iter(self) -> Self::IntoIter {
        CustomType3(self.0)
    }
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 2048 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let ops = _to_usize(GLOBAL_DATA, 0) % 65;
        let mut index = 2;
        
        let mut sv1 = SmallVec::<[String; 128]>::new();
        let mut sv2 = SmallVec::<[String; 128]>::with_capacity(_to_usize(GLOBAL_DATA, index) % 65);
        index += std::mem::size_of::<usize>();
        
        for _ in 0..ops {
            let op = _to_u8(GLOBAL_DATA, index) % 5;
            index += 1;
            
            match op {
                0 => {
                    let elem_len = _to_u8(GLOBAL_DATA, index) as usize;
                    index += 1;
                    let elem = _to_str(GLOBAL_DATA, index, index + elem_len);
                    sv1.push(elem.to_string());
                    index += elem_len;
                }
                1 => {
                    let drain_range = 0.._to_usize(GLOBAL_DATA, index) % (sv1.len() + 1);
                    index += std::mem::size_of::<usize>();
                    let _ = sv1.drain(drain_range);
                }
                2 => {
                    let slice_len = _to_usize(GLOBAL_DATA, index) % 65;
                    index += std::mem::size_of::<usize>();
                    let elements: Vec<String> = (0..slice_len).map(|_| {
                        let len = _to_u8(GLOBAL_DATA, index) as usize;
                        index += 1;
                        let s = _to_str(GLOBAL_DATA, index, index + len);
                        index += len;
                        s.to_string()
                    }).collect();
                    sv2 = SmallVec::from_vec(elements);
                }
                3 => {
                    let iter_data = _to_str(GLOBAL_DATA, index, GLOBAL_DATA.len());
                    index = GLOBAL_DATA.len();
                    let custom_iter = CustomType2(iter_data.to_string()).into_iter();
                    let sv = SmallVec::<[String; 128]>::from_iter(custom_iter);
                    println!("{:?}", sv.as_slice());
                }
                4 => {
                    if !sv1.is_empty() {
                        let idx = _to_usize(GLOBAL_DATA, index);
                        let fallback = sv1.as_slice();
                        let _ = &sv1[idx % sv1.len()];
                        let _ = sv2.partial_cmp(&sv1);
                    }
                }
                _ => {}
            }
        }
        
        let str_len = _to_u8(GLOBAL_DATA, index) as usize % 65;
        let data_str = _to_str(GLOBAL_DATA, index + 1, index + 1 + str_len);
        let container = CustomType2(data_str.to_string());
        let _ = SmallVec::<[String; 128]>::from_iter(container.into_iter());
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