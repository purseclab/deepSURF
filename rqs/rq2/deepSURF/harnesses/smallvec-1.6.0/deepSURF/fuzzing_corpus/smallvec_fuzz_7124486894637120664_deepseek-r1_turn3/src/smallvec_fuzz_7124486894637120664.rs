#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::ops::{Deref, DerefMut};

#[derive(Debug)]
struct CustomType1(String);

impl Clone for CustomType1 {
    fn clone(&self) -> Self {
        let global_data = get_global_data();
        let selector = _to_u8(global_data.first_half, 10);
        if selector % 3 == 0 {
            panic!("INTENTIONAL PANIC!");
        }
        let str_len = _to_u8(global_data.second_half, 18) % 17;
        let s = _to_str(global_data.second_half, 19, 19 + str_len as usize);
        CustomType1(s.to_string())
    }
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 4096 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        
        let mut operations = _to_usize(global_data.first_half, 0) % 8;
        let mut smallvecs = Vec::with_capacity(4);
        let mut data_offset = 100;

        for _ in 0..4 {
            let constructor = _to_u8(global_data.first_half, data_offset) % 4;
            data_offset += 1;

            let sv = match constructor {
                0 => {
                    let len = _to_usize(global_data.first_half, data_offset) % 65;
                    data_offset += 2;
                    let mut vec = Vec::with_capacity(len);
                    for _ in 0..len {
                        let s_len = _to_u8(global_data.first_half, data_offset) % 17;
                        data_offset += 1;
                        let s = _to_str(global_data.first_half, data_offset, data_offset + s_len as usize);
                        data_offset += s_len as usize;
                        vec.push(CustomType1(s.to_string()));
                    }
                    SmallVec::<[CustomType1; 32]>::from_vec(vec)
                }
                1 => {
                    let cap = _to_usize(global_data.first_half, data_offset);
                    data_offset += 2;
                    SmallVec::<[CustomType1; 32]>::with_capacity(cap)
                }
                2 => {
                    let len = _to_usize(global_data.first_half, data_offset) % 65;
                    data_offset += 2;
                    let mut sv = SmallVec::<[CustomType1; 32]>::new();
                    for _ in 0..len {
                        let s_len = _to_u8(global_data.first_half, data_offset) % 17;
                        data_offset += 1;
                        let s = _to_str(global_data.first_half, data_offset, data_offset + s_len as usize);
                        data_offset += s_len as usize;
                        sv.push(CustomType1(s.to_string()));
                    }
                    sv
                }
                _ => SmallVec::<[CustomType1; 32]>::new(),
            };
            smallvecs.push(sv);
        }

        while operations > 0 {
            let op = _to_u8(global_data.second_half, operations) % 6;
            match op {
                0 => {
                    if let Some(mut a) = smallvecs.pop() {
                        let mut b = smallvecs.pop().unwrap();
                        a.append(&mut b);
                        smallvecs.push(a);
                        smallvecs.push(b);
                    }
                }
                1 => {
                    let idx = _to_usize(global_data.second_half, operations) % smallvecs.len();
                    let cap = _to_usize(global_data.second_half, operations + 2);
                    smallvecs[idx].reserve(cap);
                }
                2 => {
                    let idx = _to_usize(global_data.second_half, operations) % smallvecs.len();
                    let len = _to_usize(global_data.second_half, operations + 2) % (smallvecs[idx].len() + 1);
                    smallvecs[idx].truncate(len);
                }
                3 => {
                    let idx = _to_usize(global_data.second_half, operations) % smallvecs.len();
                    let pos = _to_usize(global_data.second_half, operations + 2) % (smallvecs[idx].len() + 1);
                    let s_len = _to_u8(global_data.second_half, operations + 4) % 17;
                    let s = _to_str(global_data.second_half, operations + 5, operations + 5 + s_len as usize);
                    smallvecs[idx].insert(pos, CustomType1(s.to_string()));
                }
                4 => {
                    let idx = _to_usize(global_data.second_half, operations) % smallvecs.len();
                    println!("{:?}", smallvecs[idx].as_slice());
                }
                5 => {
                    let idx = _to_usize(global_data.second_half, operations) % smallvecs.len();
                    let _ = smallvecs[idx].pop();
                }
                _ => unreachable!(),
            }
            operations -= 1;
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