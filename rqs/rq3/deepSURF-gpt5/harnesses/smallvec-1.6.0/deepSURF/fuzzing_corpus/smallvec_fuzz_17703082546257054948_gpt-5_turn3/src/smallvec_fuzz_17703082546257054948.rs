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
        if data.len() < 160 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let FIRST = global_data.first_half;
        let SECOND = global_data.second_half;

        let choice = _to_u8(FIRST, 0) % 8;

        let arr16 = [
            _to_u8(FIRST, 10), _to_u8(FIRST, 11), _to_u8(FIRST, 12), _to_u8(FIRST, 13),
            _to_u8(FIRST, 14), _to_u8(FIRST, 15), _to_u8(FIRST, 16), _to_u8(FIRST, 17),
            _to_u8(FIRST, 18), _to_u8(FIRST, 19), _to_u8(FIRST, 20), _to_u8(FIRST, 21),
            _to_u8(FIRST, 22), _to_u8(FIRST, 23), _to_u8(FIRST, 24), _to_u8(FIRST, 25),
        ];
        let arr12 = [
            _to_u8(FIRST, 26), _to_u8(FIRST, 27), _to_u8(FIRST, 28), _to_u8(FIRST, 29),
            _to_u8(FIRST, 30), _to_u8(FIRST, 31), _to_u8(FIRST, 32), _to_u8(FIRST, 33),
            _to_u8(FIRST, 34), _to_u8(FIRST, 35), _to_u8(FIRST, 36), _to_u8(FIRST, 37),
        ];

        let vlen = (_to_u8(SECOND, 0) as usize) % 65;
        let mut vec1 = Vec::with_capacity(vlen);
        for i in 0..vlen {
            vec1.push(_to_u8(SECOND, i));
        }

        let s1_start = (_to_u8(FIRST, 38) % 10) as usize;
        let s1_end = s1_start + (_to_u8(FIRST, 39) % 20) as usize;
        let s1_slice = &FIRST[s1_start..s1_end];

        let mut sv1: SmallVec<[u8; 16]> = match choice {
            0 => SmallVec::<[u8; 16]>::new(),
            1 => SmallVec::<[u8; 16]>::with_capacity(_to_usize(FIRST, 2)),
            2 => SmallVec::<[u8; 16]>::from_vec(vec1.clone()),
            3 => SmallVec::<[u8; 16]>::from_slice(s1_slice),
            4 => SmallVec::<[u8; 16]>::from_buf(arr16),
            5 => SmallVec::<[u8; 16]>::from_buf_and_len(arr16, _to_usize(FIRST, 4)),
            6 => SmallVec::<[u8; 16]>::from_iter(vec1.clone().into_iter()),
            _ => SmallVec::<[u8; 16]>::from_elem(_to_u8(SECOND, 2), _to_usize(SECOND, 3)),
        };

        let c1 = (&sv1).capacity();
        println!("{}", c1);
        let _ = (&sv1).len();
        let _ = (&sv1).is_empty();
        let c2 = (&sv1).capacity();
        println!("{}", c2);

        let sref = sv1.as_slice();
        if let Some(r) = sref.get(0) {
            println!("{}", *r as u8);
        }
        let sref_mut = sv1.as_mut_slice();
        if !sref_mut.is_empty() {
            sref_mut[0] = sref_mut[0].wrapping_add(1);
            println!("{}", sref_mut[0]);
        }
        let b: &[u8] = sv1.borrow();
        if let Some(r) = b.get(0) {
            println!("{}", *r as u8);
        }
        if sv1.len() > 0 {
            let v0 = sv1[0];
            println!("{}", v0);
            sv1[0] = v0.wrapping_add(1);
        }

        let mut sv2: SmallVec<[u8; 12]> = match _to_u8(SECOND, 1) % 3 {
            0 => SmallVec::<[u8; 12]>::from_buf(arr12),
            1 => SmallVec::<[u8; 12]>::from_vec(vec1.clone()),
            _ => SmallVec::<[u8; 12]>::new(),
        };
        println!("{}", (&sv2).capacity());

        let ops = (_to_u8(FIRST, 5) % 10) as usize + 1;
        for i in 0..ops {
            let code = _to_u8(FIRST, 6 + (i % 20));
            match code % 15 {
                0 => {
                    sv1.push(_to_u8(SECOND, (i % 60) as usize));
                }
                1 => {
                    let _ = sv1.pop();
                }
                2 => {
                    let idx = _to_usize(FIRST, 30);
                    let val = _to_u8(SECOND, ((i + 1) % 60) as usize);
                    sv1.insert(idx, val);
                }
                3 => {
                    if !sv1.is_empty() {
                        let idx = _to_usize(FIRST, 35);
                        let _ = sv1.remove(idx);
                    }
                }
                4 => {
                    sv1.reserve(_to_usize(FIRST, 40));
                }
                5 => {
                    sv1.reserve_exact(_to_usize(SECOND, 44));
                }
                6 => {
                    let _ = sv1.try_reserve(_to_usize(FIRST, 48));
                }
                7 => {
                    let r = 0.._to_usize(FIRST, 56);
                    {
                        let mut dr = sv1.drain(r);
                        let _ = dr.next();
                        let _ = dr.next_back();
                    }
                }
                8 => {
                    let idx = _to_usize(SECOND, 52);
                    sv1.insert_many(idx, vec1.clone().into_iter());
                }
                9 => {
                    let s2_start = (_to_u8(FIRST, 42) % 10) as usize;
                    let s2_end = s2_start + (_to_u8(FIRST, 43) % 20) as usize;
                    let s2 = &FIRST[s2_start..s2_end];
                    let idx = _to_usize(SECOND, 60);
                    sv1.insert_from_slice(idx, s2);
                }
                10 => {
                    sv1.truncate(_to_usize(SECOND, 60));
                }
                11 => {
                    sv1.grow(_to_usize(FIRST, 62));
                }
                12 => {
                    let _ = sv1.try_grow(_to_usize(SECOND, 64));
                }
                13 => {
                    sv1.extend_from_slice(s1_slice);
                }
                _ => {
                    sv1.retain(|e| {
                        if _to_u8(FIRST, 7) % 2 == 0 {
                            panic!("INTENTIONAL PANIC!");
                        }
                        *e % 2 == (_to_u8(SECOND, 7) % 2)
                    });
                }
            }
            let cap_loop = (&sv1).capacity();
            println!("{}", cap_loop);
        }

        let eqv = smallvec::SmallVec::eq(&sv1, &sv2);
        println!("{}", eqv);
        let _ = sv1.as_slice().cmp(sv2.as_slice());

        let s3_start = (_to_u8(FIRST, 70) % 10) as usize;
        let s3_end = s3_start + (_to_u8(FIRST, 71) % 20) as usize;
        let s3 = &FIRST[s3_start..s3_end];
        sv1.extend_from_slice(s3);
        let cap_after = (&sv1).capacity();
        println!("{}", cap_after);

        sv1.resize(_to_usize(SECOND, 68), _to_u8(FIRST, 8));
        sv1.resize_with(_to_usize(FIRST, 72), || {
            if _to_u8(SECOND, 9) % 3 == 0 {
                panic!("INTENTIONAL PANIC!");
            }
            _to_u8(FIRST, 9)
        });
        sv1.dedup();
        sv1.dedup_by(|a, b| (*a % 2) == (*b % 2));
        sv1.dedup_by_key(|k| *k);

        let rr = sv1.as_slice();
        if let Some(x) = rr.get(0) {
            println!("{}", *x as u8);
        }

        let mut it = SmallVec::<[u8; 12]>::into_iter(sv2.clone());
        let sl = it.as_slice();
        if let Some(xx) = sl.get(0) {
            println!("{}", *xx as u8);
        }
        let _ = it.next();
        let _ = it.next_back();
        let sl_mut = it.as_mut_slice();
        if let Some(x) = sl_mut.get(0) {
            println!("{}", *x as u8);
        }

        let bs = sv2.clone().into_boxed_slice();
        println!("{}", bs.len());

        let v_final = sv1.into_vec();
        println!("{}", v_final.len());
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