#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};
use std::hash::Hasher;
use std::borrow::{Borrow, BorrowMut};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct CustomType1(usize);
struct CustomType2(String);

impl core::marker::Copy for CustomType1 {}

impl core::clone::Clone for CustomType1 {
	fn clone(&self) -> Self {
		let global_data = get_global_data();
		let GLOBAL_DATA = global_data.first_half;
		let custom_impl_num = _to_usize(GLOBAL_DATA, 42);
		let custom_impl_inst_num = self.0;
		let selector = (custom_impl_num + custom_impl_inst_num) % 3;
		if selector == 0 {
			panic!("INTENTIONAL PANIC!");
		}
		let GLOBAL_DATA = match selector {
			1 => global_data.first_half,
			_ => global_data.second_half,
		};
		let t_5 = _to_usize(GLOBAL_DATA, 50);
		let t_6 = CustomType1(t_5);
		return t_6;
	}
}

impl core::hash::Hash for CustomType1 {
	fn hash<H: Hasher>(&self, _: &mut H) {
		let global_data = get_global_data();
		let GLOBAL_DATA = global_data.first_half;
		let custom_impl_num = _to_usize(GLOBAL_DATA, 0);
		let custom_impl_inst_num = self.0;
		let selector = (custom_impl_num + custom_impl_inst_num) % 3;
		if selector == 0 {
			panic!("INTENTIONAL PANIC!");
		}
		let GLOBAL_DATA = match selector {
			1 => global_data.first_half,
			_ => global_data.second_half,
		};
		return;
	}
}

impl core::hash::Hasher for CustomType2 {
	fn write(&mut self, _: &[u8]) {
		let global_data = get_global_data();
		let GLOBAL_DATA = global_data.first_half;
		let custom_impl_num = _to_usize(GLOBAL_DATA, 17);
		let custom_impl_inst_num = self.0.len();
		let selector = (custom_impl_num + custom_impl_inst_num) % 3;
		if selector == 0 {
			panic!("INTENTIONAL PANIC!");
		}
		let GLOBAL_DATA = match selector {
			1 => global_data.first_half,
			_ => global_data.second_half,
		};
		return;
	}
	fn finish(&self) -> u64 {
		let global_data = get_global_data();
		let GLOBAL_DATA = global_data.first_half;
		let custom_impl_num = _to_usize(GLOBAL_DATA, 25);
		let custom_impl_inst_num = self.0.len();
		let selector = (custom_impl_num + custom_impl_inst_num) % 3;
		if selector == 0 {
			panic!("INTENTIONAL PANIC!");
		}
		let GLOBAL_DATA = match selector {
			1 => global_data.first_half,
			_ => global_data.second_half,
		};
		let t_2 = _to_u64(GLOBAL_DATA, 33);
		return t_2;
	}
}

