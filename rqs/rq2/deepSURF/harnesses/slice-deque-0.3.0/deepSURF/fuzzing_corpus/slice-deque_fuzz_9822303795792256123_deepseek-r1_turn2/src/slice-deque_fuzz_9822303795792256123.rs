#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use slice_deque::*;
use global_data::*;
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
        if selector == 0 {
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector {
            1 => global_data.first_half,
            _ => global_data.second_half,
        };
        let mut t_2 = _to_u8(GLOBAL_DATA, 9) % 17;
        let t_3 = _to_str(GLOBAL_DATA, 10, 10 + t_2 as usize);
        CustomType0(String::from(t_3))
    }
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 4096 {return;}
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let mut index = 0;
        let vec_len = _to_usize(GLOBAL_DATA, index) % 65;
        index += 1;

        let mut elems = Vec::with_capacity(vec_len);
        for _ in 0..vec_len {
            let str_len = _to_u8(GLOBAL_DATA, index) % 32;
            index += 1;
            let s = _to_str(GLOBAL_DATA, index, index + str_len as usize);
            index += str_len as usize;
            elems.push(CustomType0(s.to_string()));
        }

        let init_type = _to_u8(GLOBAL_DATA, index) % 3;
        index += 1;

        let mut deque = match init_type {
            0 => SliceDeque::from(&elems[..]),
            1 => from_elem(elems.pop().unwrap(), _to_usize(GLOBAL_DATA, index) % 65),
            _ => SliceDeque::with_capacity(_to_usize(GLOBAL_DATA, index) % 65),
        };

        let ops = _to_u8(GLOBAL_DATA, index) % 8 + 2;
        index += 1;

        for _ in 0..ops {
            let op = _to_u8(GLOBAL_DATA, index) % 6;
            index += 1;

            match op {
                0 => {
                    let mut new_deque = deque.clone();
                    let suffix = _to_str(GLOBAL_DATA, index, index + 4);
                    index += 4;
                    new_deque.push_back(CustomType0(suffix.to_string()));
                    println!("Cloned len: {}", new_deque.len());
                }
                1 => {
                    let trunc = _to_usize(GLOBAL_DATA, index) % (deque.len().max(1) + 1);
                    index += 1;
                    deque.truncate(trunc);
                }
                2 => {
                    if let Some(front) = deque.front_mut() {
                        let mod_len = _to_u8(GLOBAL_DATA, index) % 8;
                        index += 1;
                        front.0 = _to_str(GLOBAL_DATA, index, index + mod_len as usize).to_string();
                        index += mod_len as usize;
                    }
                }
                3 => {
                    if !deque.is_empty() {
                        let idx = _to_usize(GLOBAL_DATA, index);
                        index += 1;
                        println!("Item at {}: {:?}", idx % deque.len(), deque[idx % deque.len()]);
                    }
                }
                4 => {
                    let cap = _to_usize(GLOBAL_DATA, index) % 128;
                    index += 1;
                    let _ = deque.try_reserve(cap);
                }
                _ => {
                    let drain_range = _to_usize(GLOBAL_DATA, index) % (deque.len() + 1);
                    index += 1;
                    let _drained = deque.drain(..drain_range);
                }
            }

            let comparison = _to_u8(GLOBAL_DATA, index) % 32;
            index += comparison as usize;
            println!("Current state: {:?} {:?}", deque.front(), deque.back());
        }

        let mut iter = deque.clone().into_iter();
        while let Some(item) = iter.next() {
            println!("Final item: {:?}", item);
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