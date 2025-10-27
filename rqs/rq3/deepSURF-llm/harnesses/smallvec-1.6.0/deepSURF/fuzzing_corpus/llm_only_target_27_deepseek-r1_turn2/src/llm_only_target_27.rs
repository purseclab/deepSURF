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

        let arr: [u8; 16] = [
            GLOBAL_DATA[0], GLOBAL_DATA[1], GLOBAL_DATA[2], GLOBAL_DATA[3],
            GLOBAL_DATA[4], GLOBAL_DATA[5], GLOBAL_DATA[6], GLOBAL_DATA[7],
            GLOBAL_DATA[8], GLOBAL_DATA[9], GLOBAL_DATA[10], GLOBAL_DATA[11],
            GLOBAL_DATA[12], GLOBAL_DATA[13], GLOBAL_DATA[14], GLOBAL_DATA[15]
        ];
        let mut sv = if t_15 {
            SmallVec::from_buf(arr)
        } else {
            SmallVec::from_slice(&arr[..t_5 % 16])
        };

        let num_ops = (t_5 % 8) + 1;
        for i in 0..num_ops {
            let op_byte = global_data.second_half.get(i).copied().unwrap_or(0);
            match op_byte % 7 {
                0 => sv.push(t_0),
                1 => { sv.pop(); },
                2 => sv.insert(t_1 as usize, t_0),
                3 => { sv.remove(t_2 as usize); },
                4 => {
                    let _drain = sv.drain(t_3 as usize..t_4 as usize);
                    println!("{:?}", _drain.size_hint());
                },
                5 => {
                    let predicate: fn(&mut u8) -> bool = if t_15 { |x| *x % 2 == 0 } else { |x| *x % 2 == 1 };
                    sv.retain(predicate);
                },
                6 => {
                    let new_len = t_5 % 65;
                    sv.resize_with(new_len, || t_0);
                },
                _ => sv.extend_from_slice(&arr[..t_6 as usize % 16]),
            }
            println!("{:?}", sv.as_slice());
        }

        let mut vec = sv.into_vec();
        vec.shrink_to_fit();
        let _ = SmallVec::<[String; 20]>::from_vec(vec![t_17.clone(); t_5 % 20]);
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