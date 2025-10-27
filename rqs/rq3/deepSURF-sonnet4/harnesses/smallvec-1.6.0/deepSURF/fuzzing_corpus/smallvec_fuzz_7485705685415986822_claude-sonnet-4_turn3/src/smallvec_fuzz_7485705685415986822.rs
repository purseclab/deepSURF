#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct CustomType1(usize);

impl core::clone::Clone for CustomType1 {
	fn clone(&self) -> Self {
		let global_data = get_global_data();
		let GLOBAL_DATA = global_data.first_half;
		let custom_impl_num = _to_usize(GLOBAL_DATA, 555);
		let custom_impl_inst_num = self.0;
		let selector = (custom_impl_num + custom_impl_inst_num) % 3;
		if selector == 0{
			panic!("INTENTIONAL PANIC!");
		}
		let GLOBAL_DATA = match selector{
				1 => global_data.first_half,
				_ => global_data.second_half,
		};
		let t_136 = _to_usize(GLOBAL_DATA, 563);
		let t_137 = CustomType1(t_136);
		return t_137;
	}
}

impl core::marker::Copy for CustomType1 {
}

fn main (){
	fuzz_nohook!(|data: &[u8]| {
		if data.len() < 5000 {return;}
		set_global_data(data);
		let global_data = get_global_data();
		let GLOBAL_DATA = global_data.first_half;
		
		let operations_count = _to_u8(GLOBAL_DATA, 0) % 20;
		for op_index in 0..operations_count {
			let operation_selector = _to_u8(GLOBAL_DATA, 1 + op_index as usize) % 8;
			
			match operation_selector {
				0 => {
					let vec_count = _to_u8(GLOBAL_DATA, 50) % 64;
					let mut t_vec = std::vec::Vec::with_capacity(vec_count as usize);
					for i in 0..vec_count {
						let val = _to_usize(GLOBAL_DATA, 51 + (i as usize * 8));
						let item = CustomType1(val);
						t_vec.push(item);
					}
					
					let constructor_choice = _to_u8(GLOBAL_DATA, 400) % 7;
					let sv1 = match constructor_choice {
						0 => smallvec::SmallVec::<[CustomType1; 16]>::new(),
						1 => smallvec::SmallVec::<[CustomType1; 16]>::with_capacity(_to_usize(GLOBAL_DATA, 401)),
						2 => smallvec::SmallVec::<[CustomType1; 16]>::from_vec(t_vec.clone()),
						3 => smallvec::SmallVec::<[CustomType1; 16]>::from_slice(&t_vec[..]),
						4 => smallvec::SmallVec::<[CustomType1; 16]>::from_iter(t_vec.clone()),
						5 => smallvec::SmallVec::<[CustomType1; 16]>::from_elem(CustomType1(_to_usize(GLOBAL_DATA, 402)), _to_usize(GLOBAL_DATA, 410)),
						_ => smallvec::ToSmallVec::<[CustomType1; 16]>::to_smallvec(&t_vec[..]),
					};
					
					let vec_count2 = _to_u8(GLOBAL_DATA, 500) % 64;
					let mut t_vec2 = std::vec::Vec::with_capacity(vec_count2 as usize);
					for i in 0..vec_count2 {
						let val = _to_usize(GLOBAL_DATA, 501 + (i as usize * 8));
						let item = CustomType1(val);
						t_vec2.push(item);
					}
					
					let constructor_choice2 = _to_u8(GLOBAL_DATA, 900) % 5;
					let sv2 = match constructor_choice2 {
						0 => smallvec::SmallVec::<[CustomType1; 16]>::from_elem(CustomType1(_to_usize(GLOBAL_DATA, 901)), _to_usize(GLOBAL_DATA, 909)),
						1 => smallvec::ToSmallVec::<[CustomType1; 16]>::to_smallvec(&t_vec2[..]),
						2 => smallvec::SmallVec::<[CustomType1; 16]>::from(t_vec2),
						3 => smallvec::SmallVec::<[CustomType1; 16]>::from_slice(&t_vec[..]),
						_ => smallvec::SmallVec::<[CustomType1; 16]>::new(),
					};
					
					let sv1_iter = sv1.iter();
					let sv2_iter = sv2.iter();
					let result = sv1_iter.cmp(sv2_iter);
					println!("{:?}", result);
					
					if !sv1.is_empty() {
						let first_elem = &sv1[0];
						println!("{}", first_elem.0);
					}
					if !sv2.is_empty() {
						let last_elem = &sv2[sv2.len() - 1];
						println!("{}", last_elem.0);
					}
					
					let sv1_slice = sv1.as_slice();
					let sv2_slice = sv2.as_slice();
					for item in sv1_slice {
						println!("{}", item.0);
					}
					for item in sv2_slice {
						println!("{}", item.0);
					}
				}
				1 => {
					let mut sv = smallvec::SmallVec::<[CustomType1; 16]>::new();
					let element_count = _to_u8(GLOBAL_DATA, 1000) % 35;
					for i in 0..element_count {
						let val = _to_usize(GLOBAL_DATA, 1001 + (i as usize * 8));
						sv.push(CustomType1(val));
					}
					
					let idx = _to_usize(GLOBAL_DATA, 1200);
					if !sv.is_empty() {
						let elem_ref = &sv[idx % sv.len()];
						println!("{}", elem_ref.0);
						
						let slice_ref = sv.as_slice();
						for item in slice_ref {
							println!("{}", item.0);
						}
						
						let mut_slice_ref = sv.as_mut_slice();
						for item in mut_slice_ref {
							println!("{}", item.0);
						}
					}
					
					sv.truncate(_to_usize(GLOBAL_DATA, 1300));
					sv.clear();
					
					let capacity = sv.capacity();
					let len = sv.len();
					println!("Capacity: {}, Len: {}", capacity, len);
					
					let is_empty = sv.is_empty();
					let spilled = sv.spilled();
					println!("Empty: {}, Spilled: {}", is_empty, spilled);
				}
				2 => {
					let capacity = _to_usize(GLOBAL_DATA, 1400);
					let mut sv1 = smallvec::SmallVec::<[CustomType1; 16]>::with_capacity(capacity);
					let mut sv2 = smallvec::SmallVec::<[CustomType1; 16]>::new();
					
					let push_count = _to_u8(GLOBAL_DATA, 1500) % 40;
					for i in 0..push_count {
						let val1 = _to_usize(GLOBAL_DATA, 1501 + (i as usize * 8));
						let val2 = _to_usize(GLOBAL_DATA, 1701 + (i as usize * 8));
						sv1.push(CustomType1(val1));
						sv2.push(CustomType1(val2));
					}
					
					let sv1_iter = sv1.iter();
					let sv2_iter = sv2.iter();
					let ordering = sv1_iter.cmp(sv2_iter);
					println!("{:?}", ordering);
					
					sv1.extend_from_slice(sv2.as_slice());
					let sv1_iter2 = sv1.iter();
					let sv2_iter2 = sv2.iter();
					let partial_ordering = sv1_iter2.partial_cmp(sv2_iter2);
					if let Some(ord) = partial_ordering {
						println!("{:?}", ord);
					}
					
					sv1.reserve(_to_usize(GLOBAL_DATA, 1800));
					sv2.shrink_to_fit();
					
					let sv1_ref = sv1.deref();
					let sv2_ref = sv2.deref();
					for item in sv1_ref {
						println!("{}", item.0);
					}
					for item in sv2_ref {
						println!("{}", item.0);
					}
				}
				3 => {
					let elem_count = _to_u8(GLOBAL_DATA, 1900) % 45;
					let elem_val = _to_usize(GLOBAL_DATA, 1901);
					let sv_from_elem = smallvec::SmallVec::<[CustomType1; 16]>::from_elem(CustomType1(elem_val), elem_count as usize);
					
					let mut clone_sv = sv_from_elem.clone();
					clone_sv.reserve(_to_usize(GLOBAL_DATA, 2000));
					
					if !clone_sv.is_empty() {
						let popped = clone_sv.pop();
						if let Some(item) = popped {
							println!("{}", item.0);
						}
					}
					
					let sv_from_elem_iter = sv_from_elem.iter();
					let clone_sv_iter = clone_sv.iter();
					let final_ordering = sv_from_elem_iter.cmp(clone_sv_iter);
					println!("{:?}", final_ordering);
					
					if !clone_sv.is_empty() && clone_sv.len() > 2 {
						let removed = clone_sv.remove(1);
						println!("Removed: {}", removed.0);
					}
					
					let into_iter_sv = sv_from_elem.clone().into_iter();
					for item in into_iter_sv {
						println!("{}", item.0);
					}
				}
				4 => {
					let vec_size = _to_u8(GLOBAL_DATA, 2100) % 50;
					let mut temp_vec = Vec::with_capacity(vec_size as usize);
					for i in 0..vec_size {
						let val = _to_usize(GLOBAL_DATA, 2101 + (i as usize * 8));
						temp_vec.push(CustomType1(val));
					}
					
					let sv_from_vec = smallvec::SmallVec::<[CustomType1; 16]>::from_vec(temp_vec);
					let sv_default = smallvec::SmallVec::<[CustomType1; 16]>::default();
					
					let sv_from_vec_iter = sv_from_vec.iter();
					let sv_default_iter = sv_default.iter();
					let comparison = sv_from_vec_iter.cmp(sv_default_iter);
					println!("{:?}", comparison);
					
					let len = sv_from_vec.len();
					let capacity = sv_from_vec.capacity();
					println!("Len: {}, Capacity: {}", len, capacity);
					
					let spilled = sv_from_vec.spilled();
					println!("Spilled: {}", spilled);
					
					let partial_eq = sv_from_vec.eq(&sv_default);
					println!("Equal: {}", partial_eq);
					
					let into_vec = sv_from_vec.clone().into_vec();
					println!("Vec len: {}", into_vec.len());
				}
				5 => {
					let slice_size = _to_u8(GLOBAL_DATA, 2300) % 30;
					let mut slice_vec = Vec::new();
					for i in 0..slice_size {
						let val = _to_usize(GLOBAL_DATA, 2301 + (i as usize * 8));
						slice_vec.push(CustomType1(val));
					}
					
					let sv_from_slice = smallvec::SmallVec::<[CustomType1; 16]>::from_slice(&slice_vec[..]);
					let sv_to_smallvec = smallvec::ToSmallVec::<[CustomType1; 16]>::to_smallvec(&slice_vec[..]);
					
					let sv_from_slice_iter = sv_from_slice.iter();
					let sv_to_smallvec_iter = sv_to_smallvec.iter();
					let comp_result = sv_from_slice_iter.cmp(sv_to_smallvec_iter);
					println!("{:?}", comp_result);
					
					if !sv_from_slice.is_empty() && !sv_to_smallvec.is_empty() {
						let first_elem = &sv_from_slice[0];
						let last_elem = &sv_to_smallvec[sv_to_smallvec.len() - 1];
						println!("First: {}, Last: {}", first_elem.0, last_elem.0);
					}
					
					let into_vec = sv_from_slice.clone().into_vec();
					println!("Vec len: {}", into_vec.len());
					
					let into_boxed_slice = sv_to_smallvec.clone().into_boxed_slice();
					println!("Boxed slice len: {}", into_boxed_slice.len());
				}
				6 => {
					let mut sv = smallvec::SmallVec::<[CustomType1; 16]>::new();
					let ops_count = _to_u8(GLOBAL_DATA, 2500) % 30;
					for i in 0..ops_count {
						let val = _to_usize(GLOBAL_DATA, 2501 + (i as usize * 8));
						sv.push(CustomType1(val));
					}
					
					if !sv.is_empty() {
						let insert_idx = _to_usize(GLOBAL_DATA, 2700) % sv.len();
						let insert_val = CustomType1(_to_usize(GLOBAL_DATA, 2701));
						sv.insert(insert_idx, insert_val);
						
						if sv.len() > 1 {
							let swap_remove_idx = _to_usize(GLOBAL_DATA, 2702) % sv.len();
							let removed = sv.swap_remove(swap_remove_idx);
							println!("Swap removed: {}", removed.0);
						}
					}
					
					let resize_len = _to_usize(GLOBAL_DATA, 2800);
					let resize_val = CustomType1(_to_usize(GLOBAL_DATA, 2801));
					sv.resize(resize_len, resize_val);
					
					let sv_as_ref = sv.as_ref();
					for item in sv_as_ref {
						println!("{}", item.0);
					}
					
					sv.reserve_exact(_to_usize(GLOBAL_DATA, 2900));
					
					let sv_ptr = sv.as_ptr();
					println!("Ptr: {:?}", sv_ptr);
				}
				_ => {
					let drain_start = _to_usize(GLOBAL_DATA, 3000);
					let drain_end = _to_usize(GLOBAL_DATA, 3001);
					let mut sv = smallvec::SmallVec::<[CustomType1; 16]>::new();
					
					let fill_count = _to_u8(GLOBAL_DATA, 3100) % 50;
					for i in 0..fill_count {
						let val = _to_usize(GLOBAL_DATA, 3101 + (i as usize * 8));
						sv.push(CustomType1(val));
					}
					
					if !sv.is_empty() {
						let start = drain_start % sv.len();
						let end = if drain_end < sv.len() { drain_end } else { sv.len() };
						let end = if end <= start { start + 1 } else { end };
						
						let drain_iter = sv.drain(start..end);
						for item in drain_iter {
							println!("Drained: {}", item.0);
						}
					}
					
					let extend_count = _to_u8(GLOBAL_DATA, 3200) % 25;
					let mut extend_vec = Vec::new();
					for i in 0..extend_count {
						let val = _to_usize(GLOBAL_DATA, 3201 + (i as usize * 8));
						extend_vec.push(CustomType1(val));
					}
					sv.extend(extend_vec);
					
					let mut another_sv = smallvec::SmallVec::<[CustomType1; 16]>::new();
					let append_count = _to_u8(GLOBAL_DATA, 3300) % 20;
					for i in 0..append_count {
						let val = _to_usize(GLOBAL_DATA, 3301 + (i as usize * 8));
						another_sv.push(CustomType1(val));
					}
					sv.append(&mut another_sv);
					
					println!("Final sv len: {}", sv.len());
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