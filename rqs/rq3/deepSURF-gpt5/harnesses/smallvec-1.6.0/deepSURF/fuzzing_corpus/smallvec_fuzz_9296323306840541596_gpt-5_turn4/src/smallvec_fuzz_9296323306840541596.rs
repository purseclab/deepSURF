#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct CustomType1(String);
#[derive(Debug)]
struct CustomType2(String);

impl core::cmp::PartialEq for CustomType2 {
	fn eq(&self, _: &Self) -> bool {
		let global_data = get_global_data();
		let GLOBAL_DATA = global_data.first_half;
		let custom_impl_num = _to_usize(GLOBAL_DATA, 34);
		let custom_impl_inst_num = self.0.len();
		let selector = (custom_impl_num + custom_impl_inst_num) % 3;
		if selector == 0{
			panic!("INTENTIONAL PANIC!");
		}
		let GLOBAL_DATA = match selector{
				1 => global_data.first_half,
				_ => global_data.second_half,
		};
		let t_9 = _to_bool(GLOBAL_DATA, 42);
		return t_9;
	}
}

fn _custom_fn0(_: &mut CustomType1) -> CustomType2 {
	let global_data = get_global_data();
	let GLOBAL_DATA = global_data.first_half;
	let t_10 = _to_u8(GLOBAL_DATA, 43);
	if t_10 % 2 == 0{
		panic!("INTENTIONAL PANIC!");
	}
	let mut t_11 = _to_u8(GLOBAL_DATA, 44) % 17;
	let t_12 = _to_str(GLOBAL_DATA, 45, 45 + t_11 as usize);
	let t_13 = String::from(t_12);
	let t_14 = CustomType2(t_13);
	return t_14;
}

fn build_item_from_data() -> CustomType1 {
	let global_data = get_global_data();
	let GLOBAL_DATA = global_data.first_half;
	let mut l = (_to_u8(GLOBAL_DATA, 9) % 17) as usize;
	let start = 10usize;
	let end_cap = GLOBAL_DATA.len().saturating_sub(start);
	if l > end_cap { l = end_cap; }
	let s = _to_str(GLOBAL_DATA, start, start + l);
	CustomType1(String::from(s))
}

fn build_vec_from_data() -> Vec<CustomType1> {
	let global_data = get_global_data();
	let GLOBAL_DATA = global_data.first_half;
	let count = (_to_u8(GLOBAL_DATA, 12) % 65) as usize;
	let mut v = Vec::with_capacity(count);
	for _ in 0..count {
		v.push(build_item_from_data());
	}
	v
}

fn main() {
	fuzz_nohook!(|data: &[u8]| {
		if data.len() < 122 { return; }
		set_global_data(data);
		let global_data = get_global_data();
		let GLOBAL_DATA = global_data.first_half;

		let seed_vec = build_vec_from_data();
		let sel = (_to_u8(GLOBAL_DATA, 13) % 6) as usize;
		let mut sv: SmallVec<[CustomType1; 16]> = match sel {
			0 => {
				let mut tmp = SmallVec::<[CustomType1; 16]>::new();
				tmp.extend(seed_vec.clone());
				tmp
			}
			1 => {
				let mut tmp = SmallVec::<[CustomType1; 16]>::with_capacity(_to_usize(GLOBAL_DATA, 26));
				tmp.extend(seed_vec.clone());
				tmp
			}
			2 => SmallVec::<[CustomType1; 16]>::from_vec(seed_vec.clone()),
			3 => SmallVec::<[CustomType1; 16]>::from_iter(seed_vec.iter().cloned()),
			4 => SmallVec::<[CustomType1; 16]>::from_iter(seed_vec.clone().into_iter()),
			_ => {
				let elem = build_item_from_data();
				SmallVec::<[CustomType1; 16]>::from_elem(elem, _to_usize(GLOBAL_DATA, 34))
			}
		};

		let ops = (_to_u8(GLOBAL_DATA, 8) % 10) as usize + 1;
		for i in 0..ops {
			let op = (_to_u8(GLOBAL_DATA, 15 + (i % 8) as usize) % 8) as usize;
			match op {
				0 => {
					let extra = (_to_u8(GLOBAL_DATA, 14) % 7) as usize;
					for _ in 0..extra {
						sv.push(build_item_from_data());
					}
				}
				1 => {
					sv.reserve(_to_usize(GLOBAL_DATA, 26));
					let _ = sv.try_reserve(_to_usize(GLOBAL_DATA, 27));
					sv.reserve_exact(_to_usize(GLOBAL_DATA, 28));
					let _ = sv.try_reserve_exact(_to_usize(GLOBAL_DATA, 29));
				}
				2 => {
					let idx = _to_usize(GLOBAL_DATA, 30);
					sv.insert(idx, build_item_from_data());
				}
				3 => {
					let idx = _to_usize(GLOBAL_DATA, 31);
					let _ = sv.swap_remove(idx);
				}
				4 => {
					let idx = _to_usize(GLOBAL_DATA, 32);
					let _ = sv.remove(idx);
				}
				5 => {
					let add = build_vec_from_data();
					sv.extend(add.into_iter());
				}
				6 => {
					let mut keep = |x: &mut CustomType1| -> bool {
						let b = _to_bool(GLOBAL_DATA, 41);
						if b {
							x.0.push('k');
						}
						b
					};
					sv.retain(&mut keep);
				}
				_ => {
					let mut gen = || -> CustomType1 {
						let mut c = build_item_from_data();
						c.0.push('r');
						c
					};
					sv.resize_with(_to_usize(GLOBAL_DATA, 33), &mut gen);
				}
			}
		}

		let dref = sv.deref();
		println!("{:?}", dref.len());
		let drefm = sv.deref_mut();
		if !drefm.is_empty() {
			drefm[0].0.push('d');
		}
		let sref = sv.as_slice();
		if !sref.is_empty() {
			let r = &sref[0];
			println!("{:?}", *r);
		}
		let smref = sv.as_mut_slice();
		if !smref.is_empty() {
			let ch = _to_char(GLOBAL_DATA, 52);
			smref[0].0.push(ch);
		}
		if sv.len() > 0 {
			sv[0].0.push('z');
		}

		let mut key_fn = _custom_fn0;
		sv.dedup_by_key(&mut key_fn);

		let mut same_bucket = |a: &mut CustomType1, b: &mut CustomType1| -> bool {
			let gd = get_global_data();
			let G = gd.second_half;
			let t = _to_bool(G, 42);
			if t {
				a.0.push('a');
			} else {
				b.0.push('b');
			}
			t
		};
		sv.dedup_by(&mut same_bucket);

		let end = _to_usize(GLOBAL_DATA, 50);
		let mut dr = sv.drain(0..end);
		let _ = dr.next();
		let _ = dr.next_back();
		drop(dr);

		let vtmp = SmallVec::<[CustomType1; 16]>::from_iter(build_vec_from_data()).into_vec();
		let mut other = SmallVec::<[CustomType1; 16]>::from_vec(vtmp);
		let _ = sv.partial_cmp(&other);
		let _ = sv.cmp(&other);
		if !other.is_empty() {
			other.pop();
		}
		sv.append(&mut other);

		let _ = SmallVec::<[CustomType1; 16]>::from_iter(sv.clone().into_iter()).into_boxed_slice();
		let _ = usize::from_str("0");
		let mut again_key = _custom_fn0;
		sv.dedup_by_key(&mut again_key);

		sv.shrink_to_fit();
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