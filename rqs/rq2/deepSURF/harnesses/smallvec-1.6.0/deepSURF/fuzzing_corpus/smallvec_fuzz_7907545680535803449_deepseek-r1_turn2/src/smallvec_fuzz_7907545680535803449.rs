#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

struct CustomType1(String);
#[derive(Debug)]
struct CustomType3(String);

impl core::clone::Clone for CustomType3 {
    fn clone(&self) -> Self {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 19);
        let custom_impl_inst_num = self.0.len();
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0 {
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector {
            1 => global_data.first_half,
            _ => global_data.second_half,
        };
        let mut t_6 = _to_u8(GLOBAL_DATA, 27) % 17;
        let t_7 = _to_str(GLOBAL_DATA, 28, 28 + t_6 as usize);
        CustomType3(t_7.to_string())
    }
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 2500 {return;}
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let vec_size = _to_usize(GLOBAL_DATA, 0) % 65;
        let mut base_vec = Vec::with_capacity(vec_size);
        for i in 0..vec_size {
            let offset = 1 + i * 20;
            let s_len = _to_u8(GLOBAL_DATA, offset) % 17;
            let s = _to_str(GLOBAL_DATA, offset + 1, offset + 1 + s_len as usize);
            base_vec.push(CustomType3(s.to_string()));
        }

        let constructor_selector = _to_u8(GLOBAL_DATA, 500) % 4;
        let mut sv = match constructor_selector {
            0 => SmallVec::<[CustomType3; 32]>::new(),
            1 => SmallVec::with_capacity(_to_usize(GLOBAL_DATA, 501) % 65),
            2 => SmallVec::from_iter(base_vec.iter().cloned()),
            _ => SmallVec::from_vec(base_vec),
        };

        let op_count = _to_usize(GLOBAL_DATA, 502) % 8 + 5;
        for op_idx in 0..op_count {
            let op_type = _to_u8(GLOBAL_DATA, 600 + op_idx) % 7;
            match op_type {
                0 => {
                    let idx = _to_usize(GLOBAL_DATA, 700 + op_idx);
                    let s_len = _to_u8(GLOBAL_DATA, 800 + op_idx) % 17;
                    let s = _to_str(GLOBAL_DATA, 900 + op_idx, 900 + op_idx + s_len as usize);
                    sv.insert(idx, CustomType3(s.to_string()));
                }
                1 => {
                    let idx = _to_usize(GLOBAL_DATA, 1000 + op_idx);
                    if idx < sv.len() {
                        sv.remove(idx);
                    }
                }
                2 => {
                    sv.truncate(_to_usize(GLOBAL_DATA, 1100 + op_idx));
                }
                3 => {
                    let start_idx = 1200 + op_idx * 2;
                    let start = _to_usize(GLOBAL_DATA, start_idx);
                    let end = _to_usize(GLOBAL_DATA, start_idx + 1);
                    let drain_range = start..end;
                    let d = sv.drain(drain_range);
                    println!("Drain count: {}", d.count());
                }
                4 => {
                    let reserve_size = _to_usize(GLOBAL_DATA, 1400 + op_idx);
                    sv.reserve(reserve_size);
                }
                5 => {
                    if let Some(e) = sv.pop() {
                        println!("Popped: {:?}", e);
                    }
                }
                _ => {
                    let s_len = _to_u8(GLOBAL_DATA, 1500 + op_idx) % 17;
                    let s = _to_str(GLOBAL_DATA, 1600 + op_idx, 1600 + op_idx + s_len as usize);
                    sv.push(CustomType3(s.to_string()));
                }
            }

            if !sv.is_empty() {
                println!("Current len: {:?}", sv.get(0).unwrap());
            }
            println!("Capacity after op: {}", sv.capacity());
        }

        let start = _to_usize(GLOBAL_DATA, 2000);
        let end = _to_usize(GLOBAL_DATA, 2001);
        let final_range = start..end;
        let last_drain = sv.drain(final_range);
        last_drain.for_each(|item| println!("Drained: {:?}", item));
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