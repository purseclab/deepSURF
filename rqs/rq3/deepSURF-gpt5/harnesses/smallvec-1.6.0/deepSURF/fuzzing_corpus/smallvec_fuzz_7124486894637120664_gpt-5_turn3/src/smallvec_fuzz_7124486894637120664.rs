#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

#[derive(Debug, PartialEq)]
struct CustomType1(String);

impl core::clone::Clone for CustomType1 {
	
	fn clone(&self) -> Self {
		let global_data = get_global_data();
		let GLOBAL_DATA = global_data.first_half;
		let custom_impl_num = _to_usize(GLOBAL_DATA, 19);
		let custom_impl_inst_num = self.0.len();
		let selector = (custom_impl_num + custom_impl_inst_num) % 3;
		if selector == 0{
			panic!("INTENTIONAL PANIC!");
		}
		let GLOBAL_DATA = match selector{
				1 => global_data.first_half,
				_ => global_data.second_half,
		};
		let mut t_6 = _to_u8(GLOBAL_DATA, 27) % 17;
		let t_7 = _to_str(GLOBAL_DATA, 28, 28 + t_6 as usize);
		let t_8 = String::from(t_7);
		let t_9 = CustomType1(t_8);
		return t_9;
	}
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 512 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let FH = global_data.first_half;
        let SH = global_data.second_half;

        let mut v1 = std::vec::Vec::with_capacity(32);
        let c1 = (_to_u8(FH, 2) % 65) as usize;
        for i in 0..c1 {
            let l = (_to_u8(FH, 4 + (i % 20)) % 15) as usize;
            let base = 16 + (i * 3 % 200);
            let s = _to_str(FH, base, base + l);
            v1.push(CustomType1(String::from(s)));
        }

        let mut v2 = std::vec::Vec::with_capacity(32);
        let c2 = (_to_u8(SH, 2) % 65) as usize;
        for i in 0..c2 {
            let l = (_to_u8(SH, 4 + (i % 20)) % 15) as usize;
            let base = 16 + (i * 5 % 200);
            let s = _to_str(SH, base, base + l);
            v2.push(CustomType1(String::from(s)));
        }

