#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

fn main (){
	fuzz_nohook!(|data: &[u8]| {
		if data.len() < 500 {return;}
		set_global_data(data);
		let global_data = get_global_data();
		let GLOBAL_DATA = global_data.first_half;
		
		let op_count = _to_usize(GLOBAL_DATA, 0) % 25;
		for i in 0..op_count {
			let operation = _to_u8(GLOBAL_DATA, 8 + i) % 30;
			
			match operation {
				0 => {
					let capacity = _to_usize(GLOBAL_DATA, 16 + i * 8);
					let mut sv1: SmallVec<[i32; 15]> = SmallVec::with_capacity(capacity);
					
					let item_count = _to_usize(GLOBAL_DATA, 24 + i * 8) % 65;
					for j in 0..item_count {
						let value = _to_i32(GLOBAL_DATA, 32 + i * 8 + j * 4);
						sv1.push(value);
					}
					
					let slice_ref = sv1.as_slice();
					println!("{:?}", slice_ref);
					
					let spilled_check = sv1.spilled();
					println!("{}", spilled_check);
					
					sv1.clear();
					
					let len_after_clear = sv1.len();
					println!("{}", len_after_clear);
				},
				1 => {
					let vec_size = _to_usize(GLOBAL_DATA, 16 + i * 8) % 65;
					let mut vec_data: Vec<f32> = Vec::new();
					for k in 0..vec_size {
						let val = _to_f32(GLOBAL_DATA, 24 + i * 8 + k * 4);
						vec_data.push(val);
					}
					
					let mut sv2: SmallVec<[f32; 20]> = SmallVec::from_vec(vec_data);
					let initial_len = sv2.len();
					
					let capacity_before = sv2.capacity();
					
					sv2.clear();
					
					let final_len = sv2.len();
					println!("{} -> {}", initial_len, final_len);
				},
				2 => {
					let elem_val = _to_u64(GLOBAL_DATA, 16 + i * 8);
					let elem_count = _to_usize(GLOBAL_DATA, 24 + i * 8) % 65;
					
					let mut sv3: SmallVec<[u64; 12]> = SmallVec::from_elem(elem_val, elem_count);
					
					let before_capacity = sv3.capacity();
					
					sv3.shrink_to_fit();
					
					let after_capacity = sv3.capacity();
					println!("Capacity: {} -> {}", before_capacity, after_capacity);
					
					sv3.clear();
				},
				3 => {
					let mut sv4: SmallVec<[bool; 18]> = SmallVec::new();
					
					let bool_count = _to_usize(GLOBAL_DATA, 16 + i * 8) % 65;
					for m in 0..bool_count {
						let bool_val = _to_bool(GLOBAL_DATA, 24 + i * 8 + m);
						sv4.push(bool_val);
					}
					
					let drain_start = _to_usize(GLOBAL_DATA, 32 + i * 8);
					let drain_end = _to_usize(GLOBAL_DATA, 40 + i * 8);
					let range = drain_start..drain_end;
					
					{
						let mut drain_iter = sv4.drain(range);
						let next_item = drain_iter.next();
						if let Some(item) = next_item {
							println!("{}", item);
						}
					}
					
					sv4.clear();
				},
				4 => {
					let slice_len = _to_usize(GLOBAL_DATA, 16 + i * 8) % 65;
					let mut slice_data: Vec<char> = Vec::new();
					for n in 0..slice_len {
						let char_val = _to_char(GLOBAL_DATA, 24 + i * 8 + n * 4);
						slice_data.push(char_val);
					}
					
					let mut sv5: SmallVec<[char; 25]> = SmallVec::from_slice(&slice_data);
					
					let ptr = sv5.as_ptr();
					println!("{:?}", ptr);
					
					let mut_ptr = sv5.as_mut_ptr();
					println!("{:?}", mut_ptr);
					
					sv5.clear();
					
					let empty_check = sv5.is_empty();
					println!("{}", empty_check);
				},
				5 => {
					let capacity = _to_usize(GLOBAL_DATA, 16 + i * 8);
					let mut sv6: SmallVec<[String; 14]> = SmallVec::with_capacity(capacity);
					
					let str_count = _to_usize(GLOBAL_DATA, 24 + i * 8) % 10;
					for p in 0..str_count {
						let str_len = _to_usize(GLOBAL_DATA, 32 + i * 8 + p * 8) % 20;
						let start_idx = 40 + i * 8 + p * 8;
						let end_idx = start_idx + str_len;
						let str_val = _to_str(GLOBAL_DATA, start_idx, end_idx);
						sv6.push(String::from(str_val));
					}
					
					let spilled = sv6.spilled();
					println!("Spilled: {}", spilled);
					
					sv6.shrink_to_fit();
					
					sv6.clear();
					
					let len_check = sv6.len();
					println!("Length after clear: {}", len_check);
				},
				6 => {
					let iter_size = _to_usize(GLOBAL_DATA, 16 + i * 8) % 65;
					let iter_data: Vec<u8> = (0..iter_size).map(|x| _to_u8(GLOBAL_DATA, 24 + i * 8 + x)).collect();
					
					let mut sv7: SmallVec<[u8; 22]> = SmallVec::from_iter(iter_data);
					
					let mut_slice = sv7.as_mut_slice();
					println!("{:?}", mut_slice);
					
					sv7.reserve(_to_usize(GLOBAL_DATA, 32 + i * 8));
					
					sv7.clear();
				},
				7 => {
					let arr_data: [i16; 16] = [
						_to_i16(GLOBAL_DATA, 16 + i * 8),
						_to_i16(GLOBAL_DATA, 18 + i * 8),
						_to_i16(GLOBAL_DATA, 20 + i * 8),
						_to_i16(GLOBAL_DATA, 22 + i * 8),
						_to_i16(GLOBAL_DATA, 24 + i * 8),
						_to_i16(GLOBAL_DATA, 26 + i * 8),
						_to_i16(GLOBAL_DATA, 28 + i * 8),
						_to_i16(GLOBAL_DATA, 30 + i * 8),
						_to_i16(GLOBAL_DATA, 32 + i * 8),
						_to_i16(GLOBAL_DATA, 34 + i * 8),
						_to_i16(GLOBAL_DATA, 36 + i * 8),
						_to_i16(GLOBAL_DATA, 38 + i * 8),
						_to_i16(GLOBAL_DATA, 40 + i * 8),
						_to_i16(GLOBAL_DATA, 42 + i * 8),
						_to_i16(GLOBAL_DATA, 44 + i * 8),
						_to_i16(GLOBAL_DATA, 46 + i * 8)
					];
					
					let mut sv8: SmallVec<[i16; 16]> = SmallVec::from_buf(arr_data);
					
					let sv8_ref = &sv8;
					println!("{:?}", sv8_ref.deref());
					
					let other_sv8: SmallVec<[i16; 16]> = sv8.clone();
					let comparison = sv8.cmp(&other_sv8);
					println!("{:?}", comparison);
					
					sv8.clear();
				},
				8 => {
					let len_param = _to_usize(GLOBAL_DATA, 16 + i * 8);
					let arr_data2: [f64; 13] = [
						_to_f64(GLOBAL_DATA, 24 + i * 8),
						_to_f64(GLOBAL_DATA, 32 + i * 8),
						_to_f64(GLOBAL_DATA, 40 + i * 8),
						_to_f64(GLOBAL_DATA, 48 + i * 8),
						_to_f64(GLOBAL_DATA, 56 + i * 8),
						_to_f64(GLOBAL_DATA, 64 + i * 8),
						_to_f64(GLOBAL_DATA, 72 + i * 8),
						_to_f64(GLOBAL_DATA, 80 + i * 8),
						_to_f64(GLOBAL_DATA, 88 + i * 8),
						_to_f64(GLOBAL_DATA, 96 + i * 8),
						_to_f64(GLOBAL_DATA, 104 + i * 8),
						_to_f64(GLOBAL_DATA, 112 + i * 8),
						_to_f64(GLOBAL_DATA, 120 + i * 8)
					];
					
					let mut sv9: SmallVec<[f64; 13]> = SmallVec::from_buf_and_len(arr_data2, len_param);
					
					let sv9_len = sv9.len();
					println!("Original length: {}", sv9_len);
					
					sv9.extend_from_slice(&[_to_f64(GLOBAL_DATA, 128 + i * 8)]);
					
					sv9.clear();
					
					let sv9_final_len = sv9.len();
					println!("Final length: {}", sv9_final_len);
				},
				9 => {
					let mut sv10: SmallVec<[isize; 11]> = SmallVec::new();
					
					let insert_count = _to_usize(GLOBAL_DATA, 16 + i * 8) % 65;
					for q in 0..insert_count {
						let val = _to_isize(GLOBAL_DATA, 24 + i * 8 + q * 8);
						sv10.push(val);
					}
					
					let reserve_amount = _to_usize(GLOBAL_DATA, 32 + i * 8);
					sv10.reserve(reserve_amount);
					
					sv10.resize_with(_to_usize(GLOBAL_DATA, 40 + i * 8), || _to_isize(GLOBAL_DATA, 48 + i * 8));
					
					sv10.clear();
					
					let capacity_after = sv10.capacity();
					println!("Capacity after clear: {}", capacity_after);
				},
				10 => {
					let mut sv11: SmallVec<[u32; 19]> = SmallVec::new();
					
					let pop_test_count = _to_usize(GLOBAL_DATA, 16 + i * 8) % 65;
					for r in 0..pop_test_count {
						let val = _to_u32(GLOBAL_DATA, 24 + i * 8 + r * 4);
						sv11.push(val);
					}
					
					let popped_item = sv11.pop();
					if let Some(item) = popped_item {
						println!("Popped: {}", item);
					}
					
					if !sv11.is_empty() {
						let remove_idx = _to_usize(GLOBAL_DATA, 32 + i * 8) % sv11.len();
						let removed_item = sv11.swap_remove(remove_idx);
						println!("Removed: {}", removed_item);
					}
					
					sv11.clear();
				},
				11 => {
					let mut sv12: SmallVec<[i8; 17]> = SmallVec::new();
					
					let truncate_count = _to_usize(GLOBAL_DATA, 16 + i * 8) % 65;
					for s in 0..truncate_count {
						let val = _to_i8(GLOBAL_DATA, 24 + i * 8 + s);
						sv12.push(val);
					}
					
					let truncate_len = _to_usize(GLOBAL_DATA, 32 + i * 8);
					sv12.truncate(truncate_len);
					
					sv12.retain(|&mut x| x % 2 == 0);
					
					sv12.clear();
				},
				12 => {
					let mut sv13: SmallVec<[u16; 21]> = SmallVec::new();
					
					let append_count = _to_usize(GLOBAL_DATA, 16 + i * 8) % 65;
					for t in 0..append_count {
						let val = _to_u16(GLOBAL_DATA, 24 + i * 8 + t * 2);
						sv13.push(val);
					}
					
					let mut sv13_other: SmallVec<[u16; 21]> = SmallVec::new();
					let other_count = _to_usize(GLOBAL_DATA, 32 + i * 8) % 10;
					for u in 0..other_count {
						let val = _to_u16(GLOBAL_DATA, 40 + i * 8 + u * 2);
						sv13_other.push(val);
					}
					
					sv13.append(&mut sv13_other);
					
					let eq_check = sv13 == sv13_other;
					println!("Equal: {}", eq_check);
					
					sv13.clear();
				},
				13 => {
					let mut sv14: SmallVec<[i64; 23]> = SmallVec::new();
					
					let pre_count = _to_usize(GLOBAL_DATA, 32 + i * 8) % 65;
					for v in 0..pre_count {
						let val = _to_i64(GLOBAL_DATA, 40 + i * 8 + v * 8);
						sv14.push(val);
					}
					
					let insert_pos = _to_usize(GLOBAL_DATA, 16 + i * 8);
					let insert_val = _to_i64(GLOBAL_DATA, 24 + i * 8);
					
					if insert_pos <= sv14.len() {
						sv14.insert(insert_pos, insert_val);
					}
					
					let into_vec_result = sv14.into_vec();
					println!("Vec length: {}", into_vec_result.len());
				},
				14 => {
					let mut sv15: SmallVec<[usize; 26]> = SmallVec::new();
					
					let shrink_count = _to_usize(GLOBAL_DATA, 16 + i * 8) % 65;
					for w in 0..shrink_count {
						let val = _to_usize(GLOBAL_DATA, 24 + i * 8 + w * 8);
						sv15.push(val);
					}
					
					sv15.dedup();
					
					sv15.shrink_to_fit();
					
					sv15.clear();
				},
				15 => {
					let slice_data: [f32; 16] = [
						_to_f32(GLOBAL_DATA, 16 + i * 8),
						_to_f32(GLOBAL_DATA, 20 + i * 8),
						_to_f32(GLOBAL_DATA, 24 + i * 8),
						_to_f32(GLOBAL_DATA, 28 + i * 8),
						_to_f32(GLOBAL_DATA, 32 + i * 8),
						_to_f32(GLOBAL_DATA, 36 + i * 8),
						_to_f32(GLOBAL_DATA, 40 + i * 8),
						_to_f32(GLOBAL_DATA, 44 + i * 8),
						_to_f32(GLOBAL_DATA, 48 + i * 8),
						_to_f32(GLOBAL_DATA, 52 + i * 8),
						_to_f32(GLOBAL_DATA, 56 + i * 8),
						_to_f32(GLOBAL_DATA, 60 + i * 8),
						_to_f32(GLOBAL_DATA, 64 + i * 8),
						_to_f32(GLOBAL_DATA, 68 + i * 8),
						_to_f32(GLOBAL_DATA, 72 + i * 8),
						_to_f32(GLOBAL_DATA, 76 + i * 8)
					];
					
					let mut sv16: SmallVec<[f32; 16]> = SmallVec::from(slice_data);
					
					let slice_ref: SmallVec<[f32; 16]> = sv16.as_slice().to_smallvec();
					println!("{:?}", slice_ref.len());
					
					sv16.clear();
				},
				16 => {
					let capacity_val = _to_usize(GLOBAL_DATA, 16 + i * 8);
					let mut sv17: SmallVec<[String; 12]> = SmallVec::with_capacity(capacity_val);
					
					let str_elements = _to_usize(GLOBAL_DATA, 24 + i * 8) % 10;
					for idx in 0..str_elements {
						let str_len = _to_usize(GLOBAL_DATA, 32 + i * 8 + idx * 8) % 15;
						let start_pos = 40 + i * 8 + idx * 8;
						let end_pos = start_pos + str_len;
						let string_val = _to_str(GLOBAL_DATA, start_pos, end_pos);
						sv17.push(String::from(string_val));
					}
					
					let boxed_slice = sv17.into_boxed_slice();
					println!("Boxed slice length: {}", boxed_slice.len());
				},
				17 => {
					let mut sv18: SmallVec<[u8; 32]> = SmallVec::new();
					
					let element_count = _to_usize(GLOBAL_DATA, 16 + i * 8) % 65;
					for k in 0..element_count {
						sv18.push(_to_u8(GLOBAL_DATA, 24 + i * 8 + k));
					}
					
					if !sv18.is_empty() {
						let remove_idx = _to_usize(GLOBAL_DATA, 32 + i * 8) % sv18.len();
						let removed = sv18.remove(remove_idx);
						println!("Removed element: {}", removed);
					}
					
					sv18.clear();
				},
				18 => {
					let mut sv19: SmallVec<[char; 15]> = SmallVec::new();
					
					let char_count = _to_usize(GLOBAL_DATA, 16 + i * 8) % 65;
					for m in 0..char_count {
						let char_val = _to_char(GLOBAL_DATA, 24 + i * 8 + m * 4);
						sv19.push(char_val);
					}
					
					let insert_idx = _to_usize(GLOBAL_DATA, 32 + i * 8);
					if insert_idx <= sv19.len() {
						let slice_to_insert = &[_to_char(GLOBAL_DATA, 40 + i * 8), _to_char(GLOBAL_DATA, 44 + i * 8)];
						sv19.insert_from_slice(insert_idx, slice_to_insert);
					}
					
					sv19.clear();
				},
				19 => {
					let mut sv20: SmallVec<[bool; 20]> = SmallVec::new();
					
					let bool_elements = _to_usize(GLOBAL_DATA, 16 + i * 8) % 65;
					for idx in 0..bool_elements {
						sv20.push(_to_bool(GLOBAL_DATA, 24 + i * 8 + idx));
					}
					
					let resize_len = _to_usize(GLOBAL_DATA, 32 + i * 8);
					let resize_val = _to_bool(GLOBAL_DATA, 40 + i * 8);
					sv20.resize(resize_len, resize_val);
					
					sv20.clear();
				},
				20 => {
					let mut sv21: SmallVec<[u64; 11]> = SmallVec::new();
					
					let collection_size = _to_usize(GLOBAL_DATA, 16 + i * 8) % 65;
					for j in 0..collection_size {
						sv21.push(_to_u64(GLOBAL_DATA, 24 + i * 8 + j * 8));
					}
					
					let many_items: Vec<u64> = (0..5).map(|x| _to_u64(GLOBAL_DATA, 32 + i * 8 + x * 8)).collect();
					let insert_pos = _to_usize(GLOBAL_DATA, 72 + i * 8);
					if insert_pos <= sv21.len() {
						sv21.insert_many(insert_pos, many_items);
					}
					
					sv21.clear();
				},
				21 => {
					let mut sv22: SmallVec<[i32; 18]> = SmallVec::new();
					
					let elems = _to_usize(GLOBAL_DATA, 16 + i * 8) % 65;
					for idx in 0..elems {
						sv22.push(_to_i32(GLOBAL_DATA, 24 + i * 8 + idx * 4));
					}
					
					sv22.dedup_by(|a, b| a == b);
					
					sv22.clear();
				},
				22 => {
					let mut sv23: SmallVec<[f64; 14]> = SmallVec::new();
					
					let data_size = _to_usize(GLOBAL_DATA, 16 + i * 8) % 65;
					for idx in 0..data_size {
						sv23.push(_to_f64(GLOBAL_DATA, 24 + i * 8 + idx * 8));
					}
					
					sv23.dedup_by_key(|x| *x as i64);
					
					sv23.clear();
				},
				23 => {
					let data: Vec<u16> = (0..20).map(|x| _to_u16(GLOBAL_DATA, 16 + i * 8 + x * 2)).collect();
					let sv24: SmallVec<[u16; 25]> = SmallVec::from_slice(&data);
					
					let into_iter = sv24.into_iter();
					let collected: Vec<u16> = into_iter.collect();
					println!("Collected length: {}", collected.len());
				},
				24 => {
					let mut sv25: SmallVec<[f64; 12]> = SmallVec::new();
					let elements = _to_usize(GLOBAL_DATA, 16 + i * 8) % 65;
					for idx in 0..elements {
						sv25.push(_to_f64(GLOBAL_DATA, 24 + i * 8 + idx * 8));
					}
					let hash_input = _to_usize(GLOBAL_DATA, 32 + i * 8);
					sv25.clear();
				},
				25 => {
					let mut sv26: SmallVec<[i128; 15]> = SmallVec::new();
					let count = _to_usize(GLOBAL_DATA, 16 + i * 8) % 65;
					for idx in 0..count {
						sv26.push(_to_i128(GLOBAL_DATA, 24 + i * 8 + idx * 16));
					}
					sv26.sort();
					sv26.clear();
				},
				26 => {
					let mut sv27: SmallVec<[bool; 30]> = SmallVec::new();
					let capacity_exact = _to_usize(GLOBAL_DATA, 16 + i * 8);
					sv27.reserve_exact(capacity_exact);
					let items = _to_usize(GLOBAL_DATA, 24 + i * 8) % 65;
					for idx in 0..items {
						sv27.push(_to_bool(GLOBAL_DATA, 32 + i * 8 + idx));
					}
					sv27.clear();
				},
				27 => {
					let arr: [u32; 20] = [
						_to_u32(GLOBAL_DATA, 16 + i * 8),
						_to_u32(GLOBAL_DATA, 20 + i * 8),
						_to_u32(GLOBAL_DATA, 24 + i * 8),
						_to_u32(GLOBAL_DATA, 28 + i * 8),
						_to_u32(GLOBAL_DATA, 32 + i * 8),
						_to_u32(GLOBAL_DATA, 36 + i * 8),
						_to_u32(GLOBAL_DATA, 40 + i * 8),
						_to_u32(GLOBAL_DATA, 44 + i * 8),
						_to_u32(GLOBAL_DATA, 48 + i * 8),
						_to_u32(GLOBAL_DATA, 52 + i * 8),
						_to_u32(GLOBAL_DATA, 56 + i * 8),
						_to_u32(GLOBAL_DATA, 60 + i * 8),
						_to_u32(GLOBAL_DATA, 64 + i * 8),
						_to_u32(GLOBAL_DATA, 68 + i * 8),
						_to_u32(GLOBAL_DATA, 72 + i * 8),
						_to_u32(GLOBAL_DATA, 76 + i * 8),
						_to_u32(GLOBAL_DATA, 80 + i * 8),
						_to_u32(GLOBAL_DATA, 84 + i * 8),
						_to_u32(GLOBAL_DATA, 88 + i * 8),
						_to_u32(GLOBAL_DATA, 92 + i * 8)
					];
					let sv28: SmallVec<[u32; 20]> = SmallVec::from(arr);
					let sv_ref = &sv28[0..5];
					println!("{:?}", sv_ref);
				},
				28 => {
					let mut sv29: SmallVec<[char; 18]> = SmallVec::new();
					let char_elements = _to_usize(GLOBAL_DATA, 16 + i * 8) % 65;
					for idx in 0..char_elements {
						sv29.push(_to_char(GLOBAL_DATA, 24 + i * 8 + idx * 4));
					}
					let as_mut_ref = sv29.as_mut();
					println!("{:?}", as_mut_ref.len());
					sv29.clear();
				},
				_ => {
					let mut sv30: SmallVec<[isize; 30]> = SmallVec::new();
					
					let capacity_reserve = _to_usize(GLOBAL_DATA, 16 + i * 8);
					sv30.reserve_exact(capacity_reserve);
					
					let data_count = _to_usize(GLOBAL_DATA, 24 + i * 8) % 65;
					for idx in 0..data_count {
						sv30.push(_to_isize(GLOBAL_DATA, 32 + i * 8 + idx * 8));
					}
					
					let into_inner_result = sv30.into_inner();
					if let Ok(array_data) = into_inner_result {
						println!("Array size: {}", <[isize; 30]>::size());
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