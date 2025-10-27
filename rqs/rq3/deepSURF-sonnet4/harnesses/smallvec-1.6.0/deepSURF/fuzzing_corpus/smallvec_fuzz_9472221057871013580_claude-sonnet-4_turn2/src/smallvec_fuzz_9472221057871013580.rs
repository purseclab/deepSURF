#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

struct CustomType0(String);

fn main (){
	fuzz_nohook!(|data: &[u8]| {
		if data.len() < 200 {return;}
		set_global_data(data);
		let global_data = get_global_data();
		let GLOBAL_DATA = global_data.first_half;
		
		let num_operations = _to_u8(GLOBAL_DATA, 0) % 65;
		for i in 0..num_operations {
			let op_offset = (i as usize * 8) % GLOBAL_DATA.len().saturating_sub(8);
			let operation = _to_u8(GLOBAL_DATA, op_offset) % 10;
			
			match operation {
				0 => {
					let choice = _to_u8(GLOBAL_DATA, op_offset + 1) % 5;
					let mut vec = match choice {
						0 => {
							let cap = _to_usize(GLOBAL_DATA, op_offset + 2);
							smallvec::SmallVec::<[i32; 12]>::with_capacity(cap)
						},
						1 => {
							let slice_len = (_to_u8(GLOBAL_DATA, op_offset + 2) % 65) as usize;
							let slice_start = op_offset + 3;
							if slice_start + slice_len <= GLOBAL_DATA.len() {
								let slice_data: Vec<i32> = GLOBAL_DATA[slice_start..slice_start + slice_len]
									.iter().map(|&b| b as i32).collect();
								smallvec::SmallVec::<[i32; 12]>::from_vec(slice_data)
							} else {
								smallvec::SmallVec::<[i32; 12]>::new()
							}
						},
						2 => {
							let elem = _to_i32(GLOBAL_DATA, op_offset + 2);
							let count = _to_usize(GLOBAL_DATA, op_offset + 6);
							smallvec::SmallVec::<[i32; 12]>::from_elem(elem, count)
						},
						3 => {
							let slice_len = (_to_u8(GLOBAL_DATA, op_offset + 2) % 65) as usize;
							let slice_start = op_offset + 3;
							if slice_start + slice_len * 4 <= GLOBAL_DATA.len() {
								let slice_data: Vec<i32> = (0..slice_len)
									.map(|j| _to_i32(GLOBAL_DATA, slice_start + j * 4))
									.collect();
								smallvec::SmallVec::<[i32; 12]>::from_iter(slice_data)
							} else {
								smallvec::SmallVec::<[i32; 12]>::new()
							}
						},
						_ => smallvec::SmallVec::<[i32; 12]>::new(),
					};
					
					vec.reserve(_to_usize(GLOBAL_DATA, op_offset + 3));
					let ref_vec = &vec;
					let vec_slice = ref_vec.deref();
					println!("{:?}", vec_slice.len());
					
					let len = _to_usize(GLOBAL_DATA, op_offset + 7);
					vec.truncate(len);
					println!("{:?}", vec.len());
				},
				1 => {
					let mut vec = smallvec::SmallVec::<[u8; 16]>::new();
					let push_count = _to_u8(GLOBAL_DATA, op_offset + 1) % 32;
					for j in 0..push_count {
						let val = _to_u8(GLOBAL_DATA, (op_offset + 2 + j as usize) % GLOBAL_DATA.len());
						vec.push(val);
					}
					let capacity_before = vec.capacity();
					println!("{:?}", capacity_before);
					
					let len = _to_usize(GLOBAL_DATA, op_offset + 2);
					vec.truncate(len);
					
					let slice = vec.as_slice();
					println!("{:?}", slice.len());
				},
				2 => {
					let mut vec1 = smallvec::SmallVec::<[f64; 8]>::new();
					let mut vec2 = smallvec::SmallVec::<[f64; 8]>::new();
					
					let count1 = _to_u8(GLOBAL_DATA, op_offset + 1) % 16;
					for j in 0..count1 {
						let val = _to_f64(GLOBAL_DATA, (op_offset + 2 + j as usize * 8) % GLOBAL_DATA.len().saturating_sub(8));
						vec1.push(val);
					}
					
					let count2 = _to_u8(GLOBAL_DATA, op_offset + 3) % 16;
					for j in 0..count2 {
						let val = _to_f64(GLOBAL_DATA, (op_offset + 4 + j as usize * 8) % GLOBAL_DATA.len().saturating_sub(8));
						vec2.push(val);
					}
					
					let vec1_clone = vec1.clone();
					println!("{:?}", vec1_clone.spilled());
					
					let len1 = _to_usize(GLOBAL_DATA, op_offset + 5);
					vec1.truncate(len1);
					
					let len2 = _to_usize(GLOBAL_DATA, op_offset + 6);
					vec2.truncate(len2);
					
					vec1.append(&mut vec2);
					println!("{:?}", vec1.is_empty());
				},
				3 => {
					let mut vec = smallvec::SmallVec::<[bool; 20]>::new();
					let elem_count = _to_u8(GLOBAL_DATA, op_offset + 1) % 30;
					for j in 0..elem_count {
						let val = _to_bool(GLOBAL_DATA, (op_offset + 2 + j as usize) % GLOBAL_DATA.len());
						vec.push(val);
					}
					
					let index = _to_usize(GLOBAL_DATA, op_offset + 2);
					let new_elem = _to_bool(GLOBAL_DATA, op_offset + 3);
					vec.insert(index, new_elem);
					
					let vec_ref = &vec;
					let slice_ref = vec_ref.as_slice();
					println!("{:?}", slice_ref.len());
					
					let truncate_len = _to_usize(GLOBAL_DATA, op_offset + 4);
					vec.truncate(truncate_len);
					
					let removed_elem = vec.pop();
					println!("{:?}", removed_elem);
				},
				4 => {
					let mut vec = smallvec::SmallVec::<[char; 15]>::new();
					let char_count = _to_u8(GLOBAL_DATA, op_offset + 1) % 20;
					for j in 0..char_count {
						let val = _to_char(GLOBAL_DATA, (op_offset + 2 + j as usize * 4) % GLOBAL_DATA.len().saturating_sub(4));
						vec.push(val);
					}
					
					let reserve_amount = _to_usize(GLOBAL_DATA, op_offset + 3);
					vec.reserve(reserve_amount);
					
					let as_ref = vec.as_ref();
					println!("{:?}", as_ref.len());
					
					let len = _to_usize(GLOBAL_DATA, op_offset + 4);
					vec.truncate(len);
					
					vec.shrink_to_fit();
					println!("{:?}", vec.capacity());
				},
				5 => {
					let mut vec = smallvec::SmallVec::<[i64; 10]>::new();
					let init_count = _to_u8(GLOBAL_DATA, op_offset + 1) % 15;
					for j in 0..init_count {
						let val = _to_i64(GLOBAL_DATA, (op_offset + 2 + j as usize * 8) % GLOBAL_DATA.len().saturating_sub(8));
						vec.push(val);
					}
					
					let range_start = _to_usize(GLOBAL_DATA, op_offset + 2);
					let range_end = _to_usize(GLOBAL_DATA, op_offset + 3);
					let drain_iter = vec.drain(range_start..range_end);
					let collected: Vec<_> = drain_iter.collect();
					println!("{:?}", collected.len());
					
					let into_vec = vec.into_vec();
					let mut vec_back = smallvec::SmallVec::<[i64; 10]>::from_vec(into_vec);
					
					let len = _to_usize(GLOBAL_DATA, op_offset + 4);
					vec_back.truncate(len);
				},
				6 => {
					let mut vec = smallvec::SmallVec::<[u32; 18]>::new();
					let size = _to_u8(GLOBAL_DATA, op_offset + 1) % 25;
					for j in 0..size {
						let val = _to_u32(GLOBAL_DATA, (op_offset + 2 + j as usize * 4) % GLOBAL_DATA.len().saturating_sub(4));
						vec.push(val);
					}
					
					let index = _to_usize(GLOBAL_DATA, op_offset + 3);
					if index < vec.len() {
						let removed = vec.remove(index);
						println!("{:?}", removed);
					}
					
					let mut_slice = vec.as_mut_slice();
					println!("{:?}", mut_slice.len());
					
					let truncate_len = _to_usize(GLOBAL_DATA, op_offset + 4);
					vec.truncate(truncate_len);
					
					vec.clear();
					println!("{:?}", vec.len());
				},
				7 => {
					let mut vec = smallvec::SmallVec::<[usize; 14]>::new();
					let elements = _to_u8(GLOBAL_DATA, op_offset + 1) % 20;
					for j in 0..elements {
						let val = _to_usize(GLOBAL_DATA, (op_offset + 2 + j as usize * 8) % GLOBAL_DATA.len().saturating_sub(8));
						vec.push(val);
					}
					
					let resize_len = _to_usize(GLOBAL_DATA, op_offset + 2);
					let fill_val = _to_usize(GLOBAL_DATA, op_offset + 3);
					vec.resize(resize_len, fill_val);
					
					let final_len = _to_usize(GLOBAL_DATA, op_offset + 4);
					vec.truncate(final_len);
					
					let slice_ref = vec.as_slice();
					println!("{:?}", slice_ref.len());
				},
				8 => {
					let mut vec = smallvec::SmallVec::<[isize; 22]>::new();
					let count = _to_u8(GLOBAL_DATA, op_offset + 1) % 30;
					for j in 0..count {
						let val = _to_isize(GLOBAL_DATA, (op_offset + 2 + j as usize * 8) % GLOBAL_DATA.len().saturating_sub(8));
						vec.push(val);
					}
					
					let index = _to_usize(GLOBAL_DATA, op_offset + 2);
					if index < vec.len() {
						let swapped = vec.swap_remove(index);
						println!("{:?}", swapped);
					}
					
					let preserve_len = _to_usize(GLOBAL_DATA, op_offset + 3);
					vec.truncate(preserve_len);
					
					let ptr = vec.as_ptr();
					println!("{:?}", ptr);
				},
				_ => {
					let mut vec = smallvec::SmallVec::<[String; 6]>::new();
					let str_count = _to_u8(GLOBAL_DATA, op_offset + 1) % 8;
					for j in 0..str_count {
						let str_len = (_to_u8(GLOBAL_DATA, (op_offset + 2 + j as usize) % GLOBAL_DATA.len()) % 10) as usize;
						let str_start = (op_offset + 3 + j as usize * 10) % GLOBAL_DATA.len().saturating_sub(str_len);
						let str_slice = _to_str(GLOBAL_DATA, str_start, str_start + str_len);
						vec.push(String::from(str_slice));
					}
					
					let trunc_to = _to_usize(GLOBAL_DATA, op_offset + 2);
					vec.truncate(trunc_to);
					
					let into_iter = vec.into_iter();
					let collected_back: smallvec::SmallVec<[String; 6]> = into_iter.collect();
					
					let capacity = collected_back.capacity();
					println!("{:?}", capacity);
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