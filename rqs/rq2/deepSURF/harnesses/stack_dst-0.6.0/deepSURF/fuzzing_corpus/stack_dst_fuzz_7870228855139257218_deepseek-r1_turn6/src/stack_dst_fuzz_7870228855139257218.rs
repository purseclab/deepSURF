#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use stack_dst::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};
use std::fmt::{self, Display};

struct CustomType0(String);
struct CustomType1(usize);
struct CustomType2(String);

impl Display for CustomType0 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
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
        let t_14 = Box::leak(t_13);
        return t_14;
    }
}

impl std::clone::Clone for CustomType1 {
    fn clone(&self) -> Self {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 34);
        let custom_impl_inst_num = self.0;
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0 {
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector {
            1 => global_data.first_half,
            _ => global_data.second_half,
        };
        let t_8 = _to_usize(GLOBAL_DATA, 42);
        let t_9 = CustomType1(t_8);
        return t_9;
    }
}

impl std::default::Default for CustomType1 {
    fn default() -> Self {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let t_5 = _to_u8(GLOBAL_DATA, 25);
        if t_5 % 2 == 0 {
            panic!("INTENTIONAL PANIC!");
        }
        let t_6 = _to_usize(GLOBAL_DATA, 26);
        let t_7 = CustomType1(t_6);
        return t_7;
    }
}

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
        let t_4 = Box::leak(t_3);
        return t_4;
    }
}

fn _custom_fn0(str0: &CustomType2) -> &CustomType0 {
    let global_data = get_global_data();
    let GLOBAL_DATA = global_data.first_half;
    let custom_impl_num = _to_usize(GLOBAL_DATA, 92);
    let custom_impl_inst_num = str0.0.len();
    let selector = (custom_impl_num + custom_impl_inst_num) % 3;
    if selector == 0 {
        panic!("INTENTIONAL PANIC!");
    }
    let GLOBAL_DATA = match selector {
        1 => global_data.first_half,
        _ => global_data.second_half,
    };
    let mut t_19 = _to_u8(GLOBAL_DATA, 100) % 17;
    let t_20 = _to_str(GLOBAL_DATA, 101, 101 + t_19 as usize);
    let t_21 = String::from(t_20);
    let t_22 = Box::new(CustomType0(t_21));
    let t_23 = Box::leak(t_22);
    return t_23;
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 512 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let num_ops = _to_usize(GLOBAL_DATA, 0) % 8;
        let mut stack = StackA::<dyn Display, [usize; 9]>::new();
        let mut data_offset = 75;

        for i in 1..=num_ops {
            let op_byte = _to_u8(GLOBAL_DATA, i);
            match op_byte % 7 {
                0 => {
                    let str_len = _to_u8(GLOBAL_DATA, data_offset) % 17;
                    let s = _to_str(GLOBAL_DATA, data_offset + 1, data_offset + 1 + str_len as usize);
                    let s_str = String::from(s);
                    if let Ok(_) = stack.push_stable(s_str, |v| v as &dyn Display) {
                        if let Some(top) = stack.top() {
                            println!("{}", top);
                        }
                    }
                    data_offset += 1 + str_len as usize;
                },
                1 => {
                    let t_15 = _to_u8(GLOBAL_DATA, data_offset) % 17;
                    let t_16 = _to_str(GLOBAL_DATA, data_offset + 1, data_offset + 1 + t_15 as usize);
                    let t_17 = String::from(t_16);
                    let t_18 = CustomType2(t_17);
                    let t_25 = ValueA::<CustomType0, [usize; 9]>::new_stable(t_18, _custom_fn0);
                    if let Ok(val) = t_25 {
                        if stack.push_stable(val, |v| v as &dyn Display).is_ok() {
                            if let Some(mut top) = stack.top_mut() {
                                let _ = top.deref_mut();
                            }
                        }
                    }
                    data_offset += 1 + t_15 as usize;
                },
                2 => {
                    stack.pop();
                },
                3 => {
                    if let Some(mut top) = stack.top_mut() {
                        let _ = top.deref_mut();
                    }
                },
                4 => {
                    if let Some(top) = stack.top() {
                        println!("{}", top);
                    }
                },
                5 => {
                    let arr_val = _to_usize(GLOBAL_DATA, data_offset) % 1024;
                    let slice: &[usize] = &[arr_val; 11];
                    let mut concrete_stack = StackA::<[usize], [usize; 9]>::new();
                    let _ = concrete_stack.push_cloned(slice);
                    data_offset += 1;
                },
                _ => {
                    let t_15 = _to_u8(GLOBAL_DATA, data_offset) % 17;
                    let t_16 = _to_str(GLOBAL_DATA, data_offset + 1, data_offset + 1 + t_15 as usize);
                    let t_17 = String::from(t_16);
                    let t_18 = CustomType2(t_17);
                    let t_25 = ValueA::<CustomType0, [usize; 9]>::new_stable(t_18, _custom_fn0);
                    if let Ok(val) = t_25 {
                        let _ = val.deref();
                    }
                    data_offset += 1 + t_15 as usize;
                }
            }
        }

        {
            let mut temp_stack = StackA::<dyn Display, [usize; 9]>::new();
            for _ in 0..3 {
                if let Some(item) = stack.top() {
                    println!("{}", item);
                    temp_stack.push_stable(item, |v| v as &dyn Display).ok();
                }
            }
        }
        for _ in 0..3 {
            stack.pop();
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