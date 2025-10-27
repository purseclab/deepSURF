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
		
		let num_operations = _to_u8(GLOBAL_DATA, 0) % 65;
		
		for op_idx in 0..num_operations {
			let base_offset = 10 + (op_idx as usize * 20);
			if base_offset + 20 > GLOBAL_DATA.len() { break; }
			
			let operation = _to_u8(GLOBAL_DATA, base_offset) % 15;
			
			match operation {
				0 => {
					let constructor_choice = _to_u8(GLOBAL_DATA, base_offset + 1) % 5;
					let mut sv = match constructor_choice {
						0 => SmallVec::<[i32; 16]>::new(),
						1 => {
							let capacity = _to_usize(GLOBAL_DATA, base_offset + 2);
							SmallVec::<[i32; 16]>::with_capacity(capacity)
						},
						2 => {
							let vec_size = _to_u8(GLOBAL_DATA, base_offset + 10) % 65;
							let mut vec = Vec::new();
							for i in 0..vec_size {
								vec.push(_to_i32(GLOBAL_DATA, base_offset + 11 + (i as usize * 4)));
							}
							SmallVec::<[i32; 16]>::from_vec(vec)
						},
						3 => {
							let slice_size = _to_u8(GLOBAL_DATA, base_offset + 10) % 65;
							let mut slice_data = Vec::new();
							for i in 0..slice_size {
								slice_data.push(_to_i32(GLOBAL_DATA, base_offset + 11 + (i as usize * 4)));
							}
							SmallVec::<[i32; 16]>::from_slice(&slice_data)
						},
						_ => {
							let elem = _to_i32(GLOBAL_DATA, base_offset + 2);
							let count = _to_usize(GLOBAL_DATA, base_offset + 6);
							SmallVec::<[i32; 16]>::from_elem(elem, count)
						}
					};
					
					let result = sv.into_vec();
					println!("{:?}", result.len());
					
					for item in &result {
						println!("{}", *item);
					}
				},
				1 => {
					let arr = [_to_i32(GLOBAL_DATA, base_offset + 1), _to_i32(GLOBAL_DATA, base_offset + 5), _to_i32(GLOBAL_DATA, base_offset + 9)];
					let mut sv = SmallVec::from_buf(arr);
					sv.push(_to_i32(GLOBAL_DATA, base_offset + 13));
					
					let vec_result = sv.into_vec();
					for elem in &vec_result {
						println!("{}", *elem);
					}
				},
				2 => {
					let arr_len = _to_u8(GLOBAL_DATA, base_offset + 1) % 65;
					let mut arr = [0i64; 32];
					for i in 0..(arr_len.min(32) as usize) {
						arr[i] = _to_i64(GLOBAL_DATA, base_offset + 2 + i * 8);
					}
					let len = _to_usize(GLOBAL_DATA, base_offset + 10);
					let sv = SmallVec::from_buf_and_len(arr, len);
					
					let converted = sv.into_vec();
					for val in &converted {
						println!("{}", *val);
					}
				},
				3 => {
					let mut sv1 = SmallVec::<[u8; 24]>::new();
					let push_count = _to_u8(GLOBAL_DATA, base_offset + 1) % 65;
					for i in 0..push_count {
						sv1.push(_to_u8(GLOBAL_DATA, base_offset + 2 + i as usize));
					}
					
					let capacity_val = sv1.capacity();
					println!("{}", capacity_val);
					
					let len_val = sv1.len();
					println!("{}", len_val);
					
					let is_empty_val = sv1.is_empty();
					println!("{}", is_empty_val);
					
					let as_slice_ref = sv1.as_slice();
					for byte in as_slice_ref {
						println!("{}", *byte);
					}
					
					let final_vec = sv1.into_vec();
					for item in &final_vec {
						println!("{}", *item);
					}
				},
				4 => {
					let mut sv = SmallVec::<[f32; 12]>::new();
					let extend_count = _to_u8(GLOBAL_DATA, base_offset + 1) % 65;
					for i in 0..extend_count {
						sv.push(_to_f32(GLOBAL_DATA, base_offset + 2 + i as usize * 4));
					}
					
					let reserve_amount = _to_usize(GLOBAL_DATA, base_offset + 10);
					sv.reserve(reserve_amount);
					
					let mut as_mut_slice_ref = sv.as_mut_slice();
					for elem in as_mut_slice_ref.iter_mut() {
						*elem += 1.0;
						println!("{}", *elem);
					}
					
					let output_vec = sv.into_vec();
					for val in &output_vec {
						println!("{}", *val);
					}
				},
				5 => {
					let mut sv = SmallVec::<[String; 15]>::new();
					let str_count = _to_u8(GLOBAL_DATA, base_offset + 1) % 65;
					for i in 0..str_count {
						let str_len = (_to_u8(GLOBAL_DATA, base_offset + 2 + i as usize) % 10) as usize;
						let start_idx = base_offset + 3 + i as usize * 10;
						let end_idx = (start_idx + str_len).min(GLOBAL_DATA.len());
						let str_val = _to_str(GLOBAL_DATA, start_idx, end_idx);
						sv.push(String::from(str_val));
					}
					
					sv.shrink_to_fit();
					
					let deref_slice = sv.deref();
					for s in deref_slice {
						println!("{}", s);
					}
					
					let converted_vec = sv.into_vec();
					for string_item in &converted_vec {
						println!("{}", string_item);
					}
				},
				6 => {
					let iter_count = _to_u8(GLOBAL_DATA, base_offset + 1) % 65;
					let mut iter_data = Vec::new();
					for i in 0..iter_count {
						iter_data.push(_to_u16(GLOBAL_DATA, base_offset + 2 + i as usize * 2));
					}
					let sv = SmallVec::<[u16; 20]>::from_iter(iter_data.into_iter());
					
					let slice_ref = sv.as_slice();
					for item in slice_ref {
						println!("{}", *item);
					}
					
					let result_vec = sv.into_vec();
					for element in &result_vec {
						println!("{}", *element);
					}
				},
				7 => {
					let mut sv1 = SmallVec::<[i16; 15]>::new();
					let mut sv2 = SmallVec::<[i16; 15]>::new();
					
					let fill_count1 = _to_u8(GLOBAL_DATA, base_offset + 1) % 65;
					for i in 0..fill_count1 {
						sv1.push(_to_i16(GLOBAL_DATA, base_offset + 2 + i as usize * 2));
					}
					
					let fill_count2 = _to_u8(GLOBAL_DATA, base_offset + 10) % 65;
					for i in 0..fill_count2 {
						sv2.push(_to_i16(GLOBAL_DATA, base_offset + 11 + i as usize * 2));
					}
					
					let eq_result = sv1.eq(&sv2);
					println!("{}", eq_result);
					
					let partial_cmp_result = sv1.partial_cmp(&sv2);
					println!("{:?}", partial_cmp_result);
					
					let cmp_result = sv1.cmp(&sv2);
					println!("{:?}", cmp_result);
					
					let sv1_vec = sv1.into_vec();
					for val in &sv1_vec {
						println!("{}", *val);
					}
				},
				8 => {
					let mut sv = SmallVec::<[char; 18]>::new();
					let char_count = _to_u8(GLOBAL_DATA, base_offset + 1) % 65;
					for i in 0..char_count {
						sv.push(_to_char(GLOBAL_DATA, base_offset + 2 + i as usize * 4));
					}
					
					let index_val = _to_usize(GLOBAL_DATA, base_offset + 10);
					if index_val < sv.len() {
						let indexed_ref = sv.index(index_val);
						println!("{}", *indexed_ref);
					}
					
					let as_ptr_val = sv.as_ptr();
					println!("{:p}", as_ptr_val);
					
					let final_result = sv.into_vec();
					for ch in &final_result {
						println!("{}", *ch);
					}
				},
				9 => {
					let mut sv = SmallVec::<[bool; 22]>::new();
					let bool_count = _to_u8(GLOBAL_DATA, base_offset + 1) % 65;
					for i in 0..bool_count {
						sv.push(_to_bool(GLOBAL_DATA, base_offset + 2 + i as usize));
					}
					
					let truncate_len = _to_usize(GLOBAL_DATA, base_offset + 10);
					sv.truncate(truncate_len);
					
					let remove_idx = _to_usize(GLOBAL_DATA, base_offset + 14);
					if remove_idx < sv.len() {
						let removed_item = sv.remove(remove_idx);
						println!("{}", removed_item);
					}
					
					let converted_output = sv.into_vec();
					for boolean in &converted_output {
						println!("{}", *boolean);
					}
				},
				10 => {
					let mut sv = SmallVec::<[u64; 16]>::new();
					let initial_count = _to_u8(GLOBAL_DATA, base_offset + 1) % 65;
					for i in 0..initial_count {
						sv.push(_to_u64(GLOBAL_DATA, base_offset + 2 + i as usize * 8));
					}
					
					let insert_idx = _to_usize(GLOBAL_DATA, base_offset + 10);
					let insert_val = _to_u64(GLOBAL_DATA, base_offset + 14);
					if insert_idx <= sv.len() {
						sv.insert(insert_idx, insert_val);
					}
					
					let swap_remove_idx = _to_usize(GLOBAL_DATA, base_offset + 18);
					if swap_remove_idx < sv.len() {
						let swapped = sv.swap_remove(swap_remove_idx);
						println!("{}", swapped);
					}
					
					let output_result = sv.into_vec();
					for num in &output_result {
						println!("{}", *num);
					}
				},
				11 => {
					let mut sv = SmallVec::<[i8; 25]>::new();
					let fill_amount = _to_u8(GLOBAL_DATA, base_offset + 1) % 65;
					for i in 0..fill_amount {
						sv.push(_to_i8(GLOBAL_DATA, base_offset + 2 + i as usize));
					}
					
					sv.clear();
					
					let new_fill = _to_u8(GLOBAL_DATA, base_offset + 10) % 65;
					for i in 0..new_fill {
						sv.push(_to_i8(GLOBAL_DATA, base_offset + 11 + i as usize));
					}
					
					let cloned_sv = sv.clone();
					let cloned_vec = cloned_sv.into_vec();
					for item in &cloned_vec {
						println!("{}", *item);
					}
					
					let original_vec = sv.into_vec();
					for val in &original_vec {
						println!("{}", *val);
					}
				},
				12 => {
					let mut sv = SmallVec::<[u32; 14]>::new();
					let resize_len = _to_usize(GLOBAL_DATA, base_offset + 1);
					let resize_val = _to_u32(GLOBAL_DATA, base_offset + 9);
					sv.resize(resize_len, resize_val);
					
					let drain_start = _to_usize(GLOBAL_DATA, base_offset + 13);
					let drain_end = _to_usize(GLOBAL_DATA, base_offset + 17);
					let mut drain_iter = sv.drain(drain_start..drain_end);
					
					for drained_item in drain_iter {
						println!("{}", drained_item);
					}
					
					let remaining_vec = sv.into_vec();
					for remaining in &remaining_vec {
						println!("{}", *remaining);
					}
				},
				13 => {
					let mut sv = SmallVec::<[usize; 13]>::new();
					let append_count = _to_u8(GLOBAL_DATA, base_offset + 1) % 65;
					for i in 0..append_count {
						sv.push(_to_usize(GLOBAL_DATA, base_offset + 2 + i as usize * 8));
					}
					
					let extend_slice_size = _to_u8(GLOBAL_DATA, base_offset + 10) % 65;
					let mut extend_data = Vec::new();
					for i in 0..extend_slice_size {
						extend_data.push(_to_usize(GLOBAL_DATA, base_offset + 11 + i as usize * 8));
					}
					sv.extend_from_slice(&extend_data);
					
					let popped_item = sv.pop();
					println!("{:?}", popped_item);
					
					let final_output = sv.into_vec();
					for elem in &final_output {
						println!("{}", *elem);
					}
				},
				_ => {
					let mut sv = SmallVec::<[isize; 19]>::new();
					let count = _to_u8(GLOBAL_DATA, base_offset + 1) % 65;
					for i in 0..count {
						sv.push(_to_isize(GLOBAL_DATA, base_offset + 2 + i as usize * 8));
					}
					
					let grow_cap = _to_usize(GLOBAL_DATA, base_offset + 10);
					sv.grow(grow_cap);
					
					let mut into_iter = sv.into_iter();
					let iter_slice = into_iter.as_slice();
					for item in iter_slice {
						println!("{}", *item);
					}
					
					let collected_vec: Vec<isize> = into_iter.collect();
					let final_vec_result = collected_vec;
					for val in &final_vec_result {
						println!("{}", *val);
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