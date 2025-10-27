#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use stackvector::*;
use global_data::*;
use std::ops::{Deref, DerefMut};

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 1024 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let constructor_selector = _to_u8(GLOBAL_DATA, 0) % 3;
        let mut stack_vec = match constructor_selector {
            0 => {
                let len = _to_usize(GLOBAL_DATA, 1) % 65;
                let mut buffer = [0u8; 128];
                let data_start = 9;
                for i in 0..buffer.len() {
                    buffer[i] = if data_start + i < GLOBAL_DATA.len() {
                        GLOBAL_DATA[data_start + i]
                    } else {
                        0
                    };
                }
                StackVec::from_buf_and_len(buffer, len)
            }
            1 => {
                let elem = _to_u8(GLOBAL_DATA, 1);
                let len = _to_usize(GLOBAL_DATA, 2) % 65;
                StackVec::from_elem(elem, len)
            }
            2 => {
                let start = _to_usize(GLOBAL_DATA, 1);
                let end = _to_usize(GLOBAL_DATA, 9);
                let slice = if start < end && end <= GLOBAL_DATA.len() {
                    &GLOBAL_DATA[start..end]
                } else {
                    &[]
                };
                StackVec::from_slice(slice)
            }
            _ => unreachable!(),
        };

        let num_ops = _to_u8(GLOBAL_DATA, 17) % 16;
        for i in 0..num_ops {
            let op_selector = _to_u8(GLOBAL_DATA, 18 + i as usize) % 8;
            match op_selector {
                0 => {
                    let val = _to_u8(GLOBAL_DATA, (34 + i * 8) as usize);
                    stack_vec.push(val);
                }
                1 => {
                    let _ = stack_vec.pop();
                }
                2 => {
                    let index = _to_usize(GLOBAL_DATA, (34 + i * 8) as usize);
                    let val = _to_u8(GLOBAL_DATA, (34 + i * 8 + 8) as usize);
                    stack_vec.insert(index, val);
                }
                3 => {
                    let len = _to_usize(GLOBAL_DATA, (34 + i * 8) as usize);
                    stack_vec.truncate(len);
                }
                4 => {
                    stack_vec.dedup_by_key(|x| *x % 2);
                }
                5 => {
                    let index = _to_usize(GLOBAL_DATA, (34 + i * 8) as usize);
                    let _ = stack_vec.swap_remove(index);
                }
                6 => {
                    stack_vec.dedup();
                }
                7 => {
                    if let Some(elem) = stack_vec.as_mut_slice().get_mut(0) {
                        *elem = _to_u8(GLOBAL_DATA, (34 + i * 8) as usize);
                    }
                }
                _ => unreachable!(),
            }
        }

        stack_vec.dedup();

        println!("{:?}", stack_vec.as_slice());
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