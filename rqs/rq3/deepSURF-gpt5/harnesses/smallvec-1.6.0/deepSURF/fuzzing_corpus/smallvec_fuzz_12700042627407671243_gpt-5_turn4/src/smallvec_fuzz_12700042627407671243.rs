#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

struct CustomType4(String, usize, usize);
struct CustomType1(String);
struct CustomType0(String);
struct CustomType2(String);
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct CustomType3(String);

impl core::ops::RangeBounds<usize> for CustomType4 {
    fn start_bound(&self) -> core::ops::Bound<&usize> {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 588);
        let custom_impl_inst_num = self.0.len();
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0 {
            panic!("INTENTIONAL PANIC!");
        }
        let _GLOBAL_DATA = match selector {
            1 => global_data.first_half,
            _ => global_data.second_half,
        };
        core::ops::Bound::Excluded(&self.1)
    }

    fn end_bound(&self) -> core::ops::Bound<&usize> {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 613);
        let custom_impl_inst_num = self.0.len();
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0 {
            panic!("INTENTIONAL PANIC!");
        }
        let _GLOBAL_DATA = match selector {
            1 => global_data.first_half,
            _ => global_data.second_half,
        };
        core::ops::Bound::Included(&self.2)
    }
}

impl core::clone::Clone for CustomType3 {
    fn clone(&self) -> Self {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 19);
        let custom_impl_inst_num = self.0.len();
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0 {
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector {
            1 => global_data.first_half,
            _ => global_data.second_half,
        };
        let mut t_6 = _to_u8(GLOBAL_DATA, 27) % 17;
        let t_7 = _to_str(GLOBAL_DATA, 28, 28 + t_6 as usize);
        let t_8 = String::from(t_7);
        let t_9 = CustomType3(t_8);
        t_9
    }
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 1310 {
            return;
        }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let mut base_len = _to_u8(GLOBAL_DATA, 18) % 65;
        let mut v = std::vec::Vec::with_capacity(64);
        let g_len = GLOBAL_DATA.len();
        let mut seed = _to_usize(GLOBAL_DATA, 1) % (g_len.saturating_sub(32));
        let mut i = 0usize;
        while i < base_len as usize {
            let l0 = (_to_u8(GLOBAL_DATA, 40 + i) % 17) as usize;
            let start0 = (seed + i) % (g_len - (l0 + 1));
            let s0 = _to_str(GLOBAL_DATA, start0, start0 + l0);
            v.push(CustomType3(String::from(s0)));

            let l1 = (_to_u8(GLOBAL_DATA, 80 + i) % 17) as usize;
            let start1 = (seed + i + 13) % (g_len - (l1 + 1));
            let s1 = _to_str(GLOBAL_DATA, start1, start1 + l1);
            v.push(CustomType3(String::from(s1)));

            let l2 = (_to_u8(GLOBAL_DATA, 120 + i) % 17) as usize;
            let start2 = (seed + i + 23) % (g_len - (l2 + 1));
            let s2 = _to_str(GLOBAL_DATA, start2, start2 + l2);
            v.push(CustomType3(String::from(s2)));
            i += 3;
        }
        v.truncate(base_len as usize);

        let slice_all = &v[..];
        let mut sv_from_slice = smallvec::SmallVec::<[CustomType3; 32]>::from(slice_all);
        let mut sv_from_vec = smallvec::SmallVec::<[CustomType3; 64]>::from_vec(v.clone());
        let mut sv_with_cap = smallvec::SmallVec::<[CustomType3; 16]>::with_capacity(_to_usize(GLOBAL_DATA, 200));
        let mut sv_new = smallvec::SmallVec::<[CustomType3; 36]>::new();

        if !sv_from_slice.is_empty() {
            let s = sv_from_slice.as_slice();
            if let Some(first) = s.get(0) {
                println!("{:?}", first);
            }
        }

        sv_with_cap.reserve(_to_usize(GLOBAL_DATA, 220));
        let grow_to = _to_usize(GLOBAL_DATA, 236);
        sv_with_cap.grow(grow_to);
        sv_with_cap.try_reserve(_to_usize(GLOBAL_DATA, 228)).ok();
        sv_with_cap.try_reserve_exact(_to_usize(GLOBAL_DATA, 229)).ok();
        sv_with_cap.reserve_exact(_to_usize(GLOBAL_DATA, 230));

