#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

struct CustomType1(String);
struct CustomType3(String);

#[derive(Debug)]
struct ArrayType([u32; 12]);

impl core::clone::Clone for CustomType1 {
	fn clone(&self) -> Self {
		let global_data = get_global_data();
		let GLOBAL_DATA = global_data.first_half;
		let custom_impl_num = _to_usize(GLOBAL_DATA, 28);
		let custom_impl_inst_num = self.0.len();
		let selector = (custom_impl_num + custom_impl_inst_num) % 3;
		if selector == 0{
			panic!("INTENTIONAL PANIC!");
		}
		let GLOBAL_DATA = match selector{
				1 => global_data.first_half,
				_ => global_data.second_half,
		};
		let mut t_7 = _to_u8(GLOBAL_DATA, 36) % 17;
		let t_8 = _to_str(GLOBAL_DATA, 37, 37 + t_7 as usize);
		let t_9 = String::from(t_8);
		let t_10 = CustomType1(t_9);
		return t_10;
	}
}

impl core::cmp::PartialEq for CustomType1 {
	fn eq(&self, _: &Self) -> bool {
		let global_data = get_global_data();
		let GLOBAL_DATA = global_data.first_half;
		let custom_impl_num = _to_usize(GLOBAL_DATA, 0);
		let custom_impl_inst_num = self.0.len();
		let selector = (custom_impl_num + custom_impl_inst_num) % 3;
		if selector == 0{
			panic!("INTENTIONAL PANIC!");
		}
		let GLOBAL_DATA = match selector{
				1 => global_data.first_half,
				_ => global_data.second_half,
		};
		let t_0 = _to_bool(GLOBAL_DATA, 8);
		return t_0;
	}
}

