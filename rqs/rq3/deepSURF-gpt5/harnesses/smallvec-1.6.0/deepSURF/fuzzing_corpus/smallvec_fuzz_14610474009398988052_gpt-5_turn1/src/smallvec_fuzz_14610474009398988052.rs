#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 160 { return; }
        set_global_data(data);
        let gd = get_global_data();
        let first = gd.first_half;
        let second = gd.second_half;

        let mut arr_u8_16: [u8; 16] = [0u8; 16];
        for i in 0..16 {
            arr_u8_16[i] = second[i];
        }
        let mut buf32: [u8; 32] = [0u8; 32];
        for i in 0..32 {
            buf32[i] = first[i];
        }
        let mut arr_i16_12: [i16; 12] = [0i16; 12];
        for i in 0..12 {
            let idx = 32 + i * 2;
            let val = _to_i16(first, idx % (first.len() - 2));
            arr_i16_12[i] = val;
        }

        let cap1 = _to_usize(first, 8 % (first.len() - 8));
        let mut sv1: SmallVec<[u8; 16]> = SmallVec::with_capacity(cap1);

        let sel = _to_u8(second, 0);
        let mut sv2: SmallVec<[i16; 12]> = match sel % 5 {
            0 => SmallVec::new(),
            1 => {
                let val = _to_i16(first, 2 % (first.len() - 2));
                let n = _to_usize(second, 10 % (second.len() - 8));
                SmallVec::from_elem(val, n)
            }
            2 => {
                let lenv = (second[1] as usize) % 65;
                let mut v = Vec::with_capacity(lenv);
                for i in 0..lenv {
                    let base = (2 * i) % (first.len() - 2);
                    v.push(_to_i16(first, base));
                }
                SmallVec::from_vec(v)
            }
            3 => {
                let lenv = (second[2] as usize) % 65;
                let mut v = Vec::with_capacity(lenv);
                for i in 0..lenv {
                    let base = (2 * i + 1) % (second.len() - 2);
                    v.push(_to_i16(second, base));
                }
                SmallVec::from_slice(&v)
            }
            _ => {
                let len = _to_usize(second, 12 % (second.len() - 8));
                SmallVec::from_buf_and_len(arr_i16_12, len)
            }
        };

        let lenv_u8 = (first[3] as usize) % 65;
        let mut tmpv = Vec::with_capacity(lenv_u8);
        for i in 0..lenv_u8 {
            tmpv.push(second[i % second.len()]);
        }
        let mut sv4: SmallVec<[u8; 32]> = SmallVec::from(tmpv);

        let slice_len = (first[4] as usize) % 65;
        let mut tmpv2 = Vec::with_capacity(slice_len);
        for i in 0..slice_len {
            tmpv2.push(first[i % first.len()]);
        }
        let mut sv5: SmallVec<[u8; 32]> = SmallVec::from(&tmpv2[..]);

        let ops = 1 + (first[5] as usize % 20);
        for i in 0..ops {
            let sel2 = second[(6 + i) % second.len()];
            match sel2 % 16 {
                0 => {
                    sv1.push(second[(i) % second.len()]);
                }
                1 => {
                    let idx_u = _to_usize(first, (7 + i) % (first.len() - 8));
                    sv1.insert(idx_u, second[(8 + i) % second.len()]);
                }
                2 => {
                    let idx_u = _to_usize(second, (9 + i) % (second.len() - 8));
                    let _ = sv1.remove(idx_u);
                }
                3 => {
                    let n = _to_usize(first, (10 + i) % (first.len() - 8));
                    sv1.truncate(n);
                }
                4 => {
                    let add = _to_usize(second, (11 + i) % (second.len() - 8));
                    sv1.reserve(add);
                }
                5 => {
                    let add = _to_usize(second, (12 + i) % (second.len() - 8));
                    let _ = sv1.try_reserve(add);
                    let _ = sv1.try_reserve_exact(add);
                }
                6 => {
                    let s = sv1.as_slice();
                    let _ = s.len();
                    if let Some(r) = s.get(0) {
                        println!("{}", *r);
                    }
                }
                7 => {
                    let m = sv1.as_mut_slice();
                    if !m.is_empty() {
                        let a = m[0];
                        m[0] = a.wrapping_add(1);
                    }
                }
                8 => {
                    let upto = _to_usize(first, (13 + i) % (first.len() - 8));
                    let mut dr = sv1.drain(0..upto);
                    if let Some(x) = dr.next() {
                        println!("{}", x);
                    }
                    if let Some(y) = dr.next_back() {
                        println!("{}", y);
                    }
                }
                9 => {
                    sv1.extend_from_slice(&arr_u8_16[..]);
                }
                10 => {
                    let idx = _to_usize(first, (14 + i) % (first.len() - 8));
                    let _ = sv1.swap_remove(idx);
                }
                11 => {
                    sv1.clear();
                }
                12 => {
                    let new_len = _to_usize(second, (15 + i) % (second.len() - 8));
                    let value = _to_u8(first, (16 + i) % (first.len() - 1));
                    sv1.resize(new_len, value);
                }
                13 => {
                    let new_len = _to_usize(second, (17 + i) % (second.len() - 8));
                    let mut ctr = 0usize;
                    sv1.resize_with(new_len, || {
                        let idxb = ctr % second.len();
                        ctr = ctr.wrapping_add(1);
                        second[idxb]
                    });
                }
                14 => {
                    let e = _to_u8(first, (18 + i) % (first.len() - 1));
                    let n = (second[(19 + i) % second.len()] as usize) % 65;
                    let mut other = SmallVec::<[u8; 16]>::from_elem(e, n);
                    sv1.append(&mut other);
                }
                _ => {
                    let _ = sv1.len();
                    let _ = sv1.capacity();
                    let _ = sv1.is_empty();
                }
            }
        }

        sv2.retain(|v| {
            let b = _to_u8(second, 20 % (second.len() - 1));
            *v = v.wrapping_add(b as i16);
            b % 2 == 0
        });
        sv2.dedup();
        sv2.dedup_by(|a, b| {
            let flag = _to_bool(first, 21 % (first.len() - 1));
            if flag {
                let tmp = *a;
                *a = *b;
                *b = tmp;
            }
            flag
        });
        sv2.dedup_by_key(|x| {
            let k = _to_u8(second, 22 % (second.len() - 1));
            *x = x.wrapping_add(k as i16);
            k
        });

        let idx_index = _to_usize(first, 23 % (first.len() - 8));
        let r_i16 = &sv2[idx_index];
        println!("{}", *r_i16);
        let pm: &mut i16 = &mut sv2[idx_index];
        *pm = pm.wrapping_add(1);

        let vec_from_sv1 = sv1.clone().into_vec();
        let back: SmallVec<[u8; 32]> = SmallVec::from_vec(vec_from_sv1);
        let _ = back.partial_cmp(&sv4);
        let _ = back.cmp(&sv5);
        let _ = back == sv5;

        let mut it = back.into_iter();
        let _ = it.as_slice();
        let _ = it.as_mut_slice();
        let _ = it.next();
        let _ = it.next_back();

        let cap2 = _to_usize(second, 24 % (second.len() - 8));
        let mut sv_cap2: SmallVec<[u8; 32]> = SmallVec::with_capacity(cap2);
        sv_cap2.push(_to_u8(first, 25 % (first.len() - 1)));

        let br = sv1.as_ref();
        if let Some(r) = br.get(0) {
            println!("{}", *r);
        }
        let bm = sv1.as_mut_slice();
        if !bm.is_empty() {
            bm[0] = bm[0].wrapping_add(1);
        }

        let add2 = _to_usize(first, 26 % (first.len() - 8));
        sv1.reserve_exact(add2);
        sv1.shrink_to_fit();
        let ng = _to_usize(second, 27 % (second.len() - 8));
        sv1.grow(ng);
        let _ = sv1.try_grow(_to_usize(first, 28 % (first.len() - 8)));

        let slice_ins_len = (29 % (first.len() - 1)) as usize;
        let mut slice_ins_src: [u8; 16] = [0u8; 16];
        for i in 0..16 {
            slice_ins_src[i] = first[(i + slice_ins_len) % first.len()];
        }
        sv1.insert_from_slice(_to_usize(second, 30 % (second.len() - 8)), &slice_ins_src[..]);

        let range_end = _to_usize(first, 31 % (first.len() - 8));
        let mut drain_handle = sv5.drain(0..range_end);
        let _ = drain_handle.next();
        let _ = drain_handle.next_back();
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