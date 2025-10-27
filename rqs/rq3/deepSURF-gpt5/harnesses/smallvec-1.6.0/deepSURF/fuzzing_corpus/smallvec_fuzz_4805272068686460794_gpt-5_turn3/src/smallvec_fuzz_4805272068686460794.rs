#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};
use std::borrow::{Borrow, BorrowMut};

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 192 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let CONTROL = global_data.second_half;
        let base_val = _to_i32(GLOBAL_DATA, 0);
        let cap0 = _to_usize(GLOBAL_DATA, 8);
        let count_a = (_to_u8(GLOBAL_DATA, 16) % 65) as usize;
        let count_b = (_to_u8(GLOBAL_DATA, 17) % 65) as usize;
        let mut vec_a: Vec<i32> = Vec::new();
        for i in 0..count_a {
            let v = _to_i32(GLOBAL_DATA, 18 + ((i % 10) as usize) * 4);
            vec_a.push(v);
        }
        let mut vec_b: Vec<i32> = Vec::new();
        for i in 0..count_b {
            let v = _to_i32(GLOBAL_DATA, 22 + ((i % 10) as usize) * 4);
            vec_b.push(v);
        }
        let slice_a: &[i32] = &vec_a[..];
        let array_init_val = _to_i32(GLOBAL_DATA, 58);
        let buf12: [i32; 12] = [array_init_val; 12];
        let selector = _to_u8(CONTROL, 0);
        let mut sv: SmallVec<[i32; 12]> = match selector % 7 {
            0 => SmallVec::<[i32; 12]>::new(),
            1 => SmallVec::<[i32; 12]>::with_capacity(cap0),
            2 => SmallVec::<[i32; 12]>::from_elem(base_val, _to_usize(GLOBAL_DATA, 22)),
            3 => SmallVec::<[i32; 12]>::from_vec(vec_a.clone()),
            4 => SmallVec::<[i32; 12]>::from_slice(slice_a),
            5 => SmallVec::<[i32; 12]>::from_buf(buf12),
            _ => SmallVec::<[i32; 12]>::from_buf_and_len(buf12, _to_usize(GLOBAL_DATA, 30)),
        };
        let mut other: SmallVec<[i32; 12]> = slice_a.to_smallvec();
        let add0 = _to_usize(GLOBAL_DATA, 32);
        sv.reserve(add0);
        let add1 = _to_usize(GLOBAL_DATA, 40);
        sv.reserve_exact(add1);
        let _ = sv.try_reserve(_to_usize(GLOBAL_DATA, 46));
        let _ = sv.try_reserve_exact(_to_usize(GLOBAL_DATA, 52));
        sv.extend_from_slice(slice_a);
        sv.push(_to_i32(GLOBAL_DATA, 62));
        let sref1 = sv.as_slice();
        if !sref1.is_empty() { println!("{}", sref1[0]); }
        let sref2 = sv.as_ref();
        println!("{}", sref2.len());
        let srefm = sv.as_mut_slice();
        if !srefm.is_empty() { srefm[0] = srefm[0]; println!("{}", srefm[0]); }
        let dref = sv.deref();
        println!("{}", dref.len());
        let mode = _to_u8(CONTROL, 1);
        if mode % 3 == 0 {
            sv.retain(|_| {
                let f = _to_u8(GLOBAL_DATA, 64);
                if f % 2 == 0 { panic!("INTENTIONAL PANIC!"); }
                true
            });
        }
        if mode % 5 == 1 {
            sv.dedup_by(|a, b| {
                if _to_u8(GLOBAL_DATA, 65) % 7 == 0 { panic!("INTENTIONAL PANIC!"); }
                *a == *b
            });
        }
        let ops = (_to_u8(CONTROL, 2) % 16) as usize;
        for i in 0..ops {
            let op = _to_u8(CONTROL, 3 + i) % 14;
            match op {
                0 => {
                    let add = _to_usize(GLOBAL_DATA, 66 + i);
                    sv.reserve(add);
                    let slice = sv.as_slice();
                    if !slice.is_empty() { println!("{}", slice[0]); }
                }
                1 => {
                    if sv.len() > 0 {
                        let idx = _to_usize(GLOBAL_DATA, 70 + i);
                        let _r = sv.remove(idx);
                    }
                }
                2 => {
                    let val = _to_i32(GLOBAL_DATA, 36);
                    sv.push(val);
                }
                3 => {
                    let idx = _to_usize(GLOBAL_DATA, 44 + i);
                    let val = _to_i32(GLOBAL_DATA, 68);
                    sv.insert(idx, val);
                }
                4 => {
                    let idx = _to_usize(GLOBAL_DATA, 50 + i);
                    let _ = sv.swap_remove(idx);
                }
                5 => {
                    sv.truncate(_to_usize(GLOBAL_DATA, 56));
                }
                6 => {
                    sv.grow(_to_usize(GLOBAL_DATA, 60));
                }
                7 => {
                    let r = _to_usize(GLOBAL_DATA, 72 + i);
                    let mut dr = sv.drain(0..r);
                    let _ = dr.next();
                    let _ = dr.next_back();
                }
                8 => {
                    sv.append(&mut other);
                }
                9 => {
                    let eq = sv.eq(&other);
                    println!("{}", eq);
                }
                10 => {
                    let ord = sv.cmp(&other);
                    match ord {
                        std::cmp::Ordering::Less => println!("L"),
                        std::cmp::Ordering::Equal => println!("E"),
                        std::cmp::Ordering::Greater => println!("G"),
                    }
                }
                11 => {
                    let mut it = sv.clone().into_iter();
                    let _ = it.clone();
                    let _ = it.next();
                    let _ = it.next_back();
                }
                12 => {
                    let ms: &mut [i32] = sv.borrow_mut();
                    if !ms.is_empty() { println!("{}", ms[0]); }
                }
                _ => {
                    let mm = sv.deref_mut();
                    if !mm.is_empty() { mm[0] = mm[0]; println!("{}", mm[0]); }
                }
            }
        }
        sv.reserve(_to_usize(GLOBAL_DATA, 74));
        sv.shrink_to_fit();
        let _cap2 = sv.capacity();
        println!("{}", _cap2);
        let _len2 = sv.len();
        println!("{}", _len2);
        let _empty = sv.is_empty();
        println!("{}", _empty);
        if sv.len() > 0 {
            let idx = _to_usize(GLOBAL_DATA, 76);
            let r = &sv[idx];
            println!("{}", *r);
        }
        sv.insert_many(_to_usize(GLOBAL_DATA, 78), vec_b.clone());
        let _ = sv.try_grow(_to_usize(GLOBAL_DATA, 80));
        let _ = sv.pop();
        let brr: &[i32] = sv.borrow();
        if !brr.is_empty() { println!("{}", brr[0]); }
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