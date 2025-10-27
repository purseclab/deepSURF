#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

#[derive(Debug)]
struct CustomType1(String);

impl core::cmp::PartialEq for CustomType1 {
    fn eq(&self, _: &Self) -> bool {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 0);
        let custom_impl_inst_num = self.0.len();
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0{
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector{
                1 => global_data.first_half,
                _ => global_data.second_half,
        };
        let t_0 = _to_bool(GLOBAL_DATA, 8);
        return t_0;
    }
}

impl core::clone::Clone for CustomType1 {
    fn clone(&self) -> Self {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 28);
        let custom_impl_inst_num = self.0.len();
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0{
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector{
                1 => global_data.first_half,
                _ => global_data.second_half,
        };
        let mut t_7 = _to_u8(GLOBAL_DATA, 36) % 17;
        let t_8 = _to_str(GLOBAL_DATA, 37, 37 + t_7 as usize);
        let t_9 = String::from(t_8);
        let t_10 = CustomType1(t_9);
        return t_10;
    }
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 1200 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let FIRST = global_data.first_half;
        let SECOND = global_data.second_half;

        let n1 = (_to_u8(FIRST, 27) % 65) as usize;
        let mut v1: Vec<CustomType1> = Vec::with_capacity(64);
        for j in 0..n1 {
            let start = 50 + 7 * j;
            let len = (_to_u8(FIRST, 52 + j) % 17) as usize;
            let s = _to_str(FIRST, start, start + len);
            v1.push(CustomType1(String::from(s)));
        }

        let n2 = (_to_u8(FIRST, 118) % 65) as usize;
        let mut v2: Vec<CustomType1> = Vec::with_capacity(64);
        for j in 0..n2 {
            let start = 200 + 5 * j;
            let len = (_to_u8(FIRST, 120 + j) % 17) as usize;
            let s = _to_str(FIRST, start, start + len);
            v2.push(CustomType1(String::from(s)));
        }

        let l3 = (_to_u8(FIRST, 300) % 17) as usize;
        let base_s = _to_str(FIRST, 301, 301 + l3);
        let base_elem = CustomType1(String::from(base_s));

        let c1 = _to_u8(FIRST, 9) % 5;
        let mut s1: SmallVec<[CustomType1; 16]> = match c1 {
            0 => SmallVec::<[CustomType1; 16]>::from(&v1[..]),
            1 => SmallVec::from_vec(v1.clone()),
            2 => {
                let mut sv = SmallVec::<[CustomType1; 16]>::new();
                for it in &v1 {
                    sv.push(it.clone());
                }
                sv
            }
            3 => {
                let mut sv = SmallVec::<[CustomType1; 16]>::with_capacity(_to_usize(FIRST, 10));
                let take = if v1.is_empty() { 0 } else { (_to_u8(FIRST, 11) as usize) % (v1.len()) };
                for it in v1.iter().take(take) {
                    sv.push(it.clone());
                }
                sv
            }
            _ => SmallVec::<[CustomType1; 16]>::from_elem(base_elem.clone(), _to_usize(FIRST, 12)),
        };

        let c2 = _to_u8(FIRST, 13) % 5;
        let mut s2: SmallVec<[CustomType1; 16]> = match c2 {
            0 => SmallVec::<[CustomType1; 16]>::from(&v2[..]),
            1 => SmallVec::from_vec(v2.clone()),
            2 => {
                let mut sv = SmallVec::<[CustomType1; 16]>::new();
                for it in &v2 {
                    sv.push(it.clone());
                }
                sv
            }
            3 => {
                let mut sv = SmallVec::<[CustomType1; 16]>::with_capacity(_to_usize(FIRST, 14));
                let take = if v2.is_empty() { 0 } else { (_to_u8(FIRST, 15) as usize) % (v2.len()) };
                for it in v2.iter().take(take) {
                    sv.push(it.clone());
                }
                sv
            }
            _ => SmallVec::<[CustomType1; 16]>::from_elem(base_elem.clone(), _to_usize(FIRST, 16)),
        };

