#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::ops::{DerefMut, IndexMut};

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 512 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let mut vecs = vec![];
        let mut data_idx = 0;
        let mut op_count = _to_usize(GLOBAL_DATA, data_idx) % 65;
        data_idx += 8;

        for _ in 0..op_count {
            let op_selector = _to_u8(GLOBAL_DATA, data_idx) % 10;
            data_idx += 1;

            match op_selector {
                0 => {
                    let cap = _to_usize(GLOBAL_DATA, data_idx);
                    data_idx += 8;
                    vecs.push(SmallVec::<[String; 64]>::with_capacity(cap));
                }
                1 => {
                    let elem = _to_str(GLOBAL_DATA, data_idx, data_idx + 16).to_string();
                    let count = _to_usize(GLOBAL_DATA, data_idx + 16);
                    data_idx += 24;
                    vecs.push(SmallVec::from_elem(elem, count));
                }
                2 => {
                    let start = _to_usize(GLOBAL_DATA, data_idx);
                    let end = _to_usize(GLOBAL_DATA, data_idx + 8);
                    data_idx += 16;
                    let slice_str = _to_str(GLOBAL_DATA, start, end);
                    let strings: Vec<String> = slice_str.chars().map(|c| c.to_string()).collect();
                    vecs.push(SmallVec::from_vec(strings));
                }
                3 => {
                    if let Some(v) = vecs.last_mut() {
                        let elem = _to_str(GLOBAL_DATA, data_idx, data_idx + 4).to_string();
                        data_idx += 4;
                        v.push(elem);
                    }
                }
                4 => {
                    if let Some(v) = vecs.last_mut() {
                        v.pop();
                    }
                }
                5 => {
                    if let Some(v) = vecs.last_mut() {
                        v.deref_mut();
                        println!("{:?}", v.as_slice());
                    }
                }
                6 => {
                    if let Some(v) = vecs.last_mut() {
                        let idx = _to_usize(GLOBAL_DATA, data_idx);
                        data_idx += 8;
                        if !v.is_empty() {
                            v.swap_remove(idx % v.len());
                        }
                    }
                }
                7 => {
                    if let Some(v) = vecs.last_mut() {
                        let new_len = _to_usize(GLOBAL_DATA, data_idx);
                        data_idx += 8;
                        v.truncate(new_len);
                    }
                }
                8 => {
                    if let Some(v) = vecs.last_mut() {
                        let index = _to_usize(GLOBAL_DATA, data_idx);
                        let elem = _to_str(GLOBAL_DATA, data_idx + 8, data_idx + 12).to_string();
                        data_idx += 16;
                        if !v.is_empty() {
                            v.insert(index % v.len(), elem);
                        }
                    }
                }
                _ => {
                    let pos = _to_usize(GLOBAL_DATA, data_idx);
                    data_idx += 8;
                    vecs.truncate(pos % (vecs.len() + 1));
                }
            }

            if let Some(v) = vecs.last_mut() {
                v.shrink_to_fit();
                let _ = v.as_ptr();
                let _ = v.as_mut_ptr();
                let _ = v.capacity();
            }
        }

        let mut final_sv = SmallVec::<[String; 64]>::new();
        if op_count % 2 == 0 {
            let cap = _to_usize(GLOBAL_DATA, data_idx);
            data_idx += 8;
            final_sv = SmallVec::with_capacity(cap);
        } else {
            let elem = _to_str(GLOBAL_DATA, data_idx, data_idx + 8).to_string();
            data_idx += 8;
            final_sv = SmallVec::from_elem(elem, _to_usize(GLOBAL_DATA, data_idx));
        }

        for _ in 0.._to_usize(GLOBAL_DATA, data_idx) % 65 {
            final_sv.deref_mut();
            println!("{:?}", final_sv.as_mut_slice());
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