#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use string_interner::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};
use std::collections::hash_map::RandomState;

struct CustomType5(String);
#[derive(Debug, Copy)]
struct CustomType0(usize);
struct CustomType3(String);
struct CustomType4(String);

impl std::iter::IntoIterator for CustomType3 {
    type Item = CustomType4;
    type IntoIter = CustomType5;
    
    fn into_iter(self) -> Self::IntoIter {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 198);
        let custom_impl_inst_num = self.0.len();
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0{
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector{
                    1 => global_data.first_half,
                    _ => global_data.second_half,
        };
        let mut t_40 = _to_u8(GLOBAL_DATA, 206) % 17;
        let t_41 = _to_str(GLOBAL_DATA, 207, 207 + t_40 as usize);
        let t_42 = String::from(t_41);
        let t_43 = CustomType5(t_42);
        return t_43;
    }
}

impl std::convert::AsRef<str> for CustomType4 {
    
    fn as_ref(&self) -> &str {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 123);
        let custom_impl_inst_num = self.0.len();
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0{
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector{
                    1 => global_data.first_half,
                    _ => global_data.second_half,
        };
        let mut t_26 = _to_u8(GLOBAL_DATA, 131) % 17;
        let t_27 = _to_str(GLOBAL_DATA, 132, 132 + t_26 as usize);
        t_27
    }
}

impl std::convert::From<CustomType4> for String {
    
    fn from(val: CustomType4) -> Self {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 148);
        let custom_impl_inst_num = val.0.len();
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0{
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector{
                    1 => global_data.first_half,
                    _ => global_data.second_half,
        };
        let mut t_31 = _to_u8(GLOBAL_DATA, 156) % 17;
        let t_32 = _to_str(GLOBAL_DATA, 157, 157 + t_31 as usize);
        String::from(t_32)
    }
}

impl std::cmp::Ord for CustomType0 {
    
    fn cmp(&self, _: &Self) -> std::cmp::Ordering {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 58);
        let custom_impl_inst_num = self.0;
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0{
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector{
                    1 => global_data.first_half,
                    _ => global_data.second_half,
        };
        let t_13 = _to_usize(GLOBAL_DATA, 66);
        let t_14 = string_interner::Sym::from_usize(t_13);
        let t_15 = &t_14;
        let t_16 = _to_usize(GLOBAL_DATA, 74);
        let t_17 = string_interner::Sym::from_usize(t_16);
        let t_18 = &t_17;
        string_interner::Sym::cmp(t_15, t_18)
    }
}

impl std::clone::Clone for CustomType0 {
    
    fn clone(&self) -> Self {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 82);
        let custom_impl_inst_num = self.0;
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0{
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector{
                    1 => global_data.first_half,
                    _ => global_data.second_half,
        };
        let t_20 = _to_usize(GLOBAL_DATA, 90);
        CustomType0(t_20)
    }
}

impl std::cmp::PartialOrd for CustomType0 {
    
    fn partial_cmp(&self, _: &Self) -> Option<std::cmp::Ordering> {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 0);
        let custom_impl_inst_num = self.0;
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0{
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector{
                    1 => global_data.first_half,
                    _ => global_data.second_half,
        };
        let t_0 = _to_usize(GLOBAL_DATA, 8);
        let t_1 = string_interner::Sym::from_usize(t_0);
        let t_2 = &t_1;
        let t_3 = _to_usize(GLOBAL_DATA, 16);
        let t_4 = string_interner::Sym::from_usize(t_3);
        let t_5 = &t_4;
        Some(string_interner::Sym::cmp(t_2, t_5))
    }
}

impl std::cmp::PartialEq for CustomType0 {
    
    fn eq(&self, _: &Self) -> bool {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 49);
        let custom_impl_inst_num = self.0;
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0{
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector{
                    1 => global_data.first_half,
                    _ => global_data.second_half,
        };
        _to_bool(GLOBAL_DATA, 57)
    }
}

