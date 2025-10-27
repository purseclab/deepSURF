#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::borrow::BorrowMut;
use std::ops::{Deref, DerefMut};

#[derive(Debug, PartialEq, PartialOrd)]
struct CustomType1(usize);

impl core::clone::Clone for CustomType1 {
    fn clone(&self) -> Self {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 10);
        let custom_impl_inst_num = self.0;
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0 {
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector {
            1 => global_data.first_half,
            _ => global_data.second_half,
        };
        let t_4 = _to_usize(GLOBAL_DATA, 18);
        CustomType1(t_4)
    }
}

impl core::marker::Copy for CustomType1 {}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 2048 {
            return;
        }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let num_ops = _to_u8(GLOBAL_DATA, 0) % 10 + 5;
        let mut sv: SmallVec<[CustomType1; 32]> = SmallVec::new();
        let mut secondary_sv: SmallVec<[CustomType1; 32]> = SmallVec::with_capacity(64);
        let mut operations_executed = 0;

        for i in 0..num_ops {
            let op_selector = _to_u8(GLOBAL_DATA, i as usize + 1) % 12;
            match op_selector {
                0 => {
                    let cnt = _to_usize(GLOBAL_DATA, ((i as usize) * 8)) % 65;
                    for j in 0..cnt {
                        let val = _to_usize(GLOBAL_DATA, (i as usize * 8 + j * 8) as usize);
                        sv.push(CustomType1(val));
                    }
                }
                1 => {
                    if !sv.is_empty() {
                        secondary_sv.extend(sv.drain(..));
                    }
                }
                2 => {
                    let idx = _to_usize(GLOBAL_DATA, (i as usize * 8));
                    let val = _to_usize(GLOBAL_DATA, (i as usize * 8 + 8));
                    if idx <= sv.len() {
                        sv.insert(idx, CustomType1(val));
                    }
                }
                3 => {
                    if !sv.is_empty() {
                        sv.truncate(sv.len() / 2);
                    }
                }
                4 => {
                    let _: &mut [_] = sv.borrow_mut();
                }
                5 => {
                    let slice = sv.as_slice();
                    println!("{:?}", slice);
                    let mut new_sv: SmallVec<[CustomType1; 32]> = SmallVec::from_slice(slice);
                    secondary_sv.append(&mut new_sv);
                }
                6 => {
                    let cap = _to_usize(GLOBAL_DATA, (i as usize * 8));
                    let mut temp_sv = SmallVec::with_capacity(cap);
                    temp_sv.extend_from_slice(sv.as_slice());
                    sv = temp_sv;
                }
                7 => {
                    if !secondary_sv.is_empty() {
                        let new_len = _to_usize(GLOBAL_DATA, i as usize * 8) % (secondary_sv.len() + 1);
                        secondary_sv.truncate(new_len);
                    }
                }
                8 => {
                    let cmp_result = sv.partial_cmp(&secondary_sv);
                    println!("Comparison: {:?}", cmp_result);
                }
                9 => {
                    let new_cap = _to_usize(GLOBAL_DATA, i as usize * 8);
                    let _ = sv.try_reserve(new_cap);
                }
                10 => {
                    if !sv.is_empty() {
                        sv.shrink_to_fit();
                    }
                }
                _ => {
                    secondary_sv.as_mut_slice().reverse();
                }
            };

            if operations_executed % 2 == 0 && !secondary_sv.is_empty() {
                let split_point = _to_usize(GLOBAL_DATA, i as usize * 8) % secondary_sv.len();
                let drained = secondary_sv.drain(0..split_point);
                sv.extend(drained);
            }

            if i % 3 == 0 && !secondary_sv.is_empty() {
                let removed = secondary_sv.remove(_to_usize(GLOBAL_DATA, (i as usize * 8)) % secondary_sv.len());
                sv.push(removed);
            }

            operations_executed += 1;
        }

        let mut split_sv = SmallVec::<[CustomType1; 16]>::from_vec(sv.into_vec());
        let _: &mut [_] = split_sv.borrow_mut();
        println!("{:?}", split_sv.as_ptr());
        println!("Final capacity: {}", split_sv.capacity());
        split_sv.shrink_to_fit();
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