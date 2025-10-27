#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 614 {
            return;
        }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let mode = _to_u8(GLOBAL_DATA, 0) % 4;
        let mut small_vec: SmallVec<[u8; 32]> = match mode {
            0 => SmallVec::new(),
            1 => {
                let cap = _to_usize(GLOBAL_DATA, 8);
                SmallVec::with_capacity(cap)
            }
            2 => {
                let slice_len_raw = _to_u8(GLOBAL_DATA, 16) as usize;
                let slice_len = std::cmp::min(slice_len_raw, GLOBAL_DATA.len().saturating_sub(20));
                let slice = &GLOBAL_DATA[20..20 + slice_len];
                SmallVec::from_slice(slice)
            }
            _ => {
                let elem = _to_u8(GLOBAL_DATA, 28);
                let count = _to_usize(GLOBAL_DATA, 30);
                SmallVec::from_elem(elem, count)
            }
        };

        if !_to_bool(GLOBAL_DATA, 38) {
            let val = _to_u8(GLOBAL_DATA, 40);
            small_vec.push(val);
        }

        let slice2_len_raw = _to_u8(GLOBAL_DATA, 42) as usize;
        let slice2_len = std::cmp::min(slice2_len_raw, GLOBAL_DATA.len().saturating_sub(46));
        let slice2 = &GLOBAL_DATA[46..46 + slice2_len];
        small_vec.extend_from_slice(slice2);

        let ops = (_to_u8(GLOBAL_DATA, 52) % 10) as usize;
        for i in 0..ops {
            let sel = _to_u8(GLOBAL_DATA, 53 + i) % 6;
            match sel {
                0 => {
                    let idx = _to_usize(GLOBAL_DATA, 128 + i);
                    if small_vec.len() > 0 {
                        let _ = small_vec.remove(idx % small_vec.len());
                    }
                }
                1 => {
                    let additional = _to_usize(GLOBAL_DATA, 192 + i);
                    small_vec.reserve(additional);
                }
                2 => {
                    let truncate_to = _to_usize(GLOBAL_DATA, 224 + i);
                    small_vec.truncate(truncate_to);
                }
                3 => {
                    let val = _to_u8(GLOBAL_DATA, 256 + i);
                    small_vec.push(val);
                }
                4 => {
                    let elem = _to_u8(GLOBAL_DATA, 288 + i);
                    let count = _to_usize(GLOBAL_DATA, 320 + i);
                    small_vec.resize(count, elem);
                }
                _ => {
                    let slice_len_raw = _to_u8(GLOBAL_DATA, 352 + i) as usize;
                    let slice_len = std::cmp::min(slice_len_raw, GLOBAL_DATA.len().saturating_sub(384 + i));
                    let slice_dyn = &GLOBAL_DATA[384 + i..384 + i + slice_len];
                    small_vec.extend_from_slice(slice_dyn);
                }
            }
        }

        let second_half = global_data.second_half;
        if !second_half.is_empty() {
            let cmp_vec = SmallVec::<[u8; 32]>::from_slice(second_half);
            let _order = small_vec.cmp(&cmp_vec);
        }

        if let Some(first_ref) = small_vec.first() {
            println!("{:?}", *first_ref.deref());
        }

        let as_mut_slice = small_vec.as_mut_slice();
        if !as_mut_slice.is_empty() {
            as_mut_slice[0] = as_mut_slice[0].wrapping_add(1);
            println!("{:?}", as_mut_slice[0]);
        }

        let _vec_back = small_vec.into_vec();
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