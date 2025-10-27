#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::ops::{Deref, DerefMut, Index, IndexMut};
use std::str::FromStr;

fn construct_smallvec(seed: &[u8]) -> SmallVec<[u8; 64]> {
    if seed.len() < 20 {
        return SmallVec::<[u8; 64]>::new();
    }
    let selector = _to_u8(seed, 0);
    match selector % 4 {
        0 => SmallVec::<[u8; 64]>::new(),
        1 => {
            let cap = _to_usize(seed, 1);
            SmallVec::<[u8; 64]>::with_capacity(cap)
        }
        2 => {
            let n = (_to_u8(seed, 9) % 55) as usize;
            let mut v = Vec::with_capacity(n);
            for i in 0..n {
                v.push(_to_u8(seed, 10 + i));
            }
            SmallVec::<[u8; 64]>::from_vec(v)
        }
        _ => {
            let elem = _to_u8(seed, 1);
            let n = (_to_u8(seed, 2) % 55) as usize;
            SmallVec::<[u8; 64]>::from_elem(elem, n)
        }
    }
}

fn dispatch_ops(vec: &mut SmallVec<[u8; 64]>, script: &[u8]) {
    if script.is_empty() {
        return;
    }
    let op_cnt = (_to_u8(script, 0) % 20) as usize;
    let mut cursor = 1usize;
    for _ in 0..op_cnt {
        if cursor >= script.len() {
            break;
        }
        let opcode = _to_u8(script, cursor);
        cursor += 1;
        match opcode % 16 {
            0 => {
                vec.deref();
            }
            1 => {
                if cursor < script.len() {
                    let v = _to_u8(script, cursor);
                    cursor += 1;
                    vec.push(v);
                }
            }
            2 => {
                vec.pop();
            }
            3 => {
                if !vec.is_empty() && cursor + 8 <= script.len() {
                    let raw = _to_usize(script, cursor);
                    cursor += 8;
                    let idx = raw % vec.len();
                    vec.remove(idx);
                }
            }
            4 => {
                if cursor + 8 <= script.len() {
                    let len = _to_usize(script, cursor);
                    cursor += 8;
                    vec.truncate(len);
                }
            }
            5 => {
                if cursor + 8 <= script.len() {
                    let add = _to_usize(script, cursor);
                    cursor += 8;
                    vec.reserve(add);
                }
            }
            6 => {
                if !vec.is_empty() && cursor + 9 <= script.len() {
                    let raw = _to_usize(script, cursor);
                    cursor += 8;
                    let idx = raw % vec.len();
                    let val = _to_u8(script, cursor);
                    cursor += 1;
                    vec.insert(idx, val);
                }
            }
            7 => {
                if cursor < script.len() {
                    let slice_len = (_to_u8(script, cursor) % 55) as usize;
                    cursor += 1;
                    if cursor + slice_len <= script.len() {
                        let slice = &script[cursor..cursor + slice_len];
                        vec.extend_from_slice(slice);
                        cursor += slice_len;
                    } else {
                        cursor = script.len();
                    }
                }
            }
            8 => {
                vec.clear();
            }
            9 => {
                let r = vec.deref_mut();
                if !r.is_empty() {
                    println!("{:?}", r[0]);
                }
            }
            10 => {
                let other = vec.clone();
                (&*vec).cmp(&other);
            }
            11 => {
                if cursor + 9 <= script.len() {
                    let new_len = _to_usize(script, cursor);
                    cursor += 8;
                    let filler = _to_u8(script, cursor);
                    cursor += 1;
                    vec.resize(new_len, filler);
                }
            }
            12 => {
                vec.is_empty();
            }
            13 => {
                if !vec.is_empty() && cursor < script.len() {
                    let drain_len = (_to_u8(script, cursor) as usize) % (vec.len() + 1);
                    cursor += 1;
                    let mut dr = vec.drain(0..drain_len);
                    dr.next();
                    dr.next_back();
                }
            }
            _ => {
                vec.shrink_to_fit();
            }
        }
    }
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 130 {
            return;
        }
        set_global_data(data);
        let halves = get_global_data();
        let first = halves.first_half;
        let second = halves.second_half;

        let mut sv = construct_smallvec(first);
        dispatch_ops(&mut sv, second);
        let slice = sv.deref_mut();
        if !slice.is_empty() {
            println!("{:?}", slice.len());
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