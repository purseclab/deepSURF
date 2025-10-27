#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 400 {
            return;
        }
        set_global_data(data);
        let global_data = get_global_data();
        let first = global_data.first_half;

        let mut arr8 = [0u8; 8];
        arr8.copy_from_slice(&first[0..8]);
        let len_buf = (_to_u8(first, 16) as usize) % 8;
        let mut sv_main = SmallVec::<[u8; 8]>::from_buf_and_len(arr8, len_buf);

        let cap = _to_usize(first, 24);
        let mut sv_aux = SmallVec::<[u8; 16]>::with_capacity(cap);
        sv_aux.push(_to_u8(first, 32));
        sv_aux.extend_from_slice(&first[33..36]);

        let iterations = (_to_u8(first, 40) % 20) as usize + 1;
        for i in 0..iterations {
            let sel = _to_u8(first, 41 + i) % 10;
            match sel {
                0 => {
                    let start = (_to_usize(first, 60 + i)) % (first.len() - 50);
                    let end = start + 50;
                    let iter = first[start..end].iter().cloned();
                    sv_main = SmallVec::<[u8; 8]>::from_iter(iter);
                }
                1 => {
                    let val = _to_u8(first, 80 + i);
                    sv_main.push(val);
                }
                2 => {
                    let _ = sv_main.pop();
                }
                3 => {
                    if !sv_main.is_empty() {
                        let idx = (_to_usize(first, 90 + i)) % sv_main.len();
                        sv_main.insert(idx, _to_u8(first, 95 + i));
                    }
                }
                4 => {
                    if !sv_main.is_empty() {
                        let idx = (_to_usize(first, 100 + i)) % sv_main.len();
                        let _ = sv_main.remove(idx);
                    }
                }
                5 => {
                    let slice_len = (_to_u8(first, 105 + i) as usize) % 65;
                    let end = 110 + slice_len;
                    if end <= first.len() {
                        sv_main.extend_from_slice(&first[110..end]);
                    }
                }
                6 => {
                    let new_len = (_to_usize(first, 120 + i)) % 65;
                    sv_main.truncate(new_len);
                }
                7 => {
                    let additional = _to_usize(first, 130 + i);
                    sv_main.reserve(additional);
                }
                8 => {
                    if !sv_main.is_empty() {
                        let idx = (_to_usize(first, 140 + i)) % sv_main.len();
                        let val_ref = &sv_main[idx];
                        println!("{:?}", *val_ref);
                    }
                }
                9 => {
                    if sv_main.len() > 0 {
                        let start = (_to_usize(first, 150 + i)) % sv_main.len();
                        let end = start + ((_to_u8(first, 160 + i) as usize) % 65);
                        let range_end = std::cmp::min(end, sv_main.len());
                        let mut dr = sv_main.drain(start..range_end);
                        let _ = dr.next();
                        let _ = dr.next_back();
                    }
                }
                _ => {}
            }
        }

        let len_ref = &sv_main.len();
        println!("{:?}", *len_ref);

        let slice_ref = sv_main.as_slice();
        if !slice_ref.is_empty() {
            println!("{:?}", slice_ref[0]);
        }

        let _cmp = sv_main.partial_cmp(&sv_main.clone());
        let _vec = sv_main.clone().into_vec();
        sv_aux.append(&mut sv_main);
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