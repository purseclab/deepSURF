#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

#[derive(Clone, Debug)]
struct CustomType1(String);
#[derive(Clone)]
struct CustomType2(String);
#[derive(Clone)]
struct CustomType3(String);

impl core::iter::IntoIterator for CustomType2 {
    type Item = CustomType1;
    type IntoIter = CustomType3;
    fn into_iter(self) -> Self::IntoIter {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 83);
        let custom_impl_inst_num = self.0.len();
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0 {
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector {
            1 => global_data.first_half,
            _ => global_data.second_half,
        };
        let mut t_18 = _to_u8(GLOBAL_DATA, 91) % 17;
        let t_19 = _to_str(GLOBAL_DATA, 92, 92 + t_18 as usize);
        let t_20 = String::from(t_19);
        let t_21 = CustomType3(t_20);
        t_21
    }
}

impl core::iter::Iterator for CustomType3 {
    type Item = CustomType1;
    fn next(&mut self) -> Option<Self::Item> {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 34);
        let custom_impl_inst_num = self.0.len();
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0 {
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector {
            1 => global_data.first_half,
            _ => global_data.second_half,
        };
        let mut t_9 = _to_u8(GLOBAL_DATA, 42) % 17;
        let t_10 = _to_str(GLOBAL_DATA, 43, 43 + t_9 as usize);
        let t_11 = String::from(t_10);
        let t_12 = CustomType1(t_11);
        let t_13 = Some(t_12);
        t_13
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 59);
        let custom_impl_inst_num = self.0.len();
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0 {
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector {
            1 => global_data.first_half,
            _ => global_data.second_half,
        };
        let t_14 = _to_usize(GLOBAL_DATA, 67);
        let t_15 = _to_usize(GLOBAL_DATA, 75);
        let t_16 = Some(t_15);
        let t_17 = (t_14, t_16);
        t_17
    }
}

fn mk_custom1_from_data(start: usize, use_first: bool) -> CustomType1 {
    let gd = get_global_data();
    let g = if use_first { gd.first_half } else { gd.second_half };
    let len = _to_u8(g, start) % 17;
    let s = _to_str(g, start + 1, start + 1 + len as usize);
    CustomType1(String::from(s))
}

fn build_vec_from(start: usize, use_first: bool) -> Vec<CustomType1> {
    let gd = get_global_data();
    let g = if use_first { gd.first_half } else { gd.second_half };
    let mut v = Vec::new();
    let n = (_to_u8(g, start) % 65) as usize;
    for i in 0..n {
        let off = (start + 1 + i * 3) % 100;
        v.push(mk_custom1_from_data(off, use_first));
    }
    v
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 250 { return; }
        set_global_data(data);
        let global = get_global_data();
        let first = global.first_half;
        let second = global.second_half;

        let mut sv_a: SmallVec<[CustomType1; 16]> = SmallVec::new();
        sv_a.push(mk_custom1_from_data(10, true));
        sv_a.push(mk_custom1_from_data(20, false));
        println!("{}", sv_a.len());

        let cap_b = _to_usize(first, 32);
        let mut sv_b: SmallVec<[CustomType1; 16]> = SmallVec::with_capacity(cap_b);
        sv_b.push(mk_custom1_from_data(40, true));
        println!("{}", sv_b.capacity());

        let v1 = build_vec_from(50, true);
        let mut sv_from_vec = SmallVec::<[CustomType1; 16]>::from_vec(v1.clone());
        let s1 = &v1[..];
        let mut sv_from_slice = SmallVec::<[CustomType1; 16]>::from(s1);
        println!("{}", sv_from_slice.len());

        let _ = sv_a.capacity();
        let _ = sv_a.is_empty();

        sv_a.extend(v1.clone());
        let slice2 = sv_from_vec.as_slice();
        println!("{}", slice2.len());
        if let Some(x) = slice2.get(0) { println!("{:?}", x); }
        sv_a.extend(slice2.iter().cloned());

        let mut t_22 = _to_u8(first, 108) % 17;
        let t_23 = _to_str(first, 109, 109 + t_22 as usize);
        let ct2 = CustomType2(String::from(t_23));
        sv_a.extend(ct2);

        sv_a.insert_many(_to_usize(second, 60), sv_from_slice.clone().into_iter());
        sv_a.reserve(_to_usize(second, 70));
        let _ = sv_a.try_reserve(_to_usize(first, 80));
        sv_a.reserve_exact(_to_usize(first, 88));
        sv_a.grow(_to_usize(second, 96));

        let r1 = sv_a.as_slice();
        println!("{}", r1.len());
        if let Some(x) = r1.get(0) { println!("{:?}", x); }

        let r2 = sv_a.as_mut_slice();
        if let Some(x) = r2.get_mut(0) { println!("{:?}", x); }

        let idx1 = _to_usize(first, 104);
        println!("{:?}", &sv_a[idx1]);
        let new_item = mk_custom1_from_data(60, true);
        sv_a.insert(_to_usize(first, 120), new_item.clone());
        let idx2 = _to_usize(second, 128);
        sv_a[idx2] = new_item.clone();

        if let Some(p) = sv_a.pop() { println!("{:?}", p); }
        let rm = sv_a.remove(_to_usize(first, 136));
        println!("{:?}", rm);
        let sw = sv_a.swap_remove(_to_usize(second, 140));
        println!("{:?}", sw);

        let mut dr = sv_from_vec.drain(0.._to_usize(second, 144));
        if let Some(a) = dr.next() { println!("{:?}", a); }
        if let Some(b) = dr.next_back() { println!("{:?}", b); }

        sv_a.dedup_by(|a, b| {
            let gd = get_global_data();
            let g = gd.first_half;
            let s = _to_usize(g, 152);
            if s % 5 == 0 {
                panic!("INTENTIONAL PANIC!");
            }
            a.0 == b.0
        });

        sv_a.dedup_by_key(|x| x.0.len());

        sv_a.retain(|x| {
            let gd = get_global_data();
            let g = gd.second_half;
            println!("{:?}", x);
            _to_bool(g, 168)
        });

        sv_a.resize_with(_to_usize(first, 176), || mk_custom1_from_data(70, false));
        sv_a.resize(_to_usize(second, 184), mk_custom1_from_data(72, true));

        let v3 = build_vec_from(74, true);
        let slice3 = &v3[..];
        sv_a.insert_many(_to_usize(first, 192), slice3.iter().cloned());
        sv_a.extend(slice3.iter().cloned());
        sv_a.truncate(_to_usize(second, 200));
        let _ = sv_a.try_reserve_exact(_to_usize(first, 208));

        let vec_final = sv_a.clone().into_vec();
        println!("{}", vec_final.len());
        let sv_from_iter = SmallVec::<[CustomType1; 16]>::from_iter(vec_final.clone());
        println!("{}", sv_from_iter.len());

        sv_b.append(&mut sv_from_slice);
        let b1 = sv_b.as_slice();
        println!("{}", b1.len());
        let b2 = sv_b.as_mut_slice();
        if let Some(x) = b2.get_mut(0) { println!("{:?}", x); }

        let res = sv_from_iter.clone().into_inner();
        match res {
            Ok(arr) => {
                let tmp = SmallVec::<[CustomType1; 16]>::from_buf(arr);
                println!("{}", tmp.len());
            }
            Err(mut sv) => {
                println!("{}", sv.len());
                sv.clear();
            }
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