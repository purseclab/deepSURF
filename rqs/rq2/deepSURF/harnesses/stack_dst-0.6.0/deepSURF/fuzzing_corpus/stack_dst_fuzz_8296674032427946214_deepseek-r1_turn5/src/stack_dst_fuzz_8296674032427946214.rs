#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use stack_dst::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut};

struct CustomType1(usize);
struct CustomType2(String);
struct CustomType0(String);

impl std::default::Default for CustomType1 {
    fn default() -> Self {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let t_5 = _to_u8(GLOBAL_DATA, 25);
        if t_5 % 2 == 0{
            panic!("INTENTIONAL PANIC!");
        }
        let t_6 = _to_usize(GLOBAL_DATA, 26);
        let t_7 = CustomType1(t_6);
        return t_7;
    }
}

impl std::clone::Clone for CustomType1 {
    fn clone(&self) -> Self {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 34);
        let custom_impl_inst_num = self.0;
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0{
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector{
                    1 => global_data.first_half,
                    _ => global_data.second_half,
        };
        let t_8 = _to_usize(GLOBAL_DATA, 42);
        let t_9 = CustomType1(t_8);
        return t_9;
    }
}

impl std::marker::Copy for CustomType1 {}

impl std::convert::AsRef<CustomType0> for CustomType1 {
    fn as_ref(&self) -> &CustomType0 {
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
        let mut t_0 = _to_u8(GLOBAL_DATA, 8) % 17;
        let t_1 = _to_str(GLOBAL_DATA, 9, 9 + t_0 as usize);
        let t_3 = CustomType0(t_1.to_string());
        Box::leak(Box::new(t_3))
    }
}

impl std::convert::AsMut<CustomType0> for CustomType1 {
    fn as_mut(&mut self) -> &mut CustomType0 {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 50);
        let custom_impl_inst_num = self.0;
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0{
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector{
                    1 => global_data.first_half,
                    _ => global_data.second_half,
        };
        let mut t_10 = _to_u8(GLOBAL_DATA, 58) % 17;
        let t_11 = _to_str(GLOBAL_DATA, 59, 59 + t_10 as usize);
        let mut t_13 = CustomType0(t_11.to_string());
        Box::leak(Box::new(t_13))
    }
}

fn _custom_fn0(str0: &CustomType2) -> &CustomType0 {
    let global_data = get_global_data();
    let GLOBAL_DATA = global_data.first_half;
    let custom_impl_num = _to_usize(GLOBAL_DATA, 92);
    let custom_impl_inst_num = str0.0.len();
    let selector = (custom_impl_num + custom_impl_inst_num) % 3;
    if selector == 0{
        panic!("INTENTIONAL PANIC!");
    }
    let GLOBAL_DATA = match selector{
                1 => global_data.first_half,
                _ => global_data.second_half,
    };
    let mut t_19 = _to_u8(GLOBAL_DATA, 100) % 17;
    let t_20 = _to_str(GLOBAL_DATA, 101, 101 + t_19 as usize);
    let t_22 = CustomType0(t_20.to_string());
    Box::leak(Box::new(t_22))
}

fn main (){
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 500 {return;}
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let num_ops = _to_u8(GLOBAL_DATA, 0);
        let mut cursor = 1;
        let mut stack = stack_dst::StackA::<str, [usize; 9]>::new();
        let mut value_instance = None;

        for _ in 0..num_ops {
            if cursor >= GLOBAL_DATA.len() { break; }
            let op = _to_u8(GLOBAL_DATA, cursor);
            cursor += 1;

            match op % 6 {
                0 => {
                    let len = _to_u8(GLOBAL_DATA, cursor) as usize % 65;
                    cursor += 1;
                    if cursor + len > GLOBAL_DATA.len() { break; }
                    let s = _to_str(GLOBAL_DATA, cursor, cursor + len);
                    let _ = stack.push_str(s);
                    cursor += len;
                }
                1 => {
                    let len = _to_u8(GLOBAL_DATA, cursor) as usize % 65;
                    cursor += 1;
                    if cursor + len > GLOBAL_DATA.len() { break; }
                    let s = String::from(_to_str(GLOBAL_DATA, cursor, cursor + len));
                    let custom = CustomType2(s);
                    let cnt = stack.push_stable(custom, |c| String::leak(c.0.clone()));
                    if let Err(_) = cnt { panic!("INTENTIONAL PANIC!"); }
                    cursor += len;
                }
                2 => {
                    stack.pop();
                }
                3 => {
                    if let Some(top) = stack.top() {
                        println!("{:?}", top);
                    }
                }
                4 => {
                    if let Some(top) = stack.top_mut() {
                        println!("{:?}", top);
                    }
                }
                5 => {
                    let len = _to_u8(GLOBAL_DATA, cursor) as usize % 17;
                    cursor += 1;
                    if cursor + len > GLOBAL_DATA.len() { break; }
                    let s = String::from(_to_str(GLOBAL_DATA, cursor, cursor + len));
                    let t = CustomType2(s);
                    let val = stack_dst::ValueA::<CustomType0, [usize; 9]>::new_stable(t, _custom_fn0);
                    cursor += len;
                    if let Ok(v) = val {
                        value_instance = Some(v);
                    }
                }
                _ => {}
            }
        }

        let mut t_15 = _to_u8(GLOBAL_DATA, cursor) % 17;
        cursor += 1;
        let t_16 = _to_str(GLOBAL_DATA, cursor, cursor + t_15 as usize);
        let t_17 = String::from(t_16);
        let t_18 = CustomType2(t_17);
        let t_25 = stack_dst::ValueA::<CustomType0, [usize;9]>::new_stable(t_18, _custom_fn0);
        let t_26 = _unwrap_result(t_25);
        let t_27 = &t_26;
        t_27.deref();
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