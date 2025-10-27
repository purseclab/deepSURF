#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 200 {return;}
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        
        let num_operations = _to_usize(GLOBAL_DATA, 0) % 32 + 1;
        
        for i in 0..num_operations {
            let base_offset = (i * 6) % (GLOBAL_DATA.len() - 40);
            let operation = _to_u8(GLOBAL_DATA, base_offset) % 15;
            
            match operation {
                0 => {
                    let constructor_choice = _to_u8(GLOBAL_DATA, base_offset + 1) % 8;
                    let mut smallvec: SmallVec<[String; 16]> = match constructor_choice {
                        0 => SmallVec::new(),
                        1 => {
                            let capacity = _to_usize(GLOBAL_DATA, base_offset + 2);
                            SmallVec::with_capacity(capacity)
                        },
                        2 => {
                            let elem_count = _to_usize(GLOBAL_DATA, base_offset + 2) % 10;
                            let mut vec = Vec::new();
                            for j in 0..elem_count {
                                let str_len = _to_u8(GLOBAL_DATA, (base_offset + 3 + j) % GLOBAL_DATA.len()) % 20 + 1;
                                let start_idx = (base_offset + 4 + j * 2) % (GLOBAL_DATA.len() - str_len as usize);
                                let end_idx = start_idx + str_len as usize;
                                let s = _to_str(GLOBAL_DATA, start_idx, end_idx);
                                vec.push(String::from(s));
                            }
                            SmallVec::from_vec(vec)
                        },
                        3 => {
                            let elem = String::from("test");
                            let count = _to_usize(GLOBAL_DATA, base_offset + 2) % 10;
                            SmallVec::from_elem(elem, count)
                        },
                        4 => {
                            let slice_len = _to_u8(GLOBAL_DATA, base_offset + 2) % 15 + 1;
                            let slice_start = (base_offset + 3) % (GLOBAL_DATA.len() - slice_len as usize);
                            let slice_end = slice_start + slice_len as usize;
                            let slice_str = _to_str(GLOBAL_DATA, slice_start, slice_end);
                            let mut slice_vec = Vec::new();
                            slice_vec.push(String::from(slice_str));
                            SmallVec::from(&slice_vec[..])
                        },
                        5 => {
                            let iter_size = _to_usize(GLOBAL_DATA, base_offset + 2) % 8;
                            let iter_vec: Vec<String> = (0..iter_size).map(|_| String::from("iter")).collect();
                            SmallVec::from_iter(iter_vec)
                        },
                        6 => {
                            let arr_size = [String::from("array1"), String::from("array2"), String::from("array3"), 
                                          String::from("array4"), String::from("array5"), String::from("array6"),
                                          String::from("array7"), String::from("array8"), String::from("array9"),
                                          String::from("array10"), String::from("array11"), String::from("array12"),
                                          String::from("array13"), String::from("array14"), String::from("array15"),
                                          String::from("array16")];
                            SmallVec::from(arr_size)
                        },
                        _ => {
                            let vec_elements = vec![String::from("from_vec")];
                            SmallVec::from(vec_elements)
                        }
                    };
                    
                    let push_count = _to_usize(GLOBAL_DATA, base_offset + 4) % 5;
                    for j in 0..push_count {
                        let push_str_len = _to_u8(GLOBAL_DATA, (base_offset + 5 + j) % GLOBAL_DATA.len()) % 10 + 1;
                        let push_start = (base_offset + 6 + j * 2) % (GLOBAL_DATA.len() - push_str_len as usize);
                        let push_end = push_start + push_str_len as usize;
                        let push_str = _to_str(GLOBAL_DATA, push_start, push_end);
                        smallvec.push(String::from(push_str));
                    }
                    
                    if !smallvec.is_empty() {
                        let remove_index = _to_usize(GLOBAL_DATA, base_offset + 3) % smallvec.len();
                        let removed_item = smallvec.remove(remove_index);
                        println!("{:?}", removed_item);
                    }
                },
                1 => {
                    let mut smallvec: SmallVec<[u32; 32]> = SmallVec::new();
                    let element_count = _to_usize(GLOBAL_DATA, base_offset + 1) % 20;
                    
                    for j in 0..element_count {
                        let value = _to_u32(GLOBAL_DATA, (base_offset + 2 + j * 4) % (GLOBAL_DATA.len() - 4));
                        smallvec.push(value);
                    }
                    
                    if !smallvec.is_empty() {
                        let index = _to_usize(GLOBAL_DATA, base_offset + 2) % smallvec.len();
                        let removed = smallvec.remove(index);
                        println!("{}", removed);
                        
                        let slice_ref = smallvec.as_slice();
                        println!("{:?}", slice_ref);
                    }
                },
                2 => {
                    let mut smallvec1: SmallVec<[i64; 12]> = SmallVec::new();
                    let mut smallvec2: SmallVec<[i64; 12]> = SmallVec::new();
                    
                    let count1 = _to_usize(GLOBAL_DATA, base_offset + 1) % 5;
                    let count2 = _to_usize(GLOBAL_DATA, base_offset + 2) % 5;
                    
                    for j in 0..count1 {
                        let val = _to_i64(GLOBAL_DATA, (base_offset + 3 + j * 8) % (GLOBAL_DATA.len() - 8));
                        smallvec1.push(val);
                    }
                    
                    for j in 0..count2 {
                        let val = _to_i64(GLOBAL_DATA, (base_offset + 4 + j * 8) % (GLOBAL_DATA.len() - 8));
                        smallvec2.push(val);
                    }
                    
                    if !smallvec1.is_empty() {
                        let index = _to_usize(GLOBAL_DATA, base_offset + 3) % smallvec1.len();
                        let removed = smallvec1.remove(index);
                        println!("{}", removed);
                    }
                    
                    smallvec1.append(&mut smallvec2);
                    println!("{}", smallvec1.len());
                },
                3 => {
                    let mut smallvec: SmallVec<[f64; 16]> = SmallVec::with_capacity(_to_usize(GLOBAL_DATA, base_offset + 1));
                    
                    let insert_count = _to_usize(GLOBAL_DATA, base_offset + 2) % 10;
                    for j in 0..insert_count {
                        let val = _to_f64(GLOBAL_DATA, (base_offset + 3 + j * 8) % (GLOBAL_DATA.len() - 8));
                        let insert_idx = _to_usize(GLOBAL_DATA, base_offset + 4 + j) % (smallvec.len() + 1);
                        smallvec.insert(insert_idx, val);
                    }
                    
                    if !smallvec.is_empty() {
                        let remove_idx = _to_usize(GLOBAL_DATA, base_offset + 5) % smallvec.len();
                        let removed = smallvec.remove(remove_idx);
                        println!("{}", removed);
                        
                        let mut_slice_ref = smallvec.as_mut_slice();
                        println!("{:?}", mut_slice_ref);
                    }
                },
                4 => {
                    let mut smallvec: SmallVec<[char; 24]> = SmallVec::new();
                    
                    let char_count = _to_usize(GLOBAL_DATA, base_offset + 1) % 15;
                    for j in 0..char_count {
                        let ch = _to_char(GLOBAL_DATA, (base_offset + 2 + j * 4) % (GLOBAL_DATA.len() - 4));
                        smallvec.push(ch);
                    }
                    
                    if !smallvec.is_empty() {
                        let swap_remove_idx = _to_usize(GLOBAL_DATA, base_offset + 3) % smallvec.len();
                        let swapped = smallvec.swap_remove(swap_remove_idx);
                        println!("{}", swapped);
                        
                        if !smallvec.is_empty() {
                            let remove_idx = _to_usize(GLOBAL_DATA, base_offset + 4) % smallvec.len();
                            let removed = smallvec.remove(remove_idx);
                            println!("{}", removed);
                        }
                    }
                },
                5 => {
                    let mut smallvec: SmallVec<[bool; 64]> = SmallVec::from_elem(true, _to_usize(GLOBAL_DATA, base_offset + 1) % 30);
                    
                    let bool_count = _to_usize(GLOBAL_DATA, base_offset + 2) % 10;
                    for j in 0..bool_count {
                        let b = _to_bool(GLOBAL_DATA, (base_offset + 3 + j) % GLOBAL_DATA.len());
                        smallvec.push(b);
                    }
                    
                    if !smallvec.is_empty() {
                        let remove_idx = _to_usize(GLOBAL_DATA, base_offset + 4) % smallvec.len();
                        let removed = smallvec.remove(remove_idx);
                        println!("{}", removed);
                        
                        smallvec.truncate(_to_usize(GLOBAL_DATA, base_offset + 5));
                        let cap = smallvec.capacity();
                        println!("{}", cap);
                    }
                },
                6 => {
                    let mut smallvec: SmallVec<[u8; 128]> = SmallVec::new();
                    
                    let byte_count = _to_usize(GLOBAL_DATA, base_offset + 1) % 65;
                    for j in 0..byte_count {
                        let byte_val = _to_u8(GLOBAL_DATA, (base_offset + 2 + j) % GLOBAL_DATA.len());
                        smallvec.push(byte_val);
                    }
                    
                    if !smallvec.is_empty() {
                        let remove_idx = _to_usize(GLOBAL_DATA, base_offset + 3) % smallvec.len();
                        let removed = smallvec.remove(remove_idx);
                        println!("{}", removed);
                        
                        let cloned = smallvec.clone();
                        println!("{}", cloned.len());
                        
                        let max_drain_start = if smallvec.len() > 0 { smallvec.len() - 1 } else { 0 };
                        let drain_start = _to_usize(GLOBAL_DATA, base_offset + 4) % (max_drain_start + 1);
                        let drain_end = drain_start + (_to_usize(GLOBAL_DATA, base_offset + 5) % (smallvec.len() - drain_start + 1));
                        let drained: Vec<_> = smallvec.drain(drain_start..drain_end).collect();
                        println!("{:?}", drained);
                    }
                },
                7 => {
                    let vec_data: Vec<usize> = (0..10).collect();
                    let mut smallvec: SmallVec<[usize; 20]> = SmallVec::from_vec(vec_data);
                    
                    let extend_count = _to_usize(GLOBAL_DATA, base_offset + 1) % 8;
                    let extend_vec: Vec<usize> = (0..extend_count).collect();
                    smallvec.extend(extend_vec);
                    
                    if !smallvec.is_empty() {
                        let remove_idx = _to_usize(GLOBAL_DATA, base_offset + 2) % smallvec.len();
                        let removed = smallvec.remove(remove_idx);
                        println!("{}", removed);
                        
                        let pop_result = smallvec.pop();
                        if let Some(popped) = pop_result {
                            println!("{}", popped);
                        }
                    }
                },
                8 => {
                    let mut smallvec: SmallVec<[u16; 32]> = SmallVec::new();
                    
                    let resize_len = _to_usize(GLOBAL_DATA, base_offset + 1) % 25;
                    let resize_val = _to_u16(GLOBAL_DATA, (base_offset + 2) % (GLOBAL_DATA.len() - 2));
                    smallvec.resize(resize_len, resize_val);
                    
                    if !smallvec.is_empty() {
                        let remove_idx = _to_usize(GLOBAL_DATA, base_offset + 3) % smallvec.len();
                        let removed = smallvec.remove(remove_idx);
                        println!("{}", removed);
                        
                        smallvec.reserve(_to_usize(GLOBAL_DATA, base_offset + 4));
                        smallvec.shrink_to_fit();
                        println!("{}", smallvec.capacity());
                    }
                },
                9 => {
                    let mut smallvec: SmallVec<[i32; 12]> = SmallVec::new();
                    
                    let elem_count = _to_usize(GLOBAL_DATA, base_offset + 1) % 8;
                    for j in 0..elem_count {
                        let val = _to_i32(GLOBAL_DATA, (base_offset + 2 + j * 4) % (GLOBAL_DATA.len() - 4));
                        smallvec.push(val);
                    }
                    
                    if !smallvec.is_empty() {
                        let remove_idx = _to_usize(GLOBAL_DATA, base_offset + 3) % smallvec.len();
                        let removed = smallvec.remove(remove_idx);
                        println!("{}", removed);
                        
                        let hash_val = {
                            let mut hasher = DefaultHasher::new();
                            smallvec.hash(&mut hasher);
                            hasher.finish()
                        };
                        println!("{}", hash_val);
                    }
                },
                10 => {
                    let mut smallvec1: SmallVec<[u64; 16]> = SmallVec::from_elem(42u64, _to_usize(GLOBAL_DATA, base_offset + 1) % 10);
                    let mut smallvec2: SmallVec<[u64; 16]> = SmallVec::from_elem(99u64, _to_usize(GLOBAL_DATA, base_offset + 2) % 10);
                    
                    if !smallvec1.is_empty() {
                        let remove_idx = _to_usize(GLOBAL_DATA, base_offset + 3) % smallvec1.len();
                        let removed = smallvec1.remove(remove_idx);
                        println!("{}", removed);
                    }
                    
                    let cmp_result = smallvec1.cmp(&smallvec2);
                    println!("{:?}", cmp_result);
                    
                    let partial_cmp_result = smallvec1.partial_cmp(&smallvec2);
                    if let Some(ord) = partial_cmp_result {
                        println!("{:?}", ord);
                    }
                },
                11 => {
                    let mut smallvec: SmallVec<[isize; 12]> = SmallVec::new();
                    
                    let data_count = _to_usize(GLOBAL_DATA, base_offset + 1) % 6;
                    for j in 0..data_count {
                        let val = _to_isize(GLOBAL_DATA, (base_offset + 2 + j * 8) % (GLOBAL_DATA.len() - 8));
                        smallvec.push(val);
                    }
                    
                    if !smallvec.is_empty() {
                        let remove_idx = _to_usize(GLOBAL_DATA, base_offset + 3) % smallvec.len();
                        let removed = smallvec.remove(remove_idx);
                        println!("{}", removed);
                        
                        smallvec.clear();
                        println!("{}", smallvec.is_empty());
                    }
                },
                12 => {
                    let mut smallvec: SmallVec<[f32; 16]> = SmallVec::with_capacity(_to_usize(GLOBAL_DATA, base_offset + 1));
                    
                    let val_count = _to_usize(GLOBAL_DATA, base_offset + 2) % 12;
                    for j in 0..val_count {
                        let val = _to_f32(GLOBAL_DATA, (base_offset + 3 + j * 4) % (GLOBAL_DATA.len() - 4));
                        smallvec.push(val);
                    }
                    
                    if !smallvec.is_empty() {
                        let deref_slice = &*smallvec;
                        println!("{:?}", deref_slice);
                        
                        let deref_mut_slice = &mut *smallvec;
                        println!("{:?}", deref_mut_slice);
                        
                        let index_val = _to_usize(GLOBAL_DATA, base_offset + 4) % smallvec.len();
                        let indexed_ref = &smallvec[index_val];
                        println!("{}", indexed_ref);
                        
                        let indexed_mut_ref = &mut smallvec[index_val];
                        println!("{}", indexed_mut_ref);
                    }
                },
                13 => {
                    let slice_vec: Vec<i16> = vec![1, 2, 3, 4, 5];
                    let smallvec: SmallVec<[i16; 12]> = slice_vec.as_slice().to_smallvec();
                    
                    if !smallvec.is_empty() {
                        let as_ref_slice = smallvec.as_ref();
                        println!("{:?}", as_ref_slice);
                        
                        let borrow_slice = std::borrow::Borrow::<[i16]>::borrow(&smallvec);
                        println!("{:?}", borrow_slice);
                        
                        let into_vec = smallvec.into_vec();
                        println!("{:?}", into_vec);
                    }
                },
                _ => {
                    let constructor_choice = _to_u8(GLOBAL_DATA, base_offset + 1) % 4;
                    let mut smallvec: SmallVec<[u128; 12]> = match constructor_choice {
                        0 => SmallVec::new(),
                        1 => SmallVec::with_capacity(_to_usize(GLOBAL_DATA, base_offset + 2)),
                        2 => SmallVec::from_elem(42u128, _to_usize(GLOBAL_DATA, base_offset + 2) % 8),
                        _ => {
                            let vec_data: Vec<u128> = (0..5).map(|x| x as u128).collect();
                            SmallVec::from_vec(vec_data)
                        }
                    };
                    
                    let insert_count = _to_usize(GLOBAL_DATA, base_offset + 3) % 6;
                    for j in 0..insert_count {
                        let val = _to_u128(GLOBAL_DATA, (base_offset + 4 + j * 16) % (GLOBAL_DATA.len() - 16));
                        let insert_idx = _to_usize(GLOBAL_DATA, base_offset + 5 + j) % (smallvec.len() + 1);
                        smallvec.insert(insert_idx, val);
                    }
                    
                    if !smallvec.is_empty() {
                        let remove_idx = _to_usize(GLOBAL_DATA, base_offset + 6) % smallvec.len();
                        let removed = smallvec.remove(remove_idx);
                        println!("{}", removed);
                        
                        let spilled = smallvec.spilled();
                        println!("{}", spilled);
                        
                        let inline_size = smallvec.inline_size();
                        println!("{}", inline_size);
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