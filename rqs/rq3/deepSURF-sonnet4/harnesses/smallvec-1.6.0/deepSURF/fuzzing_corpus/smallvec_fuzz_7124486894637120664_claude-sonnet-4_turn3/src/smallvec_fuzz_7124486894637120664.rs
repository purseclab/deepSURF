#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};
use std::borrow::{Borrow, BorrowMut};

#[derive(Clone, PartialEq, Debug)]
struct CustomType1(String);

impl CustomType1 {
    fn new(s: String) -> Self {
        CustomType1(s)
    }
}

fn main (){
	fuzz_nohook!(|data: &[u8]| {
		if data.len() < 3000 {return;}
		set_global_data(data);
		let global_data = get_global_data();
		let GLOBAL_DATA = global_data.first_half;
		
		let num_operations = _to_u8(GLOBAL_DATA, 0) % 64;
		
		for op_idx in 0..num_operations {
			let operation = _to_u8(GLOBAL_DATA, op_idx as usize + 1) % 10;
			
			match operation {
				0 => {
					let constructor_choice = _to_u8(GLOBAL_DATA, op_idx as usize + 100) % 4;
					let mut smallvec = match constructor_choice {
						0 => SmallVec::<[String; 16]>::new(),
						1 => SmallVec::<[String; 16]>::with_capacity(_to_usize(GLOBAL_DATA, op_idx as usize + 200)),
						2 => {
							let vec_size = _to_u8(GLOBAL_DATA, op_idx as usize + 300) % 64;
							let mut temp_vec = Vec::with_capacity(vec_size as usize);
							for i in 0..vec_size {
								let str_len = _to_u8(GLOBAL_DATA, op_idx as usize + 400 + i as usize) % 17;
								let start_idx = op_idx as usize + 500 + i as usize * 20;
								let temp_str = _to_str(GLOBAL_DATA, start_idx, start_idx + str_len as usize);
								temp_vec.push(String::from(temp_str));
							}
							SmallVec::<[String; 16]>::from_vec(temp_vec)
						},
						_ => {
							let elem_str_len = _to_u8(GLOBAL_DATA, op_idx as usize + 1500) % 17;
							let elem_start = op_idx as usize + 1600;
							let elem_str = _to_str(GLOBAL_DATA, elem_start, elem_start + elem_str_len as usize);
							let elem = String::from(elem_str);
							let count = _to_usize(GLOBAL_DATA, op_idx as usize + 1700);
							SmallVec::<[String; 16]>::from_elem(elem, count)
						}
					};
					
					let push_count = _to_u8(GLOBAL_DATA, op_idx as usize + 1800) % 32;
					for j in 0..push_count {
						let str_len = _to_u8(GLOBAL_DATA, op_idx as usize + 1900 + j as usize) % 17;
						let start_idx = op_idx as usize + 2000 + j as usize * 20;
						let temp_str = _to_str(GLOBAL_DATA, start_idx, start_idx + str_len as usize);
						smallvec.push(String::from(temp_str));
					}
					
					let mut smallvec2 = SmallVec::<[String; 16]>::new();
					let second_vec_size = _to_u8(GLOBAL_DATA, op_idx as usize + 2500) % 32;
					for k in 0..second_vec_size {
						let str_len = _to_u8(GLOBAL_DATA, op_idx as usize + 2600 + k as usize) % 17;
						let start_idx = op_idx as usize + 2700 + k as usize * 20;
						let temp_str = _to_str(GLOBAL_DATA, start_idx, start_idx + str_len as usize);
						smallvec2.push(String::from(temp_str));
					}
					
					let append_target_mut = &mut smallvec;
					let append_source_mut = &mut smallvec2;
					append_target_mut.append(append_source_mut);
					
					let slice_ref = smallvec.as_slice();
					println!("{:?}", slice_ref.len());
					
					let capacity_val = smallvec.capacity();
					let deref_val = &*smallvec.deref();
					println!("{:?}", deref_val.len());
					
					if !smallvec.is_empty() {
						let _removed = smallvec.pop();
					}
					
					let len_val = smallvec.len();
					println!("{:?}", len_val);
				},
				1 => {
					let mut sv = SmallVec::<[String; 32]>::new();
					let insert_count = _to_u8(GLOBAL_DATA, op_idx as usize + 100);
					for i in 0..insert_count {
						let str_len = _to_u8(GLOBAL_DATA, op_idx as usize + 200 + i as usize) % 17;
						let start_idx = op_idx as usize + 300 + i as usize * 20;
						let temp_str = _to_str(GLOBAL_DATA, start_idx, start_idx + str_len as usize);
						sv.push(String::from(temp_str));
					}
					
					let reserve_amount = _to_usize(GLOBAL_DATA, op_idx as usize + 800);
					sv.reserve(reserve_amount);
					
					let truncate_len = _to_usize(GLOBAL_DATA, op_idx as usize + 900);
					sv.truncate(truncate_len);
					
					let clone_sv = sv.clone();
					println!("{:?}", clone_sv.len());
					
					let spilled = sv.spilled();
					println!("{:?}", spilled);
				},
				2 => {
					let mut sv1 = SmallVec::<[String; 64]>::with_capacity(_to_usize(GLOBAL_DATA, op_idx as usize + 100));
					let mut sv2 = SmallVec::<[String; 64]>::new();
					
					let fill_count1 = _to_u8(GLOBAL_DATA, op_idx as usize + 200) % 64;
					for i in 0..fill_count1 {
						let str_len = _to_u8(GLOBAL_DATA, op_idx as usize + 300 + i as usize) % 17;
						let start_idx = op_idx as usize + 400 + i as usize * 20;
						let temp_str = _to_str(GLOBAL_DATA, start_idx, start_idx + str_len as usize);
						sv1.push(String::from(temp_str));
					}
					
					let fill_count2 = _to_u8(GLOBAL_DATA, op_idx as usize + 800) % 64;
					for i in 0..fill_count2 {
						let str_len = _to_u8(GLOBAL_DATA, op_idx as usize + 900 + i as usize) % 17;
						let start_idx = op_idx as usize + 1000 + i as usize * 20;
						let temp_str = _to_str(GLOBAL_DATA, start_idx, start_idx + str_len as usize);
						sv2.push(String::from(temp_str));
					}
					
					sv1.append(&mut sv2);
					
					let equal = sv1.eq(&sv1);
					println!("{:?}", equal);
					
					let as_ref = sv1.as_ref();
					println!("{:?}", as_ref.len());
					
					let as_mut_slice = sv1.as_mut_slice();
					println!("{:?}", as_mut_slice.len());
				},
				3 => {
					let elem_str_len = _to_u8(GLOBAL_DATA, op_idx as usize + 100) % 17;
					let elem_start = op_idx as usize + 200;
					let elem_str = _to_str(GLOBAL_DATA, elem_start, elem_start + elem_str_len as usize);
					let elem = String::from(elem_str);
					let count = _to_usize(GLOBAL_DATA, op_idx as usize + 300);
					
					let mut sv = SmallVec::<[String; 16]>::from_elem(elem, count);
					
					let drain_start = _to_usize(GLOBAL_DATA, op_idx as usize + 400);
					let drain_end = _to_usize(GLOBAL_DATA, op_idx as usize + 500);
					let drained = sv.drain(drain_start..drain_end);
					
					for item in drained {
						println!("{:?}", item.len());
					}
					
					if !sv.is_empty() {
						let iter = sv.into_iter();
						for item in iter {
							println!("{:?}", item.len());
						}
					}
				},
				4 => {
					let mut sv = SmallVec::<[String; 128]>::new();
					let extend_size = _to_u8(GLOBAL_DATA, op_idx as usize + 100) % 64;
					let mut extend_vec = Vec::new();
					for i in 0..extend_size {
						let str_len = _to_u8(GLOBAL_DATA, op_idx as usize + 200 + i as usize) % 17;
						let start_idx = op_idx as usize + 300 + i as usize * 20;
						let temp_str = _to_str(GLOBAL_DATA, start_idx, start_idx + str_len as usize);
						extend_vec.push(String::from(temp_str));
					}
					
					sv.extend(extend_vec);
					
					let insert_idx = _to_usize(GLOBAL_DATA, op_idx as usize + 1500);
					let insert_str_len = _to_u8(GLOBAL_DATA, op_idx as usize + 1600) % 17;
					let insert_start = op_idx as usize + 1700;
					let insert_str = _to_str(GLOBAL_DATA, insert_start, insert_start + insert_str_len as usize);
					sv.insert(insert_idx, String::from(insert_str));
					
					if !sv.is_empty() {
						let remove_idx = _to_usize(GLOBAL_DATA, op_idx as usize + 1800);
						let _removed = sv.remove(remove_idx);
					}
				},
				5 => {
					let mut sv = SmallVec::<[String; 256]>::new();
					let size = _to_u8(GLOBAL_DATA, op_idx as usize + 100) % 64;
					for i in 0..size {
						let str_len = _to_u8(GLOBAL_DATA, op_idx as usize + 200 + i as usize) % 17;
						let start_idx = op_idx as usize + 300 + i as usize * 20;
						let temp_str = _to_str(GLOBAL_DATA, start_idx, start_idx + str_len as usize);
						sv.push(String::from(temp_str));
					}
					
					let resize_len = _to_usize(GLOBAL_DATA, op_idx as usize + 800);
					let resize_str_len = _to_u8(GLOBAL_DATA, op_idx as usize + 900) % 17;
					let resize_start = op_idx as usize + 1000;
					let resize_str = _to_str(GLOBAL_DATA, resize_start, resize_start + resize_str_len as usize);
					sv.resize(resize_len, String::from(resize_str));
					
					sv.shrink_to_fit();
					sv.clear();
					
					let from_iter_size = _to_u8(GLOBAL_DATA, op_idx as usize + 1200) % 64;
					let iter_data: Vec<String> = (0..from_iter_size).map(|i| {
						let str_len = _to_u8(GLOBAL_DATA, op_idx as usize + 1300 + i as usize) % 17;
						let start_idx = op_idx as usize + 1400 + i as usize * 20;
						let temp_str = _to_str(GLOBAL_DATA, start_idx, start_idx + str_len as usize);
						String::from(temp_str)
					}).collect();
					
					let from_iter_sv = SmallVec::<[String; 256]>::from_iter(iter_data);
					println!("{:?}", from_iter_sv.len());
				},
				6 => {
					let mut sv1 = SmallVec::<[String; 32]>::new();
					let mut sv2 = SmallVec::<[String; 32]>::new();
					
					let size1 = _to_u8(GLOBAL_DATA, op_idx as usize + 100) % 32;
					for i in 0..size1 {
						let str_len = _to_u8(GLOBAL_DATA, op_idx as usize + 200 + i as usize) % 17;
						let start_idx = op_idx as usize + 300 + i as usize * 20;
						let temp_str = _to_str(GLOBAL_DATA, start_idx, start_idx + str_len as usize);
						sv1.push(String::from(temp_str));
					}
					
					let size2 = _to_u8(GLOBAL_DATA, op_idx as usize + 600) % 32;
					for i in 0..size2 {
						let str_len = _to_u8(GLOBAL_DATA, op_idx as usize + 700 + i as usize) % 17;
						let start_idx = op_idx as usize + 800 + i as usize * 20;
						let temp_str = _to_str(GLOBAL_DATA, start_idx, start_idx + str_len as usize);
						sv2.push(String::from(temp_str));
					}
					
					let equal = sv1.eq(&sv2);
					let partial_cmp_result = sv1.partial_cmp(&sv2);
					println!("{:?}", equal);
					println!("{:?}", partial_cmp_result);
					
					sv1.append(&mut sv2);
					
					let ptr = sv1.as_ptr();
					println!("{:?}", ptr);
					
					let mut_ptr = sv1.as_mut_ptr();
					println!("{:?}", mut_ptr);
				},
				7 => {
					let mut sv = SmallVec::<[String; 64]>::new();
					let size = _to_u8(GLOBAL_DATA, op_idx as usize + 100) % 64;
					for i in 0..size {
						let str_len = _to_u8(GLOBAL_DATA, op_idx as usize + 200 + i as usize) % 17;
						let start_idx = op_idx as usize + 300 + i as usize * 20;
						let temp_str = _to_str(GLOBAL_DATA, start_idx, start_idx + str_len as usize);
						sv.push(String::from(temp_str));
					}
					
					if !sv.is_empty() {
						let swap_remove_idx = _to_usize(GLOBAL_DATA, op_idx as usize + 800);
						let _removed = sv.swap_remove(swap_remove_idx);
					}
					
					let vec = sv.into_vec();
					println!("{:?}", vec.len());
				},
				8 => {
					let mut sv = SmallVec::<[String; 128]>::new();
					let size = _to_u8(GLOBAL_DATA, op_idx as usize + 100) % 64;
					for i in 0..size {
						let str_len = _to_u8(GLOBAL_DATA, op_idx as usize + 200 + i as usize) % 17;
						let start_idx = op_idx as usize + 300 + i as usize * 20;
						let temp_str = _to_str(GLOBAL_DATA, start_idx, start_idx + str_len as usize);
						sv.push(String::from(temp_str));
					}
					
					let reserve_exact_amount = _to_usize(GLOBAL_DATA, op_idx as usize + 800);
					sv.reserve_exact(reserve_exact_amount);
					
					let grow_amount = _to_usize(GLOBAL_DATA, op_idx as usize + 900);
					sv.grow(grow_amount);
					
					let try_reserve_amount = _to_usize(GLOBAL_DATA, op_idx as usize + 1000);
					let _try_result = sv.try_reserve(try_reserve_amount);
					
					let spilled = sv.spilled();
					println!("{:?}", spilled);
					
					let insert_many_idx = _to_usize(GLOBAL_DATA, op_idx as usize + 1100);
					let many_size = _to_u8(GLOBAL_DATA, op_idx as usize + 1200) % 32;
					let mut many_vec = Vec::new();
					for i in 0..many_size {
						let str_len = _to_u8(GLOBAL_DATA, op_idx as usize + 1300 + i as usize) % 17;
						let start_idx = op_idx as usize + 1400 + i as usize * 20;
						let temp_str = _to_str(GLOBAL_DATA, start_idx, start_idx + str_len as usize);
						many_vec.push(String::from(temp_str));
					}
					sv.insert_many(insert_many_idx, many_vec);
					
					let boxed_slice = sv.into_boxed_slice();
					println!("{:?}", boxed_slice.len());
				},
				_ => {
					let mut sv1 = SmallVec::<[String; 16]>::new();
					let mut sv2 = SmallVec::<[String; 16]>::new();
					
					let size1 = _to_u8(GLOBAL_DATA, op_idx as usize + 100) % 32;
					for i in 0..size1 {
						let str_len = _to_u8(GLOBAL_DATA, op_idx as usize + 200 + i as usize) % 17;
						let start_idx = op_idx as usize + 300 + i as usize * 20;
						let temp_str = _to_str(GLOBAL_DATA, start_idx, start_idx + str_len as usize);
						sv1.push(String::from(temp_str));
					}
					
					let size2 = _to_u8(GLOBAL_DATA, op_idx as usize + 600) % 32;
					for i in 0..size2 {
						let str_len = _to_u8(GLOBAL_DATA, op_idx as usize + 700 + i as usize) % 17;
						let start_idx = op_idx as usize + 800 + i as usize * 20;
						let temp_str = _to_str(GLOBAL_DATA, start_idx, start_idx + str_len as usize);
						sv2.push(String::from(temp_str));
					}
					
					sv1.append(&mut sv2);
					
					if !sv1.is_empty() {
						let index_val = _to_usize(GLOBAL_DATA, op_idx as usize + 1100);
						let indexed_ref = &sv1[index_val];
						println!("{:?}", indexed_ref.len());
					}
					
					let borrow_ref: &[String] = sv1.borrow();
					println!("{:?}", borrow_ref.len());
					
					let borrow_mut_ref: &mut [String] = sv1.borrow_mut();
					println!("{:?}", borrow_mut_ref.len());
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