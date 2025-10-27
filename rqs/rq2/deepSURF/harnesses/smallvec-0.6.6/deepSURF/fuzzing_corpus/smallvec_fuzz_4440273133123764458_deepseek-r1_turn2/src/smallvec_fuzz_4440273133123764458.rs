#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::ops::{DerefMut, IndexMut};
use std::fmt::{Debug, Write};

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 256 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let first_half = global_data.first_half;

        let construct_sel = _to_u8(first_half, 0) % 3;
        let mut sv: SmallVec<[String; 16]> = match construct_sel {
            0 => SmallVec::new(),
            1 => SmallVec::with_capacity(_to_usize(first_half, 1)),
            2 => {
                let elem_count = _to_usize(first_half, 1) % 65;
                let elem = _to_str(first_half, 2, 6).to_string();
                SmallVec::from_elem(elem, elem_count)
            }
            _ => SmallVec::new()
        };

        let op_count = _to_u8(first_half, 10) % 10;
        for i in 0..op_count {
            let op_byte = _to_u8(first_half, (11 + i) as usize) % 7;
            match op_byte {
                0 => sv.push(_to_str(first_half, (20 + i*4) as usize, (24 + i*4) as usize).to_string()),
                1 => { sv.pop(); }
                2 => sv.insert(_to_usize(first_half, (30 + i*8) as usize), _to_str(first_half, (34 + i*8) as usize, (38 + i*8) as usize).to_string()),
                3 => sv.truncate(_to_usize(first_half, (40 + i*8) as usize)),
                4 => { sv.swap_remove(_to_usize(first_half, 50 as usize)); }
                5 => sv.extend([_to_str(first_half, 60, 64).to_string(), _to_str(first_half, 64, 68).to_string()]),
                6 => { let _ = sv.as_mut_slice().get_mut(0).map(|s| *s = String::new()); }
                _ => ()
            }
        }

        sv.dedup_by(&mut _custom_fn0);

        if let Some(first) = sv.as_slice().first() {
            println!("{:?}", first);
        }

        let _ = sv.drain();
        let mut s = String::new();
        write!(&mut s, "{:?}", sv).ok();
    });
}

fn _custom_fn0(_: &mut String, _: &mut String) -> bool {
    let global_data = get_global_data();
    let first_half = global_data.first_half;
    let t_9 = _to_u8(first_half, 49);
    if t_9 % 2 == 0 {
        panic!("INTENTIONAL PANIC!");
    }
    _to_bool(first_half, 50)
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