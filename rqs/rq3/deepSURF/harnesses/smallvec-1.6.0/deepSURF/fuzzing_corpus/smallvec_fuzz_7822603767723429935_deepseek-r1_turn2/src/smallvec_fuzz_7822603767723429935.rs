#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut};

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 256 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let mut vecs = vec![];
        let op_count = _to_u8(GLOBAL_DATA, 0) % 8 + 1;

        for i in 0..op_count {
            let i = i as usize;
            let method = _to_u8(GLOBAL_DATA, i + 1) % 6;
            let cap = _to_usize(GLOBAL_DATA, i * 4) % 65;

            match method {
                0 => {
                    let mut sv = SmallVec::<[String; 64]>::new();
                    vecs.push(sv);
                }
                1 => {
                    let mut sv = SmallVec::with_capacity(cap);
                    vecs.push(sv);
                }
                2 => {
                    let start = i * 10;
                    let end = start + 32;
                    let elem = String::from(_to_str(GLOBAL_DATA, start, end));
                    let mut sv = SmallVec::from_elem(elem, cap);
                    vecs.push(sv);
                }
                3 => {
                    let start = i * 8;
                    let end = start + 16;
                    let s = String::from(_to_str(GLOBAL_DATA, start, end));
                    let mut sv = SmallVec::from_vec(vec![s]);
                    vecs.push(sv);
                }
                4 => {
                    let v: Vec<String> = (0..cap).map(|_| String::new()).collect();
                    let mut sv = SmallVec::from_vec(v);
                    vecs.push(sv);
                }
                _ => {
                    let start = i * 12;
                    let end = start + 12;
                    let elem = String::from(_to_str(GLOBAL_DATA, start, end));
                    let len = _to_usize(GLOBAL_DATA, i * 3) % 65;
                    let elements = vec![elem; len];
                    let mut sv = SmallVec::from_vec(elements);
                    vecs.push(sv);
                }
            }
        }

        for sv in &mut vecs {
            let ops = _to_u8(GLOBAL_DATA, 128) % 8;
            match ops {
                0 => {
                    let idx1 = _to_usize(GLOBAL_DATA, 64);
                    let idx2 = _to_usize(GLOBAL_DATA, 72);
                    sv.drain(idx1..idx2);
                }
                1 => {
                    let val = String::from(_to_str(GLOBAL_DATA, 80, 96));
                    sv.push(val);
                }
                2 => {
                    sv.truncate(_to_usize(GLOBAL_DATA, 100) % 65);
                }
                3 => {
                    let slice = sv.as_mut();
                    println!("{:?}", slice);
                }
                4 => {
                    sv.insert(_to_usize(GLOBAL_DATA, 110), String::from(""));
                }
                5 => {
                    let val = String::from(_to_str(GLOBAL_DATA, 120, 130));
                    sv.push(val);
                }
                6 => {
                    let _ = sv.pop();
                }
                _ => {
                    let _ = sv.as_mut_slice().get_mut(0);
                }
            }

            let target_op = _to_u8(GLOBAL_DATA, 200) % 3;
            let mut guard = sv.as_mut();
            match target_op {
                0 => guard.fill(String::new()),
                1 => guard.reverse(),
                _ => { guard.as_mut_ptr(); },
            };

            if sv.capacity() % 2 == 0 {
                panic!("INTENTIONAL PANIC!");
            }
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