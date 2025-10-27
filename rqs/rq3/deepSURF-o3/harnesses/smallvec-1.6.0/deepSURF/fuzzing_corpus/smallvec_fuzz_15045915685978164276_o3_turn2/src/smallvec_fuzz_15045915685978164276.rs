#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 100 {
            return;
        }
        set_global_data(data);
        let gd = get_global_data();
        let GLOBAL_DATA = gd.first_half;
        let len_g = GLOBAL_DATA.len();

        let mut idx = 0usize;
        fn next(idx: &mut usize, step: usize, len: usize) -> usize {
            let i = *idx % len;
            *idx = idx.wrapping_add(step);
            i
        }

        let arr_bytes: [u8; 32] = core::array::from_fn(|i| _to_u8(GLOBAL_DATA, (i + 1) % len_g));
        let variant = _to_u8(GLOBAL_DATA, next(&mut idx, 1, len_g)) % 6;
        let mut sv: SmallVec<[u8; 32]> = match variant {
            0 => SmallVec::new(),
            1 => {
                let start = _to_u8(GLOBAL_DATA, next(&mut idx, 1, len_g)) as usize % len_g;
                let end = start + (_to_u8(GLOBAL_DATA, next(&mut idx, 1, len_g)) as usize % 65);
                let slice_end = end.min(len_g);
                SmallVec::from_slice(&GLOBAL_DATA[start..slice_end])
            }
            2 => SmallVec::from_buf(arr_bytes),
            3 => {
                let len = _to_u8(GLOBAL_DATA, next(&mut idx, 1, len_g)) as usize % 32;
                SmallVec::from_buf_and_len(arr_bytes, len)
            }
            4 => {
                let elem = _to_u8(GLOBAL_DATA, next(&mut idx, 1, len_g));
                let n = _to_u8(GLOBAL_DATA, next(&mut idx, 1, len_g)) as usize % 65;
                SmallVec::from_elem(elem, n)
            }
            _ => {
                let cap = _to_u8(GLOBAL_DATA, next(&mut idx, 1, len_g)) as usize % 65;
                SmallVec::with_capacity(cap)
            }
        };

        let pre_ops = _to_u8(GLOBAL_DATA, next(&mut idx, 1, len_g)) % 3;
        for _ in 0..pre_ops {
            match _to_u8(GLOBAL_DATA, next(&mut idx, 1, len_g)) % 3 {
                0 => { let _ = sv.capacity(); }
                1 => sv.shrink_to_fit(),
                _ => { let _ = sv.len(); }
            }
        }

        let operations = (_to_u8(GLOBAL_DATA, next(&mut idx, 1, len_g)) % 10) + 1;
        for _ in 0..operations {
            match _to_u8(GLOBAL_DATA, next(&mut idx, 1, len_g)) % 8 {
                0 => {
                    let val = _to_u8(GLOBAL_DATA, next(&mut idx, 1, len_g));
                    sv.push(val);
                }
                1 => { let _ = sv.pop(); }
                2 => {
                    let additional = _to_u8(GLOBAL_DATA, next(&mut idx, 1, len_g)) as usize;
                    sv.reserve(additional);
                }
                3 => {
                    if !sv.is_empty() {
                        let idx_rem = (_to_u8(GLOBAL_DATA, next(&mut idx, 1, len_g)) as usize) % sv.len();
                        let _ = sv.remove(idx_rem);
                    }
                }
                4 => sv.clear(),
                5 => {
                    let val = _to_u8(GLOBAL_DATA, next(&mut idx, 1, len_g));
                    let n = (_to_u8(GLOBAL_DATA, next(&mut idx, 1, len_g)) as usize) % 65;
                    sv.extend(core::iter::repeat(val).take(n));
                }
                6 => {
                    let len_new = _to_u8(GLOBAL_DATA, next(&mut idx, 1, len_g)) as usize % 65;
                    let fill = _to_u8(GLOBAL_DATA, next(&mut idx, 1, len_g));
                    sv.resize(len_new, fill);
                }
                _ => {
                    let start = _to_u8(GLOBAL_DATA, next(&mut idx, 1, len_g)) as usize % len_g;
                    let end = start + (_to_u8(GLOBAL_DATA, next(&mut idx, 1, len_g)) as usize % 65);
                    let slice_end = end.min(len_g);
                    sv.extend_from_slice(&GLOBAL_DATA[start..slice_end]);
                }
            }
        }

        println!("{:?}", sv.as_slice());

        let mut clone_sv = sv.clone();
        let iter = sv.into_iter();
        println!("{:?}", iter.as_slice());

        let mut iter_mut = iter.clone();
        if let Some(item) = iter_mut.next() {
            println!("{:?}", item);
        }
        if let Some(item) = iter_mut.next_back() {
            println!("{:?}", item);
        }

        let add_more = _to_u8(GLOBAL_DATA, next(&mut idx, 1, len_g)) % 5;
        for _ in 0..add_more {
            let val = _to_u8(GLOBAL_DATA, next(&mut idx, 1, len_g));
            clone_sv.push(val);
        }

        println!("{:?}", clone_sv.as_slice());
        let _vec = clone_sv.into_vec();
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