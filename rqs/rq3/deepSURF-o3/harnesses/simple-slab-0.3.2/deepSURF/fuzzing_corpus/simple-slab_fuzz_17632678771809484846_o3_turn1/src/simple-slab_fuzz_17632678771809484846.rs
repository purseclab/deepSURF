#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use simple_slab::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

#[derive(Clone, Debug)]
struct CustomType0(String);

impl Deref for CustomType0 {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for CustomType0 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 200 {
            return;
        }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        if GLOBAL_DATA.len() < 120 {
            return;
        }

        let capacity = _to_usize(GLOBAL_DATA, 0) % 65;
        let use_capacity_ctor = _to_bool(GLOBAL_DATA, 8);
        let mut slab: simple_slab::Slab<CustomType0> = if use_capacity_ctor {
            simple_slab::Slab::with_capacity(capacity)
        } else {
            simple_slab::Slab::new()
        };

        let mut cursor = 9usize;
        let op_count = (_to_u8(GLOBAL_DATA, cursor) % 25) as usize;
        cursor += 1;

        for _ in 0..op_count {
            if cursor >= GLOBAL_DATA.len() {
                break;
            }
            let op = _to_u8(GLOBAL_DATA, cursor) % 6;
            cursor += 1;
            match op {
                0 => {
                    if cursor >= GLOBAL_DATA.len() {
                        break;
                    }
                    let str_len = (_to_u8(GLOBAL_DATA, cursor) % 32) as usize;
                    cursor += 1;
                    if cursor + str_len >= GLOBAL_DATA.len() {
                        break;
                    }
                    let s = _to_str(GLOBAL_DATA, cursor, cursor + str_len);
                    cursor += str_len;
                    slab.insert(CustomType0(String::from(s)));
                }
                1 => {
                    if slab.len() > 0 {
                        if cursor >= GLOBAL_DATA.len() {
                            break;
                        }
                        let idx = (_to_u8(GLOBAL_DATA, cursor) as usize) % slab.len();
                        cursor += 1;
                        slab.remove(idx);
                    } else if cursor < GLOBAL_DATA.len() {
                        cursor += 1;
                    }
                }
                2 => {
                    for elem in slab.iter() {
                        println!("{:?}", *elem);
                    }
                }
                3 => {
                    for elem in slab.iter_mut() {
                        elem.push_str("x");
                        println!("{:?}", &*elem);
                    }
                }
                4 => {
                    for elem in (&slab).into_iter() {
                        println!("{:?}", *elem);
                    }
                }
                5 => {
                    for elem in (&mut slab).into_iter() {
                        elem.push_str("y");
                    }
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