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
        if data.len() < 180 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let fh = global_data.first_half;
        let sh = global_data.second_half;

        let mut arr1 = [0u8; 32];
        arr1.copy_from_slice(&sh[0..32]);
        let mut arr2 = [0u8; 32];
        arr2.copy_from_slice(&sh[32..64]);

        let constructor_selector = _to_u8(fh, 1) % 6;
        let mut sv: SmallVec<[u8; 32]> = match constructor_selector {
            0 => SmallVec::new(),
            1 => SmallVec::with_capacity(_to_usize(fh, 2)),
            2 => SmallVec::from_buf(arr1),
            3 => SmallVec::from_buf_and_len(arr2, _to_usize(fh, 10)),
            4 => {
                let len_vec = (_to_u8(fh, 20) % 65) as usize;
                SmallVec::from_vec(sh[0..len_vec].to_vec())
            }
            _ => {
                let n = (_to_u8(fh, 30) % 65) as usize;
                let end = 10 + n;
                let end_b = if end <= sh.len() { end } else { sh.len() };
                SmallVec::from_slice(&sh[10..end_b])
            }
        };

        sv.push(_to_u8(fh, 40));
        sv.reserve(_to_usize(fh, 44));
        let _ = sv.try_reserve(_to_usize(fh, 48));
        let _ = sv.try_reserve_exact(_to_usize(fh, 52));
        sv.insert(_to_usize(fh, 60), _to_u8(fh, 62));

        let ext_len = (_to_u8(fh, 63) % 65) as usize;
        sv.extend_from_slice(&sh[0..ext_len]);

        sv.dedup();
        sv.resize_with(_to_usize(fh, 64), || _to_u8(sh, 0));
        sv.retain(|x| {
            let b = _to_bool(fh, 65);
            if b { *x = x.wrapping_add(1); }
            b
        });
        sv.dedup_by(|a, b| {
            let flip = _to_u8(sh, 1) % 2 == 0;
            if flip { *a = *b; }
            flip
        });
        sv.dedup_by_key(|x| { let k = *x; k });

        let s0 = sv.as_slice();
        println!("{:?}", s0);

        let cap = sv.capacity();
        let empty = sv.is_empty();
        println!("{} {}", cap, empty);

        let msl = sv.as_mut_slice();
        if !msl.is_empty() { msl[0] = msl[0].wrapping_add(_to_u8(sh, 2)); }
        println!("{:?}", msl);

        let b1: &[u8] = sv.borrow();
        println!("{:?}", b1);

        let d1 = sv.deref();
        println!("{}", d1.len());

        if sv.len() > 0 {
            let first_ref = &sv[0];
            println!("{}", *first_ref);
            let first_mut = &mut sv[0];
            *first_mut = first_mut.wrapping_add(1);
        }

        let other_len = (_to_u8(sh, 3) % 65) as usize;
        let other = SmallVec::<[u8; 32]>::from_slice(&sh[0..other_len]);
        let _ = SmallVec::partial_cmp(&sv, &other);
        let _ = SmallVec::cmp(&sv, &other);

        sv.truncate(_to_usize(sh, 4));
        let _ = sv.pop();
        let _ = sv.remove(_to_usize(sh, 12));
        let _ = sv.swap_remove(_to_usize(sh, 16));

        let ins_len = (_to_u8(sh, 24) % 65) as usize;
        sv.insert_from_slice(_to_usize(sh, 20), &sh[0..ins_len]);

        let s1 = sv.as_slice();
        println!("{:?}", s1);

        let mut sv2 = sv.clone();
        let v2 = SmallVec::into_vec(sv2.clone());
        let _sv3 = SmallVec::<[u8; 32]>::from_vec(v2);

        let mut iter = SmallVec::into_iter(sv2);
        let si = iter.as_slice();
        println!("{:?}", si);
        if let Some(m) = iter.next() { println!("{}", m); }
        if let Some(mb) = iter.next_back() { println!("{}", mb); }
        let sm = iter.as_mut_slice();
        if !sm.is_empty() { sm[0] = sm[0].wrapping_add(1); }
        println!("{:?}", sm);

        let range_end = _to_usize(sh, 28);
        let mut dr = sv.drain(0..range_end);
        let _ = dr.next();
        let _ = dr.next_back();
        drop(dr);

        let s2 = sv.as_slice();
        println!("{:?}", s2);

        let op_count = (_to_u8(fh, 70) % 5) as usize + 1;
        for i in 0..op_count {
            match _to_u8(fh, 71 + (i % 4)) % 5 {
                0 => sv.push(_to_u8(sh, 32 + i)),
                1 => { let _ = sv.pop(); }
                2 => sv.reserve(_to_usize(sh, 36 + i)),
                3 => sv.truncate(_to_usize(fh, 72 + i)),
                _ => {
                    let sr = sv.as_slice();
                    println!("{}", sr.len());
                }
            }
        }

        let s3 = sv.as_slice();
        println!("{:?}", s3);
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