#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::ops::{Deref, DerefMut};
use std::cmp::Ordering;

#[derive(Debug, PartialEq, PartialOrd)]
struct CustomType1(String);

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 4096 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let mut offset = 0;
        let op_count = _to_u8(GLOBAL_DATA, offset) % 8;
        offset += 1;

        let mut sv_list = Vec::new();

        for _ in 0..op_count {
            let op_type = _to_u8(GLOBAL_DATA, offset) % 7;
            offset += 1;

            match op_type {
                0 => {
                    let capacity = _to_usize(GLOBAL_DATA, offset);
                    offset += _to_usize(GLOBAL_DATA, offset.wrapping_add(1)) % 8;
                    sv_list.push(SmallVec::<[CustomType1; 32]>::with_capacity(capacity));
                }
                1 => {
                    let elem_count = _to_usize(GLOBAL_DATA, offset) % 65;
                    offset += 1;
                    let mut vec = Vec::new();
                    for _ in 0..elem_count {
                        let slice_len = _to_usize(GLOBAL_DATA, offset) % 20;
                        offset += 1;
                        vec.push(CustomType1(_to_str(GLOBAL_DATA, offset, offset + slice_len).to_string()));
                        offset = offset.wrapping_add(slice_len);
                    }
                    let sv = SmallVec::<[CustomType1; 32]>::from_vec(vec);
                    sv_list.push(sv);
                }
                2 => {
                    let idx = _to_usize(GLOBAL_DATA, offset) % sv_list.len().max(1);
                    offset += 1;
                    if let Some(sv) = sv_list.get_mut(idx) {
                        let new_len = _to_usize(GLOBAL_DATA, offset);
                        offset += 1;
                        sv.truncate(new_len);
                    }
                }
                3 => {
                    let idx = _to_usize(GLOBAL_DATA, offset) % sv_list.len().max(1);
                    offset += 1;
                    if let Some(sv) = sv_list.get(idx) {
                        let other_idx = _to_usize(GLOBAL_DATA, offset) % sv_list.len().max(1);
                        offset += 1;
                        if let Some(other) = sv_list.get(other_idx) {
                            let _ = sv.partial_cmp(other);
                        }
                    }
                }
                4 => {
                    let idx = _to_usize(GLOBAL_DATA, offset) % sv_list.len().max(1);
                    offset += 1;
                    if let Some(sv) = sv_list.get(idx) {
                        println!("{:?}", sv.as_slice());
                    }
                }
                5 => {
                    let idx = _to_usize(GLOBAL_DATA, offset) % sv_list.len().max(1);
                    offset += 1;
                    if let Some(sv) = sv_list.get_mut(idx) {
                        let range = _to_usize(GLOBAL_DATA, offset).._to_usize(GLOBAL_DATA, offset + 4);
                        offset += 8;
                        let _: Vec<_> = sv.drain(range).collect();
                    }
                }
                _ => {
                    let mut vec = Vec::new();
                    let elem_count = _to_usize(GLOBAL_DATA, offset) % 65;
                    offset += 1;
                    for _ in 0..elem_count {
                        let slice_len = _to_usize(GLOBAL_DATA, offset) % 20;
                        offset += 1;
                        vec.push(CustomType1(_to_str(GLOBAL_DATA, offset, offset + slice_len).to_string()));
                        offset = offset.wrapping_add(slice_len);
                    }
                    let sv = SmallVec::<[CustomType1; 32]>::from_vec(vec);
                    sv_list.push(sv);
                }
            }
        }

        if let Some(sv) = sv_list.last() {
            println!("Final capacity: {}", sv.capacity());
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