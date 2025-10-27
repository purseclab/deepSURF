#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::ops::{Deref, DerefMut};

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 512 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let first_half = global_data.first_half;

        let op_count = _to_usize(first_half, 0) % 64;
        let base_offset = 10;

        for i in 0..op_count {
            let offset = base_offset + i * 16;
            let op_selector = _to_u8(first_half, offset) % 7;
            
            let mut sv = match op_selector {
                0 => {
                    let cap = _to_usize(first_half, offset + 1);
                    let mut sv = SmallVec::<[u8; 256]>::with_capacity(cap);
                    let push_count = _to_usize(first_half, offset + 9) % 64;
                    
                    for j in 0..push_count {
                        let elem_off = offset + 17 + j;
                        sv.push(_to_u8(first_half, elem_off));
                    }
                    
                    if !sv.is_empty() {
                        let drain_range = _to_usize(first_half, offset + 81) % sv.len();
                        sv.drain(drain_range..);
                        println!("Drained to len: {}", sv.len());
                    }
                    sv
                },
                1 => {
                    let mut sv = SmallVec::<[u8; 256]>::new();
                    let insert_idx = _to_usize(first_half, offset + 1) % 33;
                    sv.insert(insert_idx, _to_u8(first_half, offset + 9));
                    sv
                },
                2 => {
                    let slice_len = _to_usize(first_half, offset + 1) % 65;
                    let slice_start = _to_usize(first_half, offset + 9) % 256;
                    let slice = &first_half[slice_start..slice_start + slice_len];
                    let mut sv = SmallVec::<[u8; 256]>::from_slice(slice);
                    
                    let new_cap = _to_usize(first_half, offset + 17);
                    sv.try_grow(new_cap).unwrap_or_else(|_| ());
                    sv
                },
                3 => {
                    let elem = _to_u8(first_half, offset + 1);
                    let count = _to_usize(first_half, offset + 9) % 65;
                    let mut sv = SmallVec::from_elem(elem, count);
                    
                    let other_cap = _to_usize(first_half, offset + 17);
                    let mut other = SmallVec::<[u8; 256]>::with_capacity(other_cap);
                    sv.append(&mut other);
                    sv
                },
                4 => {
                    let mut sv = SmallVec::<[u8; 256]>::from_vec(vec![_to_u8(first_half, offset + 1); 32]);
                    let truncate_to = _to_usize(first_half, offset + 9) % sv.len();
                    sv.truncate(truncate_to);
                    println!("Truncated slice: {:?}", sv.as_slice());
                    sv
                },
                5 => {
                    let mut sv1 = SmallVec::<[u8; 256]>::with_capacity(_to_usize(first_half, offset + 1));
                    let mut sv2 = SmallVec::<[u8; 256]>::new();
                    
                    sv2.extend_from_slice(&first_half[offset..offset+32]);
                    let cmp_res = sv1.partial_cmp(&sv2);
                    println!("Comparison: {:?}", cmp_res);
                    sv1.append(&mut sv2);
                    sv1
                },
                _ => {
                    let mut sv = SmallVec::<[u8; 256]>::with_capacity(_to_usize(first_half, offset + 1));
                    for _ in 0..3 {
                        sv.push(_to_u8(first_half, offset + 9));
                    }
                    println!("Capacity after pushes: {}", sv.capacity());
                    sv
                }
            };

            let retained = sv.retain(|x| *x % 2 == 0);
            let capacity_before_shrink = sv.capacity();
            sv.shrink_to_fit();
            let _ = sv.pop();
            sv.insert_many(0, [0u8, 1u8, 2u8].iter().cloned());
            sv.dedup();
            {
                let _drained = sv.drain(..2);
            }
            let slice_ref = sv.as_slice();
            println!("Slice reference length: {}", slice_ref.len());
            let _boxed = sv.into_boxed_slice();
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