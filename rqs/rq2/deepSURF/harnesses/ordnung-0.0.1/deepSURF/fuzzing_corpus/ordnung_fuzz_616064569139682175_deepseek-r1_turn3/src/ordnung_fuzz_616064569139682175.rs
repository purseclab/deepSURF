#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use ordnung::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

#[derive(PartialEq, Eq, Hash, Clone)]
struct CustomType0(String);
#[derive(Clone)]
struct CustomType1(String);
struct CustomType2(String);
struct CustomType3((CustomType0, CustomType1));
struct CustomType4(String);
struct CustomType5(String);

impl core::iter::Iterator for CustomType4 {
    type Item = (CustomType0, CustomType1);
    
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
        let key_len = _to_u8(GLOBAL_DATA, 32) % 17;
        let key_str = _to_str(GLOBAL_DATA, 33, 33 + key_len as usize);
        let val_len = _to_u8(GLOBAL_DATA, 33 + key_len as usize) % 17;
        let val_str = _to_str(GLOBAL_DATA, 34 + key_len as usize, 34 + key_len as usize + val_len as usize);
        let key = CustomType0(key_str.to_string());
        let value = CustomType1(val_str.to_string());
        return Some((key, value));
    }
}

impl core::iter::IntoIterator for CustomType2 {
    type Item = (CustomType0, CustomType1);
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

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 1024 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let constructor_selector = _to_u8(GLOBAL_DATA, 0) % 3;
        let mut map = match constructor_selector {
            0 => Map::<CustomType0, CustomType1>::new(),
            1 => Map::<CustomType0, CustomType1>::with_capacity(_to_usize(GLOBAL_DATA, 1)),
            2 => {
                let mut t_13 = _to_u8(GLOBAL_DATA, 74) % 17;
                let t_14 = _to_str(GLOBAL_DATA, 75, 75 + t_13 as usize);
                Map::<CustomType0, CustomType1>::from_iter(CustomType2(t_14.to_string()))
            }
            _ => unreachable!(),
        };

        let op_count = _to_u8(GLOBAL_DATA, 100) % 10;
        for i in 0..op_count {
            let sel = _to_u8(GLOBAL_DATA, 101 + i as usize) % 6;
            match sel {
                0 => {
                    let key_len = _to_u8(GLOBAL_DATA, 150 + i as usize * 2) % 17;
                    let key_str = _to_str(GLOBAL_DATA, 151 + i as usize * 2, 151 + i as usize * 2 + key_len as usize);
                    let val_len = _to_u8(GLOBAL_DATA, 151 + i as usize * 2 + key_len as usize) % 17;
                    let val_str = _to_str(GLOBAL_DATA, 152 + i as usize * 2 + key_len as usize, 152 + i as usize * 2 + key_len as usize + val_len as usize);
                    map.insert(CustomType0(key_str.to_string()), CustomType1(val_str.to_string()));
                }
                1 => {
                    let key_len = _to_u8(GLOBAL_DATA, 200 + i as usize * 2) % 17;
                    let key_str = _to_str(GLOBAL_DATA, 201 + i as usize * 2, 201 + i as usize * 2 + key_len as usize);
                    if let Some(v) = map.get(&CustomType0(key_str.to_string())) {
                        println!("{}", v.0);
                    }
                }
                2 => {
                    let key_len = _to_u8(GLOBAL_DATA, 300 + i as usize * 2) % 17;
                    let key_str = _to_str(GLOBAL_DATA, 301 + i as usize * 2, 301 + i as usize * 2 + key_len as usize);
                    map.remove(&CustomType0(key_str.to_string()));
                }
                3 => {
                    for (k, v) in map.iter() {
                        println!("{} {}", k.0, v.0);
                    }
                }
                4 => {
                    for (k, v) in map.iter_mut() {
                        v.0.push('!');
                    }
                }
                5 => {
                    let clone = map.clone();
                    if let Some(first_key) = clone.iter().next() {
                        println!("Cloned key: {}", first_key.0.0);
                    }
                }
                _ => (),
            }
        }

        map.clear();

        let post_ops = _to_u8(GLOBAL_DATA, 500) % 5;
        for i in 0..post_ops {
            let sel = _to_u8(GLOBAL_DATA, 501 + i as usize) % 3;
            match sel {
                0 => {
                    let key_len = _to_u8(GLOBAL_DATA, 550 + i as usize * 2) % 17;
                    let key_str = _to_str(GLOBAL_DATA, 551 + i as usize * 2, 551 + i as usize * 2 + key_len as usize);
                    map.insert(CustomType0(key_str.to_string()), CustomType1(String::new()));
                }
                1 => {
                    println!("Post-clear len: {} empty: {}", map.len(), map.is_empty());
                }
                2 => {
                    let cap = map.iter().size_hint().0;
                    let _ = Map::<CustomType0, CustomType1>::with_capacity(cap);
                }
                _ => (),
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