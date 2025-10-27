#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

#[derive(Debug)]
struct CustomType1(String);

#[derive(Debug)]  
struct CustomType0(String);

fn main (){
	fuzz_nohook!(|data: &[u8]| {
		if data.len() < 200 {return;}
		set_global_data(data);
		let global_data = get_global_data();
		let GLOBAL_DATA = global_data.first_half;
		
		let operation_count = _to_u8(GLOBAL_DATA, 0) % 15;
		for i in 0..operation_count {
			let base_index = 1 + (i as usize * 12);
			if base_index + 12 > GLOBAL_DATA.len() { break; }
			
			let operation_type = _to_u8(GLOBAL_DATA, base_index) % 10;
			match operation_type {
				0 => {
					let constructor_choice = _to_u8(GLOBAL_DATA, base_index + 1) % 6;
					let mut smallvec = match constructor_choice {
						0 => smallvec::SmallVec::<[String; 16]>::new(),
						1 => {
							let capacity = _to_usize(GLOBAL_DATA, base_index + 2);
							smallvec::SmallVec::<[String; 16]>::with_capacity(capacity)
						},
						2 => {
							let vec_size = _to_usize(GLOBAL_DATA, base_index + 2) % 65;
							let vec = (0..vec_size).map(|x| x.to_string()).collect::<Vec<_>>();
							smallvec::SmallVec::<[String; 16]>::from_vec(vec)
						},
						3 => {
							let elem_count = _to_usize(GLOBAL_DATA, base_index + 2) % 65;
							let elem = String::from("test");
							smallvec::SmallVec::<[String; 16]>::from_elem(elem, elem_count)
						},
						4 => {
							let slice_size = _to_usize(GLOBAL_DATA, base_index + 2) % 65;
							let slice_data: Vec<String> = (0..slice_size).map(|x| x.to_string()).collect();
							smallvec::SmallVec::<[String; 16]>::from(&slice_data[..])
						},
						_ => {
							let iter_size = _to_usize(GLOBAL_DATA, base_index + 2) % 65;
							let iter = (0..iter_size).map(|x| x.to_string());
							smallvec::SmallVec::<[String; 16]>::from_iter(iter)
						}
					};
					
					let len_before = smallvec.len();
					println!("SmallVec length: {}", len_before);
					
					let push_count = _to_u8(GLOBAL_DATA, base_index + 10) % 10;
					for j in 0..push_count {
						smallvec.push(format!("item{}", j));
					}
					
					if !smallvec.is_empty() {
						let _ = smallvec.pop();
					}
					
					let capacity = smallvec.capacity();
					println!("Capacity: {}", capacity);
					
					if smallvec.len() > 0 {
						let index = _to_usize(GLOBAL_DATA, base_index + 11) % smallvec.len();
						let removed_item = smallvec.remove(index);
						println!("Removed: {}", removed_item);
					}
					
					smallvec.clear();
				},
				1 => {
					let mut smallvec1 = smallvec::SmallVec::<[i32; 32]>::new();
					let mut smallvec2 = smallvec::SmallVec::<[i32; 32]>::new();
					
					let items1 = _to_u8(GLOBAL_DATA, base_index + 1) % 20;
					let items2 = _to_u8(GLOBAL_DATA, base_index + 2) % 20;
					
					for j in 0..items1 { smallvec1.push(j as i32); }
					for j in 0..items2 { smallvec2.push((j + 100) as i32); }
					
					let cmp_result = smallvec1.cmp(&smallvec2);
					println!("Comparison result: {:?}", cmp_result);
					
					let partial_cmp = smallvec1.partial_cmp(&smallvec2);
					if let Some(ordering) = partial_cmp {
						println!("Partial comparison: {:?}", ordering);
					}
					
					smallvec1.append(&mut smallvec2);
					
					let as_slice_ref = smallvec1.as_slice();
					println!("Slice length: {}", as_slice_ref.len());
					for item in as_slice_ref {
						println!("Item: {}", *item);
					}
				},
				2 => {
					let constructor_type = _to_u8(GLOBAL_DATA, base_index + 1) % 4;
					let mut smallvec = match constructor_type {
						0 => smallvec::SmallVec::<[u8; 64]>::new(),
						1 => {
							let capacity = _to_usize(GLOBAL_DATA, base_index + 2);
							smallvec::SmallVec::<[u8; 64]>::with_capacity(capacity)
						},
						2 => {
							let elem = _to_u8(GLOBAL_DATA, base_index + 2);
							let count = _to_usize(GLOBAL_DATA, base_index + 3) % 65;
							smallvec::SmallVec::<[u8; 64]>::from_elem(elem, count)
						},
						_ => {
							let vec_size = _to_usize(GLOBAL_DATA, base_index + 2) % 65;
							let vec = (0..vec_size).map(|x| (x % 256) as u8).collect::<Vec<_>>();
							smallvec::SmallVec::<[u8; 64]>::from_vec(vec)
						}
					};
					
					let reserve_size = _to_usize(GLOBAL_DATA, base_index + 4);
					smallvec.reserve(reserve_size);
					
					let extend_size = _to_u8(GLOBAL_DATA, base_index + 5) % 20;
					let extend_data: Vec<u8> = (0..extend_size).map(|x| x).collect();
					smallvec.extend_from_slice(&extend_data);
					
					if smallvec.len() > 0 {
						let index = _to_usize(GLOBAL_DATA, base_index + 6) % smallvec.len();
						let swap_removed = smallvec.swap_remove(index);
						println!("Swap removed: {}", swap_removed);
					}
					
					let truncate_len = _to_usize(GLOBAL_DATA, base_index + 7) % (smallvec.len() + 1);
					smallvec.truncate(truncate_len);
					
					let as_mut_slice_ref = smallvec.as_mut_slice();
					for item in as_mut_slice_ref.iter_mut() {
						*item = (*item).wrapping_add(1);
						println!("Modified item: {}", *item);
					}
				},
				3 => {
					let mut smallvec = smallvec::SmallVec::<[f32; 24]>::new();
					
					let insert_count = _to_u8(GLOBAL_DATA, base_index + 1) % 15;
					for j in 0..insert_count {
						let value = _to_f32(GLOBAL_DATA, base_index + 2 + (j as usize % 4));
						smallvec.push(value);
					}
					
					if smallvec.len() > 0 {
						let insert_index = _to_usize(GLOBAL_DATA, base_index + 6) % (smallvec.len() + 1);
						let insert_value = _to_f32(GLOBAL_DATA, base_index + 7);
						smallvec.insert(insert_index, insert_value);
					}
					
					let drain_start = if smallvec.len() > 0 { _to_usize(GLOBAL_DATA, base_index + 11) % smallvec.len() } else { 0 };
					let drain_end = if smallvec.len() > drain_start { drain_start + (_to_usize(GLOBAL_DATA, base_index + 10) % (smallvec.len() - drain_start + 1)) } else { drain_start };
					
					let mut drain_iter = smallvec.drain(drain_start..drain_end);
					while let Some(drained_item) = drain_iter.next() {
						println!("Drained: {}", drained_item);
					}
				},
				4 => {
					let constructor_choice = _to_u8(GLOBAL_DATA, base_index + 1) % 3;
					let mut smallvec = match constructor_choice {
						0 => smallvec::SmallVec::<[char; 20]>::new(),
						1 => {
							let capacity = _to_usize(GLOBAL_DATA, base_index + 2);
							smallvec::SmallVec::<[char; 20]>::with_capacity(capacity)
						},
						_ => {
							let char_value = _to_char(GLOBAL_DATA, base_index + 2);
							let count = _to_usize(GLOBAL_DATA, base_index + 6) % 65;
							smallvec::SmallVec::<[char; 20]>::from_elem(char_value, count)
						}
					};
					
					smallvec.shrink_to_fit();
					
					let resize_len = _to_usize(GLOBAL_DATA, base_index + 7) % 50;
					let resize_value = _to_char(GLOBAL_DATA, base_index + 8);
					smallvec.resize(resize_len, resize_value);
					
					let retain_threshold = _to_u32(GLOBAL_DATA, base_index + 9);
					smallvec.retain(|&mut c| (c as u32) > retain_threshold);
					
					for item_ref in smallvec.iter() {
						println!("Character: {}", *item_ref);
					}
				},
				5 => {
					let mut smallvec1 = smallvec::SmallVec::<[bool; 32]>::new();
					let mut smallvec2 = smallvec::SmallVec::<[bool; 32]>::new();
					
					let size1 = _to_u8(GLOBAL_DATA, base_index + 1) % 30;
					let size2 = _to_u8(GLOBAL_DATA, base_index + 2) % 30;
					
					for j in 0..size1 {
						let bool_val = _to_bool(GLOBAL_DATA, base_index + 3 + (j as usize % 4));
						smallvec1.push(bool_val);
					}
					
					for j in 0..size2 {
						let bool_val = _to_bool(GLOBAL_DATA, base_index + 7 + (j as usize % 4));
						smallvec2.push(bool_val);
					}
					
					let eq_result = smallvec1.eq(&smallvec2);
					println!("Equal: {}", eq_result);
					
					if !smallvec1.is_empty() && !smallvec2.is_empty() {
						let index1 = _to_usize(GLOBAL_DATA, base_index + 11) % smallvec1.len();
						let index2 = _to_usize(GLOBAL_DATA, base_index + 10) % smallvec2.len();
						
						let slice1_ref = &smallvec1[index1..];
						let slice2_ref = &smallvec2[index2..];
						println!("Slice1 len: {}, Slice2 len: {}", slice1_ref.len(), slice2_ref.len());
					}
				},
				6 => {
					let mut smallvec = smallvec::SmallVec::<[u64; 12]>::new();
					
					let push_iterations = _to_u8(GLOBAL_DATA, base_index + 1) % 25;
					for j in 0..push_iterations {
						let value = _to_u64(GLOBAL_DATA, base_index + 2 + (j as usize % 8));
						smallvec.push(value);
					}
					
					let into_vec_result = smallvec.clone().into_vec();
					println!("Vector length: {}", into_vec_result.len());
					for vec_item_ref in &into_vec_result {
						println!("Vec item: {}", *vec_item_ref);
					}
					
					let cloned_smallvec = smallvec.clone();
					println!("Cloned length: {}", cloned_smallvec.len());
					
					smallvec.dedup();
					
					if !smallvec.is_empty() {
						let boxed_slice = smallvec.clone().into_boxed_slice();
						for boxed_item_ref in boxed_slice.iter() {
							println!("Boxed item: {}", *boxed_item_ref);
						}
					}
				},
				7 => {
					let array_type = _to_u8(GLOBAL_DATA, base_index + 1) % 3;
					match array_type {
						0 => {
							let array_data: [i16; 15] = [42; 15];
							let mut smallvec = smallvec::SmallVec::from_buf(array_data);
							let len = _to_usize(GLOBAL_DATA, base_index + 2) % 15;
							if len <= smallvec.capacity() {
								let growth_size = _to_usize(GLOBAL_DATA, base_index + 3);
								smallvec.grow(growth_size);
								
								let as_ptr_result = smallvec.as_ptr();
								println!("Pointer: {:?}", as_ptr_result);
								
								let as_mut_ptr_result = smallvec.as_mut_ptr();
								println!("Mut pointer: {:?}", as_mut_ptr_result);
							}
						},
						1 => {
							let array_data: [u32; 18] = [123; 18];
							let len = _to_usize(GLOBAL_DATA, base_index + 2) % 18;
							let mut smallvec = smallvec::SmallVec::from_buf_and_len(array_data, len);
							
							let reserve_exact_size = _to_usize(GLOBAL_DATA, base_index + 3);
							smallvec.reserve_exact(reserve_exact_size);
							
							let try_reserve_size = _to_usize(GLOBAL_DATA, base_index + 4);
							let _ = smallvec.try_reserve(try_reserve_size);
							
							println!("Final length: {}", smallvec.len());
						},
						_ => {
							let mut base_vec = Vec::new();
							let vec_size = _to_u8(GLOBAL_DATA, base_index + 2) % 25;
							for k in 0..vec_size {
								base_vec.push((k as i64) * 7);
							}
							
							let smallvec = smallvec::SmallVec::<[i64; 22]>::from(base_vec);
							let inner_result = smallvec.into_inner();
							match inner_result {
								Ok(inner_array) => {
									println!("Successfully got inner array");
								},
								Err(smallvec_back) => {
									println!("Failed to get inner, got SmallVec back with len: {}", smallvec_back.len());
								}
							}
						}
					}
				},
				8 => {
					let slice_data: [u8; 32] = [255; 32];
					let to_smallvec_result: smallvec::SmallVec<[u8; 32]> = slice_data.to_smallvec();
					println!("ToSmallVec result length: {}", to_smallvec_result.len());
					
					let as_ref_result = to_smallvec_result.as_ref();
					for as_ref_item in as_ref_result.iter() {
						println!("AsRef item: {}", *as_ref_item);
					}
					
					let deref_result = &*to_smallvec_result;
					println!("Deref result length: {}", deref_result.len());
					for deref_item_ref in deref_result {
						println!("Deref item: {}", *deref_item_ref);
					}
				},
				_ => {
					let mut smallvec = smallvec::SmallVec::<[isize; 30]>::new();
					
					let extend_count = _to_u8(GLOBAL_DATA, base_index + 1) % 20;
					let extend_iter = (0..extend_count).map(|x| x as isize);
					smallvec.extend(extend_iter);
					
					let resize_with_size = _to_usize(GLOBAL_DATA, base_index + 2) % 35;
					smallvec.resize_with(resize_with_size, || {
						if _to_u8(GLOBAL_DATA, base_index + 3) % 2 == 0 {
							panic!("INTENTIONAL PANIC!");
						}
						42
					});
					
					if !smallvec.is_empty() {
						let insert_many_index = _to_usize(GLOBAL_DATA, base_index + 4) % (smallvec.len() + 1);
						let insert_many_count = _to_u8(GLOBAL_DATA, base_index + 5) % 10;
						let insert_many_iter = (0..insert_many_count).map(|x| -(x as isize));
						smallvec.insert_many(insert_many_index, insert_many_iter);
					}
					
					smallvec.dedup_by(|a, b| {
						if _to_u8(GLOBAL_DATA, base_index + 6) % 3 == 0 {
							panic!("INTENTIONAL PANIC!");
						}
						*a == *b
					});
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