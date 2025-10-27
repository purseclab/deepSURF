#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct CustomType(String);

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 400 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let g1 = global_data.first_half;
        let g2 = global_data.second_half;

        let mut v: Vec<CustomType> = Vec::with_capacity(32);
        let vlen = _to_u8(g2, 0) % 65;
        for i in 0..vlen {
            let s_idx = (i as usize * 3) % g1.len();
            let l = (_to_u8(g1, (i as usize * 5) % g1.len()) % 17) as usize;
            let start = s_idx;
            let end = std::cmp::min(g1.len(), start.saturating_add(std::cmp::max(1, l)));
            let s = _to_str(g1, start, end);
            let st = String::from(s);
            v.push(CustomType(st));
        }

        let s_start = (1usize) % g1.len();
        let s_len = (_to_u8(g1, 2) % 17) as usize;
        let s_end = std::cmp::min(g1.len(), s_start.saturating_add(std::cmp::max(1, s_len)));
        let s0 = _to_str(g1, s_start, s_end);
        let elem0 = CustomType(String::from(s0));
        let arr16: [CustomType; 16] = [elem0.clone(), elem0.clone(), elem0.clone(), elem0.clone(),
                                       elem0.clone(), elem0.clone(), elem0.clone(), elem0.clone(),
                                       elem0.clone(), elem0.clone(), elem0.clone(), elem0.clone(),
                                       elem0.clone(), elem0.clone(), elem0.clone(), elem0.clone()];

        let slice_from_vec_count = if v.is_empty() { 0 } else { (vlen as usize).min(v.len()) };
        let slice_vec: Vec<CustomType> = v.iter().take(slice_from_vec_count).cloned().collect();
        let slice_items: &[CustomType] = &slice_vec[..];

        let sel0 = _to_u8(g2, 2);
        let num_a = _to_usize(g2, 8);
        let num_b = _to_usize(g2, 16);
        let num_c = _to_usize(g2, 24);
        let num_d = _to_usize(g2, 32);
        let num_e = _to_usize(g2, 40);
        let num_f = _to_usize(g2, 48);

        let mut sv: SmallVec<[CustomType; 16]> = match sel0 % 9 {
            0 => SmallVec::<[CustomType; 16]>::new(),
            1 => SmallVec::<[CustomType; 16]>::with_capacity(num_a),
            2 => SmallVec::<[CustomType; 16]>::from_elem(elem0.clone(), num_b),
            3 => SmallVec::<[CustomType; 16]>::from(slice_items),
            4 => SmallVec::<[CustomType; 16]>::from_vec(v.clone()),
            5 => SmallVec::<[CustomType; 16]>::from_buf(arr16.clone()),
            6 => SmallVec::<[CustomType; 16]>::from_buf_and_len(arr16.clone(), num_c),
            7 => SmallVec::<[CustomType; 16]>::from(slice_items),
            _ => SmallVec::<[CustomType; 16]>::from(slice_items),
        };

        sv.push(elem0.clone());
        if !arr16.is_empty() {
            let sl = &arr16[..std::cmp::min(4usize, arr16.len())];
            sv.extend(sl.iter().cloned());
        }
        let rs = sv.as_slice();
        if let Some(r) = rs.get(0) {
            println!("{:?}", r);
        }

        let s_from_vec1: SmallVec<[CustomType; 16]> = SmallVec::from(v.clone());

        let ops = (_to_u8(g2, 3) % 20) as usize;
        for i in 0..ops {
            let code = _to_u8(g2, 4 + i) % 12;
            match code {
                0 => {
                    let si = (i * 7) % g1.len();
                    let ll = (_to_u8(g1, (i * 11) % g1.len()) % 17) as usize;
                    let e = std::cmp::min(g1.len(), si.saturating_add(std::cmp::max(1, ll)));
                    let s = _to_str(g1, si, e);
                    sv.push(CustomType(String::from(s)));
                }
                1 => {
                    let idx = num_b.wrapping_add(i);
                    let _ = sv.remove(idx);
                }
                2 => {
                    let idx = num_c.wrapping_add(i);
                    sv.insert(idx, elem0.clone());
                }
                3 => {
                    let idx = num_d.wrapping_add(i);
                    let _ = sv.swap_remove(idx);
                }
                4 => {
                    let len = num_e.wrapping_add(i);
                    sv.truncate(len);
                }
                5 => {
                    sv.reserve(num_f.wrapping_add(i));
                }
                6 => {
                    let _ = _unwrap_result(sv.try_reserve_exact(num_a.wrapping_add(i)));
                }
                7 => {
                    sv.extend(arr16.iter().cloned());
                }
                8 => {
                    sv.retain(|x| {
                        println!("{:?}", x);
                        let b = _to_u8(g1, i % g1.len());
                        b % 2 == 0
                    });
                }
                9 => {
                    sv.dedup();
                }
                10 => {
                    let new_len = num_b.wrapping_add(i);
                    sv.resize_with(new_len, || elem0.clone());
                }
                _ => {
                    let start = num_c;
                    let end = num_d;
                    let mut dr = sv.drain(start..end);
                    let _ = dr.next();
                    let _ = dr.next_back();
                }
            }
            if sv.len() > 0 {
                let r = &sv[0];
                println!("{:?}", r);
            }
            if !sv.is_empty() {
                let ms = sv.as_mut_slice();
                let last = ms.len() - 1;
                ms[last] = elem0.clone();
                println!("{:?}", &ms[last]);
            }
        }

        let s_from_vec2: SmallVec<[CustomType; 16]> = SmallVec::from(v.clone());
        let _ = SmallVec::<[CustomType; 16]>::eq(&sv, &s_from_vec2);
        let _ = SmallVec::<[CustomType; 16]>::partial_cmp(&sv, &s_from_vec2);
        let _ = SmallVec::<[CustomType; 16]>::cmp(&sv, &s_from_vec2);

        let mut it = sv.clone().into_iter();
        let _ = it.next();
        let _ = it.next_back();
        let its = it.as_slice();
        if let Some(r) = its.get(0) {
            println!("{:?}", r);
        }

        let _ = s_from_vec1.capacity();
        let _ = s_from_vec1.is_empty();
        let _ = s_from_vec1.len();

        let _ = s_from_vec2.clone().into_boxed_slice();
        let _ = s_from_vec1.clone().into_inner();

        let mut other = SmallVec::<[CustomType; 16]>::from_buf(arr16.clone());
        sv.append(&mut other);

        let aref = sv.as_ref();
        if let Some(r) = aref.get(0) {
            println!("{:?}", r);
        }

        let vec_back = s_from_vec2.clone().into_vec();
        let _sv3: SmallVec<[CustomType; 16]> = SmallVec::from(vec_back);
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