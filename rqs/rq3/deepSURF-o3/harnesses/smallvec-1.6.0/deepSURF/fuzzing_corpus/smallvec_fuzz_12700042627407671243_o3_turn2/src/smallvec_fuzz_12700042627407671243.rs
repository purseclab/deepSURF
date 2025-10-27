#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::ops::RangeBounds;
use std::ops::Bound;

#[derive(Debug)]
struct CustomType3(String);
#[derive(Debug)]
struct CustomType0(String);
#[derive(Debug)]
struct CustomType4(String);
struct CustomType1(String);
struct CustomType2(String);

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
		let t_6 = _to_u8(GLOBAL_DATA, 27) % 17;
		let t_7 = _to_str(GLOBAL_DATA, 28, 28 + t_6 as usize);
		let t_8 = String::from(t_7);
		CustomType3(t_8)
	}
}

impl RangeBounds<usize> for CustomType4 {
	fn end_bound(&self) -> core::ops::Bound<&usize> {
		let global_data = get_global_data();
		let GLOBAL_DATA = global_data.first_half;
		let custom_impl_num = _to_usize(GLOBAL_DATA, 588);
		let custom_impl_inst_num = self.0.len();
		let selector = (custom_impl_num + custom_impl_inst_num) % 3;
		if selector == 0{
			panic!("INTENTIONAL PANIC!");
		}
		let _GLOBAL_DATA = match selector{
				1 => global_data.first_half,
				_ => global_data.second_half,
		};
		Bound::Unbounded
	}

	fn start_bound(&self) -> core::ops::Bound<&usize> {
		let global_data = get_global_data();
		let GLOBAL_DATA = global_data.first_half;
		let custom_impl_num = _to_usize(GLOBAL_DATA, 613);
		let custom_impl_inst_num = self.0.len();
		let selector = (custom_impl_num + custom_impl_inst_num) % 3;
		if selector == 0{
			panic!("INTENTIONAL PANIC!");
		}
		let _GLOBAL_DATA = match selector{
				1 => global_data.first_half,
				_ => global_data.second_half,
		};
		Bound::Unbounded
	}
}

fn build_custom_string(data: &[u8], idx: &mut usize) -> String {
	let len = _to_u8(data, *idx % data.len()) % 17;
	*idx = (*idx + 1) % data.len();
	let start = *idx;
	let end = (start + len as usize).min(data.len());
	*idx = end % data.len();
	String::from(_to_str(data, start, end))
}

fn smallvec_constructor(selector: u8, src: &Vec<CustomType3>) -> SmallVec<[CustomType3; 32]> {
	match selector % 4 {
		0 => SmallVec::<[CustomType3; 32]>::new(),
		1 => SmallVec::<[CustomType3; 32]>::from_vec(src.clone()),
		2 => SmallVec::<[CustomType3; 32]>::from_iter(src.clone().into_iter()),
		_ => {
			let mut v = SmallVec::<[CustomType3; 32]>::with_capacity(src.len());
			v.extend(src.clone().into_iter());
			v
		}
	}
}

fn main() {
	fuzz_nohook!(|data: &[u8]| {
		if data.len() < 1310 { return; }
		set_global_data(data);
		let global_data = get_global_data();
		let first = global_data.first_half;

		let mut cursor = 0usize;
		let initial_len = (_to_u8(first, cursor) % 65) as usize;
		cursor += 1;

		let mut backing_vec = Vec::with_capacity(initial_len);
		for _ in 0..initial_len {
			let s = build_custom_string(first, &mut cursor);
			backing_vec.push(CustomType3(s));
		}

		let constructor_selector = _to_u8(first, cursor);
		cursor += 1;
		let mut sv = smallvec_constructor(constructor_selector, &backing_vec);

		// Additional operations before first drain
		if !_to_bool(first, cursor % first.len()) {
			let extra_str = build_custom_string(first, &mut cursor);
			sv.push(CustomType3(extra_str));
		}
		cursor += 1;
		sv.reserve(_to_u8(first, cursor % first.len()) as usize);
		cursor += 1;

		let range_str = build_custom_string(first, &mut cursor);
		let range_token = CustomType4(range_str);
		{
			let mut drain_iter = sv.drain(range_token);
			drain_iter.next_back();
			drain_iter.next();
		}

		// Stress loop with varied operations
		let ops = (_to_u8(first, cursor % first.len()) % 20) as usize;
		cursor += 1;
		for i in 0..ops {
			match _to_u8(first, (cursor + i) % first.len()) % 6 {
				0 => {
					let s = build_custom_string(first, &mut cursor);
					sv.push(CustomType3(s));
				}
				1 => {
					sv.pop();
				}
				2 => {
					let token = CustomType4(build_custom_string(first, &mut cursor));
					let mut d = sv.drain(token);
					d.next_back();
					d.next();
				}
				3 => {
					let new_len = (_to_u8(first, (cursor + i + 3) % first.len()) % 65) as usize;
					sv.truncate(new_len);
				}
				4 => {
					let slice_ref = sv.as_slice();
					println!("{:?}", slice_ref.len());
				}
				_ => {
					let cap = sv.capacity();
					println!("{:?}", cap);
				}
			}
		}

		// Final interaction with Drain to ensure drop logic
		let token_final = CustomType4(build_custom_string(first, &mut cursor));
		let mut final_drain = sv.drain(token_final);
		while let Some(_item) = final_drain.next_back() {}
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