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
        if data.len() < 288 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let g1 = global_data.first_half;
        let g2 = global_data.second_half;

        let mut arr16: [u8; 16] = [0; 16];
        for i in 0..16 {
            arr16[i] = _to_u8(g1, i);
        }

        let len1 = _to_usize(g1, 20) % 65;
        let cap1 = _to_usize(g1, 28);
        let reserve_amt = _to_usize(g1, 36);
        let idx1 = _to_usize(g1, 44);
        let idx2 = _to_usize(g1, 52);
        let trunc_to = _to_usize(g1, 60);
        let grow_to = _to_usize(g1, 68);
        let op_count = (_to_u8(g1, 76) % 12) as usize;

        let mut sv_a = SmallVec::<[u8; 16]>::from_buf_and_len(arr16, len1);
        let mut sv_b = SmallVec::<[u8; 16]>::with_capacity(cap1);
        let push_n = if g2.len() > 0 { (_to_u8(g2, 0) as usize) % 65 } else { 0 };
        for j in 0..push_n.min(g2.len()) {
            sv_b.push(g2[j]);
        }

        let start = if g2.len() > 32 { 16 } else { 0 };
        let slice_len = if g2.len() > start { (_to_u8(g2, 1) as usize % 65).min(g2.len() - start) } else { 0 };
        let sv_c = SmallVec::<[u8; 16]>::from_slice(&g2[start..start + slice_len]);

        let mut base_vec: Vec<u8> = Vec::new();
        let vlen = (_to_u8(g1, 77) as usize % 65).min(g2.len());
        for i in 0..vlen {
            base_vec.push(g2[i]);
        }
        let mut sv_d = SmallVec::<[u8; 16]>::from_vec(base_vec);

        let elem_i32 = _to_i32(g1, 80);
        let count_i32 = (_to_usize(g1, 88) % 65);
        let mut svi32 = SmallVec::<[i32; 16]>::from_elem(elem_i32, count_i32);

        let _ = sv_a.capacity();
        sv_a.reserve(reserve_amt);
        let _ = sv_a.try_grow(grow_to);
        sv_a.push(_to_u8(g2, 2));
        let _ = sv_a.pop();
        let _ = sv_a.is_empty();
        let _ = sv_a.len();

        let sref = sv_a.as_slice();
        println!("{:?}", sref);
        if sv_a.len() > 0 {
            let r = &sv_a[0];
            println!("{}", *r);
        }
        let sref2 = sv_a.deref();
        println!("{:?}", sref2);
        let srefm = sv_a.as_mut_slice();
        if srefm.len() > 0 {
            srefm[0] = srefm[0].wrapping_add(1);
        }
        println!("{:?}", srefm);

        let mut toggle = _to_u8(g1, 96);
        sv_a.retain(|x| {
            toggle = toggle.wrapping_add(*x);
            if toggle % 7 == 0 { panic!("INTENTIONAL PANIC!"); }
            *x % 2 == 0
        });
        sv_a.dedup_by(|a, b| {
            if (*a).wrapping_add(*b) == _to_u8(g2, 3) { panic!("INTENTIONAL PANIC!"); }
            a == b
        });
        sv_a.dedup_by_key(|k| {
            if *k == _to_u8(g2, 4) { panic!("INTENTIONAL PANIC!"); }
            *k
        });
        sv_a.resize_with(_to_usize(g1, 104) % 65, || {
            let v = _to_u8(g2, 5);
            if v % 9 == 0 { panic!("INTENTIONAL PANIC!"); }
            v
        });

        sv_a.extend_from_slice(sv_c.as_slice());
        sv_a.insert(idx1, _to_u8(g2, 6));
        let _ = sv_a.remove(idx2);
        sv_a.truncate(trunc_to);
        let _ = sv_a.swap_remove(_to_usize(g1, 112));

        let _ = sv_b.eq(&sv_d);
        let _ = sv_b.partial_cmp(&sv_d);
        let _ = sv_b.cmp(&sv_d);

        let mut dr = sv_b.drain(0.._to_usize(g1, 120));
        let _ = dr.next();
        let _ = dr.next_back();
        drop(dr);

        let mut it = sv_d.clone().into_iter();
        let _ = it.next();
        let _ = it.next_back();
        let slice_from_it = it.as_slice();
        println!("{:?}", slice_from_it);
        let slice_mut_from_it = it.as_mut_slice();
        if !slice_mut_from_it.is_empty() {
            slice_mut_from_it[0] = slice_mut_from_it[0].wrapping_add(1);
        }
        println!("{:?}", slice_mut_from_it);

        let br: &[u8] = sv_a.borrow();
        println!("{:?}", br);
        let brm: &mut [u8] = sv_a.borrow_mut();
        if !brm.is_empty() {
            brm[0] = brm[0].wrapping_add(1);
        }
        println!("{:?}", brm);
        let aref = sv_a.as_ref();
        println!("{:?}", aref);
        let amut = sv_a.as_mut();
        if !amut.is_empty() {
            amut[0] = amut[0].wrapping_add(1);
        }
        println!("{:?}", amut);

        for i in 0..op_count {
            let op = if g2.len() > 10 + i { _to_u8(g2, 10 + i) % 8 } else { 0 };
            match op {
                0 => {
                    let tmp = sv_b.clone().into_vec();
                    println!("{}", tmp.len());
                }
                1 => {
                    if g2.len() > 12 + i {
                        sv_b.push(_to_u8(g2, 12 + i));
                    }
                }
                2 => {
                    sv_b.try_reserve(_to_usize(g1, 128));
                }
                3 => {
                    sv_b.reserve_exact(_to_usize(g1, 136));
                }
                4 => {
                    let _ = sv_b.capacity();
                    let _ = sv_b.len();
                }
                5 => {
                    if sv_b.len() > 0 {
                        let v = &sv_b[0];
                        println!("{}", *v);
                    }
                }
                6 => {
                    sv_b.clear();
                }
                _ => {
                    let tmp2 = sv_d.clone().into_vec();
                    println!("{}", tmp2.len());
                }
            }
        }

        let v1 = sv_a.into_vec();
        println!("{}", v1.len());
        let mut sv_after = SmallVec::<[u8; 16]>::from_vec(v1);
        sv_after.shrink_to_fit();
        let _b = sv_after.into_boxed_slice();

        let v2 = sv_d.into_vec();
        let mut sv2_again = SmallVec::<[u8; 16]>::from_vec(v2);
        if g2.len() > 7 {
            sv2_again.push(_to_u8(g2, 7));
        }
        let _ = sv2_again.into_inner();

        let vi32 = svi32.into_vec();
        println!("{}", vi32.len());
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