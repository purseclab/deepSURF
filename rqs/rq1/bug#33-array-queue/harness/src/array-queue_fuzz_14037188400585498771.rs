#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use array_queue::*;
use global_data::*;
use std::ops::{Deref, DerefMut};

#[derive(Debug)]
struct CustomType1(String);

impl Clone for CustomType1 {
    fn clone(&self) -> Self {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 0);
        let selector = (custom_impl_num + self.0.len()) % 3;
        if selector == 0 { panic!("INTENTIONAL PANIC!") }
        let data_slice = match selector {
            1 => global_data.first_half,
            _ => global_data.second_half,
        };
        let str_len = _to_u8(data_slice, 8) % 17;
        let s = _to_str(data_slice, 9, 9 + str_len as usize);
        CustomType1(String::from(s))
    }
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 1024 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        
        let constructor_sel = _to_u8(global_data.first_half, 0) % 2;
        let mut queue = match constructor_sel {
            0 => ArrayQueue::<[CustomType1; 64]>::new(),
            _ => ArrayQueue::<[CustomType1; 64]>::default()
        };

        let operation_count = _to_usize(global_data.second_half, 0) % 65;
        let mut data_ptr = 1;

        for _ in 0..operation_count {
            if data_ptr + 2 > global_data.first_half.len() { break; }
            let op_code = _to_u8(global_data.first_half, data_ptr) % 6;
            data_ptr += 1;

            match op_code {
                0 => {
                    let str_len = _to_u8(global_data.first_half, data_ptr) % 17;
                    data_ptr += 1;
                    let s = _to_str(global_data.first_half, data_ptr, data_ptr + str_len as usize);
                    data_ptr += str_len as usize;
                    let elem = CustomType1(s.to_string());
                    let _ = queue.push_front(&elem);
                    println!("Front: {:?}", queue.first());
                }
                1 => {
                    let pop_result = queue.pop_front();
                    println!("Popped: {:?}", pop_result);
                }
                2 => {
                    let str_len = _to_u8(global_data.first_half, data_ptr) % 17;
                    data_ptr += 1;
                    let s = _to_str(global_data.first_half, data_ptr, data_ptr + str_len as usize);
                    data_ptr += str_len as usize;
                    let elem = CustomType1(s.to_string());
                    let _ = queue.push_back(&elem);
                    println!("Back: {:?}", queue.last());
                }
                3 => {
                    let popped = queue.pop_back();
                    println!("Popped back: {:?}", popped);
                }
                4 => {
                    let cloned = queue.clone();
                    println!("Clone len: {}", cloned.len());
                }
                5 => {
                    println!("Capacity: {:?}", queue.is_full());
                    println!("Elements: {}", queue.len());
                }
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