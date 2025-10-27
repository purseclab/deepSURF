#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};
use std::borrow::{Borrow, BorrowMut};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct CustomType1(String);
#[derive(Clone, Debug)]
struct CustomType2(String);
#[derive(Clone, Debug)]
struct CustomType3(String);

impl core::iter::IntoIterator for CustomType2 {
	type Item = CustomType1;
	type IntoIter = CustomType3;
	fn into_iter(self) -> Self::IntoIter {
		let global_data = get_global_data();
		let GLOBAL_DATA = global_data.first_half;
		let custom_impl_num = _to_usize(GLOBAL_DATA, 91);
		let custom_impl_inst_num = self.0.len();
		let selector = (custom_impl_num + custom_impl_inst_num) % 3;
		if selector == 0 {
			panic!("INTENTIONAL PANIC!");
		}
		let GLOBAL_DATA = match selector {
			1 => global_data.first_half,
			_ => global_data.second_half,
		};
		let mut t_19 = _to_u8(GLOBAL_DATA, 99) % 17;
		let t_20 = _to_str(GLOBAL_DATA, 100, 100 + t_19 as usize);
		let t_21 = String::from(t_20);
		let t_22 = CustomType3(t_21);
		t_22
	}
}

impl core::iter::Iterator for CustomType3 {
	type Item = CustomType1;
	fn next(&mut self) -> Option<Self::Item> {
		let global_data = get_global_data();
		let GLOBAL_DATA = global_data.first_half;
		let custom_impl_num = _to_usize(GLOBAL_DATA, 42);
		let custom_impl_inst_num = self.0.len();
		let selector = (custom_impl_num + custom_impl_inst_num) % 3;
		if selector == 0 {
			panic!("INTENTIONAL PANIC!");
		}
		let GLOBAL_DATA = match selector {
			1 => global_data.first_half,
			_ => global_data.second_half,
		};
		let mut t_10 = _to_u8(GLOBAL_DATA, 50) % 17;
		let t_11 = _to_str(GLOBAL_DATA, 51, 51 + t_10 as usize);
		let t_12 = String::from(t_11);
		let t_13 = CustomType1(t_12);
		let t_14 = Some(t_13);
		t_14
	}
	fn size_hint(&self) -> (usize, Option<usize>) {
		let global_data = get_global_data();
		let GLOBAL_DATA = global_data.first_half;
		let custom_impl_num = _to_usize(GLOBAL_DATA, 67);
		let custom_impl_inst_num = self.0.len();
		let selector = (custom_impl_num + custom_impl_inst_num) % 3;
		if selector == 0 {
			panic!("INTENTIONAL PANIC!");
		}
		let GLOBAL_DATA = match selector {
			1 => global_data.first_half,
			_ => global_data.second_half,
		};
		let t_15 = _to_usize(GLOBAL_DATA, 75);
		let t_16 = _to_usize(GLOBAL_DATA, 83);
		let t_17 = Some(t_16);
		let t_18 = (t_15, t_17);
		t_18
	}
}

