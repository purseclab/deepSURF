#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};
use std::borrow::{Borrow, BorrowMut};
use std::char;

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 1024 {
            return;
        }
        set_global_data(data);
        let global_data = get_global_data();
        let first = global_data.first_half;
        let second = global_data.second_half;

        let constructor_selector = _to_u8(first, 0);
        let mut sv: SmallVec<[u8; 32]> = match constructor_selector % 6 {
            0 => SmallVec::new(),
            1 => {
                let cap = _to_usize(first, 1);
                SmallVec::with_capacity(cap)
            }
            2 => {
                let size = _to_u8(first, 9) % 65;
                let mut v = Vec::with_capacity(size as usize);
                for i in 0..size {
                    v.push(_to_u8(first, 10 + i as usize));
                }
                SmallVec::from_vec(v)
            }
            3 => {
                let elem = _to_u8(first, 80);
                let n = _to_usize(first, 81);
                SmallVec::from_elem(elem, n)
            }
            4 => {
                let len = (_to_u8(first, 89) % 65) as usize;
                let slice = &second[..len];
                SmallVec::from_slice(slice)
            }
            _ => {
                let mut buf: [u8; 32] = [0; 32];
                for i in 0..32 {
                    buf[i] = _to_u8(first, 100 + i);
                }
                let len = _to_usize(first, 140);
                SmallVec::from_buf_and_len(buf, len)
            }
        };

        let drain_end = _to_usize(first, 160);
        {
            let mut dr = sv.drain(0..drain_end);
            if let Some(item) = dr.next() {
                println!("{:?}", item);
            }
            let _ = dr.next_back();
        }

        let op_count = (_to_u8(second, 150) % 20) as usize;
        for i in 0..op_count {
            let op_sel = _to_u8(second, 151 + i);
            match op_sel % 10 {
                0 => {
                    let val = _to_u8(second, 300 + i);
                    sv.push(val);
                }
                1 => {
                    let _ = sv.pop();
                }
                2 => {
                    let idx = _to_usize(second, 350 + i);
                    let val = _to_u8(second, 360 + i);
                    sv.insert(idx, val);
                }
                3 => {
                    let idx = _to_usize(second, 400 + i);
                    if !sv.is_empty() {
                        let _ = sv.remove(idx);
                    }
                }
                4 => {
                    let len = _to_usize(second, 450 + i);
                    sv.truncate(len);
                }
                5 => {
                    let slice_len = (_to_u8(second, 460 + i) % 65) as usize;
                    let slice = &second[..slice_len];
                    sv.extend_from_slice(slice);
                }
                6 => {
                    let additional = _to_usize(second, 470 + i);
                    sv.reserve(additional);
                }
                7 => {
                    println!("{:?}", sv.len());
                }
                8 => {
                    println!("{:?}", sv.capacity());
                }
                _ => {
                    let _ = sv.deref();
                }
            }
            let slice_mut: &mut [u8] = sv.as_mut_slice();
            if !slice_mut.is_empty() {
                slice_mut[0] = slice_mut[0].wrapping_add(1);
            }
            println!("{:?}", slice_mut.len());
        }

        let final_slice: &[u8] = sv.as_slice();
        println!("{:?}", final_slice.len());
        let _ = sv.as_slice();
        let _ = sv.as_mut_slice();
        let _ = sv.into_vec();
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