fn main (){
	fuzz_nohook!(|data: &[u8]| {
		if data.len() < 3500 {return;}
		set_global_data(data);
		let global_data = get_global_data();
		let GLOBAL_DATA = global_data.first_half;
		
		let num_operations = _to_u8(GLOBAL_DATA, 0) % 65;
		for op_idx in 0..num_operations {
			let operation = _to_u8(GLOBAL_DATA, op_idx as usize + 1) % 10;
			
			match operation {
				0 => {
					let mut t_5 = _to_u8(GLOBAL_DATA, 50 + op_idx as usize) % 33;
					let mut vec_a = std::vec::Vec::with_capacity(32);
					for i in 0..t_5 {
						let mut str_len = _to_u8(GLOBAL_DATA, 100 + i as usize * 20) % 17;
						let str_start = 101 + i as usize * 20;
						let t_str = _to_str(GLOBAL_DATA, str_start, str_start + str_len as usize);
						let custom_item = CustomType1(String::from(t_str));
						vec_a.push(custom_item);
					}
					let sv1 = smallvec::SmallVec::<[CustomType1; 16]>::from_vec(vec_a);
					
					let mut t_6 = _to_u8(GLOBAL_DATA, 600 + op_idx as usize) % 33;
					let mut vec_b = std::vec::Vec::with_capacity(32);
					for i in 0..t_6 {
						let mut str_len = _to_u8(GLOBAL_DATA, 700 + i as usize * 20) % 17;
						let str_start = 701 + i as usize * 20;
						let t_str = _to_str(GLOBAL_DATA, str_start, str_start + str_len as usize);
						let custom_item = CustomType1(String::from(t_str));
						vec_b.push(custom_item);
					}
					let sv2 = smallvec::SmallVec::<[CustomType1; 16]>::from_vec(vec_b);
					
					let result = sv1.eq(&sv2);
					println!("{:?}", result);
				},
				1 => {
					let sv1 = smallvec::SmallVec::<[u32; 16]>::new();
					let sv2 = smallvec::SmallVec::<[u32; 16]>::with_capacity(_to_usize(GLOBAL_DATA, 1000 + op_idx as usize));
					let result = sv1.eq(&sv2);
					println!("{:?}", result);
				},
				2 => {
					let elem_count = _to_u8(GLOBAL_DATA, 1100 + op_idx as usize) % 33;
					let elem_val = _to_u32(GLOBAL_DATA, 1150 + op_idx as usize * 4);
					let sv1 = smallvec::SmallVec::<[u32; 16]>::from_elem(elem_val, elem_count as usize);
					let sv2 = smallvec::SmallVec::<[u32; 16]>::from_elem(elem_val, elem_count as usize);
					let result = sv1.eq(&sv2);
					println!("{:?}", result);
				},
				3 => {
					let slice_len = _to_u8(GLOBAL_DATA, 1300 + op_idx as usize) % 33;
					let mut temp_vec = Vec::with_capacity(slice_len as usize);
					for i in 0..slice_len {
						temp_vec.push(_to_u32(GLOBAL_DATA, 1400 + i as usize * 4));
					}
					let sv1 = smallvec::SmallVec::<[u32; 16]>::from_slice(&temp_vec);
					let sv2 = smallvec::SmallVec::<[u32; 16]>::from_slice(&temp_vec);
					let result = sv1.eq(&sv2);
					println!("{:?}", result);
				},
				4 => {
					let array_data = [_to_u32(GLOBAL_DATA, 1500 + op_idx as usize * 4); 16];
					let sv1 = smallvec::SmallVec::from_buf(array_data);
					let sv2 = smallvec::SmallVec::from_buf(array_data);
					let result = sv1.eq(&sv2);
					println!("{:?}", result);
				},
				5 => {
					let mut sv1 = smallvec::SmallVec::<[u32; 16]>::new();
					let mut sv2 = smallvec::SmallVec::<[u32; 16]>::new();
					let push_count = _to_u8(GLOBAL_DATA, 1600 + op_idx as usize) % 33;
					for i in 0..push_count {
						let val = _to_u32(GLOBAL_DATA, 1700 + i as usize * 4);
						sv1.push(val);
						sv2.push(val);
					}
					let capacity1 = sv1.capacity();
					let capacity2 = sv2.capacity();
					println!("{:?}", capacity1);
					println!("{:?}", capacity2);
					let result = sv1.eq(&sv2);
					println!("{:?}", result);
				},
				6 => {
					let mut sv1 = smallvec::SmallVec::<[CustomType1; 16]>::new();
					let mut sv2 = smallvec::SmallVec::<[CustomType1; 16]>::new();
					let item_count = _to_u8(GLOBAL_DATA, 1800 + op_idx as usize) % 17;
					for i in 0..item_count {
						let str_len = _to_u8(GLOBAL_DATA, 1900 + i as usize * 20) % 17;
						let str_start = 1901 + i as usize * 20;
						let t_str = _to_str(GLOBAL_DATA, str_start, str_start + str_len as usize);
						let item1 = CustomType1(String::from(t_str));
						let item2 = item1.clone();
						sv1.push(item1);
						sv2.push(item2);
					}
					let len1 = sv1.len();
					let len2 = sv2.len();
					println!("{:?}", len1);
					println!("{:?}", len2);
					let result = sv1.eq(&sv2);
					println!("{:?}", result);
					let slice1 = sv1.as_slice();
					let slice2 = sv2.as_slice();
					println!("{:?}", slice1.len());
					println!("{:?}", slice2.len());
				},
				7 => {
					let constructor_choice = _to_u8(GLOBAL_DATA, 2100 + op_idx as usize) % 5;
					let sv1 = match constructor_choice {
						0 => smallvec::SmallVec::<[i32; 16]>::new(),
						1 => smallvec::SmallVec::<[i32; 16]>::with_capacity(_to_usize(GLOBAL_DATA, 2200 + op_idx as usize)),
						2 => smallvec::SmallVec::<[i32; 16]>::from_elem(_to_i32(GLOBAL_DATA, 2300 + op_idx as usize * 4), _to_usize(GLOBAL_DATA, 2350 + op_idx as usize) % 33),
						3 => {
							let temp_vec = vec![_to_i32(GLOBAL_DATA, 2400 + op_idx as usize * 4); _to_u8(GLOBAL_DATA, 2450 + op_idx as usize) as usize % 33];
							smallvec::SmallVec::<[i32; 16]>::from_vec(temp_vec)
						},
						_ => {
							let array_val = _to_i32(GLOBAL_DATA, 2500 + op_idx as usize * 4);
							smallvec::SmallVec::from_buf([array_val; 16])
						},
					};
					let sv2 = sv1.clone();
					let result = sv1.eq(&sv2);
					println!("{:?}", result);
					let is_empty1 = sv1.is_empty();
					let is_empty2 = sv2.is_empty();
					println!("{:?}", is_empty1);
					println!("{:?}", is_empty2);
				},
				8 => {
					let mut sv1 = smallvec::SmallVec::<[ArrayType; 12]>::new();
					let mut sv2 = smallvec::SmallVec::<[ArrayType; 12]>::new();
					let array_count = _to_u8(GLOBAL_DATA, 2600 + op_idx as usize) % 17;
					for i in 0..array_count {
						let mut array_data = [0u32; 12];
						for j in 0..12 {
							array_data[j] = _to_u32(GLOBAL_DATA, 2700 + i as usize * 50 + j * 4);
						}
						let array_item = ArrayType(array_data);
						sv1.push(ArrayType(array_data));
						sv2.push(array_item);
					}
					let spilled1 = sv1.spilled();
					let spilled2 = sv2.spilled();
					println!("{:?}", spilled1);
					println!("{:?}", spilled2);
					println!("{:?}", sv1);
					println!("{:?}", sv2);
				},
				_ => {
					let vec_size = _to_u8(GLOBAL_DATA, 3000 + op_idx as usize) % 65;
					let mut vec_data = Vec::with_capacity(vec_size as usize);
					for i in 0..vec_size {
						vec_data.push(_to_u64(GLOBAL_DATA, 3100 + i as usize * 8));
					}
					let sv1 = smallvec::SmallVec::<[u64; 32]>::from_iter(vec_data.iter().cloned());
					let sv2 = smallvec::SmallVec::<[u64; 32]>::from_slice(&vec_data);
					let result = sv1.eq(&sv2);
					println!("{:?}", result);
					let partial_cmp_result = sv1.partial_cmp(&sv2);
					println!("{:?}", partial_cmp_result);
					let ordering_result = sv1.cmp(&sv2);
					println!("{:?}", ordering_result);
				},
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