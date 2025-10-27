#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};
use std::borrow::{Borrow, BorrowMut};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct CustomType1(String);
struct CustomType3(String);
struct CustomType2(String);

impl core::iter::Iterator for CustomType3 {
	type Item = CustomType1;
	
	fn size_hint(&self) -> (usize, Option<usize>) {
		let global_data = get_global_data();
		let GLOBAL_DATA = global_data.first_half;
		let custom_impl_num = _to_usize(GLOBAL_DATA, 9);
		let custom_impl_inst_num = self.0.len();
		let selector = (custom_impl_num + custom_impl_inst_num) % 3;
		if selector == 0{
			panic!("INTENTIONAL PANIC!");
		}
		let GLOBAL_DATA = match selector{
				1 => global_data.first_half,
				_ => global_data.second_half,
		};
		let t_2 = _to_usize(GLOBAL_DATA, 17);
		let t_3 = _to_usize(GLOBAL_DATA, 25);
		let t_4 = Some(t_3);
		let t_5 = (t_2, t_4);
		return t_5;
	}
	
	fn next(&mut self) -> Option<Self::Item> {
		let global_data = get_global_data();
		let GLOBAL_DATA = global_data.first_half;
		let custom_impl_num = _to_usize(GLOBAL_DATA, 33);
		let custom_impl_inst_num = self.0.len();
		let selector = (custom_impl_num + custom_impl_inst_num) % 3;
		if selector == 0{
			panic!("INTENTIONAL PANIC!");
		}
		let GLOBAL_DATA = match selector{
				1 => global_data.first_half,
				_ => global_data.second_half,
		};
		let mut t_6 = _to_u8(GLOBAL_DATA, 41) % 17;
		let t_7 = _to_str(GLOBAL_DATA, 42, 42 + t_6 as usize);
		let t_8 = String::from(t_7);
		let t_9 = CustomType1(t_8);
		let t_10 = Some(t_9);
		return t_10;
	}
}

impl core::iter::IntoIterator for CustomType2 {
	type Item = CustomType1;
	type IntoIter = CustomType3;
	
	fn into_iter(self) -> Self::IntoIter {
		let global_data = get_global_data();
		let GLOBAL_DATA = global_data.first_half;
		let custom_impl_num = _to_usize(GLOBAL_DATA, 58);
		let custom_impl_inst_num = self.0.len();
		let selector = (custom_impl_num + custom_impl_inst_num) % 3;
		if selector == 0{
			panic!("INTENTIONAL PANIC!");
		}
		let GLOBAL_DATA = match selector{
				1 => global_data.first_half,
				_ => global_data.second_half,
		};
		let mut t_11 = _to_u8(GLOBAL_DATA, 66) % 17;
		let t_12 = _to_str(GLOBAL_DATA, 67, 67 + t_11 as usize);
		let t_13 = String::from(t_12);
		let t_14 = CustomType3(t_13);
		return t_14;
	}
}

