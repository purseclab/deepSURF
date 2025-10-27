#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::ops::{Deref, DerefMut};

fn pick_u8(slice: &[u8], cursor: &mut usize) -> u8 {
    let pos = *cursor % slice.len();
    *cursor += 1;
    _to_u8(slice, pos)
}

fn pick_usize(slice: &[u8], cursor: &mut usize) -> usize {
    let pos = *cursor % (slice.len() - 8);
    *cursor += 8;
    _to_usize(slice, pos)
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 200 {
            return;
        }
        set_global_data(data);
        let global = get_global_data();
        let gd = global.first_half;
        let mut cursor: usize = 0;

        let constructor_selector = pick_u8(gd, &mut cursor);
        let mut sv: SmallVec<[u8; 32]> = match constructor_selector % 6 {
            0 => SmallVec::<[u8; 32]>::new(),
            1 => {
                let cap = pick_usize(gd, &mut cursor) % 65;
                SmallVec::<[u8; 32]>::with_capacity(cap)
            }
            2 => {
                let elem = pick_u8(gd, &mut cursor);
                let n = pick_usize(gd, &mut cursor) % 65;
                SmallVec::<[u8; 32]>::from_elem(elem, n)
            }
            3 => {
                let mut buf = [0u8; 32];
                for i in 0..32 {
                    buf[i] = pick_u8(gd, &mut cursor);
                }
                let len = pick_usize(gd, &mut cursor) % 32;
                SmallVec::<[u8; 32]>::from_buf_and_len(buf, len)
            }
            4 => {
                let slice_len = (pick_u8(gd, &mut cursor) as usize % 65) + 1;
                let mut tmp = Vec::with_capacity(slice_len);
                for _ in 0..slice_len {
                    tmp.push(pick_u8(gd, &mut cursor));
                }
                SmallVec::<[u8; 32]>::from_slice(&tmp)
            }
            _ => {
                let vec_len = (pick_u8(gd, &mut cursor) as usize % 65) + 1;
                let mut tmp = Vec::with_capacity(vec_len);
                for _ in 0..vec_len {
                    tmp.push(pick_u8(gd, &mut cursor));
                }
                SmallVec::<[u8; 32]>::from_vec(tmp)
            }
        };

        let operations = (pick_u8(gd, &mut cursor) % 10) as usize + 1;
        for i in 0..operations {
            let op_code = pick_u8(gd, &mut cursor) % 10;
            match op_code {
                0 => {
                    let val = pick_u8(gd, &mut cursor);
                    sv.push(val);
                }
                1 => {
                    let _ = sv.pop();
                }
                2 => {
                    if !sv.is_empty() {
                        let idx = pick_usize(gd, &mut cursor);
                        let _ = sv.remove(idx);
                    }
                }
                3 => {
                    let idx = pick_usize(gd, &mut cursor);
                    let val = pick_u8(gd, &mut cursor);
                    sv.insert(idx, val);
                }
                4 => {
                    let slice = sv.as_mut_slice();
                    if !slice.is_empty() {
                        slice[0] = slice[0].wrapping_add(1);
                    }
                    println!("{:?}", slice);
                }
                5 => {
                    let _ = sv.capacity();
                }
                6 => {
                    let additional = pick_usize(gd, &mut cursor);
                    sv.reserve(additional);
                }
                7 => {
                    let len = pick_usize(gd, &mut cursor);
                    sv.truncate(len);
                }
                8 => {
                    sv.dedup();
                }
                _ => {
                    sv.clear();
                }
            }
            let _ = sv.as_mut_slice();
        }

        let final_slice = sv.as_slice();
        println!("{:?}", final_slice);
        let vec_output = sv.clone().into_vec();
        println!("{:?}", vec_output.len());
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