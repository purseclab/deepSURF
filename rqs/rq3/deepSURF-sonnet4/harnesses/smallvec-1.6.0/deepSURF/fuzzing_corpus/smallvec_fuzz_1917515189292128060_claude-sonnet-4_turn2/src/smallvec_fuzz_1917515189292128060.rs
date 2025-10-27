#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};
use std::hash::Hash;

#[derive(Debug)]
struct CustomType0(String);
#[derive(Debug)]
struct CustomType1(String);

fn main (){
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 256 {return;}
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let num_operations = _to_u8(GLOBAL_DATA, 0) % 65;
        
        for i in 0..num_operations {
            let operation = _to_u8(GLOBAL_DATA, (1 + i) as usize);
            let mut offset = (1 + num_operations + i * 16) as usize;
            
            match operation % 6 {
                0 => {
                    let constructor_choice = _to_u8(GLOBAL_DATA, offset) % 8;
                    let mut smallvec = match constructor_choice {
                        0 => {
                            let capacity = _to_usize(GLOBAL_DATA, offset + 1);
                            smallvec::SmallVec::<[u32; 16]>::with_capacity(capacity)
                        },
                        1 => {
                            let elem = _to_u32(GLOBAL_DATA, offset + 1);
                            let count = _to_usize(GLOBAL_DATA, offset + 5);
                            smallvec::SmallVec::<[u32; 16]>::from_elem(elem, count)
                        },
                        2 => {
                            let vec_size = _to_u8(GLOBAL_DATA, offset + 1) % 65;
                            let mut vec = Vec::new();
                            for j in 0..vec_size {
                                vec.push(_to_u32(GLOBAL_DATA, offset + 2 + j as usize * 4));
                            }
                            smallvec::SmallVec::<[u32; 16]>::from_vec(vec)
                        },
                        3 => {
                            let slice_size = _to_u8(GLOBAL_DATA, offset + 1) % 65;
                            let mut slice_data = Vec::new();
                            for j in 0..slice_size {
                                slice_data.push(_to_u32(GLOBAL_DATA, offset + 2 + j as usize * 4));
                            }
                            smallvec::SmallVec::<[u32; 16]>::from_slice(&slice_data)
                        },
                        4 => {
                            let arr: [u32; 16] = [
                                _to_u32(GLOBAL_DATA, offset + 1), _to_u32(GLOBAL_DATA, offset + 5),
                                _to_u32(GLOBAL_DATA, offset + 9), _to_u32(GLOBAL_DATA, offset + 13),
                                _to_u32(GLOBAL_DATA, offset + 17), _to_u32(GLOBAL_DATA, offset + 21),
                                _to_u32(GLOBAL_DATA, offset + 25), _to_u32(GLOBAL_DATA, offset + 29),
                                _to_u32(GLOBAL_DATA, offset + 33), _to_u32(GLOBAL_DATA, offset + 37),
                                _to_u32(GLOBAL_DATA, offset + 41), _to_u32(GLOBAL_DATA, offset + 45),
                                _to_u32(GLOBAL_DATA, offset + 49), _to_u32(GLOBAL_DATA, offset + 53),
                                _to_u32(GLOBAL_DATA, offset + 57), _to_u32(GLOBAL_DATA, offset + 61)
                            ];
                            smallvec::SmallVec::from_buf(arr)
                        },
                        5 => {
                            let iter_size = _to_u8(GLOBAL_DATA, offset + 1) % 65;
                            let iter = (0..iter_size).map(|j| _to_u32(GLOBAL_DATA, offset + 2 + j as usize * 4));
                            smallvec::SmallVec::<[u32; 16]>::from_iter(iter)
                        },
                        6 => {
                            let slice_size = _to_u8(GLOBAL_DATA, offset + 1) % 65;
                            let mut slice_data = Vec::new();
                            for j in 0..slice_size {
                                slice_data.push(_to_u32(GLOBAL_DATA, offset + 2 + j as usize * 4));
                            }
                            slice_data.to_smallvec()
                        },
                        _ => smallvec::SmallVec::<[u32; 16]>::new()
                    };

                    let push_count = _to_u8(GLOBAL_DATA, offset + 9) % 65;
                    for j in 0..push_count {
                        let value = _to_u32(GLOBAL_DATA, offset + 10 + j as usize);
                        smallvec.push(value);
                    }

                    let result = smallvec.pop();
                    if let Some(popped_value) = result {
                        println!("{}", popped_value);
                    }

                    let slice_ref = smallvec.as_slice();
                    for elem in slice_ref.iter() {
                        println!("{}", *elem);
                    }

                    if !smallvec.is_empty() {
                        let len = smallvec.len();
                        let index = _to_usize(GLOBAL_DATA, offset + 12);
                        if len > 0 {
                            let actual_index = index % len;
                            let elem_ref = &smallvec[actual_index];
                            println!("{}", *elem_ref);
                        }
                    }
                },
                1 => {
                    let mut smallvec1 = smallvec::SmallVec::<[u8; 32]>::new();
                    let mut smallvec2 = smallvec::SmallVec::<[u8; 32]>::new();
                    
                    let count1 = _to_u8(GLOBAL_DATA, offset) % 65;
                    for j in 0..count1 {
                        smallvec1.push(_to_u8(GLOBAL_DATA, offset + 1 + j as usize));
                    }
                    
                    let count2 = _to_u8(GLOBAL_DATA, offset + 8) % 65;
                    for j in 0..count2 {
                        smallvec2.push(_to_u8(GLOBAL_DATA, offset + 9 + j as usize));
                    }

                    let capacity1 = smallvec1.capacity();
                    let capacity2 = smallvec2.capacity();
                    println!("{}", capacity1);
                    println!("{}", capacity2);

                    if smallvec1.eq(&smallvec2) {
                        println!("equal");
                    }

                    let comparison = smallvec1.cmp(&smallvec2);
                    println!("{:?}", comparison);

                    let partial_comparison = smallvec1.partial_cmp(&smallvec2);
                    if let Some(ord) = partial_comparison {
                        println!("{:?}", ord);
                    }

                    smallvec1.append(&mut smallvec2);

                    let result = smallvec1.pop();
                    if let Some(popped) = result {
                        println!("{}", popped);
                    }
                },
                2 => {
                    let mut smallvec = smallvec::SmallVec::<[i16; 64]>::new();
                    
                    let push_count = _to_u8(GLOBAL_DATA, offset) % 65;
                    for j in 0..push_count {
                        let value = _to_i16(GLOBAL_DATA, offset + 1 + j as usize * 2);
                        smallvec.push(value);
                    }

                    let reserve_additional = _to_usize(GLOBAL_DATA, offset + 8);
                    smallvec.reserve(reserve_additional);

                    let reserve_exact_additional = _to_usize(GLOBAL_DATA, offset + 16);
                    smallvec.reserve_exact(reserve_exact_additional);

                    let grow_capacity = _to_usize(GLOBAL_DATA, offset + 24);
                    smallvec.grow(grow_capacity);

                    let result = smallvec.pop();
                    if let Some(popped) = result {
                        println!("{}", popped);
                    }

                    let as_mut_slice = smallvec.as_mut_slice();
                    for elem in as_mut_slice.iter_mut() {
                        *elem = _to_i16(GLOBAL_DATA, offset + 10);
                    }

                    let slice_ref = smallvec.as_slice();
                    for elem in slice_ref.iter() {
                        println!("{}", *elem);
                    }
                },
                3 => {
                    let mut smallvec = smallvec::SmallVec::<[f32; 16]>::new();

                    let push_count = _to_u8(GLOBAL_DATA, offset) % 65;
                    for j in 0..push_count {
                        let value = _to_f32(GLOBAL_DATA, offset + 1 + j as usize * 4);
                        smallvec.push(value);
                    }

                    if !smallvec.is_empty() {
                        let remove_index = _to_usize(GLOBAL_DATA, offset + 8);
                        if smallvec.len() > 0 {
                            let actual_index = remove_index % smallvec.len();
                            let removed = smallvec.remove(actual_index);
                            println!("{}", removed);
                        }
                    }

                    if !smallvec.is_empty() {
                        let swap_remove_index = _to_usize(GLOBAL_DATA, offset + 16);
                        if smallvec.len() > 0 {
                            let actual_index = swap_remove_index % smallvec.len();
                            let swap_removed = smallvec.swap_remove(actual_index);
                            println!("{}", swap_removed);
                        }
                    }

                    let insert_index = _to_usize(GLOBAL_DATA, offset + 24);
                    let insert_value = _to_f32(GLOBAL_DATA, offset + 32);
                    if smallvec.len() > 0 {
                        let actual_index = insert_index % (smallvec.len() + 1);
                        smallvec.insert(actual_index, insert_value);
                    } else {
                        smallvec.insert(0, insert_value);
                    }

                    let result = smallvec.pop();
                    if let Some(popped) = result {
                        println!("{}", popped);
                    }

                    let cloned = smallvec.clone();
                    let drain_start = _to_usize(GLOBAL_DATA, offset + 36);
                    let drain_end = _to_usize(GLOBAL_DATA, offset + 44);
                    if cloned.len() > 0 {
                        let start = drain_start % cloned.len();
                        let end = if drain_end % cloned.len() >= start {
                            drain_end % cloned.len()
                        } else {
                            cloned.len()
                        };
                        let mut drain_iter = smallvec.drain(start..end);
                        while let Some(drained) = drain_iter.next() {
                            println!("{}", drained);
                        }
                    }
                },
                4 => {
                    let mut smallvec = smallvec::SmallVec::<[bool; 128]>::new();

                    let push_count = _to_u8(GLOBAL_DATA, offset) % 65;
                    for j in 0..push_count {
                        let value = _to_bool(GLOBAL_DATA, offset + 1 + j as usize);
                        smallvec.push(value);
                    }

                    let truncate_len = _to_usize(GLOBAL_DATA, offset + 8);
                    smallvec.truncate(truncate_len);

                    let resize_len = _to_usize(GLOBAL_DATA, offset + 16);
                    let resize_value = _to_bool(GLOBAL_DATA, offset + 24);
                    smallvec.resize(resize_len, resize_value);

                    let result = smallvec.pop();
                    if let Some(popped) = result {
                        println!("{}", popped);
                    }

                    smallvec.retain(|x| *x == true);

                    smallvec.clear();

                    let final_result = smallvec.pop();
                    if let Some(final_popped) = final_result {
                        println!("{}", final_popped);
                    }

                    let mut hasher = std::collections::hash_map::DefaultHasher::new();
                    smallvec.hash(&mut hasher);
                    
                    smallvec.shrink_to_fit();
                },
                _ => {
                    let mut smallvec = smallvec::SmallVec::<[char; 16]>::new();

                    let push_count = _to_u8(GLOBAL_DATA, offset) % 65;
                    for j in 0..push_count {
                        let value = _to_char(GLOBAL_DATA, offset + 1 + j as usize * 4);
                        smallvec.push(value);
                    }

                    let extend_count = _to_u8(GLOBAL_DATA, offset + 8) % 65;
                    let extend_iter = (0..extend_count).map(|j| _to_char(GLOBAL_DATA, offset + 9 + j as usize * 4));
                    smallvec.extend(extend_iter);

                    let slice_data: Vec<char> = (0..5).map(|j| _to_char(GLOBAL_DATA, offset + 12 + j * 4)).collect();
                    smallvec.extend_from_slice(&slice_data);

                    let insert_many_index = _to_usize(GLOBAL_DATA, offset + 16);
                    let insert_many_count = _to_u8(GLOBAL_DATA, offset + 24) % 65;
                    let insert_many_iter = (0..insert_many_count).map(|j| _to_char(GLOBAL_DATA, offset + 25 + j as usize * 4));
                    if smallvec.len() > 0 {
                        let actual_index = insert_many_index % (smallvec.len() + 1);
                        smallvec.insert_many(actual_index, insert_many_iter);
                    } else {
                        smallvec.insert_many(0, insert_many_iter);
                    }

                    let result = smallvec.pop();
                    if let Some(popped) = result {
                        println!("{}", popped);
                    }

                    let into_vec = smallvec.into_vec();
                    for elem in into_vec.iter() {
                        println!("{}", *elem);
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