#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

struct CustomType1(String);

fn main (){
	fuzz_nohook!(|data: &[u8]| {
		if data.len() < 500 {return;}
		set_global_data(data);
		let global_data = get_global_data();
		let GLOBAL_DATA = global_data.first_half;
		
		let num_operations = _to_u8(GLOBAL_DATA, 0) % 65;
		
		for i in 0..num_operations {
			let operation = _to_u8(GLOBAL_DATA, 1 + i as usize) % 15;
			
			match operation {
				0 => {
					let constructor_choice = _to_u8(GLOBAL_DATA, 66 + i as usize) % 6;
					let mut smallvec_target = match constructor_choice {
						0 => {
							let capacity = _to_usize(GLOBAL_DATA, 132 + i as usize * 8);
							SmallVec::<[u32; 16]>::with_capacity(capacity)
						},
						1 => {
							let data_len = _to_u8(GLOBAL_DATA, 132 + i as usize * 8) % 65;
							let mut vec_data = Vec::new();
							for j in 0..data_len {
								vec_data.push(_to_u32(GLOBAL_DATA, 200 + i as usize * 8 + j as usize));
							}
							SmallVec::<[u32; 16]>::from_vec(vec_data)
						},
						2 => {
							let arr: [u32; 16] = [
								_to_u32(GLOBAL_DATA, 200 + i as usize * 8),
								_to_u32(GLOBAL_DATA, 204 + i as usize * 8),
								_to_u32(GLOBAL_DATA, 208 + i as usize * 8),
								_to_u32(GLOBAL_DATA, 212 + i as usize * 8),
								_to_u32(GLOBAL_DATA, 216 + i as usize * 8),
								_to_u32(GLOBAL_DATA, 220 + i as usize * 8),
								_to_u32(GLOBAL_DATA, 224 + i as usize * 8),
								_to_u32(GLOBAL_DATA, 228 + i as usize * 8),
								_to_u32(GLOBAL_DATA, 232 + i as usize * 8),
								_to_u32(GLOBAL_DATA, 236 + i as usize * 8),
								_to_u32(GLOBAL_DATA, 240 + i as usize * 8),
								_to_u32(GLOBAL_DATA, 244 + i as usize * 8),
								_to_u32(GLOBAL_DATA, 248 + i as usize * 8),
								_to_u32(GLOBAL_DATA, 252 + i as usize * 8),
								_to_u32(GLOBAL_DATA, 256 + i as usize * 8),
								_to_u32(GLOBAL_DATA, 260 + i as usize * 8)
							];
							let len = _to_usize(GLOBAL_DATA, 264 + i as usize * 8);
							SmallVec::<[u32; 16]>::from_buf_and_len(arr, len)
						},
						3 => {
							let slice_len = _to_u8(GLOBAL_DATA, 132 + i as usize * 8) % 65;
							let mut slice_data = Vec::new();
							for j in 0..slice_len {
								slice_data.push(_to_u32(GLOBAL_DATA, 200 + i as usize * 8 + j as usize * 4));
							}
							SmallVec::<[u32; 16]>::from_slice(&slice_data)
						},
						4 => {
							let elem = _to_u32(GLOBAL_DATA, 132 + i as usize * 8);
							let count = _to_usize(GLOBAL_DATA, 136 + i as usize * 8);
							SmallVec::<[u32; 16]>::from_elem(elem, count)
						},
						_ => SmallVec::<[u32; 16]>::new()
					};
					
					let reserve_amount = _to_usize(GLOBAL_DATA, 200 + i as usize * 8);
					smallvec_target.reserve_exact(reserve_amount);
					
					let capacity = smallvec_target.capacity();
					println!("{}", capacity);
					
					let slice_ref = smallvec_target.as_slice();
					if !slice_ref.is_empty() {
						let elem = &slice_ref[0];
						println!("{}", elem);
					}
					
					let len = smallvec_target.len();
					println!("{}", len);
				},
				1 => {
					let mut sv = SmallVec::<[u32; 32]>::new();
					let push_count = _to_u8(GLOBAL_DATA, 132 + i as usize) % 20;
					for j in 0..push_count {
						let value = _to_u32(GLOBAL_DATA, 150 + i as usize * 8 + j as usize * 4);
						sv.push(value);
					}
					
					let additional = _to_usize(GLOBAL_DATA, 200 + i as usize * 8);
					sv.reserve_exact(additional);
					
					if !sv.is_empty() {
						let pop_result = sv.pop();
						if let Some(val) = pop_result {
							println!("{}", val);
						}
					}
					
					let mut_slice_ref = sv.as_mut_slice();
					if !mut_slice_ref.is_empty() {
						let first_elem = &mut mut_slice_ref[0];
						println!("{}", first_elem);
					}
				},
				2 => {
					let mut sv1 = SmallVec::<[u32; 8]>::new();
					let mut sv2 = SmallVec::<[u32; 8]>::new();
					
					let push_count1 = _to_u8(GLOBAL_DATA, 132 + i as usize) % 10;
					for j in 0..push_count1 {
						sv1.push(_to_u32(GLOBAL_DATA, 150 + j as usize * 4));
					}
					
					let push_count2 = _to_u8(GLOBAL_DATA, 133 + i as usize) % 10;
					for j in 0..push_count2 {
						sv2.push(_to_u32(GLOBAL_DATA, 200 + j as usize * 4));
					}
					
					let reserve_amount = _to_usize(GLOBAL_DATA, 250 + i as usize * 8);
					sv1.reserve_exact(reserve_amount);
					
					sv1.append(&mut sv2);
					
					let ordering = sv1.cmp(&sv2);
					println!("{:?}", ordering);
				},
				3 => {
					let mut sv = SmallVec::<[u32; 64]>::with_capacity(_to_usize(GLOBAL_DATA, 132 + i as usize * 8));
					
					let insert_count = _to_u8(GLOBAL_DATA, 200 + i as usize) % 15;
					for j in 0..insert_count {
						let value = _to_u32(GLOBAL_DATA, 210 + j as usize * 4);
						sv.push(value);
					}
					
					let reserve_exact_amount = _to_usize(GLOBAL_DATA, 250 + i as usize * 8);
					sv.reserve_exact(reserve_exact_amount);
					
					if !sv.is_empty() {
						let index = _to_usize(GLOBAL_DATA, 260 + i as usize * 8);
						let new_elem = _to_u32(GLOBAL_DATA, 270 + i as usize * 4);
						sv.insert(index, new_elem);
					}
					
					sv.shrink_to_fit();
				},
				4 => {
					let mut sv = SmallVec::<[u32; 12]>::new();
					let extend_count = _to_u8(GLOBAL_DATA, 132 + i as usize) % 30;
					let mut extend_vec = Vec::new();
					for j in 0..extend_count {
						extend_vec.push(_to_u32(GLOBAL_DATA, 150 + j as usize * 4));
					}
					
					sv.extend(extend_vec.iter().cloned());
					
					let reserve_additional = _to_usize(GLOBAL_DATA, 250 + i as usize * 8);
					sv.reserve_exact(reserve_additional);
					
					let truncate_len = _to_usize(GLOBAL_DATA, 260 + i as usize * 8);
					sv.truncate(truncate_len);
				},
				5 => {
					let slice_data = [
						_to_u32(GLOBAL_DATA, 132 + i as usize * 8),
						_to_u32(GLOBAL_DATA, 136 + i as usize * 8),
						_to_u32(GLOBAL_DATA, 140 + i as usize * 8),
						_to_u32(GLOBAL_DATA, 144 + i as usize * 8),
						_to_u32(GLOBAL_DATA, 148 + i as usize * 8)
					];
					let mut sv = SmallVec::<[u32; 20]>::from_slice(&slice_data);
					
					let additional_capacity = _to_usize(GLOBAL_DATA, 200 + i as usize * 8);
					sv.reserve_exact(additional_capacity);
					
					let extend_slice = [
						_to_u32(GLOBAL_DATA, 250 + i as usize * 4),
						_to_u32(GLOBAL_DATA, 254 + i as usize * 4)
					];
					sv.extend_from_slice(&extend_slice);
				},
				6 => {
					let mut sv = SmallVec::<[u32; 16]>::new();
					let resize_len = _to_usize(GLOBAL_DATA, 132 + i as usize * 8);
					let resize_value = _to_u32(GLOBAL_DATA, 140 + i as usize * 4);
					
					sv.resize(resize_len, resize_value);
					
					let exact_reserve = _to_usize(GLOBAL_DATA, 200 + i as usize * 8);
					sv.reserve_exact(exact_reserve);
					
					sv.dedup();
				},
				7 => {
					let vec_data = vec![
						_to_u32(GLOBAL_DATA, 132 + i as usize * 8),
						_to_u32(GLOBAL_DATA, 136 + i as usize * 8),
						_to_u32(GLOBAL_DATA, 140 + i as usize * 8)
					];
					let mut sv = SmallVec::<[u32; 24]>::from(vec_data);
					
					let pre_reserve = _to_usize(GLOBAL_DATA, 200 + i as usize * 8);
					sv.reserve_exact(pre_reserve);
					
					let drain_start = _to_usize(GLOBAL_DATA, 250 + i as usize * 8);
					let drain_end = _to_usize(GLOBAL_DATA, 258 + i as usize * 8);
					let drain_range = drain_start..drain_end;
					let mut drain_iter = sv.drain(drain_range);
					
					while let Some(item) = drain_iter.next() {
						println!("{}", item);
					}
				},
				8 => {
					let arr: [u32; 28] = [0; 28];
					let mut sv = SmallVec::<[u32; 28]>::from_buf(arr);
					
					let capacity_request = _to_usize(GLOBAL_DATA, 132 + i as usize * 8);
					sv.reserve_exact(capacity_request);
					
					let clear_flag = _to_bool(GLOBAL_DATA, 200 + i as usize);
					if clear_flag {
						sv.clear();
					}
					
					let grow_amount = _to_usize(GLOBAL_DATA, 250 + i as usize * 8);
					sv.grow(grow_amount);
				},
				9 => {
					let mut sv = SmallVec::<[u32; 36]>::new();
					let initial_push = _to_u32(GLOBAL_DATA, 132 + i as usize * 4);
					sv.push(initial_push);
					
					let target_reserve = _to_usize(GLOBAL_DATA, 200 + i as usize * 8);
					sv.reserve_exact(target_reserve);
					
					let as_ptr_result = sv.as_ptr();
					println!("{:?}", as_ptr_result);
					
					let as_mut_ptr_result = sv.as_mut_ptr();
					println!("{:?}", as_mut_ptr_result);
				},
				10 => {
					let init_data = [_to_u32(GLOBAL_DATA, 132 + i as usize * 4); 15];
					let mut sv = SmallVec::<[u32; 15]>::from_slice(&init_data);
					
					let reserve_before = _to_usize(GLOBAL_DATA, 200 + i as usize * 8);
					sv.reserve_exact(reserve_before);
					
					let remove_index = _to_usize(GLOBAL_DATA, 250 + i as usize * 8);
					if !sv.is_empty() {
						let removed = sv.remove(remove_index);
						println!("{}", removed);
					}
				},
				11 => {
					let elem = _to_u32(GLOBAL_DATA, 132 + i as usize * 4);
					let count = _to_usize(GLOBAL_DATA, 136 + i as usize * 8);
					let mut sv = SmallVec::<[u32; 32]>::from_elem(elem, count);
					
					let exact_additional = _to_usize(GLOBAL_DATA, 200 + i as usize * 8);
					sv.reserve_exact(exact_additional);
					
					let swap_remove_index = _to_usize(GLOBAL_DATA, 250 + i as usize * 8);
					if !sv.is_empty() {
						let swapped = sv.swap_remove(swap_remove_index);
						println!("{}", swapped);
					}
				},
				12 => {
					let slice_source = [
						_to_u32(GLOBAL_DATA, 132 + i as usize * 8),
						_to_u32(GLOBAL_DATA, 136 + i as usize * 8),
						_to_u32(GLOBAL_DATA, 140 + i as usize * 8),
						_to_u32(GLOBAL_DATA, 144 + i as usize * 8)
					];
					let mut sv: SmallVec<[u32; 32]> = slice_source.to_smallvec();
					
					let additional_reserve = _to_usize(GLOBAL_DATA, 200 + i as usize * 8);
					sv.reserve_exact(additional_reserve);
					
					let insert_index = _to_usize(GLOBAL_DATA, 250 + i as usize * 8);
					let insert_slice = [
						_to_u32(GLOBAL_DATA, 260 + i as usize * 4),
						_to_u32(GLOBAL_DATA, 264 + i as usize * 4)
					];
					sv.insert_from_slice(insert_index, &insert_slice);
				},
				13 => {
					let source_vec = vec![
						_to_u32(GLOBAL_DATA, 132 + i as usize * 8),
						_to_u32(GLOBAL_DATA, 136 + i as usize * 8)
					];
					let mut sv = SmallVec::<[u32; 32]>::from_iter(source_vec.into_iter());
					
					let capacity_increase = _to_usize(GLOBAL_DATA, 200 + i as usize * 8);
					sv.reserve_exact(capacity_increase);
					
					let into_vec_result = sv.into_vec();
					println!("{:?}", into_vec_result);
				},
				_ => {
					let mut sv = SmallVec::<[u32; 64]>::new();
					let final_capacity = _to_usize(GLOBAL_DATA, 132 + i as usize * 8);
					sv.reserve_exact(final_capacity);
					
					let final_elem = _to_u32(GLOBAL_DATA, 200 + i as usize * 4);
					sv.push(final_elem);
					
					let spilled_status = sv.spilled();
					println!("{}", spilled_status);
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