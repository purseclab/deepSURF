#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use endian_trait::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 4096 { return; }
        set_global_data(data);
        let global_data = get_global_data();

        let mut vec_chars = Vec::new();
        let mut vec_u16 = Vec::new();
        let mut vec_char_ct = Vec::new();

        let num_chars = (_to_u8(global_data.first_half, 0) % 65) as usize;
        for i in 0..num_chars {
            let offset = 1 + i * 4;
            let c = _to_char(global_data.first_half, offset);
            vec_chars.push(c);
        }

        let num_u16 = (_to_u8(global_data.second_half, 0) % 65) as usize;
        for i in 0..num_u16 {
            let offset = 1 + i * 2;
            vec_u16.push(_to_u16(global_data.second_half, offset));
        }

        let ct_entries = (_to_u8(global_data.first_half, 256) % 65) as usize;
        for i in 0..ct_entries {
            let offset = 257 + i * 4;
            let c = _to_char(global_data.first_half, offset);
            vec_char_ct.push(c);
        }

        let ops = (_to_u8(global_data.second_half, 128) % 65) as usize;
        for op_idx in 0..ops {
            let op_selector = _to_u8(global_data.second_half, 129 + op_idx) % 7;
            match op_selector {
                0 => {
                    let slice = &mut vec_chars[..];
                    slice.from_be();
                    println!("Converted chars: {:?}", &slice[0]);
                }
                1 => {
                    let mut slice = &mut vec_u16[..];
                    slice.to_le();
                }
                2 => {
                    if !vec_char_ct.is_empty() {
                        let idx = _to_u8(global_data.second_half, 194 + op_idx) as usize % vec_char_ct.len();
                        let elem = vec_char_ct[idx].to_be();
                        vec_char_ct[idx] = elem;
                    }
                }
                3 => {
                    let slice = &mut vec_u16[..];
                    slice.from_le();
                    println!("u16[0]: {:?}", slice[0]);
                }
                4 => {
                    let slice = &mut vec_char_ct[..];
                    slice.from_be();
                }
                5 => {
                    if !vec_chars.is_empty() {
                        let idx = _to_u8(global_data.second_half, 250 + op_idx) as usize % vec_chars.len();
                        vec_chars[idx] = vec_chars[idx].from_le();
                    }
                }
                6 => {
                    let slice = &mut vec_u16[..];
                    slice.from_be();
                }
                _ => {}
            }
        }

        if !vec_char_ct.is_empty() {
            let final_slice = &mut vec_char_ct[..];
            final_slice.from_be();
            println!("Final CT[0]: {:?}", &final_slice[0]);
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