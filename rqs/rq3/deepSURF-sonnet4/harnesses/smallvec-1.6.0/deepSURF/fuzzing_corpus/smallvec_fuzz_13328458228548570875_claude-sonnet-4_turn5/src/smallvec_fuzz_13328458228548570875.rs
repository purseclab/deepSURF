#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};
use std::hash::{Hash, Hasher};

#[derive(Debug)]
struct CustomType0(String);

#[derive(Debug)]
struct CustomType1(usize);

#[derive(Debug)]
struct CustomType2(String);

impl core::clone::Clone for CustomType1 {
	
	fn clone(&self) -> Self {
		let global_data = get_global_data();
		let GLOBAL_DATA = global_data.first_half;
		let custom_impl_num = _to_usize(GLOBAL_DATA, 42);
		let custom_impl_inst_num = self.0;
		let selector = (custom_impl_num + custom_impl_inst_num) % 3;
		if selector == 0{
			panic!("INTENTIONAL PANIC!");
		}
		let GLOBAL_DATA = match selector{
				1 => global_data.first_half,
				_ => global_data.second_half,
		};
		let t_5 = _to_usize(GLOBAL_DATA, 50);
		let t_6 = CustomType1(t_5);
		return t_6;
	}
}

impl core::marker::Copy for CustomType1 {
}

impl core::hash::Hash for CustomType1 {
	
	fn hash<H: Hasher>(&self, _: &mut H) {
		let global_data = get_global_data();
		let GLOBAL_DATA = global_data.first_half;
		let custom_impl_num = _to_usize(GLOBAL_DATA, 0);
		let custom_impl_inst_num = self.0;
		let selector = (custom_impl_num + custom_impl_inst_num) % 3;
		if selector == 0{
			panic!("INTENTIONAL PANIC!");
		}
		let GLOBAL_DATA = match selector{
				1 => global_data.first_half,
				_ => global_data.second_half,
		};
		return ;
	}
}