impl std::cmp::Eq for CustomType0 {}

impl string_interner::Symbol for CustomType0 {
    
    fn to_usize(self) -> usize {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 24);
        let custom_impl_inst_num = self.0;
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0{
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector{
                    1 => global_data.first_half,
                    _ => global_data.second_half,
        };
        _to_usize(GLOBAL_DATA, 32)
    }
    
    fn from_usize(_: usize) -> Self {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let t_9 = _to_u8(GLOBAL_DATA, 40);
        if t_9 % 2 == 0{
            panic!("INTENTIONAL PANIC!");
        }
        let t_10 = _to_usize(GLOBAL_DATA, 41);
        CustomType0(t_10)
    }
}

impl std::iter::Iterator for CustomType5 {
    type Item = CustomType4;
    
    fn next(&mut self) -> Option<Self::Item> {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 173);
        let custom_impl_inst_num = self.0.len();
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0{
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector{
                    1 => global_data.first_half,
                    _ => global_data.second_half,
        };
        let mut t_35 = _to_u8(GLOBAL_DATA, 181) % 17;
        let t_36 = _to_str(GLOBAL_DATA, 182, 182 + t_35 as usize);
        let t_37 = String::from(t_36);
        Some(CustomType4(t_37))
    }
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 1024 {return;}
        set_global_data(data);
        let global_data = get_global_data();
        let (first_half, second_half) = (global_data.first_half, global_data.second_half);

        let constructor_selector = _to_usize(first_half, 0) % 4;
        let mut interner = match constructor_selector {
            0 => string_interner::StringInterner::<CustomType0>::new(),
            1 => {
                let cap = _to_usize(first_half, 8);
                string_interner::StringInterner::<CustomType0>::with_capacity(cap)
            },
            2 => {
                let hasher = RandomState::new();
                string_interner::StringInterner::<CustomType0>::with_hasher(hasher)
            },
            _ => {
                let cap = _to_usize(first_half, 16);
                let hasher = RandomState::new();
                string_interner::StringInterner::<CustomType0>::with_capacity_and_hasher(cap, hasher)
            }
        };

        let num_iters = _to_usize(second_half, 0) % 5 + 1;
        for _ in 0..num_iters {
            let op_selector = _to_u8(second_half, 8) % 6;
            match op_selector {
                0 => {
                    let len = _to_u8(second_half, 16) % 65;
                    let start = 24;
                    let end = start + len as usize;
                    if end >= second_half.len() {continue;}
                    let s = _to_str(second_half, start, end);
                    let sym = interner.get_or_intern(s);
                    let resolved = interner.resolve(sym);
                    if let Some(s) = resolved {
                        println!("Resolved: {}", s);
                    }
                },
                1 => {
                    let idx = _to_usize(second_half, 200);
                    let sym = CustomType0::from_usize(idx);
                    if let Some(s) = interner.resolve(sym) {
                        println!("Resolved: {}", s);
                    }
                },
                2 => {
                    let idx = _to_usize(second_half, 208);
                    let sym = CustomType0(idx);
                    let exists = interner.resolve(sym);
                    println!("Exists: {:?}", exists.is_some());
                },
                3 => {
                    let prev_cap = interner.capacity();
                    interner.reserve(_to_usize(second_half, 216));
                    println!("Reserved from {} to {}", prev_cap, interner.capacity());
                },
                4 => {
                    let iter = interner.iter();
                    for (sym, s) in iter {
                        println!("{} -> {}", sym.to_usize(), s);
                    }
                },
                _ => {
                    let other_interner = string_interner::StringInterner::<CustomType0>::new();
                    interner.shrink_to_fit();
                    println!("Shrunk to {} capacity", interner.capacity());
                }
            }
        }

        let mut target = string_interner::StringInterner::<CustomType0>::from_iter(interner.into_iter().map(|(_, s)| s));
        target.shrink_to_fit();
        println!("Final len: {}", target.len());
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