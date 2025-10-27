#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::ops::{Deref, DerefMut, Index, IndexMut};
use std::str::FromStr;

fn build_array(bytes: &[u8]) -> [u8; 16] {
    let mut buf = [0u8; 16];
    for (i, b) in buf.iter_mut().enumerate() {
        *b = bytes.get(i).copied().unwrap_or(0);
    }
    buf
}

fn construct_smallvec(choice: u8, buf: [u8; 16], slice: &[u8], elem: u8, len: usize) -> SmallVec<[u8; 16]> {
    match choice % 5 {
        0 => SmallVec::<[u8; 16]>::new(),
        1 => SmallVec::<[u8; 16]>::with_capacity(len),
        2 => SmallVec::<[u8; 16]>::from_slice(slice),
        3 => SmallVec::<[u8; 16]>::from_elem(elem, len),
        _ => SmallVec::<[u8; 16]>::from_buf_and_len(buf, len.min(16)),
    }
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 96 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let gd = global_data.first_half;

        let ctor_choice = _to_u8(gd, 0);
        let op_count = (_to_u8(gd, 1) % 10) as usize;
        let len_param = (_to_usize(gd, 2) % 65) as usize;
        let elem = _to_u8(gd, 10);
        let buf = build_array(&gd[11..27]);
        let slice_end = 27 + len_param.min(16);
        let slice = &gd[27..slice_end];

        let mut vec1 = construct_smallvec(ctor_choice, buf, slice, elem, len_param);

        for i in 0..op_count {
            let selector = _to_u8(gd, 40 + i) % 12;
            match selector {
                0 => {
                    let val = _to_u8(gd, 52 + i);
                    vec1.push(val);
                }
                1 => {
                    vec1.pop();
                }
                2 => {
                    let idx = _to_usize(gd, 60) % (vec1.len() + 1).max(1);
                    vec1.insert(idx, _to_u8(gd, 68 + i));
                }
                3 => {
                    if !vec1.is_empty() {
                        let idx = _to_usize(gd, 70) % vec1.len();
                        vec1.remove(idx);
                    }
                }
                4 => {
                    let extra_slice_end = 80 + len_param.min(16);
                    let extra_slice = &gd[80..extra_slice_end];
                    vec1.extend_from_slice(extra_slice);
                }
                5 => {
                    let cap = _to_usize(gd, 40) % 65;
                    vec1.reserve(cap);
                }
                6 => {
                    vec1.truncate(_to_usize(gd, 32) % (vec1.len() + 1));
                }
                7 => {
                    vec1.clear();
                }
                8 => {
                    let _ = vec1.clone();
                }
                9 => {
                    let mut other = SmallVec::<[u8; 16]>::from_slice(slice);
                    let _ = vec1.cmp(&other);
                    other.append(&mut vec1);
                    vec1 = other;
                }
                10 => {
                    let _ = vec1.capacity();
                }
                _ => {
                    let _ = vec1.is_empty();
                }
            }
        }

        let slice_ref = vec1.as_slice();
        println!("{:?}", slice_ref);

        let first_clone = vec1.clone();
        let _ = first_clone.as_slice();

        if !vec1.is_empty() {
            println!("{}", slice_ref[0]);
        }

        vec1.as_slice();
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