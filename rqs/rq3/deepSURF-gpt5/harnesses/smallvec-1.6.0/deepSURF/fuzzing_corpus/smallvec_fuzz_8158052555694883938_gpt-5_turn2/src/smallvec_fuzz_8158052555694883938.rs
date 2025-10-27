#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

#[derive(Debug)]
struct CustomType0(String);
#[derive(Debug, PartialEq)]
struct CustomType3(String);
#[derive(Debug)]
struct CustomType4(String);

impl core::clone::Clone for CustomType3 {
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
		let t_9 = CustomType3(t_8);
		return t_9;
	}
}

impl core::ops::RangeBounds<usize> for CustomType4 {
	fn start_bound(&self) -> core::ops::Bound<&usize> {
		let global_data = get_global_data();
		let GLOBAL_DATA = global_data.first_half;
		let custom_impl_num = _to_usize(GLOBAL_DATA, 588);
		let custom_impl_inst_num = self.0.len();
		let selector = (custom_impl_num + custom_impl_inst_num) % 3;
		if selector == 0{
			panic!("INTENTIONAL PANIC!");
		}
		let GLOBAL_DATA = match selector{
				1 => global_data.first_half,
				_ => global_data.second_half,
		};
		let _t_141 = _to_u8(GLOBAL_DATA, 596) % 17;
		let _t_142 = _to_str(GLOBAL_DATA, 597, 597 + _t_141 as usize);
		let _t_143 = String::from(_t_142);
		let _t_144 = CustomType0(_t_143);
		std::mem::drop(_t_144);
		core::ops::Bound::Unbounded
	}
	fn end_bound(&self) -> core::ops::Bound<&usize> {
		let global_data = get_global_data();
		let GLOBAL_DATA = global_data.first_half;
		let custom_impl_num = _to_usize(GLOBAL_DATA, 613);
		let custom_impl_inst_num = self.0.len();
		let selector = (custom_impl_num + custom_impl_inst_num) % 3;
		if selector == 0{
			panic!("INTENTIONAL PANIC!");
		}
		let GLOBAL_DATA = match selector{
				1 => global_data.first_half,
				_ => global_data.second_half,
		};
		let _t_147 = _to_u8(GLOBAL_DATA, 621) % 17;
		let _t_148 = _to_str(GLOBAL_DATA, 622, 622 + _t_147 as usize);
		let _t_149 = String::from(_t_148);
		let _t_150 = CustomType0(_t_149);
		std::mem::drop(_t_150);
		core::ops::Bound::Unbounded
	}
}

