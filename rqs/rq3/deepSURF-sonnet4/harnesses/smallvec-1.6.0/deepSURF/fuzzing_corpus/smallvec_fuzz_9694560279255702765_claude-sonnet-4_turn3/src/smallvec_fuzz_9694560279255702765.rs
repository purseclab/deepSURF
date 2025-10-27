#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

#[derive(Debug)]
struct CustomType3(String);
#[derive(Debug)]
struct CustomType2(String);
#[derive(Debug)]
struct CustomType1(String);

impl core::iter::Iterator for CustomType3 {
	type Item = CustomType1;
	
	fn size_hint(&self) -> (usize, Option<usize>) {
		let global_data = get_global_data();
		let GLOBAL_DATA = global_data.first_half;
		let custom_impl_num = _to_usize(GLOBAL_DATA, 9);
		let custom_impl_inst_num = self.0.len();
		let selector = (custom_impl_num + custom_impl_inst_num) % 3;
		if selector == 0{
			panic!("INTENTIONAL PANIC!");
		}
		let GLOBAL_DATA = match selector{
				1 => global_data.first_half,
				_ => global_data.second_half,
		};
		let t_2 = _to_usize(GLOBAL_DATA, 17);
		let t_3 = _to_usize(GLOBAL_DATA, 25);
		let t_4 = Some(t_3);
		let t_5 = (t_2, t_4);
		return t_5;
	}
	
	fn next(&mut self) -> Option<Self::Item> {
		let global_data = get_global_data();
		let GLOBAL_DATA = global_data.first_half;
		let custom_impl_num = _to_usize(GLOBAL_DATA, 33);
		let custom_impl_inst_num = self.0.len();
		let selector = (custom_impl_num + custom_impl_inst_num) % 3;
		if selector == 0{
			panic!("INTENTIONAL PANIC!");
		}
		let GLOBAL_DATA = match selector{
				1 => global_data.first_half,
				_ => global_data.second_half,
		};
		let mut t_6 = _to_u8(GLOBAL_DATA, 41) % 17;
		let t_7 = _to_str(GLOBAL_DATA, 42, 42 + t_6 as usize);
		let t_8 = String::from(t_7);
		let t_9 = CustomType1(t_8);
		let t_10 = Some(t_9);
		return t_10;
	}
}

impl core::iter::IntoIterator for CustomType2 {
	type Item = CustomType1;
	type IntoIter = CustomType3;
	
	fn into_iter(self) -> Self::IntoIter {
		let global_data = get_global_data();
		let GLOBAL_DATA = global_data.first_half;
		let custom_impl_num = _to_usize(GLOBAL_DATA, 58);
		let custom_impl_inst_num = self.0.len();
		let selector = (custom_impl_num + custom_impl_inst_num) % 3;
		if selector == 0{
			panic!("INTENTIONAL PANIC!");
		}
		let GLOBAL_DATA = match selector{
				1 => global_data.first_half,
				_ => global_data.second_half,
		};
		let mut t_11 = _to_u8(GLOBAL_DATA, 66) % 17;
		let t_12 = _to_str(GLOBAL_DATA, 67, 67 + t_11 as usize);
		let t_13 = String::from(t_12);
		let t_14 = CustomType3(t_13);
		return t_14;
	}
}

