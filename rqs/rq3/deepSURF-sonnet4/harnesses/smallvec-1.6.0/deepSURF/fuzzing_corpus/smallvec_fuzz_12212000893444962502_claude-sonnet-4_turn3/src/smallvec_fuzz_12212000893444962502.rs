#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

#[derive(Debug)]
struct CustomType0(String);
#[derive(Debug)]
struct CustomType1(String);

fn _custom_fn0(_: &mut CustomType1) -> bool {
    let global_data = get_global_data();
    let GLOBAL_DATA = global_data.first_half;
    let t_9 = _to_u8(GLOBAL_DATA, 34);
    if t_9 % 2 == 0{
        panic!("INTENTIONAL PANIC!");
    }
    let t_10 = _to_bool(GLOBAL_DATA, 35);
    return t_10;
}

fn _custom_fn1(_: &mut u32) -> bool {
    let global_data = get_global_data();
    let GLOBAL_DATA = global_data.second_half;
    let t_1 = _to_u8(GLOBAL_DATA, 5);
    if t_1 % 3 == 0{
        panic!("INTENTIONAL PANIC!");
    }
    let t_2 = _to_bool(GLOBAL_DATA, 6);
    return t_2;
}

fn _custom_fn2(_: &mut i64) -> bool {
    let global_data = get_global_data();
    let GLOBAL_DATA = global_data.first_half;
    let t_3 = _to_u8(GLOBAL_DATA, 50);
    if t_3 % 4 == 0{
        panic!("INTENTIONAL PANIC!");
    }
    return true;
}