fn main() {
	fuzz_nohook!(|data: &[u8]| {
		if data.len() < 1310 { return; }
		set_global_data(data);
		let global_data = get_global_data();
		let first = global_data.first_half;
		let second = global_data.second_half;

		let mut base_len = _to_u8(first, 0) % 65;
		let mut src: Vec<CustomType3> = Vec::with_capacity(64);
		let fh_len = first.len();
		for i in 0..base_len {
			let l = (_to_u8(first, 10 + i as usize) % 17) as usize;
			let bound = if fh_len > l + 1 { fh_len - (l + 1) } else { 0 };
			let start_seed = 32 + ((i as usize) % 4) * 8;
			let start = if bound > 0 { _to_usize(first, start_seed) % bound } else { 0 };
			let s = _to_str(first, start, start + l);
			src.push(CustomType3(String::from(s)));
		}

		let sel = _to_u8(first, 64) % 6;
		let mut sv: SmallVec<[CustomType3; 32]> = match sel {
			0 => SmallVec::from_vec(src.clone()),
			1 => SmallVec::<[CustomType3; 32]>::from(&src[..]),
			2 => { let mut t = SmallVec::<[CustomType3; 32]>::new(); t.extend(src.iter().cloned()); t },
			3 => { let cap = _to_usize(first, 72); let mut t = SmallVec::<[CustomType3; 32]>::with_capacity(cap); if let Some(v) = src.get(0) { t.push(v.clone()); } t },
			4 => {
				let l2 = (_to_u8(first, 80) % 17) as usize;
				let bound2 = if fh_len > l2 + 1 { fh_len - (l2 + 1) } else { 0 };
				let start2 = if bound2 > 0 { _to_usize(first, 84) % bound2 } else { 0 };
				let elem = CustomType3(String::from(_to_str(first, start2, start2 + l2)));
				let n = _to_usize(first, 92);
				SmallVec::<[CustomType3; 32]>::from_elem(elem, n)
			}
			_ => {
				let sl = &src[..];
				SmallVec::<[CustomType3; 32]>::from(&sl[..])
			}
		};

		let add1 = _to_usize(first, 100);
		sv.reserve(add1);
		let add2 = _to_usize(first, 108);
		let _ = sv.try_reserve(add2);
		let add3 = _to_usize(first, 116);
		let _ = sv.try_reserve_exact(add3);

		let sh_len = second.len();
		let l3 = (_to_u8(second, 0) % 17) as usize;
		let bound3 = if sh_len > l3 + 1 { sh_len - (l3 + 1) } else { 0 };
		let start3 = if bound3 > 0 { _to_usize(second, 8) % bound3 } else { 0 };
		let elem_a = CustomType3(String::from(_to_str(second, start3, start3 + l3)));
		sv.push(elem_a.clone());
		let _ = sv.pop();

		let idx_ins = _to_usize(first, 124);
		let l4 = (_to_u8(second, 16) % 17) as usize;
		let bound4 = if sh_len > l4 + 1 { sh_len - (l4 + 1) } else { 0 };
		let start4 = if bound4 > 0 { _to_usize(second, 24) % bound4 } else { 0 };
		let elem_b = CustomType3(String::from(_to_str(second, start4, start4 + l4)));
		let _ = std::panic::catch_unwind(core::panic::AssertUnwindSafe(|| { sv.insert(idx_ins, elem_b.clone()); }));

		let idx_rem = _to_usize(first, 132);
		let _ = std::panic::catch_unwind(core::panic::AssertUnwindSafe(|| { let _ = sv.remove(idx_rem); }));

		let sref = sv.as_slice();
		if !sref.is_empty() {
			let r0 = &sref[0];
			println!("{:?}", r0);
		}
		{
			let mref = sv.as_mut_slice();
			if !mref.is_empty() {
				let r0 = &mut mref[0];
				r0.0.push_str("x");
				println!("{:?}", r0);
			}
		}
		{
			let dref: &[CustomType3] = sv.deref();
			if !dref.is_empty() {
				let last = &dref[dref.len() - 1];
				println!("{:?}", last);
			}
		}

		let mut op_count = (_to_u8(first, 140) % 10) as usize;
		while op_count > 0 {
			let which = _to_u8(first, 141 + op_count) % 8;
			match which {
				0 => { let _ = sv.swap_remove(_to_usize(first, 200 + op_count)); }
				1 => { sv.truncate(_to_usize(first, 220 + op_count)); }
				2 => { sv.grow(_to_usize(first, 240 + op_count)); }
				3 => {
					let l5 = (_to_u8(second, 40 + op_count as usize) % 17) as usize;
					let bound5 = if sh_len > l5 + 1 { sh_len - (l5 + 1) } else { 0 };
					let start5 = if bound5 > 0 { _to_usize(second, 48 + (op_count % 4) as usize * 8) % bound5 } else { 0 };
					let e = CustomType3(String::from(_to_str(second, start5, start5 + l5)));
					let new_len = _to_usize(first, 260 + op_count);
					let _ = std::panic::catch_unwind(core::panic::AssertUnwindSafe(|| { sv.resize(new_len, e); }));
				}
				4 => {
					let keep = _to_bool(first, 300 + op_count);
					sv.retain(|v| {
						let cond = (v.0.len() + op_count) % 2 == 0;
						if keep { cond } else { !cond }
					});
				}
				5 => { sv.dedup(); }
				6 => {
					let l6 = (_to_u8(second, 80 + op_count as usize) % 17) as usize;
					let bound6 = if sh_len > l6 + 1 { sh_len - (l6 + 1) } else { 0 };
					let start6 = if bound6 > 0 { _to_usize(second, 88 + (op_count % 4) as usize * 8) % bound6 } else { 0 };
					let e = CustomType3(String::from(_to_str(second, start6, start6 + l6)));
					let idx_many = _to_usize(first, 320 + op_count);
					let iterable = vec![e.clone(), e.clone()];
					let _ = std::panic::catch_unwind(core::panic::AssertUnwindSafe(|| { sv.insert_many(idx_many, iterable); }));
				}
				_ => {
					let l7 = (_to_u8(first, 360 + op_count as usize) % 5) as usize;
					let bound7 = if fh_len > l7 + 1 { fh_len - (l7 + 1) } else { 0 };
					let start7 = if bound7 > 0 { _to_usize(first, 368 + (op_count % 4) as usize * 8) % bound7 } else { 0 };
					let e = CustomType3(String::from(_to_str(first, start7, start7 + l7)));
					let n = _to_usize(first, 400 + op_count);
					let mut extra = SmallVec::<[CustomType3; 32]>::from_elem(e, n);
					sv.append(&mut extra);
				}
			}
			op_count -= 1;
		}

		let mut t_153 = _to_u8(first, 638) % 17;
		let t_154 = _to_str(first, 639, 639 + t_153 as usize);
		let t_155 = String::from(t_154);
		let t_156 = CustomType4(t_155);

		let how = _to_u8(first, 500);
		match how % 3 {
			0 => {
				let mut dr = sv.drain(t_156);
				let iters = (_to_u8(second, 32) % 10) as usize;
				for _ in 0..iters {
					if let Some(x) = dr.next() {
						println!("{:?}", x);
					}
				}
				let iters2 = (_to_u8(second, 33) % 10) as usize;
				for _ in 0..iters2 {
					let _ = dr.next_back();
				}
			}
			1 => {
				let end = _to_usize(first, 508);
				let mut dr = sv.drain(0..end);
				let iters = (_to_u8(second, 34) % 10) as usize;
				for _ in 0..iters {
					let _ = dr.next();
				}
			}
			_ => {
				let start = _to_usize(first, 516);
				let end = _to_usize(first, 524);
				let mut dr = sv.drain(start..end);
				let iters = (_to_u8(second, 35) % 10) as usize;
				for _ in 0..iters {
					let _ = dr.next();
				}
				let _ = dr.next_back();
			}
		}

		let slice_again = sv.as_slice();
		if let Some(r) = slice_again.get(0) { println!("{:?}", r); }

		let mut it = sv.clone().into_iter();
		let _ = it.next();
		let rem = it.as_slice();
		if let Some(r) = rem.get(0) { println!("{:?}", r); }
		let _ = it.next_back();
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