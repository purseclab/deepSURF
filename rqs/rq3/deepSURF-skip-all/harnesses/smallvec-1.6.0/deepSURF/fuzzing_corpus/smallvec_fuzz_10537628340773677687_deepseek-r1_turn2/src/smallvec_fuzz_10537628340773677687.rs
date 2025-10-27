#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct CustomType1(usize);

impl core::marker::Copy for CustomType1 {}

impl core::clone::Clone for CustomType1 {
    fn clone(&self) -> Self {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 555);
        let custom_impl_inst_num = self.0;
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0 {
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector {
            1 => global_data.first_half,
            _ => global_data.second_half,
        };
        let t_136 = _to_usize(GLOBAL_DATA, 563);
        CustomType1(t_136)
    }
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 1654 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let mut offset = 0;
        let num_ops = _to_u8(GLOBAL_DATA, offset) % 8 + 1;
        offset += 1;

        let mut containers = Vec::with_capacity(num_ops as usize);

        for _ in 0..num_ops {
            if offset >= GLOBAL_DATA.len() { break; }
            let op_selector = _to_u8(GLOBAL_DATA, offset) % 6;
            offset += 1;

            match op_selector {
                0 => {
                    let cap = _to_usize(GLOBAL_DATA, offset);
                    offset += std::mem::size_of::<usize>();
                    let mut sv = SmallVec::<[CustomType1; 32]>::with_capacity(cap);
                    containers.push(sv);
                }
                1 => {
                    let elem_count = _to_usize(GLOBAL_DATA, offset) % 65;
                    offset += std::mem::size_of::<usize>();
                    let mut elems = Vec::new();
                    for _ in 0..elem_count {
                        elems.push(CustomType1(_to_usize(GLOBAL_DATA, offset)));
                        offset += std::mem::size_of::<usize>();
                    }
                    containers.push(SmallVec::from_slice(&elems));
                }
                2 => {
                    if let Some(sv) = containers.last_mut() {
                        let new_elem = CustomType1(_to_usize(GLOBAL_DATA, offset));
                        offset += std::mem::size_of::<usize>();
                        sv.push(new_elem);
                        println!("Pushed {:?}", new_elem);
                    }
                }
                3 => {
                    if let Some(sv) = containers.last_mut() {
                        let trunc_len = _to_usize(GLOBAL_DATA, offset) % (sv.len() + 1);
                        offset += std::mem::size_of::<usize>();
                        sv.truncate(trunc_len);
                        println!("Truncated to {}: {:?}", trunc_len, sv.as_slice());
                    }
                }
                4 => {
                    if containers.len() > 1 {
                        let idx1 = _to_usize(GLOBAL_DATA, offset) % containers.len();
                        offset += std::mem::size_of::<usize>();
                        let idx2 = _to_usize(GLOBAL_DATA, offset) % containers.len();
                        offset += std::mem::size_of::<usize>();
                        let result = containers[idx1] == containers[idx2];
                        println!("Compare {} vs {}: {}", idx1, idx2, result);
                    }
                }
                5 => {
                    let clone_src = _to_usize(GLOBAL_DATA, offset) % containers.len();
                    offset += std::mem::size_of::<usize>();
                    let cloned = containers[clone_src].clone();
                    containers.push(cloned);
                }
                _ => {
                    let elem = CustomType1(_to_usize(GLOBAL_DATA, offset));
                    offset += std::mem::size_of::<usize>();
                    containers.push(SmallVec::from_elem(elem, 1));
                }
            }
        }

        if containers.len() >= 2 {
            let a = &containers[0];
            let b = &containers[1];
            println!("Final eq: {}", a == b);
            let ordering = a.cmp(b);
            println!("Cmp result: {:?}", ordering);
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