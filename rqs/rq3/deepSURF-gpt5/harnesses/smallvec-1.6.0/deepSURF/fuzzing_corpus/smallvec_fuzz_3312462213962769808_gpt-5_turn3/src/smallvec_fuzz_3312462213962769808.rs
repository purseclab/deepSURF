#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
		if data.len() < 512 { return; }
		set_global_data(data);
		let global_data = get_global_data();
		let GLOBAL_DATA = global_data.first_half;
		let SECOND = global_data.second_half;
		if GLOBAL_DATA.len() < 16 || SECOND.len() < 16 { return; }
		let mut base_count = (_to_u8(GLOBAL_DATA, 0) % 65) as usize;
		let mut v: Vec<CustomType1> = Vec::with_capacity(64);
		let mut i = 0usize;
		while i < base_count {
			let idx = ((i as usize) * 8) % (GLOBAL_DATA.len() - 8);
			v.push(CustomType1(_to_usize(GLOBAL_DATA, idx)));
			i += 1;
		}
		let s = &v[..];
		let ctor_sel = _to_u8(GLOBAL_DATA, 3) % 5;
		let mut sv_pre: SmallVec<[CustomType1; 32]> = match ctor_sel {
			0 => SmallVec::new(),
			1 => SmallVec::with_capacity(_to_usize(GLOBAL_DATA, 5)),
			2 => SmallVec::from_vec(v.clone()),
			3 => SmallVec::from_elem(CustomType1(_to_usize(GLOBAL_DATA, 13)), _to_usize(GLOBAL_DATA, 21)),
			_ => SmallVec::from_slice(s),
		};
		let _ = sv_pre.capacity();
		let _ = sv_pre.is_empty();
		let _ = sv_pre.len();
		sv_pre.reserve(_to_usize(SECOND, 0));
		let _ = sv_pre.try_reserve_exact(_to_usize(SECOND, 8));
		sv_pre.resize_with(_to_usize(SECOND, 16), || CustomType1(_to_usize(SECOND, 24)));
		sv_pre.dedup_by(|a, b| {
			let _ = a.0 ^ b.0;
			_to_bool(SECOND, 32)
		});
		sv_pre.retain(|x| {
			let _ = x.0;
			_to_bool(SECOND, 40)
		});
		sv_pre.extend_from_slice(s);
		let rpre = sv_pre.as_slice();
		if let Some(x) = rpre.get(0) { println!("{:?}", *x); }
		let mut sv = SmallVec::<[CustomType1; 32]>::from_slice(s);
		let r1 = sv.as_slice();
		if let Some(x) = r1.get(0) { println!("{:?}", *x); }
		let r2 = <SmallVec<[CustomType1; 32]> as Deref>::deref(&sv);
		if let Some(x) = r2.get(1) { println!("{:?}", *x); }
		let r3 = sv.as_mut_slice();
		if !r3.is_empty() {
			r3[0] = CustomType1(_to_usize(SECOND, 48));
			let _l = r3.len();
			println!("{}", _l);
		}
		sv.insert_from_slice(_to_usize(SECOND, 56), s);
		sv.extend_from_slice(s);
		sv.insert(_to_usize(SECOND, 64), CustomType1(_to_usize(SECOND, 72)));
		let _ = sv.remove(_to_usize(SECOND, 80));
		sv.push(CustomType1(_to_usize(SECOND, 88)));
		let _ = sv.pop();
		let _ = sv.swap_remove(_to_usize(SECOND, 96));
		let _ = sv.capacity();
		let _ = sv.len();
		let _ = sv.is_empty();
		sv.try_grow(_to_usize(SECOND, 104)).ok();
		sv.grow(_to_usize(SECOND, 112));
		sv.truncate(_to_usize(SECOND, 120));
		let _ = sv.partial_cmp(&sv_pre);
		let _ = sv.cmp(&sv_pre);
		let mut sv2 = sv.clone();
		let _b = sv2.into_boxed_slice();
		let _ = sv.clone().into_inner();
		let _: SmallVec<[CustomType1; 16]> = s.to_smallvec();
		let range_end = _to_usize(SECOND, 128);
		{
			let mut d = sv.drain(0..range_end);
			let _ = d.next();
			let _ = d.next_back();
		}
		let _ = sv.as_slice();
		let _ = sv.as_mut_slice();
		sv.append(&mut sv_pre);
		if sv.len() > 0 {
			let r = &sv[0];
			println!("{:?}", *r);
			let rmut = &mut sv[0];
			rmut.0 = rmut.0.wrapping_add(1);
			println!("{:?}", *rmut);
		}
		let _p = sv.as_ptr();
		let mut ops = (_to_u8(SECOND, 136) % 20) as usize;
		let mut j = 0usize;
		while j < ops {
			let code = _to_u8(SECOND, (140 + j) % (SECOND.len() - 1));
			match code % 10 {
				0 => sv.push(CustomType1(_to_usize(SECOND, (144 + j * 2) % (SECOND.len() - 8)))),
				1 => { let _ = sv.pop(); },
				2 => sv.insert(_to_usize(SECOND, (148 + j * 2) % (SECOND.len() - 8)), CustomType1(_to_usize(SECOND, (152 + j * 2) % (SECOND.len() - 8)))),
				3 => { let _ = sv.remove(_to_usize(SECOND, (156 + j * 2) % (SECOND.len() - 8))); },
				4 => sv.extend_from_slice(s),
				5 => sv.truncate(_to_usize(SECOND, (160 + j * 2) % (SECOND.len() - 8))),
				6 => { let rs = sv.as_slice(); if let Some(x) = rs.get(0) { println!("{:?}", *x); } },
				7 => { let rms = sv.as_mut_slice(); if !rms.is_empty() { rms[0] = CustomType1(_to_usize(SECOND, (164 + j * 2) % (SECOND.len() - 8))); } },
				8 => { let _ = sv.swap_remove(_to_usize(SECOND, (168 + j * 2) % (SECOND.len() - 8))); },
				_ => { let _ = sv.try_reserve(_to_usize(SECOND, (172 + j * 2) % (SECOND.len() - 8))); },
			}
			j += 1;
		}
		let arr16 = [CustomType1(_to_usize(GLOBAL_DATA, 80)); 16];
		let _sv_from_buf = SmallVec::<[CustomType1; 16]>::from_buf(arr16);
		let arr32 = [CustomType1(_to_usize(GLOBAL_DATA, 88)); 32];
		let _sv_from_buf_and_len = SmallVec::<[CustomType1; 32]>::from_buf_and_len(arr32, _to_usize(SECOND, 180));
		let _ = SmallVec::<[CustomType1; 32]>::from_slice(s);
		let _ = SmallVec::<[CustomType1; 32]>::from_vec(v.clone());
		let _ = SmallVec::<[CustomType1; 32]>::from_elem(CustomType1(_to_usize(GLOBAL_DATA, 96)), _to_usize(SECOND, 188));
		let _ = SmallVec::<[CustomType1; 32]>::with_capacity(_to_usize(GLOBAL_DATA, 104));
		let _ = SmallVec::<[CustomType1; 32]>::new();
		let vec_out = sv.clone().into_vec();
		println!("{}", vec_out.len());
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