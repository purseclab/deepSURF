#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use ordnung::*;
use global_data::*;
use std::hash::Hasher;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

#[derive(Eq, Hash, PartialEq, Clone)]
struct CustomType0(String);
#[derive(Clone, PartialEq)]
struct CustomType1(String);
struct CustomType2(String);
struct CustomType3(String);
struct CustomType4(String);
struct CustomType5(String);
struct CustomType6(String);

impl core::hash::Hash for CustomType6 {
    fn hash<H: Hasher>(&self, _: &mut H) {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 91);
        let custom_impl_inst_num = self.0.len();
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0 {
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector {
            1 => global_data.first_half,
            _ => global_data.second_half,
        };
    }
}

impl core::iter::IntoIterator for CustomType2 {
    type Item = CustomType5;
    type IntoIter = CustomType4;
    
    fn into_iter(self) -> Self::IntoIter {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 49);
        let custom_impl_inst_num = self.0.len();
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0 {
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector {
            1 => global_data.first_half,
            _ => global_data.second_half,
        };
        let mut t_9 = _to_u8(GLOBAL_DATA, 57) % 17;
        let t_10 = _to_str(GLOBAL_DATA, 58, 58 + t_9 as usize);
        let t_11 = String::from(t_10);
        let t_12 = CustomType4(t_11);
        t_12
    }
}

impl core::cmp::PartialEq for CustomType6 {
    fn eq(&self, _: &Self) -> bool {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 99);
        let custom_impl_inst_num = self.0.len();
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0 {
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector {
            1 => global_data.first_half,
            _ => global_data.second_half,
        };
        let t_19 = _to_bool(GLOBAL_DATA, 107);
        t_19
    }
}

impl core::cmp::Eq for CustomType6 {}

impl core::iter::Iterator for CustomType4 {
    type Item = CustomType5;
    
    fn size_hint(&self) -> (usize, Option<usize>) {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 0);
        let custom_impl_inst_num = self.0.len();
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0 {
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector {
            1 => global_data.first_half,
            _ => global_data.second_half,
        };
        let t_0 = _to_usize(GLOBAL_DATA, 8);
        let t_1 = _to_usize(GLOBAL_DATA, 16);
        let t_2 = Some(t_1);
        (t_0, t_2)
    }
    
    fn next(&mut self) -> Option<Self::Item> {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 24);
        let custom_impl_inst_num = self.0.len();
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0 {
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector {
            1 => global_data.first_half,
            _ => global_data.second_half,
        };
        let mut t_4 = _to_u8(GLOBAL_DATA, 32) % 17;
        let t_5 = _to_str(GLOBAL_DATA, 33, 33 + t_4 as usize);
        let t_6 = String::from(t_5);
        Some(CustomType5(t_6))
    }
}

struct CustomType7(Vec<(CustomType0, CustomType1)>);

impl core::iter::IntoIterator for CustomType7 {
    type Item = (CustomType0, CustomType1);
    type IntoIter = std::vec::IntoIter<(CustomType0, CustomType1)>;
    
    fn into_iter(self) -> Self::IntoIter {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 209);
        let custom_impl_inst_num = self.0.len();
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0 {
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector {
            1 => global_data.first_half,
            _ => global_data.second_half,
        };
        self.0.into_iter()
    }
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 500 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let op_count = _to_usize(GLOBAL_DATA, 0) % 8 + 3;
        let mut map = ordnung::Map::<CustomType0, CustomType1>::new();
        let mut alt_map = ordnung::Map::<CustomType0, CustomType1>::with_capacity(_to_usize(GLOBAL_DATA, 8) % 65);
        
        for i in 0..op_count {
            let op_selector = _to_u8(GLOBAL_DATA, 16 + i) % 7;
            
            match op_selector {
                0 => {
                    let key_len = _to_u8(GLOBAL_DATA, 24 + i) % 17;
                    let key = _to_str(GLOBAL_DATA, 32 + i * 17, 32 + i * 17 + key_len as usize);
                    let val_len = _to_u8(GLOBAL_DATA, 48 + i) % 17;
                    let val = _to_str(GLOBAL_DATA, 64 + i * 17, 64 + i * 17 + val_len as usize);
                    let _ = map.insert(CustomType0(key.to_string()), CustomType1(val.to_string()));
                },
                1 => {
                    let key_len = _to_u8(GLOBAL_DATA, 112 + i) % 17;
                    let key_str = _to_str(GLOBAL_DATA, 128 + i * 17, 128 + i * 17 + key_len as usize);
                    let key = CustomType0(key_str.to_string());
                    let _ = map.contains_key(&key);
                },
                2 => {
                    let key_len = _to_u8(GLOBAL_DATA, 192 + i) % 17;
                    let key_str = _to_str(GLOBAL_DATA, 208 + i * 17, 208 + i * 17 + key_len as usize);
                    let key = CustomType0(key_str.to_string());
                    let _ = map.remove(&key);
                },
                3 => {
                    let key_len = _to_u8(GLOBAL_DATA, 272 + i) % 17;
                    let key_str = _to_str(GLOBAL_DATA, 288 + i * 17, 288 + i * 17 + key_len as usize);
                    let key = CustomType0(key_str.to_string());
                    let _ = map.get(&key);
                },
                4 => {
                    let clone = map.clone();
                    let _ = map.eq(&clone);
                },
                5 => {
                    for (k, v) in map.iter() {
                        let _k = k.0.len();
                        let _v = v.0.len();
                    }
                },
                6 => {
                    let mut entries = Vec::new();
                    for _ in 0..(_to_u8(GLOBAL_DATA, 368 + i) % 5) {
                        let key_len = _to_u8(GLOBAL_DATA, 384 + i) % 17;
                        let key = _to_str(GLOBAL_DATA, 400 + i * 32, 400 + i * 32 + key_len as usize);
                        let val_len = _to_u8(GLOBAL_DATA, 416 + i) % 17;
                        let val = _to_str(GLOBAL_DATA, 432 + i * 32, 432 + i * 32 + val_len as usize);
                        entries.push((CustomType0(key.to_string()), CustomType1(val.to_string())));
                    }
                    let vec_builder = CustomType7(entries);
                    alt_map = ordnung::Map::from_iter(vec_builder);
                },
                _ => {
                    let _ = map.is_empty();
                    let _ = map.clear();
                }
            }
        }

        let final_key_len = _to_u8(GLOBAL_DATA, 480) % 17;
        let final_key_str = _to_str(GLOBAL_DATA, 496, 496 + final_key_len as usize);
        let final_key = CustomType0(final_key_str.to_string());
        map.contains_key(&final_key);
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