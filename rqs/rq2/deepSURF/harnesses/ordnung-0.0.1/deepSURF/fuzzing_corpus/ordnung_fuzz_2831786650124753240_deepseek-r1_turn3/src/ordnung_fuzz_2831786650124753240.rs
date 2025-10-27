#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use ordnung::*;
use global_data::*;
use std::hash::Hasher;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
struct CustomType0(String);
#[derive(Debug)]
struct CustomType6(String);
#[derive(Debug)]
struct CustomType3(String);
#[derive(Debug)]
struct CustomType5(String);
#[derive(Debug)]
struct CustomType4(String);
#[derive(Debug)]
struct CustomType2(String);
#[derive(Debug, Clone)]
struct CustomType1(String);

impl core::iter::IntoIterator for CustomType2 {
    type Item = CustomType5;
    type IntoIter = CustomType4;
    
    fn into_iter(self) -> Self::IntoIter {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 49);
        let custom_impl_inst_num = self.0.len();
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0{
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector{
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
        if selector == 0{
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector{
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
        if selector == 0{
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector{
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

impl core::cmp::PartialEq for CustomType6 {
    
    fn eq(&self, _: &Self) -> bool {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 99);
        let custom_impl_inst_num = self.0.len();
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0{
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector{
            1 => global_data.first_half,
            _ => global_data.second_half,
        };
        let t_19 = _to_bool(GLOBAL_DATA, 107);
        return t_19;
    }
}

impl core::cmp::Eq for CustomType6 {
}

impl core::hash::Hash for CustomType6 {
    
    fn hash<H: Hasher>(&self, _: &mut H) {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 91);
        let custom_impl_inst_num = self.0.len();
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0{
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector{
            1 => global_data.first_half,
            _ => global_data.second_half,
        };
        return ;
    }
}

fn main (){
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 500 {return;}
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let base_offset = 200;
        let construct_selector = _to_u8(GLOBAL_DATA, base_offset) % 3;
        let map = match construct_selector {
            0 => {
                let capacity = _to_usize(GLOBAL_DATA, base_offset + 1) % 65;
                ordnung::Map::with_capacity(capacity)
            },
            1 => {
                let capacity = _to_usize(GLOBAL_DATA, base_offset + 1) % 65;
                ordnung::Map::with_capacity(capacity)
            },
            _ => ordnung::Map::new()
        };

        let op_count = _to_u8(GLOBAL_DATA, base_offset + 10) % 10 + 1;
        let mut working_map = map;
        let mut temp_vec = ordnung::compact::Vec::new();

        for i in 0..op_count {
            let op_select = _to_u8(GLOBAL_DATA, base_offset + 20 + i as usize) % 7;
            match op_select {
                0 => {
                    let k_len = _to_u8(GLOBAL_DATA, base_offset + 50 + i as usize) % 17;
                    let k_start = base_offset + 100 + i as usize * 2;
                    let key = CustomType0(String::from(_to_str(GLOBAL_DATA, k_start, k_start + k_len as usize)));
                    working_map.insert(key, CustomType1(String::new()));
                },
                1 => {
                    let k_len = _to_u8(GLOBAL_DATA, base_offset + 150 + i as usize) % 17;
                    let k_start = base_offset + 200 + i as usize * 2;
                    let key = CustomType0(String::from(_to_str(GLOBAL_DATA, k_start, k_start + k_len as usize)));
                    let _ = working_map.get(&key);
                },
                2 => {
                    let k_len = _to_u8(GLOBAL_DATA, base_offset + 250 + i as usize) % 17;
                    let k_start = base_offset + 300 + i as usize * 2;
                    let key = CustomType0(String::from(_to_str(GLOBAL_DATA, k_start, k_start + k_len as usize)));
                    let _ = working_map.remove(&key);
                },
                3 => {
                    working_map.clear();
                },
                4 => {
                    let cloned = working_map.clone();
                    let _len = cloned.len();
                },
                5 => {
                    let mut iter = working_map.iter();
                    for (k, v) in &mut iter {
                        println!("{:?} {:?}", k, v);
                    }
                },
                6 => {
                    let capacity = _to_usize(GLOBAL_DATA, base_offset + 350 + i as usize) % 65;
                    let mut local_vec = ordnung::compact::Vec::with_capacity(capacity);
                    local_vec.push(CustomType0(String::new()));
                    temp_vec = local_vec;
                },
                _ => ()
            }
        }

        let key_len = _to_u8(GLOBAL_DATA, 400) % 17;
        let key_str = _to_str(GLOBAL_DATA, 401, 401 + key_len as usize);
        let lookup_key = CustomType0(String::from(key_str));
        let _val_ref = working_map.index(&lookup_key);

        if temp_vec.len() > 0 {
            let elem = temp_vec.remove(0);
            println!("{:?}", elem);
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