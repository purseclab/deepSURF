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
		
		let num_operations = _to_u8(GLOBAL_DATA, 0) % 20 + 5;
		for op_idx in 0..num_operations {
			let operation = _to_u8(GLOBAL_DATA, 1 + op_idx as usize) % 12;
			
			match operation {
				0 => {
					let constructor_choice = _to_u8(GLOBAL_DATA, 25 + op_idx as usize) % 3;
					let sv = match constructor_choice {
						0 => smallvec::SmallVec::<[String; 16]>::new(),
						1 => {
							let cap = _to_usize(GLOBAL_DATA, 50 + op_idx as usize * 8);
							smallvec::SmallVec::<[String; 16]>::with_capacity(cap)
						},
						_ => {
							let vec_size = _to_usize(GLOBAL_DATA, 100 + op_idx as usize * 8) % 65;
							let mut vec = Vec::new();
							for i in 0..vec_size {
								let str_len = _to_u8(GLOBAL_DATA, 120 + i + op_idx as usize) % 10 + 1;
								let str_val = _to_str(GLOBAL_DATA, 130 + i * 10 + op_idx as usize, 130 + i * 10 + op_idx as usize + str_len as usize);
								vec.push(String::from(str_val));
							}
							smallvec::SmallVec::<[String; 16]>::from_vec(vec)
						}
					};
					let boxed_slice = sv.into_boxed_slice();
					for item in boxed_slice.iter() {
						println!("{:?}", *item);
					}
				},
				1 => {
					let mut sv = smallvec::SmallVec::<[i32; 20]>::new();
					let push_count = _to_usize(GLOBAL_DATA, 30 + op_idx as usize * 8) % 25;
					for i in 0..push_count {
						let val = _to_i32(GLOBAL_DATA, 60 + i * 4 + op_idx as usize * 32);
						sv.push(val);
					}
					let capacity_before = sv.capacity();
					println!("Capacity before: {}", capacity_before);
					let len_before = sv.len();
					println!("Length before: {}", len_before);
					let spilled = sv.spilled();
					println!("Spilled: {}", spilled);
					
					let boxed_slice = sv.into_boxed_slice();
					for item in boxed_slice.iter() {
						println!("{:?}", *item);
					}
				},
				2 => {
					let elem_count = _to_usize(GLOBAL_DATA, 40 + op_idx as usize * 8) % 30;
					let elem_val = _to_u8(GLOBAL_DATA, 70 + op_idx as usize);
					let sv = smallvec::SmallVec::<[u8; 32]>::from_elem(elem_val, elem_count);
					let len_before = sv.len();
					println!("Length before: {}", len_before);
					let slice_ref = sv.as_slice();
					println!("Slice length: {}", slice_ref.len());
					
					let boxed_slice = sv.into_boxed_slice();
					for item in boxed_slice.iter() {
						println!("{:?}", *item);
					}
				},
				3 => {
					let iter_size = _to_usize(GLOBAL_DATA, 80 + op_idx as usize * 8) % 40;
					let iter_data: Vec<char> = (0..iter_size).map(|i| {
						_to_char(GLOBAL_DATA, 90 + i * 4 + op_idx as usize * 8)
					}).collect();
					let sv = smallvec::SmallVec::<[char; 24]>::from_iter(iter_data.into_iter());
					let is_empty = sv.is_empty();
					println!("Is empty: {}", is_empty);
					let as_ptr = sv.as_ptr();
					println!("Pointer: {:?}", as_ptr);
					
					let boxed_slice = sv.into_boxed_slice();
					for item in boxed_slice.iter() {
						println!("{:?}", *item);
					}
				},
				4 => {
					let mut sv = smallvec::SmallVec::<[f32; 12]>::new();
					let extend_count = _to_usize(GLOBAL_DATA, 110 + op_idx as usize * 8) % 15;
					for i in 0..extend_count {
						let val = _to_f32(GLOBAL_DATA, 140 + i * 4 + op_idx as usize * 16);
						sv.push(val);
					}
					
					let additional_count = _to_usize(GLOBAL_DATA, 150 + op_idx as usize * 8) % 10;
					let extend_data: Vec<f32> = (0..additional_count).map(|i| {
						_to_f32(GLOBAL_DATA, 160 + i * 4 + op_idx as usize * 8)
					}).collect();
					sv.extend(extend_data);
					
					let slice_ref = sv.as_slice();
					println!("Slice length: {}", slice_ref.len());
					
					let boxed_slice = sv.into_boxed_slice();
					for item in boxed_slice.iter() {
						println!("{:?}", *item);
					}
				},
				5 => {
					let mut sv1 = smallvec::SmallVec::<[bool; 18]>::new();
					let mut sv2 = smallvec::SmallVec::<[bool; 18]>::new();
					
					let count1 = _to_usize(GLOBAL_DATA, 170 + op_idx as usize * 8) % 12;
					let count2 = _to_usize(GLOBAL_DATA, 180 + op_idx as usize * 8) % 12;
					
					for i in 0..count1 {
						let val = _to_bool(GLOBAL_DATA, 190 + i + op_idx as usize * 8);
						sv1.push(val);
					}
					
					for i in 0..count2 {
						let val = _to_bool(GLOBAL_DATA, 200 + i + op_idx as usize * 8);
						sv2.push(val);
					}
					
					sv1.append(&mut sv2);
					let spilled = sv1.spilled();
					println!("Spilled: {}", spilled);
					sv1.shrink_to_fit();
					
					let boxed_slice = sv1.into_boxed_slice();
					for item in boxed_slice.iter() {
						println!("{:?}", *item);
					}
				},
				6 => {
					let buffer: [u64; 14] = [
						_to_u64(GLOBAL_DATA, 210 + op_idx as usize * 8),
						_to_u64(GLOBAL_DATA, 218 + op_idx as usize * 8),
						_to_u64(GLOBAL_DATA, 226 + op_idx as usize * 8),
						_to_u64(GLOBAL_DATA, 234 + op_idx as usize * 8),
						_to_u64(GLOBAL_DATA, 242 + op_idx as usize * 8),
						_to_u64(GLOBAL_DATA, 250 + op_idx as usize * 8),
						_to_u64(GLOBAL_DATA, 258 + op_idx as usize * 8),
						_to_u64(GLOBAL_DATA, 266 + op_idx as usize * 8),
						_to_u64(GLOBAL_DATA, 274 + op_idx as usize * 8),
						_to_u64(GLOBAL_DATA, 282 + op_idx as usize * 8),
						_to_u64(GLOBAL_DATA, 290 + op_idx as usize * 8),
						_to_u64(GLOBAL_DATA, 298 + op_idx as usize * 8),
						_to_u64(GLOBAL_DATA, 306 + op_idx as usize * 8),
						_to_u64(GLOBAL_DATA, 314 + op_idx as usize * 8),
					];
					let sv = smallvec::SmallVec::<[u64; 14]>::from_buf(buffer);
					let inline_size = sv.inline_size();
					println!("Inline size: {}", inline_size);
					sv.clone().clear();
					
					let boxed_slice = sv.into_boxed_slice();
					for item in boxed_slice.iter() {
						println!("{:?}", *item);
					}
				},
				7 => {
					let buffer: [i16; 22] = [
						_to_i16(GLOBAL_DATA, 320 + op_idx as usize * 2),
						_to_i16(GLOBAL_DATA, 322 + op_idx as usize * 2),
						_to_i16(GLOBAL_DATA, 324 + op_idx as usize * 2),
						_to_i16(GLOBAL_DATA, 326 + op_idx as usize * 2),
						_to_i16(GLOBAL_DATA, 328 + op_idx as usize * 2),
						_to_i16(GLOBAL_DATA, 330 + op_idx as usize * 2),
						_to_i16(GLOBAL_DATA, 332 + op_idx as usize * 2),
						_to_i16(GLOBAL_DATA, 334 + op_idx as usize * 2),
						_to_i16(GLOBAL_DATA, 336 + op_idx as usize * 2),
						_to_i16(GLOBAL_DATA, 338 + op_idx as usize * 2),
						_to_i16(GLOBAL_DATA, 340 + op_idx as usize * 2),
						_to_i16(GLOBAL_DATA, 342 + op_idx as usize * 2),
						_to_i16(GLOBAL_DATA, 344 + op_idx as usize * 2),
						_to_i16(GLOBAL_DATA, 346 + op_idx as usize * 2),
						_to_i16(GLOBAL_DATA, 348 + op_idx as usize * 2),
						_to_i16(GLOBAL_DATA, 350 + op_idx as usize * 2),
						_to_i16(GLOBAL_DATA, 352 + op_idx as usize * 2),
						_to_i16(GLOBAL_DATA, 354 + op_idx as usize * 2),
						_to_i16(GLOBAL_DATA, 356 + op_idx as usize * 2),
						_to_i16(GLOBAL_DATA, 358 + op_idx as usize * 2),
						_to_i16(GLOBAL_DATA, 360 + op_idx as usize * 2),
						_to_i16(GLOBAL_DATA, 362 + op_idx as usize * 2),
					];
					let len = _to_usize(GLOBAL_DATA, 380 + op_idx as usize * 8);
					let sv = smallvec::SmallVec::<[i16; 22]>::from_buf_and_len(buffer, len);
					let as_ptr = sv.as_ptr();
					println!("Pointer: {:?}", as_ptr);
					let cloned_sv = sv.clone();
					println!("Cloned capacity: {}", cloned_sv.capacity());
					
					let boxed_slice = sv.into_boxed_slice();
					for item in boxed_slice.iter() {
						println!("{:?}", *item);
					}
				},
				8 => {
					let mut sv = smallvec::SmallVec::<[isize; 15]>::new();
					let insert_count = _to_usize(GLOBAL_DATA, 400 + op_idx as usize * 8) % 8;
					for i in 0..insert_count {
						let idx = _to_usize(GLOBAL_DATA, 420 + i * 8 + op_idx as usize * 16);
						let val = _to_isize(GLOBAL_DATA, 450 + i * 8 + op_idx as usize * 16);
						if idx <= sv.len() {
							sv.insert(idx, val);
						}
					}
					
					let remove_count = _to_usize(GLOBAL_DATA, 480 + op_idx as usize * 8) % 5;
					for i in 0..remove_count {
						let idx = _to_usize(GLOBAL_DATA, 500 + i * 8 + op_idx as usize * 16);
						if idx < sv.len() {
							let removed = sv.remove(idx);
							println!("Removed: {}", removed);
						}
					}
					
					sv.reserve(_to_usize(GLOBAL_DATA, 600 + op_idx as usize * 8));
					
					let boxed_slice = sv.into_boxed_slice();
					for item in boxed_slice.iter() {
						println!("{:?}", *item);
					}
				},
				9 => {
					let mut sv = smallvec::SmallVec::<[f64; 12]>::new();
					let initial_count = _to_usize(GLOBAL_DATA, 520 + op_idx as usize * 8) % 12;
					for i in 0..initial_count {
						let val = _to_f64(GLOBAL_DATA, 540 + i * 8 + op_idx as usize * 16);
						sv.push(val);
					}
					
					let drain_start = _to_usize(GLOBAL_DATA, 600 + op_idx as usize * 8);
					let drain_end = _to_usize(GLOBAL_DATA, 610 + op_idx as usize * 8);
					if drain_start < sv.len() && drain_end > drain_start {
						let actual_end = std::cmp::min(drain_end, sv.len());
						let mut drain_iter = sv.drain(drain_start..actual_end);
						while let Some(item) = drain_iter.next() {
							println!("Drained: {}", item);
						}
					}
					
					if !sv.is_empty() {
						let pop_val = sv.pop();
						if let Some(val) = pop_val {
							println!("Popped: {}", val);
						}
					}
					
					let boxed_slice = sv.into_boxed_slice();
					for item in boxed_slice.iter() {
						println!("{:?}", *item);
					}
				},
				10 => {
					let mut sv = smallvec::SmallVec::<[usize; 15]>::new();
					let fill_count = _to_usize(GLOBAL_DATA, 650 + op_idx as usize * 8) % 20;
					for i in 0..fill_count {
						let val = _to_usize(GLOBAL_DATA, 670 + i * 8 + op_idx as usize * 16);
						sv.push(val);
					}
					
					let truncate_len = _to_usize(GLOBAL_DATA, 750 + op_idx as usize * 8);
					sv.truncate(truncate_len);
					
					let reserve_additional = _to_usize(GLOBAL_DATA, 760 + op_idx as usize * 8);
					sv.reserve(reserve_additional);
					
					let mut_slice = sv.as_mut_slice();
					println!("Mutable slice length: {}", mut_slice.len());
					
					sv.grow(_to_usize(GLOBAL_DATA, 770 + op_idx as usize * 8));
					
					let boxed_slice = sv.into_boxed_slice();
					for item in boxed_slice.iter() {
						println!("{:?}", *item);
					}
				},
				_ => {
					let mut sv1 = smallvec::SmallVec::<[u32; 16]>::new();
					let mut sv2 = smallvec::SmallVec::<[u32; 16]>::new();
					
					let count1 = _to_usize(GLOBAL_DATA, 800 + op_idx as usize * 8) % 10;
					let count2 = _to_usize(GLOBAL_DATA, 810 + op_idx as usize * 8) % 10;
					
					for i in 0..count1 {
						let val = _to_u32(GLOBAL_DATA, 820 + i * 4 + op_idx as usize * 8);
						sv1.push(val);
					}
					
					for i in 0..count2 {
						let val = _to_u32(GLOBAL_DATA, 860 + i * 4 + op_idx as usize * 8);
						sv2.push(val);
					}
					
					let eq_result = sv1.eq(&sv2);
					println!("Equal: {}", eq_result);
					
					let cmp_result = sv1.cmp(&sv2);
					println!("Comparison: {:?}", cmp_result);
					
					let partial_cmp_result = sv1.partial_cmp(&sv2);
					if let Some(ordering) = partial_cmp_result {
						println!("Partial comparison: {:?}", ordering);
					}
					
					let into_vec = sv1.clone().into_vec();
					println!("Vec length: {}", into_vec.len());
					
					let boxed_slice = sv1.into_boxed_slice();
					for item in boxed_slice.iter() {
						println!("{:?}", *item);
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