fn main (){
	fuzz_nohook!(|data: &[u8]| {
		if data.len() < 2800 {return;}
		set_global_data(data);
		let global_data = get_global_data();
		let GLOBAL_DATA = global_data.first_half;
		
		let num_operations = _to_u8(GLOBAL_DATA, 10) % 64;
		
		for operation_idx in 0..num_operations {
			let op_selector = _to_u8(GLOBAL_DATA, 11 + operation_idx as usize) % 12;
			
			match op_selector {
				0 => {
					let mut t_vec = std::vec::Vec::with_capacity(32);
					for i in 0..32 {
						let t_val = _to_usize(GLOBAL_DATA, 50 + i * 8);
						let t_item = CustomType1(t_val);
						t_vec.push(t_item);
					}
					let constructor_choice = _to_u8(GLOBAL_DATA, 300) % 5;
					let smallvec = match constructor_choice {
						0 => smallvec::SmallVec::<[CustomType1; 16]>::new(),
						1 => smallvec::SmallVec::<[CustomType1; 16]>::with_capacity(_to_usize(GLOBAL_DATA, 400)),
						2 => smallvec::SmallVec::<[CustomType1; 16]>::from_vec(t_vec.clone()),
						3 => smallvec::SmallVec::<[CustomType1; 16]>::from_slice(&t_vec[..]),
						_ => {
							let elem = CustomType1(_to_usize(GLOBAL_DATA, 450));
							let count = _to_usize(GLOBAL_DATA, 458) % 64;
							smallvec::SmallVec::<[CustomType1; 16]>::from_elem(elem, count)
						}
					};
					
					let t_ref = &smallvec;
					let mut t_hasher = std::collections::hash_map::DefaultHasher::new();
					t_ref.hash(&mut t_hasher);
					
					let t_slice = t_ref.as_slice();
					if t_slice.len() > 0 {
						println!("{:?}", t_slice[0].0);
					}
					let t_deref = t_ref.deref();
					if t_deref.len() > 0 {
						println!("{:?}", t_deref[0].0);
					}
				},
				1 => {
					let mut smallvec = smallvec::SmallVec::<[u8; 32]>::new();
					let capacity_before = smallvec.capacity();
					let len_before = smallvec.len();
					println!("{}", capacity_before);
					println!("{}", len_before);
					
					let push_count = _to_u8(GLOBAL_DATA, 500) % 64;
					for i in 0..push_count {
						let value = _to_u8(GLOBAL_DATA, 501 + i as usize);
						smallvec.push(value);
					}
					
					let mut drain_start = _to_usize(GLOBAL_DATA, 600);
					let mut drain_end = _to_usize(GLOBAL_DATA, 608);
					if drain_end <= drain_start || drain_start >= smallvec.len() {
						drain_start = 0;
						drain_end = smallvec.len();
					}
					if drain_end > smallvec.len() {
						drain_end = smallvec.len();
					}
					
					{
						let mut drain_iter = smallvec.drain(drain_start..drain_end);
						if let Some(first_item) = drain_iter.next() {
							println!("{}", first_item);
						}
					}
					
					smallvec.shrink_to_fit();
					smallvec.clear();
				},
				2 => {
					let mut vec1 = smallvec::SmallVec::<[i32; 16]>::new();
					let mut vec2 = smallvec::SmallVec::<[i32; 16]>::new();
					
					let count1 = _to_u8(GLOBAL_DATA, 700) % 32;
					for i in 0..count1 {
						let value = _to_i32(GLOBAL_DATA, 701 + (i as usize * 4));
						vec1.push(value);
					}
					
					let count2 = _to_u8(GLOBAL_DATA, 800) % 32;
					for i in 0..count2 {
						let value = _to_i32(GLOBAL_DATA, 801 + (i as usize * 4));
						vec2.push(value);
					}
					
					let eq_result = vec1.eq(&vec2);
					println!("{}", eq_result);
					
					let cmp_result = vec1.cmp(&vec2);
					println!("{:?}", cmp_result);
					
					let partial_cmp_result = vec1.partial_cmp(&vec2);
					if let Some(ord) = partial_cmp_result {
						println!("{:?}", ord);
					}
					
					vec1.append(&mut vec2);
					let capacity_final = vec1.capacity();
					println!("{}", capacity_final);
				},
				3 => {
					let mut smallvec = smallvec::SmallVec::<[f64; 8]>::new();
					
					let extend_count = _to_u8(GLOBAL_DATA, 900) % 16;
					for i in 0..extend_count {
						let value = _to_f64(GLOBAL_DATA, 901 + (i as usize * 8));
						smallvec.push(value);
					}
					
					let reserve_amount = _to_usize(GLOBAL_DATA, 1000);
					smallvec.reserve(reserve_amount);
					
					let insert_index = _to_usize(GLOBAL_DATA, 1008);
					let insert_value = _to_f64(GLOBAL_DATA, 1016);
					if insert_index <= smallvec.len() {
						smallvec.insert(insert_index, insert_value);
					}
					
					if !smallvec.is_empty() {
						let remove_index = _to_usize(GLOBAL_DATA, 1024) % smallvec.len();
						let removed = smallvec.remove(remove_index);
						println!("{}", removed);
					}
					
					if !smallvec.is_empty() {
						let swap_remove_index = _to_usize(GLOBAL_DATA, 1032) % smallvec.len();
						let swap_removed = smallvec.swap_remove(swap_remove_index);
						println!("{}", swap_removed);
					}
					
					if !smallvec.is_empty() {
						let pop_result = smallvec.pop();
						if let Some(popped) = pop_result {
							println!("{}", popped);
						}
					}
				},
				4 => {
					let slice_data = [1u64, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
					let mut smallvec = smallvec::SmallVec::<[u64; 20]>::from_slice(&slice_data);
					
					let truncate_len = _to_usize(GLOBAL_DATA, 1100) % 20;
					smallvec.truncate(truncate_len);
					
					let as_slice_result = smallvec.as_slice();
					println!("{}", as_slice_result.len());
					
					let as_mut_slice_result = smallvec.as_mut_slice();
					if !as_mut_slice_result.is_empty() {
						let first_elem = &mut as_mut_slice_result[0];
						println!("{}", *first_elem);
					}
					
					let additional_data = [17u64, 18, 19, 20];
					smallvec.extend_from_slice(&additional_data);
					
					let insert_from_slice_index = _to_usize(GLOBAL_DATA, 1200);
					let insert_slice_data = [99u64, 100];
					if insert_from_slice_index <= smallvec.len() {
						smallvec.insert_from_slice(insert_from_slice_index, &insert_slice_data);
					}
					
					let as_ptr_result = smallvec.as_ptr();
					println!("{:p}", as_ptr_result);
					
					let as_mut_ptr_result = smallvec.as_mut_ptr();
					println!("{:p}", as_mut_ptr_result);
				},
				5 => {
					let initial_vec = vec![42i16; 12];
					let mut smallvec = smallvec::SmallVec::<[i16; 8]>::from_iter(initial_vec.into_iter());
					
					let resize_len = _to_usize(GLOBAL_DATA, 1300) % 32;
					let resize_value = _to_i16(GLOBAL_DATA, 1308);
					smallvec.resize(resize_len, resize_value);
					
					let resize_with_len = _to_usize(GLOBAL_DATA, 1350) % 16;
					smallvec.resize_with(resize_with_len, || _to_i16(GLOBAL_DATA, 1400));
					
					smallvec.dedup();
					
					smallvec.retain(|x| *x % 2 == 0);
					
					let final_len = smallvec.len();
					println!("{}", final_len);
					
					let into_vec_result = smallvec.into_vec();
					let vec_len = into_vec_result.len();
					println!("{}", vec_len);
				},
				6 => {
					let mut smallvec = smallvec::SmallVec::<[char; 12]>::new();
					
					let char_count = _to_u8(GLOBAL_DATA, 1500) % 16;
					for i in 0..char_count {
						let char_val = _to_char(GLOBAL_DATA, 1501 + (i as usize * 4));
						smallvec.push(char_val);
					}
					
					let into_iter_result = smallvec.into_iter();
					for (idx, ch) in into_iter_result.enumerate() {
						if idx < 5 {
							println!("{}", ch);
						}
					}
				},
				7 => {
					let mut smallvec1 = smallvec::SmallVec::<[String; 18]>::new();
					let constructor_choice = _to_u8(GLOBAL_DATA, 1700) % 5;
					
					let smallvec2 = match constructor_choice {
						0 => smallvec::SmallVec::<[String; 18]>::new(),
						1 => smallvec::SmallVec::<[String; 18]>::with_capacity(_to_usize(GLOBAL_DATA, 1705)),
						2 => {
							let str_vec = vec![String::from("test1"), String::from("test2")];
							smallvec::SmallVec::<[String; 18]>::from_vec(str_vec)
						},
						3 => {
							let elem = String::from("repeated");
							let count = _to_usize(GLOBAL_DATA, 1720) % 10;
							smallvec::SmallVec::<[String; 18]>::from_elem(elem, count)
						},
						_ => {
							let iter_vec = vec![String::from("iter1"), String::from("iter2")];
							smallvec::SmallVec::<[String; 18]>::from_iter(iter_vec.into_iter())
						}
					};
					
					let str_item = String::from("pushed");
					smallvec1.push(str_item);
					
					let len_result = smallvec1.len();
					let capacity_result = smallvec1.capacity();
					let is_empty_result = smallvec1.is_empty();
					
					println!("{}", len_result);
					println!("{}", capacity_result);
					println!("{}", is_empty_result);
					
					let spilled_result = smallvec1.spilled();
					println!("{}", spilled_result);
					
					let clone_result = smallvec2.clone();
					let clone_len = clone_result.len();
					println!("{}", clone_len);
				},
				8 => {
					let mut smallvec = smallvec::SmallVec::<[bool; 24]>::new();
					
					let bool_count = _to_u8(GLOBAL_DATA, 1800) % 30;
					for i in 0..bool_count {
						let bool_val = _to_bool(GLOBAL_DATA, 1801 + i as usize);
						smallvec.push(bool_val);
					}
					
					let drain_start = _to_usize(GLOBAL_DATA, 1850);
					let drain_end = _to_usize(GLOBAL_DATA, 1860);
					
					if drain_start < smallvec.len() && drain_end <= smallvec.len() && drain_start <= drain_end {
						let mut drain_iter = smallvec.drain(drain_start..drain_end);
						if let Some(drained_item) = drain_iter.next() {
							println!("{}", drained_item);
						}
					}
					
					let reserve_exact_amount = _to_usize(GLOBAL_DATA, 1900);
					smallvec.reserve_exact(reserve_exact_amount);
					
					let grow_amount = _to_usize(GLOBAL_DATA, 1920);
					smallvec.grow(grow_amount);
					
					if !smallvec.is_empty() {
						let index_to_access = _to_usize(GLOBAL_DATA, 1950) % smallvec.len();
						let indexed_value = &smallvec[index_to_access];
						println!("{}", *indexed_value);
					}
					
					smallvec.dedup_by(|a, b| *a == *b);
				},
				9 => {
					let constructor_choice = _to_u8(GLOBAL_DATA, 2000) % 4;
					let mut smallvec = match constructor_choice {
						0 => smallvec::SmallVec::<[u32; 32]>::new(),
						1 => smallvec::SmallVec::<[u32; 32]>::with_capacity(_to_usize(GLOBAL_DATA, 2010)),
						2 => {
							let elem = _to_u32(GLOBAL_DATA, 2020);
							let count = _to_usize(GLOBAL_DATA, 2030) % 20;
							smallvec::SmallVec::<[u32; 32]>::from_elem(elem, count)
						},
						_ => {
							let data_vec = vec![_to_u32(GLOBAL_DATA, 2040), _to_u32(GLOBAL_DATA, 2050)];
							smallvec::SmallVec::<[u32; 32]>::from_vec(data_vec)
						}
					};
					
					let extend_iter = vec![_to_u32(GLOBAL_DATA, 2100), _to_u32(GLOBAL_DATA, 2110), _to_u32(GLOBAL_DATA, 2120)];
					smallvec.extend(extend_iter.into_iter());
					
					let insert_many_index = _to_usize(GLOBAL_DATA, 2150);
					let insert_many_data = vec![_to_u32(GLOBAL_DATA, 2160), _to_u32(GLOBAL_DATA, 2170)];
					if insert_many_index <= smallvec.len() {
						smallvec.insert_many(insert_many_index, insert_many_data.into_iter());
					}
					
					let try_reserve_result = smallvec.try_reserve(_to_usize(GLOBAL_DATA, 2200));
					if try_reserve_result.is_ok() {
						println!("Reserve successful");
					}
					
					let try_reserve_exact_result = smallvec.try_reserve_exact(_to_usize(GLOBAL_DATA, 2220));
					if try_reserve_exact_result.is_ok() {
						println!("Reserve exact successful");
					}
					
					if !smallvec.is_empty() {
						let as_ref_result = smallvec.as_ref();
						if as_ref_result.len() > 0 {
							println!("{}", as_ref_result[0]);
						}
					}
				},
				10 => {
					let mut smallvec = smallvec::SmallVec::<[usize; 16]>::new();
					
					let item_count = _to_u8(GLOBAL_DATA, 2300) % 20;
					for i in 0..item_count {
						let item_val = _to_usize(GLOBAL_DATA, 2301 + (i as usize * 8));
						smallvec.push(item_val);
					}
					
					let dedup_by_key_selector = _to_u8(GLOBAL_DATA, 2400) % 3;
					match dedup_by_key_selector {
						0 => smallvec.dedup_by_key(|x| *x % 10),
						1 => smallvec.dedup_by_key(|x| *x / 100),
						_ => smallvec.dedup_by_key(|x| *x),
					}
					
					let try_grow_amount = _to_usize(GLOBAL_DATA, 2450);
					let try_grow_result = smallvec.try_grow(try_grow_amount);
					if try_grow_result.is_ok() {
						println!("Grow successful");
					}
					
					if !smallvec.is_empty() {
						let as_mut_result = smallvec.as_mut();
						if as_mut_result.len() > 0 {
							let first_mut = &mut as_mut_result[0];
							println!("{}", *first_mut);
						}
					}
					
					let into_inner_result = smallvec.into_inner();
					match into_inner_result {
						Ok(inner_array) => {
							println!("Into inner successful, size: {}", <[usize; 16] as smallvec::Array>::size());
						},
						Err(vec_back) => {
							println!("Into inner failed, len: {}", vec_back.len());
						}
					}
				},
				_ => {
					let mut final_smallvec = smallvec::SmallVec::<[CustomType1; 16]>::new();
					
					let final_count = _to_u8(GLOBAL_DATA, 1600) % 32;
					for i in 0..final_count {
						let val = _to_usize(GLOBAL_DATA, 1601 + (i as usize * 8));
						let item = CustomType1(val);
						final_smallvec.push(item);
					}
					
					let final_ref = &final_smallvec;
					let mut final_hasher = std::collections::hash_map::DefaultHasher::new();
					final_ref.hash(&mut final_hasher);
					
					let final_slice = final_smallvec.as_slice();
					println!("{}", final_slice.len());
					
					if !final_smallvec.is_empty() {
						let final_clone = final_smallvec.clone();
						let final_clone_len = final_clone.len();
						println!("{}", final_clone_len);
					}
					
					let into_boxed_slice_result = final_smallvec.into_boxed_slice();
					println!("{}", into_boxed_slice_result.len());
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