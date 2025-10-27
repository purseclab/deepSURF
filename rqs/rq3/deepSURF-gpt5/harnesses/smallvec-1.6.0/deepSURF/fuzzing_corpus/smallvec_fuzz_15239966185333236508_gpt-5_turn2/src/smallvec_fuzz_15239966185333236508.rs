#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

fn same_bucket(a: &mut u8, b: &mut u8) -> bool {
    let global_data = get_global_data();
    let FIRST = global_data.first_half;
    let t_9 = _to_u8(FIRST, 34);
    if t_9 % 2 == 0 {
        panic!("INTENTIONAL PANIC!");
    }
    println!("{}", *a);
    println!("{}", *b);
    _to_bool(FIRST, 35)
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 200 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let FIRST = global_data.first_half;
        let SECOND = global_data.second_half;

        let sel = _to_u8(FIRST, 0);
        let cap = _to_usize(FIRST, 1);
        let fill = _to_u8(FIRST, 2);
        let n_take = (_to_u8(FIRST, 3) as usize) % 65;

        let mut buf_arr32 = [0u8; 32];
        for i in 0..32 {
            let idx = i % FIRST.len();
            buf_arr32[i] = FIRST[idx];
        }

        let mut v: SmallVec<[u8; 32]> = match sel % 7 {
            0 => SmallVec::new(),
            1 => SmallVec::with_capacity(cap),
            2 => {
                let n = if n_take <= SECOND.len() { n_take } else { SECOND.len() };
                SmallVec::from_slice(&SECOND[0..n])
            }
            3 => {
                let n = if n_take <= SECOND.len() { n_take } else { SECOND.len() };
                let mut vec = Vec::with_capacity(n);
                for i in 0..n { vec.push(SECOND[i]); }
                SmallVec::from_vec(vec)
            }
            4 => SmallVec::from_buf(buf_arr32),
            5 => SmallVec::from_buf_and_len(buf_arr32, _to_usize(FIRST, 26)),
            _ => {
                let n = if n_take <= SECOND.len() { n_take } else { SECOND.len() };
                SmallVec::from_iter(SECOND[0..n].iter().cloned())
            }
        };

        let ops = (_to_u8(FIRST, 4) as usize) % 20;
        for i in 0..ops {
            let op = _to_u8(FIRST, 5 + (i % 10) as usize);
            match op % 16 {
                0 => {
                    v.reserve(_to_usize(FIRST, 6));
                }
                1 => {
                    let _ = v.try_reserve(_to_usize(FIRST, 7));
                }
                2 => {
                    v.push(fill);
                }
                3 => {
                    let idx = _to_usize(FIRST, 8);
                    v.insert(idx, _to_u8(FIRST, 9));
                }
                4 => {
                    let n = (_to_u8(FIRST, 10) as usize) % 65;
                    let n2 = if n <= SECOND.len() { n } else { SECOND.len() };
                    v.extend_from_slice(&SECOND[0..n2]);
                }
                5 => {
                    v.resize(_to_usize(FIRST, 11), _to_u8(FIRST, 12));
                }
                6 => {
                    let mut c = || -> u8 { _to_u8(SECOND, 13) };
                    v.resize_with(_to_usize(FIRST, 14), &mut c);
                }
                7 => {
                    let idx = _to_usize(FIRST, 15);
                    if v.len() > 0 {
                        let _ = v.get(0).map(|r| println!("{}", *r));
                    }
                    let _ = v.get_mut(0).map(|r| println!("{}", *r));
                    let _ = v.remove(idx);
                }
                8 => {
                    let _ = v.pop();
                }
                9 => {
                    let idx = _to_usize(FIRST, 16);
                    let _ = v.swap_remove(idx);
                }
                10 => {
                    let end = _to_usize(FIRST, 17);
                    let mut d = v.drain(0..end);
                    let _ = d.next();
                    let _ = d.next_back();
                }
                11 => {
                    let other_n = (_to_u8(FIRST, 18) as usize) % 65;
                    let other_len = if other_n <= SECOND.len() { other_n } else { SECOND.len() };
                    let mut other: SmallVec<[u8; 32]> = SmallVec::from_slice(&SECOND[0..other_len]);
                    v.append(&mut other);
                }
                12 => {
                    let m = (_to_u8(FIRST, 27) as usize) % 65;
                    let m2 = if m <= SECOND.len() { m } else { SECOND.len() };
                    v.extend(SECOND[0..m2].iter().cloned());
                }
                13 => {
                    v.grow(_to_usize(FIRST, 28));
                }
                14 => {
                    let idx = _to_usize(FIRST, 25);
                    let m = (_to_u8(FIRST, 29) as usize) % 65;
                    let m2 = if m <= SECOND.len() { m } else { SECOND.len() };
                    v.insert_many(idx, SECOND[0..m2].iter().cloned());
                }
                _ => {
                    let idx = _to_usize(FIRST, 30);
                    let m = (_to_u8(FIRST, 31) as usize) % 65;
                    let m2 = if m <= SECOND.len() { m } else { SECOND.len() };
                    v.insert_from_slice(idx, &SECOND[0..m2]);
                }
            }
        }

        let c = v.capacity();
        let l = v.len();
        let e = v.is_empty();
        println!("{} {} {}", c, l, e as u8);

        {
            let s = v.as_slice();
            if let Some(x) = s.get(0) { println!("{}", *x); }
        }
        {
            let sm = v.as_mut_slice();
            if let Some(x) = sm.get_mut(0) { println!("{}", *x); }
        }
        if v.len() > 0 {
            let r = &v[0];
            println!("{}", *r);
            let r2 = &mut v.as_mut_slice()[0];
            println!("{}", *r2);
        }

        let mut f = same_bucket;
        v.dedup_by(&mut f);

        let mut g = |x: &mut u8| -> bool {
            let t = _to_u8(FIRST, 34);
            if t % 2 == 0 { panic!("INTENTIONAL PANIC!"); }
            println!("{}", *x);
            _to_bool(FIRST, 35)
        };
        v.retain(&mut g);

        {
            let mut key = |x: &mut u8| -> u8 {
                println!("{}", *x);
                _to_u8(FIRST, 19)
            };
            v.dedup_by_key(&mut key);
        }

        v.dedup();

        let s2 = v.as_slice();
        let v2: SmallVec<[u8; 32]> = SmallVec::from_slice(s2);
        let _ = v.partial_cmp(&v2);
        let _ = v.cmp(&v2);
        println!("{}", v == v2);

        let _ = v.try_reserve_exact(_to_usize(FIRST, 21));
        v.reserve_exact(_to_usize(FIRST, 22));
        v.truncate(_to_usize(FIRST, 23));
        v.shrink_to_fit();

        let mut iter = v.clone().into_iter();
        let sl = iter.as_slice();
        if let Some(x) = sl.get(0) { println!("{}", *x); }
        let sml = iter.as_mut_slice();
        if let Some(x) = sml.get_mut(0) { println!("{}", *x); }
        let _ = iter.next();
        let _ = iter.next_back();

        let _ = v.as_ref();
        let _ = v.as_mut();
        let _ = v.deref();
        let _ = v.deref_mut();

        let end2 = _to_usize(FIRST, 24);
        let _ = v.drain(..end2);

        let boxed = v.clone().into_boxed_slice();
        println!("{}", boxed.len());
        let vecv = v.clone().into_vec();
        println!("{}", vecv.len());
        let _ = v.into_inner();
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