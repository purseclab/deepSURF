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
        if data.len() < 200 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let OTHER = global_data.second_half;

        let l_vec_len = (_to_u8(GLOBAL_DATA, 9) % 65) as usize;
        let mut buf_vec: Vec<i32> = Vec::with_capacity(l_vec_len);
        for i in 0..l_vec_len {
            let b = _to_i8(GLOBAL_DATA, 10 + i) as i32;
            buf_vec.push(b);
        }

        let mut arr16: [i32; 16] = [0; 16];
        for i in 0..16 {
            arr16[i] = _to_i8(GLOBAL_DATA, 30 + i) as i32;
        }
        let mut arr32u8: [u8; 32] = [0; 32];
        for i in 0..32 {
            arr32u8[i] = _to_u8(GLOBAL_DATA, 48 + i);
        }

        let choice = _to_u8(GLOBAL_DATA, 0);
        let mut sv_i32: SmallVec<[i32; 16]> = match choice % 6 {
            0 => SmallVec::<[i32; 16]>::new(),
            1 => SmallVec::<[i32; 16]>::with_capacity(_to_usize(GLOBAL_DATA, 2)),
            2 => SmallVec::<[i32; 16]>::from_vec(buf_vec.clone()),
            3 => SmallVec::<[i32; 16]>::from_slice(&buf_vec[..]),
            4 => SmallVec::<[i32; 16]>::from_iter(buf_vec.clone().into_iter()),
            _ => SmallVec::<[i32; 16]>::from_elem(_to_i32(GLOBAL_DATA, 4), _to_usize(GLOBAL_DATA, 6)),
        };

        let choice2 = _to_u8(GLOBAL_DATA, 1);
        let mut sv_u8: SmallVec<[u8; 32]> = match choice2 % 5 {
            0 => SmallVec::<[u8; 32]>::from_buf(arr32u8),
            1 => SmallVec::<[u8; 32]>::from_buf_and_len(arr32u8, _to_usize(GLOBAL_DATA, 12)),
            2 => SmallVec::<[u8; 32]>::from_slice(&arr32u8[..]),
            3 => SmallVec::<[u8; 32]>::with_capacity(_to_usize(GLOBAL_DATA, 14)),
            _ => SmallVec::<[u8; 32]>::new(),
        };

        sv_i32.reserve(_to_usize(GLOBAL_DATA, 16));
        sv_i32.try_reserve(_to_usize(GLOBAL_DATA, 18)).ok();
        sv_i32.push(_to_i32(GLOBAL_DATA, 20));
        if !sv_i32.is_empty() {
            let r = &sv_i32[sv_i32.len() - 1];
            println!("{:?}", *r);
        }
        let s_ref = sv_i32.as_slice();
        println!("{:?}", s_ref);
        let s_mut = sv_i32.as_mut_slice();
        if !s_mut.is_empty() { s_mut[0] = s_mut[0].wrapping_add(1); }
        println!("{:?}", s_mut);

        let cloned1 = sv_i32.clone();
        let eq = sv_i32.eq(&cloned1);
        println!("{:?}", eq);
        let _ = sv_i32.cmp(&cloned1);
        let _ = sv_i32.partial_cmp(&cloned1);
        let cap = sv_i32.capacity();
        println!("{:?}", cap);

        let ops = (_to_u8(OTHER, 0) % 20) as usize;
        for j in 0..ops {
            let sel = _to_u8(OTHER, 1 + j % (OTHER.len().saturating_sub(2))) % 12;
            match sel {
                0 => {
                    sv_i32.insert(_to_usize(OTHER, 2 + j % (OTHER.len().saturating_sub(10))), _to_i32(OTHER, 3 + j % (OTHER.len().saturating_sub(11))));
                }
                1 => {
                    if !sv_i32.is_empty() { let _ = sv_i32.remove(_to_usize(OTHER, 4 + j % (OTHER.len().saturating_sub(12)))); }
                }
                2 => {
                    if !sv_i32.is_empty() { let _ = sv_i32.swap_remove(_to_usize(OTHER, 5 + j % (OTHER.len().saturating_sub(13)))); }
                }
                3 => {
                    sv_i32.truncate(_to_usize(OTHER, 6 + j % (OTHER.len().saturating_sub(14))));
                }
                4 => {
                    sv_i32.resize(_to_usize(OTHER, 7 + j % (OTHER.len().saturating_sub(15))), _to_i32(OTHER, 8 + j % (OTHER.len().saturating_sub(16))));
                }
                5 => {
                    let mut other_sv = SmallVec::<[i32; 16]>::from_vec(buf_vec.clone());
                    sv_i32.append(&mut other_sv);
                }
                6 => {
                    let slice_len = (_to_u8(OTHER, 9 + j % (OTHER.len().saturating_sub(17))) % 65) as usize;
                    let mut tmpv: Vec<i32> = Vec::with_capacity(slice_len);
                    for i in 0..slice_len {
                        let b = _to_i8(OTHER, 10 + (j + i) % (OTHER.len().saturating_sub(18))) as i32;
                        tmpv.push(b);
                    }
                    sv_i32.extend_from_slice(&tmpv[..]);
                }
                7 => {
                    let idx = _to_usize(OTHER, 11 + j % (OTHER.len().saturating_sub(19)));
                    let slice_len = (_to_u8(OTHER, 12 + j % (OTHER.len().saturating_sub(20))) % 65) as usize;
                    let mut tmpv: Vec<i32> = Vec::with_capacity(slice_len);
                    for i in 0..slice_len {
                        let b = _to_i8(OTHER, 13 + (j + i) % (OTHER.len().saturating_sub(21))) as i32;
                        tmpv.push(b);
                    }
                    sv_i32.insert_from_slice(idx, &tmpv[..]);
                }
                8 => {
                    sv_i32.dedup_by(|a, b| {
                        let v = _to_u8(GLOBAL_DATA, 22);
                        if v % 2 == 0 { panic!("INTENTIONAL PANIC!"); }
                        *a == *b
                    });
                }
                9 => {
                    sv_i32.retain(|x| {
                        let mut y = *x;
                        y = y.wrapping_add(_to_i8(GLOBAL_DATA, 23) as i32);
                        y % 2 == 0
                    });
                }
                10 => {
                    sv_i32.dedup_by_key(|x| {
                        let k = x.wrapping_add(_to_i8(GLOBAL_DATA, 24) as i32);
                        k
                    });
                }
                _ => {
                    let r = 0.._to_usize(OTHER, 15 + j % (OTHER.len().saturating_sub(25)));
                    let mut dr = sv_i32.drain(r);
                    let _ = dr.next();
                    let _ = dr.next_back();
                }
            }
            let _ = sv_i32.clone();
        }

        let asref_slice = sv_i32.as_ref();
        println!("{:?}", asref_slice);
        let asmut_slice = sv_i32.as_mut();
        if !asmut_slice.is_empty() { asmut_slice[asmut_slice.len() - 1] = asmut_slice[asmut_slice.len() - 1].wrapping_sub(1); }
        println!("{:?}", asmut_slice);

        let rb: &[i32] = sv_i32.borrow();
        println!("{:?}", rb);
        let bm: &mut [i32] = sv_i32.borrow_mut();
        println!("{:?}", bm);

        if !sv_i32.is_empty() {
            let idx_slice = 0..1;
            let r = &sv_i32.index(idx_slice);
            println!("{:?}", *r);
            let r2 = sv_i32.index_mut(0..1);
            if !r2.is_empty() { r2[0] = r2[0].wrapping_add(2); }
        }

        let it = sv_i32.clone().into_iter();
        let mut it2 = it;
        let _ = it2.as_slice();
        let m = it2.as_mut_slice();
        println!("{:?}", m);
        let _ = it2.next();
        let _ = it2.next_back();

        let v = sv_i32.clone().into_vec();
        println!("{:?}", v.len());

        let b = sv_i32.clone().into_boxed_slice();
        println!("{:?}", b.len());

        let r2 = sv_i32.len();
        println!("{:?}", r2);

        let _ = sv_u8.clone();
        sv_u8.push(_to_u8(GLOBAL_DATA, 25));
        if !sv_u8.is_empty() {
            let r = &sv_u8[0];
            println!("{:?}", *r);
        }
        let _ = sv_u8.pop();
        sv_u8.reserve_exact(_to_usize(GLOBAL_DATA, 26));
        sv_u8.shrink_to_fit();
        let dr2 = sv_u8.drain(0.._to_usize(GLOBAL_DATA, 28));
        let mut dr3 = dr2;
        let _ = dr3.next();
        let _ = dr3.next_back();
        std::mem::drop(dr3);

        let s3 = sv_u8.as_slice();
        println!("{:?}", s3);
        let s4 = sv_u8.as_mut_slice();
        if !s4.is_empty() { s4[0] = s4[0].wrapping_add(1); }
        println!("{:?}", s4);

        let cloned_final = sv_i32.clone();
        println!("{:?}", cloned_final.len());
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