fn main() {
	fuzz_nohook!(|data: &[u8]| {
		if data.len() < 460 { return; }
		set_global_data(data);
		let global = get_global_data();
		let fh = global.first_half;
		let sh = global.second_half;

		let mut v_from_vec_src: Vec<CustomType1> = Vec::new();
		let n1 = (_to_u8(sh, 2) % 65) as usize;
		for i in 0..n1 {
			let l = _to_u8(sh, 10 + (i % 50)) % 17;
			let start = 100 + ((i * 7) % 40) as usize;
			let s = _to_str(sh, start, start + l as usize);
			v_from_vec_src.push(CustomType1(String::from(s)));
		}
		let mut sv_a = SmallVec::<[CustomType1; 32]>::from_vec(v_from_vec_src);

		let mut slice_src: Vec<CustomType1> = Vec::new();
		let n2 = (_to_u8(fh, 50) % 20) as usize;
		for i in 0..n2 {
			let l = _to_u8(fh, 55 + (i % 30)) % 17;
			let start = 120 + ((i * 5) % 30) as usize;
			let s = _to_str(fh, start, start + l as usize);
			slice_src.push(CustomType1(String::from(s)));
		}
		let slice_ref = slice_src.as_slice();
		let mut sv_b = SmallVec::<[CustomType1; 16]>::from(slice_ref);

		let cap_c = _to_usize(fh, 60);
		let mut sv_c: SmallVec<[CustomType1; 16]> = SmallVec::with_capacity(cap_c);
		let mut sv_d: SmallVec<[CustomType1; 16]> = SmallVec::new();

		let l0 = _to_u8(fh, 0) % 33;
		let seed_s = _to_str(fh, 1, 1 + l0 as usize);
		let ct2_seed = CustomType2(String::from(seed_s));
		let mut sv_target = SmallVec::<[CustomType1; 16]>::from_iter(ct2_seed);

		let sref = sv_target.as_slice();
		if !sref.is_empty() {
			let r = &sref[0];
			println!("{:?}", *r);
		}
		let mref = sv_target.as_mut_slice();
		if !mref.is_empty() {
			let r = &mut mref[0];
			println!("{:?}", *r);
		}
		let _cap = sv_target.capacity();
		let _len = sv_target.len();
		let _empty = sv_target.is_empty();
		let _spilled = sv_target.spilled();
		let _br = <SmallVec<[CustomType1; 16]> as Borrow<[CustomType1]>>::borrow(&sv_target).len();
		let _brm = <SmallVec<[CustomType1; 16]> as BorrowMut<[CustomType1]>>::borrow_mut(&mut sv_target).len();
		let _d1 = sv_target.deref().len();
		let _d2 = sv_target.deref_mut().len();

		let mut other_vec: Vec<CustomType1> = Vec::new();
		let n3 = (_to_u8(sh, 70) % 65) as usize;
		for i in 0..n3 {
			let l = _to_u8(sh, 75 + (i % 40)) % 17;
			let start = 140 + ((i * 3) % 30) as usize;
			let s = _to_str(sh, start, start + l as usize);
			other_vec.push(CustomType1(String::from(s)));
		}
		let mut sv_other = SmallVec::<[CustomType1; 32]>::from_vec(other_vec);
		sv_target.append(&mut sv_other);

		let ops = (_to_u8(fh, 70) % 12) as usize;
		for i in 0..ops {
			match _to_u8(fh, 71 + i) % 12 {
				0 => {
					let l = _to_u8(sh, 160 + (i % 10)) % 17;
					let start = 170 + (i % 10) as usize;
					let s = _to_str(sh, start, start + l as usize);
					sv_target.push(CustomType1(String::from(s)));
				}
				1 => {
					let _ = sv_target.pop();
				}
				2 => {
					let idx = _to_usize(fh, 80 + i);
					let l = _to_u8(fh, 90 + (i % 10)) % 17;
					let start = 100 + (i % 10) as usize;
					let s = _to_str(fh, start, start + l as usize);
					sv_target.insert(idx, CustomType1(String::from(s)));
				}
				3 => {
					let idx = _to_usize(fh, 90 + i);
					let _ = sv_target.remove(idx);
				}
				4 => {
					let new_len = _to_usize(sh, 100 + i);
					sv_target.truncate(new_len);
				}
				5 => {
					let add = _to_usize(fh, 110 + i);
					sv_target.reserve(add);
				}
				6 => {
					let add = _to_usize(fh, 120 + i);
					let _ = sv_target.try_reserve(add);
				}
				7 => {
					let add = _to_usize(sh, 130 + i);
					sv_target.reserve_exact(add);
				}
				8 => {
					let add = _to_usize(sh, 140 + i);
					let _ = sv_target.try_reserve_exact(add);
				}
				9 => {
					sv_target.extend(slice_ref.iter().cloned());
				}
				10 => {
					let start = _to_usize(sh, 150 + i);
					let end = _to_usize(sh, 160 + i);
					let mut dr = sv_target.drain(start..end);
					let _ = dr.next();
					let _ = dr.next_back();
				}
				_ => {
					let idx = _to_usize(fh, 170 + i);
					let l = _to_u8(fh, 175 + (i % 5)) % 17;
					let start = 180 + (i % 5) as usize;
					let s = _to_str(fh, start, start + l as usize);
					let it = CustomType2(String::from(s));
					sv_target.insert_many(idx, it);
				}
			}
		}

		sv_target.dedup_by(|a, b| {
			println!("{:?}", *a);
			println!("{:?}", *b);
			_to_bool(fh, 180)
		});
		sv_target.dedup_by_key(|x| x.0.len());
		sv_target.retain(|x| {
			println!("{:?}", *x);
			_to_bool(sh, 190)
		});

		let _eq = sv_target == sv_a;
		let _ord = sv_target.cmp(&sv_b);
		let _pord = sv_target.partial_cmp(&sv_c);

		let mut it = sv_target.clone().into_iter();
		let s = it.as_slice();
		if !s.is_empty() {
			println!("{:?}", s[0]);
		}
		let ms = it.as_mut_slice();
		if !ms.is_empty() {
			println!("{:?}", ms[0]);
		}
		let _ = it.next();
		let _ = it.next_back();

		let _vec_out = sv_target.clone().into_vec();
		let _boxed = sv_target.clone().into_boxed_slice();
		let _inner = sv_target.clone().into_inner();

		let grow_to = _to_usize(fh, 200);
		sv_target.grow(grow_to);
		let _ = sv_target.try_grow(_to_usize(sh, 205));

		let l = _to_u8(fh, 210) % 17;
		let s = _to_str(fh, 211, 211 + l as usize);
		let ext = CustomType2(String::from(s));
		sv_target.extend(ext);

		let _p = sv_target.as_ptr();
		let _mp = sv_target.as_mut_ptr();

		let idx_print = _to_usize(sh, 215);
		let _ = sv_target.index(idx_print);
		let idx_mut = _to_usize(sh, 216);
		let r = sv_target.index_mut(idx_mut);
		println!("{:?}", *r);

		let ts: SmallVec<[CustomType1; 32]> = SmallVec::from(slice_ref);
		let _ = sv_d.eq(&sv_c);
		let _ = sv_d.partial_cmp(&sv_c);
		let _ = sv_d.cmp(&sv_c);
		let _ = ts.len();
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