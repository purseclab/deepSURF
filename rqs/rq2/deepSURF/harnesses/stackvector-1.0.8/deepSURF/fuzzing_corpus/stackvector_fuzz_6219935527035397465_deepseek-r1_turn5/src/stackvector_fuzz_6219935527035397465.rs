#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use stackvector::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 4096 {return;}
        set_global_data(data);
        let mut offset = 0;
        
        let constructor = _to_u8(get_global_data().first_half, offset) % 5;
        offset += 1;
        
        let mut vec = match constructor {
            0 => {
                let elem_count = _to_usize(get_global_data().first_half, offset) % 64;
                offset += 8;
                let mut v = StackVec::<[String; 128]>::new();
                for _ in 0..elem_count {
                    let len = _to_u8(get_global_data().first_half, offset) % 32;
                    let s = _to_str(get_global_data().first_half, offset + 1, offset + 1 + len as usize);
                    v.push(s.to_string());
                    offset += len as usize + 1;
                }
                v
            }
            1 => {
                let elem = _to_str(get_global_data().first_half, offset, offset + 16).to_string();
                StackVec::from_elem(elem, _to_usize(get_global_data().first_half, offset + 16) % 64)
            }
            2 => {
                let slice_data = _to_str(get_global_data().first_half, offset, offset + 128);
                let items: Vec<String> = slice_data.split_whitespace().map(|s| s.to_string()).collect();
                StackVec::from_iter(items)
            }
            3 => {
                let buf = _to_str(get_global_data().first_half, offset, offset + 256).as_bytes().to_vec();
                StackVec::from_vec(buf.into_iter().map(|b| b.to_string()).collect())
            }
            _ => StackVec::new(),
        };
        
        let op_count = _to_u8(get_global_data().second_half, 0) % 32 + 1;
        
        for op_idx in 0..op_count {
            let op_select = _to_u8(get_global_data().second_half, 1 + op_idx as usize) % 10;
            match op_select {
                0 => {
                    let idx = _to_usize(get_global_data().second_half, 128);
                    let s = _to_str(get_global_data().second_half, 136, 144);
                    vec.insert(idx, s.to_string());
                }
                1 => {
                    let idx = _to_usize(get_global_data().second_half, 144);
                    if idx < vec.len() {
                        let _ = vec.remove(idx);
                    }
                }
                2 => {
                    let new_len = _to_usize(get_global_data().second_half, 152);
                    vec.truncate(new_len);
                }
                3 => {
                    if let Some(e) = vec.pop() {
                        println!("{:?}", e);
                    }
                }
                4 => {
                    let slice = vec.as_mut();
                    println!("{:?}", slice);
                    let _ = vec.dedup();
                }
                5 => {
                    let slice = vec.as_ref();
                    if !slice.is_empty() {
                        let mid = _to_usize(get_global_data().second_half, 160) % slice.len();
                        println!("{:?}", &slice[..mid]);
                    }
                }
                6 => {
                    let idx = _to_usize(get_global_data().second_half, 168);
                    let value = _to_str(get_global_data().second_half, 176, 184).to_string();
                    if idx <= vec.len() {
                        vec.insert(idx, value);
                    }
                }
                7 => {
                    let other_slice = vec.as_slice();
                    let cmp = vec.partial_cmp(&vec);
                    println!("{:?}", cmp);
                }
                8 => {
                    let mut cloned = vec.clone();
                    cloned.extend(vec.iter().cloned());
                    vec = cloned;
                }
                _ => {
                    vec.clear();
                }
            }
        }
        
        let view = vec.as_mut();
        if !view.is_empty() {
            view.reverse();
            let idx = _to_usize(get_global_data().second_half, 192) % view.len();
            println!("{:?}", view[idx]);
        }
        
        let drained: Vec<_> = vec.drain().collect();
        println!("Drained {} items", drained.len());
        
        if let Ok(inner) = vec.into_inner() {
            let mut new_vec = StackVec::from_buf(inner);
            new_vec.push("final".to_string());
            println!("New vec len: {}", new_vec.len());
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