        if !sv_from_vec.is_empty() {
            let idx_i = _to_usize(GLOBAL_DATA, 252);
            let l = (_to_u8(GLOBAL_DATA, 260) % 17) as usize;
            let sidx = (_to_usize(GLOBAL_DATA, 261) % (g_len.saturating_sub(33))) as usize;
            let s = _to_str(GLOBAL_DATA, sidx, sidx + l);
            sv_from_vec.insert(idx_i, CustomType3(String::from(s)));
        }

        let mut extra = smallvec::SmallVec::<[CustomType3; 32]>::from_iter(slice_all.iter().cloned());
        sv_new.append(&mut extra);

        if sv_from_slice.len() > 0 {
            let r = &sv_from_slice[0];
            println!("{:?}", r);
        }
        if sv_from_vec.len() > 0 {
            let s_mut = sv_from_vec.as_mut_slice();
            let l = (_to_u8(GLOBAL_DATA, 300) % 17) as usize;
            let sidx = (_to_usize(GLOBAL_DATA, 301) % (g_len.saturating_sub(33))) as usize;
            let s = _to_str(GLOBAL_DATA, sidx, sidx + l);
            if let Some(first_mut) = s_mut.get_mut(0) {
                *first_mut = CustomType3(String::from(s));
            }
        }

        let rlen = (_to_u8(GLOBAL_DATA, 638) % 17) as usize;
        let rstart = 639usize;
        let rstr = _to_str(GLOBAL_DATA, rstart, rstart + rlen);
        let bstart1 = _to_usize(GLOBAL_DATA, 600);
        let bend1 = _to_usize(GLOBAL_DATA, 608);
        let range1 = CustomType4(String::from(rstr), bstart1, bend1);
        let mut drain1 = sv_from_slice.drain(range1);
        drain1.next_back();

        let mut cnt = (_to_u8(GLOBAL_DATA, 310) % 10) as usize;
        while cnt > 0 {
            let choice = _to_u8(GLOBAL_DATA, 311 + cnt as usize);
            match choice % 6 {
                0 => {
                    drain1.next();
                    drain1.next_back();
                }
                1 => {
                    let sz = _to_usize(GLOBAL_DATA, 320 + cnt as usize);
                    sv_with_cap.truncate(sz);
                }
                2 => {
                    if !sv_with_cap.is_empty() {
                        let idx_rm = _to_usize(GLOBAL_DATA, 340 + cnt as usize);
                        let _ = sv_with_cap.remove(idx_rm);
                    }
                }
                3 => {
                    if !sv_from_vec.is_empty() {
                        let slice_ref = sv_from_vec.as_ref();
                        if let Some(item) = slice_ref.get(0) {
                            println!("{:?}", item);
                        }
                    }
                }
                4 => {
                    sv_new.extend(slice_all.iter().cloned());
                }
                _ => {
                    sv_with_cap.retain(|e| {
                        let n = e.0.len();
                        let b = _to_bool(GLOBAL_DATA, 360);
                        if b {
                            n % 2 == 0
                        } else {
                            n % 3 == 0
                        }
                    });
                }
            }
            cnt -= 1;
        }

        drop(drain1);

        let rlen2 = (_to_u8(GLOBAL_DATA, 350) % 17) as usize;
        let rstart2 = 351usize;
        let rstr2 = _to_str(GLOBAL_DATA, rstart2, rstart2 + rlen2);
        let bstart2 = _to_usize(GLOBAL_DATA, 370);
        let bend2 = _to_usize(GLOBAL_DATA, 378);
        let range2 = CustomType4(String::from(rstr2), bstart2, bend2);
        let mut drain2 = sv_from_vec.drain(range2);
        drain2.next_back();
        drain2.next();

        drop(drain2);

        let _ = sv_from_vec.partial_cmp(&sv_from_vec.clone());
        let _ = sv_from_slice.cmp(&sv_from_slice.clone());
        let _ = sv_with_cap.eq(&sv_new);

        let mut into_it = sv_new.clone().into_iter();
        let _ = into_it.next();
        let _ = into_it.next_back();

        let boxed = sv_with_cap.clone().into_boxed_slice();
        println!("{}", boxed.len());

        let vec_conv = sv_with_cap.clone().into_vec();
        println!("{}", vec_conv.len());

        let t_small: smallvec::SmallVec<[CustomType3; 32]> = smallvec::SmallVec::from(slice_all);
        println!("{}", t_small.len());

        let _ = sv_from_slice.capacity();
        let _ = sv_from_vec.is_empty();
        let _ = sv_with_cap.len();
        sv_from_vec.shrink_to_fit();
        sv_with_cap.clear();
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