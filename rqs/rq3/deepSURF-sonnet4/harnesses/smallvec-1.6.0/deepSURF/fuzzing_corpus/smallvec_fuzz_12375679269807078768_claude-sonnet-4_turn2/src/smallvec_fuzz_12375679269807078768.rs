#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

struct CustomType0(String);
struct CustomType1(String);

fn main (){
	fuzz_nohook!(|data: &[u8]| {
		if data.len() < 200 {return;}
		set_global_data(data);
		let global_data = get_global_data();
		let GLOBAL_DATA = global_data.first_half;
		
		let num_operations = _to_u8(GLOBAL_DATA, 0) % 10 + 1;
		
		for i in 0..num_operations {
			let base_offset = (10 + i * 18) as usize;
			if base_offset + 18 > GLOBAL_DATA.len() { break; }
			
			let operation = _to_u8(GLOBAL_DATA, base_offset) % 6;
			
			match operation {
				0 => {
					let constructor_choice = _to_u8(GLOBAL_DATA, base_offset + 1) % 4;
					let mut sv = match constructor_choice {
						0 => smallvec::SmallVec::<[i32; 16]>::new(),
						1 => {
							let cap = _to_usize(GLOBAL_DATA, base_offset + 2);
							smallvec::SmallVec::<[i32; 16]>::with_capacity(cap)
						},
						2 => {
							let elem = _to_i32(GLOBAL_DATA, base_offset + 2);
							let count = _to_usize(GLOBAL_DATA, base_offset + 6);
							smallvec::SmallVec::<[i32; 16]>::from_elem(elem, count)
						},
						_ => {
							let vec_size = _to_u8(GLOBAL_DATA, base_offset + 2) % 65;
							let mut vec = Vec::new();
							for j in 0..vec_size {
								let idx = base_offset + 3 + ((j as usize % 4) * 4);
								if idx + 4 <= GLOBAL_DATA.len() {
									vec.push(_to_i32(GLOBAL_DATA, idx));
								}
							}
							smallvec::SmallVec::<[i32; 16]>::from_vec(vec)
						}
					};
					
					let push_count = _to_u8(GLOBAL_DATA, base_offset + 10) % 20;
					for j in 0..push_count {
						let idx = base_offset + 11 + ((j as usize % 4) * 4);
						if idx + 4 <= GLOBAL_DATA.len() {
							let val = _to_i32(GLOBAL_DATA, idx);
							sv.push(val);
						}
					}
					
					sv.shrink_to_fit();
					
					let len_val = sv.len();
					println!("Length after shrink: {}", len_val);
					
					let cap_val = sv.capacity();
					println!("Capacity after shrink: {}", cap_val);
					
					if !sv.is_empty() {
						let slice_ref = sv.as_slice();
						println!("First element: {}", slice_ref[0]);
						
						let deref_slice = sv.deref();
						println!("Deref first: {}", deref_slice[0]);
					}
				},
				1 => {
					let slice_size = _to_u8(GLOBAL_DATA, base_offset + 1) % 65;
					let mut slice_data = Vec::new();
					for j in 0..slice_size {
						let idx = base_offset + 2 + ((j as usize % 4) * 4);
						if idx + 4 <= GLOBAL_DATA.len() {
							slice_data.push(_to_i32(GLOBAL_DATA, idx));
						}
					}
					
					let mut sv = smallvec::SmallVec::<[i32; 16]>::from_slice(&slice_data);
					
					let reserve_amount = _to_usize(GLOBAL_DATA, base_offset + 10);
					sv.reserve(reserve_amount);
					
					sv.shrink_to_fit();
					
					let comparison_sv = smallvec::SmallVec::<[i32; 16]>::from_slice(&slice_data);
					let cmp_result = sv.cmp(&comparison_sv);
					println!("Comparison result: {:?}", cmp_result);
					
					let eq_result = sv.eq(&comparison_sv);
					println!("Equality result: {}", eq_result);
				},
				2 => {
					let mut sv = smallvec::SmallVec::<[f64; 12]>::new();
					
					let extend_count = _to_u8(GLOBAL_DATA, base_offset + 1) % 30;
					for j in 0..extend_count {
						let idx = base_offset + 2 + ((j as usize % 2) * 8);
						if idx + 8 <= GLOBAL_DATA.len() {
							let val = _to_f64(GLOBAL_DATA, idx);
							sv.push(val);
						}
					}
					
					let clone_sv = sv.clone();
					sv.append(&mut clone_sv.clone());
					
					sv.shrink_to_fit();
					
					let truncate_len = _to_usize(GLOBAL_DATA, base_offset + 10);
					sv.truncate(truncate_len);
					
					let iter = sv.into_iter();
					let as_slice = iter.as_slice();
					println!("Iterator slice len: {}", as_slice.len());
				},
				3 => {
					let mut sv = smallvec::SmallVec::<[u8; 32]>::new();
					
					let initial_capacity = _to_usize(GLOBAL_DATA, base_offset + 1);
					sv.reserve_exact(initial_capacity);
					
					let insert_count = _to_u8(GLOBAL_DATA, base_offset + 9) % 50;
					for j in 0..insert_count {
						let idx = base_offset + 10 + j as usize;
						if idx < GLOBAL_DATA.len() {
							let val = _to_u8(GLOBAL_DATA, idx);
							sv.push(val);
						}
					}
					
					sv.shrink_to_fit();
					
					let drain_start = _to_usize(GLOBAL_DATA, base_offset + 15);
					let drain_end = _to_usize(GLOBAL_DATA, base_offset + 16);
					if sv.len() > 0 {
						let actual_start = drain_start % sv.len();
						let actual_end = if drain_end % sv.len() >= actual_start { 
							drain_end % sv.len() 
						} else { 
							actual_start 
						};
						let mut drain_iter = sv.drain(actual_start..actual_end);
						while let Some(item) = drain_iter.next() {
							println!("Drained: {}", item);
						}
					}
					
					sv.clear();
					sv.shrink_to_fit();
				},
				4 => {
					let array_data: [bool; 20] = [
						_to_bool(GLOBAL_DATA, base_offset + 1),
						_to_bool(GLOBAL_DATA, base_offset + 2),
						_to_bool(GLOBAL_DATA, base_offset + 3),
						_to_bool(GLOBAL_DATA, base_offset + 4),
						_to_bool(GLOBAL_DATA, base_offset + 5),
						_to_bool(GLOBAL_DATA, base_offset + 6),
						_to_bool(GLOBAL_DATA, base_offset + 7),
						_to_bool(GLOBAL_DATA, base_offset + 8),
						_to_bool(GLOBAL_DATA, base_offset + 9),
						_to_bool(GLOBAL_DATA, base_offset + 10),
						_to_bool(GLOBAL_DATA, base_offset + 11),
						_to_bool(GLOBAL_DATA, base_offset + 12),
						_to_bool(GLOBAL_DATA, base_offset + 13),
						_to_bool(GLOBAL_DATA, base_offset + 14),
						_to_bool(GLOBAL_DATA, base_offset + 15),
						_to_bool(GLOBAL_DATA, base_offset + 16),
						_to_bool(GLOBAL_DATA, base_offset + 17),
						false, false, false
					];
					
					let mut sv = smallvec::SmallVec::<[bool; 20]>::from_buf(array_data);
					let len = _to_usize(GLOBAL_DATA, base_offset + 1);
					sv = smallvec::SmallVec::<[bool; 20]>::from_buf_and_len(array_data, len);
					
					sv.retain(|x| *x);
					sv.shrink_to_fit();
					
					let mut slice_ref = sv.as_mut_slice();
					if slice_ref.len() > 0 {
						println!("Deref mut first: {}", slice_ref[0]);
					}
				},
				_ => {
					let iter_size = _to_u8(GLOBAL_DATA, base_offset + 1) % 65;
					let mut iter_data = Vec::new();
					for j in 0..iter_size {
						let idx = base_offset + 2 + ((j as usize % 4) * 4);
						if idx + 4 <= GLOBAL_DATA.len() {
							iter_data.push(_to_i32(GLOBAL_DATA, idx));
						}
					}
					
					let mut sv = smallvec::SmallVec::<[i32; 16]>::from_iter(iter_data.iter().copied());
					
					let resize_len = _to_usize(GLOBAL_DATA, base_offset + 10);
					let resize_val = _to_i32(GLOBAL_DATA, base_offset + 11);
					sv.resize(resize_len, resize_val);
					
					sv.shrink_to_fit();
					
					sv.dedup();
					
					if sv.len() > 0 {
						let index = _to_usize(GLOBAL_DATA, base_offset + 15) % sv.len();
						let indexed_ref = sv.index(index);
						println!("Indexed value: {}", *indexed_ref);
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