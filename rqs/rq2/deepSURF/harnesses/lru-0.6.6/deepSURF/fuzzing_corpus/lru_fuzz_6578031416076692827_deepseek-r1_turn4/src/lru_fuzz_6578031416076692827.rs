#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use lru::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

#[derive(Debug, Hash, Eq, PartialEq)]
struct CustomType0(String);
#[derive(Debug)]
struct CustomType1(String);
#[derive(Debug)]
struct CustomType4(String);

impl core::cmp::PartialEq for CustomType4 {
    fn eq(&self, _: &Self) -> bool {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 74);
        let custom_impl_inst_num = self.0.len();
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0{
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector{
                1 => global_data.first_half,
                _ => global_data.second_half,
        };
        let t_12 = _to_bool(GLOBAL_DATA, 82);
        return t_12;
    }
}

impl core::cmp::Eq for CustomType4 {}

impl core::hash::Hash for CustomType4 {
    fn hash<H: std::hash::Hasher>(&self, _: &mut H) {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 83);
        let custom_impl_inst_num = self.0.len();
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0{
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector{
                1 => global_data.first_half,
                _ => global_data.second_half,
        };
        return;
    }
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 2048 {return;}
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let constructor_selector = _to_u8(GLOBAL_DATA, 0) % 4;
        let mut cache;
        match constructor_selector {
            0 => {
                let cap = _to_usize(GLOBAL_DATA, 8);
                let hasher = lru::DefaultHasher::default();
                cache = lru::LruCache::<CustomType4, CustomType1>::with_hasher(cap, hasher);
            }
            1 => {
                let cap = _to_usize(GLOBAL_DATA, 8);
                cache = lru::LruCache::<CustomType4, CustomType1>::new(cap);
            }
            2 => {
                let hasher = lru::DefaultHasher::default();
                cache = lru::LruCache::<CustomType4, CustomType1>::unbounded_with_hasher(hasher);
            }
            _ => {
                cache = lru::LruCache::<CustomType4, CustomType1>::unbounded();
            }
        };

        let op_count = _to_usize(GLOBAL_DATA, 16) % 65;
        for i in 0..op_count {
            let op_selector = _to_u8(GLOBAL_DATA, 24 + i * 4) % 9;
            match op_selector {
                0 => {
                    let key_len = _to_u8(GLOBAL_DATA, 100 + i * 50) % 17;
                    let key_start = 200 + i * 50;
                    let key_str = _to_str(GLOBAL_DATA, key_start, key_start + key_len as usize);
                    let key = CustomType4(key_str.to_string());
                    
                    let val_len = _to_u8(GLOBAL_DATA, 300 + i * 50) % 17;
                    let val_start = 400 + i * 50;
                    let val_str = _to_str(GLOBAL_DATA, val_start, val_start + val_len as usize);
                    let val = CustomType1(val_str.to_string());
                    
                    cache.put(key, val);
                }
                1 => {
                    let key_len = _to_u8(GLOBAL_DATA, 500 + i * 50) % 17;
                    let key_start = 600 + i * 50;
                    let key_str = _to_str(GLOBAL_DATA, key_start, key_start + key_len as usize);
                    let key = CustomType4(key_str.to_string());
                    if let Some(v) = cache.get(&key) {
                        println!("{:?}", v);
                    }
                }
                2 => {
                    let key_len = _to_u8(GLOBAL_DATA, 700 + i * 50) % 17;
                    let key_start = 800 + i * 50;
                    let key_str = _to_str(GLOBAL_DATA, key_start, key_start + key_len as usize);
                    let key = CustomType4(key_str.to_string());
                    if let Some(v) = cache.get_mut(&key) {
                        *v = CustomType1("modified".to_string());
                    }
                }
                3 => {
                    let mut iter = cache.iter_mut();
                    while let Some((k, v)) = iter.next() {
                        println!("{:?} {:?}", k, v);
                        *v = CustomType1("mutated".to_string());
                    }
                }
                4 => {
                    if let Some((k, v)) = cache.pop_lru() {
                        println!("{:?} {:?}", k, v);
                    }
                }
                5 => {
                    let key_len = _to_u8(GLOBAL_DATA, 900 + i * 50) % 17;
                    let key_start = 1000 + i * 50;
                    let key_str = _to_str(GLOBAL_DATA, key_start, key_start + key_len as usize);
                    let key = CustomType4(key_str.to_string());
                    if let Some(v) = cache.peek(&key) {
                        println!("{:?}", v);
                    }
                }
                6 => {
                    let new_cap = _to_usize(GLOBAL_DATA, 1200 + i * 50);
                    cache.resize(new_cap);
                }
                7 => {
                    let mut iter = cache.iter();
                    while let Some((k, v)) = iter.next() {
                        println!("{:?} {:?}", k, v);
                    }
                }
                _ => {
                    cache.clear();
                }
            }
        }

        let key_len = _to_u8(GLOBAL_DATA, 1800) % 17;
        let key_start = 1801;
        let key_str = _to_str(GLOBAL_DATA, key_start, key_start + key_len as usize);
        let key = CustomType4(key_str.to_string());
        cache.peek_mut(&key);
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