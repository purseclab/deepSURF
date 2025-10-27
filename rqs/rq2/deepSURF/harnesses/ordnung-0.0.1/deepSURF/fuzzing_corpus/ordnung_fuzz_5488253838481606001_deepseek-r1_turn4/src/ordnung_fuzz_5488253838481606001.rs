#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use ordnung::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};
use std::hash::Hasher;

#[derive(Debug)]
struct CustomType4(String);
#[derive(Debug)]
struct CustomType5(String);
#[derive(Debug, Clone, PartialEq)]
struct CustomType1(String);
#[derive(Debug, Clone)]
struct CustomType0(String);
#[derive(Debug)]
struct CustomType2(String);
#[derive(Debug)]
struct CustomType3(String);

impl core::cmp::Eq for CustomType0 {}

impl core::cmp::PartialEq for CustomType0 {
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
        return t_19;
    }
}

impl core::hash::Hash for CustomType0 {
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
        return;
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
        return t_12;
    }
}

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
        let t_3 = (t_0, t_2);
        return t_3;
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
        let t_7 = CustomType5(t_6);
        let t_8 = Some(t_7);
        return t_8;
    }
}

fn _custom_fn0() -> CustomType1 {
    let global_data = get_global_data();
    let GLOBAL_DATA = global_data.first_half;
    let t_24 = _to_u8(GLOBAL_DATA, 125);
    if t_24 % 2 == 0 {
        panic!("INTENTIONAL PANIC!");
    }
    let mut t_25 = _to_u8(GLOBAL_DATA, 126) % 17;
    let t_26 = _to_str(GLOBAL_DATA, 127, 127 + t_25 as usize);
    let t_27 = String::from(t_26);
    let t_28 = CustomType1(t_27);
    return t_28;
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 1024 {
            return;
        }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let constructor_choice = _to_u8(GLOBAL_DATA, 0) % 3;
        let mut map = match constructor_choice {
            0 => Map::new(),
            1 => Map::with_capacity(_to_usize(GLOBAL_DATA, 1)),
            _ => {
                let num_entries = _to_usize(GLOBAL_DATA, 74) % 65;
                let mut entries = Vec::new();
                for i in 0..num_entries {
                    let entry_base = 75 + i * 20;
                    let key_len = _to_u8(GLOBAL_DATA, entry_base) % 17;
                    let key_start = entry_base + 1;
                    let key_end = key_start + key_len as usize;
                    let key_str = _to_str(GLOBAL_DATA, key_start, key_end);
                    let key = CustomType0(key_str.to_string());
                    
                    let val_start = key_end;
                    let val_len = _to_u8(GLOBAL_DATA, val_start) % 17;
                    let val_str = _to_str(GLOBAL_DATA, val_start + 1, val_start + 1 + val_len as usize);
                    let val = CustomType1(val_str.to_string());
                    
                    entries.push((key, val));
                }
                Map::from_iter(entries)
            }
        };

        let op_count = _to_usize(GLOBAL_DATA, 2) % 10;
        for op_idx in 0..op_count {
            let op_selector = _to_u8(GLOBAL_DATA, 3 + op_idx) % 8;

            match op_selector {
                0 => {
                    let key_start = 100 + op_idx * 5;
                    let key_len = _to_u8(GLOBAL_DATA, key_start) % 17;
                    let key_str = _to_str(GLOBAL_DATA, key_start + 1, key_start + 1 + key_len as usize);
                    let key = CustomType0(key_str.to_string());
                    let val_closure = || {
                        let val_start = 200 + op_idx * 5;
                        let val_len = _to_u8(GLOBAL_DATA, val_start) % 17;
                        CustomType1(_to_str(GLOBAL_DATA, val_start + 1, val_start + 1 + val_len as usize).to_string())
                    };
                    let val_ref = map.get_or_insert(key, val_closure);
                    println!("{:?}", *val_ref);
                }
                1 => {
                    let key_start = 300 + op_idx * 5;
                    let key_len = _to_u8(GLOBAL_DATA, key_start) % 17;
                    let key = CustomType0(_to_str(GLOBAL_DATA, key_start + 1, key_start + 1 + key_len as usize).to_string());
                    let val_start = 400 + op_idx * 5;
                    let val_len = _to_u8(GLOBAL_DATA, val_start) % 17;
                    let val = CustomType1(_to_str(GLOBAL_DATA, val_start + 1, val_start + 1 + val_len as usize).to_string());
                    map.insert(key, val);
                }
                2 => {
                    let key_start = 500 + op_idx * 5;
                    let key_len = _to_u8(GLOBAL_DATA, key_start) % 17;
                    let key = CustomType0(_to_str(GLOBAL_DATA, key_start + 1, key_start + 1 + key_len as usize).to_string());
                    map.remove(&key);
                }
                3 => {
                    for (k, v) in map.iter_mut() {
                        *v = _custom_fn0();
                    }
                }
                4 => {
                    let key_start = 600 + op_idx * 5;
                    let key_len = _to_u8(GLOBAL_DATA, key_start) % 17;
                    let key = CustomType0(_to_str(GLOBAL_DATA, key_start + 1, key_start + 1 + key_len as usize).to_string());
                    if let Some(v) = map.get(&key) {
                        println!("{:?}", *v);
                    }
                }
                5 => {
                    let cloned_map = map.clone();
                    if map == cloned_map {
                        panic!("Maps equal");
                    }
                }
                6 => {
                    let mut entries = map.iter().count();
                    entries = entries;
                }
                7 => {
                    map.clear();
                }
                _ => unreachable!(),
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