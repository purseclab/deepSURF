#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::SmallVec;
use global_data::*;
use std::cmp::Ordering;

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 4096 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let (f, s) = (global_data.first_half, global_data.second_half);

        let mut sv = match _to_u8(f, 0) % 4 {
            0 => SmallVec::<[usize; 32]>::new(),
            1 => SmallVec::<[usize; 32]>::with_capacity(_to_usize(f, 1) % 65),
            2 => SmallVec::<[usize; 32]>::from_slice(&_to_vec_usize(f, 2, 32)),
            _ => SmallVec::<[usize; 32]>::from_elem(_to_usize(f, 64), _to_usize(f, 72) % 65)
        };

        for i in 0.._to_usize(s, 0) % 65 {
            match _to_u8(s, i * 8) % 7 {
                0 => sv.push(_to_usize(s, i * 8 + 1)),
                1 => { sv.pop(); sv.shrink_to_fit(); }
                2 => sv.clear(),
                3 => sv.insert(_to_usize(s, i * 8 + 1) % 65, _to_usize(s, i * 8 + 9)),
                4 => sv.truncate(_to_usize(s, i * 8 + 1)),
                5 => { 
                    let _ = sv.drain();
                    sv.extend_from_slice(&_to_vec_usize(s, i * 8 + 1, 16));
                }
                _ => {
                    let mut other = SmallVec::<[usize; 32]>::new();
                    other.extend(sv.clone());
                    println!("{:?}", sv.cmp(&other));
                }
            }
            if !sv.is_empty() {
                let idx = _to_usize(s, i * 8 + 17) % sv.len();
                println!("{:?}", sv[idx]);
                sv[idx] = _to_usize(s, i * 8 + 25);
            }
        }

        sv.clear();
        if sv.capacity() > 0 {
            println!("{:?}", sv.as_slice());
        }
    });
}

fn _to_vec_usize(data: &[u8], start: usize, count: usize) -> Vec<usize> {
    let mut v = Vec::new();
    for i in 0..count {
        let idx = start + i * 8;
        if idx + 8 > data.len() { break; }
        v.push(_to_usize(data, idx));
    }
    v
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