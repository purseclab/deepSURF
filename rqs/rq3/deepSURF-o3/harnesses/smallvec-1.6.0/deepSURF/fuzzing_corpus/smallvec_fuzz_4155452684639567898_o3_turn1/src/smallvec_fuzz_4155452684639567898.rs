#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 150 {
            return;
        }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let mut buf = [0u8; 32];
        buf.copy_from_slice(&GLOBAL_DATA[0..32]);

        let mut cursor = 32;
        let selector = _to_u8(GLOBAL_DATA, cursor);
        cursor += 1;

        let mut sv: SmallVec<[u8; 32]> = match selector % 5 {
            0 => SmallVec::new(),
            1 => {
                let cap = _to_usize(GLOBAL_DATA, cursor) % 65;
                cursor += 8;
                SmallVec::with_capacity(cap)
            }
            2 => {
                let len_byte = _to_u8(GLOBAL_DATA, cursor) as usize;
                cursor += 1;
                let end = 33 + len_byte.min(GLOBAL_DATA.len() - 33);
                SmallVec::from_slice(&GLOBAL_DATA[33..end])
            }
            3 => SmallVec::from_buf(buf),
            _ => {
                let len = _to_usize(GLOBAL_DATA, cursor);
                cursor += 8;
                SmallVec::from_buf_and_len(buf, len)
            }
        };

        let op_cnt = _to_u8(GLOBAL_DATA, cursor) % 20;
        cursor += 1;

        for _ in 0..op_cnt {
            let op = _to_u8(GLOBAL_DATA, cursor);
            cursor += 1;
            match op % 10 {
                0 => {
                    let val = _to_u8(GLOBAL_DATA, cursor);
                    cursor += 1;
                    sv.push(val);
                }
                1 => {
                    sv.pop();
                }
                2 => {
                    let idx = _to_usize(GLOBAL_DATA, cursor);
                    cursor += 8;
                    if !sv.is_empty() {
                        let _ = sv.remove(idx);
                    }
                }
                3 => {
                    let idx = _to_usize(GLOBAL_DATA, cursor);
                    cursor += 8;
                    let val = _to_u8(GLOBAL_DATA, cursor);
                    cursor += 1;
                    sv.insert(idx, val);
                }
                4 => {
                    let new_len = _to_usize(GLOBAL_DATA, cursor) % 65;
                    cursor += 8;
                    sv.truncate(new_len);
                }
                5 => {
                    let additional = _to_usize(GLOBAL_DATA, cursor);
                    cursor += 8;
                    let _ = sv.try_reserve(additional);
                }
                6 => {
                    let slice_len = _to_u8(GLOBAL_DATA, cursor) as usize;
                    cursor += 1;
                    let start = cursor;
                    let end = start + slice_len.min(GLOBAL_DATA.len() - start);
                    sv.extend_from_slice(&GLOBAL_DATA[start..end]);
                    cursor = end;
                }
                7 => {
                    println!("{:?}", sv.as_slice());
                    println!("{:?}", sv.len());
                }
                8 => {
                    sv.dedup();
                }
                _ => {
                    println!("{:?}", sv.is_empty());
                }
            }
        }

        let res = sv.clone().into_inner();
        match res {
            Ok(arr) => {
                println!("{:?}", arr[0]);
                let mut sv2 = SmallVec::from_buf(arr);
                sv2.shrink_to_fit();
                println!("{:?}", sv2.capacity());
            }
            Err(mut rem) => {
                rem.clear();
                rem.resize_with(5, || 0);
                println!("{:?}", rem.len());
            }
        }

        let slice_ref = sv.as_slice();
        if let Some(first) = slice_ref.get(0) {
            println!("{:?}", *first);
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