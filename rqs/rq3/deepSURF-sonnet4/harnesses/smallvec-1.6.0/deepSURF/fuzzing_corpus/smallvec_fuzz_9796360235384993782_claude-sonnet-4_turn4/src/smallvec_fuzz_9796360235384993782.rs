#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};
use std::borrow::{Borrow, BorrowMut};

#[derive(Clone, Copy, PartialEq, Debug, PartialOrd, Ord, Eq)]
struct CustomType1(u32);

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 600 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        
        let num_operations = _to_u8(GLOBAL_DATA, 0) % 65;
        
        for i in 0..num_operations {
            let op_type = _to_u8(GLOBAL_DATA, 1 + i as usize) % 20;
            
            match op_type {
                0 => {
                    let constructor_choice = _to_u8(GLOBAL_DATA, 50 + i as usize) % 10;
                    let mut smallvec = match constructor_choice {
                        0 => SmallVec::<[CustomType1; 16]>::new(),
                        1 => {
                            let cap = _to_usize(GLOBAL_DATA, 100 + i as usize * 8);
                            SmallVec::<[CustomType1; 16]>::with_capacity(cap)
                        },
                        2 => {
                            let vec_size = _to_usize(GLOBAL_DATA, 200 + i as usize * 8) % 65;
                            let mut vec = Vec::new();
                            for j in 0..vec_size {
                                let val = _to_u32(GLOBAL_DATA, 150 + j * 4);
                                vec.push(CustomType1(val));
                            }
                            SmallVec::<[CustomType1; 16]>::from_vec(vec)
                        },
                        3 => {
                            let slice_size = _to_usize(GLOBAL_DATA, 180 + i as usize * 8) % 65;
                            let mut slice_vec = Vec::new();
                            for j in 0..slice_size {
                                let val = _to_u32(GLOBAL_DATA, 220 + j * 4);
                                slice_vec.push(CustomType1(val));
                            }
                            SmallVec::<[CustomType1; 16]>::from(&slice_vec[..])
                        },
                        4 => {
                            let elem_val = _to_u32(GLOBAL_DATA, 250 + i as usize * 4);
                            let elem = CustomType1(elem_val);
                            let count = _to_usize(GLOBAL_DATA, 260 + i as usize * 8) % 65;
                            SmallVec::<[CustomType1; 16]>::from_elem(elem, count)
                        },
                        5 => {
                            let iter_size = _to_usize(GLOBAL_DATA, 270 + i as usize * 8) % 65;
                            let mut iter_vec = Vec::new();
                            for j in 0..iter_size {
                                let val = _to_u32(GLOBAL_DATA, 280 + j * 4);
                                iter_vec.push(CustomType1(val));
                            }
                            SmallVec::<[CustomType1; 16]>::from_iter(iter_vec.into_iter())
                        },
                        6 => {
                            let buf_arr = [CustomType1(_to_u32(GLOBAL_DATA, 300 + i as usize * 64)); 16];
                            SmallVec::<[CustomType1; 16]>::from_buf(buf_arr)
                        },
                        7 => {
                            let buf_arr = [CustomType1(_to_u32(GLOBAL_DATA, 301 + i as usize * 64)); 16];
                            let len = _to_usize(GLOBAL_DATA, 302 + i as usize * 8) % 17;
                            SmallVec::<[CustomType1; 16]>::from_buf_and_len(buf_arr, len)
                        },
                        8 => {
                            let slice_size = _to_usize(GLOBAL_DATA, 303 + i as usize * 8) % 65;
                            let mut slice_vec = Vec::new();
                            for j in 0..slice_size {
                                let val = _to_u32(GLOBAL_DATA, 304 + j * 4);
                                slice_vec.push(CustomType1(val));
                            }
                            slice_vec[..].to_smallvec()
                        },
                        _ => SmallVec::<[CustomType1; 16]>::new()
                    };
                    
                    let insert_index = _to_usize(GLOBAL_DATA, 120 + i as usize * 8);
                    let element_val = _to_u32(GLOBAL_DATA, 140 + i as usize * 4);
                    let element = CustomType1(element_val);
                    
                    smallvec.insert(insert_index, element);
                    
                    let slice_ref = smallvec.as_slice();
                    println!("{:?}", slice_ref.len());
                    
                    let mut_slice_ref = smallvec.as_mut_slice();
                    println!("{:?}", mut_slice_ref.len());
                    
                    if !smallvec.is_empty() {
                        let first_ref = &smallvec[0];
                        println!("{:?}", first_ref);
                    }
                    
                    let deref_slice = &*smallvec;
                    println!("{:?}", deref_slice.len());
                },
                1 => {
                    let mut smallvec = SmallVec::<[CustomType1; 32]>::new();
                    let push_count = _to_u8(GLOBAL_DATA, 80 + i as usize) % 30;
                    for j in 0..push_count {
                        let val = _to_u32(GLOBAL_DATA, 90 + j as usize * 4);
                        smallvec.push(CustomType1(val));
                    }
                    
                    let capacity = smallvec.capacity();
                    println!("{:?}", capacity);
                    
                    let popped_item = smallvec.pop();
                    if let Some(item) = popped_item {
                        println!("{:?}", item);
                    }
                    
                    let inline_size = smallvec.inline_size();
                    println!("{:?}", inline_size);
                    
                    let spilled = smallvec.spilled();
                    println!("{:?}", spilled);
                },
                2 => {
                    let mut smallvec = SmallVec::<[CustomType1; 16]>::new();
                    let reserve_amount = _to_usize(GLOBAL_DATA, 70 + i as usize * 8);
                    smallvec.reserve(reserve_amount);
                    
                    let remove_index = _to_usize(GLOBAL_DATA, 160 + i as usize * 8);
                    let val = _to_u32(GLOBAL_DATA, 170 + i as usize * 4);
                    smallvec.push(CustomType1(val));
                    
                    if !smallvec.is_empty() {
                        let removed_item = smallvec.remove(remove_index);
                        println!("{:?}", removed_item);
                    }
                    
                    let len = smallvec.len();
                    println!("{:?}", len);
                },
                3 => {
                    let mut smallvec = SmallVec::<[CustomType1; 12]>::new();
                    let truncate_len = _to_usize(GLOBAL_DATA, 190 + i as usize * 8);
                    
                    for j in 0..20 {
                        let val = _to_u32(GLOBAL_DATA, 210 + j * 4);
                        smallvec.push(CustomType1(val));
                    }
                    
                    smallvec.truncate(truncate_len);
                    
                    let as_slice = smallvec.as_slice();
                    for item in as_slice {
                        println!("{:?}", item);
                    }
                    
                    let borrow_slice: &[CustomType1] = smallvec.borrow();
                    println!("{:?}", borrow_slice.len());
                    
                    let as_ref_slice = smallvec.as_ref();
                    println!("{:?}", as_ref_slice.len());
                },
                4 => {
                    let mut smallvec = SmallVec::<[CustomType1; 8]>::new();
                    let val = _to_u32(GLOBAL_DATA, 230 + i as usize * 4);
                    smallvec.push(CustomType1(val));
                    
                    let swap_remove_index = _to_usize(GLOBAL_DATA, 240 + i as usize * 8);
                    if !smallvec.is_empty() {
                        let swapped_item = smallvec.swap_remove(swap_remove_index);
                        println!("{:?}", swapped_item);
                    }
                    
                    let borrow_mut_slice: &mut [CustomType1] = smallvec.borrow_mut();
                    println!("{:?}", borrow_mut_slice.len());
                    
                    let as_mut_slice = smallvec.as_mut();
                    println!("{:?}", as_mut_slice.len());
                },
                5 => {
                    let mut smallvec1 = SmallVec::<[CustomType1; 16]>::new();
                    let mut smallvec2 = SmallVec::<[CustomType1; 16]>::new();
                    
                    let val1 = _to_u32(GLOBAL_DATA, 250 + i as usize * 4);
                    smallvec1.push(CustomType1(val1));
                    
                    let val2 = _to_u32(GLOBAL_DATA, 260 + i as usize * 4);
                    smallvec2.push(CustomType1(val2));
                    
                    smallvec1.append(&mut smallvec2);
                    
                    let len_result = smallvec1.len();
                    println!("{:?}", len_result);
                    
                    let eq_result = smallvec1.eq(&smallvec2);
                    println!("{:?}", eq_result);
                },
                6 => {
                    let mut smallvec = SmallVec::<[CustomType1; 20]>::new();
                    let resize_len = _to_usize(GLOBAL_DATA, 270 + i as usize * 8) % 65;
                    let val = _to_u32(GLOBAL_DATA, 280 + i as usize * 4);
                    let value = CustomType1(val);
                    
                    smallvec.resize(resize_len, value);
                    
                    if !smallvec.is_empty() {
                        let item_ref = &smallvec[0];
                        println!("{:?}", item_ref);
                    }
                    
                    let as_ptr = smallvec.as_ptr();
                    println!("{:?}", as_ptr as usize);
                },
                7 => {
                    let mut smallvec = SmallVec::<[CustomType1; 24]>::new();
                    let grow_amount = _to_usize(GLOBAL_DATA, 290 + i as usize * 8);
                    smallvec.grow(grow_amount);
                    
                    let spilled_status = smallvec.spilled();
                    println!("{:?}", spilled_status);
                    
                    let as_mut_ptr = smallvec.as_mut_ptr();
                    println!("{:?}", as_mut_ptr as usize);
                },
                8 => {
                    let mut smallvec = SmallVec::<[CustomType1; 10]>::new();
                    for j in 0..15 {
                        let val = _to_u32(GLOBAL_DATA, 100 + j * 4);
                        smallvec.push(CustomType1(val));
                    }
                    
                    let drain_range = _to_usize(GLOBAL_DATA, 110 + i as usize * 8) % 5..;
                    let mut drain_iter = smallvec.drain(drain_range);
                    
                    while let Some(item) = drain_iter.next() {
                        println!("{:?}", item);
                    }
                    
                    let back_item = drain_iter.next_back();
                    if let Some(item) = back_item {
                        println!("{:?}", item);
                    }
                },
                9 => {
                    let smallvec1 = SmallVec::<[CustomType1; 16]>::new();
                    let smallvec2 = SmallVec::<[CustomType1; 16]>::new();
                    
                    let cmp_result = smallvec1.cmp(&smallvec2);
                    println!("{:?}", cmp_result);
                    
                    let smallvec3 = SmallVec::<[CustomType1; 16]>::new();
                    let smallvec4 = SmallVec::<[CustomType1; 16]>::new();
                    let partial_cmp_result = smallvec3.partial_cmp(&smallvec4);
                    if let Some(ordering) = partial_cmp_result {
                        println!("{:?}", ordering);
                    }
                },
                10 => {
                    let mut smallvec = SmallVec::<[CustomType1; 14]>::new();
                    let insert_many_index = _to_usize(GLOBAL_DATA, 120 + i as usize * 8);
                    
                    let items_count = _to_u8(GLOBAL_DATA, 130 + i as usize) % 20;
                    let mut items = Vec::new();
                    for j in 0..items_count {
                        let val = _to_u32(GLOBAL_DATA, 140 + j as usize * 4);
                        items.push(CustomType1(val));
                    }
                    
                    smallvec.insert_many(insert_many_index, items);
                    
                    let deref_slice = &*smallvec;
                    println!("{:?}", deref_slice.len());
                    
                    let retain_filter = _to_u8(GLOBAL_DATA, 131 + i as usize) % 3;
                    smallvec.retain(|x| x.0 % 3 == retain_filter as u32);
                },
                11 => {
                    let mut smallvec = SmallVec::<[CustomType1; 18]>::new();
                    let val = _to_u32(GLOBAL_DATA, 150 + i as usize * 4);
                    let item = CustomType1(val);
                    
                    smallvec.push(item);
                    smallvec.push(item);
                    smallvec.push(item);
                    
                    smallvec.dedup();
                    
                    let final_len = smallvec.len();
                    println!("{:?}", final_len);
                    
                    let resize_with_len = _to_usize(GLOBAL_DATA, 151 + i as usize * 8) % 30;
                    let resize_val = _to_u32(GLOBAL_DATA, 152 + i as usize * 4);
                    smallvec.resize_with(resize_with_len, || CustomType1(resize_val));
                },
                12 => {
                    let mut smallvec = SmallVec::<[CustomType1; 22]>::new();
                    let exact_reserve_amount = _to_usize(GLOBAL_DATA, 160 + i as usize * 8);
                    smallvec.reserve_exact(exact_reserve_amount);
                    
                    smallvec.shrink_to_fit();
                    
                    let final_capacity = smallvec.capacity();
                    println!("{:?}", final_capacity);
                    
                    let dedup_by_threshold = _to_u32(GLOBAL_DATA, 161 + i as usize * 4);
                    for j in 0..5 {
                        let val = _to_u32(GLOBAL_DATA, 162 + j * 4);
                        smallvec.push(CustomType1(val));
                    }
                    if dedup_by_threshold > 0 {
                        smallvec.dedup_by(|a, b| (a.0 / dedup_by_threshold) == (b.0 / dedup_by_threshold));
                    }
                },
                13 => {
                    let slice_size = _to_u8(GLOBAL_DATA, 170 + i as usize) % 30;
                    let mut slice_items = Vec::new();
                    for j in 0..slice_size {
                        let val = _to_u32(GLOBAL_DATA, 180 + j as usize * 4);
                        slice_items.push(CustomType1(val));
                    }
                    
                    let converted_smallvec = if !slice_items.is_empty() {
                        slice_items[..].to_smallvec()
                    } else {
                        SmallVec::<[CustomType1; 26]>::new()
                    };
                    
                    let into_vec = converted_smallvec.into_vec();
                    println!("{:?}", into_vec.len());
                },
                14 => {
                    let mut smallvec = SmallVec::<[CustomType1; 30]>::new();
                    for j in 0..10 {
                        let val = _to_u32(GLOBAL_DATA, 190 + j * 4);
                        smallvec.push(CustomType1(val));
                    }
                    
                    let into_iter = smallvec.into_iter();
                    for item in into_iter {
                        println!("{:?}", item);
                    }
                },
                15 => {
                    let mut smallvec = SmallVec::<[CustomType1; 32]>::new();
                    let as_mut_ptr = smallvec.as_mut_ptr();
                    println!("{:?}", as_mut_ptr as usize);
                    
                    smallvec.clear();
                    
                    let is_empty_result = smallvec.is_empty();
                    println!("{:?}", is_empty_result);
                    
                    let default_smallvec = SmallVec::<[CustomType1; 32]>::default();
                    println!("{:?}", default_smallvec.len());
                },
                16 => {
                    let mut smallvec = SmallVec::<[CustomType1; 32]>::new();
                    let insert_slice_index = _to_usize(GLOBAL_DATA, 200 + i as usize * 8);
                    
                    let slice_size = _to_u8(GLOBAL_DATA, 210 + i as usize) % 25;
                    let mut slice_items = Vec::new();
                    for j in 0..slice_size {
                        let val = _to_u32(GLOBAL_DATA, 220 + j as usize * 4);
                        slice_items.push(CustomType1(val));
                    }
                    
                    if !slice_items.is_empty() {
                        smallvec.insert_from_slice(insert_slice_index, &slice_items);
                    }
                    
                    let as_ptr_result = smallvec.as_ptr();
                    println!("{:?}", as_ptr_result as usize);
                    
                    let dedup_key_mod = _to_u32(GLOBAL_DATA, 221 + i as usize * 4);
                    if dedup_key_mod > 0 {
                        smallvec.dedup_by_key(|x| x.0 / dedup_key_mod);
                    }
                },
                17 => {
                    let mut smallvec = SmallVec::<[CustomType1; 32]>::new();
                    let extend_items_count = _to_u8(GLOBAL_DATA, 230 + i as usize) % 40;
                    let mut extend_items = Vec::new();
                    for j in 0..extend_items_count {
                        let val = _to_u32(GLOBAL_DATA, 240 + j as usize * 4);
                        extend_items.push(CustomType1(val));
                    }
                    
                    smallvec.extend(extend_items);
                    
                    let cloned_smallvec = smallvec.clone();
                    println!("{:?}", cloned_smallvec.len());
                    
                    let into_iter = cloned_smallvec.into_iter();
                    let as_slice = into_iter.as_slice();
                    println!("{:?}", as_slice.len());
                },
                18 => {
                    let mut smallvec = SmallVec::<[CustomType1; 32]>::new();
                    let try_reserve_amount = _to_usize(GLOBAL_DATA, 250 + i as usize * 8);
                    let try_result = smallvec.try_reserve(try_reserve_amount);
                    
                    match try_result {
                        Ok(_) => println!("Reserve successful"),
                        Err(_) => println!("Reserve failed"),
                    }
                    
                    let inline_size = smallvec.inline_size();
                    println!("{:?}", inline_size);
                    
                    let try_reserve_exact_amount = _to_usize(GLOBAL_DATA, 251 + i as usize * 8);
                    let try_exact_result = smallvec.try_reserve_exact(try_reserve_exact_amount);
                    match try_exact_result {
                        Ok(_) => println!("Reserve exact successful"),
                        Err(_) => println!("Reserve exact failed"),
                    }
                },
                _ => {
                    let mut smallvec = SmallVec::<[CustomType1; 32]>::new();
                    let try_grow_amount = _to_usize(GLOBAL_DATA, 260 + i as usize * 8);
                    let try_grow_result = smallvec.try_grow(try_grow_amount);
                    
                    match try_grow_result {
                        Ok(_) => println!("Grow successful"),
                        Err(_) => println!("Grow failed"),
                    }
                    
                    let boxed_slice = smallvec.into_boxed_slice();
                    println!("{:?}", boxed_slice.len());
                    
                    let into_inner_result = SmallVec::<[CustomType1; 32]>::new().into_inner();
                    match into_inner_result {
                        Ok(arr) => println!("Into inner successful"),
                        Err(vec) => println!("Into inner failed, len: {:?}", vec.len()),
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