#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use stackvector::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};
use std::fmt::Debug;

fn main (){
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 4096 {return;}
        set_global_data(data);
        let global_data = get_global_data();
        
        let num_vectors = _to_u8(global_data.first_half, 0) % 5 + 1;
        let mut stackvecs = Vec::with_capacity(num_vectors as usize);

        for i in 0..num_vectors {
            let constructor_selector = _to_u8(global_data.first_half, i as usize * 2) % 3;
            let data_slice = if i % 2 == 0 {global_data.first_half} else {global_data.second_half};

            match constructor_selector {
                0 => {
                    let elem_count = _to_u8(data_slice, 32) % 65;
                    let mut vec = Vec::with_capacity(elem_count as usize);
                    for j in 0..elem_count {
                        let str_len = _to_u8(data_slice, 64 + j as usize) % 17;
                        let s = _to_str(data_slice, 128 + j as usize * 20, 128 + j as usize * 20 + str_len as usize);
                        vec.push(String::from(s));
                    }
                    let sv = StackVec::<[String; 64]>::from(vec);
                    stackvecs.push(sv);
                }
                1 => {
                    let mut elements = Vec::with_capacity(64);
                    for k in 0..64 {
                        let start = 384 + k * 2;
                        let str_len = _to_u8(data_slice, start) % 17;
                        let s = _to_str(data_slice, start + 1, start + 1 + str_len as usize);
                        elements.push(s.to_string());
                    }
                    let len = _to_usize(data_slice, 512) % 64;
                    let sv = StackVec::<[String; 64]>::from_buf_and_len(elements.try_into().unwrap(), len);
                    stackvecs.push(sv);
                }
                _ => {
                    let slice_len = _to_u8(data_slice, 640) % 65;
                    let mut elements = Vec::with_capacity(slice_len as usize);
                    for k in 0..slice_len {
                        let seg = _to_str(data_slice, 768 + k as usize * 16, 768 + k as usize * 16 + 8);
                        elements.push(seg.to_string());
                    }
                    let sv = StackVec::<[String; 64]>::from(elements);
                    stackvecs.push(sv);
                }
            };
        }

        for sv in &mut stackvecs {
            let ops = _to_u8(global_data.second_half, 0) % 6;
            for _ in 0..ops {
                let op_type = _to_u8(global_data.second_half, 1) % 4;
                let idx = _to_usize(global_data.second_half, 2) % sv.len().max(1);
                match op_type {
                    0 => sv.push(_to_str(global_data.second_half, 64, 72).to_string()),
                    1 => {sv.pop();},
                    2 => sv.truncate(idx),
                    3 => sv.insert(idx % (sv.len() + 1), String::new()),
                    _ => {sv.swap_remove(idx % sv.len());},
                }
            }
        }

        for i in 0..stackvecs.len() {
            for j in (i+1)..stackvecs.len() {
                let _ = stackvecs[i].partial_cmp(&stackvecs[j]);
                println!("{:?}", stackvecs[i].as_slice());
                println!("{:?}", stackvecs[j].as_slice());
            }
        }

        let _ = format!("{:?}", &stackvecs[0]);
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