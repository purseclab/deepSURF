#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use stackvector::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 8192 {return;}
        set_global_data(data);
        let global_data = get_global_data();
        let g = global_data.first_half;

        let op_count = _to_u8(g, 0) % 16;
        for i in 0..op_count {
            let selector = _to_u8(g, 1 + i as usize * 3) % 9;
            match selector {
                0 => {
                    let mut sv = StackVec::<[String; 64]>::new();
                    let push_count = _to_u8(g, 20 + i as usize * 5) % 65;
                    for _ in 0..push_count {
                        let s_len = _to_u8(g, 100 + i as usize * 7) % 65;
                        let start = 200 + i as usize * 11;
                        let end = start + s_len as usize;
                        let item = _to_str(g, start, end);
                        sv.push(item.to_string());
                        println!("Pushed: {:?}", sv.last().map(|s| s.as_str()));
                        if let Some(last) = sv.last_mut() {
                            *last = last.to_uppercase();
                        }
                    }
                    sv.truncate(_to_usize(g, 300) % (sv.len() + 1));
                }
                1 => {
                    let elem_count = _to_u8(g, 300 + i as usize * 13) % 65;
                    let fill_str = _to_str(g, 400 + i as usize * 17, 400 + i as usize * 17 + 16);
                    let sv = StackVec::<[String; 128]>::from_elem(fill_str.to_string(), elem_count as usize);
                    println!("From elem len: {}", sv.len());
                    let cmp_result = sv.partial_cmp(&sv);
                    println!("Comparison: {:?}", cmp_result);
                }
                2 => {
                    let mut sv = StackVec::<[String; 256]>::new();
                    println!("New vec capacity: {}", sv.capacity());
                    sv.extend((0.._to_u8(g, 500) % 65).map(|j| j.to_string()));
                    let _ = sv.drain();
                }
                3 => {
                    let slice_size = _to_u8(g, 500 + i as usize * 19) % 65;
                    let mut temp_vec = Vec::with_capacity(slice_size as usize);
                    for j in 0..slice_size {
                        let start = 600 + i as usize * 23 + j as usize * 11;
                        let s = _to_str(g, start, start + 8);
                        temp_vec.push(s.to_string());
                    }
                    let mut sv = StackVec::<[String; 32]>::from_vec(temp_vec);
                    println!("From slice: {:?}", sv.deref());
                    if !sv.is_empty() {
                        let idx = _to_usize(g, 700) % sv.len();
                        sv.swap_remove(idx);
                    }
                }
                4 => {
                    let mut temp_vec = Vec::new();
                    let vec_len = _to_u8(g, 700 + i as usize * 29) % 65;
                    for j in 0..vec_len {
                        let s_len = _to_u8(g, 800 + j as usize * 31) % 65;
                        let start = 900 + j as usize * 37;
                        let end = start + s_len as usize;
                        let s = _to_str(g, start, end);
                        temp_vec.push(s.to_string());
                    }
                    let mut sv = StackVec::<[String; 32]>::from_vec(temp_vec);
                    println!("From vec: {:?}", sv.as_slice());
                    if !sv.is_empty() {
                        let removed = sv.swap_remove(_to_u8(g, 1000) as usize % sv.len());
                        println!("Removed: {:?}", removed);
                        sv.insert(_to_usize(g, 1001) % (sv.len() + 1), removed);
                    }
                }
                5 => {
                    let mut temp_vec = Vec::new();
                    let vec_len = _to_u8(g, 1100 + i as usize * 3) % 65;
                    for j in 0..vec_len {
                        let s_len = _to_u8(g, 1200 + j as usize * 5) % 65;
                        let start = 1300 + j as usize * 7;
                        let end = start + s_len as usize;
                        let s = _to_str(g, start, end);
                        temp_vec.push(s.to_string());
                    }
                    let mut sv = StackVec::<[String; 64]>::from_vec(temp_vec);
                    if !sv.is_empty() {
                        let idx = _to_usize(g, 1400) % sv.len();
                        println!("Index {}: {:?}", idx, sv.index(idx));
                        *sv.index_mut(idx) = "modified".into();
                    }
                    let drain = sv.drain();
                    println!("Drained count: {}", drain.count());
                }
                6 => {
                    let a = StackVec::<[String; 32]>::from_elem("A".into(), 8);
                    let b = StackVec::<[String; 64]>::from_elem("B".into(), 16);
                    println!("Equality check: {}", a == b);
                }
                7 => {
                    let mut sv = StackVec::<[String; 128]>::new();
                    let insert_pos = _to_usize(g, 1500) % (sv.len() + 1);
                    sv.insert(insert_pos, "inserted".into());
                    sv.retain(|s| s.len() > 4);
                }
                8 => {
                    let mut sv = StackVec::<[String; 256]>::from_elem("X".into(), 32);
                    sv.dedup();
                    let clone = sv.clone();
                    println!("Clone len: {}", clone.len());
                }
                _ => {}
            }
        }

        let final_vec_len = _to_u8(g, 1500) % 65;
        let mut final_vec = Vec::with_capacity(final_vec_len as usize);
        for j in 0..final_vec_len {
            let s_len = _to_u8(g, 1600 + j as usize * 3) % 65;
            let start = 1700 + j as usize * 5;
            let end = start + s_len as usize;
            let s = _to_str(g, start, end);
            final_vec.push(s.to_string());
        }
        let sv = StackVec::<[String; 32]>::from_vec(final_vec);
        let _ = sv.into_vec();
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