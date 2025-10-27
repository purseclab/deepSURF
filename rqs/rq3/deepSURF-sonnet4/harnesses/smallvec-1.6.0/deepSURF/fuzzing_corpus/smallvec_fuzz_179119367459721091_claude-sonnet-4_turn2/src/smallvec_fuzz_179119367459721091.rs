#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

struct CustomType1(usize);

#[derive(Debug)]
struct CustomType0(String);

impl core::clone::Clone for CustomType1 {
	
	fn clone(&self) -> Self {
		let global_data = get_global_data();
		let GLOBAL_DATA = global_data.first_half;
		let custom_impl_num = _to_usize(GLOBAL_DATA, 35);
		let custom_impl_inst_num = self.0;
		let selector = (custom_impl_num + custom_impl_inst_num) % 3;
		if selector == 0{
			panic!("INTENTIONAL PANIC!");
		}
		let GLOBAL_DATA = match selector{
				1 => global_data.first_half,
				_ => global_data.second_half,
		};
		let t_11 = _to_usize(GLOBAL_DATA, 43);
		let t_12 = CustomType1(t_11);
		return t_12;
	}
}

impl core::marker::Copy for CustomType1 {
}

impl Ord for CustomType1 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

impl PartialOrd for CustomType1 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for CustomType1 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for CustomType1 {}

