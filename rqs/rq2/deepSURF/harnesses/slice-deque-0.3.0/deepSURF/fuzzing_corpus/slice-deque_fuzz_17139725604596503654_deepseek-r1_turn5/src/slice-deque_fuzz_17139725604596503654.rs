#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use slice_deque::*;
use global_data::*;
use std::ops::{Deref, DerefMut, RangeBounds, Bound};
use std::fmt;

#[derive(Debug)]
struct CustomType0(String);

impl Clone for CustomType0 {
    fn clone(&self) -> Self {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 1);
        let selector = (custom_impl_num + self.0.len()) % 3;
        if selector == 0 { panic!("INTENTIONAL PANIC!") }
        let GLOBAL_DATA = match selector {
            1 => global_data.first_half,
            _ => global_data.second_half,
        };
        let str_len = _to_u8(GLOBAL_DATA, 9) % 17;
        let s = _to_str(GLOBAL_DATA, 10, 10 + str_len as usize);
        CustomType0(String::from(s))
    }
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 4096 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        
        let mut data_idx = 0;

        let vec_len = _to_u8(GLOBAL_DATA, data_idx) % 65;
        data_idx += 1;
        
        let mut elements = Vec::new();
        for _ in 0..vec_len {
            if data_idx + 1 > GLOBAL_DATA.len() { break; }
            let s_len = _to_u8(GLOBAL_DATA, data_idx) % 17;
            data_idx += 1;
            let s = if data_idx + s_len as usize > GLOBAL_DATA.len() {
                data_idx = GLOBAL_DATA.len();
                ""
            } else {
                let part = _to_str(GLOBAL_DATA, data_idx, data_idx + s_len as usize);
                data_idx += s_len as usize;
                part
            };
            elements.push(CustomType0(s.to_string()));
        }

        let constructor = _to_u8(GLOBAL_DATA, data_idx) % 6;
        data_idx += 1;

        let mut deque = match constructor {
            0 => SliceDeque::from(&elements[..]),
            1 => SliceDeque::with_capacity(_to_usize(GLOBAL_DATA, data_idx) % 1024),
            2 => SliceDeque::new(),
            3 => {
                if data_idx + 2 > GLOBAL_DATA.len() {
                    SliceDeque::new()
                } else {
                    let elem = if data_idx + 1 > GLOBAL_DATA.len() {
                        CustomType0(String::new())
                    } else {
                        let len = _to_u8(GLOBAL_DATA, data_idx) % 17;
                        data_idx += 1;
                        let s = _to_str(GLOBAL_DATA, data_idx, data_idx + len as usize);
                        data_idx += len as usize;
                        CustomType0(s.to_string())
                    };
                    slice_deque::from_elem(elem, _to_usize(GLOBAL_DATA, data_idx) % 65)
                }
            },
            4 => SliceDeque::from_iter(elements.iter().cloned()),
            _ => {
                let capacity = _to_usize(GLOBAL_DATA, data_idx) % 128;
                data_idx += 1;
                let mut deq = SliceDeque::with_capacity(capacity);
                deq.extend(elements);
                deq
            },
        };

        let num_ops = _to_u8(GLOBAL_DATA, data_idx) % 15;
        data_idx += 1;

        for _ in 0..num_ops {
            if data_idx >= GLOBAL_DATA.len() { break; }
            let op = _to_u8(GLOBAL_DATA, data_idx) % 9;
            data_idx += 1;

            match op {
                0 => {
                    let s_len = _to_u8(GLOBAL_DATA, data_idx) % 17;
                    data_idx += 1;
                    let s = _to_str(GLOBAL_DATA, data_idx, data_idx + s_len as usize);
                    data_idx += s_len as usize;
                    deque.push_back(CustomType0(s.to_string()));
                },
                1 => {
                    let s_len = _to_u8(GLOBAL_DATA, data_idx) % 17;
                    data_idx += 1;
                    let s = _to_str(GLOBAL_DATA, data_idx, data_idx + s_len as usize);
                    data_idx += s_len as usize;
                    deque.push_front(CustomType0(s.to_string()));
                },
                2 => { let _ = deque.pop_front(); },
                3 => { let _ = deque.pop_back(); },
                4 => deque.truncate(_to_usize(GLOBAL_DATA, data_idx) % (deque.len() + 1)),
                5 => {
                    let start = _to_usize(GLOBAL_DATA, data_idx) % (deque.len() + 1);
                    data_idx += 1;
                    let end = _to_usize(GLOBAL_DATA, data_idx) % (deque.len() + 1);
                    data_idx += 1;
                    let drain = deque.drain(start..end);
                    println!("Drained {} elements", drain.count());
                },
                6 => {
                    if let (front, back) = deque.as_slices() {
                        println!("Front slice len: {}", front.len());
                        println!("Back slice len: {}", back.len());
                    }
                },
                7 => {
                    deque.shrink_to_fit();
                    println!("Shrunk to capacity: {}", deque.capacity());
                },
                _ => {
                    if let Some(front) = deque.front_mut() {
                        front.0.push_str("_modified");
                    }
                    if let Some(back) = deque.back_mut() {
                        back.0.push_str("_mutated");
                    }
                }
            }
        }

        let start = _to_usize(GLOBAL_DATA, data_idx) % (deque.len() + 1);
        data_idx += 1;
        let end = _to_usize(GLOBAL_DATA, data_idx) % (deque.len() + 1);
        data_idx += 1;
        let drain = deque.drain(start..end);
        drain.for_each(|x| println!("{:?}", x));

        if !deque.is_empty() {
            let split_at = _to_usize(GLOBAL_DATA, data_idx) % deque.len();
            let mut split = deque.split_off(split_at);
            split.append(&mut deque);
            println!("Final split len: {}", split.len());
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