#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

#[derive(Debug, Clone)]
struct CustomType0(String);
#[derive(Debug, Clone)]
struct CustomType1(String);
#[derive(Debug, Clone)]
struct CustomType2(String);
#[derive(Debug, Clone)]
struct CustomType3(String);
#[derive(Debug, Clone)]
struct CustomType4(usize, usize);

impl core::ops::RangeBounds<usize> for CustomType4 {
	
	fn start_bound(&self) -> core::ops::Bound<&usize> {
		let global_data = get_global_data();
		let GLOBAL_DATA = global_data.first_half;
		let custom_impl_num = _to_usize(GLOBAL_DATA, 588);
		let custom_impl_inst_num = self.0;
		let selector = (custom_impl_num + custom_impl_inst_num) % 3;
		if selector == 0{
			panic!("INTENTIONAL PANIC!");
		}
		core::ops::Bound::Excluded(&self.1)
	}
	
	fn end_bound(&self) -> core::ops::Bound<&usize> {
		let global_data = get_global_data();
		let GLOBAL_DATA = global_data.first_half;
		let custom_impl_num = _to_usize(GLOBAL_DATA, 613);
		let custom_impl_inst_num = self.0;
		let selector = (custom_impl_num + custom_impl_inst_num) % 3;
		if selector == 0{
			panic!("INTENTIONAL PANIC!");
		}
		core::ops::Bound::Included(&self.1)
	}
}

