#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::SmallVec;
use global_data::*;
use std::fmt::Debug;

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 2048 { return; }
        set_global_data(data);
        let global_data = get_global_data();

        let constructor_byte = _to_u8(global_data.first_half, 0);
        let mut smallvec = match constructor_byte % 4 {
            0 => {
                let mut vec = Vec::new();
                let num_elements = _to_usize(global_data.first_half, 1) % 65;
                let mut offset = 2;
                for _ in 0..num_elements {
                    if offset + 2 >= global_data.first_half.len() { break; }
                    let str_len = _to_u8(global_data.first_half, offset) % 17;
                    let s = _to_str(global_data.first_half, offset+1, offset+1+str_len as usize);
                    vec.push(String::from(s));
                    offset += 1 + str_len as usize;
                }
                SmallVec::<[String; 32]>::from_vec(vec)
            }
            1 => {
                let mut offset = 1;
                let mut slice_data = Vec::new();
                while offset + 1 < global_data.first_half.len() {
                    let str_len = _to_u8(global_data.first_half, offset) % 17;
                    let s = _to_str(global_data.first_half, offset+1, offset+1+str_len as usize);
                    slice_data.push(String::from(s));
                    offset += 1 + str_len as usize;
                }
                SmallVec::<[String; 32]>::from_vec(slice_data)
            }
            2 => {
                let str_len = _to_u8(global_data.first_half, 1) % 17;
                let s = _to_str(global_data.first_half, 2, 2 + str_len as usize);
                let count = _to_usize(global_data.first_half, 2 + str_len as usize) % 65;
                SmallVec::<[String; 32]>::from_elem(String::from(s), count)
            }
            _ => {
                let capacity = _to_usize(global_data.first_half, 1);
                let mut sv = SmallVec::<[String; 32]>::with_capacity(capacity);
                let num_pushes = _to_u8(global_data.first_half, 9) % 32;
                let mut offset = 10;
                for _ in 0..num_pushes {
                    if offset + 1 >= global_data.first_half.len() { break; }
                    let str_len = _to_u8(global_data.first_half, offset) % 17;
                    let s = _to_str(global_data.first_half, offset+1, offset+1+str_len as usize);
                    sv.push(String::from(s));
                    offset += 1 + str_len as usize;
                }
                sv
            }
        };

        let op_count = _to_u8(global_data.second_half, 0) % 20;
        let mut offset = 1;
        for _ in 0..op_count {
            if offset >= global_data.second_half.len() { break; }
            match _to_u8(global_data.second_half, offset) % 10 {
                0 => {
                    if offset + 3 < global_data.second_half.len() {
                        let index = _to_usize(global_data.second_half, offset+1);
                        let str_len = _to_u8(global_data.second_half, offset+9) % 17;
                        let s = _to_str(global_data.second_half, offset+10, offset+10+str_len as usize);
                        smallvec.insert(index, String::from(s));
                        offset += 10 + str_len as usize;
                    }
                }
                1 => {
                    smallvec.pop();
                    offset += 1;
                }
                2 => {
                    let len = _to_usize(global_data.second_half, offset+1);
                    smallvec.truncate(len);
                    offset += 9;
                }
                3 => {
                    let index = _to_usize(global_data.second_half, offset+1);
                    let _ = smallvec.swap_remove(index);
                    offset += 9;
                }
                4 => {
                    let index = _to_usize(global_data.second_half, offset+1);
                    let _ = smallvec.get(index);
                    offset += 9;
                }
                5 => {
                    let cap = _to_usize(global_data.second_half, offset+1);
                    smallvec.reserve(cap);
                    offset += 9;
                }
                6 => {
                    let index = _to_usize(global_data.second_half, offset+1);
                    let _ = smallvec.remove(index);
                    offset += 9;
                }
                7 => {
                    let cap = _to_usize(global_data.second_half, offset+1);
                    smallvec.reserve_exact(cap);
                    offset += 9;
                }
                8 => {
                    let other = smallvec.clone();
                    let _ = smallvec.partial_cmp(&other);
                    offset += 1;
                }
                _ => {
                    let _slice = smallvec.as_slice();
                    offset += 1;
                }
            }
        }

        let _ = smallvec.as_mut_slice();
        let _ = smallvec.capacity();
        let _ = smallvec.len();
        let _ = smallvec.is_empty();
        let _ = SmallVec::<[String; 32]>::from_elem(String::new(), _to_usize(global_data.second_half, 0) % 65);
        let _ = smallvec.as_ptr();
        println!("{:?}", smallvec);
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