        let sel = _to_u8(FH, 1) % 3;
        if sel == 0 {
            let mut a: SmallVec<[CustomType1; 16]> = match _to_u8(FH, 5) % 5 {
                0 => {
                    let mut tmp = SmallVec::<[CustomType1; 16]>::new();
                    for x in v1.iter().cloned().take((_to_u8(FH, 6) % 65) as usize) { tmp.push(x); }
                    tmp
                }
                1 => {
                    let mut tmp = SmallVec::<[CustomType1; 16]>::with_capacity(_to_usize(FH, 24));
                    let take_n = (_to_u8(FH, 7) % 65) as usize;
                    if take_n <= v1.len() { tmp.extend(v1[..take_n].iter().cloned()); }
                    tmp
                }
                2 => SmallVec::<[CustomType1; 16]>::from_vec(v1.clone()),
                3 => {
                    let take_n = (_to_u8(FH, 8) % 65) as usize;
                    if take_n <= v1.len() {
                        SmallVec::<[CustomType1; 16]>::from_vec(v1[..take_n].to_vec())
                    } else {
                        SmallVec::<[CustomType1; 16]>::new()
                    }
                }
                _ => {
                    let ln = (_to_u8(FH, 9) % 15) as usize;
                    let s = _to_str(FH, 40, 40 + ln);
                    SmallVec::<[CustomType1; 16]>::from_elem(CustomType1(String::from(s)), _to_usize(FH, 32))
                }
            };

            let mut b: SmallVec<[CustomType1; 32]> = match _to_u8(SH, 5) % 5 {
                0 => {
                    let mut tmp = SmallVec::<[CustomType1; 32]>::new();
                    for x in v2.iter().cloned().take((_to_u8(SH, 6) % 65) as usize) { tmp.push(x); }
                    tmp
                }
                1 => {
                    let mut tmp = SmallVec::<[CustomType1; 32]>::with_capacity(_to_usize(SH, 24));
                    let take_n = (_to_u8(SH, 7) % 65) as usize;
                    if take_n <= v2.len() { tmp.extend(v2[..take_n].iter().cloned()); }
                    tmp
                }
                2 => SmallVec::<[CustomType1; 32]>::from_vec(v2.clone()),
                3 => {
                    let take_n = (_to_u8(SH, 8) % 65) as usize;
                    if take_n <= v2.len() {
                        SmallVec::<[CustomType1; 32]>::from_vec(v2[..take_n].to_vec())
                    } else {
                        SmallVec::<[CustomType1; 32]>::new()
                    }
                }
                _ => {
                    let ln = (_to_u8(SH, 9) % 15) as usize;
                    let s = _to_str(SH, 40, 40 + ln);
                    SmallVec::<[CustomType1; 32]>::from_elem(CustomType1(String::from(s)), _to_usize(SH, 32))
                }
            };

            let _ = a.capacity();
            a.reserve(_to_usize(FH, 40));
            let _ = a.try_reserve(_to_usize(FH, 48));
            a.reserve_exact(_to_usize(FH, 56));
            let _ = a.try_reserve_exact(_to_usize(FH, 64));
            let aslc = a.as_slice();
            if let Some(r) = aslc.get(0) { println!("{:?}", r); }
            let _ = a.as_mut_slice();
            let el_len = (_to_u8(FH, 79) % 15) as usize;
            let el_str = _to_str(FH, 80, 80 + el_len);
            a.insert(_to_usize(FH, 72), CustomType1(String::from(el_str)));
            a.extend(b.as_slice().iter().cloned());
            a.retain(|e| { let keep = _to_bool(FH, 96); let _ = e.0.len(); keep });
            a.dedup();
            a.dedup_by(|x, y| { let _ = _to_bool(FH, 97); x.0 == y.0 });
            a.dedup_by_key(|x| { x.0.len() });
            a.resize_with(_to_usize(FH, 104), || {
                let l = (_to_u8(SH, 112) % 10) as usize;
                let s = _to_str(SH, 120, 120 + l);
                CustomType1(String::from(s))
            });
            a.truncate(_to_usize(FH, 128));
            let _ = &a[0];
            println!("{:?}", &a[0]);
            a.append(&mut b);
            let _ = a.len();
            let _ = a.is_empty();
            let sl = a.as_slice();
            if let Some(r) = sl.get(0) { println!("{:?}", r); }
            let mut d = a.drain(0.._to_usize(FH, 136));
            let _ = d.next();
            let _ = d.next_back();
            drop(d);
            let _ = a.pop();
            let _ = a.remove(_to_usize(FH, 144));
            let _ = a.swap_remove(_to_usize(FH, 152));
            a.shrink_to_fit();
            let mut v = a.into_vec();
            if !v.is_empty() { println!("{:?}", &v[0]); }
            let _ = SmallVec::<[CustomType1; 16]>::from_vec(v);
        } else if sel == 1 {
            let mut a: SmallVec<[CustomType1; 32]> = match _to_u8(FH, 10) % 4 {
                0 => SmallVec::<[CustomType1; 32]>::from_vec(v1.clone()),
                1 => {
                    let take_n = (_to_u8(FH, 11) % 65) as usize;
                    if take_n <= v1.len() {
                        SmallVec::<[CustomType1; 32]>::from_vec(v1[..take_n].to_vec())
                    } else {
                        SmallVec::<[CustomType1; 32]>::new()
                    }
                }
                2 => {
                    let mut tmp = SmallVec::<[CustomType1; 32]>::new();
                    for x in v1.iter().cloned().take((_to_u8(FH, 12) % 65) as usize) { tmp.push(x); }
                    tmp
                }
                _ => {
                    let ln = (_to_u8(FH, 13) % 15) as usize;
                    let s = _to_str(FH, 140, 140 + ln);
                    SmallVec::<[CustomType1; 32]>::from_elem(CustomType1(String::from(s)), _to_usize(FH, 36))
                }
            };

            let mut b: SmallVec<[CustomType1; 64]> = match _to_u8(SH, 10) % 4 {
                0 => SmallVec::<[CustomType1; 64]>::from_vec(v2.clone()),
                1 => {
                    let take_n = (_to_u8(SH, 11) % 65) as usize;
                    if take_n <= v2.len() {
                        SmallVec::<[CustomType1; 64]>::from_vec(v2[..take_n].to_vec())
                    } else {
                        SmallVec::<[CustomType1; 64]>::new()
                    }
                }
                2 => {
                    let mut tmp = SmallVec::<[CustomType1; 64]>::with_capacity(_to_usize(SH, 44));
                    let take_n = (_to_u8(SH, 12) % 65) as usize;
                    if take_n <= v2.len() { tmp.extend(v2[..take_n].iter().cloned()); }
                    tmp
                }
                _ => {
                    let ln = (_to_u8(SH, 13) % 15) as usize;
                    let s = _to_str(SH, 140, 140 + ln);
                    SmallVec::<[CustomType1; 64]>::from_elem(CustomType1(String::from(s)), _to_usize(SH, 36))
                }
            };

            a.reserve(_to_usize(FH, 160));
            let _ = a.try_reserve(_to_usize(FH, 168));
            a.reserve_exact(_to_usize(FH, 176));
            let _ = a.try_reserve_exact(_to_usize(FH, 184));
            let _ = a.capacity();
            a.extend(b.as_slice().iter().cloned());
            let l = (_to_u8(FH, 185) % 10) as usize;
            let s = _to_str(FH, 190, 190 + l);
            a.insert(_to_usize(FH, 200), CustomType1(String::from(s)));
            let _ = a.as_ptr();
            let _ = a.as_mut_ptr();
            a.append(&mut b);
            let _ = a.len();
            let mut it = a.clone().into_iter();
            let _ = it.next();
            let _ = it.next_back();
            let _ = it.as_slice();
            let _ = it.as_mut_slice();
            let _ = a.remove(_to_usize(FH, 208));
            a.clear();
            let _ = a.is_empty();
        } else {
            let mut a: SmallVec<[CustomType1; 64]> = {
                let take_n = (_to_u8(FH, 14) % 65) as usize;
                if take_n <= v1.len() {
                    SmallVec::<[CustomType1; 64]>::from_vec(v1[..take_n].to_vec())
                } else {
                    SmallVec::<[CustomType1; 64]>::new()
                }
            };

            let mut b: SmallVec<[CustomType1; 16]> = {
                let mut tmp = SmallVec::<[CustomType1; 16]>::new();
                for x in v2.iter().cloned().take ((_to_u8(SH, 14) % 65) as usize) { tmp.push(x); }
                tmp
            };

            a.grow(_to_usize(FH, 216));
            let _ = a.try_grow(_to_usize(FH, 224));
            let sref = a.as_slice();
            if let Some(r) = sref.get(0) { println!("{:?}", r); }
            let ms = a.as_mut_slice();
            if let Some(r) = ms.get_mut(0) { println!("{:?}", r); }
            println!("{:?}", &a[0]);
            a.append(&mut b);
            let _ = a.swap_remove(_to_usize(FH, 232));
            let _ = a.pop();
            a.truncate(_to_usize(FH, 240));
            let _ = a.len();
            let mut d = a.drain(0.._to_usize(FH, 248));
            let _ = d.next();
            drop(d);
            let v = a.into_vec();
            if !v.is_empty() { println!("{:?}", &v[0]); }
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