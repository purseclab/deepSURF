#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use ordnung::*;
use global_data::*;
use std::ops::{Deref, DerefMut};

#[derive(PartialEq, Eq, Hash, Clone)]
struct CustomType0(String);
#[derive(Clone)]
struct CustomType1(String);
struct CustomType2(String);
struct CustomType3(CustomType0, CustomType1);

struct CustomType4 {
    data: Vec<(CustomType0, CustomType1)>,
    index: usize,
}

struct CustomType6(String);

impl core::cmp::Eq for CustomType6 {}
impl core::cmp::PartialEq for CustomType6 {
    fn eq(&self, _: &Self) -> bool {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 99);
        let custom_impl_inst_num = self.0.len();
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0 { panic!("INTENTIONAL PANIC!"); }
        let GLOBAL_DATA = match selector {
            1 => global_data.first_half,
            _ => global_data.second_half,
        };
        _to_bool(GLOBAL_DATA, 107)
    }
}

impl core::hash::Hash for CustomType6 {
    fn hash<H: core::hash::Hasher>(&self, _: &mut H) {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 91);
        let custom_impl_inst_num = self.0.len();
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0 { panic!("INTENTIONAL PANIC!"); }
        let GLOBAL_DATA = match selector {
            1 => global_data.first_half,
            _ => global_data.second_half,
        };
    }
}

impl core::iter::Iterator for CustomType4 {
    type Item = (CustomType0, CustomType1);

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.data.len(), Some(self.data.len()))
    }

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.data.len() {
            let item = self.data[self.index].clone();
            self.index += 1;
            Some(item)
        } else {
            None
        }
    }
}

impl core::iter::IntoIterator for CustomType2 {
    type Item = (CustomType0, CustomType1);
    type IntoIter = CustomType4;

    fn into_iter(self) -> Self::IntoIter {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let entries_len = _to_u8(GLOBAL_DATA, 57) % 17;
        let entries_str = _to_str(GLOBAL_DATA, 58, 58 + entries_len as usize);
        let mut data = Vec::new();
        for chunk in entries_str.as_bytes().chunks(2) {
            if chunk.len() >= 1 {
                let key = CustomType0(chunk[0..1].iter().map(|&b| b as char).collect());
                let val = if chunk.len() >= 2 {
                    CustomType1(chunk[1..2].iter().map(|&b| b as char).collect())
                } else {
                    CustomType1(String::new())
                };
                data.push((key, val));
            }
        }
        CustomType4 { data, index: 0 }
    }
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 600 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let (first, second) = (global_data.first_half, global_data.second_half);
        
        let constructor = _to_u8(first, 0) % 3;
        let mut map = match constructor {
            0 => ordnung::Map::<CustomType0, CustomType1>::new(),
            1 => ordnung::Map::with_capacity(_to_usize(first, 1)),
            _ => {
                let entries_len = _to_u8(first, 10) % 17;
                let entries_str = _to_str(first, 11, 11 + entries_len as usize);
                ordnung::Map::from_iter(CustomType2(entries_str.to_string()))
            }
        };

        let ops = _to_u8(second, 0) % 5 + 1;
        for i in 0..ops {
            match _to_u8(second, 1 + i as usize) % 7 {
                0 => {
                    let key_len = _to_u8(second, 10 + (i as usize) * 5) % 17;
                    let key = CustomType0(_to_str(second, 15 + (i as usize) * 5, 15 + (i as usize) * 5 + key_len as usize).to_string());
                    let val_len = _to_u8(second, 50 + (i as usize) * 5) % 17;
                    let val = CustomType1(_to_str(second, 55 + (i as usize) * 5, 55 + (i as usize) * 5 + val_len as usize).to_string());
                    map.insert(key, val);
                }
                1 => {
                    let key_len = _to_u8(second, 100 + (i as usize) * 3) % 17;
                    let key = CustomType0(_to_str(second, 103 + (i as usize) * 3, 103 + (i as usize) * 3 + key_len as usize).to_string());
                    map.remove(&key);
                }
                2 => {
                    let key_len = _to_u8(second, 150 + (i as usize) * 2) % 17;
                    let key = CustomType0(_to_str(second, 152 + (i as usize) * 2, 152 + (i as usize) * 2 + key_len as usize).to_string());
                    if let Some(v) = map.get_mut(&key) {
                        *v = CustomType1(v.0.chars().rev().collect());
                    }
                }
                3 => {
                    let mut iter = map.iter_mut();
                    while let Some((k, v)) = iter.next() {
                        v.0.push_str("_mut");
                    }
                }
                4 => {
                    map.clear();
                }
                5 => {
                    println!("Map len: {}", map.len());
                    let _ = map.iter().collect::<Vec<_>>();
                }
                _ => {
                    let key_len = _to_u8(second, 200 + (i as usize) * 4) % 17;
                    let key = CustomType0(_to_str(second, 204 + (i as usize) * 4, 204 + (i as usize) * 4 + key_len as usize).to_string());
                    let _ = map.get_or_insert(key, || {
                        let val_len = _to_u8(second, 300 + (i as usize) * 4) % 17;
                        CustomType1(_to_str(second, 304 + (i as usize) * 4, 304 + (i as usize) * 4 + val_len as usize).to_string())
                    });
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