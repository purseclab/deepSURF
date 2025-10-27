#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut, RangeBounds};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

struct CustomType0;
struct CustomType1;

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 512 {return;}
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = &global_data.first_half;
        
        let num_operations = _to_u8(GLOBAL_DATA, 0) % 65;
        let mut offset = 1;
        
        for _ in 0..num_operations {
            if offset + 100 >= GLOBAL_DATA.len() { break; }
            
            let operation = _to_u8(GLOBAL_DATA, offset) % 6;
            offset += 1;
            
            match operation {
                0 => {
                    let constructor_choice = _to_u8(GLOBAL_DATA, offset) % 4;
                    offset += 1;
                    
                    let mut smallvec = match constructor_choice {
                        0 => {
                            let capacity = _to_usize(GLOBAL_DATA, offset);
                            offset += 8;
                            SmallVec::<[i32; 15]>::with_capacity(capacity)
                        },
                        1 => {
                            let elem = _to_i32(GLOBAL_DATA, offset);
                            offset += 4;
                            let n = _to_usize(GLOBAL_DATA, offset);
                            offset += 8;
                            SmallVec::<[i32; 15]>::from_elem(elem, n)
                        },
                        2 => {
                            let mut vec = Vec::new();
                            let vec_size = _to_u8(GLOBAL_DATA, offset) % 65;
                            offset += 1;
                            for i in 0..vec_size {
                                if offset + 4 >= GLOBAL_DATA.len() { break; }
                                vec.push(_to_i32(GLOBAL_DATA, offset));
                                offset += 4;
                            }
                            SmallVec::<[i32; 15]>::from_vec(vec)
                        },
                        _ => SmallVec::<[i32; 15]>::new()
                    };
                    
                    let reserve_amount = _to_usize(GLOBAL_DATA, offset);
                    offset += 8;
                    
                    let _result = smallvec.try_reserve(reserve_amount);
                    
                    let item = _to_i32(GLOBAL_DATA, offset);
                    offset += 4;
                    smallvec.push(item);
                    
                    let len = smallvec.len();
                    println!("Length: {}", len);
                    
                    if !smallvec.is_empty() {
                        let slice = smallvec.as_slice();
                        println!("First element: {}", *slice.index(0));
                        let _item = smallvec.pop();
                    }
                    
                    let capacity = smallvec.capacity();
                    println!("Capacity: {}", capacity);
                    
                    let spilled = smallvec.spilled();
                    println!("Spilled: {}", spilled);
                },
                1 => {
                    let constructor_choice = _to_u8(GLOBAL_DATA, offset) % 3;
                    offset += 1;
                    
                    let mut smallvec = match constructor_choice {
                        0 => SmallVec::<[u8; 20]>::new(),
                        1 => {
                            let cap = _to_usize(GLOBAL_DATA, offset);
                            offset += 8;
                            SmallVec::<[u8; 20]>::with_capacity(cap)
                        },
                        _ => {
                            let mut vec = Vec::new();
                            let vec_size = _to_u8(GLOBAL_DATA, offset) % 65;
                            offset += 1;
                            for i in 0..vec_size {
                                if offset >= GLOBAL_DATA.len() { break; }
                                vec.push(_to_u8(GLOBAL_DATA, offset));
                                offset += 1;
                            }
                            SmallVec::<[u8; 20]>::from_vec(vec)
                        }
                    };
                    
                    let slice_size = _to_u8(GLOBAL_DATA, offset) % 65;
                    offset += 1;
                    
                    let mut slice_data = Vec::new();
                    for i in 0..slice_size {
                        if offset >= GLOBAL_DATA.len() { break; }
                        slice_data.push(_to_u8(GLOBAL_DATA, offset));
                        offset += 1;
                    }
                    
                    smallvec.extend_from_slice(&slice_data);
                    
                    let reserve_amount = _to_usize(GLOBAL_DATA, offset);
                    offset += 8;
                    
                    let _result = smallvec.try_reserve(reserve_amount);
                    
                    if !smallvec.is_empty() {
                        let index = _to_usize(GLOBAL_DATA, offset);
                        offset += 8;
                        if index < smallvec.len() {
                            let _removed = smallvec.remove(index);
                        }
                    }
                    
                    let mut_slice = smallvec.as_mut_slice();
                    if !mut_slice.is_empty() {
                        println!("First mutable element: {}", *mut_slice.index_mut(0));
                    }
                },
                2 => {
                    let constructor_choice = _to_u8(GLOBAL_DATA, offset) % 3;
                    offset += 1;
                    
                    let mut smallvec1 = match constructor_choice {
                        0 => SmallVec::<[String; 12]>::new(),
                        1 => {
                            let cap = _to_usize(GLOBAL_DATA, offset);
                            offset += 8;
                            SmallVec::<[String; 12]>::with_capacity(cap)
                        },
                        _ => {
                            let slice_len = _to_u8(GLOBAL_DATA, offset) % 5;
                            offset += 1;
                            let mut slice_data = Vec::new();
                            for _ in 0..slice_len {
                                if offset + 5 < GLOBAL_DATA.len() {
                                    let str_data = _to_str(GLOBAL_DATA, offset, offset + 5);
                                    slice_data.push(String::from(str_data));
                                    offset += 5;
                                }
                            }
                            SmallVec::<[String; 12]>::from_vec(slice_data)
                        }
                    };
                    
                    let mut smallvec2 = SmallVec::<[String; 12]>::new();
                    
                    let str_len = _to_u8(GLOBAL_DATA, offset) % 10 + 1;
                    offset += 1;
                    
                    if offset + (str_len as usize) < GLOBAL_DATA.len() {
                        let string_data = _to_str(GLOBAL_DATA, offset, offset + str_len as usize);
                        let string = String::from(string_data);
                        smallvec1.push(string.clone());
                        smallvec2.push(string);
                        offset += str_len as usize;
                    }
                    
                    let reserve_amount = _to_usize(GLOBAL_DATA, offset);
                    offset += 8;
                    
                    let _result = smallvec1.try_reserve(reserve_amount);
                    
                    smallvec1.append(&mut smallvec2);
                    
                    let cloned = smallvec1.clone();
                    let are_equal = smallvec1.eq(&cloned);
                    println!("Equal: {}", are_equal);
                    
                    let ordering = smallvec1.cmp(&cloned);
                    println!("Ordering: {:?}", ordering);
                    
                    let partial_ordering = smallvec1.partial_cmp(&cloned);
                    println!("Partial ordering: {:?}", partial_ordering);
                    
                    let deref_result = smallvec1.deref();
                    println!("Dereferenced length: {}", deref_result.len());
                },
                3 => {
                    let constructor_choice = _to_u8(GLOBAL_DATA, offset) % 3;
                    offset += 1;
                    
                    let mut smallvec = match constructor_choice {
                        0 => SmallVec::<[i32; 15]>::new(),
                        1 => {
                            let cap = _to_usize(GLOBAL_DATA, offset);
                            offset += 8;
                            SmallVec::<[i32; 15]>::with_capacity(cap)
                        },
                        _ => {
                            let items_count = _to_u8(GLOBAL_DATA, offset) % 10;
                            offset += 1;
                            let mut items = Vec::new();
                            for _ in 0..items_count {
                                if offset + 4 >= GLOBAL_DATA.len() { break; }
                                items.push(_to_i32(GLOBAL_DATA, offset));
                                offset += 4;
                            }
                            SmallVec::<[i32; 15]>::from_iter(items.into_iter())
                        }
                    };
                    
                    let iter_size = _to_u8(GLOBAL_DATA, offset) % 10;
                    offset += 1;
                    
                    let mut items = Vec::new();
                    for i in 0..iter_size {
                        if offset + 4 >= GLOBAL_DATA.len() { break; }
                        items.push(_to_i32(GLOBAL_DATA, offset));
                        offset += 4;
                    }
                    
                    smallvec.extend(items.iter().cloned());
                    
                    let reserve_amount = _to_usize(GLOBAL_DATA, offset);
                    offset += 8;
                    
                    let _result = smallvec.try_reserve(reserve_amount);
                    
                    let insert_index = _to_usize(GLOBAL_DATA, offset);
                    offset += 8;
                    let insert_item = _to_i32(GLOBAL_DATA, offset);
                    offset += 4;
                    
                    if insert_index <= smallvec.len() {
                        smallvec.insert(insert_index, insert_item);
                    }
                    
                    let truncate_len = _to_usize(GLOBAL_DATA, offset);
                    offset += 8;
                    smallvec.truncate(truncate_len);
                    
                    let range_start = _to_usize(GLOBAL_DATA, offset);
                    offset += 8;
                    let range_end = _to_usize(GLOBAL_DATA, offset);
                    offset += 8;
                    
                    if range_start <= smallvec.len() && range_end <= smallvec.len() && range_start <= range_end {
                        let mut drain = smallvec.drain(range_start..range_end);
                        if let Some(item) = drain.next() {
                            println!("Drained item: {}", item);
                        }
                    }
                    
                    let as_ref_result = smallvec.as_ref();
                    println!("AsRef slice length: {}", as_ref_result.len());
                },
                4 => {
                    let mut smallvec = SmallVec::<[u8; 20]>::with_capacity(_to_usize(GLOBAL_DATA, offset));
                    offset += 8;
                    
                    let byte_val = _to_u8(GLOBAL_DATA, offset);
                    offset += 1;
                    let count = _to_usize(GLOBAL_DATA, offset);
                    offset += 8;
                    
                    smallvec.resize(count, byte_val);
                    
                    let reserve_amount = _to_usize(GLOBAL_DATA, offset);
                    offset += 8;
                    
                    let _result = smallvec.try_reserve(reserve_amount);
                    
                    let mut hasher = DefaultHasher::new();
                    smallvec.hash(&mut hasher);
                    let hash_value = hasher.finish();
                    println!("Hash: {}", hash_value);
                    
                    smallvec.retain(|&mut x| x % 2 == 0);
                    
                    smallvec.dedup();
                    
                    smallvec.shrink_to_fit();
                    
                    let as_ptr = smallvec.as_ptr();
                    println!("Pointer: {:?}", as_ptr);
                    
                    let as_mut_ptr = smallvec.as_mut_ptr();
                    println!("Mutable pointer: {:?}", as_mut_ptr);
                    
                    let into_iter = smallvec.into_iter();
                    for item in into_iter {
                        println!("Consumed item: {}", item);
                        break;
                    }
                },
                _ => {
                    let initial_cap = _to_usize(GLOBAL_DATA, offset);
                    offset += 8;
                    let mut smallvec = SmallVec::<[i32; 15]>::with_capacity(initial_cap);
                    
                    let num_pushes = _to_u8(GLOBAL_DATA, offset) % 10;
                    offset += 1;
                    
                    for i in 0..num_pushes {
                        if offset + 4 >= GLOBAL_DATA.len() { break; }
                        let item = _to_i32(GLOBAL_DATA, offset);
                        offset += 4;
                        smallvec.push(item);
                    }
                    
                    let first_reserve = _to_usize(GLOBAL_DATA, offset);
                    offset += 8;
                    smallvec.reserve(first_reserve);
                    
                    let try_reserve_amount = _to_usize(GLOBAL_DATA, offset);
                    offset += 8;
                    
                    let _result = smallvec.try_reserve(try_reserve_amount);
                    
                    let exact_reserve = _to_usize(GLOBAL_DATA, offset);
                    offset += 8;
                    smallvec.reserve_exact(exact_reserve);
                    
                    let try_exact_reserve = _to_usize(GLOBAL_DATA, offset);
                    offset += 8;
                    let _exact_result = smallvec.try_reserve_exact(try_exact_reserve);
                    
                    if !smallvec.is_empty() {
                        let swap_index = _to_usize(GLOBAL_DATA, offset);
                        offset += 8;
                        if swap_index < smallvec.len() {
                            let _swapped = smallvec.swap_remove(swap_index);
                        }
                    }
                    
                    let into_vec = smallvec.into_vec();
                    let from_vec = SmallVec::<[i32; 15]>::from_vec(into_vec);
                    
                    let boxed_slice = from_vec.into_boxed_slice();
                    println!("Boxed slice length: {}", boxed_slice.len());
                    
                    let borrow_result = boxed_slice.as_ref();
                    if !borrow_result.is_empty() {
                        println!("Borrowed first element: {}", *borrow_result.index(0));
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