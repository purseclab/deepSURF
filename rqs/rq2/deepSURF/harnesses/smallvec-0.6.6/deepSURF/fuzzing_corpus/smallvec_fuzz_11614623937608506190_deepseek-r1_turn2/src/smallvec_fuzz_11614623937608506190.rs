#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::SmallVec;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 512 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let constructor_selector = _to_u8(GLOBAL_DATA, 0) % 4;
        let mut sv: SmallVec<[String; 16]> = match constructor_selector {
            0 => SmallVec::new(),
            1 => SmallVec::with_capacity(_to_usize(GLOBAL_DATA, 1)),
            2 => SmallVec::from_vec(vec![String::new(); _to_usize(GLOBAL_DATA, 1) % 65]),
            3 => {
                let elements: Vec<_> = GLOBAL_DATA[1..=(_to_usize(GLOBAL_DATA, 2) % 64)]
                    .iter()
                    .map(|b| format!("{:x}", b))
                    .collect();
                SmallVec::from_vec(elements)
            }
            _ => SmallVec::default()
        };

        let mut cursor = 4;
        let op_count = _to_usize(GLOBAL_DATA, cursor) % 10;
        cursor += 1;

        for _ in 0..op_count {
            if cursor >= GLOBAL_DATA.len() { break; }
            let op_byte = _to_u8(GLOBAL_DATA, cursor);
            cursor += 1;

            match op_byte % 8 {
                0 => {
                    let idx = _to_usize(GLOBAL_DATA, cursor);
                    cursor += std::mem::size_of::<usize>();
                    let str_len = _to_u8(GLOBAL_DATA, cursor) % 65;
                    cursor += 1;
                    if cursor + str_len as usize > GLOBAL_DATA.len() { continue; }
                    sv.insert(idx, String::from(_to_str(GLOBAL_DATA, cursor, cursor + str_len as usize)));
                    cursor += str_len as usize;
                }
                1 => {
                    let len = sv.len();
                    if len > 0 { sv.truncate(_to_usize(GLOBAL_DATA, cursor) % (len + 1)); }
                    cursor += std::mem::size_of::<usize>();
                }
                2 => {
                    let pop_count = _to_u8(GLOBAL_DATA, cursor) % 5;
                    cursor += 1;
                    for _ in 0..pop_count { sv.pop(); }
                }
                3 => {
                    let count = _to_usize(GLOBAL_DATA, cursor) % 8;
                    cursor += 1;
                    sv.extend((0..count).map(|_| format!("{:x}", _to_u8(GLOBAL_DATA, cursor + count))));
                }
                4 => sv.shrink_to_fit(),
                5 => {
                    let _ = sv.drain();
                }
                6 => {
                    let cmp_vec = SmallVec::from_vec(vec!["cmp".into()]);
                    let _cmp = sv.partial_cmp(&cmp_vec);
                    println!("{:?}", _cmp);
                }
                _ => {
                    let elem = format!("{:02x}", _to_u8(GLOBAL_DATA, cursor));
                    cursor += 1;
                    sv.push(elem);
                }
            };
        }

        let _ = sv.capacity().wrapping_add(sv.len());
        println!("{:?}", sv.as_slice());
        let _slice: &[String] = sv.deref();
        let _mut_slice: &mut [String] = sv.deref_mut();
        let _ = sv.last().unwrap_or(&String::new()).clone();
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