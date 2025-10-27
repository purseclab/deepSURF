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
        if data.len() < 140 { return; }
        set_global_data(data);
        let global = get_global_data();
        let g1 = global.first_half;
        let g2 = global.second_half;

        let mut arr1 = [0u8; 32];
        for i in 0..32 { arr1[i] = _to_u8(g1, 9 + i); }
        let mut arr2 = [0u8; 32];
        for i in 0..32 { arr2[i] = _to_u8(g1, 9 + i); }

        let ctor_tag = _to_u8(g1, 0) % 8;
        let mut v: SmallVec<[u8; 32]> = match ctor_tag {
            0 => SmallVec::new(),
            1 => SmallVec::with_capacity(_to_usize(g1, 1)),
            2 => SmallVec::from_buf(arr1),
            3 => SmallVec::from_buf_and_len(arr2, _to_usize(g1, 42)),
            4 => {
                let end = 9 + (g1.len().saturating_sub(9).min(32));
                SmallVec::from_slice(&g1[9..end])
            }
            5 => {
                let len0 = (_to_u8(g2, 0) % 65) as usize;
                let len = len0.min(g2.len());
                let mut tmp = Vec::with_capacity(len);
                for i in 0..len { tmp.push(g2[i]); }
                SmallVec::from_vec(tmp)
            }
            6 => SmallVec::from_elem(_to_u8(g1, 5), _to_usize(g1, 50)),
            _ => SmallVec::from_iter(g2.iter().cloned().take(_to_usize(g1, 6))),
        };

        let _ = v.pop();

        let t_len0 = (_to_u8(g2, 0) % 65) as usize;
        let t_len = t_len0.min(g2.len());
        for i in 0..t_len { v.push(g2[i]); }

        v.reserve(_to_usize(g2, 25));
        let _ = v.try_reserve(_to_usize(g2, 41));
        v.reserve_exact(_to_usize(g2, 33));
        let _ = v.try_reserve_exact(_to_usize(g1, 26));
        v.grow(_to_usize(g1, 26));

        v.extend_from_slice(&g2[0..t_len]);
        v.insert_many(_to_usize(g2, 1), g2[0..t_len].iter().cloned());
        v.insert_from_slice(_to_usize(g2, 9), &g2[0..t_len]);
        v.insert(_to_usize(g2, 1), _to_u8(g1, 7));

        let s_ref = v.as_slice();
        println!("{:?}", s_ref);
        let s_borrow: &[u8] = v.borrow();
        println!("{:?}", s_borrow);
        let s_mut_ref = v.as_mut_slice();
        if !s_mut_ref.is_empty() {
            s_mut_ref[0] = s_mut_ref[0].wrapping_add(1);
        }
        println!("{:?}", s_mut_ref);

        if !v.is_empty() {
            let first_ref = &v[0];
            println!("{}", *first_ref);
            let first_mut = &mut v[0];
            *first_mut = first_mut.wrapping_add(1);
        }

        v.retain(|x| {
            let b = _to_bool(g1, 8);
            if b { *x = x.wrapping_add(_to_u8(g1, 9)); }
            true
        });
        v.dedup();
        v.dedup_by(|a, b| {
            let flip = _to_bool(g1, 10);
            if flip { *a == *b } else { false }
        });
        v.dedup_by_key(|k| {
            let add = _to_u8(g1, 11);
            *k = k.wrapping_add(add);
            add
        });

        v.resize_with(_to_usize(g1, 50), || _to_u8(g1, 12));
        v.truncate(_to_usize(g2, 57));

        let mut v2 = SmallVec::<[u8; 32]>::from_slice(&g2[0..t_len]);
        let _ = v.eq(&v2);
        let _ = v.partial_cmp(&v2);
        let _ = v.cmp(&v2);
        v.append(&mut v2);

        let range_end = _to_usize(g1, 50);
        let mut d = v.drain(0..range_end);
        let _ = d.next();
        let _ = d.next_back();
        drop(d);

        println!("{:?}", v.as_slice());

        let mut it = v.clone().into_iter();
        println!("{:?}", it.as_slice());
        let _ = it.next();
        let _ = it.next_back();

        for _ in 0..3 {
            let popped = v.pop();
            if let Some(val) = popped { println!("{}", val); }
        }

        let vec_out = v.clone().into_vec();
        println!("{}", vec_out.len());
        let _ = v.clone().into_boxed_slice();

        {
            let mref: &mut [u8] = v.borrow_mut();
            println!("{:?}", mref);
        }
        {
            let dref = v.deref();
            println!("{:?}", dref);
        }

        let s_small: SmallVec<[u8; 32]> = (&g2[0..t_len]).to_smallvec();
        let mut s_small2 = s_small;
        v.append(&mut s_small2);

        let _cap = v.capacity();
        let _len = v.len();
        let _empty = v.is_empty();
        let _ptr = v.as_ptr();
        if v.len() > 0 {
            let last_ref = &v[v.len() - 1];
            println!("{}", *last_ref);
        }

        let _ = v.try_grow(_to_usize(g1, 42));
        v.shrink_to_fit();
        v.clear();
        let _ = v.pop();
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