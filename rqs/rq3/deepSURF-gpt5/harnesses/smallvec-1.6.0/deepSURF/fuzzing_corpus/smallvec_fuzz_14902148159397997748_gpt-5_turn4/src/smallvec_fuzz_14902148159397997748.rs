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
        if data.len() < 208 { return; }
        set_global_data(data);
        let gd = get_global_data();
        let g1 = gd.first_half;
        let g2 = gd.second_half;
        let l1 = g1.len();
        let l2 = g2.len();
        let base1 = (_to_u8(g1, 3) as usize) % l1;
        let base2 = (_to_u8(g2, 4) as usize) % l2;
        let arr16: [u8; 16] = std::array::from_fn(|i| g1[(base1 + i) % l1]);
        let arr32: [u8; 32] = std::array::from_fn(|i| g2[(base2 + i) % l2]);
        let n_vec = (_to_u8(g2, 0) % 65) as usize;
        let mut vbytes = Vec::with_capacity(n_vec);
        for i in 0..n_vec { vbytes.push(g2[(base2 + i) % l2]); }
        let sl_start = (_to_u8(g2, 2) as usize) % l2;
        let sl_len = (_to_u8(g2, 1) % 65) as usize;
        let sl_end = std::cmp::min(sl_start + sl_len, l2);
        let sl = &g2[sl_start..sl_end];
        let choice = _to_u8(g1, 0) % 6;
        let mut sv: SmallVec<[u8; 16]> = match choice {
            0 => SmallVec::<[u8; 16]>::new(),
            1 => SmallVec::<[u8; 16]>::with_capacity(_to_usize(g1, 1)),
            2 => SmallVec::<[u8; 16]>::from_vec(vbytes),
            3 => SmallVec::<[u8; 16]>::from_slice(sl),
            4 => SmallVec::<[u8; 16]>::from_buf(arr16),
            _ => SmallVec::<[u8; 16]>::from_buf_and_len(arr16, _to_usize(g1, 5)),
        };
        let _ = sv.capacity();
        let s0 = sv.as_slice();
        println!("{:?}", s0);
        let r0 = (&sv).deref();
        println!("{:?}", r0);
        let m0 = (&mut sv).deref_mut();
        println!("{:?}", m0);
        let s1 = sv.as_ref();
        println!("{:?}", s1);
        let m1 = sv.as_mut();
        println!("{:?}", m1);
        let ops = (_to_u8(g1, 10) % 20) as usize;
        let mut idxm = 0usize;
        for i in 0..ops {
            let code = _to_u8(g1, (11 + i) % l1) % 14;
            match code {
                0 => { sv.push(g2[(base2 + i) % l2]); }
                1 => { let _ = sv.pop(); }
                2 => { sv.insert(_to_usize(g1, 20), g2[(base2 + i) % l2]); }
                3 => { if sv.len() > 0 { let _ = sv.remove(_to_usize(g1, 28)); } }
                4 => { if sv.len() > 0 { let _ = sv.swap_remove(_to_usize(g1, 36)); } }
                5 => { sv.truncate(_to_usize(g1, 44)); }
                6 => { sv.reserve(_to_usize(g1, 52)); }
                7 => { let _ = sv.try_reserve(_to_usize(g1, 60)); }
                8 => {
                    let len_e = (_to_u8(g2, (i + 5) % l2) % 65) as usize;
                    let st = (_to_u8(g2, (i + 6) % l2) as usize) % l2;
                    let en = std::cmp::min(st + len_e, l2);
                    let ext = &g2[st..en];
                    sv.extend_from_slice(ext);
                }
                9 => { sv.resize_with(_to_usize(g1, 68), || { idxm = idxm.wrapping_add(1); g2[idxm % l2] }); }
                10 => { sv.dedup(); }
                11 => { sv.dedup_by(|a, b| { (*a ^ *b) & 1 == 0 }); }
                12 => { sv.retain(|x| { (*x & _to_u8(g2, (i + 7) % l2)) != 0 }); }
                _ => {
                    let k = (_to_u8(g2, (i + 8) % l2) % 65) as usize;
                    let mut itv = Vec::with_capacity(k);
                    for j in 0..k { itv.push(g2[(base2 + i + j) % l2]); }
                    sv.insert_many(_to_usize(g1, 76), itv);
                }
            }
            let s_loop = sv.as_slice();
            println!("{:?}", s_loop);
            let _ = (&sv).deref();
        }
        let r1 = (&sv).deref();
        println!("{:?}", r1);
        let s2 = &g2[sl_start..sl_end];
        let mut sv2: SmallVec<[u8; 16]> = s2.to_smallvec();
        let _ = sv.partial_cmp(&sv2);
        let _ = sv.cmp(&sv2);
        sv.append(&mut sv2);
        {
            let mut dr = sv.drain(0.._to_usize(g1, 88));
            let _ = dr.next();
            let _ = dr.next_back();
        }
        let mut it = sv.clone().into_iter();
        let rsl = it.as_slice();
        println!("{:?}", rsl);
        let rml = it.as_mut_slice();
        println!("{:?}", rml);
        let mut svf = SmallVec::<[u8; 16]>::from_iter(arr32.iter().cloned());
        let _ = (&svf).deref();
        let _ = svf.try_reserve_exact(_to_usize(g1, 96));
        let _ = svf.capacity();
        if svf.len() > 0 {
            let _v = &svf[0];
            println!("{}", *_v);
        }
        let bsl: &[u8] = std::borrow::Borrow::<[u8]>::borrow(&sv);
        println!("{:?}", bsl);
        let bml: &mut [u8] = std::borrow::BorrowMut::<[u8]>::borrow_mut(&mut sv);
        println!("{:?}", bml);
        let _ = sv.is_empty();
        let _ = sv.len();
        let _ = sv.into_vec();
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