#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::ops::{Deref, DerefMut, Index, IndexMut};

fn build_smallvec(data: &[u8]) -> SmallVec<[u8; 32]> {
    let selector = _to_u8(data, 0);
    match selector % 5 {
        0 => SmallVec::<[u8; 32]>::new(),
        1 => {
            let cap = _to_usize(data, 1);
            SmallVec::<[u8; 32]>::with_capacity(cap)
        }
        2 => {
            let elem = _to_u8(data, 1);
            let n = _to_usize(data, 2);
            SmallVec::<[u8; 32]>::from_elem(elem, n)
        }
        3 => {
            let slice_start = 1;
            let slice_len = (_to_u8(data, 1) as usize) % 65;
            let slice_end = std::cmp::min(slice_start + slice_len, data.len());
            let slice = &data[slice_start..slice_end];
            SmallVec::<[u8; 32]>::from_slice(slice)
        }
        _ => {
            let mut buf = [0u8; 32];
            for i in 0..32 {
                buf[i] = _to_u8(data, 1 + i);
            }
            let len = _to_usize(data, 33);
            SmallVec::<[u8; 32]>::from_buf_and_len(buf, len)
        }
    }
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 100 {
            return;
        }
        set_global_data(data);
        let _ = get_global_data();
        let mut vec = build_smallvec(data);
        vec.is_empty();
        let ops_count = (_to_u8(data, 90) % 20) as usize;
        let mut offset = 91;
        for _ in 0..ops_count {
            let opcode = _to_u8(data, offset);
            offset += 1;
            match opcode % 10 {
                0 => {
                    vec.push(_to_u8(data, offset));
                }
                1 => {
                    vec.pop();
                }
                2 => {
                    let idx = _to_usize(data, offset);
                    vec.insert(idx, _to_u8(data, offset + 8));
                }
                3 => {
                    let idx = _to_usize(data, offset);
                    if vec.len() > 0 {
                        vec.remove(idx % vec.len());
                    }
                }
                4 => {
                    let new_len = _to_usize(data, offset);
                    vec.truncate(new_len);
                }
                5 => {
                    let additional = _to_usize(data, offset);
                    vec.reserve(additional);
                }
                6 => {
                    let n = _to_usize(data, offset);
                    vec.resize(n, _to_u8(data, offset + 8));
                }
                7 => {
                    let slice_len = (vec.len() % 65) as usize;
                    let slice_end = std::cmp::min(offset + slice_len, data.len());
                    let slice = &data[offset..slice_end];
                    vec.extend_from_slice(slice);
                }
                8 => {
                    let idx = _to_usize(data, offset);
                    let slice_len = (_to_u8(data, offset + 8) as usize) % 65;
                    let slice_start = offset + 9;
                    let slice_end = std::cmp::min(slice_start + slice_len, data.len());
                    if slice_end > slice_start {
                        let slice = &data[slice_start..slice_end];
                        vec.insert_from_slice(idx, slice);
                    }
                }
                _ => {
                    vec.clear();
                }
            }
            vec.is_empty();
            offset = (offset + 16) % (data.len() - 16);
        }
        let slice_ref = vec.as_slice();
        println!("{:?}", &*slice_ref);
        vec.is_empty();
        let first_ref = &vec;
        let _ = first_ref.is_empty();
        let _deref_slice: &[u8] = &*first_ref;
        let cloned = vec.clone();
        let ordering = vec.cmp(&cloned);
        println!("{:?}", ordering);
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