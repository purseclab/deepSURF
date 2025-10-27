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
        let global = get_global_data();
        let g1 = global.first_half;
        let g2 = global.second_half;

        let mut arr16 = [0u8; 16];
        for i in 0..16 {
            arr16[i] = g1[i % g1.len()];
        }
        let mut arr24 = [0u8; 24];
        for i in 0..24 {
            arr24[i] = g2[i % g2.len()];
        }
        let mut arr32 = [0u8; 32];
        for i in 0..32 {
            arr32[i] = g1[(i + 7) % g1.len()];
        }

        let vlen1 = (_to_u8(g1, 1) % 65) as usize;
        let mut v1: Vec<u8> = (0..vlen1).map(|i| g1[(2 + i) % g1.len()]).collect();
        let vlen2 = (_to_u8(g1, 3) % 65) as usize;
        let v2: Vec<u8> = (0..vlen2).map(|i| g2[(4 + i) % g2.len()]).collect();

        let split = if v1.is_empty() { 0 } else { _to_usize(g1, 5 % (g1.len().saturating_sub(8).max(1))) % v1.len() };
        let slice1 = &v1[..split];

        let sel = _to_u8(g1, 0);
        let mut sv: SmallVec<[u8; 32]> = match sel % 8 {
            0 => SmallVec::<[u8; 32]>::new(),
            1 => SmallVec::<[u8; 32]>::with_capacity(_to_usize(g1, (7) % (g1.len().saturating_sub(8)))),
            2 => SmallVec::<[u8; 32]>::from_vec(v1.clone()),
            3 => SmallVec::<[u8; 32]>::from_buf(arr32),
            4 => SmallVec::<[u8; 32]>::from_buf_and_len(arr32, _to_usize(g1, (9) % (g1.len().saturating_sub(8)))),
            5 => SmallVec::<[u8; 32]>::from_slice(&slice1),
            6 => SmallVec::<[u8; 32]>::from_elem(_to_u8(g1, 11), (_to_u8(g1, 12) % 65) as usize),
            _ => SmallVec::<[u8; 32]>::from_iter((0..((_to_u8(g1, 13) % 65) as usize)).map(|i| g2[(i + 5) % g2.len()])),
        };

        let p0 = (&sv).as_ptr();
        println!("{:p}", p0);

        let sv2_slice = v2.as_slice();
        let mut sv2: SmallVec<[u8; 24]> = sv2_slice.to_smallvec();
        let p1 = (&sv2).as_ptr();
        println!("{:p}", p1);

        let ops = (_to_u8(g2, 0) % 20) as usize;
        for i in 0..ops {
            let code = _to_u8(g2, (i + 1) % g2.len());
            match code % 16 {
                0 => {
                    sv.push(_to_u8(g1, (i + 13) % g1.len()));
                }
                1 => {
                    let idx = _to_usize(g2, ((i * 3) % (g2.len().saturating_sub(8))).max(0));
                    sv.insert(idx, _to_u8(g1, (i + 17) % g1.len()));
                }
                2 => {
                    let _ = sv.pop();
                }
                3 => {
                    let idx = _to_usize(g1, ((i * 5) % (g1.len().saturating_sub(8))).max(0));
                    let _ = sv.swap_remove(idx);
                }
                4 => {
                    let additional = _to_usize(g2, ((i * 7) % (g2.len().saturating_sub(8))).max(0));
                    sv.reserve(additional);
                }
                5 => {
                    let additional = _to_usize(g1, ((i * 9) % (g1.len().saturating_sub(8))).max(0));
                    let _ = sv.try_reserve_exact(additional);
                }
                6 => {
                    let new_len = (_to_u8(g1, (i * 11) % g1.len()) % 65) as usize;
                    sv.resize(new_len, _to_u8(g2, (i * 13) % g2.len()));
                }
                7 => {
                    let new_len = (_to_u8(g2, (i * 15) % g2.len()) % 65) as usize;
                    let flag = _to_u8(g1, (i * 17) % g1.len());
                    sv.resize_with(new_len, || {
                        if flag % 2 == 0 {
                            panic!("INTENTIONAL PANIC!");
                        }
                        _to_u8(g1, (i * 19) % g1.len())
                    });
                }
                8 => {
                    let idx = _to_usize(g1, ((i * 21) % (g1.len().saturating_sub(8))).max(0));
                    let slice_len = (_to_u8(g2, (i * 23) % g2.len()) % 65) as usize;
                    let vtemp: Vec<u8> = (0..slice_len).map(|k| g2[(k + i) % g2.len()]).collect();
                    sv.insert_from_slice(idx, vtemp.as_slice());
                }
                9 => {
                    let slen = (_to_u8(g1, (i * 25) % g1.len()) % 65) as usize;
                    let vtemp: Vec<u8> = (0..slen).map(|k| g1[(k + i) % g1.len()]).collect();
                    sv.extend_from_slice(vtemp.as_slice());
                }
                10 => {
                    sv.dedup();
                }
                11 => {
                    let idx_dbg = _to_u8(g1, (i * 27) % g1.len());
                    sv.dedup_by(|a, b| {
                        println!("{}", *a as u8);
                        println!("{}", *b as u8);
                        if idx_dbg % 2 == 0 {
                            panic!("INTENTIONAL PANIC!");
                        }
                        (*a % 2) == (*b % 2)
                    });
                }
                12 => {
                    let keep_sel = _to_u8(g2, (i * 29) % g2.len());
                    sv.retain(|e| {
                        println!("{}", *e as u8);
                        if keep_sel % 3 == 0 {
                            panic!("INTENTIONAL PANIC!");
                        }
                        keep_sel % 2 == 0
                    });
                }
                13 => {
                    let idx = _to_usize(g2, ((i * 31) % (g2.len().saturating_sub(8))).max(0));
                    let _ = sv.remove(idx);
                }
                14 => {
                    let range_end = _to_usize(g1, ((i * 33) % (g1.len().saturating_sub(8))).max(0));
                    let mut dr = sv.drain(0..range_end);
                    if let Some(v) = dr.next() {
                        println!("{}", v as u8);
                    }
                    if let Some(vb) = dr.next_back() {
                        println!("{}", vb as u8);
                    }
                }
                _ => {
                    sv.append(&mut sv2);
                }
            }

            let ptr_loop = (&sv).as_ptr();
            println!("{:p}", ptr_loop);

            let s = sv.as_slice();
            println!("{}", s.len());
            if !s.is_empty() {
                println!("{}", s[0]);
            }
            let rlen = if sv.len() >= 1 { 1 } else { 0 };
            if rlen > 0 {
                let r = &sv[0..rlen];
                println!("{}", r[0]);
            }
        }

        let p2 = (&sv).as_ptr();
        println!("{:p}", p2);

        let cap = (&sv).capacity();
        println!("{}", cap);
        let is_empty = (&sv).is_empty();
        println!("{}", is_empty);

        let vfinal = sv.clone().into_vec();
        println!("{}", vfinal.len());
        let mut sv3 = SmallVec::<[u8; 16]>::from_vec(vfinal);
        let p3 = (&sv3).as_ptr();
        println!("{:p}", p3);

        let m = sv3.as_mut_slice();
        if !m.is_empty() {
            m[0] = m[0].wrapping_add(1);
            println!("{}", m[0]);
        }

        let sv3b = SmallVec::<[u8; 32]>::from_slice(sv3.as_slice());
        let cmp_ord = SmallVec::<[u8; 32]>::from_slice(&arr24).partial_cmp(&sv3b);
        match cmp_ord {
            Some(ord) => println!("{:?}", ord as i32),
            None => println!("{}", 0),
        }
        let _ = SmallVec::<[u8; 32]>::from_slice(&arr24).cmp(&sv3b);
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