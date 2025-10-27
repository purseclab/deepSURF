#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

fn build_initial_smallvec() -> SmallVec<[u8; 32]> {
    let gd = get_global_data().first_half;
    match _to_u8(gd, 0) % 6 {
        0 => SmallVec::<[u8; 32]>::new(),
        1 => {
            let cap = _to_usize(gd, 1);
            SmallVec::<[u8; 32]>::with_capacity(cap)
        }
        2 => {
            let mut buf = [0u8; 32];
            for i in 0..32 {
                buf[i] = _to_u8(gd, 1 + i);
            }
            SmallVec::from_buf(buf)
        }
        3 => {
            let mut buf = [0u8; 32];
            for i in 0..32 {
                buf[i] = _to_u8(gd, 33 + i);
            }
            let len = _to_usize(gd, 65);
            SmallVec::from_buf_and_len(buf, len)
        }
        4 => {
            let vec_len = (_to_u8(gd, 97) as usize) % 65;
            let mut v = Vec::with_capacity(vec_len);
            for i in 0..vec_len {
                v.push(_to_u8(gd, 98 + i));
            }
            SmallVec::from_vec(v)
        }
        _ => {
            let elem = _to_u8(gd, 163);
            let n = _to_usize(gd, 164);
            SmallVec::from_elem(elem, n)
        }
    }
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 256 {
            return;
        }
        set_global_data(data);
        let mut sv = build_initial_smallvec();
        let mut sv2 = sv.clone();
        let ops = _to_u8(data, 0) % 20;
        for i in 0..ops {
            let code = _to_u8(data, 1 + i as usize);
            match code % 12 {
                0 => {
                    let val = _to_u8(data, 2 + i as usize);
                    sv.push(val);
                }
                1 => {
                    sv.pop();
                }
                2 => {
                    let idx = _to_usize(data, 3 + i as usize);
                    let val = _to_u8(data, 4 + i as usize);
                    if sv.len() > 0 {
                        sv.insert(idx, val);
                    }
                }
                3 => {
                    let idx = _to_usize(data, 5 + i as usize);
                    if !sv.is_empty() {
                        sv.remove(idx);
                    }
                }
                4 => {
                    let len = _to_usize(data, 6 + i as usize);
                    sv.truncate(len);
                }
                5 => {
                    let slice_ref = sv.as_slice();
                    if !slice_ref.is_empty() {
                        println!("{}", slice_ref[0]);
                    }
                }
                6 => {
                    let _ = sv.len();
                }
                7 => {
                    let _ = sv.capacity();
                }
                8 => {
                    let ptr = sv.as_ptr();
                    println!("{:p}", ptr);
                }
                9 => {
                    if !sv.is_empty() {
                        let end = _to_usize(data, 7 + i as usize);
                        let _ = sv.drain(0..end);
                    }
                }
                10 => {
                    sv.clear();
                }
                _ => {
                    sv.append(&mut sv2);
                }
            }
        }
        let _final_ptr = sv.as_ptr();
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