#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use stack_dst::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 500 {
            return;
        }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let op_count = _to_usize(GLOBAL_DATA, 0) % 50;
        let mut current_idx = 1;

        let mut stack = stack_dst::StackA::<str, [usize; 12]>::new();

        for _ in 0..op_count {
            if current_idx + 2 > GLOBAL_DATA.len() {
                break;
            }
            let op_sel = _to_u8(GLOBAL_DATA, current_idx) % 5;
            current_idx += 1;

            match op_sel {
                0 => {
                    if current_idx + 9 > GLOBAL_DATA.len() {
                        continue;
                    }
                    let start = _to_usize(GLOBAL_DATA, current_idx) % (GLOBAL_DATA.len() - 64);
                    current_idx += 8;
                    let len = _to_u8(GLOBAL_DATA, current_idx) % 65;
                    current_idx += 1;
                    let s = _to_str(GLOBAL_DATA, start, start + len as usize);
                    let _ = stack.push_str(s);
                }
                1 => {
                    if current_idx + 1 > GLOBAL_DATA.len() {
                        continue;
                    }
                    let cnt = _to_u8(GLOBAL_DATA, current_idx) % 65;
                    current_idx += 1;
                    for _ in 0..cnt {
                        if current_idx + 2 > GLOBAL_DATA.len() {
                            break;
                        }
                        let slen = _to_u8(GLOBAL_DATA, current_idx) % 65;
                        current_idx += 1;
                        let s = _to_str(
                            GLOBAL_DATA,
                            current_idx,
                            current_idx + slen as usize,
                        );
                        current_idx += slen as usize;
                        let s_str = s.to_string();
                        let _ = stack.push_stable(s_str, |x| x.as_str());
                    }
                }
                2 => {
                    stack.pop();
                }
                3 => {
                    if let Some(top) = stack.top() {
                        println!("{:?}", top);
                    }
                }
                4 => {
                    let _ = stack.top_mut().map(|v| {
                        println!("{:?}", v);
                    });
                }
                _ => unreachable!(),
            }
        }

        let _ = stack.top_mut();
    });
}

// _to_u8 to _unwrap_result functions remain unchanged but are omitted as per direction II.2 and III.1.b

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