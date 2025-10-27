#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

#[derive(Clone, Copy)]
struct CustomType1(u8);
#[derive(Clone, Copy)]
struct CustomType0(u8);
#[derive(Clone, Copy, Debug)]
struct CustomType3(u8);
#[derive(Clone, Copy)]
struct CustomType4(u8);
#[derive(Clone, Copy)]
struct CustomType2(u8);

fn main (){
	fuzz_nohook!(|data: &[u8]| {
		if data.len() < 2500 {return;}
		set_global_data(data);
		let global_data = get_global_data();
		let GLOBAL_DATA = global_data.first_half;
		
		let operations_count = _to_u8(GLOBAL_DATA, 0) % 15 + 1;
		
		for i in 0..operations_count {
			let base_offset = (i as usize) * 150;
			let operation_type = _to_u8(GLOBAL_DATA, base_offset + 1) % 8;
			
			match operation_type {
				0 => {
					let constructor_choice = _to_u8(GLOBAL_DATA, base_offset + 2) % 5;
					let mut smallvec = match constructor_choice {
						0 => SmallVec::<[CustomType3; 16]>::new(),
						1 => {
							let capacity = _to_usize(GLOBAL_DATA, base_offset + 3);
							SmallVec::<[CustomType3; 16]>::with_capacity(capacity)
						},
						2 => {
							let mut vec = Vec::new();
							let vec_size = _to_u8(GLOBAL_DATA, base_offset + 11) % 65;
							for j in 0..vec_size {
								let val = _to_u8(GLOBAL_DATA, base_offset + 12 + j as usize);
								vec.push(CustomType3(val));
							}
							SmallVec::<[CustomType3; 16]>::from_vec(vec)
						},
						3 => {
							let slice_size = _to_u8(GLOBAL_DATA, base_offset + 4) % 16;
							let mut temp_vec = Vec::new();
							for j in 0..slice_size {
								let val = _to_u8(GLOBAL_DATA, base_offset + 5 + j as usize);
								temp_vec.push(CustomType3(val));
							}
							SmallVec::<[CustomType3; 16]>::from(&temp_vec[..])
						},
						_ => {
							let elem_val = _to_u8(GLOBAL_DATA, base_offset + 6);
							let elem = CustomType3(elem_val);
							let count = _to_usize(GLOBAL_DATA, base_offset + 17);
							SmallVec::<[CustomType3; 16]>::from_elem(elem, count)
						}
					};
					
					let mut operations_on_vec = _to_u8(GLOBAL_DATA, base_offset + 100) % 10 + 1;
					for j in 0..operations_on_vec {
						let vec_op = _to_u8(GLOBAL_DATA, base_offset + 101 + j as usize) % 15;
						let vec_op_offset = base_offset + 110 + j as usize * 10;
						
						match vec_op {
							0 => {
								let val = _to_u8(GLOBAL_DATA, vec_op_offset);
								smallvec.push(CustomType3(val));
							},
							1 => { let _ = smallvec.pop(); },
							2 => {
								let index = _to_usize(GLOBAL_DATA, vec_op_offset);
								if !smallvec.is_empty() && index < smallvec.len() {
									let _ = smallvec.remove(index);
								}
							},
							3 => {
								let index = _to_usize(GLOBAL_DATA, vec_op_offset);
								let val = _to_u8(GLOBAL_DATA, vec_op_offset + 8);
								if index <= smallvec.len() {
									smallvec.insert(index, CustomType3(val));
								}
							},
							4 => {
								let new_len = _to_usize(GLOBAL_DATA, vec_op_offset);
								smallvec.truncate(new_len);
							},
							5 => { smallvec.clear(); },
							6 => {
								let additional = _to_usize(GLOBAL_DATA, vec_op_offset);
								smallvec.reserve(additional);
							},
							7 => { smallvec.shrink_to_fit(); },
							8 => {
								let slice_ref = smallvec.as_slice();
								println!("{:?}", slice_ref.len());
							},
							9 => {
								let mut_slice_ref = smallvec.as_mut_slice();
								println!("{:?}", mut_slice_ref.len());
							},
							10 => {
								if !smallvec.is_empty() {
									let index = _to_usize(GLOBAL_DATA, vec_op_offset) % smallvec.len();
									let elem_ref = &smallvec[index];
									println!("{:?}", elem_ref.0);
								}
							},
							11 => {
								let capacity_info = smallvec.capacity();
								println!("{:?}", capacity_info);
							},
							12 => { println!("{:?}", smallvec.len()); },
							13 => { println!("{:?}", smallvec.is_empty()); },
							_ => {
								if !smallvec.is_empty() {
									let index = _to_usize(GLOBAL_DATA, vec_op_offset) % smallvec.len();
									let _ = smallvec.swap_remove(index);
								}
							}
						}
					}
					
					let range_start = _to_usize(GLOBAL_DATA, base_offset + 140);
					let range_end = _to_usize(GLOBAL_DATA, base_offset + 148);
					if range_start <= range_end && range_end <= smallvec.len() {
						let mut drain = smallvec.drain(range_start..range_end);
						let _ = &mut drain;
					}
				},
				1 => {
					let mut smallvec = SmallVec::<[i32; 32]>::new();
					let vec_size = _to_u8(GLOBAL_DATA, base_offset + 2) % 64;
					for j in 0..vec_size {
						let val = _to_i32(GLOBAL_DATA, base_offset + 3 + j as usize * 4);
						smallvec.push(val);
					}
					
					let clone_result = smallvec.clone();
					println!("{:?}", clone_result.len());
					
					let mut other_vec = SmallVec::<[i32; 32]>::new();
					smallvec.append(&mut other_vec);
					
					let comparison_vec = SmallVec::<[i32; 32]>::from_elem(42, 5);
					let ordering = smallvec.cmp(&comparison_vec);
					println!("{:?}", ordering);
				},
				2 => {
					let mut smallvec = SmallVec::<[u8; 64]>::new();
					let initial_size = _to_u8(GLOBAL_DATA, base_offset + 2) % 64;
					for j in 0..initial_size {
						let byte_val = _to_u8(GLOBAL_DATA, base_offset + 3 + j as usize);
						smallvec.push(byte_val);
					}
					
					let slice_data = &smallvec[..];
					let new_smallvec = SmallVec::<[u8; 64]>::from(slice_data);
					println!("{:?}", new_smallvec.len());
					
					let into_iter = smallvec.into_iter();
					for item in into_iter {
						println!("{:?}", item);
						break;
					}
				},
				3 => {
					let vec_size = _to_u8(GLOBAL_DATA, base_offset + 2) % 32;
					let mut values = Vec::new();
					for j in 0..vec_size {
						let val = _to_f32(GLOBAL_DATA, base_offset + 3 + j as usize * 4);
						values.push(val);
					}
					
					let smallvec = SmallVec::<[f32; 16]>::from_iter(values.iter().cloned());
					println!("{:?}", smallvec.len());
					
					let boxed_slice = smallvec.into_boxed_slice();
					println!("{:?}", boxed_slice.len());
				},
				4 => {
					let mut smallvec = SmallVec::<[char; 20]>::new();
					let char_count = _to_u8(GLOBAL_DATA, base_offset + 2) % 20;
					for j in 0..char_count {
						let char_val = _to_char(GLOBAL_DATA, base_offset + 3 + j as usize * 4);
						smallvec.push(char_val);
					}
					
					if !smallvec.is_empty() {
						let deref_result = &*smallvec;
						println!("{:?}", deref_result.len());
						
						let mut_deref_result = &mut *smallvec;
						println!("{:?}", mut_deref_result.len());
					}
					
					let additional_capacity = _to_usize(GLOBAL_DATA, base_offset + 100);
					let reserve_result = smallvec.try_reserve(additional_capacity);
					match reserve_result {
						Ok(_) => println!("Reserved successfully"),
						Err(_) => println!("Reserve failed"),
					}
				},
				5 => {
					let mut smallvec = SmallVec::<[bool; 128]>::new();
					let bool_count = _to_u8(GLOBAL_DATA, base_offset + 2) % 64;
					for j in 0..bool_count {
						let bool_val = _to_bool(GLOBAL_DATA, base_offset + 3 + j as usize);
						smallvec.push(bool_val);
					}
					
					let retain_threshold = _to_u8(GLOBAL_DATA, base_offset + 70);
					smallvec.retain(|x| {
						if retain_threshold % 2 == 0 {
							panic!("INTENTIONAL PANIC!");
						}
						*x
					});
					
					smallvec.dedup();
					println!("{:?}", smallvec.len());
				},
				6 => {
					let mut smallvec1 = SmallVec::<[i64; 16]>::new();
					let mut smallvec2 = SmallVec::<[i64; 16]>::new();
					
					let size1 = _to_u8(GLOBAL_DATA, base_offset + 2) % 16;
					let size2 = _to_u8(GLOBAL_DATA, base_offset + 3) % 16;
					
					for j in 0..size1 {
						let val = _to_i64(GLOBAL_DATA, base_offset + 4 + j as usize * 8);
						smallvec1.push(val);
					}
					
					for j in 0..size2 {
						let val = _to_i64(GLOBAL_DATA, base_offset + 68 + j as usize * 8);
						smallvec2.push(val);
					}
					
					let eq_result = smallvec1.eq(&smallvec2);
					println!("{:?}", eq_result);
					
					let partial_cmp_result = smallvec1.partial_cmp(&smallvec2);
					println!("{:?}", partial_cmp_result);
				},
				_ => {
					let mut dest_smallvec = SmallVec::<[CustomType3; 24]>::new();
					let init_size = _to_u8(GLOBAL_DATA, base_offset + 2) % 24;
					for j in 0..init_size {
						let val = _to_u8(GLOBAL_DATA, base_offset + 3 + j as usize);
						dest_smallvec.push(CustomType3(val));
					}
					
					let extension_size = _to_u8(GLOBAL_DATA, base_offset + 80) % 16;
					let mut extension_slice = Vec::new();
					for j in 0..extension_size {
						let val = _to_u8(GLOBAL_DATA, base_offset + 81 + j as usize);
						extension_slice.push(CustomType3(val));
					}
					
					dest_smallvec.extend_from_slice(&extension_slice[..]);
					
					let range_start = _to_usize(GLOBAL_DATA, base_offset + 140);
					let range_end = _to_usize(GLOBAL_DATA, base_offset + 141);
					
					if range_start <= range_end && range_end <= dest_smallvec.len() {
						let mut final_drain = dest_smallvec.drain(range_start..range_end);
						let _ = &mut final_drain;
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