#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct CustomType1(usize);

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 8192 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let op_count = _to_usize(GLOBAL_DATA, 0) % 8 + 2;
        let mut vec1 = SmallVec::<[CustomType1; 32]>::new();
        let mut vec2 = SmallVec::<[CustomType1; 32]>::with_capacity(_to_usize(GLOBAL_DATA, 1));
        let mut vec3 = SmallVec::<[CustomType1; 32]>::from_slice(&[]);

        for i in 0..op_count {
            let op_selector = _to_u8(GLOBAL_DATA, i * 3) % 7;
            let idx = _to_usize(GLOBAL_DATA, i * 3 + 1) % 65;
            let value = CustomType1(_to_usize(GLOBAL_DATA, i * 3 + 2));

            match op_selector {
                0 => {
                    vec1.push(value);
                    vec2.reserve(_to_usize(GLOBAL_DATA, i * 3 + 5));
                }
                1 => {
                    vec1.insert(idx, value);
                    let _ = vec2.pop();
                }
                2 => {
                    vec1.truncate(idx);
                    vec2.shrink_to_fit();
                }
                3 => {
                    let _ = vec1.remove(idx);
                    vec2.extend_from_slice(vec1.as_slice());
                }
                4 => {
                    vec1.drain(0..idx);
                    vec3 = SmallVec::from_slice(vec2.as_slice());
                }
                5 => {
                    vec1.reserve_exact(_to_usize(GLOBAL_DATA, i * 3 + 10));
                    vec2.try_reserve(_to_usize(GLOBAL_DATA, i * 3 + 15)).ok();
                }
                6 => {
                    let cloned = vec1.clone();
                    vec3 = cloned;
                }
                _ => {}
            }
        }

        println!("Vec1: {:?}", vec1.as_slice());
        println!("Vec2: {:?}", vec2.as_mut_slice());
        println!("Vec3: {:?}", &vec3[..]);

        let cmp_order = _to_u8(GLOBAL_DATA, 500) % 3;
        let _ordering = match cmp_order {
            0 => vec1.cmp(&vec2),
            1 => vec2.cmp(&vec3),
            _ => vec3.cmp(&vec1),
        };
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