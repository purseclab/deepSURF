#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use slice_deque::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

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
        let t_4 = String::from(t_3);
        CustomType0(t_4)
    }
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 2100 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let mut operations = _to_usize(GLOBAL_DATA, 0) % 10;
        let mut sdq = SliceDeque::new();
        let mut buffer = Vec::new();

        for i in 0..(_to_u8(GLOBAL_DATA, 1) % 32) {
            let offset = 2 + i as usize * 18;
            let len = _to_u8(GLOBAL_DATA, offset) % 17;
            let s = _to_str(GLOBAL_DATA, offset + 1, offset + 1 + len as usize);
            buffer.push(CustomType0(s.to_string()));
        }

        let mut sdq2 = SliceDeque::from(buffer.as_slice());
        sdq.append(&mut sdq2);

        while operations > 0 {
            match _to_u8(GLOBAL_DATA, operations + 100) % 9 {
                0 => {
                    let len = _to_usize(GLOBAL_DATA, operations + 200);
                    sdq.truncate(len);
                }
                1 => {
                    let cap = _to_usize(GLOBAL_DATA, operations + 300);
                    sdq.reserve(cap);
                }
                2 => {
                    let idx = _to_usize(GLOBAL_DATA, operations + 400);
                    if let Some(elem) = sdq.get(idx) {
                        let _ = elem.0.len();
                    }
                }
                3 => {
                    let idx = _to_usize(GLOBAL_DATA, operations + 500);
                    if let Some(elem) = sdq.get_mut(idx) {
                        elem.0.push('!');
                    }
                }
                4 => {
                    let split_point = _to_usize(GLOBAL_DATA, operations + 600);
                    let mut new_deque = sdq.split_off(split_point);
                    sdq.append(&mut new_deque);
                }
                5 => {
                    let drain_start = _to_usize(GLOBAL_DATA, operations + 700);
                    let drain_end = _to_usize(GLOBAL_DATA, operations + 800);
                    let _: Vec<_> = sdq.drain(drain_start..drain_end).collect();
                }
                6 => {
                    let val_len = _to_u8(GLOBAL_DATA, operations + 900) % 17;
                    let s = _to_str(GLOBAL_DATA, operations + 1000, operations + 1000 + val_len as usize);
                    sdq.push_front(CustomType0(s.to_string()));
                }
                7 => {
                    if let Some(e) = sdq.pop_back() {
                        buffer.push(e);
                    }
                }
                8 => {
                    let filter_count = sdq.drain_filter(|x| x.0.len() % 2 == 0).count();
                    let _ = filter_count;
                }
                _ => {}
            }
            operations -= 1;
        }

        let mut iter = sdq.into_iter();
        let cloned_iter = iter.clone();
        let mut vec_buffer = Vec::new();
        
        for item in cloned_iter {
            let _ = item.0.capacity();
            vec_buffer.push(item);
        }

        let mut combined = SliceDeque::from_iter(vec_buffer);
        let (front, back) = combined.as_slices();
        let _ = front.len() + back.len();
        println!("{:?}", combined);
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