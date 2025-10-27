#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};
use std::borrow::{Borrow, BorrowMut};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct CustomType1(String);

fn mk_item(seed: usize) -> CustomType1 {
    let gd = get_global_data();
    let a = gd.first_half;
    let b = gd.second_half;
    let al = a.len();
    let bl = b.len();
    let s0 = if al > 2 { seed % (al - 1) } else { 0 };
    let s1 = if bl > 0 { (seed * 3 + if bl > seed { b[seed % bl] as usize } else { 0 }) % (al.saturating_sub(s0)) } else { 0 };
    let end = (s0 + 1 + (s1 % 32)).min(al);
    let s = if end > s0 { _to_str(a, s0, end) } else { "" };
    CustomType1(String::from_str(s).unwrap_or_default())
}

fn mk_vec_from_seed(n: usize) -> Vec<CustomType1> {
    let mut v = Vec::new();
    let count = n % 65;
    for i in 0..count {
        v.push(mk_item(i));
    }
    v
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 192 { return; }
        set_global_data(data);
        let gd = get_global_data();
        let a = gd.first_half;
        let b = gd.second_half;
        let base_usize_idx_b = [0usize, 8, 16, 24, 32, 40, 48];
        let cap_a = _to_usize(a, 8);
        let selector = _to_u8(a, 0);
        let seed_vec = mk_vec_from_seed(_to_u8(a, 1) as usize);
        let seed_slice = &seed_vec[..];
        let mut sv: SmallVec<[CustomType1; 32]> = match selector % 4 {
            0 => SmallVec::<[CustomType1; 32]>::new(),
            1 => SmallVec::<[CustomType1; 32]>::with_capacity(cap_a),
            2 => SmallVec::<[CustomType1; 32]>::from_vec(seed_vec.clone()),
            _ => SmallVec::<[CustomType1; 32]>::from(seed_slice),
        };
        let mut other = SmallVec::<[CustomType1; 32]>::from_elem(mk_item(3), (_to_u8(a, 2) as usize) % 65);
        sv.reserve(_to_usize(a, 16));
        let _ = _unwrap_result(sv.try_reserve(_to_usize(a, 24)));
        let _ = _unwrap_result(sv.try_reserve_exact(_to_usize(a, 32)));
        sv.grow(_to_usize(a, 40));
        sv.reserve_exact(_to_usize(a, 48));
        let sref = sv.deref();
        if !sref.is_empty() {
            println!("{:?}", &sref[0]);
        }
        let smut = sv.deref_mut();
        if !smut.is_empty() {
            smut[0] = mk_item(7);
        }
        let sr = sv.as_slice();
        if !sr.is_empty() {
            println!("{:?}", &sr[0]);
        }
        let smr = sv.as_mut_slice();
        if !smr.is_empty() {
            smr[0] = mk_item(9);
        }
        let sv2 = SmallVec::<[CustomType1; 32]>::from(sv.as_slice());
        let eqv = sv.eq(&sv2);
        println!("{}", eqv);
        let ord = sv.cmp(&sv2);
        match ord {
            core::cmp::Ordering::Less => println!("L"),
            core::cmp::Ordering::Equal => println!("E"),
            core::cmp::Ordering::Greater => println!("G"),
        }
        let _ = sv.partial_cmp(&sv2);
        sv.append(&mut other);
        let p_item0 = mk_item(_to_u8(b, 0) as usize);
        sv.push(p_item0);
        let ops = (1 + (_to_u8(b, 1) % 12)) as usize;
        for i in 0..ops {
            let sel = _to_u8(b, (2 + i) % (b.len() - 8));
            match sel % 12 {
                0 => {
                    sv.push(mk_item(10 + i));
                }
                1 => {
                    let idx = base_usize_idx_b[i % base_usize_idx_b.len()];
                    let at = _to_usize(b, idx);
                    sv.insert(at, mk_item(20 + i));
                }
                2 => {
                    let idx = base_usize_idx_b[i % base_usize_idx_b.len()];
                    let at = _to_usize(b, idx);
                    let _ = sv.swap_remove(at);
                }
                3 => {
                    let idx = base_usize_idx_b[i % base_usize_idx_b.len()];
                    let at = _to_usize(b, idx);
                    let _ = sv.remove(at);
                }
                4 => {
                    let idx = base_usize_idx_b[i % base_usize_idx_b.len()];
                    let new_len = _to_usize(b, idx);
                    sv.truncate(new_len);
                }
                5 => {
                    let idx1 = base_usize_idx_b[i % base_usize_idx_b.len()];
                    let idx2 = base_usize_idx_b[(i + 1) % base_usize_idx_b.len()];
                    let r1 = _to_usize(b, idx1);
                    let r2 = _to_usize(b, idx2);
                    let mut d = sv.drain(r1..r2);
                    if let Some(x) = d.next() {
                        println!("{:?}", x);
                    }
                    if let Some(x) = d.next_back() {
                        println!("{:?}", x);
                    }
                }
                6 => {
                    sv.extend(mk_vec_from_seed(i + 30).into_iter());
                }
                7 => {
                    let idx = base_usize_idx_b[i % base_usize_idx_b.len()];
                    let at = _to_usize(b, idx);
                    sv.insert_many(at, mk_vec_from_seed(i + 40).into_iter());
                }
                8 => {
                    let idx = base_usize_idx_b[i % base_usize_idx_b.len()];
                    let at = _to_usize(b, idx);
                    sv.insert_many(at, mk_vec_from_seed(i + 50).into_iter());
                }
                9 => {
                    let mut tgl = _to_bool(a, (50 + i) % (a.len() - 8));
                    sv.retain(|e| {
                        tgl = !tgl;
                        println!("{:?}", e);
                        tgl
                    });
                }
                10 => {
                    sv.dedup();
                    sv.dedup_by(|x, y| {
                        let z = x.0.len() == y.0.len();
                        z
                    });
                }
                _ => {
                    sv.resize_with((_to_u8(a, (60 + i) % (a.len() - 8)) as usize), || mk_item(60 + i));
                }
            }
        }
        if let Some(x) = sv.pop() {
            println!("{:?}", x);
        }
        let bsl: &[CustomType1] = sv.borrow();
        if !bsl.is_empty() {
            println!("{:?}", &bsl[0]);
        }
        let bsm: &mut [CustomType1] = sv.borrow_mut();
        if !bsm.is_empty() {
            bsm[0] = mk_item(77);
        }
        let sref2 = sv.as_ref();
        if !sref2.is_empty() {
            println!("{:?}", &sref2[0]);
        }
        let smut2 = sv.as_mut();
        if !smut2.is_empty() {
            smut2[0] = mk_item(88);
        }
        let mut iter = sv.clone().into_iter();
        let rs = iter.as_slice();
        if !rs.is_empty() {
            println!("{:?}", &rs[0]);
        }
        let rms = iter.as_mut_slice();
        if !rms.is_empty() {
            println!("{:?}", &rms[0]);
        }
        let _ = iter.next();
        let _ = iter.next_back();
        let _ = sv.index(0..sv.len().min(1));
        if sv.len() > 0 {
            let _ = sv.index_mut(0..1);
        }
        let _ = sv.into_vec();
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