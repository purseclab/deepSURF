#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use slice_deque::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut};

#[derive(Debug, PartialEq)]
struct CustomType0(String);

impl std::clone::Clone for CustomType0 {
    fn clone(&self) -> Self {
        let global_data = get_global_data();
        let selector_data = global_data.first_half;
        let custom_impl_num = _to_usize(selector_data, 1);
        let custom_impl_inst_num = self.0.len();
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0 {
            panic!("INTENTIONAL PANIC!");
        }
        let selector_data = match selector {
            1 => global_data.first_half,
            _ => global_data.second_half,
        };
        let str_len = _to_u8(selector_data, 9) % 17;
        let s = _to_str(selector_data, 10, 10 + str_len as usize);
        CustomType0(s.to_string())
    }
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 2048 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let mut data_idx = 0;

        let op_count = _to_u8(GLOBAL_DATA, data_idx) % 50;
        data_idx += 1;

        let constructor_select = _to_u8(GLOBAL_DATA, data_idx) % 4;
        data_idx += 1;
        
        let mut deque = match constructor_select {
            0 => SliceDeque::new(),
            1 => {
                let cap = _to_usize(GLOBAL_DATA, data_idx) % 65;
                data_idx += 2;
                SliceDeque::with_capacity(cap)
            }
            2 => {
                let elem_count = _to_usize(GLOBAL_DATA, data_idx) % 65;
                data_idx += 2;
                if data_idx + 2 > GLOBAL_DATA.len() { return; }
                let str_len = _to_u8(GLOBAL_DATA, data_idx) % 17;
                data_idx += 1;
                let s = _to_str(GLOBAL_DATA, data_idx, data_idx + str_len as usize);
                data_idx += str_len as usize;
                slice_deque::from_elem(CustomType0(s.to_string()), elem_count)
            }
            3 => {
                let vec_len = _to_usize(GLOBAL_DATA, data_idx) % 65;
                data_idx += 2;
                let mut items = Vec::with_capacity(vec_len);
                for _ in 0..vec_len {
                    if data_idx + 1 > GLOBAL_DATA.len() { break; }
                    let str_len = _to_u8(GLOBAL_DATA, data_idx) % 17;
                    data_idx += 1;
                    if data_idx + str_len as usize > GLOBAL_DATA.len() { break; }
                    let s = _to_str(GLOBAL_DATA, data_idx, data_idx + str_len as usize);
                    data_idx += str_len as usize;
                    items.push(CustomType0(s.to_string()));
                }
                SliceDeque::from(items.as_slice())
            }
            _ => unreachable!(),
        };

        for _ in 0..op_count {
            if data_idx >= GLOBAL_DATA.len() { break; }
            let op_type = _to_u8(GLOBAL_DATA, data_idx) % 13;
            data_idx += 1;

            match op_type {
                0 => {
                    if data_idx + 2 > GLOBAL_DATA.len() { continue; }
                    let str_len = _to_u8(GLOBAL_DATA, data_idx) % 17;
                    data_idx += 1;
                    let s = _to_str(GLOBAL_DATA, data_idx, data_idx + str_len as usize);
                    data_idx += str_len as usize;
                    deque.push_back(CustomType0(s.to_string()));
                }
                1 => {
                    if data_idx + 2 > GLOBAL_DATA.len() { continue; }
                    let str_len = _to_u8(GLOBAL_DATA, data_idx) % 17;
                    data_idx += 1;
                    let s = _to_str(GLOBAL_DATA, data_idx, data_idx + str_len as usize);
                    data_idx += str_len as usize;
                    deque.push_front(CustomType0(s.to_string()));
                }
                2 => {
                    if let Some(val) = deque.pop_back() {
                        println!("{:?}", val);
                    }
                }
                3 => {
                    if let Some(val) = deque.pop_front() {
                        println!("{:?}", val);
                    }
                }
                4 => {
                    let new_len = _to_usize(GLOBAL_DATA, data_idx);
                    data_idx += 2;
                    deque.truncate(new_len);
                }
                5 => {
                    let index = _to_usize(GLOBAL_DATA, data_idx) % deque.len().max(1);
                    data_idx += 2;
                    if let Some(val) = deque.swap_remove_back(index) {
                        println!("Removed: {:?}", val);
                    }
                }
                6 => {
                    let start = _to_usize(GLOBAL_DATA, data_idx) % deque.len().max(1);
                    data_idx += 2;
                    let end = _to_usize(GLOBAL_DATA, data_idx) % (deque.len() + 1);
                    data_idx += 2;
                    let mut drain = deque.drain(start..end);
                    while let Some(item) = drain.next() {
                        println!("Draining: {:?}", item);
                    }
                }
                7 => {
                    let value = if let Some(val) = deque.back_mut() {
                        *val = CustomType0("modified".into());
                        val
                    } else {
                        &mut CustomType0(String::new())
                    };
                    println!("Modified back: {:?}", value);
                }
                8 => {
                    let iter = deque.iter();
                    let mut count = 0;
                    for item in iter {
                        println!("Iter {}: {:?}", count, item);
                        count += 1;
                    }
                }
                9 => {
                    let index = _to_usize(GLOBAL_DATA, data_idx) % (deque.len() + 1);
                    data_idx += 2;
                    if data_idx + 1 > GLOBAL_DATA.len() { continue; }
                    let str_len = _to_u8(GLOBAL_DATA, data_idx) % 17;
                    data_idx += 1;
                    let s = _to_str(GLOBAL_DATA, data_idx, data_idx + str_len as usize);
                    data_idx += str_len as usize;
                    deque.insert(index, CustomType0(s.to_string()));
                }
                10 => {
                    deque.dedup();
                }
                11 => {
                    let additional = _to_usize(GLOBAL_DATA, data_idx);
                    data_idx += 2;
                    deque.reserve(additional);
                }
                12 => {
                    let (s1, s2) = deque.as_slices();
                    println!("Slices: {:?} | {:?}", s1, s2);
                }
                _ => unreachable!(),
            }
        }

        println!("Front: {:?}", deque.front());
        if let Some(back) = deque.back_mut() {
            *back = CustomType0("modified_back".into());
            println!("Back: {:?}", back);
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