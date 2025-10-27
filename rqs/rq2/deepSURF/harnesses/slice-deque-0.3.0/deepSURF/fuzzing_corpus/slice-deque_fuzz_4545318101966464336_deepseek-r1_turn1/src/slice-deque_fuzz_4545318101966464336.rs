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
        let t_4 = String::from(t_3);
        CustomType0(t_4)
    }
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 4096 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let first_half = global_data.first_half;
        let second_half = global_data.second_half;

        let num_elems = _to_u8(first_half, 0) % 65;
        let mut current_offset = 1;
        let mut vec = Vec::with_capacity(num_elems as usize);

        for _ in 0..num_elems {
            let str_len = _to_u8(first_half, current_offset) % 17;
            current_offset += 1;
            let s = _to_str(first_half, current_offset, current_offset + str_len as usize);
            current_offset += str_len as usize;
            vec.push(CustomType0(s.to_string()));
        }

        let constructor = _to_u8(first_half, current_offset) % 3;
        current_offset += 1;

        let mut deque = match constructor {
            0 => SliceDeque::from(&vec[..]),
            1 => SliceDeque::with_capacity(_to_usize(first_half, current_offset) % 128),
            _ => SliceDeque::from_iter(vec.into_iter()),
        };

        let ops_count = _to_u8(second_half, 0) % 20;
        let mut ops_offset = 1;

        for _ in 0..ops_count {
            let op = _to_u8(second_half, ops_offset) % 7;
            ops_offset += 1;

            match op {
                0 => {
                    let str_len = _to_u8(second_half, ops_offset) % 17;
                    ops_offset += 1;
                    let s = _to_str(second_half, ops_offset, ops_offset + str_len as usize);
                    ops_offset += str_len as usize;
                    deque.push_back(CustomType0(s.to_string()));
                }
                1 => {
                    let str_len = _to_u8(second_half, ops_offset) % 17;
                    ops_offset += 1;
                    let s = _to_str(second_half, ops_offset, ops_offset + str_len as usize);
                    ops_offset += str_len as usize;
                    deque.push_front(CustomType0(s.to_string()));
                }
                2 => {
                    deque.pop_back();
                }
                3 => {
                    deque.pop_front();
                }
                4 => {
                    let trunc_len = _to_usize(second_half, ops_offset) % (deque.len() + 1);
                    ops_offset += std::mem::size_of::<usize>();
                    deque.truncate(trunc_len);
                }
                5 => {
                    let slice = deque.as_slice();
                    println!("{:?}", slice);
                }
                6 => {
                    let splice_value = CustomType0(_to_str(second_half, ops_offset, ops_offset + 8).to_string());
                    let mut temp_deque = SliceDeque::new();
                    temp_deque.push_back(splice_value);
                    let _ = deque.splice(0..0, temp_deque.into_iter());
                }
                _ => unreachable!(),
            }
        }

        let reserve_amount = _to_usize(second_half, ops_offset);
        deque.reserve(reserve_amount);

        ops_offset += std::mem::size_of::<usize>();
        let post_ops = _to_u8(second_half, ops_offset) % 5;
        ops_offset += 1;

        for _ in 0..post_ops {
            let post_op = _to_u8(second_half, ops_offset) % 4;
            ops_offset += 1;

            match post_op {
                0 => {
                    let mut new_deque = SliceDeque::new();
                    let new_elems = _to_u8(second_half, ops_offset) % 8;
                    ops_offset += 1;
                    for _ in 0..new_elems {
                        let str_len = _to_u8(second_half, ops_offset) % 17;
                        ops_offset += 1;
                        let s = _to_str(second_half, ops_offset, ops_offset + str_len as usize);
                        ops_offset += str_len as usize;
                        new_deque.push_back(CustomType0(s.to_string()));
                    }
                    deque.append(&mut new_deque);
                }
                1 => {
                    if let Some(elem) = deque.front_mut() {
                        *elem = CustomType0("mutated".to_string());
                        println!("{:?}", elem);
                    }
                }
                2 => {
                    let (s1, s2) = deque.as_slices();
                    println!("{:?} {:?}", s1, s2);
                }
                3 => {
                    let capacity = deque.capacity();
                    deque.reserve_exact(capacity + _to_usize(second_half, ops_offset));
                    ops_offset += std::mem::size_of::<usize>();
                }
                _ => unreachable!(),
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