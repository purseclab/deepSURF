#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};
use std::borrow::Borrow;

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 130 {
            return;
        }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let len_first = GLOBAL_DATA.len();

        let mode = _to_u8(GLOBAL_DATA, 0) % 5;
        let elem = _to_u8(GLOBAL_DATA, 3);
        let length = (_to_u8(GLOBAL_DATA, 4) % 65) as usize;
        let cap_raw = _to_usize(GLOBAL_DATA, 5);
        let cap = if cap_raw == 0 { 1 } else { cap_raw % 65 };

        let slice_end = 12 + length;
        let slice_end_mod = if slice_end < len_first { slice_end } else { len_first };
        let slice = &GLOBAL_DATA[12..slice_end_mod];
        let vec_data = slice.to_vec();

        let mut small_vec: SmallVec<[u8; 16]> = match mode {
            0 => SmallVec::<[u8; 16]>::new(),
            1 => SmallVec::<[u8; 16]>::from_slice(slice),
            2 => SmallVec::<[u8; 16]>::from_vec(vec_data),
            3 => SmallVec::<[u8; 16]>::with_capacity(cap),
            _ => SmallVec::<[u8; 16]>::from_elem(elem, length),
        };

        let operations = _to_u8(GLOBAL_DATA, 2) % 20;
        for i in 0..operations {
            let code = _to_u8(GLOBAL_DATA, (30 + i as usize) % len_first) % 12;
            match code {
                0 => {
                    let v = _to_u8(GLOBAL_DATA, (50 + i as usize) % len_first);
                    small_vec.push(v);
                }
                1 => {
                    if !small_vec.is_empty() {
                        let idx = (_to_u8(GLOBAL_DATA, (60 + i as usize) % len_first) as usize) % small_vec.len();
                        small_vec.remove(idx);
                    }
                }
                2 => {
                    let idx_bytes = (70 + i as usize) % (len_first - 8);
                    let idx = _to_usize(GLOBAL_DATA, idx_bytes) % (small_vec.len() + 1);
                    small_vec.insert(idx, elem);
                }
                3 => {
                    small_vec.pop();
                }
                4 => {
                    let resize_len = (_to_u8(GLOBAL_DATA, (80 + i as usize) % len_first) % 65) as usize;
                    small_vec.resize(resize_len, elem);
                }
                5 => {
                    let slice_ext_len = (_to_u8(GLOBAL_DATA, (90 + i as usize) % len_first) % 65) as usize;
                    let start = (100 + i as usize) % len_first;
                    let end = std::cmp::min(len_first, start + slice_ext_len);
                    let ext_slice = &GLOBAL_DATA[start..end];
                    small_vec.extend_from_slice(ext_slice);
                }
                6 => {
                    let mut temp = SmallVec::<[u8; 16]>::with_capacity(1);
                    temp.append(&mut small_vec);
                    small_vec = temp;
                }
                7 => {
                    let _ = small_vec.capacity();
                }
                8 => {
                    let _ = small_vec.len();
                }
                9 => {
                    let slice_ref: &[u8] = small_vec.borrow();
                    if !slice_ref.is_empty() {
                        println!("{}", slice_ref[0]);
                    }
                }
                10 => {
                    let slice_ref = small_vec.as_slice();
                    if !slice_ref.is_empty() {
                        println!("{}", slice_ref[0]);
                    }
                }
                _ => {
                    small_vec.clear();
                }
            }
        }

        let ref_small_vec = &small_vec;
        let mut iter_ref = ref_small_vec.into_iter();
        for _ in 0..3 {
            if let Some(v) = iter_ref.next() {
                println!("{}", v);
            }
        }

        let _ = small_vec.clone().into_vec();
        small_vec.shrink_to_fit();
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