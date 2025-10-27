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
        if data.len() < 96 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let first = global_data.first_half;
        let second = global_data.second_half;
        let tag = _to_u8(first, 0);
        let mut arr16 = [0u8; 16];
        for i in 0..16 {
            arr16[i] = _to_u8(first, 10 + i);
        }
        let len_for_buf = _to_usize(first, 28);
        let mut v: SmallVec<[u8; 16]> = match tag % 6 {
            0 => SmallVec::<[u8; 16]>::new(),
            1 => SmallVec::<[u8; 16]>::with_capacity(_to_usize(first, 1)),
            2 => SmallVec::<[u8; 16]>::from_vec({
                let mut vec = Vec::new();
                let n = (_to_u8(first, 2) % 65) as usize;
                let n2 = std::cmp::min(n, second.len());
                for i in 0..n2 {
                    vec.push(_to_u8(second, i));
                }
                vec
            }),
            3 => SmallVec::<[u8; 16]>::from_buf(arr16),
            4 => SmallVec::<[u8; 16]>::from_buf_and_len(arr16, len_for_buf),
            _ => SmallVec::<[u8; 16]>::from_slice({
                let n = (_to_u8(second, 0) % 65) as usize;
                let end = 2 + n;
                if end <= second.len() { &second[2..end] } else { &second[2..second.len()] }
            }),
        };
        let vb_slice = {
            let n = (_to_u8(second, 1) % 65) as usize;
            let end = 3 + n;
            if end <= second.len() { &second[3..end] } else { &second[3..second.len()] }
        };
        let mut vb = SmallVec::<[u8; 16]>::from_slice(vb_slice);
        let _ = v.len();
        let _ = v.is_empty();
        let _ = v.capacity();
        let pushes = (_to_u8(first, 5) % 8) as usize;
        for i in 0..pushes {
            v.push(_to_u8(second, i));
        }
        v.append(&mut vb);
        if _to_bool(first, 6) { v.insert(_to_usize(second, 10), _to_u8(first, 7)); }
        if _to_bool(first, 8) { let _ = v.pop(); }
        let _ = v.is_empty();
        {
            let s_mut = v.as_mut();
            println!("{:?}", &*s_mut);
            if !s_mut.is_empty() {
                let i = (_to_u8(first, 12)) as usize;
                let j = i % s_mut.len();
                s_mut[j] = s_mut[j].wrapping_add(_to_u8(second, 12));
            }
        }
        v.retain(|e| {
            if _to_u8(first, 13) % 2 == 0 { panic!("INTENTIONAL PANIC!"); }
            *e = e.wrapping_add(_to_u8(second, 13));
            true
        });
        v.dedup();
        let r = v.as_ref();
        println!("{:?}", &*r);
        let r2: &[u8] = v.borrow();
        println!("{:?}", &*r2);
        let rm: &mut [u8] = v.borrow_mut();
        println!("{:?}", &*rm);
        let ops = (_to_u8(first, 14) % 12) as usize;
        for k in 0..ops {
            match _to_u8(first, 15 + (k % 8) as usize) % 12 {
                0 => v.push(_to_u8(first, 20 + (k % 8) as usize)),
                1 => { let _ = v.pop(); }
                2 => v.truncate(_to_usize(first, 22)),
                3 => {
                    let slice_len = (_to_u8(second, 16) % 65) as usize;
                    let end = 1 + slice_len;
                    let sl = if end <= second.len() { &second[1..end] } else { &second[1..second.len()] };
                    v.extend_from_slice(sl);
                }
                4 => { let _ = v.try_reserve(_to_usize(second, 18)); }
                5 => { let _ = v.try_reserve_exact(_to_usize(first, 24)); }
                6 => {
                    let idx = _to_usize(second, 20);
                    v.insert_from_slice(idx, {
                        let n = (_to_u8(first, 26) % 65) as usize;
                        let end = 3 + n;
                        if end <= first.len() { &first[3..end] } else { &first[3..first.len()] }
                    });
                }
                7 => {
                    if !v.is_empty() {
                        let idx = _to_usize(first, 28);
                        let _ = v.remove(idx);
                    }
                }
                8 => {
                    let d = v.drain(_to_usize(second, 22)..);
                    drop(d);
                }
                9 => {
                    v.dedup_by(|a, b| {
                        if _to_u8(first, 30) % 3 == 0 { panic!("INTENTIONAL PANIC!"); }
                        *a == *b
                    });
                }
                10 => {
                    let count = (_to_u8(second, 2) % 10) as usize;
                    v.insert_many(_to_usize(first, 32), std::iter::repeat(_to_u8(second, 3)).take(count));
                }
                _ => {
                    let s = v.as_slice();
                    println!("{:?}", &*s);
                }
            }
            let s_mut2 = v.as_mut();
            println!("{:?}", &*s_mut2);
        }
        let vc = SmallVec::<[u8; 16]>::from_slice(&first[0..std::cmp::min(first.len(), 16)]);
        let _ = v.partial_cmp(&vc);
        let _ = v.cmp(&vc);
        let eq = v.eq(&vc);
        if eq { println!("{:?}", &*v.as_slice()); }
        let dr = v.deref();
        println!("{:?}", &*dr);
        let drm = v.deref_mut();
        println!("{:?}", &*drm);
        if v.len() > 0 {
            let idx = _to_usize(second, 24);
            let r = &v[idx];
            println!("{}", *r as u8);
            let idx2 = _to_usize(second, 26);
            let rmut = &mut v[idx2];
            let old = *rmut;
            *rmut = old.wrapping_add(1);
        }
        let mut it = v.clone().into_iter();
        let s_it = it.as_slice();
        println!("{:?}", &*s_it);
        let s_itm = it.as_mut_slice();
        println!("{:?}", &*s_itm);
        let _ = it.next();
        let _ = it.next_back();
        let vec_back = v.clone().into_vec();
        let mut v2 = SmallVec::<[u8; 32]>::from_vec(vec_back);
        let s_mut3 = v2.as_mut();
        println!("{:?}", &*s_mut3);
        let _ = v2.into_boxed_slice();
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