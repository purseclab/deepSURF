#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::SmallVec;
use global_data::*;

fn _custom_fn0(a: &mut String, b: &mut String) -> bool {
    let global_data = get_global_data();
    let t = _to_u8(global_data.first_half, 34);
    if t % 2 == 0 {
        panic!("INTENTIONAL PANIC!");
    }
    _to_bool(global_data.first_half, 35)
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 2048 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let mut op_count = _to_u8(global_data.first_half, 0) % 255;

        let constructor_choice = _to_u8(global_data.first_half, 1) % 4;
        let mut vec: SmallVec<[String; 16]> = match constructor_choice {
            0 => SmallVec::new(),
            1 => SmallVec::with_capacity(_to_usize(global_data.first_half, 2)),
            2 => {
                let elem = String::from(_to_str(global_data.first_half, 10, 20));
                SmallVec::from_elem(elem, 16)
            }
            _ => SmallVec::from_vec(vec![String::new()]),
        };

        for i in 0..op_count {
            let op_byte = _to_u8(global_data.first_half, 5 + i as usize) % 7;
            match op_byte {
                0 => {
                    let elem = String::from(_to_str(global_data.second_half, 300 + i as usize * 5, 305 + i as usize * 5));
                    vec.push(elem);
                }
                1 => {
                    if !vec.is_empty() {
                        vec.remove(_to_usize(global_data.second_half, i as usize * 2) % 65);
                    }
                }
                2 => vec.truncate(_to_usize(global_data.second_half, 200 + i as usize) % 65),
                3 => vec.dedup_by(|x, y| _custom_fn0(x, y)),
                4 => vec.extend([String::from("test"), String::from("test2")]),
                5 => vec.insert_many(_to_usize(global_data.second_half, 50), vec![String::new()]),
                _ => vec.reserve(_to_usize(global_data.second_half, 100)),
            }
        }

        vec.dedup_by(|x, y| _custom_fn0(x, y));
        let _ = vec.as_slice();
        println!("{:?}", vec.as_mut_slice());
        let _: Vec<_> = vec.drain(0..vec.len() / 2).collect();
        vec.shrink_to_fit();
        let _ = vec.into_vec();
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