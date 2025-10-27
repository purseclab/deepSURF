#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::ops::{Deref, DerefMut, Index, IndexMut};

fn main (){
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 1024 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let first_half = global_data.first_half;
        let second_half = global_data.second_half;

        let constructor_selector = _to_u8(first_half, 0);
        let mut sv = match constructor_selector % 7 {
            0 => SmallVec::<[u8; 16]>::new(),
            1 => {
                let cap = _to_usize(first_half, 1) % 65;
                SmallVec::<[u8; 16]>::with_capacity(cap)
            },
            2 => {
                let len = _to_usize(first_half, 1) % 65;
                let slice = &first_half[2..2 + len.min(first_half.len() - 2)];
                SmallVec::from_slice(slice)
            },
            3 => {
                let elem = _to_u8(first_half, 1);
                let n = _to_usize(first_half, 2) % 65;
                SmallVec::from_elem(elem, n)
            },
            4 => {
                let mut arr = [0u8; 16];
                for i in 0..16 {
                    arr[i] = _to_u8(first_half, 1 + i);
                }
                SmallVec::from_buf(arr)
            },
            5 => {
                let elements = first_half[1..].to_vec();
                SmallVec::from_vec(elements)
            },
            _ => SmallVec::from_iter((0..4).map(|_| _to_u8(first_half, 1))),
        };

        let op_count = _to_usize(second_half, 0) % 65;
        for op_idx in 0..op_count {
            let op_selector = _to_u8(second_half, 1 + op_idx);
            match op_selector % 13 {
                0 => sv.push(_to_u8(second_half, op_idx + 2)),
                1 => { sv.pop(); },
                2 => sv.insert(_to_usize(second_half, op_idx + 2), _to_u8(second_half, op_idx + 3)),
                3 => {
                    if !sv.is_empty() { 
                        let slice = sv.as_mut_slice();
                        *slice.get_mut(0).unwrap() = _to_u8(second_half, op_idx + 2);
                    }
                },
                4 => sv.truncate(_to_usize(second_half, op_idx + 2)),
                5 => sv.clear(),
                6 => sv.extend_from_slice(&[0, 1, 2]),
                7 => {
                    if let Some(x) = sv.get_mut(_to_usize(second_half, op_idx + 2)) {
                        *x += 1;
                    }
                },
                8 => { let _ = sv.drain(); },
                9 => sv.shrink_to_fit(),
                10 => {
                    if sv.len() > 3 {
                        sv.swap_remove(_to_usize(second_half, op_idx + 2) % sv.len());
                    }
                },
                11 => {
                    sv.reserve(_to_usize(second_half, op_idx + 2));
                },
                12 => println!("{:?}", sv.as_slice()),
                _ => (),
            }
        }

        let _ = sv.as_mut_slice();
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