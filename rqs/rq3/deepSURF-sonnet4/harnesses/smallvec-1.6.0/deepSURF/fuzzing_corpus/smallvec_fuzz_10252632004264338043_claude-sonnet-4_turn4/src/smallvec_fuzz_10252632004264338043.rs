#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};
use std::borrow::{Borrow, BorrowMut};

#[derive(Debug)]
struct CustomType0(String);

#[derive(Debug)]
struct CustomType1(usize);

impl core::clone::Clone for CustomType1 {
    fn clone(&self) -> Self {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 10);
        let custom_impl_inst_num = self.0;
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0{
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector{
                1 => global_data.first_half,
                _ => global_data.second_half,
        };
        let t_4 = _to_usize(GLOBAL_DATA, 18);
        let t_5 = CustomType1(t_4);
        return t_5;
    }
}

impl core::marker::Copy for CustomType1 {
}

fn main (){
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 2000 {return;}
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        
        let operations_count = _to_u8(GLOBAL_DATA, 0) % 20 + 5;
        
        for op_index in 0..operations_count {
            let base_offset = op_index as usize * 80;
            let op_type = _to_u8(GLOBAL_DATA, base_offset + 1) % 10;
            
            match op_type {
                0 => {
                    let constructor_choice = _to_u8(GLOBAL_DATA, base_offset + 2) % 5;
                    let mut target_vec = match constructor_choice {
                        0 => {
                            let capacity = _to_usize(GLOBAL_DATA, base_offset + 3);
                            smallvec::SmallVec::<[i32; 12]>::with_capacity(capacity)
                        },
                        1 => {
                            let mut vec_data = std::vec::Vec::with_capacity(32);
                            let vec_size = _to_u8(GLOBAL_DATA, base_offset + 11) % 33;
                            for i in 0..vec_size {
                                let val = _to_i32(GLOBAL_DATA, base_offset + 15 + (i as usize * 4));
                                vec_data.push(val);
                            }
                            smallvec::SmallVec::<[i32; 12]>::from_vec(vec_data)
                        },
                        2 => {
                            let elem = _to_i32(GLOBAL_DATA, base_offset + 3);
                            let count = _to_usize(GLOBAL_DATA, base_offset + 7);
                            smallvec::SmallVec::<[i32; 12]>::from_elem(elem, count)
                        },
                        3 => {
                            let array = [_to_i32(GLOBAL_DATA, base_offset + 3), _to_i32(GLOBAL_DATA, base_offset + 7), _to_i32(GLOBAL_DATA, base_offset + 11), _to_i32(GLOBAL_DATA, base_offset + 15), _to_i32(GLOBAL_DATA, base_offset + 19), _to_i32(GLOBAL_DATA, base_offset + 23), _to_i32(GLOBAL_DATA, base_offset + 27), _to_i32(GLOBAL_DATA, base_offset + 31), _to_i32(GLOBAL_DATA, base_offset + 35), _to_i32(GLOBAL_DATA, base_offset + 39), _to_i32(GLOBAL_DATA, base_offset + 43), _to_i32(GLOBAL_DATA, base_offset + 47)];
                            smallvec::SmallVec::from(array)
                        },
                        _ => smallvec::SmallVec::<[i32; 12]>::new()
                    };
                    
                    target_vec.push(_to_i32(GLOBAL_DATA, base_offset + 51));
                    let pop_result = target_vec.pop();
                    if let Some(popped) = pop_result {
                        println!("{:?}", popped);
                    }
                    
                    let borrowed_result: &[i32] = target_vec.borrow();
                    println!("{:?}", borrowed_result);
                    let _ = &*borrowed_result;
                    
                    let slice_ref = target_vec.as_slice();
                    println!("{:?}", slice_ref);
                    let _ = &*slice_ref;
                    
                    let deref_result = target_vec.deref();
                    println!("{:?}", deref_result);
                    let _ = &*deref_result;
                    
                    target_vec.clear();
                    target_vec.shrink_to_fit();
                },
                1 => {
                    let constructor_choice = _to_u8(GLOBAL_DATA, base_offset + 2) % 3;
                    let mut target_vec = match constructor_choice {
                        0 => smallvec::SmallVec::<[u8; 15]>::new(),
                        1 => {
                            let capacity = _to_usize(GLOBAL_DATA, base_offset + 3);
                            smallvec::SmallVec::<[u8; 15]>::with_capacity(capacity)
                        },
                        _ => {
                            let slice_data = &[_to_u8(GLOBAL_DATA, base_offset + 3), _to_u8(GLOBAL_DATA, base_offset + 4), _to_u8(GLOBAL_DATA, base_offset + 5)];
                            smallvec::SmallVec::<[u8; 15]>::from_slice(slice_data)
                        }
                    };
                    
                    let num_elements = _to_u8(GLOBAL_DATA, base_offset + 7) % 25;
                    for i in 0..num_elements {
                        target_vec.push(_to_u8(GLOBAL_DATA, base_offset + 8 + i as usize));
                    }
                    
                    let borrowed_result: &[u8] = target_vec.borrow();
                    println!("{:?}", borrowed_result);
                    let _ = &*borrowed_result;
                    
                    let mut_borrowed: &mut [u8] = target_vec.borrow_mut();
                    println!("{:?}", mut_borrowed);
                    let _ = &*mut_borrowed;
                    
                    let capacity_result = target_vec.capacity();
                    println!("{:?}", capacity_result);
                    
                    let len_result = target_vec.len();
                    println!("{:?}", len_result);
                    
                    let spilled_result = target_vec.spilled();
                    println!("{:?}", spilled_result);
                },
                2 => {
                    let slice_data = &[_to_f32(GLOBAL_DATA, base_offset + 2), _to_f32(GLOBAL_DATA, base_offset + 6), _to_f32(GLOBAL_DATA, base_offset + 10), _to_f32(GLOBAL_DATA, base_offset + 14), _to_f32(GLOBAL_DATA, base_offset + 18)];
                    let mut target_vec = smallvec::SmallVec::<[f32; 20]>::from_slice(slice_data);
                    
                    let reserve_amount = _to_usize(GLOBAL_DATA, base_offset + 22);
                    target_vec.reserve(reserve_amount);
                    
                    let borrowed_result: &[f32] = target_vec.borrow();
                    println!("{:?}", borrowed_result);
                    let _ = &*borrowed_result;
                    
                    let as_ref_result = target_vec.as_ref();
                    println!("{:?}", as_ref_result);
                    let _ = &*as_ref_result;
                    
                    let clone_result = target_vec.clone();
                    let cloned_borrowed: &[f32] = clone_result.borrow();
                    println!("{:?}", cloned_borrowed);
                    let _ = &*cloned_borrowed;
                    
                    let as_ptr_result = target_vec.as_ptr();
                    println!("{:?}", as_ptr_result);
                },
                3 => {
                    let first_constructor = _to_u8(GLOBAL_DATA, base_offset + 2) % 3;
                    let second_constructor = _to_u8(GLOBAL_DATA, base_offset + 3) % 3;
                    
                    let mut first_vec = match first_constructor {
                        0 => smallvec::SmallVec::<[i64; 16]>::new(),
                        1 => {
                            let elem = _to_i64(GLOBAL_DATA, base_offset + 4);
                            let count = _to_usize(GLOBAL_DATA, base_offset + 12) % 20;
                            smallvec::SmallVec::<[i64; 16]>::from_elem(elem, count)
                        },
                        _ => {
                            let capacity = _to_usize(GLOBAL_DATA, base_offset + 4);
                            smallvec::SmallVec::<[i64; 16]>::with_capacity(capacity)
                        }
                    };
                    
                    let mut second_vec = match second_constructor {
                        0 => smallvec::SmallVec::<[i64; 16]>::new(),
                        1 => {
                            let elem = _to_i64(GLOBAL_DATA, base_offset + 20);
                            let count = _to_usize(GLOBAL_DATA, base_offset + 28) % 20;
                            smallvec::SmallVec::<[i64; 16]>::from_elem(elem, count)
                        },
                        _ => {
                            let capacity = _to_usize(GLOBAL_DATA, base_offset + 20);
                            smallvec::SmallVec::<[i64; 16]>::with_capacity(capacity)
                        }
                    };
                    
                    let first_size = _to_u8(GLOBAL_DATA, base_offset + 36) % 12;
                    let second_size = _to_u8(GLOBAL_DATA, base_offset + 37) % 12;
                    
                    for i in 0..first_size {
                        first_vec.push(_to_i64(GLOBAL_DATA, base_offset + 38 + (i as usize * 8)));
                    }
                    for i in 0..second_size {
                        second_vec.push(_to_i64(GLOBAL_DATA, base_offset + 54 + (i as usize * 8)));
                    }
                    
                    let first_borrowed: &[i64] = first_vec.borrow();
                    println!("{:?}", first_borrowed);
                    let _ = &*first_borrowed;
                    
                    let second_borrowed: &[i64] = second_vec.borrow();
                    println!("{:?}", second_borrowed);
                    let _ = &*second_borrowed;
                    
                    let cmp_result = first_vec.cmp(&second_vec);
                    println!("{:?}", cmp_result);
                    
                    let partial_cmp_result = first_vec.partial_cmp(&second_vec);
                    println!("{:?}", partial_cmp_result);
                    
                    first_vec.append(&mut second_vec);
                    println!("{:?}", second_vec.len());
                },
                4 => {
                    let constructor_choice = _to_u8(GLOBAL_DATA, base_offset + 2) % 3;
                    let mut target_vec = match constructor_choice {
                        0 => smallvec::SmallVec::<[String; 14]>::new(),
                        1 => {
                            let capacity = _to_usize(GLOBAL_DATA, base_offset + 3);
                            smallvec::SmallVec::<[String; 14]>::with_capacity(capacity)
                        },
                        _ => {
                            let iter_data = vec![String::from("test1"), String::from("test2")];
                            smallvec::SmallVec::<[String; 14]>::from_iter(iter_data)
                        }
                    };
                    
                    let num_strings = _to_u8(GLOBAL_DATA, base_offset + 11) % 8;
                    
                    for i in 0..num_strings {
                        let str_len = (_to_u8(GLOBAL_DATA, base_offset + 12 + i as usize) % 10) + 1;
                        let start_idx = base_offset + 20 + (i as usize * 10);
                        let end_idx = start_idx + str_len as usize;
                        if end_idx <= GLOBAL_DATA.len() {
                            let string_slice = _to_str(GLOBAL_DATA, start_idx, end_idx);
                            target_vec.push(string_slice.to_string());
                        }
                    }
                    
                    let borrowed_result: &[String] = target_vec.borrow();
                    println!("{:?}", borrowed_result);
                    let _ = &*borrowed_result;
                    
                    let insert_index = _to_usize(GLOBAL_DATA, base_offset + 70);
                    if !target_vec.is_empty() {
                        let safe_index = insert_index % target_vec.len();
                        target_vec.insert(safe_index, String::from("inserted"));
                    }
                    
                    let into_iter_result = target_vec.into_iter();
                    for item in into_iter_result {
                        println!("{:?}", item);
                    }
                },
                5 => {
                    let constructor_choice = _to_u8(GLOBAL_DATA, base_offset + 2) % 2;
                    let mut target_vec = match constructor_choice {
                        0 => smallvec::SmallVec::<[bool; 14]>::new(),
                        _ => {
                            let capacity = _to_usize(GLOBAL_DATA, base_offset + 3);
                            smallvec::SmallVec::<[bool; 14]>::with_capacity(capacity)
                        }
                    };
                    
                    let num_bools = _to_u8(GLOBAL_DATA, base_offset + 11) % 20;
                    
                    for i in 0..num_bools {
                        target_vec.push(_to_bool(GLOBAL_DATA, base_offset + 12 + i as usize));
                    }
                    
                    let borrowed_result: &[bool] = target_vec.borrow();
                    println!("{:?}", borrowed_result);
                    let _ = &*borrowed_result;
                    
                    let truncate_len = _to_usize(GLOBAL_DATA, base_offset + 32);
                    target_vec.truncate(truncate_len);
                    
                    let drain_start = _to_usize(GLOBAL_DATA, base_offset + 40);
                    let drain_end = _to_usize(GLOBAL_DATA, base_offset + 48);
                    
                    if !target_vec.is_empty() {
                        let actual_drain_start = drain_start % target_vec.len();
                        let actual_drain_end = (drain_end % target_vec.len()).max(actual_drain_start);
                        let mut drain_iter = target_vec.drain(actual_drain_start..actual_drain_end);
                        if let Some(drained_item) = drain_iter.next() {
                            println!("{:?}", drained_item);
                        }
                    }
                },
                6 => {
                    let constructor_choice = _to_u8(GLOBAL_DATA, base_offset + 2) % 3;
                    let target_vec = match constructor_choice {
                        0 => {
                            let iter_data = vec![_to_char(GLOBAL_DATA, base_offset + 3), _to_char(GLOBAL_DATA, base_offset + 7), _to_char(GLOBAL_DATA, base_offset + 11)];
                            smallvec::SmallVec::<[char; 16]>::from_iter(iter_data)
                        },
                        1 => {
                            let array = [_to_char(GLOBAL_DATA, base_offset + 3), _to_char(GLOBAL_DATA, base_offset + 7), _to_char(GLOBAL_DATA, base_offset + 11), _to_char(GLOBAL_DATA, base_offset + 15), _to_char(GLOBAL_DATA, base_offset + 19), _to_char(GLOBAL_DATA, base_offset + 23), _to_char(GLOBAL_DATA, base_offset + 27), _to_char(GLOBAL_DATA, base_offset + 31), _to_char(GLOBAL_DATA, base_offset + 35), _to_char(GLOBAL_DATA, base_offset + 39), _to_char(GLOBAL_DATA, base_offset + 43), _to_char(GLOBAL_DATA, base_offset + 47), _to_char(GLOBAL_DATA, base_offset + 51), _to_char(GLOBAL_DATA, base_offset + 55), _to_char(GLOBAL_DATA, base_offset + 59), _to_char(GLOBAL_DATA, base_offset + 63)];
                            smallvec::SmallVec::from(array)
                        },
                        _ => {
                            let mut vec = smallvec::SmallVec::<[char; 16]>::new();
                            vec.push(_to_char(GLOBAL_DATA, base_offset + 3));
                            vec
                        }
                    };
                    
                    let borrowed_result: &[char] = target_vec.borrow();
                    println!("{:?}", borrowed_result);
                    let _ = &*borrowed_result;
                    
                    let index_val = _to_usize(GLOBAL_DATA, base_offset + 19);
                    if !target_vec.is_empty() {
                        let safe_index = index_val % target_vec.len();
                        let indexed_ref = &target_vec[safe_index];
                        println!("{:?}", indexed_ref);
                        let _ = &*indexed_ref;
                    }
                    
                    let into_boxed = target_vec.into_boxed_slice();
                    println!("{:?}", into_boxed.len());
                },
                7 => {
                    let to_smallvec_data: &[u16] = &[_to_u16(GLOBAL_DATA, base_offset + 2), _to_u16(GLOBAL_DATA, base_offset + 4), _to_u16(GLOBAL_DATA, base_offset + 6), _to_u16(GLOBAL_DATA, base_offset + 8)];
                    let target_vec: smallvec::SmallVec<[u16; 32]> = to_smallvec_data.to_smallvec();
                    
                    let borrowed_result: &[u16] = target_vec.borrow();
                    println!("{:?}", borrowed_result);
                    let _ = &*borrowed_result;
                    
                    let as_ptr_result = target_vec.as_ptr();
                    println!("{:?}", as_ptr_result);
                    
                    let spilled_result = target_vec.spilled();
                    println!("{:?}", spilled_result);
                    
                    let into_vec_result = target_vec.into_vec();
                    println!("{:?}", into_vec_result.len());
                },
                8 => {
                    let constructor_choice = _to_u8(GLOBAL_DATA, base_offset + 2) % 2;
                    let mut target_vec = match constructor_choice {
                        0 => {
                            let vec_data = vec![_to_f64(GLOBAL_DATA, base_offset + 3), _to_f64(GLOBAL_DATA, base_offset + 11)];
                            smallvec::SmallVec::<[f64; 12]>::from_vec(vec_data)
                        },
                        _ => smallvec::SmallVec::<[f64; 12]>::new()
                    };
                    
                    let remove_index = _to_usize(GLOBAL_DATA, base_offset + 19);
                    let swap_remove_index = _to_usize(GLOBAL_DATA, base_offset + 27);
                    
                    target_vec.push(_to_f64(GLOBAL_DATA, base_offset + 35));
                    target_vec.push(_to_f64(GLOBAL_DATA, base_offset + 43));
                    
                    if !target_vec.is_empty() {
                        let safe_remove_index = remove_index % target_vec.len();
                        let removed = target_vec.remove(safe_remove_index);
                        println!("{:?}", removed);
                    }
                    
                    if !target_vec.is_empty() {
                        let safe_swap_index = swap_remove_index % target_vec.len();
                        let swap_removed = target_vec.swap_remove(safe_swap_index);
                        println!("{:?}", swap_removed);
                    }
                    
                    let borrowed_result: &[f64] = target_vec.borrow();
                    println!("{:?}", borrowed_result);
                    let _ = &*borrowed_result;
                },
                _ => {
                    let constructor_choice = _to_u8(GLOBAL_DATA, base_offset + 2) % 2;
                    let mut target_vec = match constructor_choice {
                        0 => smallvec::SmallVec::<[i16; 20]>::new(),
                        _ => {
                            let capacity = _to_usize(GLOBAL_DATA, base_offset + 3);
                            smallvec::SmallVec::<[i16; 20]>::with_capacity(capacity)
                        }
                    };
                    
                    let resize_len = _to_usize(GLOBAL_DATA, base_offset + 11);
                    let resize_value = _to_i16(GLOBAL_DATA, base_offset + 19);
                    target_vec.resize(resize_len, resize_value);
                    
                    let extend_data = vec![_to_i16(GLOBAL_DATA, base_offset + 21), _to_i16(GLOBAL_DATA, base_offset + 23)];
                    target_vec.extend(extend_data);
                    
                    let borrowed_result: &[i16] = target_vec.borrow();
                    println!("{:?}", borrowed_result);
                    let _ = &*borrowed_result;
                    
                    target_vec.dedup();
                    
                    let as_mut_slice = target_vec.as_mut_slice();
                    println!("{:?}", as_mut_slice.len());
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