#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;    

use smallvec::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};
fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 105 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let t_0 = _to_u8(GLOBAL_DATA, 0);
        let t_1 = _to_u16(GLOBAL_DATA, 1);
        let t_2 = _to_u32(GLOBAL_DATA, 3);
        let t_3 = _to_u64(GLOBAL_DATA, 7);
        let t_4 = _to_u128(GLOBAL_DATA, 15);
        let t_5 = _to_usize(GLOBAL_DATA, 31);
        let t_6 = _to_i8(GLOBAL_DATA, 39);
        let t_7 = _to_i16(GLOBAL_DATA, 40);
        let t_8 = _to_i32(GLOBAL_DATA, 42);
        let t_9 = _to_i64(GLOBAL_DATA, 46);
        let t_10 = _to_i128(GLOBAL_DATA, 54);
        let t_11 = _to_isize(GLOBAL_DATA, 70);
        let t_12 = _to_f32(GLOBAL_DATA, 78);
        let t_13 = _to_f64(GLOBAL_DATA, 82);
        let t_14 = _to_char(GLOBAL_DATA, 90);
        let t_15 = _to_bool(GLOBAL_DATA, 94);
        let t_16 = _to_str(GLOBAL_DATA, 95, 105);
        let t_17 = String::from(t_16);

        let opt_t0 = Some(t_0);
        let unwrapped_opt = _unwrap_option(opt_t0);
        
        let res_t1: Result<u16, ()> = Ok(t_1);
        let unwrapped_res = _unwrap_result(res_t1);

        let mut vec1 = if t_15 {
            SmallVec::<[u8; 128]>::new()
        } else {
            SmallVec::with_capacity(t_5)
        };

        let elements_start = 105;
        let elements = GLOBAL_DATA.get(elements_start..).unwrap_or_default();
        let num_elements = t_5 % 65;
        for &byte in elements.iter().take(num_elements) {
            vec1.push(byte);
        }

        let mut vec2 = SmallVec::<[u8; 128]>::from_slice(t_16.as_bytes());

        let pre_ops = t_7 % 5;
        for _ in 0..pre_ops {
            match t_3 % 4 {
                0 => vec1.push(t_0),
                1 => { vec1.pop(); },
                2 => vec1.truncate(t_8 as usize % 65),
                3 => vec1.insert(t_9 as usize % (vec1.len() + 1), t_1 as u8),
                _ => (),
            }
        }

        vec1.append(&mut vec2);

        for elem in &vec1 {
            println!("{}", *elem);
        }

        let post_ops = t_6 % 5;
        for _ in 0..post_ops {
            match t_10 % 3 {
                0 => vec1.dedup(),
                1 => vec1.retain(|x| *x != t_2 as u8),
                2 => vec1.extend_from_slice(&GLOBAL_DATA[0..t_5 as usize % 65]),
                _ => (),
            }
        }

        if !vec1.is_empty() {
            let idx = (t_11 as usize) % vec1.len();
            println!("{}", *vec1.index(idx));
        }

        let mut iter = vec1.drain(..);
        while let Some(elem) = iter.next() {
            println!("{}", elem);
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