#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 900 { return; }
        set_global_data(data);
        let gd = get_global_data();
        let fh = gd.first_half;
        let sh = gd.second_half;

        let len_a = (_to_u8(fh, 1) % 65) as usize;
        let len_b = (_to_u8(sh, 1) % 65) as usize;

        let mut v_a = std::vec::Vec::with_capacity(65);
        for i in 0..len_a {
            let val = _to_u16(fh, 2 * i);
            v_a.push(val);
        }
        let mut v_b = std::vec::Vec::with_capacity(65);
        for i in 0..len_b {
            let val = _to_u16(sh, 2 * i);
            v_b.push(val);
        }

        let mut arr1: [u16; 32] = [0; 32];
        for i in 0..32 {
            arr1[i] = _to_u16(fh, 200 + 2 * i);
        }
        let mut arr2: [u16; 32] = [0; 32];
        for i in 0..32 {
            arr2[i] = _to_u16(sh, 240 + 2 * i);
        }

        let mut sv_new: SmallVec<[u16; 32]> = SmallVec::new();
        sv_new.extend(v_a.iter().cloned());

        let mut sv_cap: SmallVec<[u16; 32]> = SmallVec::with_capacity(_to_usize(fh, 312));
        sv_cap.extend(v_b.iter().cloned());

        let sv_from_slice: SmallVec<[u16; 32]> = SmallVec::from_slice(&v_b[..]);

        let sv_from_vec: SmallVec<[u16; 32]> = SmallVec::from_vec(v_a.clone());

        let sv_from_buf: SmallVec<[u16; 32]> = SmallVec::from_buf(arr1);

        let sv_from_buf_len: SmallVec<[u16; 32]> = SmallVec::from_buf_and_len(arr2, _to_usize(sh, 320));

        let sv_to_smallvec: SmallVec<[u16; 32]> = (&v_b[..]).to_smallvec();

        let sv_from_iter: SmallVec<[u16; 32]> = SmallVec::from_iter(v_a.clone().into_iter());

        let mut pool: std::vec::Vec<SmallVec<[u16; 32]>> = std::vec::Vec::with_capacity(8);
        pool.push(sv_new);
        pool.push(sv_cap);
        pool.push(sv_from_slice);
        pool.push(sv_from_vec);
        pool.push(sv_from_buf);
        pool.push(sv_from_buf_len);
        pool.push(sv_to_smallvec);
        pool.push(sv_from_iter);

        let idx_a = (_to_u8(fh, 10) as usize) % pool.len();
        let mut sv1 = pool.remove(idx_a);
        let idx_b = (_to_u8(fh, 11) as usize) % pool.len();
        let mut sv2 = pool.remove(idx_b);

        {
            let s1 = sv1.as_slice();
            if let Some(r) = s1.get(0) {
                println!("{:?}", *r);
            }
        }
        {
            let m2 = sv2.as_mut_slice();
            if let Some(rm) = m2.get_mut(0) {
                *rm = _to_u16(sh, 210);
                println!("{:?}", *rm);
            }
        }

        let op_count = (_to_u8(fh, 12) % 12) as usize;
        for i in 0..op_count {
            let which = _to_u8(sh, 13 + i);
            match which % 10 {
                0 => {
                    let val = _to_u16(fh, 20 + 2 * i);
                    sv1.push(val);
                }
                1 => {
                    let idx = _to_usize(fh, 328);
                    let val = _to_u16(sh, 40 + 2 * i);
                    sv2.insert(idx, val);
                }
                2 => {
                    let n = _to_usize(sh, 344);
                    sv1.truncate(n);
                }
                3 => {
                    let add = _to_usize(fh, 360);
                    sv2.reserve(add);
                }
                4 => {
                    let add = _to_usize(sh, 376);
                    sv1.reserve_exact(add);
                }
                5 => {
                    let add = _to_usize(fh, 392);
                    let _ = _unwrap_result(sv2.try_reserve(add));
                }
                6 => {
                    let add = _to_usize(sh, 408);
                    let _ = _unwrap_result(sv1.try_reserve_exact(add));
                }
                7 => {
                    let idx = _to_usize(fh, 424);
                    let _ = sv2.swap_remove(idx);
                }
                8 => {
                    let idx = _to_usize(sh, 312);
                    let _ = sv1.remove(idx);
                }
                _ => {
                    let idx = _to_usize(fh, 320);
                    sv2.insert_from_slice(idx, &v_a[..]);
                }
            }
        }

        let _ = sv1.partial_cmp(&sv2);
        let ord = sv1.cmp(&sv2);
        match ord {
            core::cmp::Ordering::Less => {
                let mut tmp = SmallVec::<[u16; 32]>::from_vec(v_b.clone());
                sv1.append(&mut tmp);
            }
            core::cmp::Ordering::Greater => {
                let s = sv2.as_slice();
                sv1.extend_from_slice(s);
            }
            core::cmp::Ordering::Equal => {
                sv1.dedup();
                let mut f_equal = |a: &mut u16, b: &mut u16| -> bool {
                    if _to_u8(fh, 100) % 2 == 0 {
                        panic!("INTENTIONAL PANIC!");
                    }
                    *a == *b
                };
                sv2.dedup_by(&mut f_equal);
                let mut keyf = |x: &mut u16| -> u16 {
                    if _to_u8(sh, 101) % 3 == 0 {
                        panic!("INTENTIONAL PANIC!");
                    }
                    *x
                };
                sv2.dedup_by_key(&mut keyf);
                let mut rf = |x: &mut u16| -> bool {
                    if _to_u8(fh, 102) % 5 == 0 {
                        panic!("INTENTIONAL PANIC!");
                    }
                    *x % 2 == 0
                };
                sv1.retain(&mut rf);
            }
        }

        let l1 = sv1.len();
        let c1 = sv1.capacity();
        let e1 = sv1.is_empty();
        println!("{:?}{:?}{:?}", l1, c1, e1);

        {
            let r_end = _to_usize(fh, 336);
            let mut dr = sv2.drain(0..r_end);
            let _ = dr.next();
            let _ = dr.next_back();
        }

        let ts1: SmallVec<[u16; 32]> = sv1.as_slice().to_smallvec();
        let _ = ts1.cmp(&sv1);

        let cl1 = sv1.clone();
        let _ = cl1.cmp(&sv1);

        let beq = sv1.eq(&sv2);
        println!("{:?}", beq);

        {
            let _dref: &[u16] = sv1.deref();
            if let Some(r) = _dref.get(1) {
                println!("{:?}", *r);
            }
        }
        {
            let _dmref: &mut [u16] = sv2.deref_mut();
            if let Some(rm) = _dmref.get_mut(1) {
                *rm = _to_u16(sh, 212);
                println!("{:?}", *rm);
            }
        }

        let idx_ref = _to_usize(fh, 340);
        let _ = &sv1[idx_ref];

        let mut gen = || -> u16 {
            if _to_u8(sh, 104) % 2 == 0 {
                panic!("INTENTIONAL PANIC!");
            }
            _to_u16(sh, 220)
        };
        sv2.resize_with(_to_usize(sh, 352), &mut gen);

        let s_after1 = sv1.as_slice();
        if let Some(r) = s_after1.get(0) {
            println!("{:?}", *r);
        }
        let s_after2 = sv2.as_slice();
        if let Some(r) = s_after2.get(0) {
            println!("{:?}", *r);
        }

        let _ = sv1.cmp(&sv2);
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