fn main (){
	fuzz_nohook!(|data: &[u8]| {
		if data.len() < 3000 {return;}
		set_global_data(data);
		let global_data = get_global_data();
		let GLOBAL_DATA = global_data.first_half;
		
		let num_operations = _to_u8(GLOBAL_DATA, 0) % 32;
		
		let constructor_selector = _to_u8(GLOBAL_DATA, 1) % 8;
		let mut smallvec = match constructor_selector {
			0 => smallvec::SmallVec::<[i32; 16]>::new(),
			1 => {
				let capacity = _to_usize(GLOBAL_DATA, 8);
				smallvec::SmallVec::<[i32; 16]>::with_capacity(capacity)
			},
			2 => {
				let elem = _to_i32(GLOBAL_DATA, 16);
				let n = _to_usize(GLOBAL_DATA, 20);
				smallvec::SmallVec::<[i32; 16]>::from_elem(elem, n)
			},
			3 => {
				let vec_size = _to_u8(GLOBAL_DATA, 28) % 65;
				let mut vec = Vec::with_capacity(vec_size as usize);
				for i in 0..vec_size {
					let elem = _to_i32(GLOBAL_DATA, 32 + (i as usize * 4));
					vec.push(elem);
				}
				smallvec::SmallVec::<[i32; 16]>::from_vec(vec)
			},
			4 => {
				let slice_size = _to_u8(GLOBAL_DATA, 292) % 65;
				let mut slice_vec = Vec::with_capacity(slice_size as usize);
				for i in 0..slice_size {
					let elem = _to_i32(GLOBAL_DATA, 296 + (i as usize * 4));
					slice_vec.push(elem);
				}
				smallvec::SmallVec::<[i32; 16]>::from_slice(&slice_vec[..])
			},
			5 => {
				let iter_size = _to_u8(GLOBAL_DATA, 500) % 65;
				let mut iter_vec = Vec::with_capacity(iter_size as usize);
				for i in 0..iter_size {
					let elem = _to_i32(GLOBAL_DATA, 504 + (i as usize * 4));
					iter_vec.push(elem);
				}
				smallvec::SmallVec::<[i32; 16]>::from_iter(iter_vec)
			},
			6 => {
				let buf_array: [i32; 16] = [
					_to_i32(GLOBAL_DATA, 600), _to_i32(GLOBAL_DATA, 604), _to_i32(GLOBAL_DATA, 608),
					_to_i32(GLOBAL_DATA, 612), _to_i32(GLOBAL_DATA, 616), _to_i32(GLOBAL_DATA, 620),
					_to_i32(GLOBAL_DATA, 624), _to_i32(GLOBAL_DATA, 628), _to_i32(GLOBAL_DATA, 632),
					_to_i32(GLOBAL_DATA, 636), _to_i32(GLOBAL_DATA, 640), _to_i32(GLOBAL_DATA, 644),
					_to_i32(GLOBAL_DATA, 648), _to_i32(GLOBAL_DATA, 652), _to_i32(GLOBAL_DATA, 656),
					_to_i32(GLOBAL_DATA, 660)
				];
				let len = _to_usize(GLOBAL_DATA, 664) % 17;
				smallvec::SmallVec::<[i32; 16]>::from_buf_and_len(buf_array, len)
			},
			_ => {
				let vec_from = Vec::<i32>::with_capacity(10);
				smallvec::SmallVec::<[i32; 16]>::from(vec_from)
			}
		};
		
		let mut index_offset = 700;
		
		for op_num in 0..num_operations {
			let operation = _to_u8(GLOBAL_DATA, index_offset) % 30;
			index_offset += 1;
			
			match operation {
				0 => {
					let elem = _to_i32(GLOBAL_DATA, index_offset);
					index_offset += 4;
					smallvec.push(elem);
				},
				1 => {
					let popped = smallvec.pop();
					if let Some(val) = popped {
						println!("{}", val);
					}
				},
				2 => {
					let capacity = _to_usize(GLOBAL_DATA, index_offset);
					index_offset += 8;
					smallvec.reserve(capacity);
				},
				3 => {
					let additional = _to_usize(GLOBAL_DATA, index_offset);
					index_offset += 8;
					let _ = smallvec.try_reserve(additional);
				},
				4 => {
					let range_start = _to_usize(GLOBAL_DATA, index_offset);
					index_offset += 8;
					let range_end = _to_usize(GLOBAL_DATA, index_offset);
					index_offset += 8;
					let custom_range = CustomType4(range_start, range_end);
					let mut drain_iter = smallvec.drain(custom_range);
					let drain_result = drain_iter.next();
					if let Some(val) = drain_result {
						println!("{}", val);
					}
					let drain_back = drain_iter.next_back();
					if let Some(val) = drain_back {
						println!("{}", val);
					}
				},
				5 => {
					smallvec.shrink_to_fit();
				},
				6 => {
					let len = _to_usize(GLOBAL_DATA, index_offset);
					index_offset += 8;
					smallvec.truncate(len);
				},
				7 => {
					smallvec.clear();
				},
				8 => {
					if !smallvec.is_empty() {
						let index = _to_usize(GLOBAL_DATA, index_offset);
						index_offset += 8;
						let removed = smallvec.remove(index % smallvec.len());
						println!("{}", removed);
					}
				},
				9 => {
					let index = _to_usize(GLOBAL_DATA, index_offset);
					index_offset += 8;
					let element = _to_i32(GLOBAL_DATA, index_offset);
					index_offset += 4;
					smallvec.insert(index % (smallvec.len() + 1), element);
				},
				10 => {
					let slice_ref = smallvec.as_slice();
					if !slice_ref.is_empty() {
						let elem = &slice_ref[slice_ref.len() - 1];
						println!("{}", *elem);
					}
				},
				11 => {
					let mut_slice_ref = smallvec.as_mut_slice();
					if !mut_slice_ref.is_empty() {
						let elem = &mut mut_slice_ref[0];
						*elem = _to_i32(GLOBAL_DATA, index_offset);
						index_offset += 4;
						println!("{}", *elem);
					}
				},
				12 => {
					let len = smallvec.len();
					let capacity = smallvec.capacity();
					let is_empty = smallvec.is_empty();
					println!("len: {}, capacity: {}, empty: {}", len, capacity, is_empty);
				},
				13 => {
					if !smallvec.is_empty() {
						let index = _to_usize(GLOBAL_DATA, index_offset);
						index_offset += 8;
						let swapped = smallvec.swap_remove(index % smallvec.len());
						println!("{}", swapped);
					}
				},
				14 => {
					let new_len = _to_usize(GLOBAL_DATA, index_offset);
					index_offset += 8;
					let value = _to_i32(GLOBAL_DATA, index_offset);
					index_offset += 4;
					smallvec.resize(new_len, value);
				},
				15 => {
					smallvec.dedup();
				},
				16 => {
					let retain_factor = _to_u8(GLOBAL_DATA, index_offset);
					index_offset += 1;
					smallvec.retain(|x| {
						if retain_factor % 3 == 0 {
							panic!("INTENTIONAL PANIC!");
						}
						*x % 2 == 0
					});
				},
				17 => {
					let extend_size = _to_u8(GLOBAL_DATA, index_offset) % 65;
					index_offset += 1;
					let mut extend_vec = Vec::with_capacity(extend_size as usize);
					for i in 0..extend_size {
						let elem = _to_i32(GLOBAL_DATA, index_offset);
						index_offset += 4;
						extend_vec.push(elem);
					}
					smallvec.extend(extend_vec);
				},
				18 => {
					let clone_selector = _to_u8(GLOBAL_DATA, index_offset);
					index_offset += 1;
					if clone_selector % 3 == 0 {
						panic!("INTENTIONAL PANIC!");
					}
					let cloned = smallvec.clone();
					println!("Cloned len: {}", cloned.len());
				},
				19 => {
					let into_vec = smallvec.clone().into_vec();
					println!("Into vec len: {}", into_vec.len());
				},
				20 => {
					if !smallvec.is_empty() {
						let index = _to_usize(GLOBAL_DATA, index_offset);
						index_offset += 8;
						let elem_ref = &smallvec[index % smallvec.len()];
						println!("Indexed elem: {}", *elem_ref);
					}
				},
				21 => {
					let exact_capacity = _to_usize(GLOBAL_DATA, index_offset);
					index_offset += 8;
					smallvec.reserve_exact(exact_capacity);
				},
				22 => {
					let slice_size = _to_u8(GLOBAL_DATA, index_offset) % 65;
					index_offset += 1;
					let mut slice_vec = Vec::with_capacity(slice_size as usize);
					for i in 0..slice_size {
						let elem = _to_i32(GLOBAL_DATA, index_offset);
						index_offset += 4;
						slice_vec.push(elem);
					}
					smallvec.extend_from_slice(&slice_vec[..]);
				},
				23 => {
					let insert_index = _to_usize(GLOBAL_DATA, index_offset);
					index_offset += 8;
					let slice_size = _to_u8(GLOBAL_DATA, index_offset) % 65;
					index_offset += 1;
					let mut slice_vec = Vec::with_capacity(slice_size as usize);
					for i in 0..slice_size {
						let elem = _to_i32(GLOBAL_DATA, index_offset);
						index_offset += 4;
						slice_vec.push(elem);
					}
					smallvec.insert_from_slice(insert_index % (smallvec.len() + 1), &slice_vec[..]);
				},
				24 => {
					let cmp_size = _to_u8(GLOBAL_DATA, index_offset) % 65;
					index_offset += 1;
					let mut cmp_vec = Vec::with_capacity(cmp_size as usize);
					for i in 0..cmp_size {
						let elem = _to_i32(GLOBAL_DATA, index_offset);
						index_offset += 4;
						cmp_vec.push(elem);
					}
					let other_smallvec = smallvec::SmallVec::<[i32; 16]>::from_vec(cmp_vec);
					let cmp_result = smallvec.cmp(&other_smallvec);
					println!("Comparison: {:?}", cmp_result);
					let eq_result = smallvec.eq(&other_smallvec);
					println!("Equal: {}", eq_result);
					let partial_cmp_result = smallvec.partial_cmp(&other_smallvec);
					if let Some(ord) = partial_cmp_result {
						println!("Partial comparison: {:?}", ord);
					}
				},
				25 => {
					let into_boxed = smallvec.clone().into_boxed_slice();
					println!("Boxed slice len: {}", into_boxed.len());
				},
				26 => {
					let mut append_vec = smallvec::SmallVec::<[i32; 16]>::new();
					append_vec.push(_to_i32(GLOBAL_DATA, index_offset));
					index_offset += 4;
					smallvec.append(&mut append_vec);
				},
				27 => {
					if !smallvec.is_empty() {
						let deref_slice = smallvec.deref();
						println!("Deref first: {}", deref_slice[0]);
					}
				},
				28 => {
					let as_ptr = smallvec.as_ptr();
					println!("Pointer: {:?}", as_ptr);
				},
				_ => {
					if !smallvec.is_empty() {
						let deref_mut_slice = smallvec.deref_mut();
						deref_mut_slice[0] = _to_i32(GLOBAL_DATA, index_offset);
						index_offset += 4;
						println!("Modified first: {}", deref_mut_slice[0]);
					}
				}
			}
		}
		
		let custom_vec_size = _to_u8(GLOBAL_DATA, 2000) % 33;
		let mut custom_vec = std::vec::Vec::with_capacity(32);
		for i in 0..custom_vec_size.min(32) {
			let mut t_10 = _to_u8(GLOBAL_DATA, 2010 + (i as usize * 17)) % 17;
			let t_11 = _to_str(GLOBAL_DATA, 2011 + (i as usize * 17), 2011 + (i as usize * 17) + t_10 as usize);
			let t_12 = String::from(t_11);
			let t_13 = CustomType3(t_12);
			custom_vec.push(t_13);
		}
		custom_vec.truncate(custom_vec_size as usize);
		
		let t_138 = &custom_vec[..];
		let mut t_139 = smallvec::SmallVec::<[CustomType3; 16]>::from(custom_vec);
		let mut t_140 = &mut t_139;
		
		let range_start = _to_usize(GLOBAL_DATA, 2400);
		let range_end = _to_usize(GLOBAL_DATA, 2408);
		
		let range_bounds = CustomType4(range_start, range_end);
		let mut t_157 = smallvec::SmallVec::<[CustomType3; 16]>::drain(t_140, range_bounds);
		let mut t_158 = &mut t_157;
		
		let next_result = t_158.next();
		if let Some(val) = next_result {
			println!("{}", val.0);
		}
		
		let next_back_result = t_158.next_back();
		if let Some(val) = next_back_result {
			println!("{}", val.0);
		}
		
		let second_smallvec_selector = _to_u8(GLOBAL_DATA, 2500) % 5;
		let mut second_smallvec = match second_smallvec_selector {
			0 => smallvec::SmallVec::<[f32; 32]>::new(),
			1 => smallvec::SmallVec::<[f32; 32]>::with_capacity(_to_usize(GLOBAL_DATA, 2508)),
			2 => {
				let mut float_vec = Vec::new();
				for j in 0..(_to_u8(GLOBAL_DATA, 2516) % 20) {
					float_vec.push(_to_f32(GLOBAL_DATA, 2520 + (j as usize * 4)));
				}
				smallvec::SmallVec::<[f32; 32]>::from_vec(float_vec)
			},
			3 => {
				let elem = _to_f32(GLOBAL_DATA, 2600);
				let n = _to_usize(GLOBAL_DATA, 2604);
				smallvec::SmallVec::<[f32; 32]>::from_elem(elem, n)
			},
			_ => {
				let mut slice_data = [0.0f32; 32];
				for k in 0..32 {
					slice_data[k] = _to_f32(GLOBAL_DATA, 2650 + (k * 4));
				}
				smallvec::SmallVec::<[f32; 32]>::from_buf_and_len(slice_data, _to_usize(GLOBAL_DATA, 2800) % 33)
			}
		};
		
		for float_op in 0..(_to_u8(GLOBAL_DATA, 2850) % 10) {
			let float_operation = _to_u8(GLOBAL_DATA, 2851 + float_op as usize) % 8;
			match float_operation {
				0 => {
					second_smallvec.push(_to_f32(GLOBAL_DATA, 2860 + (float_op as usize * 4)));
				},
				1 => {
					if let Some(popped_float) = second_smallvec.pop() {
						println!("Popped float: {}", popped_float);
					}
				},
				2 => {
					second_smallvec.reserve(_to_usize(GLOBAL_DATA, 2900 + (float_op as usize * 8)));
				},
				3 => {
					if !second_smallvec.is_empty() {
						let float_ref = &second_smallvec[_to_usize(GLOBAL_DATA, 2950 + (float_op as usize * 8)) % second_smallvec.len()];
						println!("Float elem: {}", *float_ref);
					}
				},
				4 => {
					let float_len = second_smallvec.len();
					let float_capacity = second_smallvec.capacity();
					println!("Float vec - len: {}, capacity: {}", float_len, float_capacity);
				},
				5 => {
					second_smallvec.clear();
				},
				6 => {
					second_smallvec.shrink_to_fit();
				},
				_ => {
					if !second_smallvec.is_empty() {
						let removed_float = second_smallvec.swap_remove(_to_usize(GLOBAL_DATA, 2980 + (float_op as usize * 8)) % second_smallvec.len());
						println!("Removed float: {}", removed_float);
					}
				}
			}
		}
		
		if !second_smallvec.is_empty() {
			let float_slice = second_smallvec.as_slice();
			println!("Float slice first: {}", float_slice[0]);
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