#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};
use std::borrow::Borrow;

fn complex_ops(
    sv: &mut SmallVec<[i32; 32]>,
    first: &[u8],
    helper_slice: &[i32],
    idx_insert: usize,
    idx_remove: usize,
    idx_swap_remove: usize,
    trunc_to: usize,
    reserve_add: usize,
    grow_to: usize,
    drain_start: usize,
    drain_end: usize,
    op_count: usize,
    repeat_val: i32,
) {
    for i in 0..op_count {
        let tag = _to_u8(first, 84 + (i % 3)) as usize;
        match tag % 20 {
            0 => {
                sv.push(_to_i32(first, 9));
                sv.push(_to_i32(first, 13));
            }
            1 => {
                sv.extend_from_slice(helper_slice);
            }
            2 => {
                let _ = sv.pop();
            }
            3 => {
                if !helper_slice.is_empty() {
                    sv.insert_from_slice(idx_insert, helper_slice);
                }
            }
            4 => {
                sv.insert(idx_insert, _to_i32(first, 13));
            }
            5 => {
                if !sv.is_empty() {
                    let _ = sv.remove(idx_remove);
                }
            }
            6 => {
                if !sv.is_empty() {
                    let _ = sv.swap_remove(idx_swap_remove);
                }
            }
            7 => {
                sv.truncate(trunc_to);
            }
            8 => {
                let _ = _unwrap_result(sv.try_grow(grow_to));
            }
            9 => {
                sv.reserve(reserve_add);
            }
            10 => {
                let _ = sv.len();
                let _ = sv.is_empty();
                println!("{}", sv.capacity());
            }
            11 => {
                let sref = sv.as_slice();
                if !sref.is_empty() {
                    let r = &sref[0];
                    println!("{}", *r);
                }
                let ms = sv.as_mut_slice();
                if !ms.is_empty() {
                    ms[0] = ms[0].wrapping_add(1);
                    println!("{}", ms[0]);
                }
            }
            12 => {
                sv.retain(|x| {
                    let b = _to_bool(first, 84);
                    if b {
                        *x % 2 == 0
                    } else {
                        true
                    }
                });
            }
            13 => {
                sv.dedup();
                sv.dedup_by(|a, b| *a == *b);
            }
            14 => {
                sv.dedup_by_key(|x| *x % 3);
            }
            15 => {
                let mut it = sv.clone().into_iter();
                let sl = it.as_slice();
                if !sl.is_empty() {
                    println!("{}", sl[0]);
                }
                let _ = it.next();
                let _ = it.next_back();
                let ml = it.as_mut_slice();
                if !ml.is_empty() {
                    ml[0] = ml[0].wrapping_add(repeat_val);
                    println!("{}", ml[0]);
                }
            }
            16 => {
                let mut other = SmallVec::<[i32; 32]>::from_slice(helper_slice);
                sv.append(&mut other);
            }
            17 => {
                let b1: &[i32] = sv.as_slice();
                println!("{:?}", b1);
                let b2 = sv.as_ref();
                println!("{:?}", b2);
                let bm = sv.as_mut();
                if !bm.is_empty() {
                    bm[0] = bm[0].wrapping_sub(1);
                    println!("{}", bm[0]);
                }
            }
            18 => {
                let mut d = sv.drain(drain_start..drain_end);
                let _ = d.next();
                let _ = d.next_back();
                std::mem::drop(d);
            }
            _ => {
                let other = SmallVec::<[i32; 32]>::from_slice(helper_slice);
                let _ = (*sv).partial_cmp(&other);
                let o = (*sv).cmp(&other);
                println!("{:?}", o);
                println!("{}", (*sv).eq(&other));
            }
        }
    }
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 182 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let first = global_data.first_half;
        let second = global_data.second_half;

        let choice = _to_u8(first, 0);
        let cap = _to_usize(first, 1);
        let pv1 = _to_i32(first, 9);
        let pv2 = _to_i32(first, 13);
        let idx_remove = _to_usize(first, 17);
        let idx_insert = _to_usize(first, 25);
        let idx_swap_remove = _to_usize(first, 33);
        let trunc_to = _to_usize(first, 41);
        let reserve_add = _to_usize(first, 49);
        let grow_to = _to_usize(first, 57);
        let drain_start = _to_usize(first, 65);
        let drain_end = _to_usize(first, 73);
        let op_count = (_to_u8(first, 81) % 17) as usize;
        let vec_len_mod = (_to_u8(first, 82) % 65) as usize;
        let repeat_val = _to_i32(second, 0);

        let helper_vec: Vec<i32> = vec![repeat_val; vec_len_mod];
        let helper_slice: &[i32] = &helper_vec;

        match choice % 6 {
            0 => {
                let mut sv: SmallVec<[i32; 32]> = SmallVec::new();
                sv.push(pv1);
                sv.push(pv2);
                complex_ops(&mut sv, first, helper_slice, idx_insert, idx_remove, idx_swap_remove, trunc_to, reserve_add, grow_to, drain_start, drain_end, op_count, repeat_val);
                let s = sv.as_slice();
                if !s.is_empty() { println!("{}", s[0]); }
                let ms = sv.as_mut_slice();
                if !ms.is_empty() { ms[0] = ms[0].wrapping_add(1); println!("{}", ms[0]); }
                std::mem::drop(sv);
            }
            1 => {
                let mut sv: SmallVec<[i32; 32]> = SmallVec::with_capacity(cap);
                sv.extend_from_slice(helper_slice);
                complex_ops(&mut sv, first, helper_slice, idx_insert, idx_remove, idx_swap_remove, trunc_to, reserve_add, grow_to, drain_start, drain_end, op_count, repeat_val);
                std::mem::drop(sv);
            }
            2 => {
                let mut sv: SmallVec<[i32; 32]> = SmallVec::from_elem(repeat_val, vec_len_mod);
                sv.insert(idx_insert, pv1);
                complex_ops(&mut sv, first, helper_slice, idx_insert, idx_remove, idx_swap_remove, trunc_to, reserve_add, grow_to, drain_start, drain_end, op_count, repeat_val);
                std::mem::drop(sv);
            }
            3 => {
                let mut base_vec: Vec<i32> = vec![pv1; vec_len_mod];
                base_vec.push(pv2);
                let mut sv: SmallVec<[i32; 32]> = SmallVec::from_vec(base_vec);
                complex_ops(&mut sv, first, helper_slice, idx_insert, idx_remove, idx_swap_remove, trunc_to, reserve_add, grow_to, drain_start, drain_end, op_count, repeat_val);
                let _ = sv.into_boxed_slice();
            }
            4 => {
                let mut sv: SmallVec<[i32; 32]> = SmallVec::from_slice(helper_slice);
                complex_ops(&mut sv, first, helper_slice, idx_insert, idx_remove, idx_swap_remove, trunc_to, reserve_add, grow_to, drain_start, drain_end, op_count, repeat_val);
                let _ = sv.into_vec();
            }
            _ => {
                let base = [pv1; 32];
                let len = _to_usize(first, 83);
                let mut sv = SmallVec::<[i32; 32]>::from_buf_and_len(base, len);
                complex_ops(&mut sv, first, helper_slice, idx_insert, idx_remove, idx_swap_remove, trunc_to, reserve_add, grow_to, drain_start, drain_end, op_count, repeat_val);
                std::mem::drop(sv);
            }
        }

        let tsv: SmallVec<[i32; 32]> = helper_slice.to_smallvec();
        let mut sv_a: SmallVec<[i32; 32]> = SmallVec::from(tsv.as_slice());
        sv_a.insert_from_slice(idx_insert, helper_slice);
        let _ = sv_a.pop();
        let mut dr = sv_a.drain(drain_start..drain_end);
        let _ = dr.next();
        let _ = dr.next_back();
        std::mem::drop(dr);
        let sref = sv_a.as_slice();
        if !sref.is_empty() {
            let r = &sref[0];
            println!("{}", *r);
        }
        std::mem::drop(sv_a);
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