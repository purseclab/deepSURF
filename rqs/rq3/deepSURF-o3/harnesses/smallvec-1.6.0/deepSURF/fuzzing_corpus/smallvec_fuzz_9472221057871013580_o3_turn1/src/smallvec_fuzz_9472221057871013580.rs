#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 164 {
            return;
        }
        set_global_data(data);
        let global_data = get_global_data();
        let first = global_data.first_half;
        let second = global_data.second_half;

        let mut arr32: [u8; 32] = [0; 32];
        for i in 0..32 {
            arr32[i] = _to_u8(first, i);
        }

        let len_buf = _to_usize(first, 32);
        let constructor_choice = _to_u8(first, 40) % 6;

        let mut sv: SmallVec<[u8; 32]> = match constructor_choice {
            0 => SmallVec::<[u8; 32]>::from_buf_and_len(arr32, len_buf),
            1 => {
                let cap = _to_usize(first, 48);
                SmallVec::<[u8; 32]>::with_capacity(cap)
            }
            2 => {
                let elem = _to_u8(first, 56);
                let n = (_to_usize(first, 57) % 65) as usize;
                SmallVec::<[u8; 32]>::from_elem(elem, n)
            }
            3 => {
                let slice_len = (_to_usize(first, 65) % 32) as usize;
                let slice = &arr32[..slice_len];
                SmallVec::<[u8; 32]>::from_slice(slice)
            }
            4 => {
                let vec_len = (_to_u8(first, 73) as usize) % 65;
                let mut vec_inst = Vec::<u8>::with_capacity(vec_len);
                for i in 0..vec_len {
                    vec_inst.push(_to_u8(second, i));
                }
                SmallVec::<[u8; 32]>::from_vec(vec_inst)
            }
            _ => SmallVec::<[u8; 32]>::new(),
        };

        let mut sv2 = SmallVec::<[u8; 32]>::new();
        let op_count = _to_u8(first, 80) % 20;

        for i in 0..op_count {
            let op_selector = _to_u8(second, i as usize);
            match op_selector % 11 {
                0 => {
                    let val = _to_u8(second, i as usize + 1);
                    sv.push(val);
                }
                1 => {
                    sv.pop();
                }
                2 => {
                    let index = _to_usize(second, i as usize + 2);
                    let val = _to_u8(second, i as usize + 3);
                    sv.insert(index, val);
                }
                3 => {
                    let index = _to_usize(second, i as usize + 4);
                    if !sv.is_empty() {
                        sv.remove(index % sv.len());
                    }
                }
                4 => {
                    let len = _to_usize(second, i as usize + 5);
                    sv.truncate(len);
                }
                5 => {
                    let additional = _to_usize(second, i as usize + 6);
                    sv.reserve(additional);
                }
                6 => {
                    println!("{:?}", sv.as_slice());
                }
                7 => {
                    let index = _to_usize(second, i as usize + 7);
                    if !sv.is_empty() {
                        sv.swap_remove(index % sv.len());
                    }
                }
                8 => {
                    let value = _to_u8(second, i as usize + 8);
                    sv2.push(value);
                    sv.append(&mut sv2);
                }
                9 => {
                    let slice_len = (_to_u8(second, i as usize + 9) as usize) % 32;
                    let slice = &arr32[..slice_len];
                    sv.extend_from_slice(slice);
                }
                _ => {
                    sv.clear();
                }
            }
        }

        let final_len = _to_usize(first, 81);
        sv.truncate(final_len);

        println!("{:?}", sv.capacity());
        println!("{:?}", sv.partial_cmp(&sv.clone()));
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