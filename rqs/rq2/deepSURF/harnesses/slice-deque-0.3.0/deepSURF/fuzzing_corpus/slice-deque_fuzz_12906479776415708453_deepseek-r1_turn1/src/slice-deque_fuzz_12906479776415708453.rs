#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use slice_deque::{SliceDeque, AllocError};
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut};

#[derive(Debug)]
struct CustomType0(String);

impl std::clone::Clone for CustomType0 {
    
    fn clone(&self) -> Self {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 1);
        let custom_impl_inst_num = self.0.len();
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0{
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector{
                    1 => global_data.first_half,
                    _ => global_data.second_half,
        };
        let mut t_2 = _to_u8(GLOBAL_DATA, 9) % 17;
        let t_3 = _to_str(GLOBAL_DATA, 10, 10 + t_2 as usize);
        let t_4 = String::from(t_3);
        let t_5 = CustomType0(t_4);
        return t_5;
    }
}

fn main (){
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 8192 {return;}
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let ops_count = _to_usize(GLOBAL_DATA, 0) % 32;
        let mut op_index = 8;

        let num_initial = _to_u8(GLOBAL_DATA, op_index) % 65;
        op_index += 1;
        let mut t_1 = Vec::with_capacity(num_initial as usize);
        for _ in 0..num_initial {
            if op_index + 1 >= GLOBAL_DATA.len() { break; }
            let str_len = _to_u8(GLOBAL_DATA, op_index) % 17;
            op_index += 1;
            let end = op_index + str_len as usize;
            if end > GLOBAL_DATA.len() { break; }
            let s = _to_str(GLOBAL_DATA, op_index, end);
            t_1.push(CustomType0(s.to_string()));
            op_index = end;
        }

        let constructor = _to_u8(GLOBAL_DATA, op_index) % 3;
        op_index += 1;
        let mut t_135 = match constructor {
            0 => SliceDeque::new(),
            1 => {
                let cap = _to_usize(GLOBAL_DATA, op_index) % 65;
                op_index += 8;
                SliceDeque::with_capacity(cap)
            },
            2 => SliceDeque::from(t_1.as_slice()),
            _ => unreachable!(),
        };

        for _ in 0..ops_count {
            if op_index + 1 >= GLOBAL_DATA.len() { break; }
            let op = _to_u8(GLOBAL_DATA, op_index) % 7;
            op_index += 1;

            match op {
                0 => {
                    if op_index + 1 >= GLOBAL_DATA.len() { continue; }
                    let str_len = _to_u8(GLOBAL_DATA, op_index) % 17;
                    op_index += 1;
                    let end = op_index + str_len as usize;
                    if end > GLOBAL_DATA.len() { continue; }
                    let s = _to_str(GLOBAL_DATA, op_index, end);
                    t_135.push_back(CustomType0(s.to_string()));
                    op_index = end;
                }
                1 => {
                    let len = _to_usize(GLOBAL_DATA, op_index);
                    op_index += 8;
                    t_135.truncate(len);
                }
                2 => {
                    if let Some(front) = t_135.front() {
                        println!("{:?}", front);
                    }
                }
                3 => {
                    if let Some(back) = t_135.back_mut() {
                        *back = CustomType0(String::new());
                    }
                }
                4 => {
                    let _ = t_135.pop_back();
                }
                5 => {
                    let slice = t_135.as_mut_slice();
                    println!("{:?}", slice);
                }
                6 => {
                    let extra = _to_usize(GLOBAL_DATA, op_index);
                    op_index += 8;
                    let _ = t_135.reserve(extra);
                }
                _ => unreachable!()
            }
        }

        let final_str_len = _to_u8(GLOBAL_DATA, op_index) % 17;
        op_index += 1;
        let end = op_index + final_str_len as usize;
        if end <= GLOBAL_DATA.len() {
            let s = _to_str(GLOBAL_DATA, op_index, end);
            t_135.push_back(CustomType0(s.to_string()));
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