fn main (){
	fuzz_nohook!(|data: &[u8]| {
		if data.len() < 1500 {return;}
		set_global_data(data);
		let global_data = get_global_data();
		let mut GLOBAL_DATA = global_data.first_half;
		
		let num_operations = _to_u8(GLOBAL_DATA, 0) % 64 + 1;
		let mut data_offset = 1;
		
		let constructor_choice = _to_u8(GLOBAL_DATA, data_offset) % 5;
		data_offset += 1;
		
		let mut smallvec1 = match constructor_choice {
			0 => {
				let capacity = _to_usize(GLOBAL_DATA, data_offset);
				data_offset += 8;
				smallvec::SmallVec::<[CustomType1; 16]>::with_capacity(capacity)
			},
			1 => {
				let vec_size = _to_u8(GLOBAL_DATA, data_offset) % 32;
				data_offset += 1;
				let mut vec = Vec::with_capacity(vec_size as usize);
				for _ in 0..vec_size {
					let val = _to_usize(GLOBAL_DATA, data_offset);
					data_offset += 8;
					vec.push(CustomType1(val));
				}
				smallvec::SmallVec::from_vec(vec)
			},
			2 => {
				let slice_size = _to_u8(GLOBAL_DATA, data_offset) % 32;
				data_offset += 1;
				let mut vec = Vec::with_capacity(slice_size as usize);
				for _ in 0..slice_size {
					let val = _to_usize(GLOBAL_DATA, data_offset);
					data_offset += 8;
					vec.push(CustomType1(val));
				}
				smallvec::SmallVec::from_slice(&vec[..])
			},
			3 => {
				let elem_val = _to_usize(GLOBAL_DATA, data_offset);
				data_offset += 8;
				let count = _to_usize(GLOBAL_DATA, data_offset) % 20;
				data_offset += 8;
				smallvec::SmallVec::from_elem(CustomType1(elem_val), count)
			},
			_ => {
				smallvec::SmallVec::new()
			}
		};
		
		let second_constructor_choice = _to_u8(GLOBAL_DATA, data_offset) % 3;
		data_offset += 1;
		
		let mut smallvec2 = match second_constructor_choice {
			0 => {
				let elem_val = _to_usize(GLOBAL_DATA, data_offset);
				data_offset += 8;
				let count = _to_usize(GLOBAL_DATA, data_offset) % 20;
				data_offset += 8;
				smallvec::SmallVec::from_elem(CustomType1(elem_val), count)
			},
			1 => {
				let iter_size = _to_u8(GLOBAL_DATA, data_offset) % 32;
				data_offset += 1;
				let mut vec = Vec::with_capacity(iter_size as usize);
				for _ in 0..iter_size {
					let val = _to_usize(GLOBAL_DATA, data_offset);
					data_offset += 8;
					vec.push(CustomType1(val));
				}
				smallvec::SmallVec::from_iter(vec.into_iter())
			},
			_ => {
				smallvec::SmallVec::new()
			}
		};
		
		for i in 0..num_operations {
			if data_offset + 16 >= GLOBAL_DATA.len() {
				GLOBAL_DATA = global_data.second_half;
				data_offset = 0;
			}
			
			let operation = _to_u8(GLOBAL_DATA, data_offset) % 30;
			data_offset += 1;
			
			match operation {
				0 => {
					let slice_size = _to_u8(GLOBAL_DATA, data_offset) % 32;
					data_offset += 1;
					let mut vec = Vec::with_capacity(slice_size as usize);
					for _ in 0..slice_size {
						let val = _to_usize(GLOBAL_DATA, data_offset);
						data_offset += 8;
						vec.push(CustomType1(val));
					}
					smallvec1.extend_from_slice(&vec[..]);
				},
				1 => {
					let val = _to_usize(GLOBAL_DATA, data_offset);
					data_offset += 8;
					smallvec1.push(CustomType1(val));
				},
				2 => {
					if let Some(item) = smallvec1.pop() {
						println!("{:?}", item.0);
					}
				},
				3 => {
					let index = _to_usize(GLOBAL_DATA, data_offset);
					data_offset += 8;
					let val = _to_usize(GLOBAL_DATA, data_offset);
					data_offset += 8;
					if index < smallvec1.len() {
						smallvec1.insert(index, CustomType1(val));
					}
				},
				4 => {
					let index = _to_usize(GLOBAL_DATA, data_offset);
					data_offset += 8;
					if index < smallvec1.len() {
						let removed = smallvec1.remove(index);
						println!("{:?}", removed.0);
					}
				},
				5 => {
					let len = _to_usize(GLOBAL_DATA, data_offset);
					data_offset += 8;
					smallvec1.truncate(len);
				},
				6 => {
					let additional = _to_usize(GLOBAL_DATA, data_offset);
					data_offset += 8;
					smallvec1.reserve(additional);
				},
				7 => {
					smallvec1.shrink_to_fit();
				},
				8 => {
					let slice = smallvec1.as_slice();
					if !slice.is_empty() {
						println!("{:?}", slice[0].0);
					}
				},
				9 => {
					let index = _to_usize(GLOBAL_DATA, data_offset);
					data_offset += 8;
					if index < smallvec1.len() {
						let swapped = smallvec1.swap_remove(index);
						println!("{:?}", swapped.0);
					}
				},
				10 => {
					smallvec1.clear();
				},
				11 => {
					smallvec1.append(&mut smallvec2);
				},
				12 => {
					let cmp_result = smallvec1.cmp(&smallvec2);
					println!("{:?}", cmp_result);
				},
				13 => {
					if let Some(partial_cmp) = smallvec1.partial_cmp(&smallvec2) {
						println!("{:?}", partial_cmp);
					}
				},
				14 => {
					let eq_result = smallvec1.eq(&smallvec2);
					println!("{:?}", eq_result);
				},
				15 => {
					let len = smallvec1.len();
					println!("{:?}", len);
				},
				16 => {
					let capacity = smallvec1.capacity();
					println!("{:?}", capacity);
				},
				17 => {
					let is_empty = smallvec1.is_empty();
					println!("{:?}", is_empty);
				},
				18 => {
					let spilled = smallvec1.spilled();
					println!("{:?}", spilled);
				},
				19 => {
					let range_start = _to_usize(GLOBAL_DATA, data_offset);
					data_offset += 8;
					let range_end = _to_usize(GLOBAL_DATA, data_offset);
					data_offset += 8;
					if range_start <= range_end && range_end <= smallvec1.len() {
						let mut drain = smallvec1.drain(range_start..range_end);
						while let Some(item) = drain.next() {
							println!("{:?}", item.0);
						}
					}
				},
				20 => {
					let cloned = smallvec1.clone();
					println!("{:?}", cloned.len());
				},
				21 => {
					let slice = smallvec1.as_mut_slice();
					if !slice.is_empty() {
						println!("{:?}", slice[0].0);
					}
				},
				22 => {
					let new_len = _to_usize(GLOBAL_DATA, data_offset);
					data_offset += 8;
					let val = _to_usize(GLOBAL_DATA, data_offset);
					data_offset += 8;
					smallvec1.resize(new_len, CustomType1(val));
				},
				23 => {
					let ptr = smallvec1.as_ptr();
					if !smallvec1.is_empty() {
						println!("{:?}", ptr);
					}
				},
				24 => {
					let mut_ptr = smallvec1.as_mut_ptr();
					if !smallvec1.is_empty() {
						println!("{:?}", mut_ptr);
					}
				},
				25 => {
					let additional = _to_usize(GLOBAL_DATA, data_offset);
					data_offset += 8;
					smallvec1.reserve_exact(additional);
				},
				26 => {
					smallvec1.dedup();
				},
				27 => {
					let iter_size = _to_u8(GLOBAL_DATA, data_offset) % 32;
					data_offset += 1;
					let mut vec = Vec::with_capacity(iter_size as usize);
					for _ in 0..iter_size {
						let val = _to_usize(GLOBAL_DATA, data_offset);
						data_offset += 8;
						vec.push(CustomType1(val));
					}
					smallvec1.extend(vec.into_iter());
				},
				28 => {
					let index = _to_usize(GLOBAL_DATA, data_offset);
					data_offset += 8;
					let slice_size = _to_u8(GLOBAL_DATA, data_offset) % 32;
					data_offset += 1;
					let mut vec = Vec::with_capacity(slice_size as usize);
					for _ in 0..slice_size {
						let val = _to_usize(GLOBAL_DATA, data_offset);
						data_offset += 8;
						vec.push(CustomType1(val));
					}
					if index <= smallvec1.len() {
						smallvec1.insert_from_slice(index, &vec[..]);
					}
				},
				29 => {
					let vec = smallvec1.clone().into_vec();
					println!("{:?}", vec.len());
				},
				_ => {}
			}
		}
		
		let deref_slice = &*smallvec1;
		if !deref_slice.is_empty() {
			println!("{:?}", deref_slice[0].0);
		}
		
		let mut_slice = smallvec1.as_mut_slice();
		if !mut_slice.is_empty() {
			println!("{:?}", mut_slice[0].0);
		}
		
		let into_iter = smallvec2.into_iter();
		for item in into_iter {
			println!("{:?}", item.0);
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