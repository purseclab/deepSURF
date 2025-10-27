#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};
use std::borrow::{Borrow, BorrowMut};

fn maybe_panic() {
    let gd = get_global_data();
    let fh = gd.first_half;
    let b = _to_u8(fh, 0);
    if b % 2 == 0 {
        panic!("INTENTIONAL PANIC!");
    }
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 256 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let FH = global_data.first_half;
        let SH = global_data.second_half;

        let mut arr1: [i32; 16] = [0; 16];
        let mut i = 0usize;
        while i < 16 {
            arr1[i] = _to_i32(FH, i * 4);
            i += 1;
        }
        let mut arr2: [u8; 32] = [0; 32];
        let mut j = 0usize;
        while j < 32 {
            arr2[j] = _to_u8(SH, j);
            j += 1;
        }

        let s2_len = (_to_u8(SH, 32) % 31) as usize;
        let s2 = &arr2[..s2_len];
        let ts: SmallVec<[u8; 32]> = s2.to_smallvec();
        println!("{:?}", ts.as_slice());

        let sv_u8 = SmallVec::from_buf(arr2);
        println!("{:?}", sv_u8.as_slice());

        let tag = _to_u8(SH, 40);
        let mut sv0: SmallVec<[i32; 16]> = match tag % 4 {
            0 => SmallVec::<[i32; 16]>::new(),
            1 => SmallVec::<[i32; 16]>::with_capacity(_to_usize(SH, 48)),
            2 => {
                let n = (_to_u8(SH, 56) % 65) as usize;
                let mut v = Vec::with_capacity(n);
                let mut k = 0usize;
                while k < n {
                    v.push(_to_i32(SH, 60 + (k % 8) * 4));
                    k += 1;
                }
                SmallVec::<[i32; 16]>::from_vec(v)
            }
            _ => {
                let k = _to_usize(SH, 64);
                let slice = &arr1[..k];
                SmallVec::<[i32; 16]>::from_slice(slice)
            }
        };

        let n2 = (_to_u8(FH, 112) % 65) as usize;
        let mut tmpv: Vec<i32> = Vec::with_capacity(n2);
        let mut t2 = 0usize;
        while t2 < n2 {
            let src = (t2 * 4) % (FH.len().saturating_sub(4));
            tmpv.push(_to_i32(FH, src));
            t2 += 1;
        }
        let sv_iter = SmallVec::<[i32; 16]>::from_iter(tmpv.clone());
        println!("{:?}", sv_iter.as_slice());

        let len1 = _to_usize(FH, 80);
        let mut sv_buf = SmallVec::from_buf_and_len(arr1, len1);

        let cap = sv0.capacity();
        let _ = cap;
        let sv0_clone_for_extend = sv0.clone();
        sv0.extend_from_slice(sv0_clone_for_extend.as_slice());

        let ops = (_to_u8(FH, 96) % 20) as usize;
        let mut pos_fh = 100usize;
        let mut pos_sh = 80usize;
        let mut iter = 0usize;
        while iter < ops {
            match _to_u8(FH, pos_fh % (FH.len().saturating_sub(1))) % 10 {
                0 => {
                    let val = _to_i32(SH, pos_sh % (SH.len().saturating_sub(4)));
                    sv_buf.push(val);
                }
                1 => {
                    let idx = _to_usize(FH, pos_fh % (FH.len().saturating_sub(8)));
                    let val = _to_i32(SH, pos_sh % (SH.len().saturating_sub(4)));
                    sv_buf.insert(idx, val);
                }
                2 => {
                    let idx = _to_usize(FH, pos_fh % (FH.len().saturating_sub(8)));
                    let _ = sv_buf.remove(idx);
                }
                3 => {
                    let idx = _to_usize(FH, pos_fh % (FH.len().saturating_sub(8)));
                    let _ = sv_buf.swap_remove(idx);
                }
                4 => {
                    let resize_len = _to_usize(SH, pos_sh % (SH.len().saturating_sub(8)));
                    let fill = _to_i32(FH, pos_fh % (FH.len().saturating_sub(4)));
                    sv_buf.resize(resize_len, fill);
                }
                5 => {
                    let idx = _to_usize(SH, pos_sh % (SH.len().saturating_sub(8)));
                    let slice_len = (_to_u8(FH, pos_fh % (FH.len().saturating_sub(1))) % 65) as usize;
                    let mut tmp: Vec<i32> = Vec::with_capacity(slice_len);
                    let mut t = 0usize;
                    while t < slice_len {
                        let off = (pos_sh + t * 4) % (SH.len().saturating_sub(4));
                        tmp.push(_to_i32(SH, off));
                        t += 1;
                    }
                    sv_buf.insert_from_slice(idx, &tmp[..]);
                }
                6 => {
                    let add = _to_usize(FH, pos_fh % (FH.len().saturating_sub(8)));
                    sv_buf.reserve(add);
                    let _ = sv_buf.try_reserve(add);
                    let _ = sv_buf.try_reserve_exact(add);
                    sv_buf.reserve_exact(add);
                    sv_buf.grow(add);
                    let _ = sv_buf.try_grow(add);
                }
                7 => {
                    let trunc = _to_usize(SH, pos_sh % (SH.len().saturating_sub(8)));
                    sv_buf.truncate(trunc);
                }
                8 => {
                    let drain_end = _to_usize(FH, pos_fh % (FH.len().saturating_sub(8)));
                    let mut dr = sv_buf.drain(0..drain_end);
                    let _ = dr.next();
                    let _ = dr.next_back();
                }
                _ => {
                    sv_buf.retain(|e| {
                        if _to_u8(FH, 0) % 3 == 0 { maybe_panic(); }
                        *e = *e;
                        _to_bool(SH, 0)
                    });
                    sv_buf.dedup();
                    sv_buf.dedup_by(|a, b| {
                        if _to_bool(FH, 1) { maybe_panic(); }
                        *a == *b
                    });
                    sv_buf.dedup_by_key(|e| {
                        if _to_bool(FH, 2) { maybe_panic(); }
                        *e
                    });
                }
            }
            let s = sv_buf.as_slice();
            println!("{:?}", s);
            let s_mut = sv_buf.as_mut_slice();
            if !s_mut.is_empty() {
                s_mut[0] = s_mut[0];
            }
            pos_fh += 9;
            pos_sh += 7;
            iter += 1;
        }

        let _p = sv_buf.as_ptr();
        let _mp = sv_buf.as_mut_ptr();

        let r1 = sv_buf.as_slice();
        println!("{:?}", r1);
        let brr: &[i32] = sv_buf.borrow();
        println!("{:?}", brr);
        {
            let brrm: &mut [i32] = sv_buf.borrow_mut();
            if !brrm.is_empty() {
                brrm[0] = brrm[0];
            }
        }
        let aref = sv_buf.as_ref();
        println!("{:?}", aref);
        let _ = sv_buf.is_empty();
        let l = sv_buf.len();
        let _ = l;
        let c = sv_buf.capacity();
        let _ = c;
        if sv_buf.len() > 0 {
            let first_ref = &sv_buf[0usize];
            println!("{:?}", first_ref);
            let x_mut = &mut sv_buf[0usize];
            *x_mut = *x_mut;
        }
        let dr = Deref::deref(&sv_buf);
        println!("{:?}", dr);
        let drm = DerefMut::deref_mut(&mut sv_buf);
        if !drm.is_empty() {
            drm[0] = drm[0];
        }

        let sv_clone = sv_buf.clone();
        let _ = sv_buf.cmp(&sv_clone);
        let _ = sv_buf.partial_cmp(&sv_clone);
        let _ = sv_buf.eq(&sv_clone);

        let mut it = sv_clone.clone().into_iter();
        let _ = it.clone();
        let _ = it.next();
        let _ = it.next_back();
        let rem = it.as_slice();
        println!("{:?}", rem);
        let rem_mut = it.as_mut_slice();
        if !rem_mut.is_empty() {
            rem_mut[0] = rem_mut[0];
        }

        let mut other = SmallVec::<[i32; 16]>::from_buf_and_len([0; 16], _to_usize(FH, 8));
        sv_buf.append(&mut other);
        sv_buf.shrink_to_fit();
        sv_buf.clear();
        let _ = sv_buf.into_vec();
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