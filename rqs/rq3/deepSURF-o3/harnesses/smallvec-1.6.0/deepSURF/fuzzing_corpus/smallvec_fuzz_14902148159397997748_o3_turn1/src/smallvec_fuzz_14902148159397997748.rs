#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::ops::{Deref, DerefMut, Index, IndexMut};
use std::str::FromStr;

fn build_smallvec(choice: u8, data: &[u8]) -> SmallVec<[u8; 16]> {
    let capacity_raw = _to_usize(data, 40);
    let capacity_vec = (capacity_raw % 65).max(1);
    let mut temp_vec: Vec<u8> = Vec::with_capacity(capacity_vec);
    for i in 0..capacity_vec {
        temp_vec.push(_to_u8(data, (41 + i) % data.len()));
    }
    let buf_len = (_to_u8(data, 100) as usize) % 16;
    let mut buf = [0u8; 16];
    for i in 0..16 {
        buf[i] = _to_u8(data, (101 + i) % data.len());
    }
    match choice % 5 {
        0 => SmallVec::<[u8; 16]>::new(),
        1 => SmallVec::<[u8; 16]>::with_capacity(capacity_raw),
        2 => SmallVec::<[u8; 16]>::from_slice(&temp_vec),
        3 => SmallVec::<[u8; 16]>::from_vec(temp_vec),
        _ => SmallVec::<[u8; 16]>::from_buf_and_len(buf, buf_len),
    }
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 140 {
            return;
        }
        set_global_data(data);
        let global_data = get_global_data();
        let gd = global_data.first_half;
        let constructor_choice = _to_u8(gd, 0);
        let mut sv1 = build_smallvec(constructor_choice, gd);
        let op_count = (_to_u8(gd, 20) % 32) as usize;
        let mut cursor: usize = 21;
        for _ in 0..op_count {
            if cursor >= gd.len() {
                break;
            }
            let op = _to_u8(gd, cursor);
            cursor += 1;
            match op % 12 {
                0 => {
                    let val = _to_u8(gd, cursor % gd.len());
                    cursor += 1;
                    sv1.push(val);
                }
                1 => {
                    sv1.pop();
                }
                2 => {
                    let idx = _to_usize(gd, cursor % (gd.len() - 8));
                    cursor += 8;
                    let val = _to_u8(gd, cursor % gd.len());
                    cursor += 1;
                    sv1.insert(idx, val);
                }
                3 => {
                    let idx = _to_usize(gd, cursor % (gd.len() - 8));
                    cursor += 8;
                    sv1.remove(idx);
                }
                4 => {
                    let new_len_raw = _to_usize(gd, cursor % (gd.len() - 8));
                    cursor += 8;
                    sv1.resize(new_len_raw, 0u8);
                }
                5 => {
                    sv1.clear();
                }
                6 => {
                    let additional = _to_usize(gd, cursor % (gd.len() - 8));
                    cursor += 8;
                    sv1.reserve(additional);
                }
                7 => {
                    sv1.shrink_to_fit();
                }
                8 => {
                    let range_start = _to_usize(gd, cursor % (gd.len() - 16));
                    let range_end = _to_usize(gd, (cursor + 8) % (gd.len() - 8));
                    cursor += 16;
                    let _drain = sv1.drain(range_start..range_end);
                }
                9 => {
                    let idx = _to_usize(gd, cursor % (gd.len() - 8));
                    cursor += 8;
                    let slice = [_to_u8(gd, cursor % gd.len()); 4];
                    cursor += 1;
                    sv1.insert_from_slice(idx, &slice);
                }
                10 => {
                    let slice = [_to_u8(gd, cursor % gd.len()); 6];
                    cursor += 1;
                    sv1.extend_from_slice(&slice);
                }
                _ => {
                    sv1.dedup();
                }
            }
        }
        let mut sv2 = build_smallvec(constructor_choice.wrapping_add(1), global_data.second_half);
        sv2.extend_from_slice(&sv1);
        let _cmp = sv1.cmp(&sv2);
        let _pcmp = sv1.partial_cmp(&sv2);
        let _eq = sv1.eq(&sv2);
        println!("{:?}", sv1.deref());
        println!("{:?}", sv2.deref());
        sv1.append(&mut sv2);
        let _ = sv1.deref();
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