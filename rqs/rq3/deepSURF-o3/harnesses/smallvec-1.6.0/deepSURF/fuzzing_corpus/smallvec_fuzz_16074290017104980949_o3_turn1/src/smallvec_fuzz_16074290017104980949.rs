#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::ops::{Deref, DerefMut, Index, IndexMut};

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 4096 {
            return;
        }
        set_global_data(data);
        let global_data = get_global_data();
        let first = global_data.first_half;
        let second = global_data.second_half;

        let vec_len = _to_u8(first, 0) % 65;
        let mut seed_vec = Vec::with_capacity(vec_len as usize);
        for i in 0..vec_len {
            seed_vec.push(_to_u8(first, 1 + i as usize));
        }

        let mut pre_sv = SmallVec::<[u8; 32]>::with_capacity(_to_usize(second, 0));
        if !seed_vec.is_empty() {
            pre_sv.extend_from_slice(&seed_vec);
        }
        let _ = pre_sv.capacity();
        if !pre_sv.is_empty() {
            let first_ref = &pre_sv[0];
            println!("{:?}", *first_ref);
        }

        let mut sv = SmallVec::<[u8; 32]>::from(seed_vec.clone());

        let ops_count = (_to_u8(second, 8) % 20) as usize;
        for j in 0..ops_count {
            let selector = _to_u8(second, 9 + j);
            match selector % 10 {
                0 => {
                    let val = _to_u8(second, 100 + j);
                    sv.push(val);
                }
                1 => {
                    sv.pop();
                }
                2 => {
                    let idx = _to_usize(second, 200 + j * 8);
                    let byte_ref = &sv[idx];
                    println!("{:?}", *byte_ref);
                }
                3 => {
                    sv.clear();
                }
                4 => {
                    let additional = _to_usize(second, 400 + j * 8);
                    sv.reserve(additional);
                }
                5 => {
                    let new_len = _to_usize(second, 600 + j * 8);
                    sv.truncate(new_len);
                }
                6 => {
                    let rem_idx = _to_usize(second, 800 + j * 8);
                    sv.remove(rem_idx);
                }
                7 => {
                    let ins_idx = _to_usize(second, 1000 + j * 8);
                    let val = _to_u8(second, 1100 + j);
                    sv.insert(ins_idx, val);
                }
                8 => {
                    let end = _to_usize(second, 1200 + j * 8);
                    let mut drain_iter = sv.drain(0..end);
                    let _ = drain_iter.next();
                }
                _ => {
                    let cmp_sv = SmallVec::<[u8; 32]>::from(seed_vec.clone());
                    let _ = sv.partial_cmp(&cmp_sv);
                }
            }
        }

        let out_vec = sv.into_vec();
        let _ = SmallVec::<[u8; 32]>::from(out_vec);
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