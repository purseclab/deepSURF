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
        if data.len() < 256 { return; }
        set_global_data(data);
        let gd = get_global_data();
        let fh = gd.first_half;
        let sh = gd.second_half;

        let mut arr_a = [0u8; 16];
        let mut arr_b = [0u8; 16];
        for i in 0..16 {
            arr_a[i] = fh[i];
            arr_b[i] = sh[i];
        }

        let vlen1 = (_to_u8(fh, 32) % 65) as usize;
        let vlen2 = (_to_u8(fh, 33) % 65) as usize;
        let mut vec1 = Vec::with_capacity(vlen1);
        let mut vec2 = Vec::with_capacity(vlen2);
        for i in 0..vlen1 {
            vec1.push(fh[64 + (i % (fh.len() - 64))]);
        }
        for i in 0..vlen2 {
            vec2.push(sh[64 + (i % (sh.len() - 64))]);
        }

        let smax_a = fh.len();
        let smax_b = sh.len();
        let slen_a = if smax_a == 0 { 0 } else { (_to_u8(fh, 34) as usize) % smax_a };
        let slen_b = if smax_b == 0 { 0 } else { (_to_u8(fh, 35) as usize) % smax_b };
        let slice_a = &fh[0..slen_a];
        let slice_b = &sh[0..slen_b];

        let cap = _to_usize(fh, 40);
        let len_in_buf = _to_usize(fh, 48);

        let choice = _to_u8(fh, 20) % 9;
        let mut sv: SmallVec<[u8; 16]> = match choice {
            0 => SmallVec::new(),
            1 => SmallVec::with_capacity(cap),
            2 => SmallVec::from_vec(vec1.clone()),
            3 => SmallVec::from_slice(slice_a),
            4 => SmallVec::from_buf(arr_a),
            5 => SmallVec::from_buf_and_len(arr_b, len_in_buf),
            6 => SmallVec::from_iter(vec2.clone().into_iter()),
            7 => SmallVec::from(slice_b),
            _ => slice_a.to_smallvec(),
        };

        let l0 = (&sv).len();
        println!("{}", l0);
        let empty = (&sv).is_empty();
        println!("{}", empty);
        let cap0 = (&sv).capacity();
        println!("{}", cap0);
        let s_ref = sv.as_slice();
        println!("{:?}", s_ref);
        let d_ref: &[u8] = sv.deref();
        println!("{:?}", d_ref);

        let mut steps = (_to_u8(fh, 58) % 23) as usize;
        if steps == 0 { steps = 1; }
        for i in 0..steps {
            let op = _to_u8(sh, i % sh.len()) % 23;
            match op {
                0 => {
                    let val = _to_u8(sh, (i * 3) % sh.len());
                    sv.push(val);
                }
                1 => {
                    let _ = sv.pop();
                }
                2 => {
                    let idx = _to_usize(sh, 8);
                    let val = _to_u8(fh, (i * 5) % fh.len());
                    sv.insert(idx, val);
                }
                3 => {
                    let idx = _to_usize(sh, 16);
                    let _ = sv.remove(idx);
                }
                4 => {
                    let idx = _to_usize(sh, 24);
                    let _ = sv.swap_remove(idx);
                }
                5 => {
                    let new_len = _to_usize(sh, 32);
                    sv.truncate(new_len);
                }
                6 => {
                    let add = _to_usize(sh, 40);
                    sv.reserve(add);
                }
                7 => {
                    let add = _to_usize(sh, 48);
                    let _ = sv.try_reserve(add);
                }
                8 => {
                    let add = _to_usize(sh, 56);
                    sv.reserve_exact(add);
                }
                9 => {
                    let add = _to_usize(sh, 64);
                    let _ = sv.try_reserve_exact(add);
                }
                10 => {
                    let g = _to_usize(sh, 72);
                    sv.grow(g);
                }
                11 => {
                    let g = _to_usize(sh, 80);
                    let _ = sv.try_grow(g);
                }
                12 => {
                    let len = _to_usize(fh, 88);
                    let val = _to_u8(sh, (i * 7) % sh.len());
                    sv.resize(len, val);
                }
                13 => {
                    let len = _to_usize(fh, 96);
                    let mut idx_local = 0usize;
                    sv.resize_with(len, || {
                        idx_local = idx_local.wrapping_add(1);
                        _to_u8(fh, (idx_local % (fh.len() - 1)).max(1))
                    });
                }
                14 => {
                    let take = if sh.len() == 0 { 0 } else { (_to_u8(fh, 100) as usize) % sh.len() };
                    let sl = &sh[0..take];
                    sv.extend_from_slice(sl);
                }
                15 => {
                    let idx = _to_usize(fh, 104);
                    let take = if fh.len() == 0 { 0 } else { (_to_u8(sh, 101) as usize) % fh.len() };
                    let sl = &fh[0..take];
                    sv.insert_from_slice(idx, sl);
                }
                16 => {
                    sv.dedup();
                }
                17 => {
                    let mut t = 0usize;
                    sv.dedup_by(|a, b| {
                        t = t.wrapping_add(1);
                        *a = a.wrapping_add(_to_u8(fh, (t % (fh.len() - 1)).max(1)));
                        *b = b.wrapping_sub(_to_u8(sh, (t % (sh.len() - 1)).max(1)));
                        _to_bool(fh, (t % (fh.len() - 1)).max(1))
                    });
                }
                18 => {
                    let mut t = 0usize;
                    sv.dedup_by_key(|x| {
                        t = t.wrapping_add(1);
                        let k = _to_u8(sh, (t % (sh.len() - 1)).max(1));
                        *x = (*x) ^ k;
                        k
                    });
                }
                19 => {
                    let mut t = 0usize;
                    sv.retain(|e| {
                        t = t.wrapping_add(1);
                        let k = _to_u8(fh, (t % (fh.len() - 1)).max(1));
                        *e = e.wrapping_add(k);
                        (k % 2) == 0
                    });
                }
                20 => {
                    let end = _to_usize(fh, 112);
                    let mut dr = sv.drain(0..end);
                    let n1 = dr.next();
                    let n2 = dr.next_back();
                    println!("{:?}{:?}", n1, n2);
                }
                21 => {
                    let mut other = SmallVec::<[u8; 16]>::from_vec(vec2.clone());
                    sv.append(&mut other);
                }
                _ => {
                    let r1 = (&sv).as_slice();
                    println!("{:?}", r1);
                    let r2 = (&mut sv).as_mut_slice();
                    if !r2.is_empty() {
                        r2[0] = r2[0].wrapping_add(1);
                    }
                    println!("{:?}", r2);
                }
            }
            let l = (&sv).len();
            println!("{}", l);
        }

        let b1: &[u8] = (&sv).borrow();
        println!("{:?}", b1);
        let b2: &mut [u8] = (&mut sv).borrow_mut();
        if !b2.is_empty() {
            b2[b2.len() - 1] = b2[b2.len() - 1].wrapping_sub(1);
        }
        println!("{:?}", b2);

        if sv.len() > 0 {
            let last = &sv[sv.len() - 1];
            println!("{}", *last);
            let m0 = &mut sv[0];
            *m0 = m0.wrapping_add(1);
            println!("{}", *m0);
        }

        let sv2 = SmallVec::<[u8; 16]>::from_slice((&sv).as_slice());
        let ord = SmallVec::cmp(&sv, &sv2);
        println!("{:?}", ord);
        let pord = SmallVec::partial_cmp(&sv, &sv2);
        println!("{:?}", pord);
        let eq = SmallVec::eq(&sv, &sv2);
        println!("{}", eq);

        let l_final = (&sv).len();
        println!("{}", l_final);
        let c_final = (&sv).capacity();
        println!("{}", c_final);
        let _ = (&sv).is_empty();
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