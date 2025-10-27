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
        let G1 = global.first_half;
        let G2 = global.second_half;

        let mut v: SmallVec<[u8; 16]> = match _to_u8(G1, 0) % 6 {
            0 => SmallVec::<[u8; 16]>::new(),
            1 => SmallVec::<[u8; 16]>::with_capacity(_to_usize(G1, 1)),
            2 => {
                let a: [u8; 16] = [
                    _to_u8(G1, 2), _to_u8(G1, 3), _to_u8(G1, 4), _to_u8(G1, 5),
                    _to_u8(G1, 6), _to_u8(G1, 7), _to_u8(G1, 8), _to_u8(G1, 9),
                    _to_u8(G1, 10), _to_u8(G1, 11), _to_u8(G1, 12), _to_u8(G1, 13),
                    _to_u8(G1, 14), _to_u8(G1, 15), _to_u8(G1, 16), _to_u8(G1, 17),
                ];
                SmallVec::<[u8; 16]>::from_buf(a)
            }
            3 => {
                let l = (_to_u8(G1, 18) % 65) as usize;
                let mut vec = Vec::with_capacity(l);
                for i in 0..l {
                    vec.push(_to_u8(G1, 19 + (i % 32) as usize));
                }
                SmallVec::<[u8; 16]>::from_vec(vec)
            }
            4 => {
                let l = (_to_u8(G2, 0) % 65) as usize;
                let mut tmp = Vec::with_capacity(l);
                for i in 0..l {
                    tmp.push(_to_u8(G2, 1 + (i % 32) as usize));
                }
                SmallVec::<[u8; 16]>::from_slice(&tmp)
            }
            _ => SmallVec::<[u8; 16]>::from_elem(_to_u8(G1, 20), _to_usize(G1, 21)),
        };

        let pushes = (_to_u8(G2, 2) % 20) as usize;
        for i in 0..pushes {
            v.push(_to_u8(G1, 22 + (i % 28) as usize));
        }

        let mut w: SmallVec<[u8; 16]> = match _to_u8(G2, 60) % 4 {
            0 => SmallVec::<[u8; 16]>::from_elem(_to_u8(G2, 3), _to_usize(G2, 4)),
            1 => SmallVec::<[u8; 16]>::new(),
            2 => {
                let l = (_to_u8(G2, 5) % 65) as usize;
                let mut v2 = Vec::with_capacity(l);
                for i in 0..l {
                    v2.push(_to_u8(G2, 6 + (i % 32) as usize));
                }
                SmallVec::<[u8; 16]>::from_vec(v2)
            }
            _ => {
                let b: [u8; 16] = [
                    _to_u8(G2, 30), _to_u8(G2, 31), _to_u8(G2, 32), _to_u8(G2, 33),
                    _to_u8(G2, 34), _to_u8(G2, 35), _to_u8(G2, 36), _to_u8(G2, 37),
                    _to_u8(G2, 38), _to_u8(G2, 39), _to_u8(G2, 40), _to_u8(G2, 41),
                    _to_u8(G2, 42), _to_u8(G2, 43), _to_u8(G2, 44), _to_u8(G2, 45),
                ];
                SmallVec::<[u8; 16]>::from_buf(b)
            }
        };

        let cap = v.capacity();
        let len = v.len();
        let empty = v.is_empty();
        println!("{:?}{:?}{:?}", cap, len, empty);

        v.grow(_to_usize(G2, 8));
        v.grow(_to_usize(G2, 12));
        let _ = v.try_grow(_to_usize(G2, 16));
        v.reserve(_to_usize(G2, 20));
        let _ = v.try_reserve_exact(_to_usize(G2, 24));
        v.reserve_exact(_to_usize(G2, 28));
        let _ = v.try_reserve(_to_usize(G2, 48));

        let sref = v.as_slice();
        println!("{:?}", sref);
        if let Some(x) = sref.get(0) {
            println!("{:?}", *x);
        }
        let ms = v.as_mut_slice();
        if !ms.is_empty() {
            ms[0] = _to_u8(G2, 58);
            println!("{:?}", ms[0]);
        }

        let aref = v.as_ref();
        println!("{:?}", aref);
        let r = v.deref();
        if let Some(x) = r.get(0) {
            println!("{:?}", *x);
        }
        let mr = v.deref_mut();
        if !mr.is_empty() {
            let idx = mr.len() - 1;
            mr[idx] = _to_u8(G2, 10);
            println!("{:?}", mr[idx]);
        }

        let _ = v.partial_cmp(&w);
        let _ = v.cmp(&w);
        let _ = v.eq(&w);

        v.extend_from_slice(w.as_slice());

        v.insert(_to_usize(G2, 32), _to_u8(G2, 33));
        let _ = v.pop();
        let _ = v.remove(_to_usize(G2, 36));
        let _ = v.swap_remove(_to_usize(G2, 40));
        v.truncate(_to_usize(G2, 44));
        v.shrink_to_fit();

        let l2 = (_to_u8(G2, 11) % 65) as usize;
        let mut tmp2 = Vec::with_capacity(l2);
        for i in 0..l2 {
            tmp2.push(_to_u8(G2, 12 + (i % 32) as usize));
        }
        v.insert_from_slice(_to_usize(G2, 52), &tmp2);

        {
            let drain_end = _to_usize(G2, 62);
            let mut d = v.drain(0..drain_end);
            let _ = d.next();
            let _ = d.next_back();
        }

        v.append(&mut w);

        v.dedup();
        let mut flag = _to_u8(G2, 46);
        v.retain(|e| {
            flag = flag.wrapping_add(*e);
            flag % 2 == 0
        });
        v.dedup_by(|a, b| {
            let c = _to_u8(G2, 47);
            (*a == *b) || (c % 3 == 0)
        });
        v.dedup_by_key(|x| {
            let k = _to_u8(G2, 49);
            x.wrapping_add(k)
        });

        v.resize(_to_usize(G2, 50), _to_u8(G2, 51));
        let mut tcount = 0u8;
        v.resize_with(_to_usize(G2, 54), || {
            tcount = tcount.wrapping_add(1);
            tcount
        });

        let sl = v.as_slice();
        if let Some(y) = sl.get(0) {
            println!("{:?}", *y);
        }
        let _ = v.as_ptr();
        let _ = v.as_mut_ptr();

        let clonev = v.clone();
        let _ = clonev.into_vec();
        let clonev2 = v.clone();
        let _ = clonev2.into_boxed_slice();
        let _ = v.clone().into_inner();

        let mut it = v.clone().into_iter();
        let _ = it.as_slice();
        let _ = it.as_mut_slice();
        let _ = it.next();
        let _ = it.next_back();
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