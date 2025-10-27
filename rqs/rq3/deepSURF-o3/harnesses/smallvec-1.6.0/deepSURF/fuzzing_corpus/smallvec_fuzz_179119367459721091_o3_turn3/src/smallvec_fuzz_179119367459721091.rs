#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

#[derive(Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct CustomType1(usize);

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
		t_12
	}
}

fn build_vec(data: &[u8]) -> Vec<CustomType1> {
	let global_data = get_global_data();
	let GLOBAL_DATA = global_data.second_half;
	let len = (_to_u8(GLOBAL_DATA, 2) % 65) as usize;
	let mut v = Vec::with_capacity(len);
	for i in 0..len {
		let idx = (10 + i * 8) % GLOBAL_DATA.len();
		let val = _to_usize(GLOBAL_DATA, idx);
		v.push(CustomType1(val));
	}
	v
}

fn main() {
	fuzz_nohook!(|data: &[u8]| {
		if data.len() < 1024 {
			return;
		}
		set_global_data(data);
		let global_data = get_global_data();
		let GLOBAL_DATA = global_data.first_half;

		let mut backing_vec = build_vec(data);

		let constructor_sel = _to_u8(GLOBAL_DATA, 3) % 5;
		let elem_for_from_elem = CustomType1(_to_usize(GLOBAL_DATA, 500));
		let capacity_val = _to_usize(GLOBAL_DATA, 507);

		let mut small: SmallVec<[CustomType1; 32]> = match constructor_sel {
			0 => SmallVec::new(),
			1 => SmallVec::from_slice(&backing_vec[..]),
			2 => SmallVec::from_elem(elem_for_from_elem, backing_vec.len()),
			3 => SmallVec::from_vec(backing_vec.clone()),
			_ => SmallVec::with_capacity(capacity_val),
		};

		let operations = (_to_u8(GLOBAL_DATA, 4) % 20) as usize;
		for i in 0..operations {
			let op_sel = _to_u8(GLOBAL_DATA, 100 + i) % 12;
			match op_sel {
				0 => {
					small.extend_from_slice(&backing_vec[..]);
				}
				1 => {
					let val = CustomType1(_to_usize(GLOBAL_DATA, 200 + i));
					small.push(val);
				}
				2 => {
					small.pop();
				}
				3 => {
					let idx_val = _to_usize(GLOBAL_DATA, 300 + i);
					let val = CustomType1(_to_usize(GLOBAL_DATA, 304 + i));
					small.insert(idx_val, val);
				}
				4 => {
					let trunc_val = _to_usize(GLOBAL_DATA, 400 + i);
					small.truncate(trunc_val);
				}
				5 => {
					small.dedup();
				}
				6 => {
					let reserve_val = _to_usize(GLOBAL_DATA, 500 + i);
					small.reserve(reserve_val);
				}
				7 => {
					small.shrink_to_fit();
				}
				8 => {
					if !small.is_empty() {
						let rem_idx = _to_usize(GLOBAL_DATA, 600 + i) % small.len();
						small.remove(rem_idx);
					}
				}
				9 => {
					small.clear();
				}
				10 => {
					let tmp_clone = small.clone();
					small.extend_from_slice(tmp_clone.as_slice());
				}
				_ => {
					let _ = small.len();
				}
			}
		}

		let slice_ref = small.as_slice();
		if !slice_ref.is_empty() {
			println!("{:?}", slice_ref[0]);
		}

		let clone_small = small.clone();
		let _ord = small.cmp(&clone_small);

		let final_vec = small.clone().into_vec();
		println!("{:?}", final_vec.len());
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