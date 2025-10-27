#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use through::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

#[derive(Debug)]
struct CustomType1(String);
#[derive(Debug)]
struct CustomType0(String);

fn _custom_fn0(str0: CustomType0) -> (CustomType0, CustomType1) {
    let global_data = get_global_data();
    let GLOBAL_DATA = global_data.first_half;
    let custom_impl_num = _to_usize(GLOBAL_DATA, 17);
    let custom_impl_inst_num = str0.0.len();
    let selector = (custom_impl_num + custom_impl_inst_num) % 3;
    if selector == 0 {
        panic!("INTENTIONAL PANIC!");
    }
    let GLOBAL_DATA = match selector {
        1 => global_data.first_half,
        _ => global_data.second_half,
    };
    let t_5 = _to_u8(GLOBAL_DATA, 25);
    let t_6 = _to_str(GLOBAL_DATA, 26, 26 + t_5 as usize);
    let t_8 = CustomType0(t_6.to_string());
    let t_9 = _to_u8(GLOBAL_DATA, 42);
    let t_10 = _to_str(GLOBAL_DATA, 43, 43 + t_9 as usize);
    let t_12 = CustomType1(t_10.to_string());
    (t_8, t_12)
}

fn _custom_fn1(str0: CustomType0) -> (CustomType0, CustomType1) {
    let global_data = get_global_data();
    let GLOBAL_DATA = global_data.second_half;
    let custom_impl_num = _to_usize(GLOBAL_DATA, 13);
    let selector = (custom_impl_num + str0.0.len()) % 4;
    if selector == 0 {
        panic!("INTENTIONAL PANIC!");
    }
    let t_5 = _to_u8(GLOBAL_DATA, 37);
    let t_6 = _to_str(GLOBAL_DATA, 38, 38 + t_5 as usize);
    let t_9 = _to_u8(GLOBAL_DATA, 55);
    let t_10 = _to_str(GLOBAL_DATA, 56, 56 + t_9 as usize);
    (CustomType0(t_6.to_string()), CustomType1(t_10.to_string()))
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 512 { return; }
        set_global_data(data);
        let global_data = get_global_data();

        let mut target1 = {
            let g = global_data.first_half;
            let len = _to_u8(g, 0);
            CustomType0(_to_str(g, 1, 1 + len as usize).to_string())
        };

        let mut target2 = {
            let g = global_data.second_half;
            let len = _to_u8(g, 0);
            CustomType0(_to_str(g, 1, 1 + len as usize).to_string())
        };

        let num_ops = _to_u8(global_data.first_half, 100) % 8;
        let mut outputs = Vec::new();

        for i in 0..num_ops {
            let op_selector = _to_u8(global_data.second_half, i as usize) % 6;

            match op_selector {
                0 => through(&mut target1, |x| {
                    println!("[THROUGH] Before: {:?}", x);
                    CustomType0(x.0.chars().rev().collect())
                }),
                1 => outputs.push(through_and(&mut target1, _custom_fn0)),
                2 => {
                    let o = through_and(&mut target2, |x| {
                        through(&mut target1, |y| CustomType0(y.0.clone() + &x.0));
                        (x, CustomType1(String::new()))
                    });
                    outputs.push(o);
                }
                3 => {
                    through(&mut target2, |x| {
                        CustomType0(x.0[..x.0.len().saturating_sub(1)].to_string())
                    });
                    println!("After trim: {:?}", target2);
                }
                4 => outputs.extend(vec![
                    through_and(&mut target1, _custom_fn1),
                    through_and(&mut target2, _custom_fn0)
                ]),
                5 => {
                    let temp = through_and(&mut target1, _custom_fn1);
                    outputs.push(through_and(&mut target2, |x| (x, CustomType1(temp.0.clone()))));
                }
                _ => ()
            }
        }

        let _ = target1.0.len() + target2.0.len();
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