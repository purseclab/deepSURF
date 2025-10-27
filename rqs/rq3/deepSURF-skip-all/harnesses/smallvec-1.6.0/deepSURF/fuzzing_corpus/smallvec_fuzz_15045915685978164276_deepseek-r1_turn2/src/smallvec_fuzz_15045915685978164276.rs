#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::SmallVec;
use global_data::*;
use std::cmp::Ordering;

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 2048 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let global_slice = global_data.first_half;
        let mut offset = 0;

        let mut vec1: SmallVec<[String; 16]> = SmallVec::new();
        let mut vec2 = SmallVec::with_capacity(128);
        let num_ops = _to_u8(global_slice, offset) % 64;
        offset += 1;

        for i in 0..num_ops {
            let op = _to_u8(global_slice, offset) % 11;
            offset += 1;

            match op {
                0 => {
                    let cap = _to_usize(global_slice, offset);
                    offset += 8;
                    vec2 = SmallVec::with_capacity(cap % 65);
                }
                1 => {
                    let elem = String::from(_to_str(global_slice, offset, offset + 8));
                    offset += 8;
                    vec1.push(elem);
                }
                2 => {
                    let idx = _to_usize(global_slice, offset);
                    offset += 8;
                    if !vec1.is_empty() {
                        let _ = vec1.remove(idx % vec1.len());
                    }
                }
                3 => {
                    let idx = _to_usize(global_slice, offset);
                    offset += 8;
                    let elem = String::from(_to_str(global_slice, offset, offset + 8));
                    offset += 8;
                    vec1.insert(idx, elem);
                }
                4 => {
                    let len = _to_usize(global_slice, offset);
                    offset += 8;
                    vec1.truncate(len);
                }
                5 => {
                    let len = _to_usize(global_slice, offset);
                    offset += 8;
                    vec1.resize_with(len, || String::new());
                }
                6 => {
                    let value = String::from(_to_str(global_slice, offset, offset + 8));
                    offset += 8;
                    vec2.resize(vec2.len() + 4, value);
                }
                7 => {
                    let part = vec1.as_slice();
                    println!("{:?}", part);
                }
                8 => {
                    let mut part = vec1.as_mut_slice();
                    let idx = _to_usize(global_slice, offset);
                    offset += 8;
                    part[idx % part.len()] = String::new();
                }
                9 => {
                    let ordering = vec1.partial_cmp(&vec2).unwrap_or(Ordering::Equal);
                    println!("{:?}", ordering);
                }
                10 => {
                    let cap = _to_usize(global_slice, offset);
                    offset += 8;
                    let mut temp_vec = SmallVec::from_elem(String::from("item"), cap % 65);
                    temp_vec.append(&mut vec1);
                    vec1 = temp_vec;
                }
                _ => {}
            }
        }

        let _: Vec<_> = vec1.into_iter().collect();
        let _: Vec<_> = vec2.into_iter().collect();
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