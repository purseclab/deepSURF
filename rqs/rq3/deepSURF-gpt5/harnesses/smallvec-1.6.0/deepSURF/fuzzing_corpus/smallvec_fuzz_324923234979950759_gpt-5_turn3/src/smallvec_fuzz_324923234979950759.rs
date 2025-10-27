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
        let custom_impl_num = _to_usize(GLOBAL_DATA, 35);
        let custom_impl_inst_num = self.0;
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0 {
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector {
            1 => global_data.first_half,
            _ => global_data.second_half,
        };
        let t_11 = _to_usize(GLOBAL_DATA, 43);
        let t_12 = CustomType1(t_11);
        t_12
    }
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 800 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let mut vec_data = std::vec::Vec::with_capacity(32);
        for i in 0..32usize {
            let idx = 100 + i * 8;
            vec_data.push(CustomType1(_to_usize(GLOBAL_DATA, idx)));
        }
        let vlen = (_to_u8(GLOBAL_DATA, 96) % 65) as usize;
        if vec_data.len() > vlen { vec_data.truncate(vlen); }
        let slice_ref = &vec_data[..];
        let base_item = CustomType1(_to_usize(GLOBAL_DATA, 8));
        let buf_array: [CustomType1; 16] = [base_item; 16];
        let mut v: SmallVec<[CustomType1; 16]> = match _to_u8(GLOBAL_DATA, 2) % 6 {
            0 => SmallVec::<[CustomType1; 16]>::new(),
            1 => SmallVec::<[CustomType1; 16]>::with_capacity(_to_usize(GLOBAL_DATA, 408)),
            2 => SmallVec::<[CustomType1; 16]>::from_buf(buf_array),
            3 => SmallVec::<[CustomType1; 16]>::from_buf_and_len(buf_array, _to_usize(GLOBAL_DATA, 416)),
            4 => SmallVec::<[CustomType1; 16]>::from_elem(CustomType1(_to_usize(GLOBAL_DATA, 424)), _to_usize(GLOBAL_DATA, 432)),
            _ => SmallVec::<[CustomType1; 16]>::from_slice(slice_ref),
        };
        let ops = (_to_u8(GLOBAL_DATA, 3) % 20) as usize + 1;
        for i in 0..ops {
            let opb = _to_u8(GLOBAL_DATA, 440 + i);
            match opb % 12 {
                0 => {
                    v.extend_from_slice(slice_ref);
                }
                1 => {
                    v.resize(_to_usize(GLOBAL_DATA, 460 + i * 12), CustomType1(_to_usize(GLOBAL_DATA, 468 + i * 12)));
                }
                2 => {
                    v.insert_from_slice(_to_usize(GLOBAL_DATA, 476 + i * 12), slice_ref);
                }
                3 => {
                    v.retain(|x| { println!("{:?}", *x); (x.0 % 2) == (_to_usize(GLOBAL_DATA, 484 + i * 12) % 2) });
                }
                4 => {
                    v.dedup_by(|a, b| {
                        let t = _to_u8(GLOBAL_DATA, 492 + i * 12);
                        println!("{:?}", *a);
                        println!("{:?}", *b);
                        if t % 2 == 0 { a.0 == b.0 } else { a.0.wrapping_add(b.0) % 3 == 0 }
                    });
                }
                5 => {
                    v.truncate(_to_usize(GLOBAL_DATA, 500 + i * 12));
                }
                6 => {
                    if let Some(x) = v.pop() { println!("{:?}", x); }
                }
                7 => {
                    let start = _to_usize(GLOBAL_DATA, 508 + i * 12);
                    let end = _to_usize(GLOBAL_DATA, 516 + i * 12);
                    let mut dr = v.drain(start..end);
                    if let Some(x) = dr.next() { println!("{:?}", x); }
                    if let Some(x) = dr.next_back() { println!("{:?}", x); }
                }
                8 => {
                    let mut w = SmallVec::<[CustomType1; 16]>::from_slice(slice_ref);
                    v.append(&mut w);
                }
                9 => {
                    let s = v.as_slice();
                    if !s.is_empty() { println!("{:?}", s[0]); }
                }
                10 => {
                    let tmp = v.clone();
                    let owned = tmp.into_vec();
                    let sr = &owned[..];
                    v.extend_from_slice(sr);
                }
                _ => {
                    v[_to_usize(GLOBAL_DATA, 524 + i * 12)] = CustomType1(_to_usize(GLOBAL_DATA, 532 + i * 12));
                }
            }
        }
        let _ = v.try_reserve(_to_usize(GLOBAL_DATA, 540)).ok();
        let _ = v.try_reserve_exact(_to_usize(GLOBAL_DATA, 548)).ok();
        v.reserve(_to_usize(GLOBAL_DATA, 556));
        v.reserve_exact(_to_usize(GLOBAL_DATA, 564));
        let small_from_trait: SmallVec<[CustomType1; 16]>;
        {
            let sref2 = v.as_ref();
            if !sref2.is_empty() { println!("{:?}", sref2[0]); }
            small_from_trait = sref2.to_smallvec();
        }
        let mref = v.as_mut();
        if !mref.is_empty() {
            println!("{:?}", mref[0]);
            mref[0] = CustomType1(_to_usize(GLOBAL_DATA, 572));
        }
        let v_clone = v.clone();
        let _ = v.eq(&v_clone);
        let _ = v.partial_cmp(&v_clone);
        let _ = v.cmp(&v_clone);
        let mut iter = v.clone().into_iter();
        let rs = iter.as_slice();
        if !rs.is_empty() { println!("{:?}", rs[0]); }
        let rms = iter.as_mut_slice();
        if !rms.is_empty() {
            rms[0] = CustomType1(_to_usize(GLOBAL_DATA, 580));
            println!("{:?}", rms[0]);
        }
        let _ = iter.next();
        let _ = iter.next_back();
        let cap = v.capacity();
        let len = v.len();
        let _ = (cap, len);
        v.extend_from_slice(&small_from_trait[..]);
        let _ = v.as_ptr();
        let start2 = _to_usize(GLOBAL_DATA, 588);
        let end2 = _to_usize(GLOBAL_DATA, 596);
        {
            let mut dr2 = v.drain(start2..end2);
            let _ = dr2.next();
            let _ = dr2.next_back();
        }
        v.shrink_to_fit();
        let s = v.as_slice();
        if !s.is_empty() {
            let i0 = 0usize;
            println!("{:?}", s[i0]);
        }
        println!("{}", v.is_empty());
        v.grow(_to_usize(GLOBAL_DATA, 612));
        v.try_grow(_to_usize(GLOBAL_DATA, 620)).ok();
        let mut other = SmallVec::<[CustomType1; 16]>::from_slice(slice_ref);
        v.append(&mut other);
        if v.len() > 0 {
            let _ = &v[_to_usize(GLOBAL_DATA, 628)];
        }
        let mut v2 = SmallVec::<[CustomType1; 16]>::from_iter(slice_ref.iter().cloned());
        v2.extend_from_slice(slice_ref);
        let _ = v2.into_boxed_slice();
        let _ = SmallVec::<[CustomType1; 16]>::from_vec(slice_ref.to_vec()).into_inner();
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