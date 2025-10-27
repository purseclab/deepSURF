#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use stackvector::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

#[derive(Clone, Copy, Debug)]
struct CustomType1(usize);

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 1024 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let op_count = _to_usize(GLOBAL_DATA, 0) % 8;
        let constructor_selector = _to_u8(GLOBAL_DATA, 8) % 3;
        
        let mut sv = match constructor_selector {
            0 => {
                let elem = CustomType1(_to_usize(GLOBAL_DATA, 9));
                StackVec::from_elem(elem, _to_usize(GLOBAL_DATA, 17) % 65)
            }
            1 => {
                let slice_data = &global_data.second_half[.._to_usize(GLOBAL_DATA, 25) % 65];
                let items: Vec<CustomType1> = slice_data.chunks_exact(2)
                    .map(|c| CustomType1(_to_usize(c, 0)))
                    .collect();
                StackVec::from_slice(&items)
            }
            _ => StackVec::from_buf_and_len([CustomType1(0); 32], _to_usize(GLOBAL_DATA, 33) % 65)
        };

        for i in 0..op_count {
            let op_byte = _to_u8(GLOBAL_DATA, 41 + i as usize);
            match op_byte % 7 {
                0 => {
                    let idx = _to_usize(GLOBAL_DATA, 49 + i*8);
                    let val = CustomType1(_to_usize(GLOBAL_DATA, 57 + i*8));
                    sv.insert(idx, val);
                }
                1 => {
                    let _ = sv.pop();
                }
                2 => {
                    let new_len = _to_usize(GLOBAL_DATA, 65 + i*8);
                    sv.truncate(new_len);
                }
                3 => {
                    let idx = _to_usize(GLOBAL_DATA, 73 + i*8);
                    let _ = sv.remove(idx);
                }
                4 => {
                    let slice_len = _to_usize(GLOBAL_DATA, 81 + i*8) % 65;
                    let items: Vec<CustomType1> = global_data.second_half[81+i*8..81+i*8+slice_len]
                        .chunks_exact(2)
                        .map(|c| CustomType1(_to_usize(c, 0)))
                        .collect();
                    sv.extend_from_slice(&items);
                }
                5 => {
                    let idx = _to_usize(GLOBAL_DATA, 89 + i*8);
                    let slice_len = _to_usize(GLOBAL_DATA, 97 + i*8) % 65;
                    let items: Vec<CustomType1> = global_data.second_half[97+i*8..97+i*8+slice_len]
                        .chunks_exact(2)
                        .map(|c| CustomType1(_to_usize(c, 0)))
                        .collect();
                    sv.insert_from_slice(idx, &items);
                }
                _ => {
                    let _ = sv.as_slice();
                    println!("{:?}", sv.deref());
                }
            };
        }

        let final_op = _to_u8(GLOBAL_DATA, 137) % 3;
        let target_idx = _to_usize(GLOBAL_DATA, 138);
        let slice_len = _to_usize(GLOBAL_DATA, 146) % 65;
        let items: Vec<CustomType1> = global_data.second_half[146..146+slice_len]
            .chunks_exact(2)
            .map(|c| CustomType1(_to_usize(c, 0)))
            .collect();

        match final_op {
            0 => sv.insert_from_slice(target_idx, &items),
            1 => sv.extend_from_slice(&items),
            _ => sv.truncate(target_idx),
        };

        let _ = sv.drain();
        println!("Final vec: {:?}", sv.as_slice());
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