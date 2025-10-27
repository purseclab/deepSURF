#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};
use std::borrow::{Borrow, BorrowMut};

fn main (){
	fuzz_nohook!(|data: &[u8]| {
		if data.len() < 250 {return;}
		set_global_data(data);
		let global_data = get_global_data();
		let GLOBAL_DATA = global_data.first_half;
		
		let num_operations = _to_usize(GLOBAL_DATA, 0) % 65;
		let mut current_offset = 8;
		
		for _ in 0..num_operations {
			let operation_type = _to_u8(GLOBAL_DATA, current_offset) % 15;
			current_offset += 1;
			
			match operation_type {
				0 => {
					let capacity = _to_usize(GLOBAL_DATA, current_offset);
					current_offset += 8;
					let mut sv = smallvec::SmallVec::<[u32; 16]>::with_capacity(capacity);
					let reserve_amount = _to_usize(GLOBAL_DATA, current_offset);
					current_offset += 8;
					sv.reserve(reserve_amount);
					
					let push_count = _to_usize(GLOBAL_DATA, current_offset) % 65;
					current_offset += 8;
					for i in 0..push_count {
						let value = _to_u32(GLOBAL_DATA, current_offset + i * 4);
						sv.push(value);
					}
					current_offset += push_count * 4;
					
					let slice_ref = sv.as_slice();
					println!("{:?}", slice_ref);
					
					if !sv.is_empty() {
						let index = _to_usize(GLOBAL_DATA, current_offset) % sv.len();
						current_offset += 8;
						let element_ref = &sv[index];
						println!("{}", *element_ref);
					}
				},
				1 => {
					let slice_len = _to_usize(GLOBAL_DATA, current_offset) % 65;
					current_offset += 8;
					let mut vec = Vec::new();
					for i in 0..slice_len {
						let value = _to_u8(GLOBAL_DATA, current_offset + i);
						vec.push(value);
					}
					current_offset += slice_len;
					
					let mut sv = smallvec::SmallVec::<[u8; 32]>::from_vec(vec);
					let reserve_amount = _to_usize(GLOBAL_DATA, current_offset);
					current_offset += 8;
					sv.reserve(reserve_amount);
					
					let extend_slice_len = _to_usize(GLOBAL_DATA, current_offset) % 65;
					current_offset += 8;
					let mut extend_vec = Vec::new();
					for i in 0..extend_slice_len {
						let value = _to_u8(GLOBAL_DATA, current_offset + i);
						extend_vec.push(value);
					}
					current_offset += extend_slice_len;
					sv.extend_from_slice(&extend_vec);
					
					let mut_slice_ref = sv.as_mut_slice();
					for elem in mut_slice_ref.iter_mut() {
						println!("{}", *elem);
					}
				},
				2 => {
					let elem_count = _to_usize(GLOBAL_DATA, current_offset) % 65;
					current_offset += 8;
					let elem_value = _to_u64(GLOBAL_DATA, current_offset);
					current_offset += 8;
					
					let mut sv = smallvec::SmallVec::<[u64; 8]>::from_elem(elem_value, elem_count);
					let reserve_amount = _to_usize(GLOBAL_DATA, current_offset);
					current_offset += 8;
					sv.reserve(reserve_amount);
					
					let len = sv.len();
					println!("Length: {}", len);
					let capacity = sv.capacity();
					println!("Capacity: {}", capacity);
					
					sv.truncate(_to_usize(GLOBAL_DATA, current_offset) % (len + 1));
					current_offset += 8;
				},
				3 => {
					let vec_size = _to_usize(GLOBAL_DATA, current_offset) % 65;
					current_offset += 8;
					let mut vec = Vec::new();
					for i in 0..vec_size {
						let value = _to_i32(GLOBAL_DATA, current_offset + i * 4);
						vec.push(value);
					}
					current_offset += vec_size * 4;
					
					let mut sv = smallvec::SmallVec::<[i32; 12]>::from_vec(vec);
					let reserve_amount = _to_usize(GLOBAL_DATA, current_offset);
					current_offset += 8;
					sv.reserve(reserve_amount);
					
					if !sv.is_empty() {
						let pop_result = sv.pop();
						if let Some(val) = pop_result {
							println!("Popped: {}", val);
						}
					}
					
					let insert_index = _to_usize(GLOBAL_DATA, current_offset) % (sv.len() + 1);
					current_offset += 8;
					let insert_value = _to_i32(GLOBAL_DATA, current_offset);
					current_offset += 4;
					sv.insert(insert_index, insert_value);
				},
				4 => {
					let slice_len = _to_usize(GLOBAL_DATA, current_offset) % 65;
					current_offset += 8;
					let mut vec = Vec::new();
					for i in 0..slice_len {
						let value = _to_f32(GLOBAL_DATA, current_offset + i * 4);
						vec.push(value);
					}
					current_offset += slice_len * 4;
					
					let mut sv = smallvec::SmallVec::<[f32; 20]>::from_slice(&vec);
					let reserve_amount = _to_usize(GLOBAL_DATA, current_offset);
					current_offset += 8;
					sv.reserve(reserve_amount);
					
					let drain_start = _to_usize(GLOBAL_DATA, current_offset) % (sv.len() + 1);
					current_offset += 8;
					let drain_end = drain_start + (_to_usize(GLOBAL_DATA, current_offset) % (sv.len() - drain_start + 1));
					current_offset += 8;
					
					let drain_iter = sv.drain(drain_start..drain_end);
					for val in drain_iter {
						println!("Drained: {}", val);
					}
				},
				5 => {
					let constructor_choice = _to_u8(GLOBAL_DATA, current_offset) % 3;
					current_offset += 1;
					
					let mut sv = match constructor_choice {
						0 => smallvec::SmallVec::<[bool; 24]>::new(),
						1 => {
							let cap = _to_usize(GLOBAL_DATA, current_offset);
							current_offset += 8;
							smallvec::SmallVec::<[bool; 24]>::with_capacity(cap)
						},
						_ => {
							let elem_value = _to_bool(GLOBAL_DATA, current_offset);
							current_offset += 1;
							let count = _to_usize(GLOBAL_DATA, current_offset) % 65;
							current_offset += 8;
							smallvec::SmallVec::<[bool; 24]>::from_elem(elem_value, count)
						}
					};
					
					let reserve_amount = _to_usize(GLOBAL_DATA, current_offset);
					current_offset += 8;
					sv.reserve(reserve_amount);
					
					let push_count = _to_usize(GLOBAL_DATA, current_offset) % 65;
					current_offset += 8;
					for i in 0..push_count {
						let value = _to_bool(GLOBAL_DATA, current_offset + i);
						sv.push(value);
					}
					current_offset += push_count;
					
					sv.shrink_to_fit();
					let ptr = sv.as_ptr();
					println!("Ptr: {:?}", ptr);
				},
				6 => {
					let mut sv1 = smallvec::SmallVec::<[u16; 14]>::new();
					let mut sv2 = smallvec::SmallVec::<[u16; 14]>::new();
					
					let fill_count1 = _to_usize(GLOBAL_DATA, current_offset) % 65;
					current_offset += 8;
					for i in 0..fill_count1 {
						let value = _to_u16(GLOBAL_DATA, current_offset + i * 2);
						sv1.push(value);
					}
					current_offset += fill_count1 * 2;
					
					let fill_count2 = _to_usize(GLOBAL_DATA, current_offset) % 65;
					current_offset += 8;
					for i in 0..fill_count2 {
						let value = _to_u16(GLOBAL_DATA, current_offset + i * 2);
						sv2.push(value);
					}
					current_offset += fill_count2 * 2;
					
					let reserve_amount = _to_usize(GLOBAL_DATA, current_offset);
					current_offset += 8;
					sv1.reserve(reserve_amount);
					
					sv1.append(&mut sv2);
					let eq_result = sv1.eq(&sv2);
					println!("Equal: {}", eq_result);
					
					let clone_sv = sv1.clone();
					let cmp_result = sv1.cmp(&clone_sv);
					println!("{:?}", cmp_result);
				},
				7 => {
					let mut sv = smallvec::SmallVec::<[char; 15]>::new();
					let fill_count = _to_usize(GLOBAL_DATA, current_offset) % 65;
					current_offset += 8;
					
					for i in 0..fill_count {
						let char_val = _to_char(GLOBAL_DATA, current_offset + i * 4);
						sv.push(char_val);
					}
					current_offset += fill_count * 4;
					
					let reserve_amount = _to_usize(GLOBAL_DATA, current_offset);
					current_offset += 8;
					sv.reserve(reserve_amount);
					
					if !sv.is_empty() {
						let remove_index = _to_usize(GLOBAL_DATA, current_offset) % sv.len();
						current_offset += 8;
						let removed = sv.remove(remove_index);
						println!("Removed: {}", removed);
					}
					
					sv.clear();
					println!("Is empty: {}", sv.is_empty());
				},
				8 => {
					let mut initial_vec = Vec::new();
					let vec_size = _to_usize(GLOBAL_DATA, current_offset) % 65;
					current_offset += 8;
					
					for i in 0..vec_size {
						let value = _to_i64(GLOBAL_DATA, current_offset + i * 8);
						initial_vec.push(value);
					}
					current_offset += vec_size * 8;
					
					let mut sv = smallvec::SmallVec::<[i64; 16]>::from_iter(initial_vec);
					let reserve_amount = _to_usize(GLOBAL_DATA, current_offset);
					current_offset += 8;
					sv.reserve(reserve_amount);
					
					let resize_len = _to_usize(GLOBAL_DATA, current_offset) % 65;
					current_offset += 8;
					let resize_val = _to_i64(GLOBAL_DATA, current_offset);
					current_offset += 8;
					sv.resize(resize_len, resize_val);
					
					let into_vec = sv.into_vec();
					println!("Vec len: {}", into_vec.len());
				},
				9 => {
					let slice_len = _to_usize(GLOBAL_DATA, current_offset) % 65;
					current_offset += 8;
					let mut vec = Vec::new();
					for i in 0..slice_len {
						let value = _to_u8(GLOBAL_DATA, current_offset + i);
						vec.push(value);
					}
					current_offset += slice_len;
					
					let mut sv = smallvec::SmallVec::<[u8; 22]>::from_slice(&vec);
					let reserve_amount = _to_usize(GLOBAL_DATA, current_offset);
					current_offset += 8;
					sv.reserve(reserve_amount);
					
					let to_smallvec: smallvec::SmallVec<[u8; 22]> = vec.as_slice().to_smallvec();
					println!("ToSmallVec len: {}", to_smallvec.len());
					
					if !sv.is_empty() {
						let swap_index = _to_usize(GLOBAL_DATA, current_offset) % sv.len();
						current_offset += 8;
						let swapped = sv.swap_remove(swap_index);
						println!("Swapped: {}", swapped);
					}
				},
				10 => {
					let mut sv = smallvec::SmallVec::<[f64; 18]>::new();
					let push_count = _to_usize(GLOBAL_DATA, current_offset) % 65;
					current_offset += 8;
					
					for i in 0..push_count {
						let value = _to_f64(GLOBAL_DATA, current_offset + i * 8);
						sv.push(value);
					}
					current_offset += push_count * 8;
					
					let reserve_amount = _to_usize(GLOBAL_DATA, current_offset);
					current_offset += 8;
					sv.reserve(reserve_amount);
					
					sv.dedup();
					let ptr_ref = sv.as_ptr();
					println!("Ptr after dedup: {:?}", ptr_ref);
					
					let mut_ptr = sv.as_mut_ptr();
					println!("Mut ptr: {:?}", mut_ptr);
				},
				11 => {
					let initial_size = _to_usize(GLOBAL_DATA, current_offset) % 65;
					current_offset += 8;
					let mut sv = smallvec::SmallVec::<[isize; 15]>::with_capacity(initial_size);
					
					for i in 0..initial_size {
						let value = _to_isize(GLOBAL_DATA, current_offset + i * 8);
						sv.push(value);
					}
					current_offset += initial_size * 8;
					
					let additional = _to_usize(GLOBAL_DATA, current_offset);
					current_offset += 8;
					sv.reserve(additional);
					
					let try_reserve_result = sv.try_reserve(_to_usize(GLOBAL_DATA, current_offset));
					current_offset += 8;
					match try_reserve_result {
						Ok(_) => println!("Try reserve succeeded"),
						Err(_) => println!("Try reserve failed"),
					}
					
					sv.reserve_exact(_to_usize(GLOBAL_DATA, current_offset));
					current_offset += 8;
					
					let final_ref = sv.deref();
					println!("Deref len: {}", final_ref.len());
				},
				12 => {
					let mut sv = smallvec::SmallVec::<[u128; 16]>::new();
					let push_count = _to_usize(GLOBAL_DATA, current_offset) % 65;
					current_offset += 8;
					
					for i in 0..push_count {
						let value = _to_u128(GLOBAL_DATA, current_offset + i * 16);
						sv.push(value);
					}
					current_offset += push_count * 16;
					
					let reserve_amount = _to_usize(GLOBAL_DATA, current_offset);
					current_offset += 8;
					sv.reserve(reserve_amount);
					
					let insert_index = _to_usize(GLOBAL_DATA, current_offset) % (sv.len() + 1);
					current_offset += 8;
					let insert_count = _to_usize(GLOBAL_DATA, current_offset) % 65;
					current_offset += 8;
					
					let mut insert_values = Vec::new();
					for i in 0..insert_count {
						let value = _to_u128(GLOBAL_DATA, current_offset + i * 16);
						insert_values.push(value);
					}
					current_offset += insert_count * 16;
					sv.insert_many(insert_index, insert_values);
					
					let as_ref_result = sv.as_ref();
					println!("AsRef len: {}", as_ref_result.len());
				},
				13 => {
					let slice_len = _to_usize(GLOBAL_DATA, current_offset) % 65;
					current_offset += 8;
					let mut vec = Vec::new();
					for i in 0..slice_len {
						let value = _to_i16(GLOBAL_DATA, current_offset + i * 2);
						vec.push(value);
					}
					current_offset += slice_len * 2;
					
					let mut sv = smallvec::SmallVec::<[i16; 30]>::from_slice(&vec);
					let reserve_amount = _to_usize(GLOBAL_DATA, current_offset);
					current_offset += 8;
					sv.reserve(reserve_amount);
					
					let insert_index = _to_usize(GLOBAL_DATA, current_offset) % (sv.len() + 1);
					current_offset += 8;
					let insert_slice_len = _to_usize(GLOBAL_DATA, current_offset) % 65;
					current_offset += 8;
					
					let mut insert_vec = Vec::new();
					for i in 0..insert_slice_len {
						let value = _to_i16(GLOBAL_DATA, current_offset + i * 2);
						insert_vec.push(value);
					}
					current_offset += insert_slice_len * 2;
					sv.insert_from_slice(insert_index, &insert_vec);
					
					let borrow_result: &[i16] = sv.borrow();
					println!("Borrow len: {}", borrow_result.len());
					
					let mut_borrow_result: &mut [i16] = sv.borrow_mut();
					println!("MutBorrow len: {}", mut_borrow_result.len());
				},
				_ => {
					let mut sv = smallvec::SmallVec::<[i8; 64]>::new();
					let push_count = _to_usize(GLOBAL_DATA, current_offset) % 65;
					current_offset += 8;
					
					for i in 0..push_count {
						let value = _to_i8(GLOBAL_DATA, current_offset + i);
						sv.push(value);
					}
					current_offset += push_count;
					
					let reserve_amount = _to_usize(GLOBAL_DATA, current_offset);
					current_offset += 8;
					sv.reserve(reserve_amount);
					
					let into_iter = sv.into_iter();
					for val in into_iter {
						println!("Iter val: {}", val);
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