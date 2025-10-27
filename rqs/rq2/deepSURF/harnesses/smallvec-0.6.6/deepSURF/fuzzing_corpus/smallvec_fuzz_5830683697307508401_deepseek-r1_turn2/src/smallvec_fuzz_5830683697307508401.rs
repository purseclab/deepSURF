#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::cmp::Ordering;
use std::ops::{Deref, DerefMut};

struct CustomType0(String);
struct CustomType1(String);
struct CustomType2(String);

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 2048 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let data_half1 = global_data.first_half;
        let data_half2 = global_data.second_half;

        let constructor_selector = _to_u8(data_half1, 0) % 6;
        let mut sv = match constructor_selector {
            0 => SmallVec::<[u8; 64]>::new(),
            1 => SmallVec::with_capacity(_to_usize(data_half1, 1) % 65),
            2 => SmallVec::from_vec((0.._to_usize(data_half1, 1) % 65).map(|i| _to_u8(data_half1, 2 + i)).collect()),
            3 => SmallVec::from_slice(&data_half1[1..1 + (_to_usize(data_half1, 1) % 65).min(data_half1.len()-1)]),
            4 => SmallVec::from_elem(_to_u8(data_half1, 1), _to_usize(data_half1, 2) % 65),
            5 => {
                let start = 1;
                let mut arr = [0u8; 64];
                let slice = &data_half1[start..start+64];
                arr.copy_from_slice(slice);
                let len = _to_usize(data_half1, start + 64) % 65;
                SmallVec::from_buf_and_len(arr, len)
            },
            _ => unreachable!()
        };

        let op_count = _to_usize(data_half2, 0) % 20;
        let mut offset = 1;
        for _ in 0..op_count {
            if offset >= data_half2.len() { break; }
            let op = _to_u8(data_half2, offset) % 12;
            offset += 1;

            match op {
                0 => sv.push(_to_u8(data_half2, offset)),
                1 => { let idx = _to_usize(data_half2, offset); sv.insert(idx, _to_u8(data_half2, offset + 8)); },
                2 => sv.truncate(_to_usize(data_half2, offset)),
                3 => sv.reserve(_to_usize(data_half2, offset)),
                4 => sv.extend_from_slice(&data_half2[offset..offset + (_to_usize(data_half2, offset) % 65).min(data_half2.len()-offset)]),
                5 => { sv.pop(); },
                6 => { let idx = _to_usize(data_half2, offset); sv.remove(idx); },
                7 => { let mut drain = sv.drain(); while let Some(item) = drain.next() { println!("{:?}", item); } },
                8 => println!("{:?}", sv.as_slice()),
                9 => { let sv_clone = sv.clone(); println!("{:?}", sv.cmp(&sv_clone)); },
                10 => { let idx = _to_usize(data_half2, offset); println!("{:?}", sv.get(idx).map(|v| *v)); },
                11 => { let idx = _to_usize(data_half2, offset); sv[_to_usize(data_half2, idx)] = _to_u8(data_half2, offset + 8); },
                _ => unreachable!()
            }
            offset += match op { 1 | 4 | 7 | 10 | 11 => 16, 2 | 3 | 5 | 6 | 9 => 8, _ => 1 };
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