#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};
use std::borrow::{Borrow, BorrowMut};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
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

impl core::marker::Copy for CustomType1 {
}

fn main() {
	fuzz_nohook!(|data: &[u8]| {
		if data.len() < 256 { return; }
		set_global_data(data);
		let global_data = get_global_data();
		let FH = global_data.first_half;
		let SH = global_data.second_half;

		let n0 = _to_u8(FH, 0) % 65;
		let base = 8usize;
		let safe_count = std::cmp::min(n0 as usize, FH.len().saturating_sub(base + 8) / 8);
		let mut v0 = std::vec::Vec::with_capacity(64);
		for i in 0..safe_count {
			let idx = base + i * 8;
			let u = _to_usize(FH, idx);
			v0.push(CustomType1(u));
		}
		let s0 = &v0[..];

		let choice = _to_u8(FH, 1) % 6;
		let mut s_main: SmallVec<[CustomType1; 16]> = match choice {
			0 => SmallVec::<[CustomType1; 16]>::new(),
			1 => SmallVec::<[CustomType1; 16]>::with_capacity(_to_usize(SH, 8)),
			2 => SmallVec::<[CustomType1; 16]>::from_elem(CustomType1(_to_usize(FH, 4)), _to_usize(FH, 12)),
			3 => SmallVec::<[CustomType1; 16]>::from_slice(s0),
			4 => SmallVec::<[CustomType1; 16]>::from_vec(v0.clone()),
			_ => SmallVec::<[CustomType1; 16]>::from_iter(v0.clone()),
		};

		let mut sv_from_slice: SmallVec<[CustomType1; 32]> = s0.to_smallvec();

		let mut sv_b: SmallVec<[CustomType1; 32]> = SmallVec::with_capacity(_to_usize(SH, 16));
		s_main.append(&mut sv_b);
		if !sv_from_slice.is_empty() {
			let idxp = _to_usize(FH, 20);
			let valp = CustomType1(_to_usize(FH, 28));
			let _ = sv_from_slice.get(idxp);
			sv_from_slice.push(valp);
		}

		let mut clone_sink = sv_from_slice.clone();
		s_main.append(&mut clone_sink);

		let s_as_slice = s_main.as_slice();
		let mut sv_again: SmallVec<[CustomType1; 10]> = s_as_slice.to_smallvec();

		let ops = (_to_u8(SH, 2) % 10) as usize + 1;
		for i in 0..ops {
			let code = _to_u8(SH, 3 + i);
			match code % 12 {
				0 => {
					s_main.push(CustomType1(_to_usize(FH, 36)));
				}
				1 => {
					s_main.insert(_to_usize(FH, 44), CustomType1(_to_usize(FH, 52)));
				}
				2 => {
					let _ = s_main.pop();
				}
				3 => {
					let _ = s_main.remove(_to_usize(FH, 60));
				}
				4 => {
					let _ = s_main.swap_remove(_to_usize(FH, 68));
				}
				5 => {
					s_main.truncate(_to_usize(FH, 76));
				}
				6 => {
					s_main.reserve(_to_usize(FH, 84));
				}
				7 => {
					s_main.reserve_exact(_to_usize(FH, 92));
				}
				8 => {
					let _ = s_main.try_reserve(_to_usize(FH, 100));
					let _ = s_main.try_reserve_exact(_to_usize(FH, 108));
				}
				9 => {
					s_main.resize(_to_usize(FH, 116), CustomType1(_to_usize(FH, 24)));
				}
				10 => {
					s_main.resize_with(_to_usize(FH, 32), || CustomType1(_to_usize(SH, 24)));
				}
				_ => {
					s_main.retain(|x| {
						let _c = x.clone();
						_to_bool(FH, 124)
					});
				}
			}
		}

		let arr12 = [
			CustomType1(_to_usize(SH, 0)), CustomType1(_to_usize(SH, 8)), CustomType1(_to_usize(SH, 16)),
			CustomType1(_to_usize(SH, 24)), CustomType1(_to_usize(SH, 32)), CustomType1(_to_usize(SH, 40)),
			CustomType1(_to_usize(SH, 48)), CustomType1(_to_usize(SH, 56)), CustomType1(_to_usize(SH, 64)),
			CustomType1(_to_usize(SH, 72)), CustomType1(_to_usize(SH, 80)), CustomType1(_to_usize(SH, 88)),
		];
		s_main.insert_many(_to_usize(FH, 24), arr12.into_iter());

		{
			let end = _to_usize(SH, 32);
			let mut dr = s_main.drain(0..end);
			let _ = dr.next();
			let _ = dr.next_back();
		}

		let rs = s_main.as_slice();
		let idx_r = _to_usize(SH, 36);
		let r0 = &rs[idx_r];
		println!("{:?}", *r0);

		let rms = s_main.as_mut_slice();
		let k = _to_usize(SH, 40);
		rms[k] = CustomType1(_to_usize(SH, 48));

		{
			let drf: &[CustomType1] = s_main.deref();
			let i0 = _to_usize(SH, 52);
			println!("{:?}", drf[i0]);
		}
		{
			let dmf: &mut [CustomType1] = s_main.deref_mut();
			let i1 = _to_usize(SH, 60);
			dmf[i1] = CustomType1(_to_usize(SH, 68));
		}

		let b1: &[CustomType1] = s_main.borrow();
		let b2: &[CustomType1] = s_main.as_ref();
		let rbx = _to_usize(SH, 76);
		let rby = _to_usize(SH, 84);
		println!("{:?}", b1[rbx]);
		println!("{:?}", b2[rby]);
		{
			let bm1: &mut [CustomType1] = s_main.borrow_mut();
			let mbx = _to_usize(SH, 92);
			bm1[mbx] = CustomType1(_to_usize(FH, 40));
		}
		{
			let bm2 = s_main.as_mut();
			let mby = _to_usize(SH, 96);
			bm2[mby] = CustomType1(_to_usize(FH, 48));
		}

		s_main.insert_from_slice(_to_usize(SH, 56), s0);
		s_main.extend_from_slice(s0);

		s_main.grow(_to_usize(SH, 64));
		let _ = s_main.try_grow(_to_usize(SH, 72));

		let _ref_item = &s_main[_to_usize(SH, 80)];
		println!("{:?}", *_ref_item);
		s_main[_to_usize(SH, 88)] = CustomType1(_to_usize(FH, 56));

		let mut it = s_main.clone().into_iter();
		let _ = it.next();
		let _ = it.next_back();
		let rit = it.as_slice();
		let rit_idx = _to_usize(SH, 104);
		let rr = &rit[rit_idx];
		println!("{:?}", *rr);

		let v2 = s_main.clone().into_vec();
		let s2: SmallVec<[CustomType1; 18]> = SmallVec::from_vec(v2);
		let s3: SmallVec<[CustomType1; 10]> = s2.as_slice().to_smallvec();
		let _ = sv_again.cmp(&s3);
		let _ = sv_again.partial_cmp(&s3);
		let _ = sv_again.eq(&s3);

		let final_slice = s_main.as_slice();
		let _final_sv: SmallVec<[CustomType1; 14]> = final_slice.to_smallvec();
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