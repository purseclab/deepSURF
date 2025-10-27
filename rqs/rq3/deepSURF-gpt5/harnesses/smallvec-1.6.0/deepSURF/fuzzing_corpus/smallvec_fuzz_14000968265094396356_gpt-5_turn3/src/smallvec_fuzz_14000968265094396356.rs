#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};
use std::borrow::Borrow;

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 196 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let first = global_data.first_half;
        let second = global_data.second_half;

        let sel = _to_u8(first, 0);
        let seed_len = (_to_u8(first, 1) as usize) % 65;
        let mut seed_vec: Vec<u8> = Vec::new();
        for i in 0..seed_len {
            if i < second.len() { seed_vec.push(second[i]); }
        }

        let mut arr32 = [0u8; 32];
        for i in 0..32 {
            arr32[i] = if i < second.len() { second[i] } else { first[i % first.len()] };
        }

        let mut sv: SmallVec<[u8; 32]> = match sel % 6 {
            0 => {
                let mut s = SmallVec::<[u8; 32]>::new();
                if !seed_vec.is_empty() { s.extend_from_slice(&seed_vec); }
                s
            }
            1 => {
                let mut s = SmallVec::<[u8; 32]>::with_capacity(_to_usize(first, 2));
                if !seed_vec.is_empty() { s.extend_from_slice(&seed_vec); }
                s
            }
            2 => SmallVec::<[u8; 32]>::from_vec(seed_vec.clone()),
            3 => SmallVec::<[u8; 32]>::from_slice(&seed_vec),
            4 => SmallVec::<[u8; 32]>::from_buf(arr32),
            _ => SmallVec::<[u8; 32]>::from_buf_and_len(arr32, _to_usize(first, 10)),
        };

        let op_count = (1 + (_to_u8(first, 20) % 25)) as usize;
        for i in 0..op_count {
            let op = _to_u8(first, 21 + (i % 40)) % 30;
            match op {
                0 => { if i < second.len() { sv.push(second[i]); } }
                1 => { let _ = sv.pop(); }
                2 => {
                    let idx = _to_usize(first, 30 + (i % 10));
                    let val = if i < second.len() { second[i] } else { 0 };
                    sv.insert(idx, val);
                }
                3 => {
                    let idx = _to_usize(first, 60 + (i % 10));
                    let _ = sv.remove(idx);
                }
                4 => {
                    let idx = _to_usize(first, 90 + (i % 10));
                    let _ = sv.swap_remove(idx);
                }
                5 => {
                    let new_len = _to_usize(first, 40);
                    let value = if i < second.len() { second[i] } else { 1 };
                    sv.resize(new_len, value);
                }
                6 => { let add = _to_usize(first, 50); sv.reserve(add); }
                7 => { let add = _to_usize(first, 51); sv.reserve_exact(add); }
                8 => { let add = _to_usize(first, 52); let _ = _unwrap_result(sv.try_reserve(add)); }
                9 => { let add = _to_usize(first, 53); let _ = _unwrap_result(sv.try_reserve_exact(add)); }
                10 => { sv.dedup(); }
                11 => {
                    sv.dedup_by(|a, b| {
                        if *a == _to_u8(first, 54) { *b = b.wrapping_add(1); }
                        *a == *b
                    });
                }
                12 => {
                    sv.dedup_by_key(|x| {
                        let k = (*x).wrapping_add(_to_u8(first, 55));
                        k
                    });
                }
                13 => {
                    sv.retain(|x| {
                        println!("{}", *x);
                        (*x % 2) == (_to_u8(first, 56) % 2)
                    });
                }
                14 => {
                    let idx = _to_usize(first, 57);
                    let it_len = (_to_u8(first, 58) as usize) % 65;
                    let iterable = (0..it_len).map(|k| if k < second.len() { second[k] } else { 0 });
                    sv.insert_many(idx, iterable);
                }
                15 => {
                    let ext_len = (_to_u8(first, 59) as usize) % 65;
                    let mut tmp = Vec::new();
                    for k in 0..ext_len {
                        if k < first.len() { tmp.push(first[k]); }
                    }
                    sv.extend_from_slice(&tmp);
                }
                16 => { let cap = _to_usize(first, 60); sv.grow(cap); }
                17 => { let cap = _to_usize(first, 61); let _ = _unwrap_result(sv.try_grow(cap)); }
                18 => { let l = _to_usize(first, 62); sv.truncate(l); }
                19 => { sv.clear(); }
                20 => { let r = sv.as_slice(); println!("{:?}", r); }
                21 => {
                    let r = sv.as_mut_slice();
                    if !r.is_empty() { r[0] = r[0].wrapping_add(1); }
                    println!("{:?}", r);
                }
                22 => {
                    let idx = _to_usize(first, 63);
                    let r = &sv[idx];
                    println!("{}", *r);
                }
                23 => {
                    let idx = _to_usize(first, 64);
                    let r = &mut sv[idx];
                    *r = r.wrapping_add(1);
                    println!("{}", *r);
                }
                24 => {
                    let start = _to_usize(first, 65);
                    let end = _to_usize(first, 66);
                    let mut dr = sv.drain(start..end);
                    let _ = dr.next();
                    let _ = dr.next_back();
                }
                25 => {
                    let l = sv.len();
                    let c = sv.capacity();
                    let e = sv.is_empty();
                    println!("{} {} {}", l, c, e);
                }
                26 => {
                    let sv2 = sv.clone();
                    let _ = sv.partial_cmp(&sv2);
                    let _ = sv.cmp(&sv2);
                    let _ = sv == sv2;
                }
                27 => {
                    let r1: &[u8] = sv.borrow();
                    println!("{:?}", r1);
                    let r2 = sv.as_ref();
                    println!("{:?}", r2);
                    let r3 = sv.deref();
                    println!("{:?}", r3);
                    let r4 = sv.deref_mut();
                    if !r4.is_empty() { r4[0] = r4[0].wrapping_add(1); }
                    println!("{:?}", r4);
                }
                28 => {
                    let mut other = SmallVec::<[u8; 32]>::from_slice(&seed_vec);
                    sv.append(&mut other);
                }
                29 => {
                    let cap = sv.capacity();
                    println!("{}", cap);
                }
                _ => {}
            }
        }

        let boxed1 = sv.clone().into_boxed_slice();
        let mut v_after: Vec<u8> = Vec::from(boxed1);
        let mut sv3 = SmallVec::<[u8; 32]>::from_vec(v_after.clone());
        sv3.push(_to_u8(first, 67));
        let sref = sv3.as_slice();
        println!("{:?}", sref);
        let boxed2 = sv3.into_boxed_slice();
        let v2: Vec<u8> = Vec::from(boxed2);
        println!("{}", v2.len());
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