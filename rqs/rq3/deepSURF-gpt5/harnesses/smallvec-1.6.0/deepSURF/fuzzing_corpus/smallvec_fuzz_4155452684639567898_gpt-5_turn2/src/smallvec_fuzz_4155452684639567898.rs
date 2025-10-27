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
        if data.len() < 140 { return; }
        set_global_data(data);
        let g = get_global_data();
        let FIRST = g.first_half;
        let SECOND = g.second_half;

        let mut seed_vals = [
            _to_i32(FIRST, 8),
            _to_i32(FIRST, 16),
            _to_i32(FIRST, 24),
            _to_i32(FIRST, 32),
            _to_i32(FIRST, 40),
            _to_i32(FIRST, 48),
            _to_i32(FIRST, 56),
        ];
        let n_vec1 = (_to_u8(FIRST, 0) % 65) as usize;
        let n_vec2 = (_to_u8(SECOND, 0) % 65) as usize;
        let n_push = (_to_u8(FIRST, 1) % 65) as usize;
        let n_insert_many = (_to_u8(SECOND, 1) % 65) as usize;

        let mut vec1: Vec<i32> = Vec::with_capacity(n_vec1);
        for i in 0..n_vec1 {
            vec1.push(seed_vals[i % seed_vals.len()]);
        }
        let mut vec2: Vec<i32> = Vec::with_capacity(n_vec2);
        for i in 0..n_vec2 {
            vec2.push(seed_vals[(i + 3) % seed_vals.len()]);
        }

        let mut arr36 = [0i32; 36];
        for i in 0..36 {
            arr36[i] = seed_vals[i % seed_vals.len()];
        }

        let ctor_sel = _to_u8(FIRST, 2) % 6;
        let mut v_main: SmallVec<[i32; 36]> = match ctor_sel {
            0 => SmallVec::new(),
            1 => SmallVec::with_capacity(_to_usize(FIRST, 24)),
            2 => SmallVec::from_elem(_to_i32(FIRST, 32), _to_usize(FIRST, 40)),
            3 => SmallVec::from_vec(vec1.clone()),
            4 => SmallVec::from_slice(vec1.as_slice()),
            _ => SmallVec::from_buf(arr36),
        };

        for i in 0..n_push {
            v_main.push(seed_vals[(i + 1) % seed_vals.len()]);
        }

        let _ = v_main.capacity();
        let _ = v_main.len();
        let _ = v_main.is_empty();

        let s1 = v_main.as_slice();
        println!("{:?}", s1);
        let s2 = v_main.as_mut_slice();
        println!("{:?}", s2);
        let s3: &[i32] = v_main.borrow();
        println!("{:?}", s3);
        let s4 = v_main.as_ref();
        println!("{:?}", s4);
        let dref = v_main.deref();
        println!("{:?}", dref);

        v_main.reserve(_to_usize(FIRST, 48));
        let _ = v_main.try_reserve(_to_usize(SECOND, 48));
        v_main.reserve_exact(_to_usize(FIRST, 56));
        let _ = v_main.try_reserve_exact(_to_usize(SECOND, 56));
        v_main.grow(_to_usize(FIRST, 8));
        let _ = v_main.try_grow(_to_usize(SECOND, 8));

        v_main.insert(_to_usize(SECOND, 16), _to_i32(SECOND, 24));
        let _ = v_main.pop();
        let _ = v_main.swap_remove(_to_usize(SECOND, 32));

        v_main.extend_from_slice(vec2.as_slice());
        v_main.insert_from_slice(_to_usize(SECOND, 40), vec2.as_slice());
        v_main.resize(_to_usize(SECOND, 24), _to_i32(FIRST, 16));
        v_main.resize_with(_to_usize(SECOND, 32), || _to_i32(FIRST, 24));

        v_main.dedup();
        v_main.dedup_by(|a, b| *a == *b);
        v_main.dedup_by_key(|x| *x);
        let keep_flag0 = _to_bool(FIRST, 12);
        let mut flip = keep_flag0;
        v_main.retain(|_| {
            flip = !flip;
            flip
        });

        let _ = v_main.index(_to_usize(FIRST, 20));
        let _ = v_main.index_mut(_to_usize(FIRST, 28));
        let _ = v_main[_to_usize(SECOND, 20)];
        if v_main.len() > 0 {
            let r = &mut v_main[0];
            println!("{:?}", r);
            *r ^= _to_i32(FIRST, 12);
        }

        let mut dr = v_main.drain(0.._to_usize(FIRST, 16));
        let _ = dr.next();
        let _ = dr.next_back();
        drop(dr);

        let mut v_for_inner = v_main.clone();
        let res = v_for_inner.into_inner();
        match res {
            Ok(arr) => {
                println!("{:?}", &arr[..]);
                let mut sv_from_buf = SmallVec::<[i32; 36]>::from_buf(arr);
                let s = sv_from_buf.as_slice();
                println!("{:?}", s);
                let _ = sv_from_buf.into_vec();
            }
            Err(mut back) => {
                let s = back.as_slice();
                println!("{:?}", s);
                let _ = back.capacity();
            }
        }

        let mut it = v_main.clone().into_iter();
        let it_s = it.as_slice();
        println!("{:?}", it_s);
        let it_sm = it.as_mut_slice();
        println!("{:?}", it_sm);
        let _ = it.next();
        let _ = it.next_back();

        let other = SmallVec::<[i32; 36]>::from_vec(vec2.clone());
        let _ = v_main.eq(&other);
        let ord = v_main.cmp(&other);
        match ord {
            std::cmp::Ordering::Less => {}
            std::cmp::Ordering::Equal => {}
            std::cmp::Ordering::Greater => {}
        }

        let mut other2 = SmallVec::<[i32; 36]>::from_slice(vec1.as_slice());
        v_main.append(&mut other2);

        v_main.truncate(_to_usize(SECOND, 44));

        let iters = (_to_u8(SECOND, 4) % 8) as usize;
        for i in 0..iters {
            let op = _to_u8(SECOND, 5 + i) % 8;
            match op {
                0 => v_main.push(_to_i32(SECOND, 8)),
                1 => { let _ = v_main.pop(); }
                2 => v_main.reserve(_to_usize(FIRST, 24)),
                3 => { let _ = v_main.try_reserve_exact(_to_usize(SECOND, 16)); }
                4 => v_main.clear(),
                5 => v_main.shrink_to_fit(),
                6 => v_main.insert(_to_usize(FIRST, 32), _to_i32(SECOND, 24)),
                _ => { let _ = v_main.swap_remove(_to_usize(SECOND, 32)); }
            }
        }

        let many = std::iter::repeat(_to_i32(FIRST, 56)).take(n_insert_many);
        v_main.insert_many(_to_usize(SECOND, 36), many);

        let _ = v_main.clone().into_inner();
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