fn main (){
	fuzz_nohook!(|data: &[u8]| {
		if data.len() < 500 {return;}
		set_global_data(data);
		let global_data = get_global_data();
		let mut GLOBAL_DATA = global_data.first_half;
		
		let num_operations = _to_u8(GLOBAL_DATA, 0) % 65;
		
		for i in 0..num_operations {
			let operation = _to_u8(GLOBAL_DATA, 1 + i as usize) % 20;
			
			match operation {
				0 => {
					let t_15 = _to_u8(GLOBAL_DATA, 83) % 17;
					let t_16 = _to_str(GLOBAL_DATA, 84, 84 + t_15 as usize);
					let t_17 = String::from(t_16);
					let t_18 = CustomType2(t_17);
					let mut result = smallvec::SmallVec::<[CustomType1; 32]>::from_iter(t_18);
					let slice_ref = result.as_slice();
					println!("{:?}", slice_ref.len());
					for item in slice_ref.iter() {
						println!("{:?}", *item);
					}
					result.clear();
					let _ = result.capacity();
				},
				1 => {
					let constructor_selector = _to_u8(GLOBAL_DATA, 100) % 5;
					let mut sv = match constructor_selector {
						0 => smallvec::SmallVec::<[u8; 16]>::new(),
						1 => smallvec::SmallVec::<[u8; 16]>::with_capacity(_to_usize(GLOBAL_DATA, 101)),
						2 => {
							let vec_data = vec![_to_u8(GLOBAL_DATA, 105), _to_u8(GLOBAL_DATA, 106), _to_u8(GLOBAL_DATA, 107)];
							smallvec::SmallVec::<[u8; 16]>::from_vec(vec_data)
						},
						3 => {
							let slice_data = [_to_u8(GLOBAL_DATA, 108), _to_u8(GLOBAL_DATA, 109)];
							smallvec::SmallVec::<[u8; 16]>::from_slice(&slice_data)
						},
						_ => {
							let elem = _to_u8(GLOBAL_DATA, 110);
							let count = _to_usize(GLOBAL_DATA, 111);
							smallvec::SmallVec::<[u8; 16]>::from_elem(elem, count)
						}
					};
					let len = _to_usize(GLOBAL_DATA, 120);
					sv.resize(len, _to_u8(GLOBAL_DATA, 128));
					println!("{:?}", sv.len());
					let as_slice = sv.as_slice();
					println!("{:?}", as_slice.len());
					sv.extend_from_slice(&[_to_u8(GLOBAL_DATA, 130), _to_u8(GLOBAL_DATA, 131)]);
				},
				2 => {
					let capacity = _to_usize(GLOBAL_DATA, 140);
					let mut sv = smallvec::SmallVec::<[i32; 20]>::with_capacity(capacity);
					let element = _to_i32(GLOBAL_DATA, 148);
					sv.push(element);
					sv.push(_to_i32(GLOBAL_DATA, 152));
					let popped = sv.pop();
					println!("{:?}", popped);
					sv.reserve(_to_usize(GLOBAL_DATA, 156));
					sv.shrink_to_fit();
					let as_ptr = sv.as_ptr();
					println!("{:?}", as_ptr);
					sv.truncate(_to_usize(GLOBAL_DATA, 164));
				},
				3 => {
					let mut sv = smallvec::SmallVec::<[f64; 14]>::new();
					let slice_data = [_to_f64(GLOBAL_DATA, 170), _to_f64(GLOBAL_DATA, 178), _to_f64(GLOBAL_DATA, 186)];
					sv.extend_from_slice(&slice_data);
					let slice_ref = sv.as_slice();
					for item in slice_ref {
						println!("{:?}", *item);
					}
					let mut_slice = sv.as_mut_slice();
					mut_slice[0] = _to_f64(GLOBAL_DATA, 194);
					println!("{:?}", mut_slice[0]);
					let deref_result = sv.deref();
					println!("{:?}", deref_result.len());
				},
				4 => {
					let elem = _to_u16(GLOBAL_DATA, 200);
					let count = _to_usize(GLOBAL_DATA, 202);
					let sv = smallvec::SmallVec::<[u16; 24]>::from_elem(elem, count);
					let vec_result = sv.into_vec();
					println!("{:?}", vec_result.len());
				},
				5 => {
					let vec_data = vec![_to_i8(GLOBAL_DATA, 210), _to_i8(GLOBAL_DATA, 211), _to_i8(GLOBAL_DATA, 212)];
					let mut sv = smallvec::SmallVec::<[i8; 12]>::from_vec(vec_data);
					let slice_ref = sv.as_slice();
					println!("{:?}", slice_ref[0]);
					sv.retain(|&mut x| x > 0);
					let retained_slice = sv.as_slice();
					for item in retained_slice.iter() {
						println!("{:?}", *item);
					}
				},
				6 => {
					let array_data = [_to_u32(GLOBAL_DATA, 220), _to_u32(GLOBAL_DATA, 224), _to_u32(GLOBAL_DATA, 228), _to_u32(GLOBAL_DATA, 232)];
					let sv = smallvec::SmallVec::from_buf(array_data);
					let mut sv2 = smallvec::SmallVec::<[u32; 14]>::new();
					sv2.append(&mut sv.clone());
					println!("{:?}", sv2.capacity());
					let iter = sv2.into_iter();
					let next_item = iter.as_slice();
					println!("{:?}", next_item.len());
				},
				7 => {
					let mut sv1 = smallvec::SmallVec::<[char; 18]>::new();
					let mut sv2 = smallvec::SmallVec::<[char; 18]>::new();
					sv1.push(_to_char(GLOBAL_DATA, 240));
					sv2.push(_to_char(GLOBAL_DATA, 244));
					let cmp_result = sv1.cmp(&sv2);
					println!("{:?}", cmp_result);
					let eq_result = sv1.eq(&sv2);
					println!("{:?}", eq_result);
					let mut sv3 = sv1.clone();
					sv3.grow(_to_usize(GLOBAL_DATA, 248));
				},
				8 => {
					let mut sv = smallvec::SmallVec::<[bool; 20]>::new();
					sv.push(_to_bool(GLOBAL_DATA, 260));
					sv.push(_to_bool(GLOBAL_DATA, 261));
					sv.push(_to_bool(GLOBAL_DATA, 262));
					let drain_result = sv.drain(..);
					let drained_vec: Vec<bool> = drain_result.collect();
					println!("{:?}", drained_vec.len());
				},
				9 => {
					let mut sv = smallvec::SmallVec::<[usize; 12]>::new();
					let range_start = _to_usize(GLOBAL_DATA, 270);
					let range_end = _to_usize(GLOBAL_DATA, 278);
					for j in range_start..range_end {
						sv.push(j);
					}
					let len_before = sv.len();
					sv.clear();
					let len_after = sv.len();
					println!("{:?} -> {:?}", len_before, len_after);
					let is_empty = sv.is_empty();
					println!("{:?}", is_empty);
				},
				10 => {
					let slice_data = [_to_isize(GLOBAL_DATA, 290), _to_isize(GLOBAL_DATA, 298), _to_isize(GLOBAL_DATA, 306)];
					let sv = smallvec::SmallVec::<[isize; 32]>::from_slice(&slice_data);
					let to_smallvec_result: smallvec::SmallVec<[isize; 32]> = slice_data.to_smallvec();
					let comparison = sv.eq(&to_smallvec_result);
					println!("{:?}", comparison);
					let partial_cmp = sv.partial_cmp(&to_smallvec_result);
					println!("{:?}", partial_cmp);
				},
				11 => {
					let mut sv = smallvec::SmallVec::<[u64; 16]>::new();
					let index = _to_usize(GLOBAL_DATA, 320);
					let element = _to_u64(GLOBAL_DATA, 328);
					sv.push(element);
					sv.push(_to_u64(GLOBAL_DATA, 336));
					sv.insert(index, element);
					let removed = sv.remove(index);
					println!("{:?}", removed);
					let swapped = sv.swap_remove(index);
					println!("{:?}", swapped);
				},
				12 => {
					let mut sv = smallvec::SmallVec::<[i128; 14]>::new();
					let value = _to_i128(GLOBAL_DATA, 350);
					sv.push(value);
					sv.push(value);
					let partial_cmp_result = sv.partial_cmp(&sv);
					println!("{:?}", partial_cmp_result);
					let boxed_slice = sv.clone().into_boxed_slice();
					println!("{:?}", boxed_slice.len());
				},
				13 => {
					let mut sv = smallvec::SmallVec::<[f32; 16]>::new();
					sv.push(_to_f32(GLOBAL_DATA, 370));
					sv.push(_to_f32(GLOBAL_DATA, 374));
					sv.retain(|&mut x| x > 0.0);
					let slice_mut = sv.as_mut_slice();
					slice_mut[0] = _to_f32(GLOBAL_DATA, 378);
					println!("{:?}", slice_mut[0]);
					sv.dedup();
					let deref_mut = sv.deref_mut();
					println!("{:?}", deref_mut.len());
				},
				14 => {
					let vector = vec![_to_u128(GLOBAL_DATA, 390), _to_u128(GLOBAL_DATA, 406)];
					let iter = vector.into_iter();
					let sv = smallvec::SmallVec::<[u128; 14]>::from_iter(iter);
					let into_iter = sv.into_iter();
					let collected: Vec<u128> = into_iter.collect();
					println!("{:?}", collected.len());
				},
				15 => {
					let mut sv = smallvec::SmallVec::<[String; 12]>::new();
					sv.push(String::from("test"));
					sv.resize_with(_to_usize(GLOBAL_DATA, 420), || String::from("default"));
					let as_ref = sv.as_ref();
					println!("{:?}", as_ref.len());
					sv.insert_many(_to_usize(GLOBAL_DATA, 428), vec![String::from("inserted")]);
				},
				16 => {
					let mut sv = smallvec::SmallVec::<[Vec<u8>; 12]>::new();
					sv.push(vec![1, 2, 3]);
					sv.push(vec![4, 5, 6]);
					let hash_result = {
						use std::collections::hash_map::DefaultHasher;
						use std::hash::{Hash, Hasher};
						let mut hasher = DefaultHasher::new();
						sv.hash(&mut hasher);
						hasher.finish()
					};
					println!("{:?}", hash_result);
				},
				17 => {
					let slice_data = &[1u8, 2, 3, 4, 5];
					let mut sv = smallvec::SmallVec::<[u8; 20]>::from_slice(slice_data);
					sv.insert_from_slice(_to_usize(GLOBAL_DATA, 440), &[10, 11]);
					let as_mut_ptr = sv.as_mut_ptr();
					println!("{:?}", as_mut_ptr);
					sv.reserve_exact(_to_usize(GLOBAL_DATA, 442));
				},
				18 => {
					let mut sv1 = smallvec::SmallVec::<[i32; 15]>::new();
					let mut sv2 = smallvec::SmallVec::<[i32; 15]>::new();
					sv1.extend([1, 2, 3].iter().cloned());
					sv2.extend([4, 5, 6].iter().cloned());
					let spilled1 = sv1.spilled();
					let spilled2 = sv2.spilled();
					println!("{:?} {:?}", spilled1, spilled2);
					let inline_size = sv1.inline_size();
					println!("{:?}", inline_size);
				},
				_ => {
					let array_data = [_to_u32(GLOBAL_DATA, 450); 15];
					let sv = smallvec::SmallVec::from_buf(array_data);
					let into_inner_result = sv.into_inner();
					match into_inner_result {
						Ok(array) => println!("{:?}", array.len()),
						Err(sv_back) => println!("{:?}", sv_back.len()),
					}
				}
			}
			
			GLOBAL_DATA = if i % 2 == 0 { global_data.first_half } else { global_data.second_half };
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