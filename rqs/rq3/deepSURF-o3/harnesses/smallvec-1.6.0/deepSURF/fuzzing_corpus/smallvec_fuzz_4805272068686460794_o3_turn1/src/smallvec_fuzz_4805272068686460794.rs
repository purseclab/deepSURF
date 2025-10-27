#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::ops::{Deref, DerefMut, Index, IndexMut};

fn build_smallvec(data: &[u8]) -> SmallVec<[u8; 32]> {
    let selector = _to_u8(data, 0) % 5;
    match selector {
        0 => SmallVec::<[u8; 32]>::new(),
        1 => {
            let cap = _to_usize(data, 1) % 65;
            SmallVec::<[u8; 32]>::with_capacity(cap)
        }
        2 => {
            let len = _to_u8(data, 9) as usize % 65;
            let mut vec: Vec<u8> = (0..len).map(|i| _to_u8(data, 10 + i)).collect();
            SmallVec::<[u8; 32]>::from_vec(vec)
        }
        3 => {
            let len = _to_u8(data, 75) as usize % 65;
            let slice_vec: Vec<u8> = (0..len).map(|i| _to_u8(data, 76 + i)).collect();
            SmallVec::<[u8; 32]>::from_slice(&slice_vec)
        }
        _ => {
            let mut buf = [0u8; 32];
            for i in 0..32 {
                buf[i] = _to_u8(data, 140 + i);
            }
            let len = _to_usize(data, 50) % 33;
            SmallVec::<[u8; 32]>::from_buf_and_len(buf, len)
        }
    }
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 246 {
            return;
        }
        set_global_data(data);
        let global_data = get_global_data();
        let gd = global_data.first_half;

        let mut sv_primary = build_smallvec(gd);
        let mut sv_secondary = sv_primary.clone();

        let op_count = (_to_u8(gd, 180) % 15) as usize;
        for i in 0..op_count {
            let op_id = _to_u8(gd, 181 + i) % 15;
            match op_id {
                0 => {
                    let additional = _to_usize(gd, 34 + i);
                    sv_primary.reserve(additional);
                }
                1 => {
                    let val = _to_u8(gd, 60 + i);
                    sv_primary.push(val);
                }
                2 => {
                    sv_primary.pop();
                }
                3 => {
                    let len = _to_usize(gd, 70 + i) % 65;
                    sv_primary.truncate(len);
                }
                4 => {
                    let idx = (_to_usize(gd, 80 + i)) % (sv_primary.len().max(1));
                    let val = _to_u8(gd, 90 + i);
                    sv_primary.insert(idx, val);
                }
                5 => {
                    if !sv_primary.is_empty() {
                        let idx = (_to_usize(gd, 100 + i)) % sv_primary.len();
                        sv_primary.remove(idx);
                    }
                }
                6 => {
                    sv_primary.clear();
                }
                7 => {
                    let slice_len = _to_u8(gd, 110 + i) as usize % 65;
                    let slice_vec: Vec<u8> =
                        (0..slice_len).map(|j| _to_u8(gd, 111 + i + j)).collect();
                    sv_primary.extend_from_slice(&slice_vec);
                }
                8 => {
                    let _ = sv_primary.cmp(&sv_secondary);
                }
                9 => {
                    let _ = sv_primary.partial_cmp(&sv_secondary);
                }
                10 => {
                    sv_primary.dedup();
                }
                11 => {
                    sv_primary.shrink_to_fit();
                }
                12 => {
                    if !sv_primary.is_empty() {
                        let idx = (_to_usize(gd, 120 + i)) % sv_primary.len();
                        sv_primary.swap_remove(idx);
                    }
                }
                13 => {
                    let add = _to_usize(gd, 130 + i);
                    sv_primary.reserve_exact(add);
                }
                _ => {
                    let _borrowed: &[u8] = sv_primary.as_ref();
                    if !_borrowed.is_empty() {
                        let _first = _borrowed[0];
                        println!("{:?}", _first);
                    }
                }
            }
        }

        let cap = sv_primary.capacity();
        println!("{:?}", cap);

        let slice = sv_primary.as_slice();
        if !slice.is_empty() {
            println!("{:?}", slice[0]);
        }

        let mut iter = sv_primary.clone().into_iter();
        if let Some(val) = iter.next() {
            println!("{:?}", val);
        }

        sv_primary.reserve(_to_usize(gd, 34));
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