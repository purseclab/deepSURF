#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

#[derive(Debug)]
struct CustomType1(String);

impl core::clone::Clone for CustomType1 {
    fn clone(&self) -> Self {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 42);
        let custom_impl_inst_num = self.0.len();
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0 {
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector {
            1 => global_data.first_half,
            _ => global_data.second_half,
        };
        let mut t_10 = _to_u8(GLOBAL_DATA, 50) % 17;
        let t_11 = _to_str(GLOBAL_DATA, 51, 51 + t_10 as usize);
        let t_12 = String::from(t_11);
        CustomType1(t_12)
    }
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 512 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        
        let constructor_selector = _to_u8(GLOBAL_DATA, 0) % 4;
        let mut sv = match constructor_selector {
            0 => SmallVec::<[CustomType1; 32]>::new(),
            1 => {
                let cap = _to_usize(GLOBAL_DATA, 1);
                SmallVec::<[CustomType1; 32]>::with_capacity(cap)
            }
            2 => {
                let elem_count = _to_usize(GLOBAL_DATA, 2) % 65;
                let len = _to_usize(GLOBAL_DATA, 10) % 65;
                let mut sv = SmallVec::<[CustomType1; 32]>::new();
                for _ in 0..elem_count {
                    let s_len = _to_u8(GLOBAL_DATA, 20) % 17;
                    let s = _to_str(GLOBAL_DATA, 21, 21 + s_len as usize);
                    sv.push(CustomType1(s.to_string()));
                }
                sv
            }
            _ => {
                let elem_s = _to_str(GLOBAL_DATA, 100, 110);
                let v = vec![CustomType1(elem_s.to_string())];
                SmallVec::<[CustomType1; 32]>::from_vec(v)
            }
        };

        let num_ops = _to_u8(GLOBAL_DATA, 200) % 10;
        for op_idx in 0..num_ops {
            let op_byte = _to_u8(GLOBAL_DATA, 201 + op_idx as usize) % 7;
            
            match op_byte {
                0 => {
                    let s_len = _to_u8(GLOBAL_DATA, 300) % 17;
                    let s = _to_str(GLOBAL_DATA, 301, 301 + s_len as usize);
                    sv.push(CustomType1(s.to_string()));
                }
                1 => {
                    let _ = sv.pop();
                }
                2 => {
                    let cap = _to_usize(GLOBAL_DATA, 400);
                    sv.reserve(cap);
                }
                3 => {
                    let new_len = _to_usize(GLOBAL_DATA, 450);
                    sv.truncate(new_len);
                }
                4 => {
                    let idx = _to_usize(GLOBAL_DATA, 460) % (sv.len() + 1);
                    let s_len = _to_u8(GLOBAL_DATA, 470) % 17;
                    let s = _to_str(GLOBAL_DATA, 471, 471 + s_len as usize);
                    sv.insert(idx, CustomType1(s.to_string()));
                }
                5 => {
                    let new_len = _to_usize(GLOBAL_DATA, 500);
                    let s_len = _to_u8(GLOBAL_DATA, 510) % 17;
                    let s = _to_str(GLOBAL_DATA, 511, 511 + s_len as usize);
                    sv.resize(new_len, CustomType1(s.to_string()));
                }
                6 => {
                    let other_len = _to_usize(GLOBAL_DATA, 600) % 65;
                    let mut other = SmallVec::<[CustomType1; 32]>::new();
                    for _ in 0..other_len {
                        let s_len = _to_u8(GLOBAL_DATA, 610) % 17;
                        let s = _to_str(GLOBAL_DATA, 611, 611 + s_len as usize);
                        other.push(CustomType1(s.to_string()));
                    }
                    sv.append(&mut other);
                }
                _ => {}
            }
        }

        if !sv.is_empty() {
            println!("{:?}", sv.as_slice());
            let _ = sv.as_mut_slice().get_mut(0).map(|x| *x = CustomType1("modified".to_string()));
        }
        let final_len = _to_usize(GLOBAL_DATA, 700);
        let s_len = _to_u8(GLOBAL_DATA, 710) % 17;
        let s = _to_str(GLOBAL_DATA, 711, 711 + s_len as usize);
        sv.resize(final_len, CustomType1(s.to_string()));
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