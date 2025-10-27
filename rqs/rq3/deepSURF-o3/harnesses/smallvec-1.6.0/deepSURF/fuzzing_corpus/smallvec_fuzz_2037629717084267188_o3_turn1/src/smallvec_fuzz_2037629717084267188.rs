#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 140 {
            return;
        }
        set_global_data(data);
        let global_data = get_global_data();
        let gd = global_data.first_half;

        let selector = _to_u8(gd, 0);
        let capacity = _to_usize(gd, 1);

        let mut buf16 = [0u8; 16];
        let mut buf32 = [0u8; 32];
        for i in 0..16 {
            buf16[i] = _to_u8(gd, 9 + i);
        }
        for i in 0..32 {
            buf32[i] = _to_u8(gd, 25 + i);
        }

        let mut sv1: SmallVec<[u8; 32]> = match selector % 4 {
            0 => SmallVec::<[u8; 32]>::new(),
            1 => SmallVec::<[u8; 32]>::with_capacity(capacity),
            2 => {
                let len = _to_usize(gd, 57) % 32;
                SmallVec::<[u8; 32]>::from_buf_and_len(buf32, len)
            }
            _ => {
                let len = _to_usize(gd, 65) % 16;
                SmallVec::<[u8; 32]>::from_slice(&buf16[..len])
            }
        };

        let op_cnt = (_to_u8(gd, 73) % 20) as usize;
        for i in 0..op_cnt {
            let op_selector = _to_u8(gd, 74 + i) % 10;
            match op_selector {
                0 => {
                    let val = _to_u8(gd, (90 + i) % gd.len());
                    sv1.push(val);
                }
                1 => {
                    sv1.pop();
                }
                2 => {
                    if !sv1.is_empty() {
                        let idx_src = (95 + i) % (gd.len() - 8);
                        let idx = _to_usize(gd, idx_src) % sv1.len();
                        let removed = sv1.remove(idx);
                        println!("{:?}", removed);
                    }
                }
                3 => {
                    let add_src = (100 + i) % (gd.len() - 8);
                    let additional = _to_usize(gd, add_src);
                    let _ = sv1.try_reserve(additional);
                }
                4 => {
                    let idx_src = (105 + i) % (gd.len() - 8);
                    let idx = _to_usize(gd, idx_src);
                    let val = _to_u8(gd, (110 + i) % gd.len());
                    if idx <= sv1.len() {
                        sv1.insert(idx, val);
                    }
                }
                5 => {
                    if !sv1.is_empty() {
                        let slice_ref = sv1.as_slice();
                        let first_ref = &slice_ref[0];
                        println!("{:?}", *first_ref);
                    }
                }
                6 => {
                    sv1.clear();
                }
                7 => {
                    let len_src = (115 + i) % (gd.len() - 8);
                    let new_len = _to_usize(gd, len_src) % 65;
                    let fill_val = _to_u8(gd, (120 + i) % gd.len());
                    sv1.resize(new_len, fill_val);
                }
                8 => {
                    if sv1.len() > 1 {
                        let _ = sv1.swap_remove(sv1.len() - 1);
                    }
                }
                _ => {
                    let boxed = sv1.clone().into_boxed_slice();
                    println!("{:?}", boxed.deref());
                }
            }
        }

        let elem = _to_u8(gd, 130);
        let count = _to_usize(gd, 131) % 65;
        let mut sv2: SmallVec<[u8; 32]> = SmallVec::from_elem(elem, count);

        let _order = sv1.partial_cmp(&sv2);
        let _eq = sv1.eq(&sv2);

        let mut sv1_clone = sv1.clone();
        sv1_clone.append(&mut sv2);
        std::mem::drop(sv1_clone);

        if !sv2.is_empty() {
            let mut drained = sv2.drain(..);
            while let Some(v) = drained.next() {
                println!("{:?}", v);
            }
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