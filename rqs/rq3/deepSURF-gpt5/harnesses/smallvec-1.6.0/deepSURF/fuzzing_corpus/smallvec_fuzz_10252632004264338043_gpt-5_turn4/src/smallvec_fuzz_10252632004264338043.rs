#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};
use std::borrow::{Borrow, BorrowMut};

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq)]
struct CustomType1(usize);

impl core::clone::Clone for CustomType1 {
	fn clone(&self) -> Self {
		let global_data = get_global_data();
		let GLOBAL_DATA = global_data.first_half;
		let custom_impl_num = _to_usize(GLOBAL_DATA, 10);
		let custom_impl_inst_num = self.0;
		let selector = (custom_impl_num + custom_impl_inst_num) % 3;
		if selector == 0{
			panic!("INTENTIONAL PANIC!");
		}
		let GLOBAL_DATA = match selector{
				1 => global_data.first_half,
				_ => global_data.second_half,
		};
		let t_4 = _to_usize(GLOBAL_DATA, 18);
		let t_5 = CustomType1(t_4);
		return t_5;
	}
}

impl core::marker::Copy for CustomType1 {}

fn main() {
	fuzz_nohook!(|data: &[u8]| {
		if data.len() < 1024 { return; }
		set_global_data(data);
		let gd = get_global_data();
		let first = gd.first_half;
		let second = gd.second_half;
		let base_len = (_to_u8(first, 2) % 60) as usize;
		let mut base = std::vec::Vec::with_capacity(64);
		for i in 0..base_len {
			let v = _to_usize(first, 16 + i * 8);
			base.push(CustomType1(v));
		}
		let slice = &base[..];
		let mut sv: SmallVec<[CustomType1; 36]> = match _to_u8(first, 4) % 4 {
			0 => {
				let mut s = SmallVec::<[CustomType1; 36]>::new();
				s.extend_from_slice(slice);
				s
			}
			1 => {
				let cap = _to_usize(first, 64);
				SmallVec::<[CustomType1; 36]>::with_capacity(cap)
			}
			2 => SmallVec::<[CustomType1; 36]>::from_slice(slice),
			_ => smallvec::ToSmallVec::<[CustomType1; 36]>::to_smallvec(slice),
		};
		if !sv.is_empty() {
			let _ = sv.as_slice();
		}
		let c = sv.capacity();
		let l = sv.len();
		println!("{:?}{:?}", c, l);
		if base_len > 0 {
			let p0 = CustomType1(_to_usize(second, 8));
			sv.push(p0);
		}
		let idx_ins = _to_usize(second, 16);
		let el_ins = CustomType1(_to_usize(first, 72));
		if (_to_u8(first, 5) & 1) == 1 {
			let _ = sv.try_reserve(_to_usize(second, 24));
			let _ = sv.try_reserve_exact(_to_usize(second, 32));
			sv.reserve(_to_usize(second, 40));
			sv.reserve_exact(_to_usize(second, 48));
			sv.grow(_to_usize(first, 80));
			let _ = sv.try_grow(_to_usize(first, 88));
		}
		if (_to_u8(first, 6) & 1) == 0 {
			if base.len() > 0 {
				sv.extend_from_slice(&base[..]);
			}
		}
		if (_to_u8(first, 7) & 1) == 1 {
			sv.insert(idx_ins, el_ins);
		}
		if (_to_u8(first, 8) & 1) == 1 {
			let _ = sv.pop();
		}
		let mut s2 = SmallVec::<[CustomType1; 64]>::from_vec(base.clone());
		sv.append(&mut s2);
		let ms = sv.as_mut_slice();
		println!("{:?}", &*ms);
		let steps = (_to_u8(second, 0) % 12) as usize;
		for i in 0..steps {
			match _to_u8(second, 1 + i) % 12 {
				0 => {
					let v = CustomType1(_to_usize(second, 64 + i * 8));
					sv.push(v);
				}
				1 => {
					let r = sv.as_slice();
					println!("{:?}", r);
					let b: &[CustomType1] = (&sv).borrow();
					println!("{:?}", b);
				}
				2 => {
					let idx = _to_usize(second, 128 + i * 8);
					let _ = sv.remove(idx);
				}
				3 => {
					sv.truncate(_to_usize(second, 192 + i * 8));
				}
				4 => {
					let new_len = _to_usize(second, 256 + i * 8);
					let val = CustomType1(_to_usize(first, 96 + i * 8));
					sv.resize(new_len, val);
				}
				5 => {
					let idx = _to_usize(second, 320 + i * 8);
					let v = CustomType1(_to_usize(first, 160 + i * 8));
					sv.insert(idx, v);
				}
				6 => {
					let idx = _to_usize(second, 384 + i * 8);
					let _ = sv.swap_remove(idx);
				}
				7 => {
					sv.dedup();
				}
				8 => {
					let mut off = 448 + i * 8;
					sv.retain(|x| {
						println!("{:?}", x);
						let b = _to_bool(second, off % second.len());
						off = off.wrapping_add(1);
						b
					});
				}
				9 => {
					let mut off = 512 + i * 4;
					sv.dedup_by(|a, b| {
						println!("{:?}{:?}", a, b);
						let b = (_to_u8(second, off % second.len()) & 1) == 1;
						off = off.wrapping_add(1);
						b
					});
				}
				10 => {
					let new_len = _to_usize(second, 576 + i * 8);
					let mut off = 640 + i * 4;
					sv.resize_with(new_len, || {
						let v = _to_usize(second, off % second.len());
						off = off.wrapping_add(1);
						CustomType1(v)
					});
				}
				_ => {
					let idx = _to_usize(second, 704 + i * 8);
					let _ = &sv[idx];
					let r = sv.as_ref();
					println!("{:?}", r);
				}
			}
		}
		let b1: &[CustomType1] = (&sv).borrow();
		println!("{:?}", b1);
		let ar = sv.as_ref();
		println!("{:?}", ar);
		let d = (&sv).deref();
		println!("{:?}", d);
		let bm: &mut [CustomType1] = sv.borrow_mut();
		println!("{:?}", &*bm);
		let end = _to_usize(second, 768);
		{
			let mut dr = sv.drain(0..end);
			let _ = dr.next();
			let _ = dr.next_back();
		}
		let _ = sv.partial_cmp(&sv.clone());
		let _ = sv.cmp(&sv.clone());
		let _ = sv.eq(&sv.clone());
		let vec2 = sv.clone().into_vec();
		println!("{:?}", vec2.len());
		let bx = sv.clone().into_boxed_slice();
		println!("{:?}", bx.len());
		let cap2 = sv.capacity();
		let len2 = sv.len();
		println!("{:?}{:?}", cap2, len2);
		let _ = sv.index(0..0);
		let ms2 = sv.as_mut();
		println!("{:?}", &*ms2);
		let sl = sv.as_slice();
		let sv_from = SmallVec::<[CustomType1; 36]>::from(sl);
		let b2: &[CustomType1] = (&sv_from).borrow();
		println!("{:?}", b2);
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