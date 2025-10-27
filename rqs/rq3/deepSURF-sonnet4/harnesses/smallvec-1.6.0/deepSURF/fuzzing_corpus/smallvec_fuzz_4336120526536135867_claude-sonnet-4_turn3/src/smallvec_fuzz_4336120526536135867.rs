#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

#[derive(Debug)]
struct CustomType0(String);

#[derive(Debug)]
struct CustomType1(usize);

impl core::marker::Copy for CustomType1 {
}

impl core::clone::Clone for CustomType1 {
	fn clone(&self) -> Self {
		let global_data = get_global_data();
		let GLOBAL_DATA = global_data.first_half;
		let custom_impl_num = _to_usize(GLOBAL_DATA, 10);
		let custom_impl_inst_num = self.0;
		let selector = (custom_impl_num + custom_impl_inst_num) % 3;
		if selector == 0{
			panic!("INTENTIONAL PANIC!");
		}
		let GLOBAL_DATA = match selector{
				1 => global_data.first_half,
				_ => global_data.second_half,
		};
		let t_4 = _to_usize(GLOBAL_DATA, 18);
		let t_5 = CustomType1(t_4);
		return t_5;
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

type CustomArray = [CustomType1; 16];

fn main (){
	fuzz_nohook!(|data: &[u8]| {
		if data.len() < 1200 {return;}
		set_global_data(data);
		let global_data = get_global_data();
		let GLOBAL_DATA = global_data.first_half;
		
		let num_operations = _to_u8(GLOBAL_DATA, 0) % 65;
		
		for op_idx in 0..num_operations {
			let operation = _to_u8(GLOBAL_DATA, (1 + op_idx) as usize) % 7;
			let base_offset = 100 + (op_idx as usize * 50);
			
			match operation {
				0 => {
					let vec_size = _to_u8(GLOBAL_DATA, base_offset) % 32;
					let mut vec_data = Vec::with_capacity(vec_size as usize);
					for i in 0..vec_size {
						let val = _to_usize(GLOBAL_DATA, base_offset + 1 + (i as usize * 8));
						vec_data.push(CustomType1(val));
					}
					
					let constructor_choice = _to_u8(GLOBAL_DATA, base_offset + 40) % 6;
					let mut sv: SmallVec<CustomArray> = match constructor_choice {
						0 => SmallVec::new(),
						1 => SmallVec::with_capacity(_to_usize(GLOBAL_DATA, base_offset + 41)),
						2 => SmallVec::from_vec(vec_data),
						3 => SmallVec::from_iter(vec_data.into_iter()),
						4 => {
							if !vec_data.is_empty() {
								SmallVec::from_elem(vec_data[0].clone(), _to_usize(GLOBAL_DATA, base_offset + 42) % 20)
							} else {
								SmallVec::new()
							}
						},
						_ => SmallVec::from_slice(&vec_data),
					};
					
					let slice_ref = sv.as_slice();
					let result_sv: SmallVec<CustomArray> = slice_ref.to_smallvec();
					println!("{:?}", result_sv.len());
					
					let deref_slice = &*sv;
					println!("{:?}", deref_slice.len());
					let another_sv: SmallVec<CustomArray> = deref_slice.to_smallvec();
					println!("{:?}", another_sv.capacity());
				},
				1 => {
					let initial_size = _to_u8(GLOBAL_DATA, base_offset) % 20;
					let mut sv: SmallVec<CustomArray> = SmallVec::new();
					
					for i in 0..initial_size {
						let val = _to_usize(GLOBAL_DATA, base_offset + 1 + (i as usize * 8));
						sv.push(CustomType1(val));
					}
					
					let as_slice_ref = sv.as_slice();
					let converted: SmallVec<CustomArray> = as_slice_ref.to_smallvec();
					println!("{:?}", converted.is_empty());
					
					let index_val = _to_usize(GLOBAL_DATA, base_offset + 30);
					if !sv.is_empty() {
						let idx = index_val % sv.len();
						let elem_ref = &sv[idx];
						println!("{:?}", elem_ref);
					}
					
					sv.clear();
					sv.shrink_to_fit();
					let empty_slice = sv.as_slice();
					let empty_sv: SmallVec<CustomArray> = empty_slice.to_smallvec();
					println!("{:?}", empty_sv.len());
				},
				2 => {
					let arr_size = _to_u8(GLOBAL_DATA, base_offset) % 16;
					let mut arr_data = Vec::new();
					for i in 0..arr_size {
						let val = _to_usize(GLOBAL_DATA, base_offset + 1 + (i as usize * 8));
						arr_data.push(CustomType1(val));
					}
					
					let sv1: SmallVec<CustomArray> = SmallVec::from_slice(&arr_data);
					let sv2: SmallVec<CustomArray> = SmallVec::from_slice(&arr_data);
					
					let cmp_result = sv1.cmp(&sv2);
					println!("{:?}", cmp_result);
					
					let partial_cmp_result = sv1.partial_cmp(&sv2);
					if let Some(ordering) = partial_cmp_result {
						println!("{:?}", ordering);
					}
					
					let eq_result = sv1.eq(&sv2);
					println!("{:?}", eq_result);
					
					let slice_from_sv1 = sv1.as_slice();
					let converted_sv: SmallVec<CustomArray> = slice_from_sv1.to_smallvec();
					println!("{:?}", converted_sv.spilled());
				},
				3 => {
					let capacity = _to_usize(GLOBAL_DATA, base_offset);
					let mut sv: SmallVec<CustomArray> = SmallVec::with_capacity(capacity);
					
					let push_count = _to_u8(GLOBAL_DATA, base_offset + 8) % 25;
					for i in 0..push_count {
						let val = _to_usize(GLOBAL_DATA, base_offset + 9 + (i as usize * 8));
						sv.push(CustomType1(val));
					}
					
					let mut_slice = sv.as_mut_slice();
					for elem in mut_slice.iter_mut() {
						println!("{:?}", elem);
					}
					
					let immut_slice = sv.as_slice();
					let target_sv: SmallVec<CustomArray> = immut_slice.to_smallvec();
					
					let remove_idx = _to_usize(GLOBAL_DATA, base_offset + 40);
					if !sv.is_empty() {
						let idx = remove_idx % sv.len();
						let removed = sv.remove(idx);
						println!("{:?}", removed);
					}
					
					let final_slice = sv.as_slice();
					let final_sv: SmallVec<CustomArray> = final_slice.to_smallvec();
					println!("{:?}", final_sv.capacity());
				},
				4 => {
					let size1 = _to_u8(GLOBAL_DATA, base_offset) % 15;
					let size2 = _to_u8(GLOBAL_DATA, base_offset + 1) % 15;
					
					let mut sv1: SmallVec<CustomArray> = SmallVec::new();
					let mut sv2: SmallVec<CustomArray> = SmallVec::new();
					
					for i in 0..size1 {
						let val = _to_usize(GLOBAL_DATA, base_offset + 2 + (i as usize * 8));
						sv1.push(CustomType1(val));
					}
					
					for i in 0..size2 {
						let val = _to_usize(GLOBAL_DATA, base_offset + 20 + (i as usize * 8));
						sv2.push(CustomType1(val));
					}
					
					sv1.append(&mut sv2);
					
					let drain_start = _to_usize(GLOBAL_DATA, base_offset + 35);
					let drain_end = _to_usize(GLOBAL_DATA, base_offset + 36);
					
					if !sv1.is_empty() {
						let start = drain_start % sv1.len();
						let end = start + (drain_end % (sv1.len() - start + 1));
						let end = std::cmp::min(end, sv1.len());
						
						let mut drain_iter = sv1.drain(start..end);
						while let Some(item) = drain_iter.next() {
							println!("{:?}", item);
						}
					}
					
					let remaining_slice = sv1.as_slice();
					let converted: SmallVec<CustomArray> = remaining_slice.to_smallvec();
					println!("{:?}", converted.len());
				},
				5 => {
					let extend_size = _to_u8(GLOBAL_DATA, base_offset) % 30;
					let mut extension_data = Vec::new();
					for i in 0..extend_size {
						let val = _to_usize(GLOBAL_DATA, base_offset + 1 + (i as usize * 8));
						extension_data.push(CustomType1(val));
					}
					
					let mut sv: SmallVec<CustomArray> = SmallVec::new();
					sv.extend(extension_data.iter().cloned());
					
					let reserve_amount = _to_usize(GLOBAL_DATA, base_offset + 35);
					sv.reserve(reserve_amount);
					
					let resize_len = _to_usize(GLOBAL_DATA, base_offset + 40) % 20;
					let fill_val = _to_usize(GLOBAL_DATA, base_offset + 41);
					sv.resize(resize_len, CustomType1(fill_val));
					
					let truncate_len = _to_usize(GLOBAL_DATA, base_offset + 42) % (sv.len() + 1);
					sv.truncate(truncate_len);
					
					let slice_view = sv.as_slice();
					let final_result: SmallVec<CustomArray> = slice_view.to_smallvec();
					println!("{:?}", final_result.inline_size());
				},
				_ => {
					let initial_capacity = _to_usize(GLOBAL_DATA, base_offset);
					let mut sv: SmallVec<CustomArray> = SmallVec::with_capacity(initial_capacity);
					
					let insert_count = _to_u8(GLOBAL_DATA, base_offset + 8) % 20;
					for i in 0..insert_count {
						let insert_idx = _to_usize(GLOBAL_DATA, base_offset + 9 + (i as usize * 8));
						let insert_val = _to_usize(GLOBAL_DATA, base_offset + 10 + (i as usize * 8));
						let idx = if sv.is_empty() { 0 } else { insert_idx % (sv.len() + 1) };
						sv.insert(idx, CustomType1(insert_val));
					}
					
					let clone_sv = sv.clone();
					let cloned_slice = clone_sv.as_slice();
					let cloned_result: SmallVec<CustomArray> = cloned_slice.to_smallvec();
					
					let swap_remove_idx = _to_usize(GLOBAL_DATA, base_offset + 35);
					if !sv.is_empty() {
						let idx = swap_remove_idx % sv.len();
						let swapped = sv.swap_remove(idx);
						println!("{:?}", swapped);
					}
					
					let into_vec_result = sv.into_vec();
					let vec_slice = into_vec_result.as_slice();
					let final_conversion: SmallVec<CustomArray> = vec_slice.to_smallvec();
					println!("{:?}", final_conversion.spilled());
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