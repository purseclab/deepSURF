#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

fn main (){
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 4096 {return;}
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let mut vecs = Vec::new();
        let op_count = _to_u8(GLOBAL_DATA, 0) % 8;

        for i in 0..op_count {
            match _to_u8(GLOBAL_DATA, 1 + i as usize) % 5 {
                0 => {
                    let cap = _to_usize(GLOBAL_DATA, (2 + i * 8).into());
                    let sv = SmallVec::<[String;16]>::with_capacity(cap);
                    vecs.push(sv);
                }
                1 => {
                    let elem_count = _to_usize(GLOBAL_DATA, (3 + i * 8).into()) % 65;
                    let s = _to_str(GLOBAL_DATA, 100 * i as usize, 100 * (i+1) as usize).to_string();
                    let sv = SmallVec::<[String;16]>::from_elem(s, elem_count);
                    vecs.push(sv);
                }
                2 => {
                    let mut sv = SmallVec::<[String;16]>::new();
                    let push_cnt = _to_usize(GLOBAL_DATA, (4 + i * 8).into()) % 65;
                    for j in 0..push_cnt {
                        let s = _to_str(GLOBAL_DATA, 200 * j as usize, 200 * (j+1) as usize).to_string();
                        sv.push(s);
                        if j % 3 == 0 {
                            println!("{:?}", sv.as_slice());
                        }
                    }
                    vecs.push(sv);
                }
                3 => {
                    if let Some(v) = vecs.last_mut() {
                        let drain_start = _to_usize(GLOBAL_DATA, (5 + i * 16).into());
                        let drain_end = _to_usize(GLOBAL_DATA, (13 + i * 16).into());
                        let _ = v.drain(drain_start..drain_end);
                        println!("{:?}", v.as_slice());
                    }
                }
                4 => {
                    let idx = _to_usize(GLOBAL_DATA, (6 + i * 8).into()) % vecs.len();
                    let other_idx = _to_usize(GLOBAL_DATA, (14 + i * 8).into()) % vecs.len();
                    let ordering = vecs[idx].cmp(&vecs[other_idx]);
                    println!("{:?}", ordering);
                }
                _ => unreachable!()
            }
        }

        let cap = _to_usize(GLOBAL_DATA, global_data.first_half.len() / 2);
        let mut target_sv = SmallVec::<[String;16]>::with_capacity(cap);
        for v in vecs {
            target_sv.append(&mut v.clone());
        }

        let insert_pos = _to_usize(GLOBAL_DATA, global_data.first_half.len() - 8);
        let insert_str = _to_str(global_data.second_half, 0, 64).to_string();
        target_sv.insert(insert_pos, insert_str);

        let _removed = target_sv.remove(_to_usize(GLOBAL_DATA, 16));
        let _popped = target_sv.pop();
        println!("{:?}", target_sv.as_ptr());
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