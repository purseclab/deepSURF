#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};
use std::borrow::Borrow;

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 200 {return;}
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        
        let operations_count = _to_u8(GLOBAL_DATA, 0) % 15 + 1;
        
        for op_idx in 0..operations_count {
            let base_offset = 1 + op_idx as usize * 12;
            if base_offset + 12 > GLOBAL_DATA.len() { break; }
            
            let operation = _to_u8(GLOBAL_DATA, base_offset) % 15;
            
            match operation {
                0 => {
                    let constructor_choice = _to_u8(GLOBAL_DATA, base_offset + 1) % 8;
                    let smallvec = match constructor_choice {
                        0 => SmallVec::<[u32; 16]>::new(),
                        1 => {
                            let capacity = _to_usize(GLOBAL_DATA, base_offset + 2);
                            SmallVec::<[u32; 16]>::with_capacity(capacity)
                        },
                        2 => {
                            let vec_size = _to_u8(GLOBAL_DATA, base_offset + 2) % 8;
                            let mut vec = Vec::new();
                            for i in 0..vec_size {
                                vec.push(_to_u32(GLOBAL_DATA, base_offset + 3 + i as usize * 4));
                            }
                            SmallVec::<[u32; 16]>::from_vec(vec)
                        },
                        3 => {
                            let array: [u32; 16] = [
                                _to_u32(GLOBAL_DATA, base_offset + 2),
                                _to_u32(GLOBAL_DATA, base_offset + 6),
                                _to_u32(GLOBAL_DATA, base_offset + 10),
                                _to_u32(GLOBAL_DATA, base_offset + 2),
                                _to_u32(GLOBAL_DATA, base_offset + 6),
                                _to_u32(GLOBAL_DATA, base_offset + 10),
                                _to_u32(GLOBAL_DATA, base_offset + 2),
                                _to_u32(GLOBAL_DATA, base_offset + 6),
                                _to_u32(GLOBAL_DATA, base_offset + 10),
                                _to_u32(GLOBAL_DATA, base_offset + 2),
                                _to_u32(GLOBAL_DATA, base_offset + 6),
                                _to_u32(GLOBAL_DATA, base_offset + 10),
                                _to_u32(GLOBAL_DATA, base_offset + 2),
                                _to_u32(GLOBAL_DATA, base_offset + 6),
                                _to_u32(GLOBAL_DATA, base_offset + 10),
                                _to_u32(GLOBAL_DATA, base_offset + 2)
                            ];
                            let len = _to_usize(GLOBAL_DATA, base_offset + 8);
                            SmallVec::from_buf_and_len(array, len)
                        },
                        4 => {
                            let elem = _to_u32(GLOBAL_DATA, base_offset + 2);
                            let count = _to_usize(GLOBAL_DATA, base_offset + 6);
                            SmallVec::<[u32; 16]>::from_elem(elem, count)
                        },
                        5 => {
                            let slice_len = _to_u8(GLOBAL_DATA, base_offset + 2) % 8;
                            let mut slice_data = Vec::new();
                            for i in 0..slice_len {
                                slice_data.push(_to_u32(GLOBAL_DATA, base_offset + 3 + i as usize * 4));
                            }
                            SmallVec::<[u32; 16]>::from_slice(&slice_data)
                        },
                        6 => {
                            let iter_size = _to_u8(GLOBAL_DATA, base_offset + 2) % 8;
                            let mut iter_data = Vec::new();
                            for i in 0..iter_size {
                                iter_data.push(_to_u32(GLOBAL_DATA, base_offset + 3 + i as usize * 4));
                            }
                            SmallVec::<[u32; 16]>::from_iter(iter_data.into_iter())
                        },
                        _ => {
                            let array: [u32; 16] = [0; 16];
                            SmallVec::from_buf(array)
                        }
                    };
                    
                    let ptr = smallvec.as_ptr();
                    println!("{:?}", ptr);
                    
                    let slice_ref = smallvec.as_slice();
                    println!("{:?}", slice_ref);
                    
                    let len = smallvec.len();
                    let capacity = smallvec.capacity();
                    let is_empty = smallvec.is_empty();
                    let spilled = smallvec.spilled();
                    
                    println!("len: {}, capacity: {}, empty: {}, spilled: {}", len, capacity, is_empty, spilled);
                    
                    if !slice_ref.is_empty() {
                        let first_elem = &slice_ref[0];
                        println!("first: {}", *first_elem);
                    }
                },
                1 => {
                    let mut smallvec = SmallVec::<[u32; 16]>::new();
                    let push_count = _to_u8(GLOBAL_DATA, base_offset + 1) % 10;
                    
                    for i in 0..push_count {
                        let value = _to_u32(GLOBAL_DATA, base_offset + 2 + i as usize * 4);
                        smallvec.push(value);
                    }
                    
                    let ptr = smallvec.as_ptr();
                    println!("{:?}", ptr);
                    
                    let deref_slice = &*smallvec;
                    println!("{:?}", deref_slice);
                    
                    if !smallvec.is_empty() {
                        let pop_result = smallvec.pop();
                        if let Some(val) = pop_result {
                            println!("popped: {}", val);
                        }
                    }
                },
                2 => {
                    let mut smallvec1 = SmallVec::<[u32; 16]>::new();
                    let mut smallvec2 = SmallVec::<[u32; 16]>::new();
                    
                    smallvec1.push(_to_u32(GLOBAL_DATA, base_offset + 1));
                    smallvec2.push(_to_u32(GLOBAL_DATA, base_offset + 5));
                    
                    let ptr1 = smallvec1.as_ptr();
                    let ptr2 = smallvec2.as_ptr();
                    
                    let cmp_result = smallvec1.cmp(&smallvec2);
                    let partial_cmp_result = smallvec1.partial_cmp(&smallvec2);
                    let eq_result = smallvec1.eq(&smallvec2);
                    
                    println!("ptr1: {:?}, ptr2: {:?}", ptr1, ptr2);
                    println!("cmp: {:?}, partial_cmp: {:?}, eq: {}", cmp_result, partial_cmp_result, eq_result);
                },
                3 => {
                    let mut smallvec = SmallVec::<[u32; 16]>::new();
                    let reserve_amount = _to_usize(GLOBAL_DATA, base_offset + 1);
                    
                    smallvec.reserve(reserve_amount);
                    
                    let ptr = smallvec.as_ptr();
                    println!("{:?}", ptr);
                    
                    let capacity = smallvec.capacity();
                    println!("reserved capacity: {}", capacity);
                    
                    smallvec.shrink_to_fit();
                    let new_capacity = smallvec.capacity();
                    println!("after shrink capacity: {}", new_capacity);
                },
                4 => {
                    let mut smallvec = SmallVec::<[u32; 16]>::new();
                    smallvec.push(_to_u32(GLOBAL_DATA, base_offset + 1));
                    smallvec.push(_to_u32(GLOBAL_DATA, base_offset + 5));
                    
                    let ptr_before = smallvec.as_ptr();
                    
                    let index = _to_usize(GLOBAL_DATA, base_offset + 9);
                    let value = _to_u32(GLOBAL_DATA, base_offset + 2);
                    
                    if index <= smallvec.len() {
                        smallvec.insert(index, value);
                    }
                    
                    let ptr_after = smallvec.as_ptr();
                    println!("ptr before: {:?}, after: {:?}", ptr_before, ptr_after);
                    
                    let slice = smallvec.as_slice();
                    for (i, elem) in slice.iter().enumerate() {
                        println!("elem[{}]: {}", i, *elem);
                    }
                },
                5 => {
                    let mut smallvec = SmallVec::<[u32; 16]>::new();
                    let slice_data = [_to_u32(GLOBAL_DATA, base_offset + 1), _to_u32(GLOBAL_DATA, base_offset + 5)];
                    
                    smallvec.extend_from_slice(&slice_data);
                    
                    let ptr = smallvec.as_ptr();
                    println!("{:?}", ptr);
                    
                    let as_slice = smallvec.as_slice();
                    println!("slice len: {}", as_slice.len());
                    
                    if !smallvec.is_empty() {
                        let as_mut_slice = smallvec.as_mut_slice();
                        as_mut_slice[0] = _to_u32(GLOBAL_DATA, base_offset + 9);
                        println!("modified first elem: {}", as_mut_slice[0]);
                    }
                },
                6 => {
                    let mut smallvec = SmallVec::<[u32; 16]>::new();
                    for i in 0..5 {
                        smallvec.push(_to_u32(GLOBAL_DATA, base_offset + 1 + i * 2));
                    }
                    
                    let ptr_before = smallvec.as_ptr();
                    
                    let range_start = _to_usize(GLOBAL_DATA, base_offset + 1) % 3;
                    let range_end = range_start + (_to_usize(GLOBAL_DATA, base_offset + 5) % 3) + 1;
                    let range_end = if range_end > smallvec.len() { smallvec.len() } else { range_end };
                    
                    let mut drain = smallvec.drain(range_start..range_end);
                    while let Some(item) = drain.next() {
                        println!("drained: {}", item);
                    }
                    drop(drain);
                    
                    let ptr_after = smallvec.as_ptr();
                    println!("ptr before drain: {:?}, after: {:?}", ptr_before, ptr_after);
                },
                7 => {
                    let mut smallvec = SmallVec::<[u32; 16]>::new();
                    smallvec.push(_to_u32(GLOBAL_DATA, base_offset + 1));
                    smallvec.push(_to_u32(GLOBAL_DATA, base_offset + 5));
                    
                    let ptr = smallvec.as_ptr();
                    
                    let index = _to_usize(GLOBAL_DATA, base_offset + 9);
                    if index < smallvec.len() {
                        let removed = smallvec.remove(index);
                        println!("removed: {}", removed);
                    }
                    
                    println!("ptr: {:?}", ptr);
                    
                    let deref_result = smallvec.deref();
                    println!("deref len: {}", deref_result.len());
                },
                8 => {
                    let mut smallvec = SmallVec::<[u32; 16]>::new();
                    for i in 0..3 {
                        smallvec.push(_to_u32(GLOBAL_DATA, base_offset + 1 + i * 4));
                    }
                    
                    let ptr = smallvec.as_ptr();
                    
                    let into_vec = smallvec.clone().into_vec();
                    println!("into_vec len: {}", into_vec.len());
                    
                    let into_iter = smallvec.into_iter();
                    for (i, item) in into_iter.enumerate() {
                        if i < 3 {
                            println!("iter item[{}]: {}", i, item);
                        }
                    }
                    
                    println!("ptr: {:?}", ptr);
                },
                9 => {
                    let mut smallvec = SmallVec::<[u32; 16]>::new();
                    let truncate_len = _to_usize(GLOBAL_DATA, base_offset + 1);
                    
                    for i in 0..8 {
                        smallvec.push(_to_u32(GLOBAL_DATA, base_offset + 2 + i * 4));
                    }
                    
                    let ptr_before = smallvec.as_ptr();
                    smallvec.truncate(truncate_len);
                    let ptr_after = smallvec.as_ptr();
                    
                    println!("truncate ptr before: {:?}, after: {:?}", ptr_before, ptr_after);
                    println!("new len: {}", smallvec.len());
                },
                10 => {
                    let mut smallvec = SmallVec::<[u32; 16]>::new();
                    for i in 0..4 {
                        smallvec.push(_to_u32(GLOBAL_DATA, base_offset + 1 + i * 4));
                    }
                    
                    let clear_choice = _to_u8(GLOBAL_DATA, base_offset + 1) % 2;
                    if clear_choice == 0 {
                        smallvec.clear();
                        println!("cleared, new len: {}", smallvec.len());
                    }
                    
                    let swap_remove_index = _to_usize(GLOBAL_DATA, base_offset + 5);
                    if swap_remove_index < smallvec.len() {
                        let removed = smallvec.swap_remove(swap_remove_index);
                        println!("swap removed: {}", removed);
                    }
                    
                    let resize_choice = _to_u8(GLOBAL_DATA, base_offset + 9) % 2;
                    if resize_choice == 0 {
                        let new_size = _to_usize(GLOBAL_DATA, base_offset + 10) % 10;
                        let fill_value = _to_u32(GLOBAL_DATA, base_offset + 11);
                        smallvec.resize(new_size, fill_value);
                        println!("resized to: {}", new_size);
                    }
                },
                11 => {
                    let mut smallvec1 = SmallVec::<[u32; 16]>::new();
                    let mut smallvec2 = SmallVec::<[u32; 16]>::new();
                    
                    for i in 0..3 {
                        smallvec1.push(_to_u32(GLOBAL_DATA, base_offset + 1 + i * 4));
                        smallvec2.push(_to_u32(GLOBAL_DATA, base_offset + 2 + i * 4));
                    }
                    
                    let append_choice = _to_u8(GLOBAL_DATA, base_offset + 1) % 2;
                    if append_choice == 0 {
                        smallvec1.append(&mut smallvec2);
                        println!("appended, new len: {}", smallvec1.len());
                    }
                    
                    let insert_many_index = _to_usize(GLOBAL_DATA, base_offset + 5);
                    if insert_many_index <= smallvec1.len() {
                        let insert_data = vec![_to_u32(GLOBAL_DATA, base_offset + 6), _to_u32(GLOBAL_DATA, base_offset + 7)];
                        smallvec1.insert_many(insert_many_index, insert_data);
                        println!("inserted many at index: {}", insert_many_index);
                    }
                },
                12 => {
                    let mut smallvec = SmallVec::<[u32; 16]>::new();
                    let extend_data = vec![_to_u32(GLOBAL_DATA, base_offset + 1), _to_u32(GLOBAL_DATA, base_offset + 5)];
                    smallvec.extend(extend_data);
                    
                    let try_reserve_amount = _to_usize(GLOBAL_DATA, base_offset + 9);
                    let try_reserve_result = smallvec.try_reserve(try_reserve_amount);
                    println!("try_reserve result: {:?}", try_reserve_result.is_ok());
                    
                    let into_boxed_slice = smallvec.clone().into_boxed_slice();
                    println!("boxed slice len: {}", into_boxed_slice.len());
                    
                    let into_inner_result = smallvec.into_inner();
                    println!("into_inner succeeded: {}", into_inner_result.is_ok());
                },
                13 => {
                    let mut smallvec = SmallVec::<[u32; 16]>::new();
                    for i in 0..6 {
                        smallvec.push(_to_u32(GLOBAL_DATA, base_offset + 1 + i * 2));
                    }
                    
                    let retain_choice = _to_u8(GLOBAL_DATA, base_offset + 1) % 2;
                    if retain_choice == 0 {
                        smallvec.retain(|&mut x| x % 2 == 0);
                        println!("retained evens, new len: {}", smallvec.len());
                    }
                    
                    let dedup_choice = _to_u8(GLOBAL_DATA, base_offset + 2) % 2;
                    if dedup_choice == 0 {
                        smallvec.dedup();
                        println!("deduped, new len: {}", smallvec.len());
                    }
                    
                    let as_mut_ptr = smallvec.as_mut_ptr();
                    println!("mut ptr: {:?}", as_mut_ptr);
                },
                _ => {
                    let smallvec = SmallVec::<[u32; 16]>::new();
                    let ptr = smallvec.as_ptr();
                    println!("default ptr: {:?}", ptr);
                    
                    let cloned = smallvec.clone();
                    let cloned_ptr = cloned.as_ptr();
                    println!("cloned ptr: {:?}", cloned_ptr);
                    
                    let borrow_result: &[u32] = smallvec.borrow();
                    println!("borrow len: {}", borrow_result.len());
                    
                    let as_ref_result = smallvec.as_ref();
                    println!("as_ref len: {}", as_ref_result.len());
                    
                    let slice_to_smallvec: SmallVec<[u32; 16]> = [1u32, 2u32, 3u32].to_smallvec();
                    println!("to_smallvec len: {}", slice_to_smallvec.len());
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