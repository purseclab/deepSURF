#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct CustomType3(String);

#[derive(Debug)]
struct CustomType4 {
    a: usize,
    b: usize,
    s: String,
}

impl core::ops::RangeBounds<usize> for CustomType4 {
    fn start_bound(&self) -> core::ops::Bound<&usize> {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 588);
        let custom_impl_inst_num = self.s.len();
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0 {
            panic!("INTENTIONAL PANIC!");
        }
        match selector {
            1 => core::ops::Bound::Excluded(&self.a),
            _ => core::ops::Bound::Included(&self.a),
        }
    }
    fn end_bound(&self) -> core::ops::Bound<&usize> {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 613);
        let custom_impl_inst_num = self.s.len();
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0 {
            panic!("INTENTIONAL PANIC!");
        }
        match selector {
            1 => core::ops::Bound::Included(&self.b),
            _ => core::ops::Bound::Excluded(&self.b),
        }
    }
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 1600 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let OTHER = global_data.second_half;
        let mut base_items = std::vec::Vec::with_capacity(64);
        let count = (_to_u8(GLOBAL_DATA, 18) % 65) as usize;
        for i in 0..count {
            let lidx = 32 + (i % 16) as usize;
            let slen = (_to_u8(GLOBAL_DATA, lidx) % 17) as usize;
            let start = (64 + i * 7) % (GLOBAL_DATA.len().saturating_sub(32));
            let s = _to_str(GLOBAL_DATA, start, start + slen);
            base_items.push(CustomType3(String::from(s)));
        }
        let mut sv = match _to_u8(GLOBAL_DATA, 200) % 5 {
            0 => {
                let mut tmp = SmallVec::<[CustomType3; 32]>::new();
                tmp.extend(base_items.clone());
                tmp
            }
            1 => SmallVec::<[CustomType3; 32]>::from(base_items.clone()),
            2 => SmallVec::<[CustomType3; 32]>::from_vec(base_items.clone()),
            3 => {
                let el_len = (_to_u8(GLOBAL_DATA, 206) % 17) as usize;
                let el_s = _to_str(GLOBAL_DATA, 207, 207 + el_len);
                let el = CustomType3(String::from(el_s));
                let n = _to_usize(GLOBAL_DATA, 210);
                SmallVec::<[CustomType3; 32]>::from_elem(el, n)
            }
            _ => {
                let cap = _to_usize(GLOBAL_DATA, 220);
                let mut tmp = SmallVec::<[CustomType3; 32]>::with_capacity(cap);
                tmp.extend(base_items.clone());
                tmp
            }
        };
        let ops = (_to_u8(GLOBAL_DATA, 228) % 20) as usize;
        for i in 0..ops {
            match _to_u8(GLOBAL_DATA, 229 + (i % 10) as usize) % 12 {
                0 => {
                    let add = _to_usize(GLOBAL_DATA, 270);
                    sv.reserve(add);
                }
                1 => {
                    let add = _to_usize(GLOBAL_DATA, 276);
                    let _ = sv.try_reserve_exact(add);
                }
                2 => {
                    let idx = _to_usize(GLOBAL_DATA, 248);
                    if !sv.is_empty() {
                        let elem = sv[0].clone();
                        sv.insert(idx, elem);
                    }
                }
                3 => {
                    let idx = _to_usize(GLOBAL_DATA, 252);
                    if !sv.is_empty() {
                        let _ = sv.remove(idx);
                    }
                }
                4 => {
                    let _ = sv.pop();
                }
                5 => {
                    let sl = sv.as_slice();
                    if !sl.is_empty() {
                        println!("{:?}", &sl[0]);
                    }
                }
                6 => {
                    let val_len = (_to_u8(GLOBAL_DATA, 283) % 17) as usize;
                    let val_s = _to_str(GLOBAL_DATA, 284, 284 + val_len);
                    let lenx = _to_usize(GLOBAL_DATA, 282);
                    sv.resize(lenx, CustomType3(String::from(val_s)));
                }
                7 => {
                    let mut k = 0usize;
                    sv.dedup_by(|a, b| {
                        k = k.wrapping_add(1);
                        let flag = _to_bool(GLOBAL_DATA, 300 + (k % 7) as usize);
                        if flag {
                            a.0.push('a');
                            b.0.push('b');
                        }
                        flag
                    });
                }
                8 => {
                    sv.retain(|e| {
                        let b = _to_bool(OTHER, 310);
                        if b {
                            e.0.push('r');
                        }
                        b
                    });
                }
                9 => {
                    let mut cgen = 0u8;
                    sv.resize_with(_to_usize(GLOBAL_DATA, 320), || {
                        cgen = cgen.wrapping_add(1);
                        if cgen % 7 == 0 {
                            panic!("INTENTIONAL PANIC!");
                        }
                        let l = (_to_u8(GLOBAL_DATA, 321 + (cgen % 8) as usize) % 17) as usize;
                        let st = (330 + cgen as usize) % (GLOBAL_DATA.len().saturating_sub(40));
                        let s = _to_str(GLOBAL_DATA, st, st + l);
                        CustomType3(String::from(s))
                    });
                }
                10 => {
                    if sv.len() > 0 {
                        let idx = (_to_usize(GLOBAL_DATA, 340)) % sv.len();
                        println!("{:?}", &sv[idx]);
                    }
                }
                _ => {}
            }
        }
        let rlen = (_to_u8(GLOBAL_DATA, 356) % 17) as usize;
        let rs = _to_str(GLOBAL_DATA, 357, 357 + rlen);
        let range = CustomType4 {
            a: _to_usize(GLOBAL_DATA, 368),
            b: _to_usize(OTHER, 376),
            s: String::from(rs),
        };
        let mut dr = sv.drain(range);
        let steps = (_to_u8(GLOBAL_DATA, 392) % 20) as usize;
        for _ in 0..steps {
            let _ = dr.next();
        }
        for _ in 0..steps {
            let _ = dr.next_back();
        }
        std::mem::drop(dr);
        sv.shrink_to_fit();
        sv.truncate(_to_usize(GLOBAL_DATA, 396));
        let sl = sv.as_slice();
        if !sl.is_empty() {
            println!("{:?}", &sl[sl.len() - 1]);
        }
        let mut other_sv: SmallVec<[CustomType3; 32]> = SmallVec::from(sl);
        let mut dr2 = other_sv.drain(0.._to_usize(OTHER, 404));
        let _ = dr2.next();
        std::mem::drop(dr2);
        let clone_sv = sv.clone();
        let _ = sv.partial_cmp(&clone_sv);
        let _ = sv.cmp(&clone_sv);
        let _ = sv.eq(&clone_sv);
        let _ = sv.capacity();
        let _ = sv.len();
        let _ = sv.is_empty();
        let mut it = sv.clone().into_iter();
        let its = it.as_slice();
        if !its.is_empty() {
            println!("{:?}", &its[0]);
        }
        let _ = it.next();
        let _ = it.next_back();
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