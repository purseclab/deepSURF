#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

struct CustomType2(String);
#[derive(Clone, Debug)]
struct CustomType1(String);

impl core::cmp::PartialEq for CustomType2 {
    fn eq(&self, _: &Self) -> bool {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 34);
        let custom_impl_inst_num = self.0.len();
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0 {
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector {
            1 => global_data.first_half,
            _ => global_data.second_half,
        };
        let t_9 = _to_bool(GLOBAL_DATA, 42);
        return t_9;
    }
}

fn _custom_fn0(_: &mut CustomType1) -> CustomType2 {
    let global_data = get_global_data();
    let GLOBAL_DATA = global_data.first_half;
    let t_10 = _to_u8(GLOBAL_DATA, 43);
    if t_10 % 2 == 0 {
        panic!("INTENTIONAL PANIC!");
    }
    let mut t_11 = _to_u8(GLOBAL_DATA, 44) % 17;
    let t_12 = _to_str(GLOBAL_DATA, 45, 45 + t_11 as usize);
    let t_13 = String::from(t_12);
    let t_14 = CustomType2(t_13);
    return t_14;
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 500 {return;}
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let mut sv = match _to_u8(GLOBAL_DATA, 0) % 4 {
            0 => {
                let cap = _to_usize(GLOBAL_DATA, 10) % 65;
                SmallVec::<[CustomType1; 128]>::with_capacity(cap)
            }
            1 => {
                let len = _to_usize(GLOBAL_DATA, 15) % 65;
                let elem = CustomType1(String::from(_to_str(GLOBAL_DATA, 20, 25)));
                SmallVec::from_elem(elem, len)
            }
            2 => {
                let len = _to_usize(GLOBAL_DATA, 25) % 65;
                let mut vec = Vec::with_capacity(len);
                for i in 0..len {
                    let s = String::from(_to_str(GLOBAL_DATA, 30 + i*2, 35 + i*2));
                    vec.push(CustomType1(s));
                }
                SmallVec::from_vec(vec)
            }
            _ => SmallVec::<[CustomType1; 128]>::new()
        };

        for i in 0.._to_usize(GLOBAL_DATA, 100) % 50 {
            match _to_u8(GLOBAL_DATA, 150 + i) % 12 {
                0 => {
                    let s = String::from(_to_str(GLOBAL_DATA, 200 + i*3, 203 + i*3));
                    sv.push(CustomType1(s));
                }
                1 => {
                    let idx = _to_usize(GLOBAL_DATA, 300 + i) % (sv.len() + 1);
                    let s = String::from(_to_str(GLOBAL_DATA, 400 + i*3, 403 + i*3));
                    sv.insert(idx, CustomType1(s));
                }
                2 => {
                    sv.pop();
                }
                3 => {
                    sv.append(&mut sv.clone());
                }
                4 => {
                    let idx = _to_usize(GLOBAL_DATA, 500 + i) % sv.len();
                    sv.remove(idx);
                }
                5 => {
                    let cap = sv.capacity() / 2;
                    sv.reserve(cap);
                }
                6 => {
                    let slice = sv.as_slice();
                    println!("{:?}", slice);
                }
                7 => {
                    let mut slice = sv.as_mut_slice();
                    println!("{:?}", slice);
                }
                8 => {
                    let idx = _to_usize(GLOBAL_DATA, 600 + i) % sv.len();
                    sv.swap_remove(idx);
                }
                9 => {
                    let range = _to_usize(GLOBAL_DATA, 600 + i) % sv.len();
                    let _ = sv.drain(0..range);
                }
                10 => {
                    let new_len = _to_usize(GLOBAL_DATA, 700 + i) % (sv.len() + 1);
                    sv.truncate(new_len);
                }
                _ => {
                    sv.dedup_by_key(_custom_fn0);
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