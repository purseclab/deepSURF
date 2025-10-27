#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

struct CustomType1(String);

#[derive(Debug)]
struct CustomArray([i32; 16]);

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 512 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        
        let num_operations = _to_u8(GLOBAL_DATA, 0) % 65;
        
        for i in 0..num_operations {
            let base_offset = 1 + (i as usize * 8);
            if base_offset + 7 >= GLOBAL_DATA.len() { break; }
            
            let operation = _to_u8(GLOBAL_DATA, base_offset) % 8;
            
            match operation {
                0 => {
                    let constructor_choice = _to_u8(GLOBAL_DATA, base_offset + 1) % 8;
                    let mut smallvec = match constructor_choice {
                        0 => smallvec::SmallVec::<[i32; 12]>::new(),
                        1 => {
                            let cap = _to_usize(GLOBAL_DATA, base_offset + 2);
                            smallvec::SmallVec::<[i32; 12]>::with_capacity(cap)
                        },
                        2 => {
                            let size = _to_u8(GLOBAL_DATA, base_offset + 2) % 65;
                            let vec: Vec<i32> = (0..size).map(|_| _to_i32(GLOBAL_DATA, base_offset + 3)).collect();
                            smallvec::SmallVec::<[i32; 12]>::from_vec(vec)
                        },
                        3 => {
                            let arr = [_to_i32(GLOBAL_DATA, base_offset + 2); 12];
                            smallvec::SmallVec::from_buf(arr)
                        },
                        4 => {
                            let elem = _to_i32(GLOBAL_DATA, base_offset + 2);
                            let count = _to_usize(GLOBAL_DATA, base_offset + 3);
                            smallvec::SmallVec::<[i32; 12]>::from_elem(elem, count)
                        },
                        5 => {
                            let slice_size = _to_u8(GLOBAL_DATA, base_offset + 2) % 20;
                            let slice: Vec<i32> = (0..slice_size).map(|_| _to_i32(GLOBAL_DATA, base_offset + 3)).collect();
                            smallvec::SmallVec::<[i32; 12]>::from_slice(&slice)
                        },
                        6 => {
                            let iter = (0..5).map(|_| _to_i32(GLOBAL_DATA, base_offset + 2));
                            smallvec::SmallVec::<[i32; 12]>::from_iter(iter)
                        },
                        _ => {
                            let arr = [_to_i32(GLOBAL_DATA, base_offset + 2); 12];
                            let buf_len = _to_usize(GLOBAL_DATA, base_offset + 3) % 13;
                            smallvec::SmallVec::<[i32; 12]>::from_buf_and_len(arr, buf_len)
                        }
                    };
                    
                    let cap_before = smallvec.capacity();
                    println!("{:?}", cap_before);
                    
                    smallvec.push(_to_i32(GLOBAL_DATA, base_offset + 4));
                    let slice_ref = smallvec.as_slice();
                    println!("{:?}", slice_ref);
                    
                    let mut_slice_ref = smallvec.as_mut_slice();
                    println!("{:?}", mut_slice_ref);
                    
                    if !smallvec.is_empty() {
                        let elem_ref = &smallvec[0];
                        println!("{:?}", *elem_ref);
                    }
                    
                    let len = smallvec.len();
                    println!("{:?}", len);
                    
                    smallvec.reserve(_to_usize(GLOBAL_DATA, base_offset + 5));
                    
                    if let Some(popped) = smallvec.pop() {
                        println!("{:?}", popped);
                    }
                    
                    smallvec.shrink_to_fit();
                    smallvec.clear();
                },
                1 => {
                    let mut sv = smallvec::SmallVec::<[String; 16]>::new();
                    
                    let string_count = _to_u8(GLOBAL_DATA, base_offset + 1) % 10;
                    for j in 0..string_count {
                        sv.push(format!("test_{}", j));
                    }
                    
                    let cap_result = sv.capacity();
                    println!("{:?}", cap_result);
                    
                    sv.reserve(_to_usize(GLOBAL_DATA, base_offset + 2));
                    sv.shrink_to_fit();
                    
                    if !sv.is_empty() {
                        let removed = sv.remove(0);
                        println!("{:?}", removed);
                    }
                    
                    sv.truncate(_to_usize(GLOBAL_DATA, base_offset + 3) % (sv.len() + 1));
                    sv.clear();
                    
                    let len_result = sv.len();
                    println!("{:?}", len_result);
                    
                    let spilled = sv.spilled();
                    println!("{:?}", spilled);
                },
                2 => {
                    let mut sv1 = smallvec::SmallVec::<[String; 8]>::new();
                    let mut sv2 = smallvec::SmallVec::<[String; 8]>::new();
                    
                    let s = String::from("test");
                    sv1.push(s.clone());
                    sv2.push(s);
                    
                    let cap1 = sv1.capacity();
                    let cap2 = sv2.capacity();
                    println!("{:?} {:?}", cap1, cap2);
                    
                    let cmp_result = sv1.cmp(&sv2);
                    println!("{:?}", cmp_result);
                    
                    let partial_cmp_result = sv1.partial_cmp(&sv2);
                    if let Some(ord) = partial_cmp_result {
                        println!("{:?}", ord);
                    }
                    
                    let eq_result = sv1.eq(&sv2);
                    println!("{:?}", eq_result);
                    
                    sv1.append(&mut sv2);
                    println!("{:?}", sv1.len());
                },
                3 => {
                    let mut sv = smallvec::SmallVec::<[u8; 32]>::new();
                    
                    let data_size = _to_u8(GLOBAL_DATA, base_offset + 1) % 50;
                    let end_pos = (base_offset + 2 + data_size as usize).min(GLOBAL_DATA.len());
                    let data_slice = &GLOBAL_DATA[base_offset + 2..end_pos];
                    sv.extend_from_slice(data_slice);
                    
                    let drain_start = _to_usize(GLOBAL_DATA, base_offset + 3) % (sv.len() + 1);
                    let drain_end = _to_usize(GLOBAL_DATA, base_offset + 4) % (sv.len() + 1);
                    let (start, end) = if drain_start <= drain_end { (drain_start, drain_end) } else { (drain_end, drain_start) };
                    
                    {
                        let mut drain_iter = sv.drain(start..end);
                        
                        if let Some(first) = drain_iter.next() {
                            println!("{:?}", first);
                        }
                    }
                    
                    let final_capacity = sv.capacity();
                    println!("{:?}", final_capacity);
                },
                4 => {
                    let mut sv = smallvec::SmallVec::<[f64; 20]>::new();
                    
                    let elem_count = _to_u8(GLOBAL_DATA, base_offset + 1) % 30;
                    for j in 0..elem_count {
                        let val = _to_f64(GLOBAL_DATA, base_offset + 2);
                        sv.push(val);
                    }
                    
                    let insert_index = _to_usize(GLOBAL_DATA, base_offset + 3) % (sv.len() + 1);
                    let insert_val = _to_f64(GLOBAL_DATA, base_offset + 4);
                    sv.insert(insert_index, insert_val);
                    
                    if sv.len() > 1 {
                        let swap_index = _to_usize(GLOBAL_DATA, base_offset + 5) % sv.len();
                        let swapped = sv.swap_remove(swap_index);
                        println!("{:?}", swapped);
                    }
                    
                    let cloned_sv = sv.clone();
                    println!("{:?}", cloned_sv.len());
                    
                    let as_ref_slice = sv.as_ref();
                    println!("{:?}", as_ref_slice.len());
                    
                    let deref_slice = sv.deref();
                    println!("{:?}", deref_slice.len());
                },
                5 => {
                    let mut sv = smallvec::SmallVec::<[char; 15]>::new();
                    
                    let char_count = _to_u8(GLOBAL_DATA, base_offset + 1) % 25;
                    for j in 0..char_count {
                        let ch = _to_char(GLOBAL_DATA, base_offset + 2 + (j as usize * 4));
                        sv.push(ch);
                    }
                    
                    let resize_len = _to_usize(GLOBAL_DATA, base_offset + 3) % 50;
                    let fill_char = _to_char(GLOBAL_DATA, base_offset + 4);
                    sv.resize(resize_len, fill_char);
                    
                    sv.dedup();
                    
                    let into_vec = sv.into_vec();
                    println!("{:?}", into_vec.len());
                },
                6 => {
                    let vec_data: Vec<bool> = (0..20).map(|j| _to_bool(GLOBAL_DATA, base_offset + 1 + j)).collect();
                    let mut sv = smallvec::SmallVec::<[bool; 25]>::from(vec_data);
                    
                    let retain_threshold = _to_u8(GLOBAL_DATA, base_offset + 2) % 2 == 0;
                    sv.retain(|&mut x| x == retain_threshold);
                    
                    if !sv.is_empty() {
                        let index_val = sv.index(_to_usize(GLOBAL_DATA, base_offset + 3) % sv.len());
                        println!("{:?}", *index_val);
                    }
                    
                    let try_reserve_result = sv.try_reserve(_to_usize(GLOBAL_DATA, base_offset + 4));
                    match try_reserve_result {
                        Ok(_) => println!("Reserve successful"),
                        Err(_) => println!("Reserve failed"),
                    }
                    
                    let into_iter = sv.into_iter();
                    let collected: Vec<bool> = into_iter.collect();
                    println!("{:?}", collected.len());
                },
                _ => {
                    let slice_data: Vec<u32> = (0..15).map(|j| _to_u32(GLOBAL_DATA, base_offset + 1 + (j * 4))).collect();
                    let mut sv = smallvec::SmallVec::<[u32; 18]>::from_slice(&slice_data);
                    
                    let insert_index = _to_usize(GLOBAL_DATA, base_offset + 2) % (sv.len() + 1);
                    let insert_slice: Vec<u32> = (0..5).map(|j| _to_u32(GLOBAL_DATA, base_offset + 3 + (j * 4))).collect();
                    sv.insert_from_slice(insert_index, &insert_slice);
                    
                    let resize_with_len = _to_usize(GLOBAL_DATA, base_offset + 4) % 40;
                    let counter = std::cell::Cell::new(0u32);
                    sv.resize_with(resize_with_len, || {
                        let val = counter.get();
                        counter.set(val + 1);
                        val
                    });
                    
                    if sv.len() > 2 {
                        let dedup_key_fn = |x: &mut u32| *x % 3;
                        sv.dedup_by_key(dedup_key_fn);
                    }
                    
                    let as_ptr = sv.as_ptr();
                    println!("{:?}", as_ptr);
                    
                    let into_boxed = sv.into_boxed_slice();
                    println!("{:?}", into_boxed.len());
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