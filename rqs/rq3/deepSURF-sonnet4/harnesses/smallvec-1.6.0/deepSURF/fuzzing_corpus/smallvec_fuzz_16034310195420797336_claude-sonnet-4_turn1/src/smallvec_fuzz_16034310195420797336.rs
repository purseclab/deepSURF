#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

struct CustomType1(String);

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 120 {return;}
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        
        let num_operations = _to_u8(GLOBAL_DATA, 0) % 65;
        
        for i in 0..num_operations {
            let base_idx = i as usize * 4;
            if base_idx + 4 >= GLOBAL_DATA.len() { break; }
            
            let operation = _to_u8(GLOBAL_DATA, base_idx) % 8;
            
            match operation {
                0 => {
                    let capacity = _to_usize(GLOBAL_DATA, base_idx + 8);
                    let mut sv1 = smallvec::SmallVec::<[i32; 16]>::with_capacity(capacity);
                    let elem = _to_i32(GLOBAL_DATA, base_idx + 16);
                    let count = _to_usize(GLOBAL_DATA, base_idx + 24);
                    let sv2 = smallvec::SmallVec::<[i32; 16]>::from_elem(elem, count);
                    sv1.extend(sv2.iter().cloned());
                    let ptr = sv1.as_mut_ptr();
                    let slice = sv1.as_slice();
                    println!("{:?}", slice);
                }
                1 => {
                    let vec_size = _to_u8(GLOBAL_DATA, base_idx + 1) % 65;
                    let mut vec_data = Vec::new();
                    for j in 0..vec_size {
                        vec_data.push(_to_i32(GLOBAL_DATA, base_idx + 32 + j as usize * 4));
                    }
                    let mut sv = smallvec::SmallVec::<[i32; 16]>::from_vec(vec_data);
                    sv.push(_to_i32(GLOBAL_DATA, base_idx + 4));
                    let ptr = sv.as_mut_ptr();
                    let mut_slice = sv.as_mut_slice();
                    println!("{:?}", mut_slice);
                }
                2 => {
                    let array_size = _to_u8(GLOBAL_DATA, base_idx + 2) % 16;
                    let mut array = [0i32; 16];
                    for k in 0..array_size as usize {
                        if k < 16 {
                            array[k] = _to_i32(GLOBAL_DATA, base_idx + 40 + k * 4);
                        }
                    }
                    let mut sv = smallvec::SmallVec::from_buf(array);
                    let len = _to_usize(GLOBAL_DATA, base_idx + 8);
                    let sv2 = smallvec::SmallVec::from_buf_and_len(array, len);
                    sv.append(&mut sv2.clone());
                    let ptr = sv.as_mut_ptr();
                    let capacity_val = sv.capacity();
                    println!("{}", capacity_val);
                }
                3 => {
                    let slice_size = _to_u8(GLOBAL_DATA, base_idx + 3) % 65;
                    let mut slice_data = Vec::new();
                    for m in 0..slice_size {
                        slice_data.push(_to_i32(GLOBAL_DATA, base_idx + 60 + m as usize * 4));
                    }
                    let mut sv = smallvec::SmallVec::<[i32; 16]>::from_slice(&slice_data);
                    let index = _to_usize(GLOBAL_DATA, base_idx + 12);
                    let element = _to_i32(GLOBAL_DATA, base_idx + 16);
                    sv.insert(index, element);
                    let removed = sv.remove(index);
                    let ptr = sv.as_mut_ptr();
                    println!("{}", removed);
                }
                4 => {
                    let iterator_size = _to_u8(GLOBAL_DATA, base_idx + 4) % 65;
                    let mut iter_data = Vec::new();
                    for n in 0..iterator_size {
                        iter_data.push(_to_i32(GLOBAL_DATA, base_idx + 80 + n as usize * 4));
                    }
                    let mut sv = smallvec::SmallVec::<[i32; 16]>::from_iter(iter_data.into_iter());
                    let reserve_amt = _to_usize(GLOBAL_DATA, base_idx + 20);
                    sv.reserve(reserve_amt);
                    let grow_amt = _to_usize(GLOBAL_DATA, base_idx + 24);
                    sv.grow(grow_amt);
                    let ptr = sv.as_mut_ptr();
                    let len_val = sv.len();
                    println!("{}", len_val);
                }
                5 => {
                    let mut sv1 = smallvec::SmallVec::<[i32; 16]>::new();
                    let mut sv2 = smallvec::SmallVec::<[i32; 16]>::new();
                    sv1.push(_to_i32(GLOBAL_DATA, base_idx + 8));
                    sv2.push(_to_i32(GLOBAL_DATA, base_idx + 12));
                    let ordering = sv1.cmp(&sv2);
                    let partial_ordering = sv1.partial_cmp(&sv2);
                    let ptr1 = sv1.as_mut_ptr();
                    let ptr2 = sv2.as_mut_ptr();
                    println!("{:?}", ordering);
                    if let Some(ord) = partial_ordering {
                        println!("{:?}", ord);
                    }
                }
                6 => {
                    let mut sv = smallvec::SmallVec::<[i32; 16]>::new();
                    for p in 0..10 {
                        sv.push(_to_i32(GLOBAL_DATA, base_idx + 16 + p * 4));
                    }
                    let range_start = _to_usize(GLOBAL_DATA, base_idx + 8);
                    let range_end = _to_usize(GLOBAL_DATA, base_idx + 12);
                    let drain_iter = sv.drain(range_start..range_end);
                    for item in drain_iter {
                        println!("{}", item);
                    }
                    let ptr = sv.as_mut_ptr();
                    let is_empty = sv.is_empty();
                    println!("{}", is_empty);
                }
                7 => {
                    let mut sv = smallvec::SmallVec::<[i32; 16]>::new();
                    sv.push(_to_i32(GLOBAL_DATA, base_idx + 4));
                    sv.push(_to_i32(GLOBAL_DATA, base_idx + 8));
                    sv.push(_to_i32(GLOBAL_DATA, base_idx + 12));
                    let ptr = sv.as_mut_ptr();
                    let pop_result = sv.pop();
                    let swap_idx = _to_usize(GLOBAL_DATA, base_idx + 16);
                    let swap_result = sv.swap_remove(swap_idx);
                    let truncate_len = _to_usize(GLOBAL_DATA, base_idx + 20);
                    sv.truncate(truncate_len);
                    sv.clear();
                    println!("{:?}", pop_result);
                    println!("{}", swap_result);
                }
                _ => {}
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