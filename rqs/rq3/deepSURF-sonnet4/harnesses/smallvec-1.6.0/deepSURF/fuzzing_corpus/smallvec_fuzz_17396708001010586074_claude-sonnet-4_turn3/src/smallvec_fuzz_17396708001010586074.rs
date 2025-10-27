#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

fn main (){
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 1500 {return;}
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        
        let num_operations = _to_u8(GLOBAL_DATA, 0) % 65;
        
        for i in 0..num_operations {
            let offset = i as usize * 15;
            if offset + 30 >= GLOBAL_DATA.len() { break; }
            
            let operation = _to_u8(GLOBAL_DATA, offset) % 7;
            
            match operation {
                0 => {
                    let constructor_type = _to_u8(GLOBAL_DATA, offset + 1) % 8;
                    let mut small_vec = match constructor_type {
                        0 => SmallVec::<[i32; 32]>::new(),
                        1 => {
                            let capacity = _to_usize(GLOBAL_DATA, offset + 2);
                            SmallVec::<[i32; 32]>::with_capacity(capacity)
                        },
                        2 => {
                            let elem = _to_i32(GLOBAL_DATA, offset + 2);
                            let count = _to_usize(GLOBAL_DATA, offset + 6);
                            SmallVec::<[i32; 32]>::from_elem(elem, count)
                        },
                        3 => {
                            let mut vec = Vec::new();
                            let vec_size = _to_u8(GLOBAL_DATA, offset + 2) % 65;
                            for j in 0..vec_size {
                                let val = _to_i32(GLOBAL_DATA, offset + 3 + j as usize * 4);
                                vec.push(val);
                            }
                            SmallVec::<[i32; 32]>::from_vec(vec)
                        },
                        4 => {
                            let slice_size = _to_u8(GLOBAL_DATA, offset + 2) % 20;
                            let mut slice_vec = Vec::new();
                            for k in 0..slice_size {
                                slice_vec.push(_to_i32(GLOBAL_DATA, offset + 3 + k as usize * 4));
                            }
                            SmallVec::<[i32; 32]>::from_slice(&slice_vec)
                        },
                        5 => {
                            let range_start = offset + 2;
                            let range_end = range_start + 16;
                            if range_end < GLOBAL_DATA.len() {
                                let vec_iter = GLOBAL_DATA[range_start..range_end].iter().map(|&b| b as i32);
                                SmallVec::<[i32; 32]>::from_iter(vec_iter)
                            } else {
                                SmallVec::<[i32; 32]>::new()
                            }
                        },
                        6 => {
                            let array = [_to_i32(GLOBAL_DATA, offset + 2); 32];
                            SmallVec::from(array)
                        },
                        _ => {
                            let mut vec = Vec::new();
                            vec.push(_to_i32(GLOBAL_DATA, offset + 2));
                            SmallVec::<[i32; 32]>::from(vec)
                        }
                    };
                    
                    println!("SmallVec len: {}", *&small_vec.len());
                    
                    let push_val = _to_i32(GLOBAL_DATA, offset + 10);
                    small_vec.push(push_val);
                    
                    if small_vec.len() > 0 {
                        let index = _to_usize(GLOBAL_DATA, offset + 14) % small_vec.len();
                        let elem_ref = &small_vec[index];
                        println!("Element at index {}: {}", index, *elem_ref);
                        
                        if index < small_vec.len() {
                            let element = small_vec.remove(index);
                            println!("Removed element: {}", element);
                        }
                    }
                    
                    println!("Capacity: {}", *&small_vec.capacity());
                    
                    small_vec.truncate(_to_usize(GLOBAL_DATA, offset + 18));
                    small_vec.resize(_to_usize(GLOBAL_DATA, offset + 22), _to_i32(GLOBAL_DATA, offset + 26));
                },
                1 => {
                    let constructor_type = _to_u8(GLOBAL_DATA, offset + 1) % 4;
                    let mut small_vec_str = match constructor_type {
                        0 => SmallVec::<[String; 16]>::new(),
                        1 => {
                            let capacity = _to_usize(GLOBAL_DATA, offset + 2);
                            SmallVec::<[String; 16]>::with_capacity(capacity)
                        },
                        2 => {
                            let elem = String::from("test");
                            let count = _to_u8(GLOBAL_DATA, offset + 2) % 17;
                            SmallVec::<[String; 16]>::from_elem(elem, count as usize)
                        },
                        _ => {
                            let mut vec = Vec::new();
                            vec.push(String::from("initial"));
                            SmallVec::<[String; 16]>::from_vec(vec)
                        }
                    };
                    
                    println!("String SmallVec len: {}", *&small_vec_str.len());
                    
                    let str_len = _to_u8(GLOBAL_DATA, offset + 3) % 10;
                    let end_idx = offset + 4 + str_len as usize;
                    if end_idx < GLOBAL_DATA.len() {
                        let str_slice = _to_str(GLOBAL_DATA, offset + 4, end_idx);
                        small_vec_str.push(String::from(str_slice));
                    }
                    
                    small_vec_str.reserve(_to_usize(GLOBAL_DATA, offset + 14));
                    
                    let slice_ref = small_vec_str.as_slice();
                    for elem in slice_ref {
                        println!("String element: {}", elem);
                    }
                    
                    if small_vec_str.len() > 1 {
                        let popped = small_vec_str.pop();
                        if let Some(s) = popped {
                            println!("Popped string: {}", s);
                        }
                    }
                },
                2 => {
                    let constructor_type = _to_u8(GLOBAL_DATA, offset + 1) % 5;
                    let mut small_vec = match constructor_type {
                        0 => SmallVec::<[u8; 24]>::new(),
                        1 => {
                            let capacity = _to_usize(GLOBAL_DATA, offset + 2);
                            SmallVec::<[u8; 24]>::with_capacity(capacity)
                        },
                        2 => {
                            let elem = _to_u8(GLOBAL_DATA, offset + 2);
                            let count = _to_u8(GLOBAL_DATA, offset + 3) % 30;
                            SmallVec::<[u8; 24]>::from_elem(elem, count as usize)
                        },
                        3 => {
                            let slice_size = _to_u8(GLOBAL_DATA, offset + 2) % 20;
                            let end_idx = offset + 3 + slice_size as usize;
                            if end_idx < GLOBAL_DATA.len() {
                                let slice = &GLOBAL_DATA[offset + 3..end_idx];
                                SmallVec::<[u8; 24]>::from_slice(slice)
                            } else {
                                SmallVec::<[u8; 24]>::new()
                            }
                        },
                        _ => {
                            let data_slice = &GLOBAL_DATA[offset + 2..offset + 6.min(GLOBAL_DATA.len())];
                            SmallVec::<[u8; 24]>::from_iter(data_slice.iter().cloned())
                        }
                    };
                    
                    println!("U8 SmallVec len: {}", *&small_vec.len());
                    
                    let mut other_vec = SmallVec::<[u8; 24]>::new();
                    let append_size = _to_u8(GLOBAL_DATA, offset + 10) % 5;
                    for j in 0..append_size {
                        other_vec.push(_to_u8(GLOBAL_DATA, offset + 11 + j as usize));
                    }
                    
                    if !small_vec.is_empty() && !other_vec.is_empty() {
                        let ordering = small_vec.partial_cmp(&other_vec);
                        if let Some(ord) = ordering {
                            println!("Partial comparison result: {:?}", ord);
                        }
                    }
                    
                    small_vec.append(&mut other_vec);
                    small_vec.clear();
                    small_vec.shrink_to_fit();
                    
                    if small_vec.capacity() > 0 {
                        small_vec.insert(0, _to_u8(GLOBAL_DATA, offset + 20));
                    }
                },
                3 => {
                    let mut small_vec = SmallVec::<[f64; 12]>::new();
                    
                    let vec_size = _to_u8(GLOBAL_DATA, offset + 1) % 15;
                    for j in 0..vec_size {
                        let val = _to_f64(GLOBAL_DATA, offset + 2 + j as usize * 8);
                        small_vec.push(val);
                    }
                    
                    println!("F64 SmallVec len: {}", *&small_vec.len());
                    
                    if small_vec.len() > 3 {
                        let start_range = _to_usize(GLOBAL_DATA, offset + 10) % (small_vec.len() - 2);
                        let end_range = start_range + 2;
                        let mut drain_iter = small_vec.drain(start_range..end_range);
                        
                        while let Some(item) = drain_iter.next() {
                            println!("Drained item: {}", item);
                        }
                    }
                    
                    let clone_vec = small_vec.clone();
                    let into_iter = clone_vec.into_iter();
                    for item in into_iter {
                        println!("Into iter item: {}", item);
                    }
                },
                4 => {
                    let mut small_vec = SmallVec::<[i64; 20]>::new();
                    
                    let elem_count = _to_u8(GLOBAL_DATA, offset + 1) % 25;
                    for j in 0..elem_count {
                        small_vec.push(_to_i64(GLOBAL_DATA, offset + 2 + j as usize * 8));
                    }
                    
                    if !small_vec.is_empty() {
                        let swap_idx = _to_usize(GLOBAL_DATA, offset + 10) % small_vec.len();
                        let swapped = small_vec.swap_remove(swap_idx);
                        println!("Swapped element: {}", swapped);
                    }
                    
                    small_vec.extend_from_slice(&[_to_i64(GLOBAL_DATA, offset + 18), _to_i64(GLOBAL_DATA, offset + 26)]);
                    
                    let as_mut_slice = small_vec.as_mut_slice();
                    for elem in as_mut_slice {
                        *elem = *elem + 1;
                        println!("Modified element: {}", *elem);
                    }
                },
                5 => {
                    let mut small_vec = SmallVec::<[bool; 64]>::new();
                    
                    let bool_count = _to_u8(GLOBAL_DATA, offset + 1) % 65;
                    for j in 0..bool_count {
                        small_vec.push(_to_bool(GLOBAL_DATA, offset + 2 + j as usize));
                    }
                    
                    if small_vec.len() > 0 {
                        small_vec.retain(|&mut x| x);
                        println!("Retained true values, len: {}", small_vec.len());
                    }
                    
                    small_vec.dedup();
                    
                    if small_vec.spilled() {
                        println!("SmallVec has spilled to heap");
                    }
                    
                    let slice_ref = small_vec.as_slice();
                    for (idx, &val) in slice_ref.iter().enumerate() {
                        println!("Bool at {}: {}", idx, val);
                    }
                },
                _ => {
                    let mut small_vec1 = SmallVec::<[char; 32]>::new();
                    let mut small_vec2 = SmallVec::<[char; 32]>::new();
                    
                    let char_count = _to_u8(GLOBAL_DATA, offset + 1) % 10;
                    for j in 0..char_count {
                        let c = _to_char(GLOBAL_DATA, offset + 2 + j as usize * 4);
                        small_vec1.push(c);
                        small_vec2.push(c);
                    }
                    
                    if small_vec1.eq(&small_vec2) {
                        println!("SmallVecs are equal");
                    }
                    
                    let ordering = small_vec1.cmp(&small_vec2);
                    println!("Comparison result: {:?}", ordering);
                    
                    let insert_idx = if small_vec1.len() > 0 { 
                        _to_usize(GLOBAL_DATA, offset + 20) % small_vec1.len() 
                    } else { 
                        0 
                    };
                    let insert_char = _to_char(GLOBAL_DATA, offset + 24);
                    small_vec1.insert(insert_idx, insert_char);
                    
                    if small_vec1.len() > 5 {
                        let range_start = 1;
                        let range_end = 3.min(small_vec1.len());
                        small_vec1.insert_from_slice(range_start, &small_vec2[0..range_end]);
                    }
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