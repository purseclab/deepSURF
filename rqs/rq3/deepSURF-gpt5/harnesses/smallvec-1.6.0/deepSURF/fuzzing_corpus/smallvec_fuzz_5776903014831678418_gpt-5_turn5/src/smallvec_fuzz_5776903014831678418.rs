#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct CustomType1(String);

impl core::clone::Clone for CustomType1 {
    fn clone(&self) -> Self {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 42);
        let custom_impl_inst_num = self.0.len();
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0{
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector{
                1 => global_data.first_half,
                _ => global_data.second_half,
        };
        let mut t_10 = _to_u8(GLOBAL_DATA, 50) % 17;
        let t_11 = _to_str(GLOBAL_DATA, 51, 51 + t_10 as usize);
        let t_12 = String::from(t_11);
        let t_13 = CustomType1(t_12);
        return t_13;
    }
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 240 { return; }
        set_global_data(data);
        let gd = get_global_data();
        let FIRST = gd.first_half;
        let SECOND = gd.second_half;

        let l1 = (_to_u8(FIRST, 10) % 17) as usize;
        let l2 = (_to_u8(FIRST, 30) % 17) as usize;
        let l3 = (_to_u8(FIRST, 50) % 17) as usize;
        let l4 = (_to_u8(FIRST, 70) % 17) as usize;
        let l5 = (_to_u8(FIRST, 90) % 17) as usize;

        let s1 = _to_str(FIRST, 11, 11 + l1);
        let s2 = _to_str(FIRST, 31, 31 + l2);
        let s3 = _to_str(FIRST, 51, 51 + l3);
        let s4 = _to_str(FIRST, 71, 71 + l4);
        let s5 = _to_str(FIRST, 91, 91 + l5);

        let v1 = CustomType1(String::from(s1));
        let v2 = CustomType1(String::from(s2));
        let v3 = CustomType1(String::from(s3));
        let v4 = CustomType1(String::from(s4));
        let v5 = CustomType1(String::from(s5));

        let arr32: [CustomType1; 32] = core::array::from_fn(|_| v2.clone());

        let ctor_sel = _to_u8(FIRST, 0) % 7;
        let mut sv: SmallVec<[CustomType1; 32]> = match ctor_sel {
            0 => SmallVec::new(),
            1 => SmallVec::with_capacity(_to_usize(FIRST, 1)),
            2 => SmallVec::from_elem(v3.clone(), _to_usize(FIRST, 5)),
            3 => {
                let mut tmp: Vec<CustomType1> = Vec::new();
                let n = (_to_u8(FIRST, 7) % 65) as usize;
                for _ in 0..n { tmp.push(v1.clone()); }
                SmallVec::from_vec(tmp)
            }
            4 => {
                let mut tmp: Vec<CustomType1> = Vec::new();
                let n = (_to_u8(FIRST, 8) % 65) as usize;
                for _ in 0..n { tmp.push(v2.clone()); }
                SmallVec::from_vec(tmp)
            }
            5 => {
                let n = (_to_u8(FIRST, 9) % 65) as usize;
                let it = (0..n).map(|_| v4.clone());
                SmallVec::from_iter(it)
            }
            _ => {
                let len = _to_usize(FIRST, 9);
                SmallVec::from_buf_and_len(arr32, len)
            }
        };

        let mut other: SmallVec<[CustomType1; 32]> = {
            let mut tmp: Vec<CustomType1> = Vec::new();
            let n = (_to_u8(FIRST, 21) % 65) as usize;
            for _ in 0..n { tmp.push(v5.clone()); }
            SmallVec::from_vec(tmp)
        };

        let ops_bound = if SECOND.len() > 1 { SECOND.len() - 1 } else { 0 };
        let mut ops = (_to_u8(SECOND, 0) as usize) % 32;
        if ops > ops_bound { ops = ops_bound; }

        for i in 0..ops {
            let op = _to_u8(SECOND, 1 + i) % 20;
            match op {
                0 => {
                    sv.push(v1.clone());
                }
                1 => {
                    sv.insert(_to_usize(FIRST, 60), v2.clone());
                }
                2 => {
                    let _ = sv.pop();
                }
                3 => {
                    let _ = sv.remove(_to_usize(FIRST, 70));
                }
                4 => {
                    let _ = sv.swap_remove(_to_usize(FIRST, 74));
                }
                5 => {
                    sv.reserve(_to_usize(FIRST, 78));
                }
                6 => {
                    sv.reserve_exact(_to_usize(FIRST, 82));
                }
                7 => {
                    let _ = sv.try_reserve(_to_usize(FIRST, 86));
                }
                8 => {
                    let _ = sv.try_reserve_exact(_to_usize(FIRST, 88));
                }
                9 => {
                    sv.resize(_to_usize(FIRST, 90), v3.clone());
                }
                10 => {
                    sv.resize_with(_to_usize(FIRST, 94), || v4.clone());
                }
                11 => {
                    let slice = other.as_slice();
                    sv.extend(slice.iter().cloned());
                }
                12 => {
                    let slice = other.as_slice();
                    sv.insert_many(_to_usize(FIRST, 96), slice.iter().cloned());
                }
                13 => {
                    sv.clear();
                }
                14 => {
                    sv.truncate(_to_usize(FIRST, 100));
                }
                15 => {
                    let s = sv.as_slice();
                    if !s.is_empty() {
                        let r = &s[0];
                        println!("{:?}", r);
                    }
                }
                16 => {
                    let s = sv.as_mut_slice();
                    if !s.is_empty() {
                        s[0] = v5.clone();
                        let r = &s[0];
                        println!("{:?}", r);
                    }
                }
                17 => {
                    let ord = sv.cmp(&other);
                    println!("{:?}", ord);
                    let _ = sv.partial_cmp(&other);
                }
                18 => {
                    let idx = _to_usize(FIRST, 102);
                    if sv.len() > 0 {
                        let _ = &sv[idx];
                    }
                }
                _ => {
                    let toggle = _to_u8(FIRST, 41);
                    if toggle % 3 == 0 {
                        sv.dedup();
                    } else if toggle % 3 == 1 {
                        sv.dedup_by(|a, b| {
                            let t = _to_u8(FIRST, 40);
                            if t % 2 == 0 { a.0 = b.0.clone(); }
                            t % 2 == 0
                        });
                    } else {
                        sv.retain(|e| {
                            let p = _to_u8(FIRST, 39);
                            (e.0.len() as u8) != p
                        });
                        sv.dedup_by_key(|x| x.0.len());
                    }
                }
            }
        }

        let _cap = sv.capacity();
        let _len = sv.len();
        let _empty = sv.is_empty();

        let sref: &[CustomType1] = sv.deref();
        if !sref.is_empty() {
            let r = &sref[0];
            println!("{:?}", r);
        }
        let smut: &mut [CustomType1] = sv.deref_mut();
        if !smut.is_empty() {
            smut[0] = v1.clone();
            let r = &smut[0];
            println!("{:?}", r);
        }

        {
            let mut dr = sv.drain(0.._to_usize(FIRST, 104));
            let _ = dr.next();
            let _ = dr.next_back();
        }

        let sv_clone = sv.clone();
        let mut it = sv_clone.into_iter();
        let _ = it.next();
        let _ = it.next_back();
        let _ = it.as_slice();

        let boxed = other.clone().into_boxed_slice();
        let slice_from_box: &[CustomType1] = &boxed;
        let small_again: SmallVec<[CustomType1; 32]> = SmallVec::from(slice_from_box);
        let _ = small_again.len();

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