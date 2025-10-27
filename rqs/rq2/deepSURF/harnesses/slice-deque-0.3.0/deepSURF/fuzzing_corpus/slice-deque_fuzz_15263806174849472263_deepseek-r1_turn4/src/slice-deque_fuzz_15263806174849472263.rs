#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use slice_deque::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

#[derive(Debug)]
struct CustomType1(String);
#[derive(Debug, Clone)]
struct CustomType2(String);
struct CustomType3(String);
#[derive(Debug)]
struct CustomType0(String);
struct CustomType4(String);

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 2190 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let constructor_selector = _to_u8(GLOBAL_DATA, 0) % 4;
        let mut deque = match constructor_selector {
            0 => SliceDeque::new(),
            1 => {
                let cap = _to_usize(GLOBAL_DATA, 1) % 65;
                SliceDeque::with_capacity(cap)
            },
            2 => {
                let vec_len = _to_u8(GLOBAL_DATA, 1) % 65;
                let mut elements = Vec::with_capacity(vec_len as usize);
                let mut data_idx = 2;
                for _ in 0..vec_len {
                    let len = _to_u8(GLOBAL_DATA, data_idx) % 17;
                    data_idx += 1;
                    let s = _to_str(GLOBAL_DATA, data_idx, data_idx + len as usize);
                    elements.push(CustomType2(s.to_string()));
                    data_idx += len as usize;
                }
                SliceDeque::from(&elements[..])
            },
            _ => {
                let vec_len = _to_u8(GLOBAL_DATA, 1) % 65;
                let mut elements = Vec::with_capacity(vec_len as usize);
                let mut data_idx = 2;
                for _ in 0..vec_len {
                    let len = _to_u8(GLOBAL_DATA, data_idx) % 17;
                    data_idx += 1;
                    let s = _to_str(GLOBAL_DATA, data_idx, data_idx + len as usize);
                    elements.push(CustomType2(s.to_string()));
                    data_idx += len as usize;
                }
                SliceDeque::from_iter(elements.into_iter())
            }
        };

        let ops_count = _to_u8(GLOBAL_DATA, 400) % 10;
        let mut data_idx = 401;

        for _ in 0..ops_count {
            let op = _to_u8(GLOBAL_DATA, data_idx) % 8;
            data_idx += 1;

            match op {
                0 => {
                    let len = _to_u8(GLOBAL_DATA, data_idx) % 17;
                    data_idx += 1;
                    let s = _to_str(GLOBAL_DATA, data_idx, data_idx + len as usize);
                    data_idx += len as usize;
                    deque.push_back(CustomType2(s.to_string()));
                },
                1 => {
                    let len = _to_u8(GLOBAL_DATA, data_idx) % 17;
                    data_idx += 1;
                    let s = _to_str(GLOBAL_DATA, data_idx, data_idx + len as usize);
                    data_idx += len as usize;
                    deque.push_front(CustomType2(s.to_string()));
                },
                2 => {
                    let _ = deque.pop_back().map(|x| println!("{:?}", x));
                },
                3 => {
                    let _ = deque.pop_front().map(|x| println!("{:?}", x));
                },
                4 => {
                    let start = _to_usize(GLOBAL_DATA, data_idx) % (deque.len() + 1);
                    let end = start + _to_usize(GLOBAL_DATA, data_idx + 8) % (deque.len() - start + 1);
                    let replace_len = _to_u8(GLOBAL_DATA, data_idx + 16) % 17;
                    let replace_str = _to_str(GLOBAL_DATA, data_idx + 17, data_idx + 17 + replace_len as usize);
                    data_idx += 17 + replace_len as usize;
                    let _ = deque.splice(start..end, std::iter::once(CustomType2(replace_str.to_string())));
                },
                5 => {
                    let start = _to_usize(GLOBAL_DATA, data_idx) % (deque.len() + 1);
                    let end = start + _to_usize(GLOBAL_DATA, data_idx + 8) % (deque.len() - start + 1);
                    data_idx += 16;
                    let _ = deque.drain(start..end);
                },
                6 => println!("{:?}", deque.as_slice()),
                7 => {
                    let dest_cap = _to_usize(GLOBAL_DATA, data_idx) % 65;
                    data_idx += 8;
                    deque.reserve(dest_cap);
                },
                _ => ()
            }
        }

        let range_start = _to_usize(GLOBAL_DATA, 1200) % (deque.len() + 1);
        let range_end = range_start + _to_usize(GLOBAL_DATA, 1208) % (deque.len() - range_start + 1);
        let iter_str = _to_str(GLOBAL_DATA, 1216, 1232);
        let mut splice = deque.splice(range_start..range_end, std::iter::once(CustomType2(iter_str.to_string())));
        let _ = splice.next();
        let _ = splice.next_back();
    });
}

// The type conversion functions remain unchanged and should be added here as provided...

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