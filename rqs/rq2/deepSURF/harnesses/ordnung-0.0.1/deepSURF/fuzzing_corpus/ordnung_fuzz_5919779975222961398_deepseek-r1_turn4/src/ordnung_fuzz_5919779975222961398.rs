#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use ordnung::*;
use global_data::*;
use std::ops::{Deref, DerefMut};

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct CustomType0(String);
#[derive(Debug, Clone, PartialEq)]
struct CustomType1(String);
#[derive(Debug)]
struct CustomType5(String);
struct CustomType3(String);
#[derive(Debug)]
struct CustomType2(String);
struct CustomType4(String);

impl Iterator for CustomType4 {
    type Item = CustomType3;

    fn size_hint(&self) -> (usize, Option<usize>) {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 0);
        let custom_impl_inst_num = self.0.len();
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0 { panic!("INTENTIONAL PANIC!") }
        let GLOBAL_DATA = match selector { 1 => global_data.first_half, _ => global_data.second_half };
        (_to_usize(GLOBAL_DATA, 8), Some(_to_usize(GLOBAL_DATA, 16)))
    }

    fn next(&mut self) -> Option<Self::Item> {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 24);
        let custom_impl_inst_num = self.0.len();
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0 { panic!("INTENTIONAL PANIC!") }
        let GLOBAL_DATA = match selector { 1 => global_data.first_half, _ => global_data.second_half };
        let len = _to_u8(GLOBAL_DATA, 32) % 17;
        Some(CustomType3(String::from(_to_str(GLOBAL_DATA, 33, 33 + len as usize))))
    }
}

impl IntoIterator for CustomType2 {
    type Item = CustomType3;
    type IntoIter = CustomType4;

    fn into_iter(self) -> Self::IntoIter {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 49);
        let custom_impl_inst_num = self.0.len();
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0 { panic!("INTENTIONAL PANIC!") }
        let GLOBAL_DATA = match selector { 1 => global_data.first_half, _ => global_data.second_half };
        let len = _to_u8(GLOBAL_DATA, 57) % 17;
        CustomType4(String::from(_to_str(GLOBAL_DATA, 58, 58 + len as usize)))
    }
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 8192 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        
        let ops = _to_u8(GLOBAL_DATA, 0) % 16;
        let mut maps = vec![];

        for i in 0..=ops {
            let i = i as usize;
            let op_selector = _to_u8(GLOBAL_DATA, i + 1) % 7;

            match op_selector {
                0 => {
                    let mut vec = ordnung::compact::Vec::<CustomType0>::new();
                    let count = _to_usize(GLOBAL_DATA, i * 8) % 65;
                    for _ in 0..count {
                        let len = _to_u8(GLOBAL_DATA, i * 16) % 17;
                        let s = _to_str(GLOBAL_DATA, i * 24, i * 24 + len as usize);
                        vec.push(CustomType0(s.to_string()));
                    }
                    let clone_vec = vec.clone();
                    println!("{:?}", clone_vec.deref());
                }
                1 => {
                    let count = _to_usize(GLOBAL_DATA, i * 8) % 65;
                    let mut pairs = Vec::new();
                    for j in 0..count {
                        let key_start = i * 32 + j * 24;
                        let key_len = _to_u8(GLOBAL_DATA, key_start) % 17;
                        let key = _to_str(GLOBAL_DATA, key_start + 1, key_start + 1 + key_len as usize);
                        
                        let val_start = i * 64 + j * 24;
                        let val_len = _to_u8(global_data.second_half, val_start) % 17;
                        let val = _to_str(global_data.second_half, val_start + 1, val_start + 1 + val_len as usize);
                        
                        pairs.push((CustomType0(key.to_string()), CustomType1(val.to_string())));
                    }
                    maps.push(ordnung::Map::from_iter(pairs));
                }
                2 => {
                    let capacity = _to_usize(global_data.second_half, i * 4) % 65;
                    maps.push(ordnung::Map::with_capacity(capacity));
                }
                3 => if let Some(map) = maps.last_mut() {
                    let k = CustomType0(_to_str(GLOBAL_DATA, i*8, i*8+16).to_string());
                    let v = CustomType1(_to_str(global_data.second_half, i*4, i*4+8).to_string());
                    let _ = map.insert(k, v);
                }
                4 => if let Some(map) = maps.last() {
                    let k = CustomType0(_to_str(GLOBAL_DATA, i*16, i*16+8).to_string());
                    if let Some(val) = map.get(&k) {
                        println!("{:?}", val.0);
                    }
                }
                5 => if let Some(map) = maps.last_mut() {
                    let iter = map.iter_mut();
                    for node in iter {
                        println!("{:?} {:?}", node.0.0, node.1.0);
                    }
                }
                6 => {
                    let mut std_vec = vec![];
                    let count = _to_usize(global_data.second_half, i * 12) % 65;
                    for _ in 0..count {
                        let k = CustomType0(_to_str(GLOBAL_DATA, i * 24, i * 24 + 8).to_string());
                        let v = CustomType1(_to_str(global_data.second_half, i * 32, i * 32 + 8).to_string());
                        std_vec.push((k, v));
                    }
                    maps.push(ordnung::Map::from_iter(std_vec.into_iter()));
                }
                _ => {}
            }
        }

        let mut final_map = ordnung::Map::new();
        for map in maps {
            if map.len() > 0 {
                final_map = map.clone();
                let capacity = _to_usize(global_data.second_half, 128) % 65;
                let temp = ordnung::Map::from_iter(map.iter().map(|n| (CustomType0(n.0.0.clone()), CustomType1(n.1.0.clone()))));
                println!("{}", temp.eq(&final_map));
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