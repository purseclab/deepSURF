#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};
use std::borrow::Borrow;

#[derive(Debug, Clone)]
struct CustomType1(String);

fn main (){
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 500 {return;}
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        
        let num_operations = _to_u8(GLOBAL_DATA, 0) % 65;
        
        for op_index in 0..num_operations {
            let base_offset = (op_index as usize * 8) + 1;
            if base_offset + 8 > GLOBAL_DATA.len() { break; }
            
            let operation_type = _to_u8(GLOBAL_DATA, base_offset) % 8;
            
            match operation_type {
                0 => {
                    let arr_size = (_to_u8(GLOBAL_DATA, base_offset + 1) % 64) + 1;
                    let len = _to_usize(GLOBAL_DATA, base_offset + 2);
                    let constructor_choice = _to_u8(GLOBAL_DATA, base_offset + 6) % 4;
                    
                    let mut sv = match constructor_choice {
                        0 => SmallVec::<[CustomType1; 64]>::new(),
                        1 => SmallVec::<[CustomType1; 64]>::with_capacity(_to_usize(GLOBAL_DATA, base_offset + 7)),
                        2 => {
                            let vec_len = _to_u8(GLOBAL_DATA, base_offset + 3) % 5;
                            let mut vec = Vec::new();
                            for i in 0..vec_len {
                                vec.push(CustomType1(format!("vec_{}", i)));
                            }
                            SmallVec::<[CustomType1; 64]>::from_vec(vec)
                        },
                        _ => {
                            let iter_items = vec![CustomType1("iter1".to_string()), CustomType1("iter2".to_string())];
                            SmallVec::<[CustomType1; 64]>::from_iter(iter_items)
                        }
                    };
                    
                    let item = CustomType1(format!("push_{}", op_index));
                    sv.push(item);
                    
                    let slice_ref = sv.as_slice();
                    println!("{:?}", *slice_ref.get(0).unwrap_or(&CustomType1("empty".to_string())));
                    
                    let mut_slice_ref = sv.as_mut_slice();
                    if !mut_slice_ref.is_empty() {
                        println!("{:?}", *mut_slice_ref.get_mut(0).unwrap());
                    }
                    
                    let len_result = sv.len();
                    println!("{}", len_result);
                    
                    let capacity_result = sv.capacity();
                    println!("{}", capacity_result);
                    
                    let is_empty_result = sv.is_empty();
                    println!("{}", is_empty_result);
                    
                    let spilled_result = sv.spilled();
                    println!("{}", spilled_result);
                },
                1 => {
                    let mut sv1 = SmallVec::<[String; 32]>::new();
                    let mut sv2 = SmallVec::<[String; 32]>::new();
                    
                    sv1.push("item1".to_string());
                    sv2.push("item2".to_string());
                    
                    sv1.push("target_push".to_string());
                    
                    let deref_result = &*sv1;
                    println!("{:?}", *deref_result.get(0).unwrap_or(&"empty".to_string()));
                    
                    let mut_deref_result = &mut *sv1;
                    if !mut_deref_result.is_empty() {
                        println!("{:?}", *mut_deref_result.get_mut(0).unwrap());
                    }
                    
                    sv1.append(&mut sv2);
                    
                    if let Some(popped) = sv1.pop() {
                        println!("{:?}", popped);
                    }
                },
                2 => {
                    let mut sv = SmallVec::<[i32; 16]>::new();
                    sv.push(1);
                    sv.push(2);
                    sv.push(3);
                    
                    let target_item = _to_i32(GLOBAL_DATA, base_offset + 1);
                    sv.push(target_item);
                    
                    sv.reserve(_to_usize(GLOBAL_DATA, base_offset + 2));
                    
                    sv.reserve_exact(_to_usize(GLOBAL_DATA, base_offset + 3));
                    
                    sv.grow(_to_usize(GLOBAL_DATA, base_offset + 4));
                    
                    let index = _to_usize(GLOBAL_DATA, base_offset + 5);
                    sv.insert(index, 999);
                    
                    let truncate_len = _to_usize(GLOBAL_DATA, base_offset + 6);
                    sv.truncate(truncate_len);
                    
                    sv.shrink_to_fit();
                    
                    sv.clear();
                },
                3 => {
                    let mut sv = SmallVec::<[f64; 20]>::new();
                    
                    for i in 0..5 {
                        sv.push(i as f64);
                    }
                    
                    let target_val = _to_f64(GLOBAL_DATA, base_offset + 1);
                    sv.push(target_val);
                    
                    let remove_index = _to_usize(GLOBAL_DATA, base_offset + 2);
                    if !sv.is_empty() {
                        let removed = sv.swap_remove(remove_index % sv.len());
                        println!("{}", removed);
                    }
                    
                    let insert_index = _to_usize(GLOBAL_DATA, base_offset + 3);
                    let insert_val = _to_f64(GLOBAL_DATA, base_offset + 4);
                    sv.insert(insert_index % (sv.len() + 1), insert_val);
                    
                    let extend_slice = [1.1, 2.2, 3.3];
                    sv.extend_from_slice(&extend_slice);
                    
                    let drain_start = _to_usize(GLOBAL_DATA, base_offset + 5);
                    let drain_end = _to_usize(GLOBAL_DATA, base_offset + 6);
                    if !sv.is_empty() {
                        let drain_range = (drain_start % sv.len())..=(drain_end % sv.len()).min(sv.len() - 1);
                        let mut drained = sv.drain(drain_range);
                        if let Some(first_drained) = drained.next() {
                            println!("{}", first_drained);
                        }
                        if let Some(last_drained) = drained.next_back() {
                            println!("{}", last_drained);
                        }
                    }
                },
                4 => {
                    let sv1 = SmallVec::<[u8; 12]>::from_slice(&[1, 2, 3]);
                    let sv2 = SmallVec::<[u8; 12]>::from_slice(&[1, 2, 4]);
                    
                    let mut sv_target = SmallVec::<[u8; 12]>::new();
                    sv_target.push(99);
                    
                    let eq_result = sv1.eq(&sv2);
                    println!("{}", eq_result);
                    
                    let cmp_result = sv1.cmp(&sv2);
                    println!("{:?}", cmp_result);
                    
                    let partial_cmp_result = sv1.partial_cmp(&sv2);
                    if let Some(ordering) = partial_cmp_result {
                        println!("{:?}", ordering);
                    }
                    
                    let cloned = sv1.clone();
                    println!("{}", cloned.len());
                    
                    let as_ref_result = sv1.as_ref();
                    println!("{:?}", *as_ref_result.get(0).unwrap_or(&0));
                    
                    let borrowed: &[u8] = sv1.borrow();
                    println!("{:?}", *borrowed.get(0).unwrap_or(&0));
                },
                5 => {
                    let mut sv = SmallVec::<[char; 12]>::new();
                    
                    let chars = ['a', 'b', 'c', 'd'];
                    for ch in chars {
                        sv.push(ch);
                    }
                    
                    let target_char = _to_char(GLOBAL_DATA, base_offset + 1);
                    sv.push(target_char);
                    
                    sv.dedup();
                    
                    let resize_len = _to_usize(GLOBAL_DATA, base_offset + 2);
                    let resize_val = _to_char(GLOBAL_DATA, base_offset + 3);
                    sv.resize(resize_len, resize_val);
                    
                    sv.retain(|c| *c != 'x');
                    
                    let into_vec = sv.into_vec();
                    println!("{}", into_vec.len());
                },
                6 => {
                    let mut sv = SmallVec::<[bool; 24]>::new();
                    
                    for i in 0..3 {
                        let bool_val = _to_bool(GLOBAL_DATA, base_offset + i + 1);
                        sv.push(bool_val);
                    }
                    
                    let target_bool = _to_bool(GLOBAL_DATA, base_offset + 4);
                    sv.push(target_bool);
                    
                    let as_ptr_result = sv.as_ptr();
                    println!("{:p}", as_ptr_result);
                    
                    let as_mut_ptr_result = sv.as_mut_ptr();
                    println!("{:p}", as_mut_ptr_result);
                    
                    let into_iter = sv.into_iter();
                    let collected: Vec<_> = into_iter.collect();
                    println!("{}", collected.len());
                },
                _ => {
                    let mut sv = SmallVec::<[usize; 32]>::new();
                    for i in 0..10 {
                        sv.push(i);
                    }
                    
                    let target_val = _to_usize(GLOBAL_DATA, base_offset + 1);
                    sv.push(target_val);
                    
                    let index_val = _to_usize(GLOBAL_DATA, base_offset + 2);
                    if !sv.is_empty() {
                        let sv_len = sv.len();
                        let indexed = &sv[index_val % sv_len];
                        println!("{}", *indexed);
                        
                        let indexed_mut = &mut sv[index_val % sv_len];
                        println!("{}", *indexed_mut);
                    }
                    
                    let into_boxed = sv.into_boxed_slice();
                    println!("{}", into_boxed.len());
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