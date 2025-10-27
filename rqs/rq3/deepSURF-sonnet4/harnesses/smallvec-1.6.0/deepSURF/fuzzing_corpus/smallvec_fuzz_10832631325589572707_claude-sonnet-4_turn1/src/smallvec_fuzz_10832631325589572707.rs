#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

fn main (){
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 200 {return;}
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        
        let num_ops = _to_u8(GLOBAL_DATA, 0) % 15;
        for op_idx in 0..num_ops {
            let base_offset = 1 + (op_idx as usize * 12);
            if base_offset + 12 > GLOBAL_DATA.len() { break; }
            
            let op_type = _to_u8(GLOBAL_DATA, base_offset) % 8;
            match op_type {
                0 => {
                    let capacity = _to_usize(GLOBAL_DATA, base_offset + 1);
                    let mut sv1 = SmallVec::<[u8; 16]>::with_capacity(capacity);
                    
                    let items_count = _to_u8(GLOBAL_DATA, base_offset + 9) % 10;
                    for i in 0..items_count {
                        let item = _to_u8(GLOBAL_DATA, base_offset + 10);
                        sv1.push(item);
                    }
                    
                    let sv1_ref = &sv1;
                    let slice_ref = sv1_ref.as_ref();
                    println!("{:?}", slice_ref);
                    
                    let _ = sv1_ref.as_slice();
                    let _ = sv1_ref.len();
                    let _ = sv1_ref.capacity();
                    let _ = sv1_ref.is_empty();
                },
                1 => {
                    let vec_size = _to_u8(GLOBAL_DATA, base_offset + 1) % 20;
                    let mut vec_data = Vec::new();
                    for i in 0..vec_size {
                        vec_data.push(_to_u8(GLOBAL_DATA, base_offset + 2 + (i as usize)));
                    }
                    let sv2 = SmallVec::<[u8; 32]>::from_vec(vec_data);
                    
                    let sv2_ref = &sv2;
                    let slice_ref = sv2_ref.as_ref();
                    println!("{:?}", slice_ref);
                    
                    let iter = sv2.into_iter();
                    for item in iter {
                        println!("{}", item);
                    }
                },
                2 => {
                    let slice_size = _to_u8(GLOBAL_DATA, base_offset + 1) % 25;
                    let slice_data: Vec<u16> = (0..slice_size).map(|i| 
                        _to_u16(GLOBAL_DATA, base_offset + 2 + (i as usize * 2))
                    ).collect();
                    let sv3 = SmallVec::<[u16; 12]>::from_slice(&slice_data);
                    
                    let sv3_ref = &sv3;
                    let slice_ref = sv3_ref.as_ref();
                    println!("{:?}", slice_ref);
                    
                    let clone_sv = sv3_ref.clone();
                    let _ = clone_sv.as_ref();
                },
                3 => {
                    let elem_value = _to_i32(GLOBAL_DATA, base_offset + 1);
                    let count = _to_usize(GLOBAL_DATA, base_offset + 5);
                    let mut sv4 = SmallVec::<[i32; 8]>::from_elem(elem_value, count);
                    
                    let sv4_ref = &sv4;
                    let slice_ref = sv4_ref.as_ref();
                    println!("{:?}", slice_ref);
                    
                    let index = _to_usize(GLOBAL_DATA, base_offset + 10);
                    let new_elem = _to_i32(GLOBAL_DATA, base_offset + 11);
                    sv4.insert(index, new_elem);
                    
                    let sv4_ref_after = &sv4;
                    let slice_ref_after = sv4_ref_after.as_ref();
                    println!("{:?}", slice_ref_after);
                },
                4 => {
                    let mut sv5 = SmallVec::<[f32; 10]>::new();
                    let push_count = _to_u8(GLOBAL_DATA, base_offset + 1) % 12;
                    for i in 0..push_count {
                        let value = _to_f32(GLOBAL_DATA, base_offset + 2 + (i as usize * 4));
                        sv5.push(value);
                    }
                    
                    let sv5_ref = &sv5;
                    let slice_ref = sv5_ref.as_ref();
                    println!("{:?}", slice_ref);
                    
                    let remove_idx = _to_usize(GLOBAL_DATA, base_offset + 10);
                    if !sv5.is_empty() {
                        let removed = sv5.remove(remove_idx);
                        println!("{}", removed);
                    }
                    
                    let sv5_ref_final = &sv5;
                    let slice_ref_final = sv5_ref_final.as_ref();
                    println!("{:?}", slice_ref_final);
                },
                5 => {
                    let iter_data: Vec<char> = (0..8).map(|i| 
                        _to_char(GLOBAL_DATA, base_offset + 1 + (i * 4))
                    ).collect();
                    let sv6 = SmallVec::<[char; 6]>::from_iter(iter_data.into_iter());
                    
                    let sv6_ref = &sv6;
                    let slice_ref = sv6_ref.as_ref();
                    println!("{:?}", slice_ref);
                    
                    let mut sv6_mut = sv6;
                    let capacity_add = _to_usize(GLOBAL_DATA, base_offset + 9);
                    sv6_mut.reserve(capacity_add);
                    
                    let sv6_ref_reserved = &sv6_mut;
                    let slice_ref_reserved = sv6_ref_reserved.as_ref();
                    println!("{:?}", slice_ref_reserved);
                },
                6 => {
                    let arr = [_to_bool(GLOBAL_DATA, base_offset + 1), 
                              _to_bool(GLOBAL_DATA, base_offset + 2),
                              _to_bool(GLOBAL_DATA, base_offset + 3),
                              _to_bool(GLOBAL_DATA, base_offset + 4),
                              _to_bool(GLOBAL_DATA, base_offset + 5),
                              _to_bool(GLOBAL_DATA, base_offset + 6),
                              _to_bool(GLOBAL_DATA, base_offset + 7),
                              _to_bool(GLOBAL_DATA, base_offset + 8),
                              _to_bool(GLOBAL_DATA, base_offset + 9),
                              _to_bool(GLOBAL_DATA, base_offset + 10),
                              _to_bool(GLOBAL_DATA, base_offset + 11)];
                    let sv7 = SmallVec::<[bool; 11]>::from_buf(arr);
                    
                    let sv7_ref = &sv7;
                    let slice_ref = sv7_ref.as_ref();
                    println!("{:?}", slice_ref);
                    
                    let mut drain_sv = sv7;
                    let drain_start = _to_usize(GLOBAL_DATA, base_offset + 9);
                    let drain_end = _to_usize(GLOBAL_DATA, base_offset + 10);
                    let drain_iter = drain_sv.drain(drain_start..drain_end);
                    for drained in drain_iter {
                        println!("{}", drained);
                    }
                    
                    let sv7_ref_drained = &drain_sv;
                    let slice_ref_drained = sv7_ref_drained.as_ref();
                    println!("{:?}", slice_ref_drained);
                },
                7 => {
                    let buf_arr = [_to_u64(GLOBAL_DATA, base_offset + 1),
                                  _to_u64(GLOBAL_DATA, base_offset + 9)];
                    let buf_len = _to_usize(GLOBAL_DATA, base_offset + 10);
                    let sv8 = SmallVec::<[u64; 2]>::from_buf_and_len(buf_arr, buf_len);
                    
                    let sv8_ref = &sv8;
                    let slice_ref = sv8_ref.as_ref();
                    println!("{:?}", slice_ref);
                    
                    let mut extend_sv = sv8;
                    let extend_data = vec![_to_u64(GLOBAL_DATA, base_offset + 11)];
                    extend_sv.extend(extend_data.into_iter());
                    
                    let sv8_ref_extended = &extend_sv;
                    let slice_ref_extended = sv8_ref_extended.as_ref();
                    println!("{:?}", slice_ref_extended);
                },
                _ => {}
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