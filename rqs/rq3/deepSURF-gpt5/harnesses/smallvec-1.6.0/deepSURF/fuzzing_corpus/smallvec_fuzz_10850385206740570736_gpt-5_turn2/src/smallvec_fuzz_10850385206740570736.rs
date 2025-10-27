#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 160 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let GLOBAL_DATA2 = global_data.second_half;

        let len_a = (_to_u8(GLOBAL_DATA, 0) % 65) as usize;
        let len_b = (_to_u8(GLOBAL_DATA, 1) % 65) as usize;
        let len_c = (_to_u8(GLOBAL_DATA, 2) % 65) as usize;

        let end_a = std::cmp::min(3 + len_a, GLOBAL_DATA.len());
        let s_a = &GLOBAL_DATA[3..end_a];
        let end_b = std::cmp::min(end_a + len_b, GLOBAL_DATA.len());
        let s_b = &GLOBAL_DATA[end_a..end_b];

        let mut sv_u8 = SmallVec::<[u8; 32]>::from_slice(s_a);
        let mut sv_u8_b = SmallVec::<[u8; 16]>::from_elem(_to_u8(GLOBAL_DATA, 5), len_b);

        let cap_i32 = _to_usize(GLOBAL_DATA, 10);
        let mut sv_i32 = SmallVec::<[i32; 16]>::with_capacity(cap_i32);
        let push_rep = (_to_u8(GLOBAL_DATA, 12) % 65) as usize;
        let val_i32 = _to_i32(GLOBAL_DATA, 14);
        for _ in 0..push_rep {
            sv_i32.push(val_i32);
        }

        let m1 = (_to_u8(GLOBAL_DATA2, 0) % 65) as usize;
        let mut vec_i32 = Vec::with_capacity(m1);
        let base_i32 = _to_i32(GLOBAL_DATA2, 2);
        for _ in 0..m1 {
            vec_i32.push(base_i32);
        }
        let mut sv_i32_from_vec = SmallVec::<[i32; 8]>::from_vec(vec_i32);

        let mut arr32 = [0u8; 32];
        let fill_len = std::cmp::min(32, GLOBAL_DATA.len().saturating_sub(4));
        for i in 0..fill_len {
            arr32[i] = GLOBAL_DATA[4 + i];
        }
        let len_buf = _to_usize(GLOBAL_DATA, 22);
        let tag = _to_u8(GLOBAL_DATA, 30);
        let mut sv_u8_alt: SmallVec<[u8; 32]> = match tag % 4 {
            0 => SmallVec::<[u8; 32]>::from_slice(s_b),
            1 => SmallVec::<[u8; 32]>::from_elem(_to_u8(GLOBAL_DATA2, 3), len_c),
            2 => SmallVec::<[u8; 32]>::from_buf_and_len(arr32, len_buf),
            _ => SmallVec::<[u8; 32]>::with_capacity(_to_usize(GLOBAL_DATA2, 4)),
        };

        sv_u8.reserve(_to_usize(GLOBAL_DATA2, 6));
        sv_u8.try_reserve(_to_usize(GLOBAL_DATA2, 7)).ok();
        sv_u8.extend_from_slice(s_b);
        sv_u8.insert(_to_usize(GLOBAL_DATA2, 8), _to_u8(GLOBAL_DATA2, 9));
        let _ = sv_u8.pop();
        let _ = sv_u8.remove(_to_usize(GLOBAL_DATA2, 10));
        let _ = sv_u8.swap_remove(_to_usize(GLOBAL_DATA2, 11));
        sv_u8.insert_from_slice(_to_usize(GLOBAL_DATA2, 12), s_a);
        sv_u8.shrink_to_fit();
        sv_u8.dedup();
        sv_u8.dedup_by(|a, b| {
            let _ = *a;
            let _ = *b;
            _to_bool(GLOBAL_DATA2, 13)
        });
        sv_u8.dedup_by_key(|x| { *x });
        sv_u8.retain(|v| {
            let keep = _to_bool(GLOBAL_DATA2, 14);
            if !keep {
                *v = v.wrapping_add(1);
            }
            keep
        });
        sv_u8.resize_with((_to_usize(GLOBAL_DATA2, 15) % 65) as usize, || _to_u8(GLOBAL_DATA2, 16));
        sv_u8.truncate(_to_usize(GLOBAL_DATA2, 17));

        let _ = sv_u8.len();
        let _ = sv_u8.capacity();
        let _ = sv_u8.is_empty();
        let s_ref = sv_u8.as_slice();
        println!("{:?}", s_ref);
        if !sv_u8.is_empty() {
            let r0 = &sv_u8[0];
            println!("{}", *r0 as u8);
        }
        let ms_ref = sv_u8.as_mut_slice();
        if !ms_ref.is_empty() {
            ms_ref[0] = ms_ref[0].wrapping_add(1);
        }
        let d_ref = sv_u8.deref();
        println!("{:?}", d_ref);
        if sv_u8.len() > 1 {
            let e = sv_u8.index(1);
            println!("{}", *e);
            let e_mut = sv_u8.index_mut(1);
            *e_mut = e_mut.wrapping_add(1);
        }

        let mut iter_mut = (&mut sv_u8).into_iter();
        let it_ops = (_to_u8(GLOBAL_DATA2, 18) % 10) as usize;
        for i in 0..it_ops {
            if i % 2 == 0 {
                if let Some(r) = iter_mut.next() {
                    let addv = _to_u8(GLOBAL_DATA2, 19);
                    *r = (*r).wrapping_add(addv);
                }
            } else {
                if let Some(r) = iter_mut.next_back() {
                    let subv = _to_u8(GLOBAL_DATA2, 20);
                    *r = (*r).wrapping_sub(subv);
                    println!("{}", *r);
                }
            }
        }

        for r in (&mut sv_i32).into_iter() {
            let addi = _to_i32(GLOBAL_DATA2, 21);
            *r = (*r).wrapping_add(addi);
        }

        let mut iter_alt = (&mut sv_u8_alt).into_iter();
        let _ = iter_alt.next();
        let _ = iter_alt.next_back();

        let sv_cmp = SmallVec::<[u8; 32]>::from_slice(s_b);
        let _ = sv_u8.cmp(&sv_cmp);
        let _ = sv_u8.partial_cmp(&sv_cmp);
        let _ = sv_u8.eq(&sv_cmp);

        {
            let mut dr = sv_u8.drain(0.._to_usize(GLOBAL_DATA2, 22));
            let _ = dr.next();
            let _ = dr.next_back();
        }

        let _ = sv_i32_from_vec.clone().into_iter().next();
        let _ = sv_i32_from_vec.into_vec();

        sv_u8_b.append(&mut sv_u8_alt);
        sv_u8.extend(sv_u8_b.clone());
        let _ = sv_u8.try_reserve_exact(_to_usize(GLOBAL_DATA2, 23));
        sv_u8.reserve_exact(_to_usize(GLOBAL_DATA2, 24));

        let _ = (&mut sv_u8).into_iter().next();
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