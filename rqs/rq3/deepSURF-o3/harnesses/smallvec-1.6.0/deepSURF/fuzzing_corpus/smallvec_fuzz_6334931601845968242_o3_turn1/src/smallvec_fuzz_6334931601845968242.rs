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
        let global_data = get_global_data();
        let first = global_data.first_half;
        let second = global_data.second_half;
        let constructor_choice = _to_u8(first, 0) % 5;
        let mut vec_len = (_to_u8(first, 1) % 65) as usize;
        if vec_len == 0 {
            vec_len = 1;
        }
        let mut initial_vec: Vec<u8> = second[..vec_len.min(second.len())].to_vec();
        let mut small: SmallVec<[u8; 32]> = match constructor_choice {
            0 => SmallVec::<[u8; 32]>::new(),
            1 => {
                let cap = _to_usize(first, 2);
                SmallVec::<[u8; 32]>::with_capacity(cap)
            }
            2 => {
                if second.len() >= 32 {
                    let mut buf = [0u8; 32];
                    buf.copy_from_slice(&second[..32]);
                    let len = _to_usize(first, 10);
                    SmallVec::from_buf_and_len(buf, len)
                } else {
                    SmallVec::<[u8; 32]>::new()
                }
            }
            3 => {
                let slice_end = vec_len.min(second.len());
                SmallVec::from_slice(&second[..slice_end])
            }
            _ => SmallVec::from_vec(initial_vec.clone()),
        };
        let ops = (_to_u8(first, 18) % 20) as usize;
        let mut cursor = 0usize;
        for _ in 0..ops {
            if cursor >= second.len() {
                break;
            }
            let op = _to_u8(second, cursor);
            cursor += 1;
            match op % 8 {
                0 => {
                    if cursor < second.len() {
                        let val = _to_u8(second, cursor);
                        cursor += 1;
                        small.push(val);
                    }
                }
                1 => {
                    if cursor + 8 <= second.len() {
                        let idx = _to_usize(second, cursor);
                        cursor += 8;
                        let _ = small.remove(idx);
                    }
                }
                2 => {
                    let _cap = small.capacity();
                }
                3 => {
                    if cursor + 8 <= second.len() {
                        let len = _to_usize(second, cursor);
                        cursor += 8;
                        small.truncate(len);
                    }
                }
                4 => {
                    if cursor < second.len() {
                        let sl_len = (_to_u8(second, cursor) % 65) as usize;
                        cursor += 1;
                        let end = cursor + sl_len;
                        if end <= second.len() {
                            let slice = &second[cursor..end];
                            cursor = end;
                            small.extend_from_slice(slice);
                        }
                    }
                }
                5 => {
                    if cursor + 8 <= second.len() {
                        let idx = _to_usize(second, cursor);
                        cursor += 8;
                        let _ = small.swap_remove(idx);
                    }
                }
                6 => {
                    if cursor + 9 <= second.len() {
                        let idx = _to_usize(second, cursor);
                        cursor += 8;
                        let val = _to_u8(second, cursor);
                        cursor += 1;
                        small.insert(idx, val);
                    }
                }
                _ => {
                    small.clear();
                }
            }
        }
        if small.len() > 0 {
            let idx_final = _to_usize(first, 26);
            let _ = small.swap_remove(idx_final);
        }
        let slice_ref = small.as_slice();
        println!("{:?}", slice_ref.len());
        let mut_slice = small.as_mut_slice();
        println!("{:?}", mut_slice.len());
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