fn main() {
	fuzz_nohook!(|data: &[u8]| {
		if data.len() < 1216 { return; }
		set_global_data(data);
		let global_data = get_global_data();
		let GLOBAL_DATA = global_data.first_half;

		let mut base_vec = std::vec::Vec::with_capacity(64);
		let mut fill_len = _to_u8(GLOBAL_DATA, 5) % 65;
		let mut offs = 58usize;
		let mut pushes = 0usize;
		while pushes < 32 {
			let val = _to_usize(GLOBAL_DATA, offs);
			base_vec.push(CustomType1(val));
			offs += 8;
			pushes += 1;
		}
		base_vec.truncate(fill_len as usize);

		let ctor_sel = _to_u8(GLOBAL_DATA, 1) % 6;
		let mut sv: SmallVec<[CustomType1; 32]> = match ctor_sel {
			0 => {
				let mut v = SmallVec::<[CustomType1; 32]>::new();
				for x in &base_vec {
					v.push(*x);
				}
				v
			}
			1 => SmallVec::<[CustomType1; 32]>::from_slice(&base_vec),
			2 => SmallVec::<[CustomType1; 32]>::from_vec(base_vec.clone()),
			3 => ToSmallVec::<[CustomType1; 32]>::to_smallvec(&base_vec[..]),
			4 => {
				let cap = _to_usize(GLOBAL_DATA, 300);
				let mut v = SmallVec::<[CustomType1; 32]>::with_capacity(cap);
				if !base_vec.is_empty() {
					v.push(base_vec[0]);
				}
				v
			}
			_ => {
				let elem = CustomType1(_to_usize(GLOBAL_DATA, 308));
				let n = _to_usize(GLOBAL_DATA, 316);
				SmallVec::<[CustomType1; 32]>::from_elem(elem, n)
			}
		};

		let mut sv2 = SmallVec::<[CustomType1; 32]>::from_iter(base_vec.iter().cloned());
		let _is_empty = sv.is_empty();
		let _cap = sv.capacity();
		let slice_ref = sv.as_slice();
		if let Some(first) = slice_ref.get(0) {
			println!("{:?}", *first);
		}
		let deref_slice = sv.deref();
		if let Some(last) = deref_slice.get(deref_slice.len().wrapping_sub(1)) {
			println!("{:?}", *last);
		}
		let _eq = SmallVec::eq(&sv, &sv2);
		let _pcmp = SmallVec::partial_cmp(&sv, &sv2);
		let _cmp = SmallVec::cmp(&sv, &sv2);

		let mut it = sv.clone().into_iter();
		let it_slice = it.as_slice();
		if let Some(f) = it_slice.get(0) {
			println!("{:?}", *f);
		}
		let _ = it.next();
		let _ = it.next_back();

		let mut str_len = (_to_u8(GLOBAL_DATA, 330) % 31) as usize;
		let mut start = 315usize;
		if start + str_len >= GLOBAL_DATA.len() { str_len = GLOBAL_DATA.len().saturating_sub(start); }
		let s = _to_str(GLOBAL_DATA, start, start + str_len);
		let mut hasher = CustomType2(String::from(s));
		core::hash::Hash::hash(&sv, &mut hasher);

		let ops = (_to_u8(GLOBAL_DATA, 2) % 20) as usize;
		let mut i = 0usize;
		while i < ops {
			let opcode = _to_u8(GLOBAL_DATA, 340 + i) % 14;
			match opcode {
				0 => {
					let v = CustomType1(_to_usize(GLOBAL_DATA, 348 + i));
					sv.push(v);
				}
				1 => {
					let idx = _to_usize(GLOBAL_DATA, 360 + i);
					let v = CustomType1(_to_usize(GLOBAL_DATA, 368 + i));
					sv.insert(idx, v);
				}
				2 => {
					let idx = _to_usize(GLOBAL_DATA, 376 + i);
					let _ = sv.remove(idx);
				}
				3 => {
					let idx = _to_usize(GLOBAL_DATA, 384 + i);
					let _ = sv.swap_remove(idx);
				}
				4 => {
					let add = _to_usize(GLOBAL_DATA, 392 + i);
					sv.reserve(add);
				}
				5 => {
					let add = _to_usize(GLOBAL_DATA, 400 + i);
					let _ = sv.try_reserve(add);
				}
				6 => {
					let len = _to_usize(GLOBAL_DATA, 408 + i);
					sv.truncate(len);
				}
				7 => {
					let len = _to_usize(GLOBAL_DATA, 416 + i);
					let val = CustomType1(_to_usize(GLOBAL_DATA, 424 + i));
					sv.resize(len, val);
				}
				8 => {
					let len = _to_usize(GLOBAL_DATA, 432 + i);
					let mut toggle = _to_u8(GLOBAL_DATA, 440 + i);
					sv.resize_with(len, move || {
						toggle = toggle.wrapping_add(1);
						if toggle % 7 == 0 { panic!("INTENTIONAL PANIC!"); }
						CustomType1(toggle as usize)
					});
				}
				9 => {
					let mut t = sv.clone();
					let idx = _to_usize(GLOBAL_DATA, 448 + i);
					let end = _to_usize(GLOBAL_DATA, 456 + i);
					let _dr = t.drain(idx..end);
				}
				10 => {
					let mut tmp = std::vec::Vec::with_capacity(32);
					let cnt = _to_u8(GLOBAL_DATA, 464 + i) % 65;
					let mut j = 0usize;
					while j < cnt as usize {
						tmp.push(CustomType1(_to_usize(GLOBAL_DATA, 472 + j)));
						j += 1;
					}
					let idx = _to_usize(GLOBAL_DATA, 520 + i);
					sv.insert_many(idx, tmp);
				}
				11 => {
					let cnt = _to_u8(GLOBAL_DATA, 528 + i) % 65;
					let mut tmp = std::vec::Vec::with_capacity(32);
					let mut j = 0usize;
					while j < cnt as usize {
						tmp.push(CustomType1(_to_usize(GLOBAL_DATA, 536 + j)));
						j += 1;
					}
					sv.extend_from_slice(&tmp);
				}
				12 => {
					sv.retain(|x| {
						let n = _to_usize(GLOBAL_DATA, 544 + i);
						(x.0 ^ n) % 2 == 0
					});
				}
				_ => {
					sv.dedup_by(|a, b| {
						let n = _to_usize(GLOBAL_DATA, 552 + i);
						(a.0.wrapping_add(b.0)) == n
					});
				}
			}
			core::hash::Hash::hash(&sv, &mut hasher);
			i += 1;
		}

		let sref = sv.as_slice();
		let mut sv3 = ToSmallVec::<[CustomType1; 32]>::to_smallvec(sref);
		let _len = sv3.len();
		{
			let _borrow: &[CustomType1] = sv3.borrow();
		}
		{
			let _borrow_mut: &mut [CustomType1] = sv3.borrow_mut();
		}
		{
			let _as_mut = sv3.as_mut();
		}
		{
			let _as_mut_slice = sv3.as_mut_slice();
		}

		if sv3.len() > 0 {
			let r = &sv3[0];
			println!("{:?}", *r);
		}

		core::hash::Hash::hash(&sv3, &mut hasher);

		let mut sv_clone = sv3.clone();
		let _ = sv_clone.try_grow(_to_usize(GLOBAL_DATA, 560));
		let _ = sv_clone.try_reserve_exact(_to_usize(GLOBAL_DATA, 568));
		sv_clone.shrink_to_fit();
		let _bx = sv_clone.clone().into_boxed_slice();
		let _vec_back = sv_clone.clone().into_vec();
		let _inner = sv_clone.clone().into_inner();
		core::hash::Hash::hash(&sv_clone, &mut hasher);
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