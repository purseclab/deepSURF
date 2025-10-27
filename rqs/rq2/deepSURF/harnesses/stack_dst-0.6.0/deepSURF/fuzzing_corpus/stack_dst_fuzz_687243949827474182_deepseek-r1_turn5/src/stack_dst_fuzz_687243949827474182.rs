#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use stack_dst::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

#[derive(Debug)]
struct CustomType2(String);
#[derive(Debug)]
struct CustomType0(String);
struct CustomType1(usize);

impl std::marker::Copy for CustomType1 {}

impl std::convert::AsRef<CustomType0> for CustomType1 {
    fn as_ref(&self) -> &CustomType0 {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 0);
        let custom_impl_inst_num = self.0;
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0 {
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector {
            1 => global_data.first_half,
            _ => global_data.second_half,
        };
        let mut t_0 = _to_u8(GLOBAL_DATA, 8) % 17;
        let t_1 = _to_str(GLOBAL_DATA, 9, 9 + t_0 as usize);
        let t_2 = String::from(t_1);
        let t_3 = Box::new(CustomType0(t_2));
        Box::leak(t_3)
    }
}

impl std::convert::AsMut<CustomType0> for CustomType1 {
    fn as_mut(&mut self) -> &mut CustomType0 {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 50);
        let custom_impl_inst_num = self.0;
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0 {
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector {
            1 => global_data.first_half,
            _ => global_data.second_half,
        };
        let mut t_10 = _to_u8(GLOBAL_DATA, 58) % 17;
        let t_11 = _to_str(GLOBAL_DATA, 59, 59 + t_10 as usize);
        let t_12 = String::from(t_11);
        let t_13 = Box::new(CustomType0(t_12));
        Box::leak(t_13)
    }
}

impl std::clone::Clone for CustomType1 {
    fn clone(&self) -> Self {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 25);
        let custom_impl_inst_num = self.0;
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0 {
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector {
            1 => global_data.first_half,
            _ => global_data.second_half,
        };
        let t_5 = _to_usize(GLOBAL_DATA, 33);
        let t_6 = CustomType1(t_5);
        t_6
    }
}

impl std::default::Default for CustomType1 {
    fn default() -> Self {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let t_7 = _to_u8(GLOBAL_DATA, 41);
        if t_7 % 2 == 0 {
            panic!("INTENTIONAL PANIC!");
        }
        let t_8 = _to_usize(GLOBAL_DATA, 42);
        let t_9 = CustomType1(t_8);
        t_9
    }
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 2048 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        
        let op_count = _to_usize(GLOBAL_DATA, 0) % 10 + 1;
        let mut offset = 1;
        
        let mut stack = stack_dst::StackA::<dyn std::fmt::Debug, [usize; 15]>::new();
        
        for _ in 0..op_count {
            if offset >= GLOBAL_DATA.len() { break; }
            let op_selector = _to_u8(GLOBAL_DATA, offset) % 6;
            offset += 1;

            match op_selector {
                0 => {
                    let str_len = (_to_u8(GLOBAL_DATA, offset) % 65) as usize;
                    offset += 1;
                    let start = offset;
                    offset += str_len;
                    if start > GLOBAL_DATA.len() || offset > GLOBAL_DATA.len() { break; }
                    let s = _to_str(GLOBAL_DATA, start, offset);
                    let custom2 = CustomType2(s.to_string());
                    match stack.push_stable(custom2, |ct2| ct2 as &dyn std::fmt::Debug) {
                        Ok(_) => {},
                        Err(_) => panic!("Push failed")
                    }
                },
                1 => {
                    let str_len = (_to_u8(GLOBAL_DATA, offset) % 65) as usize;
                    offset += 1;
                    let start = offset;
                    offset += str_len;
                    if start > GLOBAL_DATA.len() || offset > GLOBAL_DATA.len() { break; }
                    let s = _to_str(GLOBAL_DATA, start, offset);
                    match stack.push_stable(s, |s| s as &dyn std::fmt::Debug) {
                        Ok(_) => {},
                        Err(_) => panic!("Push failed")
                    }
                },
                2 => { stack.pop(); },
                3 => {
                    let str_len = (_to_u8(GLOBAL_DATA, offset) % 65) as usize;
                    offset += 1;
                    let start = offset;
                    offset += str_len;
                    if start > GLOBAL_DATA.len() || offset > GLOBAL_DATA.len() { break; }
                    let s = _to_str(GLOBAL_DATA, start, offset);
                    let custom2 = CustomType2(s.to_string());
                    let _ = stack_dst::ValueA::<CustomType0, [usize; 15]>::new_stable(custom2, |ct2: &CustomType2| {
                        let ct0 = Box::new(CustomType0(ct2.0.clone()));
                        Box::leak(ct0)
                    });
                },
                4 => {
                    if let Some(top) = stack.top() {
                        println!("Top: {:?}", top);
                    }
                },
                5 => {
                    if let Some(top) = stack.top_mut() {
                        println!("Top mut: {:?}", top);
                    }
                },
                _ => unreachable!()
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