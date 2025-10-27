#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::ops::{Deref, DerefMut, Index, IndexMut};

#[derive(Debug, Clone, Copy)]
struct CustomType1(usize);

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 2048 {return;}
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let constructor_selector = _to_u8(GLOBAL_DATA, 0) % 4;
        let mut smallvec = match constructor_selector {
            0 => SmallVec::<[CustomType1; 32]>::new(),
            1 => {
                let capacity = _to_usize(GLOBAL_DATA, 1);
                SmallVec::with_capacity(capacity)
            },
            2 => {
                let elem_count = _to_usize(GLOBAL_DATA, 2) % 64;
                let elem_value = CustomType1(_to_usize(GLOBAL_DATA, 3));
                SmallVec::from_elem(elem_value, elem_count)
            },
            _ => {
                let slice_len = _to_usize(GLOBAL_DATA, 4) % 64;
                let mut temp_vec = Vec::with_capacity(slice_len);
                for i in 0..slice_len {
                    temp_vec.push(CustomType1(_to_usize(GLOBAL_DATA, 5 + i*2)));
                }
                SmallVec::from_slice(&temp_vec)
            }
        };

        let operations = _to_u8(GLOBAL_DATA, 128) % 16;
        for op_index in 0..operations {
            let op_selector = _to_u8(GLOBAL_DATA, 129 + op_index as usize) % 7;
            
            match op_selector {
                0 => {
                    let val = CustomType1(_to_usize(GLOBAL_DATA, 200 + op_index as usize * 4));
                    smallvec.push(val);
                    println!("Pushed: {:?}", smallvec.last().unwrap());
                },
                1 => {
                    let popped = smallvec.pop();
                    if let Some(p) = popped {
                        println!("Popped: {:?}", p);
                    }
                },
                2 => {
                    if !smallvec.is_empty() {
                        let idx = _to_usize(GLOBAL_DATA, 300) % smallvec.len();
                        println!("Indexed: {:?}", *smallvec.index(idx));
                    }
                },
                3 => {
                    let new_cap = _to_usize(GLOBAL_DATA, 400);
                    smallvec.reserve(new_cap);
                    println!("Reserved capacity: {}", smallvec.capacity());
                },
                4 => {
                    let target_len = _to_usize(GLOBAL_DATA, 500) % 128;
                    smallvec.truncate(target_len);
                    println!("Truncated to {} elements", smallvec.len());
                },
                5 => {
                    let insert_idx = _to_usize(GLOBAL_DATA, 600) % (smallvec.len() + 1);
                    let insert_val = CustomType1(_to_usize(GLOBAL_DATA, 601));
                    smallvec.insert(insert_idx, insert_val);
                    println!("Inserted at {}: {:?}", insert_idx, smallvec[insert_idx]);
                },
                _ => {
                    let slice_len = _to_usize(GLOBAL_DATA, 700) % 64;
                    let mut ext_slice = Vec::with_capacity(slice_len);
                    for i in 0..slice_len {
                        ext_slice.push(CustomType1(_to_usize(GLOBAL_DATA, 701 + i*2)));
                    }
                    smallvec.extend_from_slice(&ext_slice);
                    println!("Extended with {} elements", slice_len);
                }
            }
        }

        if !smallvec.is_empty() {
            println!("Final vector: {:?}", smallvec.as_slice());
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