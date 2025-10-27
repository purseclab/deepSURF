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
        let global_data = get_global_data();
        let g1 = global_data.first_half;
        let g2 = global_data.second_half;

        let mut arr32: [u8; 32] = [0; 32];
        for i in 0..32 {
            arr32[i] = _to_u8(g1, 16 + i);
        }

        let len_vec_u8 = (_to_u8(g2, 0) % 65) as usize;
        let mut vec_u8 = Vec::with_capacity(len_vec_u8);
        let g2_safe = (g2.len().saturating_sub(1)).max(1);
        for i in 0..len_vec_u8 {
            vec_u8.push(_to_u8(g2, 1 + (i % g2_safe)));
        }

        let len_slice = (_to_u8(g1, 60) % 65) as usize;
        let end = std::cmp::min(len_slice, g2.len());
        let slice_u8 = &g2[..end];

        let tag = _to_u8(g1, 50) % 6;
        let mut sv1: SmallVec<[u8; 32]> = match tag {
            0 => SmallVec::<[u8; 32]>::new(),
            1 => SmallVec::<[u8; 32]>::with_capacity(_to_usize(g1, 54)),
            2 => SmallVec::from_buf(arr32),
            3 => SmallVec::from_buf_and_len(arr32, _to_usize(g1, 24)),
            4 => SmallVec::from_vec(vec_u8.clone()),
            _ => SmallVec::from_slice(slice_u8),
        };

        if !_to_bool(g1, 52) {
            sv1.push(_to_u8(g1, 53));
            sv1.reserve(_to_usize(g1, 28));
        } else {
            let _ = sv1.try_reserve(_to_usize(g1, 32));
            let _ = sv1.try_reserve_exact(_to_usize(g1, 40));
        }

        let r1 = sv1.as_ref();
        if let Some(v) = r1.get(0) {
            println!("{}", *v);
        }

        let ops = (_to_u8(g1, 21) % 7) as usize + 1;
        for j in 0..ops {
            match _to_u8(g1, 22 + j) % 8 {
                0 => { sv1.push(_to_u8(g2, j)); }
                1 => { let _ = sv1.pop(); }
                2 => { sv1.insert(_to_usize(g2, j), _to_u8(g1, j)); }
                3 => { let _ = sv1.remove(_to_usize(g2, j)); }
                4 => { sv1.truncate(_to_usize(g1, j)); }
                5 => { let _ = sv1.swap_remove(_to_usize(g1, j)); }
                6 => { sv1.extend_from_slice(slice_u8); }
                _ => { sv1.insert_from_slice(_to_usize(g1, j), slice_u8); }
            }
            let rr = sv1.as_ref();
            if let Some(v) = rr.get(0) { println!("{}", *v); }
        }

        let a1 = sv1.as_slice();
        if let Some(v) = a1.get(0) { println!("{}", *v); }
        let d1 = sv1.deref();
        if let Some(v) = d1.get(0) { println!("{}", *v); }
        let b1: &[u8] = sv1.borrow();
        if let Some(v) = b1.get(0) { println!("{}", *v); }

        let mut flip = _to_u8(g1, 22);
        let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            sv1.retain(|x| {
                flip ^= 1;
                if flip % 2 == 0 { panic!("INTENTIONAL PANIC!"); }
                *x % 2 == flip % 2
            });
        }));

        let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            sv1.dedup_by(|a, b| {
                if _to_u8(g2, 10) % 2 == 0 { panic!("INTENTIONAL PANIC!"); }
                *a == *b
            });
        }));

        {
            let rs = _to_usize(g1, 24);
            let re = _to_usize(g1, 32);
            let mut dr = sv1.drain(rs..re);
            let _ = dr.next();
            let _ = dr.next_back();
        }

        let mut arr_i16_16: [i16; 16] = [0; 16];
        let g2_i16_bound = g2.len().saturating_sub(2).max(1);
        for i in 0..16 {
            arr_i16_16[i] = _to_i16(g2, (i * 2) % g2_i16_bound);
        }

        let vec_len_i16 = (_to_u8(g2, 20) % 65) as usize;
        let mut vec_i16 = Vec::with_capacity(vec_len_i16);
        let g1_i16_bound = g1.len().saturating_sub(2).max(1);
        for i in 0..vec_len_i16 {
            vec_i16.push(_to_i16(g1, (2 * i) % g1_i16_bound));
        }

        let mut sv2: SmallVec<[i16; 16]> = match _to_u8(g2, 22) % 5 {
            0 => SmallVec::<[i16; 16]>::new(),
            1 => SmallVec::<[i16; 16]>::with_capacity(_to_usize(g2, 23)),
            2 => SmallVec::from_buf(arr_i16_16),
            3 => SmallVec::from_vec(vec_i16.clone()),
            _ => SmallVec::from_elem(_to_i16(g1, 0), _to_usize(g2, 30)),
        };

        let r2 = sv2.as_ref();
        if r2.len() > 0 { println!("{}", r2[0]); }
        println!("{} {}", sv2.capacity(), sv2.len());
        let sv2b = SmallVec::<[i16; 16]>::from_slice(&[1i16, 2, 3, 4]);
        let _ = sv2.eq(&sv2b);
        let _ = sv2.cmp(&sv2b);
        let _ = sv2.partial_cmp(&sv2b);

        sv2.push(_to_i16(g2, 40));
        if sv2.len() > 1 {
            let r = &sv2[0..1];
            let v = r[0];
            println!("{}", v);
        }

        let iter = sv2.clone().into_iter();
        let srem = iter.as_slice();
        if srem.len() > 0 { println!("{}", srem[0]); }

        let mut other = SmallVec::<[u8; 32]>::from_elem(_to_u8(g2, 50), (_to_u8(g2, 51) % 65) as usize);
        sv1.append(&mut other);
        let rf = sv1.as_ref();
        if let Some(x) = rf.get(0) { println!("{}", *x); }

        let ts: SmallVec<[u8; 32]> = slice_u8.to_smallvec();
        let rr = ts.as_ref();
        if let Some(x) = rr.get(0) { println!("{}", *x); }
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