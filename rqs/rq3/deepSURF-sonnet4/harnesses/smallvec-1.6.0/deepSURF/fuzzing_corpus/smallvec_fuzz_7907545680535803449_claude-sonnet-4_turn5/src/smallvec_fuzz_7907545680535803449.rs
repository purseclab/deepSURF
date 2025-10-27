#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

#[derive(Debug, Clone, PartialEq)]
struct CustomType4(String);
#[derive(Debug, Clone, PartialEq)]
struct CustomType2(String);
#[derive(Debug, Clone, PartialEq)]
struct CustomType3(String);
#[derive(Debug, Clone, PartialEq)]
struct CustomType0(String);
#[derive(Debug, Clone, PartialEq)]
struct CustomType1(String);

fn main (){
	fuzz_nohook!(|data: &[u8]| {
		if data.len() < 4000 {return;}
		set_global_data(data);
		let global_data = get_global_data();
		let GLOBAL_DATA = global_data.first_half;
		
		let operations_count = _to_u8(GLOBAL_DATA, 0) % 30 + 1;
		
		for i in 0..operations_count {
			let base_idx = (i as usize) * 120 + 1;
			let operation_type = _to_u8(GLOBAL_DATA, base_idx) % 15;
			
			match operation_type {
				0 => {
					let constructor_choice = _to_u8(GLOBAL_DATA, base_idx + 1) % 5;
					let mut sv1 = match constructor_choice {
						0 => SmallVec::<[u32; 32]>::new(),
						1 => SmallVec::<[u32; 32]>::with_capacity(_to_usize(GLOBAL_DATA, base_idx + 2)),
						2 => {
							let vec_data = vec![_to_u32(GLOBAL_DATA, base_idx + 10); _to_usize(GLOBAL_DATA, base_idx + 18) % 64];
							SmallVec::<[u32; 32]>::from_vec(vec_data)
						},
						3 => {
							let arr_data = [_to_u32(GLOBAL_DATA, base_idx + 26); 32];
							SmallVec::<[u32; 32]>::from(arr_data)
						},
						_ => SmallVec::<[u32; 32]>::from_elem(_to_u32(GLOBAL_DATA, base_idx + 34), _to_usize(GLOBAL_DATA, base_idx + 42) % 20)
					};
					
					let elem_count = _to_u8(GLOBAL_DATA, base_idx + 50) % 25;
					for j in 0..elem_count {
						sv1.push(_to_u32(GLOBAL_DATA, base_idx + 51 + j as usize * 4));
					}
					
					let range_start = _to_usize(GLOBAL_DATA, base_idx + 76);
					let range_end = _to_usize(GLOBAL_DATA, base_idx + 84);
					let mut drain_result = sv1.drain(range_start..range_end);
					
					if let Some(first_drained) = drain_result.next() {
						println!("{:?}", first_drained);
					}
					let remaining_drained: Vec<_> = drain_result.collect();
					println!("{:?}", remaining_drained.len());
					
					sv1.extend([_to_u32(GLOBAL_DATA, base_idx + 92)]);
					println!("{:?}", sv1.capacity());
				},
				1 => {
					let constructor_choice = _to_u8(GLOBAL_DATA, base_idx + 1) % 4;
					let mut sv2 = match constructor_choice {
						0 => SmallVec::<[i64; 16]>::new(),
						1 => SmallVec::<[i64; 16]>::with_capacity(_to_usize(GLOBAL_DATA, base_idx + 2)),
						2 => {
							let vec_source = vec![_to_i64(GLOBAL_DATA, base_idx + 10); _to_usize(GLOBAL_DATA, base_idx + 18) % 32];
							SmallVec::<[i64; 16]>::from_vec(vec_source)
						},
						_ => SmallVec::<[i64; 16]>::from_elem(_to_i64(GLOBAL_DATA, base_idx + 26), _to_usize(GLOBAL_DATA, base_idx + 34) % 15)
					};
					
					sv2.reserve(_to_usize(GLOBAL_DATA, base_idx + 42));
					sv2.push(_to_i64(GLOBAL_DATA, base_idx + 50));
					
					let partial_drain = sv2.drain(_to_usize(GLOBAL_DATA, base_idx + 58).._to_usize(GLOBAL_DATA, base_idx + 66));
					let collected_drain: SmallVec<[i64; 8]> = partial_drain.collect();
					println!("{:?}", collected_drain.len());
					
					sv2.resize_with(_to_usize(GLOBAL_DATA, base_idx + 74) % 20, || _to_i64(GLOBAL_DATA, base_idx + 82));
					let sv2_clone = sv2.clone();
					println!("{:?}", sv2_clone.len());
				},
				2 => {
					let vec_data = vec![_to_f32(GLOBAL_DATA, base_idx + 1); _to_usize(GLOBAL_DATA, base_idx + 5) % 64];
					let mut sv4 = SmallVec::<[f32; 8]>::from_vec(vec_data);
					
					let resize_len = _to_usize(GLOBAL_DATA, base_idx + 13) % 30;
					sv4.resize(resize_len, _to_f32(GLOBAL_DATA, base_idx + 21));
					
					let start_idx = _to_usize(GLOBAL_DATA, base_idx + 25);
					let drain_from_start = sv4.drain(start_idx..);
					for item in drain_from_start {
						println!("{:?}", item);
					}
					
					sv4.shrink_to_fit();
					println!("{:?}", sv4.is_empty());
				},
				3 => {
					let arr_slice = [_to_u8(GLOBAL_DATA, base_idx + 1); 20];
					let mut sv5 = SmallVec::<[u8; 64]>::from_slice(&arr_slice);
					
					let additional_capacity = _to_usize(GLOBAL_DATA, base_idx + 2);
					sv5.reserve(additional_capacity);
					
					let end_idx = _to_usize(GLOBAL_DATA, base_idx + 10);
					let partial_drain = sv5.drain(..end_idx);
					let collected_partial: Vec<_> = partial_drain.collect();
					println!("{:?}", collected_partial.len());
					
					let from_slice_data = &[_to_u8(GLOBAL_DATA, base_idx + 18); 14];
					sv5.extend_from_slice(from_slice_data);
					println!("{:?}", sv5.capacity());
				},
				4 => {
					let iter_data = vec![_to_i32(GLOBAL_DATA, base_idx + 1), _to_i32(GLOBAL_DATA, base_idx + 5)];
					let mut sv6 = SmallVec::<[i32; 4]>::from_iter(iter_data.into_iter());
					
					let insert_idx = _to_usize(GLOBAL_DATA, base_idx + 9);
					let insert_elem = _to_i32(GLOBAL_DATA, base_idx + 17);
					sv6.insert(insert_idx, insert_elem);
					
					let remove_idx = _to_usize(GLOBAL_DATA, base_idx + 21);
					if !sv6.is_empty() {
						let removed = sv6.remove(remove_idx % sv6.len());
						println!("{:?}", removed);
					}
					
					let drain_range = _to_usize(GLOBAL_DATA, base_idx + 29).._to_usize(GLOBAL_DATA, base_idx + 37);
					let drain_result: Vec<_> = sv6.drain(drain_range).collect();
					if !drain_result.is_empty() {
						println!("{:?}", drain_result[0]);
					}
					
					sv6.dedup();
					println!("{:?}", sv6.len());
				},
				5 => {
					let arr_slice = [_to_u16(GLOBAL_DATA, base_idx + 1); 15];
					let mut sv7 = SmallVec::<[u16; 2]>::from_slice(&arr_slice);
					let capacity = sv7.capacity();
					sv7.shrink_to_fit();
					
					let len_before = sv7.len();
					let trunc_len = _to_usize(GLOBAL_DATA, base_idx + 3);
					sv7.truncate(trunc_len);
					
					let drain_all_mut = sv7.drain(..);
					let mut drain_vec: Vec<_> = drain_all_mut.collect();
					drain_vec.sort();
					println!("{:?}", drain_vec.len());
					
					sv7.clear();
					println!("{:?}", sv7.is_empty());
				},
				6 => {
					let mut sv8 = SmallVec::<[i8; 128]>::new();
					sv8.push(_to_i8(GLOBAL_DATA, base_idx + 1));
					sv8.push(_to_i8(GLOBAL_DATA, base_idx + 2));
					
					let pop_result = sv8.pop();
					println!("{:?}", pop_result);
					
					sv8.extend([_to_i8(GLOBAL_DATA, base_idx + 3), _to_i8(GLOBAL_DATA, base_idx + 4)]);
					
					let start_bound = _to_usize(GLOBAL_DATA, base_idx + 5);
					let end_bound = _to_usize(GLOBAL_DATA, base_idx + 13);
					let bounded_drain = sv8.drain(start_bound..=end_bound);
					let count_drained = bounded_drain.count();
					println!("{:?}", count_drained);
					
					let as_slice = sv8.as_slice();
					println!("{:?}", as_slice.len());
				},
				7 => {
					let mut sv9 = SmallVec::<[f64; 256]>::with_capacity(_to_usize(GLOBAL_DATA, base_idx + 1) % 64);
					let fill_count = _to_usize(GLOBAL_DATA, base_idx + 9) % 32;
					for idx in 0..fill_count {
						sv9.push(_to_f64(GLOBAL_DATA, base_idx + 17 + idx * 8));
					}
					
					let as_mut_slice = sv9.as_mut_slice();
					println!("{:?}", as_mut_slice.len());
					
					let threshold = _to_f64(GLOBAL_DATA, base_idx + 50);
					sv9.retain(|elem| *elem > threshold);
					
					let full_drain = sv9.drain(..);
					let collected: SmallVec<[f64; 16]> = full_drain.collect();
					println!("{:?}", collected.len());
					
					let as_ptr = sv9.as_ptr();
					println!("{:?}", as_ptr as usize);
				},
				8 => {
					let elem = _to_u64(GLOBAL_DATA, base_idx + 1);
					let repeat_count = _to_usize(GLOBAL_DATA, base_idx + 9) % 20;
					let mut sv10 = SmallVec::<[u64; 512]>::from_elem(elem, repeat_count);
					
					sv10.dedup();
					let as_ptr = sv10.as_ptr();
					println!("{:?}", as_ptr as usize);
					
					sv10.clear();
					let is_empty_check = sv10.is_empty();
					println!("{:?}", is_empty_check);
					
					let empty_drain = sv10.drain(..);
					for drained_item in empty_drain {
						println!("{:?}", drained_item);
					}
					
					println!("{:?}", sv10.len());
				},
				9 => {
					let mut sv11 = SmallVec::<[char; 64]>::new();
					let char_count = _to_u8(GLOBAL_DATA, base_idx + 1) % 15;
					for idx in 0..char_count {
						sv11.push(_to_char(GLOBAL_DATA, base_idx + 2 + idx as usize * 4));
					}
					
					let insert_pos = _to_usize(GLOBAL_DATA, base_idx + 62);
					let char_to_insert = _to_char(GLOBAL_DATA, base_idx + 70);
					sv11.insert(insert_pos, char_to_insert);
					
					let swap_remove_idx = _to_usize(GLOBAL_DATA, base_idx + 74);
					if !sv11.is_empty() {
						let swapped = sv11.swap_remove(swap_remove_idx % sv11.len());
						println!("{:?}", swapped);
					}
					
					sv11.extend_from_slice(&[_to_char(GLOBAL_DATA, base_idx + 78)]);
					let range_drain = sv11.drain(_to_usize(GLOBAL_DATA, base_idx + 82).._to_usize(GLOBAL_DATA, base_idx + 90));
					let size_hint = range_drain.size_hint();
					println!("{:?}", size_hint);
					let final_collected: Vec<_> = range_drain.collect();
					println!("{:?}", final_collected.len());
				},
				10 => {
					let bool_slice = [_to_bool(GLOBAL_DATA, base_idx + 1); 30];
					let mut sv12 = SmallVec::<[bool; 128]>::from_slice(&bool_slice);
					
					sv12.reserve_exact(_to_usize(GLOBAL_DATA, base_idx + 2));
					sv12.push(_to_bool(GLOBAL_DATA, base_idx + 10));
					
					let partial_range = _to_usize(GLOBAL_DATA, base_idx + 11).._to_usize(GLOBAL_DATA, base_idx + 19);
					let partial_drain: Vec<_> = sv12.drain(partial_range).collect();
					if !partial_drain.is_empty() {
						println!("{:?}", partial_drain[0]);
					}
					let remaining = partial_drain;
					println!("{:?}", remaining.len());
					
					println!("{:?}", sv12.capacity());
				},
				11 => {
					let constructor_type = _to_u8(GLOBAL_DATA, base_idx + 1) % 3;
					let mut sv13 = match constructor_type {
						0 => SmallVec::<[usize; 32]>::new(),
						1 => SmallVec::<[usize; 32]>::with_capacity(_to_usize(GLOBAL_DATA, base_idx + 2)),
						_ => {
							let init_data = vec![_to_usize(GLOBAL_DATA, base_idx + 10); _to_usize(GLOBAL_DATA, base_idx + 18) % 20];
							SmallVec::<[usize; 32]>::from_vec(init_data)
						}
					};
					
					let many_insert_data = vec![
						_to_usize(GLOBAL_DATA, base_idx + 26),
						_to_usize(GLOBAL_DATA, base_idx + 34),
						_to_usize(GLOBAL_DATA, base_idx + 42)
					];
					let insert_pos = _to_usize(GLOBAL_DATA, base_idx + 50);
					sv13.insert_many(insert_pos, many_insert_data);
					
					let mut another_sv = SmallVec::<[usize; 32]>::new();
					another_sv.push(_to_usize(GLOBAL_DATA, base_idx + 58));
					sv13.append(&mut another_sv);
					
					let full_range_drain = sv13.drain(..);
					let final_drain_vec: Vec<_> = full_range_drain.collect();
					println!("{:?}", final_drain_vec.len());
				},
				12 => {
					let slice_source = [_to_i16(GLOBAL_DATA, base_idx + 1); 25];
					let mut sv14 = SmallVec::<[i16; 64]>::from_slice(&slice_source);
					
					sv14.insert_from_slice(_to_usize(GLOBAL_DATA, base_idx + 3), &[_to_i16(GLOBAL_DATA, base_idx + 11); 12]);
					
					let back_drain_start = _to_usize(GLOBAL_DATA, base_idx + 13);
					let mut back_drain = sv14.drain(back_drain_start..);
					if let Some(back_item) = back_drain.next_back() {
						println!("{:?}", back_item);
					}
					let back_collected: Vec<_> = back_drain.collect();
					println!("{:?}", back_collected.len());
					
					sv14.dedup_by(|a, b| a == b);
					println!("{:?}", sv14.len());
				},
				13 => {
					let small_vec_source = SmallVec::<[isize; 16]>::from_elem(_to_isize(GLOBAL_DATA, base_idx + 1), _to_usize(GLOBAL_DATA, base_idx + 9) % 10);
					let mut sv15 = small_vec_source.clone();
					
					sv15.grow(_to_usize(GLOBAL_DATA, base_idx + 17));
					sv15.push(_to_isize(GLOBAL_DATA, base_idx + 25));
					
					let as_mut_ptr = sv15.as_mut_ptr();
					println!("{:?}", as_mut_ptr as usize);
					
					let size_range = _to_usize(GLOBAL_DATA, base_idx + 33).._to_usize(GLOBAL_DATA, base_idx + 41);
					let mut size_drain = sv15.drain(size_range);
					let size_hint_result = size_drain.size_hint();
					println!("{:?}", size_hint_result);
					let size_collected: Vec<_> = size_drain.collect();
					println!("{:?}", size_collected.len());
					
					if let Ok(inner_array) = sv15.into_inner() {
						println!("{:?}", inner_array.len());
					}
				},
				_ => {
					let mut sv_default = SmallVec::<[u128; 1024]>::default();
					let insert_many_data = vec![
						_to_u128(GLOBAL_DATA, base_idx + 1),
						_to_u128(GLOBAL_DATA, base_idx + 17),
						_to_u128(GLOBAL_DATA, base_idx + 33)
					];
					let insert_pos = _to_usize(GLOBAL_DATA, base_idx + 49);
					sv_default.insert_many(insert_pos, insert_many_data);
					
					let swap_remove_idx = _to_usize(GLOBAL_DATA, base_idx + 57);
					if !sv_default.is_empty() {
						let swapped = sv_default.swap_remove(swap_remove_idx % sv_default.len());
						println!("{:?}", swapped);
					}
					
					let partial_range_start = _to_usize(GLOBAL_DATA, base_idx + 65);
					let partial_range_end = _to_usize(GLOBAL_DATA, base_idx + 73);
					let range_drain = sv_default.drain(partial_range_start..partial_range_end);
					let size_hint = range_drain.size_hint();
					println!("{:?}", size_hint);
					let final_collected: Vec<_> = range_drain.collect();
					println!("{:?}", final_collected.len());
					
					let boxed_slice = sv_default.into_boxed_slice();
					println!("{:?}", boxed_slice.len());
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