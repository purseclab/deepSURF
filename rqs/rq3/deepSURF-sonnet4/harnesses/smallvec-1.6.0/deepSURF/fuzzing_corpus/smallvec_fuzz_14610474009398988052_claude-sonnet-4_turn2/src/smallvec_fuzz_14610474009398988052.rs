#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};
use std::cmp::Ordering;

#[derive(Debug)]
struct CustomType1(String);

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 250 {return;}
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        
        let num_operations = _to_u8(GLOBAL_DATA, 0) % 65;
        
        for i in 0..num_operations {
            let base_idx = (i as usize) * 4 + 1;
            if base_idx + 30 >= GLOBAL_DATA.len() { break; }
            
            let constructor_choice = _to_u8(GLOBAL_DATA, base_idx) % 4;
            let operation = _to_u8(GLOBAL_DATA, base_idx + 1) % 16;
            
            match operation {
                0 => {
                    let capacity = _to_usize(GLOBAL_DATA, base_idx + 2);
                    let sv1 = match constructor_choice {
                        0 => smallvec::SmallVec::<[u32; 16]>::new(),
                        1 => smallvec::SmallVec::<[u32; 16]>::with_capacity(capacity),
                        2 => smallvec::SmallVec::<[u32; 16]>::from_vec(vec![_to_u32(GLOBAL_DATA, base_idx + 10)]),
                        _ => {
                            let arr = [_to_u32(GLOBAL_DATA, base_idx + 14); 16];
                            smallvec::SmallVec::<[u32; 16]>::from_buf(arr)
                        }
                    };
                    println!("{:?}", sv1.len());
                    
                    let sv2 = smallvec::SmallVec::<[i32; 32]>::new();
                    println!("{:?}", sv2.is_empty());
                },
                1 => {
                    let capacity1 = _to_usize(GLOBAL_DATA, base_idx + 2);
                    let capacity2 = _to_usize(GLOBAL_DATA, base_idx + 10);
                    
                    let mut sv1 = match constructor_choice % 3 {
                        0 => smallvec::SmallVec::<[u8; 8]>::new(),
                        1 => smallvec::SmallVec::<[u8; 8]>::with_capacity(capacity1),
                        _ => smallvec::SmallVec::<[u8; 8]>::from_slice(&[_to_u8(GLOBAL_DATA, base_idx + 18)])
                    };
                    let mut sv2 = smallvec::SmallVec::<[u8; 16]>::with_capacity(capacity2);
                    
                    let push_val = _to_u8(GLOBAL_DATA, base_idx + 20);
                    sv1.push(push_val);
                    sv2.push(push_val);
                    
                    let slice_ref = sv1.as_slice();
                    println!("{:?}", slice_ref.len());
                    
                    let mut_slice_ref = sv2.as_mut_slice();
                    println!("{:?}", mut_slice_ref.len());
                    
                    sv1.reserve(_to_usize(GLOBAL_DATA, base_idx + 22));
                    sv2.shrink_to_fit();
                },
                2 => {
                    let elem = _to_f32(GLOBAL_DATA, base_idx + 2);
                    let count = _to_usize(GLOBAL_DATA, base_idx + 6);
                    
                    let sv = smallvec::SmallVec::<[f32; 12]>::from_elem(elem, count);
                    let slice_data = sv.deref();
                    println!("{:?}", slice_data.len());
                    
                    let capacity_check = sv.capacity();
                    println!("{:?}", capacity_check);
                    
                    let spilled = sv.spilled();
                    println!("{:?}", spilled);
                },
                3 => {
                    let vec_data = vec![_to_i16(GLOBAL_DATA, base_idx + 2), _to_i16(GLOBAL_DATA, base_idx + 4)];
                    let sv = smallvec::SmallVec::<[i16; 24]>::from_vec(vec_data);
                    
                    let into_vec_result = sv.into_vec();
                    println!("{:?}", into_vec_result.len());
                },
                4 => {
                    let slice_data = &[_to_u64(GLOBAL_DATA, base_idx + 2), _to_u64(GLOBAL_DATA, base_idx + 10)];
                    let sv = smallvec::SmallVec::<[u64; 20]>::from_slice(slice_data);
                    
                    let boxed_slice = sv.into_boxed_slice();
                    println!("{:?}", boxed_slice.len());
                },
                5 => {
                    let iter_data = vec![_to_i8(GLOBAL_DATA, base_idx + 2), _to_i8(GLOBAL_DATA, base_idx + 3)].into_iter();
                    let sv = smallvec::SmallVec::<[i8; 36]>::from_iter(iter_data);
                    
                    let iter_result = sv.into_iter();
                    println!("{:?}", iter_result.len());
                },
                6 => {
                    let mut sv1 = smallvec::SmallVec::<[bool; 10]>::with_capacity(_to_usize(GLOBAL_DATA, base_idx + 2));
                    let mut sv2 = smallvec::SmallVec::<[bool; 10]>::with_capacity(_to_usize(GLOBAL_DATA, base_idx + 10));
                    
                    sv1.push(_to_bool(GLOBAL_DATA, base_idx + 18));
                    sv2.push(_to_bool(GLOBAL_DATA, base_idx + 19));
                    
                    let cmp_result = sv1.cmp(&sv2);
                    println!("{:?}", cmp_result);
                    
                    let eq_result = sv1.eq(&sv2);
                    println!("{:?}", eq_result);
                },
                7 => {
                    let mut sv = smallvec::SmallVec::<[char; 15]>::with_capacity(_to_usize(GLOBAL_DATA, base_idx + 2));
                    sv.push(_to_char(GLOBAL_DATA, base_idx + 10));
                    sv.push(_to_char(GLOBAL_DATA, base_idx + 14));
                    
                    if !sv.is_empty() {
                        let range_start = _to_usize(GLOBAL_DATA, base_idx + 18) % sv.len();
                        let range_end = range_start + (_to_usize(GLOBAL_DATA, base_idx + 22) % (sv.len() - range_start + 1));
                        
                        let drain_iter = sv.drain(range_start..range_end);
                        for item in drain_iter {
                            println!("{:?}", item);
                        }
                    }
                    
                    sv.clear();
                    println!("{:?}", sv.len());
                },
                8 => {
                    let mut sv1 = smallvec::SmallVec::<[isize; 25]>::with_capacity(_to_usize(GLOBAL_DATA, base_idx + 2));
                    let mut sv2 = smallvec::SmallVec::<[isize; 25]>::with_capacity(_to_usize(GLOBAL_DATA, base_idx + 10));
                    
                    sv1.push(_to_isize(GLOBAL_DATA, base_idx + 18));
                    sv2.push(_to_isize(GLOBAL_DATA, base_idx + 20));
                    
                    let partial_cmp_result = sv1.partial_cmp(&sv2);
                    if let Some(ordering) = partial_cmp_result {
                        println!("{:?}", ordering);
                    }
                    
                    sv1.append(&mut sv2);
                    println!("{:?}", sv1.len());
                },
                9 => {
                    let capacity = _to_usize(GLOBAL_DATA, base_idx + 2);
                    let mut sv = smallvec::SmallVec::<[f64; 30]>::with_capacity(capacity);
                    
                    if capacity > 0 {
                        sv.reserve(_to_usize(GLOBAL_DATA, base_idx + 10));
                        sv.push(_to_f64(GLOBAL_DATA, base_idx + 18));
                        
                        let len_check = sv.len();
                        println!("{:?}", len_check);
                        
                        sv.shrink_to_fit();
                        println!("{:?}", sv.capacity());
                        
                        sv.as_ptr();
                        let mut_ptr = sv.as_mut_ptr();
                        println!("{:?}", mut_ptr as usize);
                    }
                },
                10 => {
                    let capacity = _to_usize(GLOBAL_DATA, base_idx + 2);
                    let mut sv = match constructor_choice % 2 {
                        0 => smallvec::SmallVec::<[usize; 32]>::with_capacity(capacity),
                        _ => smallvec::SmallVec::<[usize; 32]>::new()
                    };
                    
                    sv.push(_to_usize(GLOBAL_DATA, base_idx + 10));
                    sv.push(_to_usize(GLOBAL_DATA, base_idx + 18));
                    
                    let truncate_len = _to_usize(GLOBAL_DATA, base_idx + 22);
                    sv.truncate(truncate_len);
                    println!("{:?}", sv.len());
                    
                    let as_ref_slice = sv.as_ref();
                    println!("{:?}", as_ref_slice.len());
                },
                11 => {
                    let capacity = _to_usize(GLOBAL_DATA, base_idx + 2);
                    let mut sv = smallvec::SmallVec::<[String; 12]>::with_capacity(capacity);
                    
                    let s1 = format!("test{}", _to_u32(GLOBAL_DATA, base_idx + 10));
                    let s2 = format!("data{}", _to_u32(GLOBAL_DATA, base_idx + 14));
                    
                    sv.push(s1);
                    sv.push(s2);
                    
                    if !sv.is_empty() {
                        let popped = sv.pop();
                        if let Some(val) = popped {
                            println!("{:?}", val.len());
                        }
                        
                        if !sv.is_empty() {
                            let removed = sv.remove(0);
                            println!("{:?}", removed.len());
                        }
                    }
                },
                12 => {
                    let capacity = _to_usize(GLOBAL_DATA, base_idx + 2);
                    let mut sv = smallvec::SmallVec::<[CustomType1; 18]>::with_capacity(capacity);
                    
                    let custom_val = CustomType1(format!("custom{}", _to_u16(GLOBAL_DATA, base_idx + 10)));
                    sv.push(custom_val);
                    
                    if !sv.is_empty() {
                        let index = _to_usize(GLOBAL_DATA, base_idx + 12) % sv.len();
                        let indexed_ref = &sv[index];
                        println!("{:?}", indexed_ref);
                        
                        let mut_slice = sv.as_mut_slice();
                        println!("{:?}", mut_slice.len());
                    }
                },
                13 => {
                    let capacity = _to_usize(GLOBAL_DATA, base_idx + 2);
                    let mut sv = smallvec::SmallVec::<[i128; 14]>::with_capacity(capacity);
                    
                    let val1 = _to_i128(GLOBAL_DATA, base_idx + 3);
                    sv.push(val1);
                    
                    let insert_index = _to_usize(GLOBAL_DATA, base_idx + 11) % (sv.len() + 1);
                    let insert_val = _to_i128(GLOBAL_DATA, base_idx + 12);
                    sv.insert(insert_index, insert_val);
                    
                    println!("{:?}", sv.len());
                    
                    if sv.len() > 1 {
                        let swap_removed = sv.swap_remove(0);
                        println!("{:?}", swap_removed);
                    }
                },
                14 => {
                    let capacity = _to_usize(GLOBAL_DATA, base_idx + 2);
                    let mut sv = smallvec::SmallVec::<[u128; 22]>::with_capacity(capacity);
                    
                    sv.push(_to_u128(GLOBAL_DATA, base_idx + 10));
                    sv.push(_to_u128(GLOBAL_DATA, base_idx + 18));
                    
                    let slice = &[_to_u128(GLOBAL_DATA, base_idx + 26)];
                    sv.extend_from_slice(slice);
                    
                    println!("{:?}", sv.len());
                    
                    sv.clear();
                    let spilled_check = sv.spilled();
                    println!("{:?}", spilled_check);
                },
                _ => {
                    let capacity = _to_usize(GLOBAL_DATA, base_idx + 2);
                    let mut sv1 = smallvec::SmallVec::<[u8; 32]>::with_capacity(capacity);
                    let mut sv2 = smallvec::SmallVec::<[u8; 32]>::new();
                    
                    sv1.push(_to_u8(GLOBAL_DATA, base_idx + 10));
                    sv2.push(_to_u8(GLOBAL_DATA, base_idx + 11));
                    
                    sv1.extend(&mut sv2.into_iter());
                    println!("{:?}", sv1.len());
                    
                    let cloned_sv = sv1.clone();
                    println!("{:?}", cloned_sv.len());
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