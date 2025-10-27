#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use slice_deque::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut};

struct CustomType0(String);

impl std::clone::Clone for CustomType0 {
    fn clone(&self) -> Self {
        let global_data = get_global_data();
        let selector = (_to_usize(global_data.first_half, 1) + self.0.len()) % 3;
        if selector == 0 {
            panic!("INTENTIONAL PANIC!");
        }
        let part = match selector {
            1 => global_data.first_half,
            _ => global_data.second_half,
        };
        let len_byte = _to_u8(part, 9) % 17;
        let s = _to_str(part, 10, 10 + len_byte as usize);
        CustomType0(s.to_string())
    }
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 4096 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let num_elems = _to_usize(GLOBAL_DATA, 0) % 65;
        let mut current_index = 1;
        let mut elements = Vec::with_capacity(num_elems);
        for _ in 0..num_elems {
            let len = _to_u8(GLOBAL_DATA, current_index) % 17;
            current_index += 1;
            let s = _to_str(GLOBAL_DATA, current_index, current_index + len as usize);
            current_index += len as usize;
            elements.push(CustomType0(s.to_string()));
        }

        let mut deque = SliceDeque::from(elements.as_slice());

        let op_count = _to_u8(GLOBAL_DATA, current_index) % 10 + 5;
        current_index += 1;

        for _ in 0..op_count {
            if current_index >= GLOBAL_DATA.len() { break; }
            
            let op = _to_u8(GLOBAL_DATA, current_index) % 7;
            current_index += 1;

            match op {
                0 => {
                    let len = _to_u8(GLOBAL_DATA, current_index) % 17;
                    current_index += 1;
                    let s = _to_str(GLOBAL_DATA, current_index, current_index + len as usize);
                    current_index += len as usize;
                    deque.push_back(CustomType0(s.to_string()));
                }
                1 => {
                    let len = _to_u8(GLOBAL_DATA, current_index) % 17;
                    current_index += 1;
                    let s = _to_str(GLOBAL_DATA, current_index, current_index + len as usize);
                    current_index += len as usize;
                    deque.push_front(CustomType0(s.to_string()));
                }
                2 => {
                    let _ = deque.pop_back();
                }
                3 => {
                    let _ = deque.pop_front();
                }
                4 => {
                    let start = _to_usize(GLOBAL_DATA, current_index);
                    current_index += 8;
                    let end = _to_usize(GLOBAL_DATA, current_index);
                    current_index += 8;
                    if start <= deque.len() && end <= deque.len() {
                        let _ = deque.drain(start..end);
                    }
                }
                5 => {
                    deque.clear();
                }
                6 => {
                    let start = _to_usize(GLOBAL_DATA, current_index);
                    current_index += 8;
                    let end = _to_usize(GLOBAL_DATA, current_index);
                    current_index += 8;
                    let replace_count = _to_usize(GLOBAL_DATA, current_index) % 5;
                    current_index += 8;
                    let mut replacements = Vec::new();
                    for _ in 0..replace_count {
                        let len = _to_u8(GLOBAL_DATA, current_index) % 17;
                        current_index += 1;
                        let s = _to_str(GLOBAL_DATA, current_index, current_index + len as usize);
                        current_index += len as usize;
                        replacements.push(CustomType0(s.to_string()));
                    }
                    let _ = deque.splice(start..end, replacements);
                }
                _ => (),
            }

            if let Some(front) = deque.front() {
                println!("{:?}", front.0);
            }
            if let Some(back) = deque.back_mut() {
                back.0.push_str("_modified");
            }
        }

        deque.clear();

        let (s1, s2) = deque.as_slices();
        println!("Slices: {} {}", s1.len(), s2.len());
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