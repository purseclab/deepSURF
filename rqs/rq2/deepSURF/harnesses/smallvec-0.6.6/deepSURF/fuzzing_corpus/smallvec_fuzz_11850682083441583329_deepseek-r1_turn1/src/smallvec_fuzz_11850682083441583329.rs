#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 2048 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let mut offset = 0;

        let constructor_selector = _to_u8(global_data.first_half, offset);
        offset += 1;
        let mut sv: SmallVec<[u8; 16]> = match constructor_selector % 5 {
            0 => SmallVec::new(),
            1 => {
                let capacity = _to_usize(global_data.first_half, offset) % 65;
                offset += 8;
                SmallVec::with_capacity(capacity)
            }
            2 => {
                let slice_start = _to_usize(global_data.first_half, offset) % global_data.first_half.len();
                let slice_len = _to_usize(global_data.first_half, offset + 8) % 65;
                let slice_end = slice_start.saturating_add(slice_len);
                let slice = &global_data.first_half[slice_start..slice_end.min(global_data.first_half.len())];
                offset += 16;
                SmallVec::from_slice(slice)
            }
            3 => {
                let elem = _to_u8(global_data.first_half, offset);
                let count = _to_usize(global_data.first_half, offset + 1) % 65;
                offset += 9;
                SmallVec::from_elem(elem, count)
            }
            4 => {
                let raw_len = _to_usize(global_data.first_half, offset) % 65;
                let raw = vec![_to_u8(global_data.first_half, offset + 8); raw_len];
                offset += 16;
                SmallVec::from_vec(raw)
            }
            _ => SmallVec::new(),
        };

        let num_ops = _to_usize(global_data.second_half, 0) % 50;
        for i in 0..num_ops {
            let op_base = i * 16 % global_data.second_half.len();
            let op_selector = _to_u8(global_data.second_half, op_base) % 9;

            match op_selector {
                0 => {
                    let value = _to_u8(global_data.second_half, op_base + 1);
                    sv.push(value);
                    println!("After push: {:?}", sv.as_slice());
                }
                1 => {
                    let _ = sv.pop();
                    let _ = sv.len();
                }
                2 => {
                    let index = _to_usize(global_data.second_half, op_base + 1);
                    let value = _to_u8(global_data.second_half, op_base + 9);
                    sv.insert(index, value);
                }
                3 => {
                    let index = _to_usize(global_data.second_half, op_base + 1);
                    sv.swap_remove(index);
                    println!("After swap: {:?}", sv.as_mut_slice());
                }
                4 => {
                    let new_len = _to_usize(global_data.second_half, op_base + 1);
                    sv.truncate(new_len);
                    let _ = sv.capacity();
                }
                5 => {
                    let ext_len = _to_usize(global_data.second_half, op_base + 1) % 65;
                    let ext_slice = &global_data.second_half[op_base + 9..][..ext_len.min(global_data.second_half.len() - op_base - 9)];
                    sv.extend_from_slice(ext_slice);
                }
                6 => {
                    let idx = _to_usize(global_data.second_half, op_base + 1);
                    let _ = sv.get(idx);
                }
                7 => {
                    let other = SmallVec::from_elem(0, _to_usize(global_data.second_half, op_base + 1) % 65);
                    let _ = sv.partial_cmp(&other);
                }
                8 => {
                    let _ = sv.drain();
                    let _ = sv.is_empty();
                }
                _ => {}
            }
        }

        if !sv.is_empty() {
            let final_index = _to_usize(global_data.second_half, global_data.second_half.len().saturating_sub(8)) % sv.len();
            sv.swap_remove(final_index);
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