fn main() {
	fuzz_nohook!(|data: &[u8]| {
		if data.len() < 600 { return; }
		set_global_data(data);
		let global_data = get_global_data();
		let G1 = global_data.first_half;
		let G2 = global_data.second_half;

		let base_len = (_to_u8(G1, 3) % 65) as usize;
		let mut base: Vec<CustomType1> = Vec::new();
		for i in 0..base_len {
			let l = (_to_u8(G1, 4 + (i % 8) as usize) % 17) as usize;
			let s = _to_str(G1, 10, 10 + l);
			base.push(CustomType1(String::from(s)));
		}

		let selector = (_to_u8(G1, 2) as usize) % 5;
		let mut sv: SmallVec<[CustomType1; 32]> = match selector {
			0 => SmallVec::new(),
			1 => SmallVec::with_capacity(_to_usize(G1, 26)),
			2 => SmallVec::from_vec(base.clone()),
			3 => SmallVec::from(&base[..]),
			_ => {
				let l = (_to_u8(G1, 32) % 17) as usize;
				let s = _to_str(G1, 33, 33 + l);
				let elem = base.get(0).cloned().unwrap_or(CustomType1(String::from(s)));
				SmallVec::from_elem(elem, (_to_u8(G1, 34) % 65) as usize)
			}
		};

		let _ = sv.try_reserve(_to_usize(G1, 40));
		let _ = sv.try_reserve_exact(_to_usize(G1, 44));
		sv.dedup();
		sv.dedup_by_key(|x| {
			let k = _to_u8(G1, 56) as usize;
			if k % 2 == 0 { x.0.push_str("a"); }
			k
		});
		sv.retain(|x| {
			let b = _to_bool(G1, 57);
			if b { x.0.push_str("b"); }
			b
		});
		sv.resize_with(_to_usize(G1, 58), || {
			let l = (_to_u8(G2, 60) % 17) as usize;
			let s = _to_str(G2, 61, 61 + l);
			CustomType1(String::from(s))
		});
		let ins_idx = _to_usize(G1, 64);
		let l2 = (_to_u8(G1, 65) % 17) as usize;
		let s2 = _to_str(G1, 66, 66 + l2);
		sv.insert(ins_idx, CustomType1(String::from(s2)));
		let lpush = (_to_u8(G1, 70) % 17) as usize;
		let spush = _to_str(G1, 71, 71 + lpush);
		sv.push(CustomType1(String::from(spush)));
		let _ = sv.pop();
		let cap = sv.capacity();
		println!("{}", cap);

		let r = sv.as_slice();
		if !r.is_empty() {
			println!("{:?}", &r[0]);
		}
		let mr = sv.as_mut_slice();
		if !mr.is_empty() {
			mr[0].0.push_str("m");
			println!("{:?}", &mr[0]);
		}
		let idx_for_index = _to_usize(G2, 72);
		if sv.len() > 0 {
			let rr = &sv[idx_for_index];
			println!("{:?}", rr);
		}

		let mut sv2: SmallVec<[CustomType1; 32]> = SmallVec::from(base.clone());
		sv2.extend(base.clone());
		sv2.truncate(_to_usize(G2, 74));
		let l3 = (_to_u8(G2, 75) % 17) as usize;
		let s3 = _to_str(G2, 76, 76 + l3);
		sv2.push(CustomType1(String::from(s3)));

		let tlen = (_to_u8(G2, 80) % 17) as usize;
		let ts = _to_str(G2, 81, 81 + tlen);
		let iter1 = CustomType2(String::from(ts));
		let idx_im1 = _to_usize(G2, 82);
		sv.insert_many(idx_im1, iter1);

		let mut vec_iter_src: Vec<CustomType1> = Vec::new();
		let vi_len = (_to_u8(G2, 84) % 65) as usize;
		for i in 0..vi_len {
			let ll = (_to_u8(G2, 85 + (i % 5) as usize) % 17) as usize;
			let ss = _to_str(G2, 90, 90 + ll);
			vec_iter_src.push(CustomType1(String::from(ss)));
		}
		let idx_im2 = _to_usize(G2, 96);
		sv.insert_many(idx_im2, vec_iter_src);

		let start = _to_usize(G2, 100);
		let end = _to_usize(G2, 108);
		let idx_im3 = _to_usize(G2, 116);
		let dr = sv2.drain(start..end);
		sv.insert_many(idx_im3, dr);

		let ops = (_to_u8(G2, 124) % 8) as usize;
		for i in 0..ops {
			match _to_u8(G2, 125 + i) % 8 {
				0 => {
					let idx = _to_usize(G1, 130 + i as usize);
					let len = (_to_u8(G1, 140 + (i % 3) as usize) % 17) as usize;
					let st = _to_str(G1, 150, 150 + len);
					sv.insert(idx, CustomType1(String::from(st)));
				}
				1 => {
					let idx = _to_usize(G1, 160 + i as usize);
					sv.remove(idx);
				}
				2 => {
					let idx = _to_usize(G1, 170 + i as usize);
					sv.swap_remove(idx);
				}
				3 => {
					let idx = _to_usize(G1, 180 + i as usize);
					let len = (_to_u8(G1, 190 + (i % 3) as usize) % 17) as usize;
					let st = _to_str(G1, 200, 200 + len);
					let temp = vec![CustomType1(String::from(st)); ((i as u8) % 2 + 1) as usize];
					sv.insert_many(idx, temp);
				}
				4 => {
					let l = (_to_u8(G1, 210 + (i % 2) as usize) % 17) as usize;
					let s = _to_str(G1, 220, 220 + l);
					sv.push(CustomType1(String::from(s)));
				}
				5 => {
					sv.shrink_to_fit();
				}
				6 => {
					sv.clear();
				}
				_ => {
					let idx = _to_usize(G2, 230 + i as usize);
					let l = (_to_u8(G2, 240 + (i % 2) as usize) % 17) as usize;
					let s = _to_str(G2, 250, 250 + l);
					let iter = vec![CustomType1(String::from(s)); ((i as u8) % 2 + 1) as usize];
					sv.insert_many(idx, iter);
				}
			}
		}

		let br: &[CustomType1] = sv.borrow();
		if !br.is_empty() {
			println!("{:?}", &br[br.len() - 1]);
		}
		let brm: &mut [CustomType1] = sv.borrow_mut();
		if !brm.is_empty() {
			brm[brm.len() - 1].0.push_str("z");
			println!("{:?}", &brm[brm.len() - 1]);
		}

		let cl = sv.clone();
		let _ = sv.partial_cmp(&cl);
		let _ = sv.cmp(&cl);

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