#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::borrow::{Borrow, BorrowMut};
use std::ops::{Deref, DerefMut};

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 196 {
            return;
        }
        set_global_data(data);
        let global_data = get_global_data();
        let first = global_data.first_half;
        let second = global_data.second_half;

        let constructor_id = _to_u8(first, 0) % 6;
        let mut sv: SmallVec<[u8; 32]> = match constructor_id {
            0 => SmallVec::<[u8; 32]>::new(),
            1 => {
                let cap = _to_usize(first, 1) % 65;
                SmallVec::<[u8; 32]>::with_capacity(cap)
            }
            2 => {
                let len = (_to_u8(first, 9) % 65) as usize;
                let end = std::cmp::min(len, second.len());
                SmallVec::<[u8; 32]>::from_slice(&second[..end])
            }
            3 => {
                let len = (_to_u8(first, 17) % 65) as usize;
                let end = std::cmp::min(len, second.len());
                let mut v = Vec::with_capacity(len);
                v.extend_from_slice(&second[..end]);
                SmallVec::<[u8; 32]>::from_vec(v)
            }
            4 => {
                let mut buf = [0u8; 32];
                for (i, b) in buf.iter_mut().enumerate() {
                    *b = second[i % second.len()];
                }
                let len = _to_usize(first, 25) % 33;
                SmallVec::<[u8; 32]>::from_buf_and_len(buf, len)
            }
            _ => {
                let elem = _to_u8(first, 33);
                let count = _to_usize(first, 34) % 65;
                SmallVec::<[u8; 32]>::from_elem(elem, count)
            }
        };

        let _ = sv.len();
        let _ = sv.is_empty();
        let _ = sv.capacity();
        println!("{:?}", sv.as_slice());
        let _ = sv.cmp(&sv.clone());
        let _ = sv.partial_cmp(&sv.clone());

        let additional = _to_usize(first, 42);
        let _ = sv.try_reserve(additional);

        let ops = (_to_u8(first, 50) % 20) as usize;
        for i in 0..ops {
            let op = _to_u8(second, i) % 10;
            match op {
                0 => {
                    let val = _to_u8(second, i + 1);
                    sv.push(val);
                }
                1 => {
                    let _ = sv.pop();
                }
                2 => {
                    let new_len = _to_usize(first, i) % 65;
                    sv.truncate(new_len);
                }
                3 => {
                    let add = _to_usize(second, i);
                    let _ = sv.try_reserve(add);
                }
                4 => {
                    let add = _to_usize(second, i);
                    sv.reserve(add);
                }
                5 => {
                    if !sv.is_empty() {
                        let idx = _to_usize(second, i) % sv.len();
                        let _ = sv.remove(idx);
                    }
                }
                6 => {
                    if !sv.is_empty() {
                        let idx = _to_usize(second, i) % sv.len();
                        let _ = sv.swap_remove(idx);
                    }
                }
                7 => {
                    let val = _to_u8(first, i);
                    sv.insert(0, val);
                }
                8 => {
                    sv.clear();
                }
                _ => {
                    println!("{:?}", sv.capacity());
                }
            }
        }

        println!("{:?}", sv.deref());
        let slice_ref: &[u8] = sv.borrow();
        println!("{:?}", slice_ref);
        let slice_mut: &mut [u8] = sv.borrow_mut();
        if !slice_mut.is_empty() {
            slice_mut[0] = slice_mut[0].wrapping_add(1);
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