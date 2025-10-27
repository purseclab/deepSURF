#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::ops::{Deref, DerefMut, Index, IndexMut};

#[derive(Debug, PartialEq)]
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
        let t_137 = CustomType1(t_136);
        t_137
    }
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 5000 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let op_count = _to_u8(GLOBAL_DATA, 0) % 10;

        let mut vec1 = SmallVec::<[CustomType1; 32]>::new();
        let mut vec2 = SmallVec::<[CustomType1; 32]>::with_capacity(16);
        let mut vec3 = SmallVec::<[CustomType1; 32]>::from_vec(Vec::new());

        for i in 0..op_count {
            let op_selector = _to_u8(GLOBAL_DATA, i as usize * 3) % 7;
            match op_selector {
                0 => {
                    let elem = CustomType1(_to_usize(GLOBAL_DATA, i as usize * 3 + 1));
                    vec1.push(elem);
                }
                1 => {
                    vec2.truncate(_to_usize(GLOBAL_DATA, i as usize * 3 + 2) as usize);
                }
                2 => {
                    let new_cap = _to_usize(GLOBAL_DATA, i as usize * 3 + 3);
                    vec3.reserve(new_cap);
                }
                3 => {
                    let idx = _to_usize(GLOBAL_DATA, i as usize * 3 + 1) % vec1.len().max(1);
                    let _ = vec1.remove(idx);
                }
                4 => {
                    let slice = vec2.as_slice();
                    if !slice.is_empty() {
                        let _derefs = slice.get(0).unwrap();
                        println!("{:?}", _derefs);
                    }
                }
                5 => {
                    let _ = vec3.insert(
                        _to_usize(GLOBAL_DATA, i as usize * 3 + 2) % (vec3.len() + 1),
                        CustomType1(_to_usize(GLOBAL_DATA, i as usize * 3 + 3))
                    );
                }
                6 => {
                    let drain_range = _to_usize(GLOBAL_DATA, i as usize * 3 + 1) % vec3.len().max(1);
                    let drainer = vec3.drain(drain_range..);
                    for _ in drainer {}
                }
                _ => (),
            }
        }

        let slice_data: Vec<_> = (800..900)
            .map(|i| CustomType1(_to_usize(GLOBAL_DATA, i)))
            .collect();
        let vec4 = SmallVec::<[CustomType1; 32]>::from_slice(&slice_data);
        
        let _ = vec1 == vec4;
        let vec5 = vec2.clone();
        let _ = vec4 == vec5;
        
        let mut dynamic_vec1: Vec<_> = (500..600)
            .filter(|i| _to_u8(GLOBAL_DATA, *i) % 2 == 0)
            .map(|i| CustomType1(_to_usize(GLOBAL_DATA, i)))
            .collect();
        let vec6 = SmallVec::<[CustomType1; 32]>::from_vec(dynamic_vec1);
        format!("{:?}", vec6.as_ptr());
        
        let cap_val = _to_usize(GLOBAL_DATA, 999);
        let mut vec7 = SmallVec::<[CustomType1; 32]>::with_capacity(cap_val);
        _ = vec7.as_mut_slice();
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