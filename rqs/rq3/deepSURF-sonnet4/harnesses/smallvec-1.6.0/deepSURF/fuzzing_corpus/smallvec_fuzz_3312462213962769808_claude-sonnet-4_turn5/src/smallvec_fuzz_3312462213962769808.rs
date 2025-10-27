#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};
use std::borrow::{Borrow, BorrowMut};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct CustomType0(String);

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

fn main (){
	fuzz_nohook!(|data: &[u8]| {
		if data.len() < 1200 {return;}
		set_global_data(data);
		let global_data = get_global_data();
		let GLOBAL_DATA = global_data.first_half;
		let mut operation_count = _to_u8(GLOBAL_DATA, 0) % 33;
		
		for i in 0..operation_count {
			let operation_selector = _to_u8(GLOBAL_DATA, 1 + i as usize) % 12;
			match operation_selector {
				0 => {
					let vec_size = _to_u8(GLOBAL_DATA, 35) % 33;
					let mut source_vec = std::vec::Vec::with_capacity(32);
					for j in 0..vec_size {
						let value = _to_usize(GLOBAL_DATA, 67 + j as usize * 8);
						source_vec.push(CustomType1(value));
					}
					source_vec.truncate(vec_size as usize);
					let slice_ref = &source_vec[..];
					
					let constructor_selector = _to_u8(GLOBAL_DATA, 334) % 8;
					let mut result = match constructor_selector {
						0 => smallvec::SmallVec::<[CustomType1; 16]>::new(),
						1 => {
							let capacity = _to_usize(GLOBAL_DATA, 335);
							smallvec::SmallVec::<[CustomType1; 16]>::with_capacity(capacity)
						},
						2 => smallvec::SmallVec::<[CustomType1; 16]>::from_vec(source_vec.clone()),
						3 => {
							let elem = CustomType1(_to_usize(GLOBAL_DATA, 343));
							let count = _to_usize(GLOBAL_DATA, 351);
							smallvec::SmallVec::<[CustomType1; 16]>::from_elem(elem, count)
						},
						4 => smallvec::SmallVec::<[CustomType1; 16]>::from(source_vec.clone()),
						5 => slice_ref.to_smallvec(),
						6 => {
							if !source_vec.is_empty() {
								let iter = source_vec.clone().into_iter();
								smallvec::SmallVec::<[CustomType1; 16]>::from_iter(iter)
							} else {
								smallvec::SmallVec::<[CustomType1; 16]>::new()
							}
						},
						_ => smallvec::SmallVec::<[CustomType1; 16]>::from_slice(slice_ref),
					};
					
					let target_slice = &slice_ref[..];
					let target_result = smallvec::SmallVec::<[CustomType1; 16]>::from_slice(target_slice);
					println!("{:?}", target_result.as_slice());
					
					if result.len() > 0 {
						let item_ref = &result[0];
						println!("{:?}", *item_ref);
					}
					
					let slice_result = result.as_slice();
					println!("{:?}", slice_result);
					
					let spilled_status = result.spilled();
					println!("{:?}", spilled_status);
					
					let len = result.len();
					println!("{:?}", len);
					
					let is_empty = result.is_empty();
					println!("{:?}", is_empty);
				},
				1 => {
					let mut vec1 = smallvec::SmallVec::<[CustomType1; 24]>::new();
					let mut vec2 = smallvec::SmallVec::<[CustomType1; 24]>::new();
					
					let push_count = _to_u8(GLOBAL_DATA, 359) % 15;
					for k in 0..push_count {
						let value = CustomType1(_to_usize(GLOBAL_DATA, 360 + k as usize * 8));
						vec1.push(value.clone());
						vec2.push(value);
					}
					
					let capacity = vec1.capacity();
					println!("{:?}", capacity);
					
					let is_empty = vec1.is_empty();
					println!("{:?}", is_empty);
					
					let len = vec1.len();
					println!("{:?}", len);
					
					if !vec1.is_empty() {
						if let Some(popped) = vec1.pop() {
							println!("{:?}", popped);
						}
					}
					
					let reserve_size = _to_usize(GLOBAL_DATA, 480);
					vec1.reserve(reserve_size);
					
					let truncate_len = _to_usize(GLOBAL_DATA, 488);
					vec1.truncate(truncate_len);
					
					vec1.append(&mut vec2);
					
					let slice_ref = vec1.as_slice();
					let deref_slice = vec1.deref();
					println!("{:?}", deref_slice);
					
					let mut_slice_ref = vec1.as_mut_slice();
					println!("{:?}", mut_slice_ref);
					
					let as_ref_slice = vec1.as_ref();
					println!("{:?}", as_ref_slice);
				},
				2 => {
					let mut vec = smallvec::SmallVec::<[CustomType1; 20]>::from_elem(CustomType1(_to_usize(GLOBAL_DATA, 496)), 1);
					
					let insert_index = _to_usize(GLOBAL_DATA, 504);
					let insert_value = CustomType1(_to_usize(GLOBAL_DATA, 512));
					vec.insert(insert_index, insert_value);
					
					if !vec.is_empty() {
						let remove_index = _to_usize(GLOBAL_DATA, 520);
						let removed = vec.remove(remove_index);
						println!("{:?}", removed);
					}
					
					let grow_size = _to_usize(GLOBAL_DATA, 528);
					vec.grow(grow_size);
					
					vec.shrink_to_fit();
					
					let element_slice = vec.as_slice();
					println!("{:?}", element_slice);
					
					if !vec.is_empty() {
						let swap_removed = vec.swap_remove(_to_usize(GLOBAL_DATA, 536));
						println!("{:?}", swap_removed);
					}
					
					let inline_size = vec.inline_size();
					println!("{:?}", inline_size);
				},
				3 => {
					let mut vec = smallvec::SmallVec::<[CustomType1; 12]>::new();
					
					let extend_size = _to_u8(GLOBAL_DATA, 536) % 20;
					let mut extend_vec = Vec::new();
					for m in 0..extend_size {
						extend_vec.push(CustomType1(_to_usize(GLOBAL_DATA, 537 + m as usize * 8)));
					}
					
					let slice_for_extend = &extend_vec[..];
					vec.extend_from_slice(slice_for_extend);
					
					let slice_to_insert = &extend_vec[..];
					let insert_index = _to_usize(GLOBAL_DATA, 697);
					vec.insert_from_slice(insert_index, slice_to_insert);
					
					let target_slice = vec.as_slice();
					let comparison_vec = smallvec::SmallVec::<[CustomType1; 12]>::from_slice(target_slice);
					println!("{:?}", comparison_vec.as_slice());
					
					vec.extend(extend_vec.iter().cloned());
					
					let many_insert_idx = _to_usize(GLOBAL_DATA, 705);
					vec.insert_many(many_insert_idx, extend_vec.clone());
				},
				4 => {
					let mut vec = smallvec::SmallVec::<[CustomType1; 18]>::new();
					let clone_vec = vec.clone();
					
					let comparison = vec == clone_vec;
					println!("{:?}", comparison);
					
					let ordering = vec.cmp(&clone_vec);
					println!("{:?}", ordering);
					
					let partial_ordering = vec.partial_cmp(&clone_vec);
					if let Some(ord) = partial_ordering {
						println!("{:?}", ord);
					}
					
					vec.clear();
					
					let is_empty_after_clear = vec.is_empty();
					println!("{:?}", is_empty_after_clear);
					
					for n in 0..5 {
						vec.push(CustomType1(_to_usize(GLOBAL_DATA, 721 + n * 8)));
					}
					
					vec.dedup();
					
					vec.dedup_by(|a, b| a.0 == b.0);
					
					vec.dedup_by_key(|x| x.0);
				},
				5 => {
					let mut vec = smallvec::SmallVec::<[CustomType1; 32]>::new();
					
					let drain_start = _to_usize(GLOBAL_DATA, 705);
					let drain_end = _to_usize(GLOBAL_DATA, 713);
					
					for n in 0..10 {
						vec.push(CustomType1(_to_usize(GLOBAL_DATA, 721 + n * 8)));
					}
					
					let mut drain_iter = vec.drain(drain_start..drain_end);
					if let Some(drained_item) = drain_iter.next() {
						println!("{:?}", drained_item);
					}
					
					drop(drain_iter);
					
					let remaining_slice = vec.as_slice();
					println!("{:?}", remaining_slice);
					
					vec.retain(|x| x.0 % 2 == 0);
					
					let resize_len = _to_usize(GLOBAL_DATA, 801);
					let resize_value = CustomType1(_to_usize(GLOBAL_DATA, 809));
					vec.resize(resize_len, resize_value);
					
					let resize_with_len = _to_usize(GLOBAL_DATA, 817);
					vec.resize_with(resize_with_len, || CustomType1(_to_usize(GLOBAL_DATA, 825)));
				},
				6 => {
					let mut vec = smallvec::SmallVec::<[CustomType1; 64]>::new();
					
					for p in 0..5 {
						vec.push(CustomType1(_to_usize(GLOBAL_DATA, 801 + p * 8)));
					}
					
					let into_iter = vec.into_iter();
					let iter_slice = into_iter.as_slice();
					println!("{:?}", iter_slice);
					
					for item in into_iter {
						println!("{:?}", item);
					}
				},
				7 => {
					let elem = CustomType1(_to_usize(GLOBAL_DATA, 841));
					let count = _to_usize(GLOBAL_DATA, 849);
					let vec = smallvec::SmallVec::<[CustomType1; 128]>::from_elem(elem, count);
					
					let vec_slice = vec.as_slice();
					let from_slice_result = smallvec::SmallVec::<[CustomType1; 128]>::from_slice(vec_slice);
					println!("{:?}", from_slice_result.as_slice());
					
					let into_vec_result = vec.into_vec();
					println!("{:?}", into_vec_result);
				},
				8 => {
					let large_vec = vec![CustomType1(_to_usize(GLOBAL_DATA, 857)); 15];
					let slice_ref = &large_vec[..];
					let target_result = smallvec::SmallVec::<[CustomType1; 256]>::from_slice(slice_ref);
					
					let deref_slice = target_result.deref();
					println!("{:?}", deref_slice);
					
					let borrowed_slice: &[CustomType1] = target_result.borrow();
					println!("{:?}", borrowed_slice);
					
					let as_ptr = target_result.as_ptr();
					println!("{:?}", as_ptr);
					
					let into_boxed = target_result.into_boxed_slice();
					println!("{:?}", into_boxed);
				},
				9 => {
					let mut vec = smallvec::SmallVec::<[CustomType1; 32]>::new();
					
					for q in 0..8 {
						vec.push(CustomType1(_to_usize(GLOBAL_DATA, 865 + q * 8)));
					}
					
					if !vec.is_empty() {
						let index = _to_usize(GLOBAL_DATA, 929);
						let indexed_ref = &vec[index];
						println!("{:?}", *indexed_ref);
					}
					
					let mut_deref = vec.deref_mut();
					println!("{:?}", mut_deref);
					
					let as_mut_ref = vec.as_mut();
					println!("{:?}", as_mut_ref);
					
					let borrow_mut_ref: &mut [CustomType1] = vec.borrow_mut();
					println!("{:?}", borrow_mut_ref);
				},
				10 => {
					let mut vec1 = smallvec::SmallVec::<[CustomType1; 16]>::new();
					let mut vec2 = smallvec::SmallVec::<[CustomType1; 16]>::new();
					
					for r in 0..3 {
						vec1.push(CustomType1(_to_usize(GLOBAL_DATA, 937 + r * 8)));
						vec2.push(CustomType1(_to_usize(GLOBAL_DATA, 961 + r * 8)));
					}
					
					let eq_result = vec1.eq(&vec2);
					println!("{:?}", eq_result);
					
					let partial_ord_result = vec1.partial_cmp(&vec2);
					println!("{:?}", partial_ord_result);
					
					let try_reserve_result = vec1.try_reserve(_to_usize(GLOBAL_DATA, 985));
					println!("{:?}", try_reserve_result.is_ok());
					
					let try_reserve_exact_result = vec1.try_reserve_exact(_to_usize(GLOBAL_DATA, 993));
					println!("{:?}", try_reserve_exact_result.is_ok());
					
					vec1.reserve_exact(_to_usize(GLOBAL_DATA, 1001));
					
					let try_grow_result = vec1.try_grow(_to_usize(GLOBAL_DATA, 1009));
					println!("{:?}", try_grow_result.is_ok());
				},
				_ => {
					let source_array = [CustomType1(_to_usize(GLOBAL_DATA, 1017)); 20];
					let from_array = smallvec::SmallVec::from(source_array);
					
					let clone_result = from_array.clone();
					println!("{:?}", clone_result.as_slice());
					
					let mut mutable_vec = from_array.clone();
					if !mutable_vec.is_empty() {
						let index_mut_ref = &mut mutable_vec[_to_usize(GLOBAL_DATA, 1025)];
						println!("{:?}", *index_mut_ref);
					}
					
					let as_mut_ptr = mutable_vec.as_mut_ptr();
					println!("{:?}", as_mut_ptr);
					
					let into_inner_result = mutable_vec.into_inner();
					match into_inner_result {
						Ok(array) => println!("Success converting to array"),
						Err(vec) => println!("Failed to convert, vec len: {:?}", vec.len()),
					}
				}
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