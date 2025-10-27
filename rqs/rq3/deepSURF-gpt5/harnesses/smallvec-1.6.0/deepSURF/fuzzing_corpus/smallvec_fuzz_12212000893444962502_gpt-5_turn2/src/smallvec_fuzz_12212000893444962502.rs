#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

#[derive(Clone, Debug)]
struct CustomType1(String);

fn _custom_fn0(_: &mut CustomType1) -> bool {
    let global_data = get_global_data();
    let GLOBAL_DATA = global_data.first_half;
    let t_9 = _to_u8(GLOBAL_DATA, 34);
    if t_9 % 2 == 0{
        panic!("INTENTIONAL PANIC!");
    }
    let t_10 = _to_bool(GLOBAL_DATA, 35);
    return t_10;
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 200 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let n0 = (_to_u8(GLOBAL_DATA, 0) % 20) as usize;
        let s0 = _to_str(GLOBAL_DATA, 1, 1 + n0);
        let base = CustomType1(String::from(s0));

        let mut v: Vec<CustomType1> = Vec::new();
        let m = (_to_u8(GLOBAL_DATA, 20) % 65) as usize;
        for i in 0..m {
            let start = 2 + (i % 10) as usize;
            let end = start + 1 + (i % 8) as usize;
            let s = _to_str(GLOBAL_DATA, start, end);
            v.push(CustomType1(String::from(s)));
        }

        let elem = CustomType1(base.0.clone());
        let arr: [CustomType1; 16] = [
            elem.clone(), elem.clone(), elem.clone(), elem.clone(),
            elem.clone(), elem.clone(), elem.clone(), elem.clone(),
            elem.clone(), elem.clone(), elem.clone(), elem.clone(),
            elem.clone(), elem.clone(), elem.clone(), elem.clone()
        ];

        let mode = _to_u8(GLOBAL_DATA, 40);
        let mut sv: SmallVec<[CustomType1; 16]> = match mode % 6 {
            0 => SmallVec::new(),
            1 => SmallVec::with_capacity(_to_usize(GLOBAL_DATA, 41)),
            2 => SmallVec::from_vec(v.clone()),
            3 => SmallVec::from_vec(v.clone()),
            4 => SmallVec::from_iter(v.clone().into_iter()),
            _ => SmallVec::from_buf_and_len(arr, _to_usize(GLOBAL_DATA, 49)),
        };

        sv.reserve(_to_usize(GLOBAL_DATA, 57));
        let _ = sv.try_reserve(_to_usize(GLOBAL_DATA, 58));
        sv.grow(_to_usize(GLOBAL_DATA, 65));

        let s1 = _to_str(GLOBAL_DATA, 12, 18);
        sv.push(CustomType1(String::from(s1)));
        let s2 = _to_str(GLOBAL_DATA, 18, 22);
        sv.push(CustomType1(String::from(s2)));
        let idx_ins = _to_usize(GLOBAL_DATA, 73);
        let s3 = _to_str(GLOBAL_DATA, 22, 28);
        sv.insert(idx_ins, CustomType1(String::from(s3)));
        let idx_remove = _to_usize(GLOBAL_DATA, 81);
        let _ = sv.remove(idx_remove);
        let idx_swap = _to_usize(GLOBAL_DATA, 89);
        let _ = sv.swap_remove(idx_swap);

        if !sv.is_empty() {
            let r = &sv[0];
            println!("{:?}", r);
        }
        let slice_ref = sv.as_slice();
        println!("{:?}", slice_ref);
        let m_slice = sv.as_mut_slice();
        if !m_slice.is_empty() {
            m_slice[0].0.push_str("x");
        }
        let deref_slice = sv.deref();
        println!("{:?}", deref_slice);
        let deref_mut_slice = sv.deref_mut();
        if !deref_mut_slice.is_empty() {
            deref_mut_slice[0].0.push_str("y");
        }

        let take = if v.len() > 0 { std::cmp::min(v.len(), (m / 2) as usize) } else { 0 };
        if take > 0 {
            sv.extend(v[..take].iter().cloned());
        }
        sv.insert_many(_to_usize(GLOBAL_DATA, 58), v.clone().into_iter());

        sv.dedup_by(|a, b| {
            let c = _to_bool(GLOBAL_DATA, 36);
            if c { a.0 == b.0 } else { a.0.len() == b.0.len() }
        });
        sv.dedup_by_key(|x| x.0.len());

        sv.resize(_to_usize(GLOBAL_DATA, 65), CustomType1(String::from("k")));
        let mut cnt = 0usize;
        sv.resize_with(_to_usize(GLOBAL_DATA, 73), || {
            cnt = cnt.wrapping_add(1);
            let s = _to_str(GLOBAL_DATA, 28, 34);
            CustomType1(String::from(s))
        });
        println!("{:?}", cnt);

        let mut f = _custom_fn0;
        sv.retain(f);

        let cap = sv.capacity();
        println!("{:?}", cap);
        let len = sv.len();
        println!("{:?}", len);

        sv.truncate(_to_usize(GLOBAL_DATA, 57));
        let _ = sv.pop();

        let mut sv2: SmallVec<[CustomType1; 16]> = SmallVec::new();
        if m > 0 {
            sv2.extend(v.clone());
        }
        sv.append(&mut sv2);

        let range_end = _to_usize(GLOBAL_DATA, 81);
        let mut dr = sv.drain(0..range_end);
        let _ = dr.next();
        let _ = dr.next_back();
        drop(dr);

        sv.shrink_to_fit();

        let b2 = _to_bool(GLOBAL_DATA, 37);
        sv.retain(|e| {
            if b2 {
                e.0.push_str("z");
            }
            e.0.len() % 2 == 0
        });

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