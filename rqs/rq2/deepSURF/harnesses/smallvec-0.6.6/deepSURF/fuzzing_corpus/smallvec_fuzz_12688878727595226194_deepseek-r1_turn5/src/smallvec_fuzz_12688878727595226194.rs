#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 1024 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let operations = _to_u8(global_data.first_half, 0) % 10;
        
        for i in 0..operations {
            let op_selector = _to_u8(global_data.second_half, i as usize) % 6;
            match op_selector {
                0 => {
                    let buf_start = 100;
                    const ARRAY_REPEAT_VALUE: String = String::new();
                    let mut array = [ARRAY_REPEAT_VALUE; 16];
                    for idx in 0..16 {
                        let len = _to_u8(global_data.first_half, buf_start + idx * 3) % 20;
                        let s = _to_str(global_data.first_half, buf_start + idx * 3 + 1, buf_start + idx * 3 + 1 + len as usize);
                        array[idx] = s.to_string();
                    }
                    let len = _to_usize(global_data.first_half, buf_start + 48) % 16;
                    let mut sv = SmallVec::<[String; 16]>::from_buf_and_len(array, len);
                    println!("{:?}", sv.as_slice());
                    
                    let new_len = _to_usize(global_data.second_half, 50);
                    sv.resize(new_len, String::from("resize_element"));
                },
                1 => {
                    let capacity = _to_usize(global_data.second_half, 10);
                    let mut sv = SmallVec::<[String; 16]>::with_capacity(capacity);
                    let pushes = _to_usize(global_data.second_half, 20) % 65;
                    for p in 0..pushes {
                        sv.push(format!("push_{}", p));
                    }
                    sv.truncate(_to_usize(global_data.second_half, 30) % (sv.len() + 1));
                },
                2 => {
                    let mut sv = SmallVec::<[String; 16]>::new();
                    let insert_pos = _to_usize(global_data.second_half, 40) % 16;
                    let insert_val = _to_str(global_data.second_half, 50, 60);
                    sv.insert(insert_pos, insert_val.to_string());
                    
                    if let Some(first) = sv.first() {
                        println!("First element: {}", first);
                    }
                },
                3 => {
                    let slice_start = 200;
                    let mut temp_vec = Vec::new();
                    let mut pos = slice_start;
                    while pos < global_data.first_half.len().saturating_sub(4) {
                        let len = _to_u8(global_data.first_half, pos) % 30;
                        let s = _to_str(global_data.first_half, pos + 1, pos + 1 + len as usize);
                        temp_vec.push(s.to_string());
                        pos += 1 + len as usize;
                    }
                    let sv = SmallVec::<[String; 16]>::from_vec(temp_vec);
                    let cloned = sv.clone();
                    let ord = sv.cmp(&cloned);
                    println!("Comparison: {:?}", ord);
                },
                4 => {
                    let mut sv = SmallVec::<[String; 32]>::from_elem("elem".to_string(), _to_usize(global_data.second_half, 70) % 32);
                    sv.dedup();
                    println!("Dedup len: {}", sv.len());
                },
                5 => {
                    let mut sv1 = SmallVec::<[String; 16]>::new();
                    let mut sv2 = SmallVec::<[String; 16]>::new();
                    for i in 0..8 {
                        sv1.push(format!("sv1_{}", i));
                        sv2.push(format!("sv2_{}", i));
                    }
                    let partial_cmp = sv1.partial_cmp(&sv2);
                    println!("Partial cmp: {:?}", partial_cmp);
                },
                _ => unreachable!()
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