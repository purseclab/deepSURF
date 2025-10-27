#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};
use std::borrow::Borrow;

struct CustomType1(String);

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 120 {return;}
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        
        let num_operations = _to_u8(GLOBAL_DATA, 0) % 65;
        
        for op_idx in 0..num_operations {
            let base_offset = 1 + (op_idx as usize * 3);
            if base_offset + 2 >= GLOBAL_DATA.len() { break; }
            
            let constructor_choice = _to_u8(GLOBAL_DATA, base_offset) % 8;
            let operation_choice = _to_u8(GLOBAL_DATA, base_offset + 1);
            let param1 = _to_u8(GLOBAL_DATA, base_offset + 2);
            
            let mut smallvec = match constructor_choice {
                0 => SmallVec::<[u32; 12]>::new(),
                1 => {
                    let capacity = _to_usize(GLOBAL_DATA, base_offset);
                    SmallVec::<[u32; 12]>::with_capacity(capacity)
                },
                2 => {
                    let vec_size = param1 % 65;
                    let mut vec = Vec::new();
                    for i in 0..vec_size {
                        vec.push(i as u32);
                    }
                    SmallVec::<[u32; 12]>::from_vec(vec)
                },
                3 => {
                    let elem = _to_u32(GLOBAL_DATA, base_offset);
                    let count = param1 as usize % 65;
                    SmallVec::<[u32; 12]>::from_elem(elem, count)
                },
                4 => {
                    let arr = [1u32, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
                    SmallVec::<[u32; 12]>::from_buf(arr)
                },
                5 => {
                    let arr = [42u32; 12];
                    let len = param1 as usize % 13;
                    SmallVec::<[u32; 12]>::from_buf_and_len(arr, len)
                },
                6 => {
                    let slice_size = param1 % 20;
                    let slice: Vec<u32> = (0..slice_size).map(|x| x as u32).collect();
                    SmallVec::<[u32; 12]>::from_slice(&slice)
                },
                _ => {
                    let iter = (0..(param1 % 15)).map(|x| x as u32);
                    SmallVec::<[u32; 12]>::from_iter(iter)
                }
            };
            
            let is_empty_result = smallvec.is_empty();
            println!("{}", is_empty_result);
            
            match operation_choice % 25 {
                0 => {
                    let value = _to_u32(GLOBAL_DATA, base_offset);
                    smallvec.push(value);
                    let ref_result = &smallvec;
                    let slice_ref = ref_result.as_slice();
                    println!("{:?}", slice_ref);
                },
                1 => {
                    if let Some(popped) = smallvec.pop() {
                        println!("{}", popped);
                    }
                },
                2 => {
                    let capacity = smallvec.capacity();
                    println!("{}", capacity);
                },
                3 => {
                    let len = smallvec.len();
                    println!("{}", len);
                },
                4 => {
                    let slice_ref = smallvec.as_slice();
                    println!("{:?}", slice_ref);
                },
                5 => {
                    let ptr = smallvec.as_ptr();
                    println!("{:?}", ptr);
                },
                6 => {
                    let spilled = smallvec.spilled();
                    println!("{}", spilled);
                },
                7 => {
                    let index = param1 as usize;
                    if index < smallvec.len() {
                        let indexed_ref = &smallvec[index];
                        println!("{}", *indexed_ref);
                    }
                },
                8 => {
                    let additional = _to_usize(GLOBAL_DATA, base_offset);
                    smallvec.reserve(additional);
                },
                9 => {
                    let new_cap = _to_usize(GLOBAL_DATA, base_offset);
                    smallvec.grow(new_cap);
                },
                10 => {
                    smallvec.clear();
                },
                11 => {
                    let len = _to_usize(GLOBAL_DATA, base_offset);
                    smallvec.truncate(len);
                },
                12 => {
                    smallvec.shrink_to_fit();
                },
                13 => {
                    let start = param1 as usize % 10;
                    let end = (start + 5) % 20;
                    let drained: Vec<_> = smallvec.drain(start..end).collect();
                    for item in drained {
                        println!("{}", item);
                    }
                },
                14 => {
                    let cloned_vec = smallvec.clone();
                    let vec = cloned_vec.into_vec();
                    let rebuilt = SmallVec::<[u32; 12]>::from_vec(vec);
                    let slice_ref = rebuilt.as_slice();
                    println!("{:?}", slice_ref);
                },
                15 => {
                    let cloned = smallvec.clone();
                    let slice_ref = cloned.as_slice();
                    println!("{:?}", slice_ref);
                },
                16 => {
                    let mut other = SmallVec::<[u32; 12]>::new();
                    other.push(999);
                    smallvec.append(&mut other);
                },
                17 => {
                    let index = param1 as usize;
                    let value = _to_u32(GLOBAL_DATA, base_offset);
                    if index <= smallvec.len() {
                        smallvec.insert(index, value);
                    }
                },
                18 => {
                    if !smallvec.is_empty() {
                        let index = param1 as usize % smallvec.len();
                        let removed = smallvec.remove(index);
                        println!("{}", removed);
                    }
                },
                19 => {
                    let new_len = param1 as usize % 30;
                    let fill_value = _to_u32(GLOBAL_DATA, base_offset);
                    smallvec.resize(new_len, fill_value);
                },
                20 => {
                    let slice_vals = [100u32, 200, 300];
                    smallvec.extend_from_slice(&slice_vals);
                },
                21 => {
                    smallvec.dedup();
                },
                22 => {
                    let additional = _to_usize(GLOBAL_DATA, base_offset);
                    if let Ok(_) = smallvec.try_reserve(additional) {
                        println!("Reserve successful");
                    }
                },
                23 => {
                    let mut_slice = smallvec.as_mut_slice();
                    if !mut_slice.is_empty() {
                        mut_slice[0] = 42;
                    }
                },
                _ => {
                    let index = param1 as usize;
                    let value = _to_u32(GLOBAL_DATA, base_offset);
                    if index <= smallvec.len() {
                        smallvec.insert(index, value);
                    }
                }
            }
            
            let final_check = smallvec.is_empty();
            println!("{}", final_check);
            
            let another_smallvec = SmallVec::<[i32; 16]>::new();
            let is_empty_2 = another_smallvec.is_empty();
            println!("{}", is_empty_2);
            
            let comparison_vec = SmallVec::<[u32; 12]>::new();
            let eq_result = smallvec.eq(&comparison_vec);
            println!("{}", eq_result);
            
            if smallvec.len() > 0 && comparison_vec.len() == 0 {
                let partial_cmp = smallvec.partial_cmp(&comparison_vec);
                if let Some(ordering) = partial_cmp {
                    println!("{:?}", ordering);
                }
            }
            
            smallvec.reserve_exact(_to_usize(GLOBAL_DATA, base_offset));
            let iter_result = smallvec.into_iter();
            let iter_slice = iter_result.as_slice();
            println!("{:?}", iter_slice);
            
            let slice_result = SmallVec::<[u32; 12]>::from_slice(&[1, 2, 3]);
            let slice_ptr = slice_result.as_ptr();
            println!("{:?}", slice_ptr);
            
            let from_vec_example = SmallVec::<[u32; 12]>::from_vec(vec![42, 43, 44]);
            let borrow_slice: &[u32] = from_vec_example.borrow();
            println!("{:?}", borrow_slice);
            
            let mut as_ref_example = SmallVec::<[u32; 12]>::new();
            as_ref_example.push(101);
            let ref_slice = as_ref_example.as_ref();
            for val in ref_slice {
                println!("{}", *val);
            }
            
            let mut deref_example = SmallVec::<[u32; 12]>::new();
            deref_example.push(200);
            let deref_slice = deref_example.deref();
            for val in deref_slice {
                println!("{}", *val);
            }
            
            let mut extend_example = SmallVec::<[u32; 12]>::new();
            extend_example.extend([100u32, 200, 300].iter().cloned());
            
            let final_vec = SmallVec::<[u8; 24]>::with_capacity(_to_usize(GLOBAL_DATA, base_offset + 1));
            let final_hash = final_vec.len() + final_vec.capacity();
            println!("{}", final_hash);
            
            let raw_parts_test = SmallVec::<[u8; 16]>::new();
            let raw_ptr = raw_parts_test.as_ptr();
            println!("{:?}", raw_ptr);
            
            let mut test_resize = SmallVec::<[u32; 12]>::new();
            test_resize.resize_with(5, || 42);
            let resize_len = test_resize.len();
            println!("{}", resize_len);
            
            let mut test_retain = SmallVec::<[u32; 12]>::new();
            test_retain.extend([1u32, 2, 3, 4, 5].iter().cloned());
            test_retain.retain(|&mut x| x % 2 == 0);
            let retain_len = test_retain.len();
            println!("{}", retain_len);
            
            let test_drain = SmallVec::<[i16; 20]>::from_slice(&[10, 20, 30]);
            let drain_capacity = test_drain.capacity();
            println!("{}", drain_capacity);
            
            let test_swap_remove = SmallVec::<[f32; 8]>::from_elem(1.5, 4);
            let swap_len = test_swap_remove.len();
            println!("{}", swap_len);
            
            let test_inline_size = SmallVec::<[i64; 6]>::new().inline_size();
            println!("{}", test_inline_size);
            
            let test_into_boxed = SmallVec::<[u16; 14]>::from_slice(&[100, 200]);
            let boxed_slice = test_into_boxed.into_boxed_slice();
            for val in boxed_slice.iter() {
                println!("{}", *val);
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