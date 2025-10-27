#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use stack_dst::*;
use global_data::*;
use std::ops::{Deref, DerefMut};

#[derive(Debug, Clone)]
struct CustomType0(String);

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 500 {
            return;
        }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let mut custom_stack = stack_dst::StackA::<CustomType0, [usize; 9]>::new();
        let mut str_stack = stack_dst::StackA::<str, [usize; 9]>::new();
        let mut slice_stack = stack_dst::StackA::<[CustomType0], [usize; 9]>::new();
        let ops_count = _to_u8(GLOBAL_DATA, 0) % 10 + 1;
        let mut idx = 1;
        for _ in 0..ops_count {
            if idx >= GLOBAL_DATA.len() {
                break;
            }
            let op = _to_u8(GLOBAL_DATA, idx) % 8;
            idx += 1;
            match op {
                0 => {
                    let len = _to_u8(GLOBAL_DATA, idx) as usize % 65;
                    idx += 1;
                    if idx + len > GLOBAL_DATA.len() {
                        idx = GLOBAL_DATA.len();
                        continue;
                    }
                    let s = _to_str(GLOBAL_DATA, idx, idx + len);
                    idx += len;
                    let _ = custom_stack.push_stable(CustomType0(s.to_string()), |c| c);
                }
                1 => custom_stack.pop(),
                2 => {
                    if let Some(t) = custom_stack.top() {
                        println!("{:?}", *t);
                    }
                }
                3 => {
                    if let Some(t) = custom_stack.top_mut() {
                        println!("{:?}", *t);
                        let len = _to_u8(GLOBAL_DATA, idx) as usize % 65;
                        idx += 1;
                        if idx + len <= GLOBAL_DATA.len() {
                            let s = _to_str(GLOBAL_DATA, idx, idx + len);
                            t.0.push_str(s);
                            idx += len;
                        }
                    }
                }
                4 => {
                    let len = _to_u8(GLOBAL_DATA, idx) as usize % 65;
                    idx += 1;
                    if idx + len > GLOBAL_DATA.len() {
                        idx = GLOBAL_DATA.len();
                        continue;
                    }
                    let s = _to_str(GLOBAL_DATA, idx, idx + len);
                    idx += len;
                    let _ = str_stack.push_str(s);
                }
                5 => {
                    let count = _to_u8(GLOBAL_DATA, idx) as usize % 65;
                    idx += 1;
                    let mut items = Vec::with_capacity(count);
                    for _ in 0..count {
                        if idx + 4 > GLOBAL_DATA.len() {
                            break;
                        }
                        let val = _to_u32(GLOBAL_DATA, idx) as usize;
                        idx += 4;
                        items.push(CustomType0(val.to_string()));
                    }
                    let _ = slice_stack.push_cloned(&items);
                }
                6 => {
                    let len = _to_u8(GLOBAL_DATA, idx) as usize % 65;
                    idx += 1;
                    if idx + len > GLOBAL_DATA.len() {
                        idx = GLOBAL_DATA.len();
                        continue;
                    }
                    let s = _to_str(GLOBAL_DATA, idx, idx + len);
                    idx += len;
                    let _ = str_stack.push_str(s);
                    if let Some(t) = str_stack.top() {
                        println!("{:?}", t);
                    }
                }
                7 => {
                    if let Some(t) = slice_stack.top() {
                        println!("{:?}", t.len());
                        if !t.is_empty() {
                            println!("{:?}", t[0]);
                        }
                    }
                }
                _ => (),
            }
        }
        let _ = custom_stack.top();
        let _ = str_stack.top();
        let _ = slice_stack.top();
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