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
		if data.len() < 300 {return;}
		set_global_data(data);
		let global_data = get_global_data();
		let GLOBAL_DATA = global_data.first_half;
		
		let num_operations = _to_u8(GLOBAL_DATA, 0) % 65;
		
		for op_idx in 0..num_operations {
			let operation = _to_u8(GLOBAL_DATA, 1 + op_idx as usize) % 20;
			
			match operation {
				0 => {
					let vec_choice = _to_u8(GLOBAL_DATA, 70 + op_idx as usize) % 3;
					let smallvec_instance = match vec_choice {
						0 => {
							let capacity = _to_usize(GLOBAL_DATA, 80 + op_idx as usize * 8);
							smallvec::SmallVec::<[u32; 16]>::with_capacity(capacity)
						},
						1 => {
							let elem = _to_u32(GLOBAL_DATA, 88 + op_idx as usize * 4);
							let count = _to_usize(GLOBAL_DATA, 92 + op_idx as usize * 8);
							smallvec::SmallVec::<[u32; 16]>::from_elem(elem, count)
						},
						_ => {
							let vec_data = vec![_to_u32(GLOBAL_DATA, 100 + op_idx as usize * 4); (_to_u8(GLOBAL_DATA, 104 + op_idx as usize) % 15) as usize];
							smallvec::SmallVec::<[u32; 16]>::from_vec(vec_data)
						}
					};
					
					let iter = (&smallvec_instance).into_iter();
					for item in iter {
						println!("{:?}", *item);
					}
					
					let spilled = smallvec_instance.spilled();
					println!("{}", spilled);
					
					let len = smallvec_instance.len();
					println!("{}", len);
					
					let capacity = smallvec_instance.capacity();
					println!("{}", capacity);
					
					let as_slice = smallvec_instance.as_slice();
					println!("{:?}", as_slice);
				},
				1 => {
					let capacity = _to_usize(GLOBAL_DATA, 110 + op_idx as usize * 8);
					let mut sv = smallvec::SmallVec::<[i32; 20]>::with_capacity(capacity);
					
					let push_count = _to_u8(GLOBAL_DATA, 118 + op_idx as usize) % 25;
					for i in 0..push_count {
						let value = _to_i32(GLOBAL_DATA, 120 + i as usize * 4);
						sv.push(value);
					}
					
					let sv_ref = &sv;
					let iter = sv_ref.into_iter();
					for element in iter {
						println!("{:?}", *element);
					}
					
					let len = sv.len();
					println!("{}", len);
					
					let capacity = sv.capacity();
					println!("{}", capacity);
					
					let is_empty = sv.is_empty();
					println!("{}", is_empty);
					
					sv.reserve(_to_usize(GLOBAL_DATA, 130 + op_idx as usize * 8));
					sv.shrink_to_fit();
				},
				2 => {
					let slice_data = &[_to_u64(GLOBAL_DATA, 150), _to_u64(GLOBAL_DATA, 158), _to_u64(GLOBAL_DATA, 166)];
					let sv = smallvec::SmallVec::<[u64; 12]>::from_slice(slice_data);
					
					let sv_ref = &sv;
					let iterator = sv_ref.into_iter();
					for val in iterator {
						println!("{:?}", *val);
					}
					
					let as_slice = sv.as_slice();
					println!("{:?}", as_slice);
					
					let sv2 = sv.clone();
					let comparison = sv.cmp(&sv2);
					println!("{:?}", comparison);
					
					let as_ptr = sv.as_ptr();
					println!("{:?}", as_ptr);
				},
				3 => {
					let initial_vec = vec![_to_f32(GLOBAL_DATA, 170); (_to_u8(GLOBAL_DATA, 174) % 10) as usize];
					let mut sv = smallvec::SmallVec::<[f32; 14]>::from_vec(initial_vec);
					
					let sv_ref = &sv;
					let iter_result = sv_ref.into_iter();
					for float_val in iter_result {
						println!("{:?}", *float_val);
					}
					
					let extend_count = _to_u8(GLOBAL_DATA, 175) % 20;
					for i in 0..extend_count {
						let new_val = _to_f32(GLOBAL_DATA, 176 + i as usize * 4);
						sv.push(new_val);
					}
					
					if !sv.is_empty() {
						let popped = sv.pop();
						if let Some(val) = popped {
							println!("{:?}", val);
						}
					}
					
					sv.truncate(_to_usize(GLOBAL_DATA, 180 + op_idx as usize * 8));
				},
				4 => {
					let arr_data = [_to_i16(GLOBAL_DATA, 180), _to_i16(GLOBAL_DATA, 182), _to_i16(GLOBAL_DATA, 184), _to_i16(GLOBAL_DATA, 186)];
					let sv = smallvec::SmallVec::<[i16; 15]>::from_slice(&arr_data);
					
					let sv_reference = &sv;
					let into_iter_ref = sv_reference.into_iter();
					for short_val in into_iter_ref {
						println!("{:?}", *short_val);
					}
					
					let spilled = sv.spilled();
					println!("{}", spilled);
					
					let deref_slice = sv.deref();
					println!("{:?}", deref_slice);
					
					let as_ref_slice = sv.as_ref();
					println!("{:?}", as_ref_slice);
				},
				5 => {
					let elem = _to_u8(GLOBAL_DATA, 190);
					let count = _to_usize(GLOBAL_DATA, 191);
					let mut sv = smallvec::SmallVec::<[u8; 32]>::from_elem(elem, count);
					
					let reference = &sv;
					let iter_on_ref = reference.into_iter();
					for byte_val in iter_on_ref {
						println!("{:?}", *byte_val);
					}
					
					let truncate_len = _to_usize(GLOBAL_DATA, 195);
					sv.truncate(truncate_len);
					
					let reserve_additional = _to_usize(GLOBAL_DATA, 197);
					sv.reserve(reserve_additional);
					
					sv.shrink_to_fit();
					
					if sv.len() > 0 {
						let removed = sv.remove(_to_usize(GLOBAL_DATA, 199) % sv.len());
						println!("{:?}", removed);
					}
				},
				6 => {
					let mut sv = smallvec::SmallVec::<[char; 24]>::new();
					let char_count = _to_u8(GLOBAL_DATA, 200) % 30;
					for i in 0..char_count {
						let ch = _to_char(GLOBAL_DATA, 201 + i as usize * 4);
						sv.push(ch);
					}
					
					let sv_ref = &sv;
					let char_iter = sv_ref.into_iter();
					for character in char_iter {
						println!("{:?}", *character);
					}
					
					if sv.len() > 0 {
						let index = _to_usize(GLOBAL_DATA, 230) % sv.len();
						let indexed_ref = &sv[index];
						println!("{:?}", *indexed_ref);
					}
					
					sv.clear();
					
					let insert_count = _to_u8(GLOBAL_DATA, 240) % 20;
					for i in 0..insert_count {
						let ch = _to_char(GLOBAL_DATA, 241 + i as usize * 4);
						sv.push(ch);
					}
				},
				7 => {
					let bool_array = [_to_bool(GLOBAL_DATA, 240), _to_bool(GLOBAL_DATA, 241), _to_bool(GLOBAL_DATA, 242)];
					let arr18 = [
						_to_bool(GLOBAL_DATA, 245), _to_bool(GLOBAL_DATA, 246), _to_bool(GLOBAL_DATA, 247),
						_to_bool(GLOBAL_DATA, 248), _to_bool(GLOBAL_DATA, 249), _to_bool(GLOBAL_DATA, 250),
						_to_bool(GLOBAL_DATA, 251), _to_bool(GLOBAL_DATA, 252), _to_bool(GLOBAL_DATA, 253),
						_to_bool(GLOBAL_DATA, 254), _to_bool(GLOBAL_DATA, 255), _to_bool(GLOBAL_DATA, 256),
						_to_bool(GLOBAL_DATA, 257), _to_bool(GLOBAL_DATA, 258), _to_bool(GLOBAL_DATA, 259),
						_to_bool(GLOBAL_DATA, 260), _to_bool(GLOBAL_DATA, 261), _to_bool(GLOBAL_DATA, 262)
					];
					let sv = smallvec::SmallVec::<[bool; 18]>::from_buf(arr18);
					
					let ref_to_sv = &sv;
					let bool_iter = ref_to_sv.into_iter();
					for boolean in bool_iter {
						println!("{:?}", *boolean);
					}
					
					let as_ref_slice = sv.as_ref();
					println!("{:?}", as_ref_slice);
					
					let into_vec = sv.into_vec();
					println!("{:?}", into_vec);
				},
				8 => {
					let iter_data = vec![_to_i64(GLOBAL_DATA, 250), _to_i64(GLOBAL_DATA, 258), _to_i64(GLOBAL_DATA, 266)];
					let sv = smallvec::SmallVec::<[i64; 14]>::from_iter(iter_data.into_iter());
					
					let sv_borrowed = &sv;
					let from_ref_iter = sv_borrowed.into_iter();
					for long_val in from_ref_iter {
						println!("{:?}", *long_val);
					}
					
					let as_ptr = sv.as_ptr();
					println!("{:?}", as_ptr);
					
					let boxed = sv.into_boxed_slice();
					println!("{:?}", boxed);
				},
				9 => {
					let slice_ref = &[_to_u128(GLOBAL_DATA, 270), _to_u128(GLOBAL_DATA, 286)];
					let sv: smallvec::SmallVec<[u128; 12]> = slice_ref.to_smallvec();
					
					let sv_reference = &sv;
					let ref_iterator = sv_reference.into_iter();
					for huge_val in ref_iterator {
						println!("{:?}", *huge_val);
					}
					
					let inner_result = sv.into_inner();
					match inner_result {
						Ok(array) => println!("Got array"),
						Err(vec) => println!("Got vec back"),
					}
				},
				10 => {
					let capacity = _to_usize(GLOBAL_DATA, 280 + op_idx as usize * 8);
					let mut sv = smallvec::SmallVec::<[String; 16]>::with_capacity(capacity);
					
					let string_count = _to_u8(GLOBAL_DATA, 290 + op_idx as usize) % 10;
					for i in 0..string_count {
						let start = 300 + i as usize * 10;
						let end = start + 8;
						let string_val = _to_str(GLOBAL_DATA, start, end).to_string();
						sv.push(string_val);
					}
					
					let iter = (&sv).into_iter();
					for string_item in iter {
						println!("{:?}", *string_item);
					}
					
					if !sv.is_empty() {
						let index = _to_usize(GLOBAL_DATA, 320 + op_idx as usize * 8) % sv.len();
						let swapped = sv.swap_remove(index);
						println!("{:?}", swapped);
					}
				},
				11 => {
					let mut sv = smallvec::SmallVec::<[Vec<u8>; 12]>::new();
					
					let vec_count = _to_u8(GLOBAL_DATA, 350 + op_idx as usize) % 8;
					for i in 0..vec_count {
						let vec_size = (_to_u8(GLOBAL_DATA, 360 + i as usize) % 5) as usize;
						let mut inner_vec = Vec::with_capacity(vec_size);
						for j in 0..vec_size {
							inner_vec.push(_to_u8(GLOBAL_DATA, 370 + i as usize * 5 + j));
						}
						sv.push(inner_vec);
					}
					
					let sv_ref = &sv;
					let iter = sv_ref.into_iter();
					for vec_item in iter {
						println!("{:?}", *vec_item);
					}
					
					let as_mut_slice = sv.as_mut_slice();
					println!("{:?}", as_mut_slice);
				},
				12 => {
					let elem = _to_i32(GLOBAL_DATA, 400 + op_idx as usize * 4);
					let count = _to_usize(GLOBAL_DATA, 410 + op_idx as usize * 8);
					let mut sv = smallvec::SmallVec::<[i32; 20]>::from_elem(elem, count);
					
					let iterator = (&sv).into_iter();
					for item in iterator {
						println!("{:?}", *item);
					}
					
					let new_elem = _to_i32(GLOBAL_DATA, 420 + op_idx as usize * 4);
					let new_count = _to_usize(GLOBAL_DATA, 425 + op_idx as usize * 8);
					sv.resize(new_count, new_elem);
					
					let drain_start = _to_usize(GLOBAL_DATA, 430 + op_idx as usize * 8);
					let drain_end = _to_usize(GLOBAL_DATA, 435 + op_idx as usize * 8);
					if drain_start < sv.len() {
						let actual_end = std::cmp::min(drain_end, sv.len());
						let actual_start = std::cmp::min(drain_start, actual_end);
						let drain_iter = sv.drain(actual_start..actual_end);
						for drained in drain_iter {
							println!("{:?}", drained);
						}
					}
				},
				13 => {
					let data1 = vec![_to_f64(GLOBAL_DATA, 450), _to_f64(GLOBAL_DATA, 458), _to_f64(GLOBAL_DATA, 466)];
					let mut sv1 = smallvec::SmallVec::<[f64; 16]>::from_iter(data1);
					
					let data2 = vec![_to_f64(GLOBAL_DATA, 470), _to_f64(GLOBAL_DATA, 478)];
					let mut sv2 = smallvec::SmallVec::<[f64; 16]>::from_iter(data2);
					
					let sv1_ref = &sv1;
					let iter1 = sv1_ref.into_iter();
					for item1 in iter1 {
						println!("{:?}", *item1);
					}
					
					sv1.append(&mut sv2);
					println!("{}", sv2.len());
					
					let partial_cmp_result = sv1.partial_cmp(&sv2);
					println!("{:?}", partial_cmp_result);
				},
				14 => {
					let bytes = [_to_u8(GLOBAL_DATA, 480), _to_u8(GLOBAL_DATA, 481), _to_u8(GLOBAL_DATA, 482), _to_u8(GLOBAL_DATA, 483)];
					let mut sv = smallvec::SmallVec::<[u8; 24]>::from_slice(&bytes);
					
					let reference = &sv;
					let iter = reference.into_iter();
					for byte_val in iter {
						println!("{:?}", *byte_val);
					}
					
					let insert_idx = _to_usize(GLOBAL_DATA, 485 + op_idx as usize * 8);
					let insert_elem = _to_u8(GLOBAL_DATA, 490 + op_idx as usize);
					if insert_idx <= sv.len() {
						sv.insert(insert_idx, insert_elem);
					}
					
					let extend_slice = &[_to_u8(GLOBAL_DATA, 495), _to_u8(GLOBAL_DATA, 496)];
					sv.extend_from_slice(extend_slice);
				},
				15 => {
					let elem = _to_u16(GLOBAL_DATA, 500 + op_idx as usize * 2);
					let count = _to_usize(GLOBAL_DATA, 510 + op_idx as usize * 8);
					let mut sv = smallvec::SmallVec::<[u16; 32]>::from_elem(elem, count);
					
					let sv_iter = (&sv).into_iter();
					for elem_ref in sv_iter {
						println!("{:?}", *elem_ref);
					}
					
					let closure_op = _to_u8(GLOBAL_DATA, 520 + op_idx as usize);
					sv.retain(|x| {
						if closure_op % 2 == 0 {
							*x > 100
						} else {
							*x < 30000
						}
					});
					
					sv.dedup();
				},
				16 => {
					let capacity = _to_usize(GLOBAL_DATA, 530 + op_idx as usize * 8);
					let mut sv = smallvec::SmallVec::<[isize; 20]>::with_capacity(capacity);
					
					let elem_count = _to_u8(GLOBAL_DATA, 540 + op_idx as usize) % 15;
					for i in 0..elem_count {
						let val = _to_isize(GLOBAL_DATA, 550 + i as usize * 8);
						sv.push(val);
					}
					
					let iterator = (&sv).into_iter();
					for item in iterator {
						println!("{:?}", *item);
					}
					
					let grow_cap = _to_usize(GLOBAL_DATA, 570 + op_idx as usize * 8);
					sv.grow(grow_cap);
					
					let hash_val = _to_u64(GLOBAL_DATA, 580 + op_idx as usize * 8);
					let mut hasher = std::collections::hash_map::DefaultHasher::new();
					use std::hash::Hash;
					sv.hash(&mut hasher);
				},
				17 => {
					let bytes = &[_to_u8(GLOBAL_DATA, 590), _to_u8(GLOBAL_DATA, 591), _to_u8(GLOBAL_DATA, 592)];
					let sv: smallvec::SmallVec<[u8; 18]> = bytes.to_smallvec();
					
					let sv_ref = &sv;
					let iterator = sv_ref.into_iter();
					for byte_item in iterator {
						println!("{:?}", *byte_item);
					}
					
					let borrow_slice = std::borrow::Borrow::<[u8]>::borrow(&sv);
					println!("{:?}", borrow_slice);
					
					let eq_check = sv.eq(&sv);
					println!("{}", eq_check);
				},
				18 => {
					let array_elems = [_to_i8(GLOBAL_DATA, 600), _to_i8(GLOBAL_DATA, 601), _to_i8(GLOBAL_DATA, 602), _to_i8(GLOBAL_DATA, 603)];
					let mut sv = smallvec::SmallVec::<[i8; 25]>::from(&array_elems[..]);
					
					let reference = &sv;
					let iter = reference.into_iter();
					for elem in iter {
						println!("{:?}", *elem);
					}
					
					let insert_slice = &[_to_i8(GLOBAL_DATA, 610), _to_i8(GLOBAL_DATA, 611)];
					let insert_pos = _to_usize(GLOBAL_DATA, 615 + op_idx as usize * 8);
					if insert_pos <= sv.len() {
						sv.insert_from_slice(insert_pos, insert_slice);
					}
					
					let as_mut_ptr = sv.as_mut_ptr();
					println!("{:?}", as_mut_ptr);
				},
				_ => {
					let vec_choice = _to_u8(GLOBAL_DATA, 620 + op_idx as usize) % 4;
					let sv = match vec_choice {
						0 => smallvec::SmallVec::<[usize; 16]>::new(),
						1 => {
							let cap = _to_usize(GLOBAL_DATA, 630 + op_idx as usize * 8);
							smallvec::SmallVec::<[usize; 16]>::with_capacity(cap)
						},
						2 => {
							let elem = _to_usize(GLOBAL_DATA, 640 + op_idx as usize * 8);
							let count = _to_usize(GLOBAL_DATA, 650 + op_idx as usize * 8);
							smallvec::SmallVec::<[usize; 16]>::from_elem(elem, count)
						},
						_ => {
							let slice_data = &[_to_usize(GLOBAL_DATA, 660), _to_usize(GLOBAL_DATA, 668)];
							smallvec::SmallVec::<[usize; 16]>::from_slice(slice_data)
						}
					};
					
					let reference = &sv;
					let iterator = reference.into_iter();
					for val in iterator {
						println!("{:?}", *val);
					}
					
					let is_empty = sv.is_empty();
					println!("{}", is_empty);
					
					let inline_size = sv.inline_size();
					println!("{}", inline_size);
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