#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 128 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let FIRST = global_data.first_half;
        let SECOND = global_data.second_half;

        let mode = _to_u8(FIRST, 0);
        let mut sv: SmallVec<[u8; 32]> = match mode % 6 {
            0 => SmallVec::new(),
            1 => SmallVec::with_capacity(_to_usize(FIRST, 32)),
            2 => {
                let n = (_to_u8(FIRST, 1) % 65) as usize;
                let mut v = Vec::with_capacity(n);
                for i in 0..n {
                    v.push(_to_u8(FIRST, 2 + i));
                }
                SmallVec::from_vec(v)
            }
            3 => {
                let n = (_to_u8(FIRST, 3) % 65) as usize;
                let mut v = Vec::with_capacity(n);
                for i in 0..n {
                    v.push(_to_u8(FIRST, 4 + i));
                }
                SmallVec::from_slice(&v)
            }
            4 => {
                let mut buf = [0u8; 32];
                for i in 0..32 {
                    buf[i] = _to_u8(FIRST, i);
                }
                SmallVec::from_buf(buf)
            }
            _ => {
                let mut buf = [0u8; 32];
                for i in 0..32 {
                    buf[i] = _to_u8(FIRST, 16 + i);
                }
                SmallVec::from_buf_and_len(buf, _to_usize(FIRST, 40))
            }
        };

        let push_count = (_to_u8(SECOND, 0) % 10) as usize;
        for i in 0..push_count {
            sv.push(_to_u8(SECOND, 1 + i));
        }

        let idx_ins = _to_usize(SECOND, 8);
        sv.insert(idx_ins, _to_u8(SECOND, 2));

        let slice_len = (_to_u8(SECOND, 3) % 65) as usize;
        let mut tmp_ext = Vec::with_capacity(slice_len);
        for i in 0..slice_len {
            tmp_ext.push(_to_u8(SECOND, 4 + i));
        }
        sv.extend_from_slice(&tmp_ext);

        {
            let s = sv.as_slice();
            if !s.is_empty() {
                println!("{}", s[0]);
            }
        }
        {
            let m = sv.as_mut_slice();
            if !m.is_empty() {
                m[0] = m[0].wrapping_add(_to_u8(SECOND, 10));
                println!("{}", m[0]);
            }
        }

        sv.retain(|x| {
            let b = _to_u8(FIRST, 5);
            (*x).wrapping_add(b) % 2 == 0
        });

        sv.dedup_by(|a, b| {
            if _to_u8(FIRST, 6) % 2 == 0 {
                panic!("INTENTIONAL PANIC!");
            }
            *a == *b
        });

        sv.reserve(_to_usize(FIRST, 48));
        println!("{}", sv.capacity());
        if sv.len() > 0 {
            let r = &sv[0];
            println!("{}", *r);
        }

        let ops = (_to_u8(SECOND, 31) % 20) as usize;
        for i in 0..ops {
            let op = _to_u8(SECOND, (i % 60) as usize);
            match op % 20 {
                0 => {
                    sv.reserve_exact(_to_usize(SECOND, 16));
                }
                1 => {
                    sv.try_reserve_exact(_to_usize(SECOND, 17)).ok();
                }
                2 => {
                    sv.truncate(_to_usize(FIRST, 8));
                }
                3 => {
                    sv.pop();
                }
                4 => {
                    let idx = _to_usize(SECOND, 24);
                    let _ = sv.get(idx).copied();
                }
                5 => {
                    let idx = _to_usize(FIRST, 24);
                    if sv.len() > 0 {
                        let _ = sv.swap_remove(idx);
                    }
                }
                6 => {
                    let idx = _to_usize(SECOND, 25);
                    if sv.len() > 0 {
                        let _ = sv.remove(idx);
                    }
                }
                7 => {
                    let new_len = _to_usize(SECOND, 32);
                    sv.resize_with(new_len, || _to_u8(FIRST, 7));
                }
                8 => {
                    sv.clear();
                }
                9 => {
                        let n = (_to_u8(FIRST, 8) % 65) as usize;
                        let mut v = Vec::with_capacity(n);
                        for k in 0..n {
                            v.push(_to_u8(FIRST, 9 + k));
                        }
                        let idx = _to_usize(SECOND, 26);
                        sv.insert_from_slice(idx, &v);
                }
                10 => {
                    let end = _to_usize(SECOND, 36);
                    let mut dr = sv.drain(0..end);
                    let _ = dr.next();
                    let _ = dr.next_back();
                }
                11 => {
                    let s = sv.as_slice();
                    if !s.is_empty() {
                        println!("{}", s[0]);
                    }
                }
                12 => {
                    let m = sv.as_mut_slice();
                    if !m.is_empty() {
                        m[0] = m[0].wrapping_sub(_to_u8(FIRST, 10));
                        println!("{}", m[0]);
                    }
                }
                13 => {
                    let other_len = (_to_u8(SECOND, 40) % 5) as usize;
                    let mut other: SmallVec<[u8; 16]> = SmallVec::from_elem(_to_u8(SECOND, 41), other_len);
                    sv.append(&mut other);
                }
                14 => {
                    let _ = sv.try_reserve(_to_usize(FIRST, 49)).ok();
                }
                15 => {
                    let _ = sv.partial_cmp(&sv.clone());
                    let _ = sv.cmp(&sv.clone());
                }
                16 => {
                    sv.shrink_to_fit();
                }
                17 => {
                    sv.dedup();
                }
                18 => {
                    let v = sv.clone().into_vec();
                    sv = SmallVec::from_vec(v);
                }
                _ => {
                    let end = _to_usize(FIRST, 12);
                    sv.truncate(end);
                }
            }
        }

        sv.reserve_exact(_to_usize(SECOND, 18));

        sv.shrink_to_fit();
        println!("{}", sv.len());
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