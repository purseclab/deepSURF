#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct CustomType1(usize);

impl core::marker::Copy for CustomType1 {}

impl core::clone::Clone for CustomType1 {
    fn clone(&self) -> Self {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 43);
        let custom_impl_inst_num = self.0;
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0{
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector{
                1 => global_data.first_half,
                _ => global_data.second_half,
        };
        let t_12 = _to_usize(GLOBAL_DATA, 51);
        let t_13 = CustomType1(t_12);
        return t_13;
    }
}

fn build_vec_from_half(half: &[u8], ctrl_idx: usize, base: usize) -> Vec<CustomType1> {
    let mut v = std::vec::Vec::with_capacity(64);
    let mut len = _to_u8(half, ctrl_idx) as usize % 65;
    let max_items = if half.len() > base + 8 { (half.len() - base - 8) / 8 } else { 0 };
    if len > max_items { len = max_items; }
    for j in 0..len {
        let idx = base + j * 8;
        let val = _to_usize(half, idx);
        v.push(CustomType1(val));
    }
    v
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 520 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let g1 = global_data.first_half;
        let g2 = global_data.second_half;

        let mut vec1 = build_vec_from_half(g1, 2, 16);
        let vec1_clone = vec1.clone();
        let mut vec2 = build_vec_from_half(g2, 3, 24);

        let ctrl0 = _to_u8(g1, 0);
        let mut sv: smallvec::SmallVec<[CustomType1; 64]> = match ctrl0 % 5 {
            0 => smallvec::SmallVec::new(),
            1 => smallvec::SmallVec::with_capacity(_to_usize(g1, 8)),
            2 => smallvec::SmallVec::from_slice(&vec1[..]),
            3 => smallvec::SmallVec::from_vec(vec1_clone),
            _ => smallvec::SmallVec::from_elem(CustomType1(_to_usize(g1, 24)), _to_usize(g1, 32)),
        };

        let _ = sv.try_reserve(_to_usize(g1, 40));
        sv.reserve(_to_usize(g1, 48));
        sv.push(CustomType1(_to_usize(g1, 56)));
        let idx_insert = _to_usize(g1, 64);
        sv.insert(idx_insert, CustomType1(_to_usize(g1, 72)));

        {
            let sref0 = sv.as_slice();
            if !sref0.is_empty() {
                let fref = &sref0[0];
                println!("{:?}", *fref);
            }
        }

        let take_vec: Vec<CustomType1> = {
            let sref = sv.as_slice();
            let take = if sref.is_empty() { 0 } else { (_to_u8(g1, 96) as usize) % std::cmp::min(sref.len(), 65) };
            sref[..take].to_vec()
        };

        {
            let mref = sv.as_mut_slice();
            if !mref.is_empty() {
                let r = &mut mref[0];
                println!("{:?}", *r);
                *r = CustomType1(_to_usize(g1, 80));
            }
        }

        let slice_b = &vec2[..];
        sv.insert_from_slice(_to_usize(g1, 88), slice_b);

        sv.extend_from_slice(&take_vec[..]);

        sv.resize(_to_usize(g1, 104), CustomType1(_to_usize(g1, 112)));
        sv.dedup();
        sv.dedup_by(|a, b| {
            let gd = get_global_data();
            let src = if _to_bool(gd.first_half, 120) { gd.first_half } else { gd.second_half };
            let val = _to_u8(src, 121);
            (a.0 ^ b.0) % (val as usize + 1) == 0
        });
        sv.retain(|x| {
            let gd = get_global_data();
            let n = _to_usize(gd.first_half, 128);
            x.0 % (n.wrapping_add(1)) != 0
        });

        if sv.len() > 0 {
            let _ = sv.remove(_to_usize(g1, 136));
        }
        if sv.len() > 0 {
            let _ = sv.swap_remove(_to_usize(g1, 140));
        }

        let end = _to_usize(g1, 144);
        let mut dr = sv.drain(0..end);
        let _ = dr.next();
        let _ = dr.next_back();
        drop(dr);

        let vec3 = build_vec_from_half(g1, 4, 160);
        sv.insert_from_slice(_to_usize(g1, 152), &vec3[..]);

        sv.truncate(_to_usize(g1, 168));
        sv.shrink_to_fit();

        let sv2 = sv.clone();
        let _ = smallvec::SmallVec::partial_cmp(&sv, &sv2);
        let _ = smallvec::SmallVec::cmp(&sv, &sv2);
        let _ = smallvec::SmallVec::eq(&sv, &sv2);

        let mut other = smallvec::SmallVec::<[CustomType1; 64]>::from_slice(&vec2[..]);
        sv.append(&mut other);

        if sv.len() > 0 {
            let i = _to_usize(g1, 176);
            let r1 = &sv[i];
            println!("{:?}", *r1);
        }
        if sv.len() > 0 {
            let i2 = _to_usize(g1, 184);
            let r2 = &mut sv[i2];
            println!("{:?}", *r2);
            *r2 = CustomType1(_to_usize(g1, 192));
        }

        let mut it = sv.clone().into_iter();
        let slice_iter = it.as_slice();
        if !slice_iter.is_empty() {
            let r = &slice_iter[0];
            println!("{:?}", *r);
        }
        let _ = it.next();
        let _ = it.next_back();
        let ms = it.as_mut_slice();
        if !ms.is_empty() {
            ms[0] = CustomType1(_to_usize(g2, 60));
        }

        let ops = (_to_u8(g2, 0) % 7) as usize;
        for i in 0..ops {
            let choice = _to_u8(g2, 1 + i) % 6;
            match choice {
                0 => {
                    sv.insert_from_slice(_to_usize(g2, 8 + i), &vec2[..]);
                }
                1 => {
                    sv.extend_from_slice(&vec1[..]);
                }
                2 => {
                    sv.push(CustomType1(_to_usize(g2, 16 + i)));
                }
                3 => {
                    let _ = sv.pop();
                }
                4 => {
                    sv.insert(_to_usize(g2, 24 + i), CustomType1(_to_usize(g2, 32 + i)));
                }
                _ => {
                    let _ = sv.try_reserve(_to_usize(g2, 40 + i));
                }
            }
        }

        let d = sv.deref();
        if !d.is_empty() {
            println!("{:?}", d[0]);
        }
        let dm = sv.deref_mut();
        if !dm.is_empty() {
            dm[0] = CustomType1(_to_usize(g2, 72));
        }
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