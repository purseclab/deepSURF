#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 132 { return; }
        set_global_data(data);
        let gd = get_global_data();
        let first = gd.first_half;
        let second = gd.second_half;
        let tag = _to_u8(first, 0);
        let mut sv: SmallVec<[u8; 32]> = match tag % 6 {
            0 => {
                let mut v = SmallVec::<[u8; 32]>::new();
                v.push(_to_u8(first, 1));
                v.push(_to_u8(first, 2));
                v.push(_to_u8(first, 3));
                v
            }
            1 => {
                let mut v = SmallVec::<[u8; 32]>::with_capacity(_to_usize(first, 8));
                v.push(_to_u8(first, 4));
                v.push(_to_u8(first, 5));
                v.push(_to_u8(first, 6));
                v
            }
            2 => {
                let n = (_to_u8(first, 7) % 65) as usize;
                let slice_len = if n <= second.len() { n } else { second.len() };
                SmallVec::<[u8; 32]>::from_slice(&second[..slice_len])
            }
            3 => {
                let m = (_to_u8(first, 9) % 65) as usize;
                let mut vv = Vec::with_capacity(m);
                for i in 0..m {
                    vv.push(second[i % second.len()]);
                }
                SmallVec::<[u8; 32]>::from_vec(vv)
            }
            4 => {
                let mut arr = [0u8; 32];
                for i in 0..32 {
                    arr[i] = first[(1 + i) % first.len()];
                }
                SmallVec::<[u8; 32]>::from_buf(arr)
            }
            _ => {
                let mut arr = [0u8; 32];
                for i in 0..32 {
                    arr[i] = second[i % second.len()];
                }
                SmallVec::<[u8; 32]>::from_buf_and_len(arr, _to_usize(first, 16))
            }
        };
        let _ = sv.capacity();
        let _ = sv.len();
        let _ = sv.is_empty();
        let s_ref = sv.as_ref();
        println!("{:?}", s_ref);
        let s_slice = sv.as_slice();
        println!("{:?}", s_slice);
        {
            let msl = sv.as_mut_slice();
            if !msl.is_empty() {
                msl[0] = msl[0].wrapping_add(_to_u8(first, 17));
                println!("{:?}", &msl[0]);
            }
        }
        let _ = sv.deref();
        let _ = sv.deref_mut();
        let _ = sv.try_grow(_to_usize(first, 24));
        let _ = sv.try_reserve(_to_usize(first, 32));
        sv.reserve_exact(_to_usize(first, 40));
        sv.grow(_to_usize(first, 48));
        sv.truncate(_to_usize(first, 56));
        sv.push(_to_u8(first, 64));
        let _ = sv.remove(_to_usize(first, 23));
        let _ = sv.swap_remove(_to_usize(first, 22));
        sv.insert(_to_usize(first, 20), _to_u8(first, 21));
        let ins_len = (_to_u8(second, 2) % 65) as usize;
        let ins_slice_len = if ins_len <= second.len() { ins_len } else { second.len() };
        sv.insert_from_slice(_to_usize(first, 26), &second[..ins_slice_len]);
        let ext_len = (_to_u8(second, 3) % 65) as usize;
        let ext_slice_len = if ext_len <= first.len() { ext_len } else { first.len() };
        sv.extend_from_slice(&first[..ext_slice_len]);
        sv.retain(|x| {
            *x = x.wrapping_add(_to_u8(first, 27));
            true
        });
        sv.dedup_by(|a, b| {
            if _to_bool(first, 28) {
                *a == *b
            } else {
                false
            }
        });
        sv.dedup_by_key(|z| z.wrapping_add(_to_u8(first, 29)));
        let _ = sv.len();
        let cl = sv.clone();
        let _ = sv.partial_cmp(&cl);
        let _ = sv.cmp(&cl);
        let _ = sv.eq(&cl);
        {
            let mut dr = sv.drain(0.._to_usize(first, 34));
            let _ = dr.next();
            let _ = dr.next_back();
        }
        let app_len = (_to_u8(second, 5) % 65) as usize;
        let app_slice_len = if app_len <= second.len() { app_len } else { second.len() };
        let mut other = SmallVec::<[u8; 32]>::from_slice(&second[..app_slice_len]);
        sv.append(&mut other);
        if !sv.is_empty() {
            let x = &sv[0];
            println!("{:?}", x);
        }
        if !sv.is_empty() {
            sv[0] = sv[0].wrapping_add(_to_u8(first, 33));
            println!("{}", sv[0]);
        }
        let mut it = sv.clone().into_iter();
        let rem = it.as_slice();
        println!("{:?}", rem);
        {
            let rem_mut = it.as_mut_slice();
            if !rem_mut.is_empty() {
                rem_mut[0] = rem_mut[0].wrapping_add(_to_u8(first, 35));
                println!("{:?}", &rem_mut[0]);
            }
        }
        let loopc = (_to_u8(second, 1) % 10) as usize;
        for _ in 0..loopc {
            let _ = it.next();
            let _ = it.next_back();
        }
        let mut it2 = it.clone();
        let _ = it2.next();
        let _ = format!("{:?}", it2);
        let _ = (&sv).into_iter();
        let _ = (&mut sv).into_iter();
        let op_count = (_to_u8(second, 7) % 8) as usize;
        for i in 0..op_count {
            match second[(8 + i) % second.len()] % 10 {
                0 => {
                    sv.push(_to_u8(first, 36));
                }
                1 => {
                    let _ = sv.pop();
                }
                2 => {
                    sv.reserve(_to_usize(second, 8));
                }
                3 => {
                    let m = (_to_u8(first, 37) % 65) as usize;
                    let mut vv = Vec::with_capacity(m);
                    for j in 0..m {
                        vv.push(first[j % first.len()]);
                    }
                    sv.extend(vv);
                }
                4 => {
                    sv.truncate(_to_usize(second, 16));
                }
                5 => {
                    let _ = sv.try_reserve_exact(_to_usize(second, 24));
                }
                6 => {
                    sv.shrink_to_fit();
                }
                7 => {
                    let ilen = (_to_u8(first, 38) % 65) as usize;
                    let sl = if ilen <= second.len() { ilen } else { second.len() };
                    sv.insert_many(_to_usize(second, 32), second[..sl].to_vec());
                }
                8 => {
                    let _ = sv.into_inner();
                    break;
                }
                _ => {
                    sv.clear();
                }
            }
        }
        let vv_len = (_to_u8(first, 39) % 65) as usize;
        let mut vec_b = Vec::with_capacity(vv_len);
        for i in 0..vv_len {
            vec_b.push(second[i % second.len()]);
        }
        let mut sv_b = SmallVec::<[u8; 32]>::from_vec(vec_b);
        let out_v = sv_b.into_vec();
        println!("{}", out_v.len());
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