        s1.reserve(_to_usize(FIRST, 320));
        let _ = s1.try_reserve(_to_usize(FIRST, 328));
        if !s1.is_empty() {
            let sl = s1.as_slice();
            println!("{:?}", &sl[0]);
        }
        if !s1.is_empty() {
            println!("{:?}", &s1[0]);
        }
        if !s1.is_empty() {
            let ms = s1.as_mut_slice();
            let x = ms[0].clone();
            ms[0] = x;
        }
        s1.extend(s2.as_slice().iter().cloned());
        let elem_str_len = (_to_u8(FIRST, 338) % 17) as usize;
        let elem_str = _to_str(FIRST, 339, 339 + elem_str_len);
        let ins_elem = CustomType1(String::from(elem_str));
        s1.insert(_to_usize(FIRST, 336), ins_elem);
        s1.insert_many(_to_usize(FIRST, 344), s2.as_slice().iter().cloned());
        {
            let mut flag_a = _to_u8(FIRST, 352);
            s1.dedup_by(|a, b| {
                if flag_a % 2 == 0 { panic!("INTENTIONAL PANIC!"); }
                flag_a = flag_a.wrapping_add(1);
                a.0.len() == b.0.len()
            });
        }
        {
            let mut flag_b = _to_u8(FIRST, 353);
            s1.dedup_by_key(|x| {
                if flag_b % 3 == 0 { panic!("INTENTIONAL PANIC!"); }
                flag_b = flag_b.wrapping_add(1);
                x.0.len()
            });
        }
        {
            let mut flag_c = _to_u8(FIRST, 354);
            s1.retain(|x| {
                if flag_c % 5 == 0 { panic!("INTENTIONAL PANIC!"); }
                flag_c = flag_c.wrapping_add(1);
                x.0.len() % 2 == 0
            });
        }
        s1.truncate(_to_usize(FIRST, 360));
        if !s1.is_empty() {
            let _ = s1.remove(_to_usize(FIRST, 368));
        }
        if !s1.is_empty() {
            let _ = s1.swap_remove(_to_usize(FIRST, 376));
        }
        s1.grow(_to_usize(FIRST, 384));
        println!("{}", s1.capacity());

        let eq_res = (&s1).eq(&s2);
        println!("{}", eq_res);

        s1.append(&mut s2);
        {
            let mut d = s1.drain(0..0);
            let _ = d.next();
            let _ = d.next_back();
        }

        let _ = s1.clone().into_vec();
        let _ = s1.clone().into_boxed_slice();

        let sr = s1.as_ref();
        println!("{:?}", sr.get(0));
        let smr = s1.as_mut();
        if !smr.is_empty() {
            let y = smr[0].clone();
            smr[0] = y;
        }
        let _ = s1.try_reserve_exact(_to_usize(FIRST, 392));
        s1.shrink_to_fit();
        let _ = s1.len();
        let _ = s1.is_empty();

        let ops = (_to_u8(SECOND, 0) % 10) as usize;
        for i in 0..ops {
            let op = _to_u8(SECOND, 1 + i) % 8;
            match op {
                0 => {
                    let l = (_to_u8(SECOND, 20 + i) % 17) as usize;
                    let st = _to_str(SECOND, 21 + i, 21 + i + l);
                    s1.push(CustomType1(String::from(st)));
                }
                1 => {
                    let _ = s1.pop();
                }
                2 => {
                    s2.reserve(_to_usize(SECOND, 8));
                }
                3 => {
                    if !s1.is_empty() {
                        println!("{:?}", &s1[0]);
                    }
                }
                4 => {
                    let slice = s1.as_slice();
                    let sv3: SmallVec<[CustomType1; 16]> = SmallVec::from_iter(slice.iter().cloned());
                    let _ = sv3.eq(&s1);
                }
                5 => {
                    let mut it = s1.clone().into_iter();
                    let _ = it.next();
                    let _ = it.next_back();
                    let rem = it.as_slice();
                    if !rem.is_empty() {
                        println!("{:?}", &rem[0]);
                    }
                }
                6 => {
                    if !s1.is_empty() {
                        let r = s1.index(0);
                        println!("{:?}", r);
                    }
                }
                _ => {
                    if !s1.is_empty() {
                        let r = s1.index_mut(0);
                        let z = r.clone();
                        *r = z;
                    }
                }
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