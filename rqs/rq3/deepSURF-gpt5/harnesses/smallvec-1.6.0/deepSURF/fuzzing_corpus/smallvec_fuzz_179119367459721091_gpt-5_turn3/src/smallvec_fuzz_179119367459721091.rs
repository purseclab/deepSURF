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

impl core::marker::Copy for CustomType1 {}

impl core::clone::Clone for CustomType1 {
	fn clone(&self) -> Self {
		let global_data = get_global_data();
		let GLOBAL_DATA = global_data.first_half;
		let custom_impl_num = _to_usize(GLOBAL_DATA, 35);
		let custom_impl_inst_num = self.0;
		let selector = (custom_impl_num + custom_impl_inst_num) % 3;
		if selector == 0 {
			panic!("INTENTIONAL PANIC!");
		}
		let GLOBAL_DATA = match selector {
			1 => global_data.first_half,
			_ => global_data.second_half,
		};
		let t_11 = _to_usize(GLOBAL_DATA, 43);
		let t_12 = CustomType1(t_11);
		return t_12;
	}
}

fn main() {
	fuzz_nohook!(|data: &[u8]| {
		if data.len() < 2000 { return; }
		set_global_data(data);
		let global_data = get_global_data();
		let FH = global_data.first_half;
		let SH = global_data.second_half;

		let mut v1 = std::vec::Vec::with_capacity(64);
		for i in 0..40usize {
			let idx = 16 + i * 8;
			let u = _to_usize(FH, idx);
			v1.push(CustomType1(u));
		}
		let l1 = (_to_u8(FH, 0) % 65) as usize;
		v1.truncate(l1);

		let mut v2 = std::vec::Vec::with_capacity(64);
		for i in 0..40usize {
			let idx = 24 + i * 8;
			let u = _to_usize(SH, idx);
			v2.push(CustomType1(u));
		}
		let l2 = (_to_u8(SH, 1) % 65) as usize;
		v2.truncate(l2);

		let slice1 = &v1[..];
		let slice2 = &v2[..];

		let sel = _to_u8(FH, 4) % 6;
		let mut sv: SmallVec<[CustomType1; 32]> = match sel {
			0 => SmallVec::new(),
			1 => SmallVec::with_capacity(_to_usize(SH, 40)),
			2 => SmallVec::from_vec(v1.clone()),
			3 => SmallVec::from_slice(slice2),
			4 => SmallVec::from_elem(CustomType1(_to_usize(FH, 48)), (_to_u8(SH, 56) % 65) as usize),
			_ => {
				let arr = [CustomType1(_to_usize(FH, 600)); 32];
				SmallVec::from_buf(arr)
			}
		};

		let cap = sv.capacity();
		println!("{}", cap);

		let pushes = (_to_u8(FH, 64) % 65) as usize;
		for i in 0..pushes {
			let idx = 200 + i * 8;
			let el = CustomType1(_to_usize(FH, idx));
			sv.push(el);
		}

		sv.reserve(_to_usize(SH, 88));
		let _ = sv.try_reserve(_to_usize(FH, 96));
		sv.insert(_to_usize(FH, 104), CustomType1(_to_usize(SH, 112)));

		let s_ref = sv.as_slice();
		if let Some(r0) = s_ref.get(0) {
			println!("{:?}", *r0);
		}

		let ms = sv.as_mut_slice();
		if !ms.is_empty() {
			let r0 = &mut ms[0];
			r0.0 = r0.0.wrapping_add(1);
			println!("{:?}", *r0);
		}

		let r = &sv[_to_usize(FH, 120)];
		println!("{:?}", *r);
		{
			let m = &mut sv[_to_usize(SH, 128)];
			m.0 = m.0.wrapping_add(2);
			println!("{:?}", *m);
		}

		sv.resize_with(_to_usize(FH, 136), || CustomType1(_to_usize(SH, 144)));

		let mut flip = _to_bool(FH, 152);
		sv.retain(|_| {
			flip = !flip;
			flip
		});

		sv.dedup_by(|a, b| {
			let cond = _to_bool(SH, 160);
			if cond {
				a.0 = a.0.wrapping_add(b.0);
			}
			cond
		});

		sv.extend_from_slice(slice1);

		let ops = (_to_u8(SH, 168) % 20) as usize;
		for i in 0..ops {
			let choice = _to_u8(FH, 176 + i) % 10;
			match choice {
				0 => {
					sv.push(CustomType1(_to_usize(FH, 184 + i * 8)));
				}
				1 => {
					let _ = sv.pop();
				}
				2 => {
					sv.extend_from_slice(slice2);
				}
				3 => {
					sv.insert(_to_usize(SH, 192 + i * 4), CustomType1(_to_usize(FH, 200 + i * 4)));
				}
				4 => {
					let _ = sv.remove(_to_usize(FH, 208 + i * 4));
				}
				5 => {
					sv.truncate(_to_usize(SH, 216 + i * 4));
				}
				6 => {
					let a = _to_usize(FH, 224 + i * 4);
					let b = _to_usize(SH, 232 + i * 4);
					let mut dr = sv.drain(a..b);
					let _ = dr.next();
					let _ = dr.next_back();
				}
				7 => {
					sv.reserve(_to_usize(FH, 240 + i * 4));
				}
				8 => {
					sv.insert_from_slice(_to_usize(SH, 248 + i * 4), if i % 2 == 0 { slice1 } else { slice2 });
				}
				_ => {
					let s = sv.as_slice();
					if let Some(rv) = s.get(_to_usize(FH, 256 + i * 4)) {
						println!("{:?}", *rv);
					}
				}
			}
		}

		let s_post = sv.as_slice();
		if let Some(rp) = s_post.get(1) {
			println!("{:?}", *rp);
		}

		let mut other: SmallVec<[CustomType1; 32]> = (&v2[..]).to_smallvec();
		println!("{}", other.len());
		sv.append(&mut other);

		sv.extend_from_slice(slice2);

		let vec_out = sv.clone().into_vec();
		let sv3 = SmallVec::<[CustomType1; 32]>::from_vec(vec_out);
		let eqv = SmallVec::eq(&sv, &sv3);
		println!("{}", eqv);
		let ord = SmallVec::cmp(&sv, &sv3);
		println!("{:?}", ord);

		let inner = sv3.into_inner();
		match inner {
			Ok(a) => {
				let _ = a.len();
			}
			Err(e) => {
				let _ = e.len();
			}
		}

		let bx = sv.clone().into_boxed_slice();
		println!("{}", bx.len());

		let asref = sv.as_ref();
		if let Some(rr) = asref.get(_to_usize(FH, 900)) {
			println!("{:?}", *rr);
		}

		let brr: &[CustomType1] = sv.borrow();
		println!("{}", brr.len());

		let bmut: &mut [CustomType1] = sv.borrow_mut();
		if !bmut.is_empty() {
			bmut[0].0 = bmut[0].0.wrapping_add(_to_usize(SH, 912));
			println!("{:?}", bmut[0]);
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