#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

#[derive(Debug, Copy, PartialEq, Eq, Hash)]
struct CustomType1(usize);

impl core::clone::Clone for CustomType1 {
    fn clone(&self) -> Self {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 10);
        let custom_impl_inst_num = self.0;
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0 {
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector {
            1 => global_data.first_half,
            _ => global_data.second_half,
        };
        let t_4 = _to_usize(GLOBAL_DATA, 18);
        CustomType1(t_4)
    }
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 2200 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let SECOND = global_data.second_half;

        let count = (_to_u8(GLOBAL_DATA, 0) % 65) as usize;
        let mut base_vec = std::vec::Vec::with_capacity(64);
        for i in 0..count {
            let idx = 10 + i * 8;
            let val = _to_usize(GLOBAL_DATA, idx);
            base_vec.push(CustomType1(val));
        }

        let s_from_slice = smallvec::SmallVec::<[CustomType1; 32]>::from_slice(&base_vec[..]);
        let s_to_smallvec = smallvec::ToSmallVec::<[CustomType1; 32]>::to_smallvec(&base_vec[..]);
        let elem = CustomType1(_to_usize(GLOBAL_DATA, 600));
        let n_elems = (_to_u8(GLOBAL_DATA, 601) % 65) as usize;
        let s_from_elem = smallvec::SmallVec::<[CustomType1; 32]>::from_elem(elem, n_elems);
        let s_from_vec = smallvec::SmallVec::<[CustomType1; 32]>::from_vec(base_vec.clone());
        let mut s_new = smallvec::SmallVec::<[CustomType1; 32]>::new();
        let fill_n = (_to_u8(GLOBAL_DATA, 4) % 65) as usize;
        for j in 0..fill_n {
            let v = _to_usize(GLOBAL_DATA, 100 + j * 8);
            s_new.push(CustomType1(v));
        }

        let sel = _to_u8(GLOBAL_DATA, 3) % 5;
        let mut sv = match sel {
            0 => s_from_slice,
            1 => s_to_smallvec,
            2 => s_from_elem,
            3 => s_from_vec,
            _ => s_new,
        };

        let ops = (_to_u8(GLOBAL_DATA, 5) % 16) as usize;
        for k in 0..ops {
            let which = _to_u8(GLOBAL_DATA, 20 + k) % 10;
            match which {
                0 => {
                    let v = _to_usize(GLOBAL_DATA, 120 + k * 8);
                    sv.push(CustomType1(v));
                }
                1 => {
                    let idx = _to_usize(GLOBAL_DATA, 200 + k * 8);
                    let v = _to_usize(GLOBAL_DATA, 300 + k * 8);
                    sv.insert(idx, CustomType1(v));
                }
                2 => {
                    sv.retain(|e| {
                        let gd = get_global_data();
                        let b = _to_u8(gd.second_half, 10 + k);
                        if b % 11 == 0 {
                            panic!("INTENTIONAL PANIC!");
                        }
                        e.0 % 2 == (b as usize % 2)
                    });
                }
                3 => {
                    sv.dedup();
                }
                4 => {
                    let new_len = (_to_u8(GLOBAL_DATA, 320 + k) % 65) as usize;
                    let mut t = 0usize;
                    sv.resize_with(new_len, || {
                        t = t.wrapping_add(1);
                        let gd = get_global_data();
                        let gate = _to_u8(gd.first_half, 330 + k);
                        if gate % 13 == 0 {
                            panic!("INTENTIONAL PANIC!");
                        }
                        CustomType1(t)
                    });
                }
                5 => {
                    let len = (_to_u8(GLOBAL_DATA, 340 + k) % 65) as usize;
                    sv.truncate(len);
                }
                6 => {
                    let slen = base_vec.len();
                    let take = if slen == 0 { 0 } else { (_to_u8(GLOBAL_DATA, 360 + k) as usize) % slen };
                    sv.extend_from_slice(&base_vec[..take]);
                }
                7 => {
                    let idx = _to_usize(GLOBAL_DATA, 380 + k * 8);
                    let _ = sv.swap_remove(idx);
                }
                8 => {
                    let _ = sv.pop();
                }
                _ => {
                    sv.clear();
                }
            }
        }

        let mode = _to_u8(GLOBAL_DATA, 512) % 4;
        if mode == 0 {
            let i = _to_usize(GLOBAL_DATA, 520);
            let r: &CustomType1 = (&sv).index(i);
            println!("{:?}", *r);
        } else if mode == 1 {
            let a = _to_usize(GLOBAL_DATA, 528);
            let b = _to_usize(GLOBAL_DATA, 536);
            let r: &[CustomType1] = (&sv).index(a..b);
            if let Some(first) = r.get(0) {
                println!("{:?}", *first);
            }
            println!("{}", r.len());
        } else if mode == 2 {
            let a = _to_usize(GLOBAL_DATA, 544);
            let r: &[CustomType1] = (&sv).index(a..);
            if let Some(first) = r.get(0) {
                println!("{:?}", *first);
            }
            println!("{}", r.len());
        } else {
            let b = _to_usize(GLOBAL_DATA, 552);
            let r: &[CustomType1] = (&sv).index(..b);
            if let Some(first) = r.get(0) {
                println!("{:?}", *first);
            }
            println!("{}", r.len());
        }

        let sref = smallvec::SmallVec::as_slice(&sv);
        println!("{}", sref.len());
        if let Some(x) = sref.get(0) {
            println!("{:?}", *x);
        }
        let bref: &[CustomType1] = std::borrow::Borrow::borrow(&sv);
        println!("{}", bref.len());
        if let Some(x) = bref.get(0) {
            println!("{:?}", *x);
        }
        let dref: &[CustomType1] = sv.deref();
        println!("{}", dref.len());
        if let Some(x) = dref.get(0) {
            println!("{:?}", *x);
        }

        let j = _to_usize(SECOND, 100);
        let mref: &mut CustomType1 = std::ops::IndexMut::index_mut(&mut sv, j);
        mref.0 = mref.0.wrapping_add(1);
        println!("{:?}", *mref);

        let rem_idx = _to_usize(GLOBAL_DATA, 600);
        let _ = sv.remove(rem_idx);

        let da = _to_usize(GLOBAL_DATA, 640);
        let db = _to_usize(GLOBAL_DATA, 648);
        let mut dr = sv.drain(da..db);
        let _ = dr.next();
        let _ = dr.next_back();
        std::mem::drop(dr);

        {
            let ms = sv.as_mut_slice();
            if !ms.is_empty() {
                ms[0].0 = ms[0].0.wrapping_add(1);
                println!("{:?}", ms[0]);
            }
        }

        let v2 = sv.into_vec();
        println!("{}", v2.len());
        if let Some(x) = v2.get(0) {
            println!("{:?}", *x);
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