fn main (){
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 400 {return;}
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let num_operations = _to_u8(GLOBAL_DATA, 0) % 10 + 1;
        
        for i in 0..num_operations {
            let op_type = _to_u8(GLOBAL_DATA, 1 + i as usize) % 8;
            
            match op_type {
                0 => {
                    let constructor_choice = _to_u8(GLOBAL_DATA, 20 + i as usize) % 5;
                    let mut sv = match constructor_choice {
                        0 => smallvec::SmallVec::<[u32; 16]>::new(),
                        1 => {
                            let cap = _to_usize(GLOBAL_DATA, 30 + i as usize * 8);
                            smallvec::SmallVec::<[u32; 16]>::with_capacity(cap)
                        },
                        2 => {
                            let vec_size = _to_usize(GLOBAL_DATA, 40 + i as usize * 8) % 65;
                            let vec: Vec<u32> = (0..vec_size).map(|j| _to_u32(GLOBAL_DATA, 50 + j * 4)).collect();
                            smallvec::SmallVec::<[u32; 16]>::from_vec(vec)
                        },
                        3 => {
                            let elem = _to_u32(GLOBAL_DATA, 60 + i as usize * 4);
                            let count = _to_usize(GLOBAL_DATA, 70 + i as usize * 8) % 65;
                            smallvec::SmallVec::<[u32; 16]>::from_elem(elem, count)
                        },
                        _ => {
                            let slice_size = _to_usize(GLOBAL_DATA, 80 + i as usize * 8) % 65;
                            let mut temp_vec = Vec::new();
                            for j in 0..slice_size {
                                temp_vec.push(_to_u32(GLOBAL_DATA, 90 + j * 4));
                            }
                            smallvec::SmallVec::<[u32; 16]>::from_slice(&temp_vec)
                        }
                    };

                    let push_count = _to_u8(GLOBAL_DATA, 100 + i as usize) % 10;
                    for j in 0..push_count {
                        let val = _to_u32(GLOBAL_DATA, 110 + j as usize * 4);
                        sv.push(val);
                    }

                    sv.retain(_custom_fn1);
                    
                    let cap_result = sv.capacity();
                    println!("{}", cap_result);
                    let len_result = sv.len();
                    println!("{}", len_result);
                    let is_empty = sv.is_empty();
                    println!("{}", is_empty);
                    let spilled = sv.spilled();
                    println!("{}", spilled);
                    
                    if !sv.is_empty() {
                        let slice_ref = sv.as_slice();
                        println!("{:?}", slice_ref);
                        let first = &slice_ref[0];
                        println!("{}", *first);
                    }
                },
                1 => {
                    let mut sv = smallvec::SmallVec::<[i64; 12]>::new();
                    let elem_count = _to_u8(GLOBAL_DATA, 120 + i as usize) % 20;
                    for j in 0..elem_count {
                        let val = _to_i64(GLOBAL_DATA, 130 + j as usize * 8);
                        sv.push(val);
                    }
                    
                    sv.retain(_custom_fn2);
                    
                    let reserve_amount = _to_usize(GLOBAL_DATA, 140 + i as usize * 8);
                    sv.reserve(reserve_amount);
                    
                    let insert_idx = _to_usize(GLOBAL_DATA, 150 + i as usize * 8);
                    let insert_val = _to_i64(GLOBAL_DATA, 160 + i as usize * 8);
                    if !sv.is_empty() && insert_idx < sv.len() {
                        sv.insert(insert_idx, insert_val);
                    }
                    
                    if !sv.is_empty() {
                        let remove_idx = _to_usize(GLOBAL_DATA, 170 + i as usize * 8);
                        if remove_idx < sv.len() {
                            let removed = sv.remove(remove_idx);
                            println!("{}", removed);
                        }
                    }
                },
                2 => {
                    let mut sv1 = smallvec::SmallVec::<[i32; 8]>::new();
                    let mut sv2 = smallvec::SmallVec::<[i32; 8]>::new();
                    
                    let count1 = _to_u8(GLOBAL_DATA, 180 + i as usize) % 15;
                    for j in 0..count1 {
                        let val = _to_i32(GLOBAL_DATA, 185 + j as usize * 4);
                        sv1.push(val);
                    }
                    
                    let count2 = _to_u8(GLOBAL_DATA, 186 + i as usize) % 15;
                    for j in 0..count2 {
                        let val = _to_i32(GLOBAL_DATA, 190 + j as usize * 4);
                        sv2.push(val);
                    }
                    
                    sv1.append(&mut sv2);
                    
                    let comparison_result = sv1.cmp(&sv2);
                    println!("{:?}", comparison_result);
                    
                    let partial_cmp_result = sv1.partial_cmp(&sv2);
                    println!("{:?}", partial_cmp_result);
                    
                    let eq_result = sv1.eq(&sv2);
                    println!("{}", eq_result);
                },
                3 => {
                    let mut sv = smallvec::SmallVec::<[CustomType1; 16]>::new();
                    let custom_count = _to_u8(GLOBAL_DATA, 9) % 10;
                    for j in 0..custom_count {
                        let str_len = _to_u8(GLOBAL_DATA, 10 + j as usize) % 17;
                        let t_3 = _to_str(GLOBAL_DATA, 10 + j as usize * 20, 10 + j as usize * 20 + str_len as usize);
                        let t_4 = String::from(t_3);
                        let t_5 = CustomType1(t_4);
                        sv.push(t_5);
                    }
                    
                    sv.retain(_custom_fn0);
                    
                    let sv_len = sv.len();
                    println!("{}", sv_len);
                },
                4 => {
                    let mut sv = smallvec::SmallVec::<[u8; 32]>::new();
                    let fill_count = _to_u8(GLOBAL_DATA, 40 + i as usize) % 50;
                    for j in 0..fill_count {
                        let val = _to_u8(GLOBAL_DATA, 50 + j as usize);
                        sv.push(val);
                    }
                    
                    let truncate_len = _to_usize(GLOBAL_DATA, 60 + i as usize * 8);
                    sv.truncate(truncate_len);
                    
                    let new_len = _to_usize(GLOBAL_DATA, 70 + i as usize * 8);
                    let fill_val = _to_u8(GLOBAL_DATA, 80 + i as usize);
                    sv.resize(new_len, fill_val);
                    
                    sv.shrink_to_fit();
                    
                    if !sv.is_empty() {
                        let clone_sv = sv.clone();
                        println!("{}", clone_sv.len());
                        
                        let drain_start = _to_usize(GLOBAL_DATA, 90 + i as usize * 8);
                        let drain_end = _to_usize(GLOBAL_DATA, 100 + i as usize * 8);
                        if drain_start < sv.len() && drain_end <= sv.len() && drain_start <= drain_end {
                            let drained: smallvec::Drain<[u8; 32]> = sv.drain(drain_start..drain_end);
                            for item in drained {
                                println!("{}", item);
                            }
                        }
                    }
                },
                5 => {
                    let mut sv = smallvec::SmallVec::<[char; 20]>::new();
                    let char_count = _to_u8(GLOBAL_DATA, 110 + i as usize) % 25;
                    for j in 0..char_count {
                        let char_val = _to_char(GLOBAL_DATA, 120 + j as usize * 4);
                        sv.push(char_val);
                    }
                    
                    sv.dedup();
                    
                    let extend_count = _to_u8(GLOBAL_DATA, 130 + i as usize) % 10;
                    let extend_vec: Vec<char> = (0..extend_count).map(|j| _to_char(GLOBAL_DATA, 140 + j as usize * 4)).collect();
                    sv.extend(extend_vec);
                    
                    if !sv.is_empty() {
                        let swap_idx = _to_usize(GLOBAL_DATA, 150 + i as usize * 8);
                        if swap_idx < sv.len() {
                            let swapped = sv.swap_remove(swap_idx);
                            println!("{}", swapped);
                        }
                        
                        let as_ptr = sv.as_ptr();
                        let as_mut_ptr = sv.as_mut_ptr();
                        println!("{:?}", as_ptr);
                        println!("{:?}", as_mut_ptr);
                    }
                },
                6 => {
                    let iter_count = _to_u8(GLOBAL_DATA, 160 + i as usize) % 30;
                    let iter_data: Vec<bool> = (0..iter_count).map(|j| _to_bool(GLOBAL_DATA, 170 + j as usize)).collect();
                    let sv = smallvec::SmallVec::<[bool; 64]>::from_iter(iter_data);
                    
                    let into_iter = sv.into_iter();
                    for (idx, item) in into_iter.enumerate() {
                        println!("{}: {}", idx, item);
                        if idx > 10 { break; }
                    }
                },
                _ => {
                    let mut sv = smallvec::SmallVec::<[String; 16]>::new();
                    let string_count = _to_u8(GLOBAL_DATA, 180 + i as usize) % 15;
                    for j in 0..string_count {
                        let str_len = _to_u8(GLOBAL_DATA, 185 + j as usize) % 10 + 1;
                        let str_data = _to_str(GLOBAL_DATA, 190 + j as usize * 10, 190 + j as usize * 10 + str_len as usize);
                        sv.push(String::from(str_data));
                    }
                    
                    if !sv.is_empty() {
                        let slice_data = sv.as_slice();
                        println!("{:?}", slice_data);
                        
                        let first_ref = &slice_data[0];
                        println!("{}", *first_ref);
                        
                        if sv.len() > 1 {
                            let mut_slice = sv.as_mut_slice();
                            let second_ref = &mut_slice[1];
                            println!("{}", *second_ref);
                        }
                    }
                    
                    sv.clear();
                    
                    let final_vec = sv.into_vec();
                    println!("{}", final_vec.len());
                }
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