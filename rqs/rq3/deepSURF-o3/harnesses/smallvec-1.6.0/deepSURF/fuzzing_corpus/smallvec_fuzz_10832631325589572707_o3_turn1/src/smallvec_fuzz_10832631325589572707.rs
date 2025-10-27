#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 130 {
            return;
        }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let selector = _to_u8(GLOBAL_DATA, 0) % 5;
        let mut smallv: SmallVec<[u8; 32]> = match selector {
            0 => SmallVec::new(),
            1 => {
                let mut slice_len = (_to_u8(GLOBAL_DATA, 1) as usize) % 32;
                if 2 + slice_len > GLOBAL_DATA.len() {
                    slice_len = GLOBAL_DATA.len() - 2;
                }
                let slice = &GLOBAL_DATA[2..2 + slice_len];
                SmallVec::<[u8; 32]>::from_slice(slice)
            }
            2 => {
                let vec_len = (_to_u8(GLOBAL_DATA, 1) as usize) % 32;
                let mut vec = Vec::with_capacity(vec_len);
                for i in 0..vec_len {
                    vec.push(_to_u8(GLOBAL_DATA, 2 + i % GLOBAL_DATA.len()));
                }
                SmallVec::<[u8; 32]>::from_vec(vec)
            }
            3 => {
                let elem = _to_u8(GLOBAL_DATA, 1);
                let count = (_to_u8(GLOBAL_DATA, 2) as usize) % 65;
                SmallVec::<[u8; 32]>::from_elem(elem, count)
            }
            _ => {
                let cap = (_to_u8(GLOBAL_DATA, 1) as usize) % 65;
                SmallVec::<[u8; 32]>::with_capacity(cap)
            }
        };
        let mut aux_cap = (_to_u8(GLOBAL_DATA, 64) as usize) % 65;
        if aux_cap == 0 {
            aux_cap = 1;
        }
        let mut aux = SmallVec::<[u8; 32]>::with_capacity(aux_cap);
        for i in 0..aux_cap {
            aux.push(_to_u8(GLOBAL_DATA, (65 + i) % GLOBAL_DATA.len()));
        }
        let mut idx: usize = 100;
        let op_count = (_to_u8(GLOBAL_DATA, 99) % 20) as usize;
        for _ in 0..op_count {
            if GLOBAL_DATA.is_empty() {
                break;
            }
            let op = _to_u8(GLOBAL_DATA, idx % GLOBAL_DATA.len()) % 12;
            idx = idx.wrapping_add(1);
            match op {
                0 => {
                    if smallv.len() < 65 {
                        let val = _to_u8(GLOBAL_DATA, idx % GLOBAL_DATA.len());
                        idx = idx.wrapping_add(1);
                        smallv.push(val);
                    }
                }
                1 => {
                    smallv.pop();
                }
                2 => {
                    if smallv.len() < 65 {
                        let pos = if smallv.is_empty() {
                            0
                        } else {
                            (_to_u8(GLOBAL_DATA, idx % GLOBAL_DATA.len()) as usize) % smallv.len()
                        };
                        idx = idx.wrapping_add(1);
                        let val = _to_u8(GLOBAL_DATA, idx % GLOBAL_DATA.len());
                        idx = idx.wrapping_add(1);
                        if pos <= smallv.len() {
                            smallv.insert(pos, val);
                        }
                    }
                }
                3 => {
                    smallv.len();
                }
                4 => {
                    smallv.is_empty();
                }
                5 => {
                    smallv.capacity();
                }
                6 => {
                    let slice = smallv.as_ref();
                    println!("{:?}", slice);
                }
                7 => {
                    let ref_slice: &[u8] = smallv.deref();
                    println!("{:?}", ref_slice);
                }
                8 => {
                    smallv.clone();
                }
                9 => {
                    let new_len = if smallv.is_empty() {
                        0
                    } else {
                        (_to_u8(GLOBAL_DATA, idx % GLOBAL_DATA.len()) as usize) % smallv.len()
                    };
                    idx = idx.wrapping_add(1);
                    smallv.truncate(new_len);
                }
                10 => {
                    if !smallv.is_empty() {
                        let index = (_to_u8(GLOBAL_DATA, idx % GLOBAL_DATA.len()) as usize) % smallv.len();
                        idx = idx.wrapping_add(1);
                        smallv.remove(index);
                    }
                }
                _ => {
                    smallv.clear();
                }
            }
        }
        smallv.append(&mut aux);
        let final_slice = smallv.as_ref();
        println!("{:?}", final_slice);
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