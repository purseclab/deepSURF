#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::SmallVec;
use global_data::*;
use std::ops::{Deref, DerefMut};
use std::fmt::Debug;

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 512 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let (first, second) = (global_data.first_half, global_data.second_half);

        let mut sv = match _to_u8(first, 0) % 5 {
            0 => SmallVec::<[String; 32]>::new(),
            1 => SmallVec::with_capacity(_to_usize(first, 1)),
            2 => {
                let mut vec = Vec::new();
                for i in 0.._to_u8(first, 2) % 65 {
                    vec.push(_to_str(first, 3 + i as usize * 5, 3 + i as usize * 5 + 4).to_string());
                }
                SmallVec::from_vec(vec)
            },
            3 => {
                let vec = vec![
                    _to_str(first, 3, 19).to_string(),
                    _to_str(first, 20, 35).to_string()
                ];
                SmallVec::from_vec(vec)
            },
            _ => SmallVec::from_elem(_to_str(first, 3, 19).to_string(), (_to_u8(first, 20) % 65) as usize)
        };

        let ops = _to_u8(second, 0) % 65;
        let mut pos = 1;
        for _ in 0..ops {
            match _to_u8(second, pos) % 11 {
                0 => sv.push(_to_str(second, pos+1, pos+5).to_string()),
                1 => { sv.pop(); },
                2 => sv.truncate(_to_usize(second, pos+1)),
                3 => sv.insert(_to_usize(second, pos+1), _to_str(second, pos+9, pos+17).to_string()),
                4 => {
                    let drained = sv.drain();
                    let _ = drained.size_hint();
                },
                5 => {
                    let slice = sv.as_slice();
                    println!("{:?}", slice);
                },
                6 => {
                    let capacity = sv.capacity();
                    pos += capacity % 8;
                },
                7 => {
                    sv.shrink_to_fit();
                },
                8 => {
                    let len = sv.len();
                    if len > 0 {
                        let _ = sv.swap_remove(len - 1);
                    }
                },
                9 => {
                    sv.push(_to_str(second, pos+1, pos+9).to_string());
                },
                _ => {
                    let mut other = SmallVec::<[String; 32]>::new();
                    other.push(_to_str(second, pos+1, pos+9).to_string());
                    sv.extend(other.drain());
                },
            }
            pos += 18;
        }

        {
            let mut drain_iter = sv.drain();
            drain_iter.next_back();
            drain_iter.next();
            drain_iter.next_back();
        }

        let _ = sv.dedup();
        let _ = sv.shrink_to_fit();
        sv.insert(0, _to_str(second, pos, pos+8).to_string());
        println!("{:?}", sv.as_mut_slice());
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