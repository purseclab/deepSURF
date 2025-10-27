#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

struct CustomType0(String);

#[derive(Debug)]
struct CustomType1(String);

impl core::clone::Clone for CustomType1 {
	fn clone(&self) -> Self {
		let global_data = get_global_data();
		let GLOBAL_DATA = global_data.first_half;
		let custom_impl_num = _to_usize(GLOBAL_DATA, 42);
		let custom_impl_inst_num = self.0.len();
		let selector = (custom_impl_num + custom_impl_inst_num) % 3;
		if selector == 0{
			panic!("INTENTIONAL PANIC!");
		}
		let GLOBAL_DATA = match selector{
				1 => global_data.first_half,
				_ => global_data.second_half,
		};
		let mut t_10 = _to_u8(GLOBAL_DATA, 50) % 17;
		let t_11 = _to_str(GLOBAL_DATA, 51, 51 + t_10 as usize);
		let t_12 = String::from(t_11);
		let t_13 = CustomType1(t_12);
		return t_13;
	}
}

impl PartialEq for CustomType1 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for CustomType1 {}

impl PartialOrd for CustomType1 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for CustomType1 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

fn main (){
	fuzz_nohook!(|data: &[u8]| {
		if data.len() < 2000 {return;}
		set_global_data(data);
		let global_data = get_global_data();
		let GLOBAL_DATA = global_data.first_half;
		
		let ops_count = _to_u8(GLOBAL_DATA, 0) % 64 + 1;
		let vec_count = _to_u8(GLOBAL_DATA, 1) % 3 + 1;
		
		let mut vectors = Vec::with_capacity(vec_count as usize);
		
		for i in 0..vec_count {
			let constructor_choice = _to_u8(GLOBAL_DATA, 2 + i as usize) % 7;
			match constructor_choice {
				0 => {
					vectors.push(smallvec::SmallVec::<[CustomType1; 12]>::new());
				},
				1 => {
					let cap = _to_usize(GLOBAL_DATA, 10 + i as usize * 8);
					vectors.push(smallvec::SmallVec::<[CustomType1; 12]>::with_capacity(cap));
				},
				2 => {
					let elem_count = _to_usize(GLOBAL_DATA, 50 + i as usize * 8) % 65;
					let mut str_len = _to_u8(GLOBAL_DATA, 100 + i as usize) % 17;
					let elem_str = _to_str(GLOBAL_DATA, 120 + i as usize * 20, 120 + i as usize * 20 + str_len as usize);
					let elem = CustomType1(String::from(elem_str));
					vectors.push(smallvec::SmallVec::<[CustomType1; 12]>::from_elem(elem, elem_count));
				},
				3 => {
					let slice_len = _to_usize(GLOBAL_DATA, 200 + i as usize * 8) % 65;
					let mut items = Vec::new();
					for j in 0..slice_len {
						let str_len = _to_u8(GLOBAL_DATA, 250 + i as usize * 50 + j * 3) % 17;
						let item_str = _to_str(GLOBAL_DATA, 300 + i as usize * 100 + j * 20, 300 + i as usize * 100 + j * 20 + str_len as usize);
						items.push(CustomType1(String::from(item_str)));
					}
					vectors.push(smallvec::SmallVec::<[CustomType1; 12]>::from_vec(items));
				},
				4 => {
					let slice_len = _to_usize(GLOBAL_DATA, 600 + i as usize * 8) % 65;
					let mut items = Vec::new();
					for j in 0..slice_len {
						let str_len = _to_u8(GLOBAL_DATA, 650 + i as usize * 50 + j * 3) % 17;
						let item_str = _to_str(GLOBAL_DATA, 670 + i as usize * 100 + j * 20, 670 + i as usize * 100 + j * 20 + str_len as usize);
						items.push(CustomType1(String::from(item_str)));
					}
					vectors.push(smallvec::SmallVec::<[CustomType1; 12]>::from_iter(items.into_iter()));
				},
				5 => {
					let slice_len = _to_usize(GLOBAL_DATA, 800 + i as usize * 8) % 65;
					let mut items = Vec::new();
					for j in 0..slice_len {
						let str_len = _to_u8(GLOBAL_DATA, 850 + i as usize * 50 + j * 3) % 17;
						let item_str = _to_str(GLOBAL_DATA, 870 + i as usize * 100 + j * 20, 870 + i as usize * 100 + j * 20 + str_len as usize);
						items.push(CustomType1(String::from(item_str)));
					}
					vectors.push(smallvec::SmallVec::<[CustomType1; 12]>::from(&items[..]));
				},
				_ => {
					vectors.push(smallvec::SmallVec::<[CustomType1; 12]>::new());
				}
			}
		}
		
		for op in 0..ops_count {
			let vec_idx = _to_usize(GLOBAL_DATA, 1000 + op as usize * 50) % vec_count as usize;
			let operation = _to_u8(GLOBAL_DATA, 1001 + op as usize * 50) % 30;
			
			match operation {
				0 => {
					let new_len = _to_usize(GLOBAL_DATA, 1002 + op as usize * 50);
					let str_len = _to_u8(GLOBAL_DATA, 1010 + op as usize * 50) % 17;
					let value_str = _to_str(GLOBAL_DATA, 1011 + op as usize * 50, 1011 + op as usize * 50 + str_len as usize);
					let value = CustomType1(String::from(value_str));
					vectors[vec_idx].resize(new_len, value);
				},
				1 => {
					let str_len = _to_u8(GLOBAL_DATA, 1020 + op as usize * 50) % 17;
					let value_str = _to_str(GLOBAL_DATA, 1021 + op as usize * 50, 1021 + op as usize * 50 + str_len as usize);
					let value = CustomType1(String::from(value_str));
					vectors[vec_idx].push(value);
				},
				2 => {
					let popped = vectors[vec_idx].pop();
					if let Some(item) = popped {
						println!("{:?}", item);
					}
				},
				3 => {
					let index = _to_usize(GLOBAL_DATA, 1030 + op as usize * 50);
					if index < vectors[vec_idx].len() {
						let removed = vectors[vec_idx].remove(index);
						println!("{:?}", removed);
					}
				},
				4 => {
					let index = _to_usize(GLOBAL_DATA, 1035 + op as usize * 50);
					let str_len = _to_u8(GLOBAL_DATA, 1043 + op as usize * 50) % 17;
					let value_str = _to_str(GLOBAL_DATA, 1044 + op as usize * 50, 1044 + op as usize * 50 + str_len as usize);
					let value = CustomType1(String::from(value_str));
					if index <= vectors[vec_idx].len() {
						vectors[vec_idx].insert(index, value);
					}
				},
				5 => {
					let additional = _to_usize(GLOBAL_DATA, 1050 + op as usize * 50);
					vectors[vec_idx].reserve(additional);
				},
				6 => {
					let additional = _to_usize(GLOBAL_DATA, 1060 + op as usize * 50);
					vectors[vec_idx].reserve_exact(additional);
				},
				7 => {
					let len = _to_usize(GLOBAL_DATA, 1070 + op as usize * 50);
					vectors[vec_idx].truncate(len);
				},
				8 => {
					vectors[vec_idx].clear();
				},
				9 => {
					vectors[vec_idx].shrink_to_fit();
				},
				10 => {
					let index = _to_usize(GLOBAL_DATA, 1080 + op as usize * 50);
					if index < vectors[vec_idx].len() {
						let removed = vectors[vec_idx].swap_remove(index);
						println!("{:?}", removed);
					}
				},
				11 => {
					let slice = vectors[vec_idx].as_slice();
					for item in slice.iter() {
						println!("{:?}", *item);
					}
				},
				12 => {
					let mut_slice = vectors[vec_idx].as_mut_slice();
					for item in mut_slice.iter_mut() {
						println!("{:?}", *item);
					}
				},
				13 => {
					let capacity = vectors[vec_idx].capacity();
					println!("{:?}", capacity);
				},
				14 => {
					let length = vectors[vec_idx].len();
					println!("{:?}", length);
				},
				15 => {
					let is_empty = vectors[vec_idx].is_empty();
					println!("{:?}", is_empty);
				},
				16 => {
					let ptr = vectors[vec_idx].as_ptr();
					println!("{:?}", ptr);
				},
				17 => {
					let mut_ptr = vectors[vec_idx].as_mut_ptr();
					println!("{:?}", mut_ptr);
				},
				18 => {
					if vec_count > 1 {
						let other_idx = (_to_usize(GLOBAL_DATA, 1090 + op as usize * 50) % (vec_count - 1) as usize + 1) % vec_count as usize;
						if other_idx != vec_idx {
							let cmp_result = vectors[vec_idx].cmp(&vectors[other_idx]);
							println!("{:?}", cmp_result);
						}
					}
				},
				19 => {
					if vec_count > 1 {
						let other_idx = (_to_usize(GLOBAL_DATA, 1095 + op as usize * 50) % (vec_count - 1) as usize + 1) % vec_count as usize;
						if other_idx != vec_idx {
							let partial_cmp_result = vectors[vec_idx].partial_cmp(&vectors[other_idx]);
							println!("{:?}", partial_cmp_result);
						}
					}
				},
				20 => {
					if vec_count > 1 {
						let other_idx = (_to_usize(GLOBAL_DATA, 1097 + op as usize * 50) % (vec_count - 1) as usize + 1) % vec_count as usize;
						if other_idx != vec_idx {
							let eq_result = vectors[vec_idx].eq(&vectors[other_idx]);
							println!("{:?}", eq_result);
						}
					}
				},
				21 => {
					let start = _to_usize(GLOBAL_DATA, 1100 + op as usize * 50) % (vectors[vec_idx].len() + 1);
					let end = start + (_to_usize(GLOBAL_DATA, 1108 + op as usize * 50) % (vectors[vec_idx].len() - start + 1));
					let mut drain = vectors[vec_idx].drain(start..end);
					if let Some(item) = drain.next() {
						println!("{:?}", item);
					}
				},
				22 => {
					let cloned = vectors[vec_idx].clone();
					println!("{:?}", cloned.len());
				},
				23 => {
					let slice_len = _to_usize(GLOBAL_DATA, 1120 + op as usize * 50) % 65;
					let mut items = Vec::new();
					for j in 0..slice_len {
						let str_len = _to_u8(GLOBAL_DATA, 1130 + op as usize * 50 + j * 3) % 17;
						let item_str = _to_str(GLOBAL_DATA, 1140 + op as usize * 50 + j * 20, 1140 + op as usize * 50 + j * 20 + str_len as usize);
						items.push(CustomType1(String::from(item_str)));
					}
					vectors[vec_idx].extend(items.into_iter());
				},
				24 => {
					let index = _to_usize(GLOBAL_DATA, 1200 + op as usize * 50);
					let slice_len = _to_usize(GLOBAL_DATA, 1208 + op as usize * 50) % 65;
					let mut items = Vec::new();
					for j in 0..slice_len {
						let str_len = _to_u8(GLOBAL_DATA, 1216 + op as usize * 50 + j * 3) % 17;
						let item_str = _to_str(GLOBAL_DATA, 1220 + op as usize * 50 + j * 20, 1220 + op as usize * 50 + j * 20 + str_len as usize);
						items.push(CustomType1(String::from(item_str)));
					}
					if index <= vectors[vec_idx].len() {
						vectors[vec_idx].insert_many(index, items.into_iter());
					}
				},
				25 => {
					let slice_len = _to_usize(GLOBAL_DATA, 1300 + op as usize * 50) % 65;
					let mut items = Vec::new();
					for j in 0..slice_len {
						let str_len = _to_u8(GLOBAL_DATA, 1308 + op as usize * 50 + j * 3) % 17;
						let item_str = _to_str(GLOBAL_DATA, 1310 + op as usize * 50 + j * 20, 1310 + op as usize * 50 + j * 20 + str_len as usize);
						items.push(CustomType1(String::from(item_str)));
					}
					vectors[vec_idx].extend(items.into_iter());
				},
				26 => {
					if vec_count > 1 {
						let other_idx = (_to_usize(GLOBAL_DATA, 1400 + op as usize * 50) % (vec_count - 1) as usize + 1) % vec_count as usize;
						if other_idx != vec_idx {
							let (left, right) = vectors.split_at_mut(vec_idx.max(other_idx));
							if vec_idx < other_idx {
								left[vec_idx].append(&mut right[other_idx - vec_idx - 1]);
							} else {
								right[0].append(&mut left[other_idx]);
							}
						}
					}
				},
				27 => {
					let new_len = _to_usize(GLOBAL_DATA, 1410 + op as usize * 50);
					vectors[vec_idx].resize_with(new_len, || {
						let global_data = get_global_data();
						let GLOBAL_DATA = global_data.second_half;
						let str_len = _to_u8(GLOBAL_DATA, 10) % 17;
						let item_str = _to_str(GLOBAL_DATA, 11, 11 + str_len as usize);
						CustomType1(String::from(item_str))
					});
				},
				28 => {
					let additional = _to_usize(GLOBAL_DATA, 1420 + op as usize * 50);
					let _ = vectors[vec_idx].try_reserve(additional);
				},
				_ => {
					let additional = _to_usize(GLOBAL_DATA, 1430 + op as usize * 50);
					let _ = vectors[vec_idx].try_reserve_exact(additional);
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