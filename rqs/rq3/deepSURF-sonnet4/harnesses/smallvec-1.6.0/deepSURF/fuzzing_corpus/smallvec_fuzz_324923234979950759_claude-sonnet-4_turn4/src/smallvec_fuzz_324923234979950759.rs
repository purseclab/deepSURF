#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};
use std::borrow::{Borrow, BorrowMut};

#[derive(Debug)]
struct CustomType0(String);

#[derive(Debug)]
struct CustomType1(usize);

impl core::marker::Copy for CustomType1 {
}

impl core::clone::Clone for CustomType1 {
	fn clone(&self) -> Self {
		let global_data = get_global_data();
		let GLOBAL_DATA = global_data.first_half;
		let custom_impl_num = _to_usize(GLOBAL_DATA, 35);
		let custom_impl_inst_num = self.0;
		let selector = (custom_impl_num + custom_impl_inst_num) % 3;
		if selector == 0{
			panic!("INTENTIONAL PANIC!");
		}
		let GLOBAL_DATA = match selector{
				1 => global_data.first_half,
				_ => global_data.second_half,
		};
		let t_11 = _to_usize(GLOBAL_DATA, 43);
		let t_12 = CustomType1(t_11);
		return t_12;
	}
}

fn main (){
	fuzz_nohook!(|data: &[u8]| {
		if data.len() < 3072 {return;}
		set_global_data(data);
		let global_data = get_global_data();
		let GLOBAL_DATA = global_data.first_half;
		
		let num_operations = _to_usize(GLOBAL_DATA, 0) % 65 + 1;
		
		let constructor_selector1 = _to_u8(GLOBAL_DATA, 8) % 3;
		let constructor_selector2 = _to_u8(GLOBAL_DATA, 9) % 3;
		let constructor_selector3 = _to_u8(GLOBAL_DATA, 10) % 3;
		
		let mut t_1 = match constructor_selector1 {
			0 => SmallVec::<[i32; 16]>::new(),
			1 => SmallVec::<[i32; 16]>::with_capacity(_to_usize(GLOBAL_DATA, 11)),
			_ => {
				let elem_data = _to_i32(GLOBAL_DATA, 19);
				let elem_count = _to_usize(GLOBAL_DATA, 23) % 65;
				SmallVec::<[i32; 16]>::from_elem(elem_data, elem_count)
			}
		};
		
		let mut t_2 = match constructor_selector2 {
			0 => SmallVec::<[u8; 32]>::new(),
			1 => SmallVec::<[u8; 32]>::with_capacity(_to_usize(GLOBAL_DATA, 31)),
			_ => {
				let elem_data = _to_u8(GLOBAL_DATA, 39);
				let elem_count = _to_usize(GLOBAL_DATA, 40) % 65;
				SmallVec::<[u8; 32]>::from_elem(elem_data, elem_count)
			}
		};
		
		let mut t_3 = match constructor_selector3 {
			0 => SmallVec::<[CustomType1; 12]>::new(),
			1 => SmallVec::<[CustomType1; 12]>::with_capacity(_to_usize(GLOBAL_DATA, 48)),
			_ => {
				let elem_data = CustomType1(_to_usize(GLOBAL_DATA, 56));
				let elem_count = _to_usize(GLOBAL_DATA, 64) % 65;
				SmallVec::<[CustomType1; 12]>::from_elem(elem_data, elem_count)
			}
		};
		
		for op_idx in 0..num_operations {
			let offset = 72 + op_idx * 32;
			if offset + 31 >= GLOBAL_DATA.len() { break; }
			
			let operation = _to_u8(GLOBAL_DATA, offset) % 15;
			
			match operation {
				0 => {
					let vec_selector = _to_u8(GLOBAL_DATA, offset + 1) % 3;
					let elem_count = _to_u8(GLOBAL_DATA, offset + 2) % 33;
					
					match vec_selector {
						0 => {
							let mut vec = Vec::with_capacity(elem_count as usize);
							for i in 0..elem_count {
								let val = _to_i32(GLOBAL_DATA, offset + 3 + (i as usize * 4));
								vec.push(val);
							}
							let slice = &vec[..];
							t_1.extend_from_slice(slice);
							println!("{:?}", t_1.as_slice());
							let capacity_ref = &t_1.capacity();
							println!("{}", *capacity_ref);
						},
						1 => {
							let mut vec = Vec::with_capacity(elem_count as usize);
							for i in 0..elem_count {
								let val = _to_u8(GLOBAL_DATA, offset + 3 + i as usize);
								vec.push(val);
							}
							let slice = &vec[..];
							t_2.extend_from_slice(slice);
							println!("{:?}", t_2.as_slice());
							let len_ref = &t_2.len();
							println!("{}", *len_ref);
						},
						_ => {
							let mut vec = Vec::with_capacity(elem_count as usize);
							for i in 0..elem_count {
								let val = _to_usize(GLOBAL_DATA, offset + 3 + (i as usize * 8));
								vec.push(CustomType1(val));
							}
							let slice = &vec[..];
							t_3.extend_from_slice(slice);
							let deref_slice = &*t_3;
							println!("{:?}", deref_slice.len());
						}
					}
				},
				1 => {
					let vec_selector = _to_u8(GLOBAL_DATA, offset + 1) % 3;
					let additional_cap = _to_usize(GLOBAL_DATA, offset + 2);
					
					match vec_selector {
						0 => {
							t_1.reserve(additional_cap);
							let cap_ref = &t_1.capacity();
							println!("{}", *cap_ref);
							let spilled_result = t_1.spilled();
							println!("{}", spilled_result);
						},
						1 => {
							t_2.reserve(additional_cap);
							let cap_ref = &t_2.capacity();
							println!("{}", *cap_ref);
							let spilled_result = t_2.spilled();
							println!("{}", spilled_result);
						},
						_ => {
							t_3.reserve(additional_cap);
							let cap_ref = &t_3.capacity();
							println!("{}", *cap_ref);
							let spilled_result = t_3.spilled();
							println!("{}", spilled_result);
						}
					}
				},
				2 => {
					let vec_selector = _to_u8(GLOBAL_DATA, offset + 1) % 3;
					let index = _to_usize(GLOBAL_DATA, offset + 2);
					
					match vec_selector {
						0 => {
							if !t_1.is_empty() {
								let val = _to_i32(GLOBAL_DATA, offset + 10);
								let insert_idx = index % t_1.len();
								t_1.insert(insert_idx, val);
								let as_ref_slice = t_1.as_ref();
								println!("{:?}", as_ref_slice);
								let inline_size = t_1.inline_size();
								println!("{}", inline_size);
							}
						},
						1 => {
							if !t_2.is_empty() {
								let val = _to_u8(GLOBAL_DATA, offset + 10);
								let insert_idx = index % t_2.len();
								t_2.insert(insert_idx, val);
								let deref_slice = &*t_2;
								println!("{:?}", deref_slice);
								let inline_size = t_2.inline_size();
								println!("{}", inline_size);
							}
						},
						_ => {
							if !t_3.is_empty() {
								let val = _to_usize(GLOBAL_DATA, offset + 10);
								let insert_idx = index % t_3.len();
								t_3.insert(insert_idx, CustomType1(val));
								let deref_slice = &*t_3;
								println!("{:?}", deref_slice.len());
								let inline_size = t_3.inline_size();
								println!("{}", inline_size);
							}
						}
					}
				},
				3 => {
					let vec_selector = _to_u8(GLOBAL_DATA, offset + 1) % 3;
					let new_len = _to_usize(GLOBAL_DATA, offset + 2);
					
					match vec_selector {
						0 => {
							t_1.truncate(new_len);
							println!("{:?}", t_1.as_slice());
							let is_empty_result = t_1.is_empty();
							println!("{}", is_empty_result);
						},
						1 => {
							t_2.truncate(new_len);
							println!("{:?}", t_2.as_slice());
							let is_empty_result = t_2.is_empty();
							println!("{}", is_empty_result);
						},
						_ => {
							t_3.truncate(new_len);
							let deref_slice = &*t_3;
							println!("{:?}", deref_slice.len());
							let is_empty_result = t_3.is_empty();
							println!("{}", is_empty_result);
						}
					}
				},
				4 => {
					let vec_selector = _to_u8(GLOBAL_DATA, offset + 1) % 3;
					
					match vec_selector {
						0 => {
							let pop_result = t_1.pop();
							if let Some(val) = pop_result {
								println!("{}", val);
							}
							let len_val = t_1.len();
							println!("{}", len_val);
						},
						1 => {
							let pop_result = t_2.pop();
							if let Some(val) = pop_result {
								println!("{}", val);
							}
							let len_val = t_2.len();
							println!("{}", len_val);
						},
						_ => {
							let pop_result = t_3.pop();
							if let Some(val) = pop_result {
								println!("{}", val.0);
							}
							let len_val = t_3.len();
							println!("{}", len_val);
						}
					}
				},
				5 => {
					let vec_selector = _to_u8(GLOBAL_DATA, offset + 1) % 3;
					
					match vec_selector {
						0 => {
							let val = _to_i32(GLOBAL_DATA, offset + 2);
							t_1.push(val);
							let mut_slice = t_1.as_mut_slice();
							println!("{:?}", mut_slice);
							let as_ptr_result = t_1.as_ptr();
							println!("{:?}", as_ptr_result);
						},
						1 => {
							let val = _to_u8(GLOBAL_DATA, offset + 2);
							t_2.push(val);
							let mut_slice = t_2.as_mut_slice();
							println!("{:?}", mut_slice);
							let as_ptr_result = t_2.as_ptr();
							println!("{:?}", as_ptr_result);
						},
						_ => {
							let val = _to_usize(GLOBAL_DATA, offset + 2);
							t_3.push(CustomType1(val));
							let deref_mut_slice = &mut *t_3;
							println!("{:?}", deref_mut_slice.len());
							let as_ptr_result = t_3.as_ptr();
							println!("{:?}", as_ptr_result);
						}
					}
				},
				6 => {
					let vec_selector = _to_u8(GLOBAL_DATA, offset + 1) % 3;
					
					match vec_selector {
						0 => {
							t_1.clear();
							println!("{}", t_1.is_empty());
							let borrow_result: &[i32] = t_1.borrow();
							println!("{:?}", borrow_result);
						},
						1 => {
							t_2.clear();
							println!("{}", t_2.is_empty());
							let borrow_result: &[u8] = t_2.borrow();
							println!("{:?}", borrow_result);
						},
						_ => {
							t_3.clear();
							println!("{}", t_3.is_empty());
							let borrow_result: &[CustomType1] = t_3.borrow();
							println!("{:?}", borrow_result.len());
						}
					}
				},
				7 => {
					let vec_selector = _to_u8(GLOBAL_DATA, offset + 1) % 3;
					
					match vec_selector {
						0 => {
							t_1.shrink_to_fit();
							let cap_ref = &t_1.capacity();
							println!("{}", *cap_ref);
							let deref_result = t_1.deref();
							println!("{:?}", deref_result);
						},
						1 => {
							t_2.shrink_to_fit();
							let cap_ref = &t_2.capacity();
							println!("{}", *cap_ref);
							let deref_result = t_2.deref();
							println!("{:?}", deref_result);
						},
						_ => {
							t_3.shrink_to_fit();
							let cap_ref = &t_3.capacity();
							println!("{}", *cap_ref);
							let deref_result = t_3.deref();
							println!("{:?}", deref_result.len());
						}
					}
				},
				8 => {
					let vec_selector = _to_u8(GLOBAL_DATA, offset + 1) % 3;
					let index = _to_usize(GLOBAL_DATA, offset + 2);
					
					match vec_selector {
						0 => {
							if !t_1.is_empty() {
								let remove_idx = index % t_1.len();
								let removed = t_1.remove(remove_idx);
								println!("{}", removed);
								let as_mut_ptr_result = t_1.as_mut_ptr();
								println!("{:?}", as_mut_ptr_result);
							}
						},
						1 => {
							if !t_2.is_empty() {
								let remove_idx = index % t_2.len();
								let removed = t_2.remove(remove_idx);
								println!("{}", removed);
								let as_mut_ptr_result = t_2.as_mut_ptr();
								println!("{:?}", as_mut_ptr_result);
							}
						},
						_ => {
							if !t_3.is_empty() {
								let remove_idx = index % t_3.len();
								let removed = t_3.remove(remove_idx);
								println!("{}", removed.0);
								let as_mut_ptr_result = t_3.as_mut_ptr();
								println!("{:?}", as_mut_ptr_result);
							}
						}
					}
				},
				9 => {
					let vec_selector = _to_u8(GLOBAL_DATA, offset + 1) % 3;
					let index = _to_usize(GLOBAL_DATA, offset + 2);
					
					match vec_selector {
						0 => {
							if !t_1.is_empty() {
								let swap_idx = index % t_1.len();
								let swapped = t_1.swap_remove(swap_idx);
								println!("{}", swapped);
								let borrow_mut_result: &mut [i32] = t_1.borrow_mut();
								println!("{:?}", borrow_mut_result);
							}
						},
						1 => {
							if !t_2.is_empty() {
								let swap_idx = index % t_2.len();
								let swapped = t_2.swap_remove(swap_idx);
								println!("{}", swapped);
								let borrow_mut_result: &mut [u8] = t_2.borrow_mut();
								println!("{:?}", borrow_mut_result);
							}
						},
						_ => {
							if !t_3.is_empty() {
								let swap_idx = index % t_3.len();
								let swapped = t_3.swap_remove(swap_idx);
								println!("{}", swapped.0);
								let borrow_mut_result: &mut [CustomType1] = t_3.borrow_mut();
								println!("{:?}", borrow_mut_result.len());
							}
						}
					}
				},
				10 => {
					let vec_selector1 = _to_u8(GLOBAL_DATA, offset + 1) % 3;
					let vec_selector2 = _to_u8(GLOBAL_DATA, offset + 2) % 3;
					
					if vec_selector1 == 0 && vec_selector2 == 0 {
						let cloned = t_1.clone();
						let cmp_result = t_1.cmp(&cloned);
						println!("{:?}", cmp_result);
						let eq_result = t_1.eq(&cloned);
						println!("{}", eq_result);
						let partial_cmp_result = t_1.partial_cmp(&cloned);
						println!("{:?}", partial_cmp_result);
					}
				},
				11 => {
					let vec_selector = _to_u8(GLOBAL_DATA, offset + 1) % 3;
					let start = _to_usize(GLOBAL_DATA, offset + 2);
					let end = _to_usize(GLOBAL_DATA, offset + 10);
					
					match vec_selector {
						0 => {
							if !t_1.is_empty() {
								let start_idx = start % t_1.len();
								let end_idx = if end > start_idx { 
									(end % (t_1.len() - start_idx)) + start_idx + 1
								} else {
									start_idx + 1
								};
								let drain_iter = t_1.drain(start_idx..end_idx);
								for item in drain_iter {
									println!("{}", item);
								}
							}
						},
						1 => {
							if !t_2.is_empty() {
								let start_idx = start % t_2.len();
								let end_idx = if end > start_idx { 
									(end % (t_2.len() - start_idx)) + start_idx + 1
								} else {
									start_idx + 1
								};
								let drain_iter = t_2.drain(start_idx..end_idx);
								for item in drain_iter {
									println!("{}", item);
								}
							}
						},
						_ => {
							if !t_3.is_empty() {
								let start_idx = start % t_3.len();
								let end_idx = if end > start_idx { 
									(end % (t_3.len() - start_idx)) + start_idx + 1
								} else {
									start_idx + 1
								};
								let drain_iter = t_3.drain(start_idx..end_idx);
								for item in drain_iter {
									println!("{}", item.0);
								}
							}
						}
					}
				},
				12 => {
					let vec_selector = _to_u8(GLOBAL_DATA, offset + 1) % 3;
					let additional_capacity = _to_usize(GLOBAL_DATA, offset + 2);
					
					match vec_selector {
						0 => {
							t_1.reserve_exact(additional_capacity);
							let cap_result = t_1.capacity();
							println!("{}", cap_result);
						},
						1 => {
							t_2.reserve_exact(additional_capacity);
							let cap_result = t_2.capacity();
							println!("{}", cap_result);
						},
						_ => {
							t_3.reserve_exact(additional_capacity);
							let cap_result = t_3.capacity();
							println!("{}", cap_result);
						}
					}
				},
				13 => {
					let vec_selector = _to_u8(GLOBAL_DATA, offset + 1) % 3;
					let new_capacity = _to_usize(GLOBAL_DATA, offset + 2);
					
					match vec_selector {
						0 => {
							t_1.grow(new_capacity);
							let capacity_result = t_1.capacity();
							println!("{}", capacity_result);
						},
						1 => {
							t_2.grow(new_capacity);
							let capacity_result = t_2.capacity();
							println!("{}", capacity_result);
						},
						_ => {
							t_3.grow(new_capacity);
							let capacity_result = t_3.capacity();
							println!("{}", capacity_result);
						}
					}
				},
				_ => {
					let vec_selector = _to_u8(GLOBAL_DATA, offset + 1) % 3;
					let index = _to_usize(GLOBAL_DATA, offset + 2);
					let count = _to_usize(GLOBAL_DATA, offset + 10) % 65;
					
					match vec_selector {
						0 => {
							let mut to_insert = Vec::with_capacity(count);
							for i in 0..count {
								let elem = _to_i32(GLOBAL_DATA, offset + 18 + (i * 4));
								to_insert.push(elem);
							}
							let insert_idx = if t_1.is_empty() { 0 } else { index % t_1.len() };
							t_1.insert_many(insert_idx, to_insert.into_iter());
							println!("{:?}", t_1.as_slice());
						},
						1 => {
							let mut to_insert = Vec::with_capacity(count);
							for i in 0..count {
								let elem = _to_u8(GLOBAL_DATA, offset + 18 + i);
								to_insert.push(elem);
							}
							let insert_idx = if t_2.is_empty() { 0 } else { index % t_2.len() };
							t_2.insert_many(insert_idx, to_insert.into_iter());
							println!("{:?}", t_2.as_slice());
						},
						_ => {
							let mut to_insert = Vec::with_capacity(count);
							for i in 0..count {
								let elem = CustomType1(_to_usize(GLOBAL_DATA, offset + 18 + (i * 8)));
								to_insert.push(elem);
							}
							let insert_idx = if t_3.is_empty() { 0 } else { index % t_3.len() };
							t_3.insert_many(insert_idx, to_insert.into_iter());
							println!("{:?}", t_3.len());
						}
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