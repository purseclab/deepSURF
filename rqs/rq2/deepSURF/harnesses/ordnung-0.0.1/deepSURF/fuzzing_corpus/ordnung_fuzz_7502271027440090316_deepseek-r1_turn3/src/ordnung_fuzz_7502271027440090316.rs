#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use ordnung::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

#[derive(Clone, PartialEq)]
struct CustomType1(String);
struct CustomType2(String);
struct CustomType0(String);

impl core::iter::IntoIterator for CustomType0 {
    type Item = CustomType1;
    type IntoIter = CustomType2;
    
    fn into_iter(self) -> Self::IntoIter {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 25);
        let custom_impl_inst_num = self.0.len();
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0{
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector{
                    1 => global_data.first_half,
                    _ => global_data.second_half,
        };
        let mut t_5 = _to_u8(GLOBAL_DATA, 33) % 17;
        let t_6 = _to_str(GLOBAL_DATA, 34, 34 + t_5 as usize);
        let t_7 = String::from(t_6);
        let t_8 = CustomType2(t_7);
        return t_8;
    }
}

impl core::iter::Iterator for CustomType2 {
    type Item = CustomType1;
    
    fn next(&mut self) -> Option<Self::Item> {
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
        let mut t_0 = _to_u8(GLOBAL_DATA, 8) % 17;
        let t_1 = _to_str(GLOBAL_DATA, 9, 9 + t_0 as usize);
        let t_2 = String::from(t_1);
        let t_3 = CustomType1(t_2);
        let t_4 = Some(t_3);
        return t_4;
    }
}

fn main (){
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 512 {return;}
        set_global_data(data);
        let global_data = get_global_data();
        
        let constructor_selector = _to_u8(global_data.first_half, 0) % 4;
        let mut vec = match constructor_selector {
            0 => ordnung::compact::Vec::new(),
            1 => ordnung::compact::Vec::with_capacity(_to_usize(global_data.first_half, 1)),
            2 => {
                let mut t_9 = _to_u8(global_data.second_half, 50) % 17;
                let t_10 = _to_str(global_data.second_half, 51, 51 + t_9 as usize);
                ordnung::compact::Vec::from_iter(CustomType0(t_10.to_string()))
            },
            _ => {
                let mut t_9 = _to_u8(global_data.second_half, 50) % 17;
                let t_10 = _to_str(global_data.second_half, 51, 51 + t_9 as usize);
                ordnung::compact::Vec::from(vec![CustomType1(t_10.to_string())])
            }
        };

        let mut map = ordnung::Map::with_capacity(_to_usize(global_data.first_half, 2));
        let num_ops = _to_usize(global_data.first_half, 3) % 16;

        for i in 0..num_ops {
            let op = _to_u8(global_data.second_half, i * 4) % 7;
            match op {
                0 => {
                    let len_val = _to_u8(global_data.second_half, i * 8) % 17;
                    let s = _to_str(global_data.second_half, i * 8 + 1, i * 8 + 1 + len_val as usize);
                    vec.push(CustomType1(s.to_string()));
                },
                1 => {vec.pop();},
                2 => {
                    let cloned = vec.clone();
                    let _ = cloned.len();
                },
                3 => {
                    let cap = _to_usize(global_data.second_half, i * 2) % 65;
                    vec = ordnung::compact::Vec::with_capacity(cap);
                },
                4 => {
                    map.insert(
                        _to_str(global_data.second_half, i*3, i*3+16).to_string(),
                        _to_u64(global_data.second_half, i*8)
                    );
                },
                5 => {
                    for (k, v) in map.iter() {
                        let _key = k.deref();
                        let _val = v.deref();
                    }
                },
                6 => {
                    let idx = _to_usize(global_data.second_half, i*4) % (vec.len() + 1);
                    if idx < vec.len() {
                        vec.remove(idx);
                    }
                },
                _ => ()
            }
        }

        let map_clone = map.clone();
        let vec_ptr = vec.as_ptr();
        let _ = vec.deref();
        let _ = vec.deref_mut();

        let mut vec2 = ordnung::compact::Vec::from_iter(CustomType0(String::new()));
        vec2.push(CustomType1(_to_u128(global_data.second_half, 200).to_string()));
        let _ = vec == vec2;

        let final_clone = vec.clone();
        let _ = &final_clone;
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