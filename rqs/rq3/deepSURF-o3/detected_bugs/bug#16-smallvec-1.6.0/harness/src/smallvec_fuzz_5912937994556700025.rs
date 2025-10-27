#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

#[derive(Clone, Debug, PartialEq)]
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
		let t_19 = _to_u8(GLOBAL_DATA, 99) % 17;
		let t_20 = _to_str(GLOBAL_DATA, 100, 100 + t_19 as usize);
		let t_21 = String::from(t_20);
		CustomType3(t_21)
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
		let t_10 = _to_u8(GLOBAL_DATA, 50) % 17;
		let t_11 = _to_str(GLOBAL_DATA, 51, 51 + t_10 as usize);
		let t_12 = String::from(t_11);
		let t_13 = CustomType1(t_12);
		Some(t_13)
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
		(t_15, Some(t_16))
	}
}

type SV = SmallVec<[CustomType1; 16]>;

fn main() {
	fuzz_nohook!(|data: &[u8]| {
		if data.len() < 550 {
			return;
		}
		set_global_data(data);
		let global_data = get_global_data();
		let GLOBAL_DATA = global_data.first_half;

		let selector = _to_u8(GLOBAL_DATA, 0) % 5;
		let base_len = _to_u8(GLOBAL_DATA, 1) % 17;
		let base_str = _to_str(GLOBAL_DATA, 2, 2 + base_len as usize);
		let base_elem = CustomType1(String::from(base_str));

		let mut sv: SV = match selector {
			0 => SV::new(),
			1 => {
				let cap = _to_usize(GLOBAL_DATA, 25);
				SV::with_capacity(cap)
			}
			2 => {
				let n = (_to_usize(GLOBAL_DATA, 33) % 65) as usize;
				SV::from_elem(base_elem.clone(), n)
			}
			3 => {
				let vec_len = (_to_u8(GLOBAL_DATA, 41) % 65) as usize;
				let mut tmp_vec = Vec::with_capacity(vec_len);
				for idx in 0..vec_len {
					let start = 42 + idx * 3;
					let l = _to_u8(GLOBAL_DATA, start) % 17;
					let s = _to_str(GLOBAL_DATA, start + 1, start + 1 + l as usize);
					tmp_vec.push(CustomType1(String::from(s)));
				}
				SV::from_vec(tmp_vec)
			}
			_ => {
				let vec_len = (_to_u8(GLOBAL_DATA, 170) % 65) as usize;
				let mut tmp_vec = Vec::with_capacity(vec_len);
				for idx in 0..vec_len {
					let start = 171 + idx * 3;
					let l = _to_u8(GLOBAL_DATA, start) % 17;
					let s = _to_str(GLOBAL_DATA, start + 1, start + 1 + l as usize);
					tmp_vec.push(CustomType1(String::from(s)));
				}
				SV::from_vec(tmp_vec)
			}
		};

		let pre_ops = _to_u8(GLOBAL_DATA, 200) % 10;
		for op_idx in 0..pre_ops {
			match _to_u8(GLOBAL_DATA, 201 + op_idx as usize) % 4 {
				0 => {
					let l = _to_u8(GLOBAL_DATA, 220 + op_idx as usize) % 17;
					let s = _to_str(
						GLOBAL_DATA,
						221 + op_idx as usize,
						221 + op_idx as usize + l as usize,
					);
					sv.push(CustomType1(String::from(s)));
				}
				1 => {
					sv.pop();
				}
				2 => {
					let idx = if sv.len() == 0 {
						0
					} else {
						_to_usize(GLOBAL_DATA, 260 + op_idx as usize) % sv.len()
					};
					let l = _to_u8(GLOBAL_DATA, 270 + op_idx as usize) % 17;
					let s = _to_str(
						GLOBAL_DATA,
						271 + op_idx as usize,
						271 + op_idx as usize + l as usize,
					);
					sv.insert(idx, CustomType1(String::from(s)));
				}
				_ => {
					let add = _to_usize(GLOBAL_DATA, 300 + op_idx as usize);
					sv.reserve(add);
				}
			}
		}

		let insert_index = if sv.len() == 0 {
			0
		} else {
			_to_usize(GLOBAL_DATA, 350) % sv.len()
		};
		let iter_len = _to_u8(GLOBAL_DATA, 351) % 17;
		let iter_str = _to_str(GLOBAL_DATA, 352, 352 + iter_len as usize);
		let iter_source = CustomType2(String::from(iter_str));
		sv.insert_many(insert_index, iter_source);

		let slice_ref = sv.as_slice();
		println!("{:?}", slice_ref.len());
		if !_to_bool(GLOBAL_DATA, 400) {
			let cap_after = sv.capacity();
			println!("{:?}", cap_after);
		}

		let aft_ops = _to_u8(GLOBAL_DATA, 401) % 10;
		for op_idx in 0..aft_ops {
			match _to_u8(GLOBAL_DATA, 402 + op_idx as usize) % 4 {
				0 => sv.clear(),
				1 => {
					if !sv.is_empty() {
						let idx = _to_usize(GLOBAL_DATA, 450 + op_idx as usize) % sv.len();
						sv.remove(idx);
					}
				}
				2 => {
					let new_len = (_to_usize(GLOBAL_DATA, 500 + op_idx as usize) % 65) as usize;
					let l = _to_u8(GLOBAL_DATA, 510 + op_idx as usize) % 17;
					let s = _to_str(
						GLOBAL_DATA,
						511 + op_idx as usize,
						511 + op_idx as usize + l as usize,
					);
					sv.resize(new_len, CustomType1(String::from(s)));
				}
				_ => sv.dedup(),
			}
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