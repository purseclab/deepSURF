#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};
use std::borrow::Borrow;

struct CustomType2(String);
struct CustomType3(String);
struct CustomType1(usize);

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

impl core::marker::Copy for CustomType1 {
}

impl std::fmt::Debug for CustomType1 {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "CustomType1({})", self.0)
	}
}

impl PartialEq for CustomType1 {
	fn eq(&self, other: &Self) -> bool {
		self.0 == other.0
	}
}

impl Eq for CustomType1 {
}

impl PartialOrd for CustomType1 {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		self.0.partial_cmp(&other.0)
	}
}

impl Ord for CustomType1 {
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		self.0.cmp(&other.0)
	}
}

fn main (){
	fuzz_nohook!(|data: &[u8]| {
		if data.len() < 2500 {return;}
		set_global_data(data);
		let global_data = get_global_data();
		let GLOBAL_DATA = global_data.first_half;
		
		let num_operations = _to_u8(GLOBAL_DATA, 0) % 65;
		
		for op_idx in 0..num_operations {
			let operation = _to_u8(GLOBAL_DATA, 1 + op_idx as usize) % 10;
			let base_offset = 66 + op_idx as usize * 35;
			
			match operation {
				0 => {
					let constructor_choice = _to_u8(GLOBAL_DATA, base_offset) % 5;
					let smallvec = match constructor_choice {
						0 => {
							SmallVec::<[CustomType1; 20]>::new()
						},
						1 => {
							let capacity = _to_usize(GLOBAL_DATA, base_offset + 1);
							SmallVec::<[CustomType1; 20]>::with_capacity(capacity)
						},
						2 => {
							let elem = CustomType1(_to_usize(GLOBAL_DATA, base_offset + 9));
							let count = _to_usize(GLOBAL_DATA, base_offset + 17);
							SmallVec::<[CustomType1; 20]>::from_elem(elem, count)
						},
						3 => {
							let mut vec = Vec::new();
							for i in 0..(_to_u8(GLOBAL_DATA, base_offset + 25) % 15) {
								vec.push(CustomType1(_to_usize(GLOBAL_DATA, base_offset + 26 + i as usize)));
							}
							SmallVec::<[CustomType1; 20]>::from_vec(vec)
						},
						_ => {
							let mut arr = [CustomType1(0); 20];
							for i in 0..20 {
								arr[i] = CustomType1(_to_usize(GLOBAL_DATA, base_offset + 1 + i));
							}
							SmallVec::<[CustomType1; 20]>::from_buf(arr)
						}
					};
					println!("{:?}", smallvec.len());
					
					let deref_slice = &*smallvec;
					if !deref_slice.is_empty() {
						let first_elem = &deref_slice[0];
						println!("{:?}", first_elem.0);
					}
				},
				1 => {
					let mut smallvec = SmallVec::<[CustomType1; 15]>::new();
					for i in 0..(_to_u8(GLOBAL_DATA, base_offset) % 25) {
						let item = CustomType1(_to_usize(GLOBAL_DATA, base_offset + 1 + i as usize));
						smallvec.push(item);
					}
					let slice_ref = smallvec.as_slice();
					if !slice_ref.is_empty() {
						let elem_ref = &slice_ref[0];
						println!("{:?}", elem_ref.0);
					}
					
					let mut_slice_ref = smallvec.as_mut_slice();
					if !mut_slice_ref.is_empty() {
						let elem_mut_ref = &mut mut_slice_ref[0];
						println!("{:?}", elem_mut_ref.0);
					}
					
					let borrow_slice: &[CustomType1] = smallvec.borrow();
					if !borrow_slice.is_empty() {
						let borrow_elem = &borrow_slice[0];
						println!("{:?}", borrow_elem.0);
					}
				},
				2 => {
					let mut smallvec = SmallVec::<[CustomType1; 12]>::new();
					for i in 0..(_to_u8(GLOBAL_DATA, base_offset) % 20) {
						smallvec.push(CustomType1(_to_usize(GLOBAL_DATA, base_offset + 1 + i as usize)));
					}
					if !smallvec.is_empty() {
						let index = _to_usize(GLOBAL_DATA, base_offset + 21) % smallvec.len();
						let result_ref = &smallvec[index];
						println!("{:?}", result_ref.0);
						
						let deref_elem = &smallvec.deref()[index];
						println!("{:?}", deref_elem.0);
					}
				},
				3 => {
					let mut smallvec1 = SmallVec::<[CustomType1; 16]>::new();
					let mut smallvec2 = SmallVec::<[CustomType1; 16]>::new();
					
					for i in 0..(_to_u8(GLOBAL_DATA, base_offset) % 10) {
						smallvec1.push(CustomType1(_to_usize(GLOBAL_DATA, base_offset + 1 + i as usize)));
					}
					for i in 0..(_to_u8(GLOBAL_DATA, base_offset + 11) % 10) {
						smallvec2.push(CustomType1(_to_usize(GLOBAL_DATA, base_offset + 12 + i as usize)));
					}
					
					let ordering = smallvec1.cmp(&smallvec2);
					println!("{:?}", ordering);
					
					let partial_ord = smallvec1.partial_cmp(&smallvec2);
					if let Some(ord) = partial_ord {
						println!("{:?}", ord);
					}
					
					let eq_result = smallvec1.eq(&smallvec2);
					println!("{:?}", eq_result);
				},
				4 => {
					let mut smallvec = SmallVec::<[CustomType1; 18]>::new();
					for i in 0..(_to_u8(GLOBAL_DATA, base_offset) % 30) {
						smallvec.push(CustomType1(_to_usize(GLOBAL_DATA, base_offset + 1 + i as usize)));
					}
					
					if !smallvec.is_empty() {
						let clone_vec = smallvec.clone();
						println!("{:?}", clone_vec.len());
						
						let popped = smallvec.pop();
						if let Some(item) = popped {
							println!("{:?}", item.0);
						}
						
						smallvec.clear();
						println!("{:?}", smallvec.is_empty());
					}
				},
				5 => {
					let mut smallvec = SmallVec::<[CustomType1; 25]>::new();
					let element = CustomType1(_to_usize(GLOBAL_DATA, base_offset + 8));
					
					for i in 0..(_to_u8(GLOBAL_DATA, base_offset + 16) % 20) {
						smallvec.push(CustomType1(_to_usize(GLOBAL_DATA, base_offset + 17 + i as usize)));
					}
					
					let insert_index = if smallvec.is_empty() { 0 } else { _to_usize(GLOBAL_DATA, base_offset) % (smallvec.len() + 1) };
					smallvec.insert(insert_index, element);
					
					let capacity = smallvec.capacity();
					println!("{:?}", capacity);
					
					smallvec.reserve(_to_usize(GLOBAL_DATA, base_offset + 9));
					println!("{:?}", smallvec.capacity());
				},
				6 => {
					let mut smallvec = SmallVec::<[CustomType1; 22]>::new();
					for i in 0..(_to_u8(GLOBAL_DATA, base_offset) % 15) {
						smallvec.push(CustomType1(_to_usize(GLOBAL_DATA, base_offset + 1 + i as usize)));
					}
					
					if !smallvec.is_empty() {
						let len = smallvec.len();
						let drain_start = _to_usize(GLOBAL_DATA, base_offset + 16) % len;
						let drain_end = std::cmp::min(drain_start + (_to_usize(GLOBAL_DATA, base_offset + 24) % (len - drain_start + 1)), len);
						
						let mut drain_iter = smallvec.drain(drain_start..drain_end);
						let first_drained = drain_iter.next();
						if let Some(item) = first_drained {
							println!("{:?}", item.0);
						}
						drop(drain_iter);
						
						let remaining_len = smallvec.len();
						println!("{:?}", remaining_len);
					}
				},
				7 => {
					let vec1 = vec![CustomType1(_to_usize(GLOBAL_DATA, base_offset)), CustomType1(_to_usize(GLOBAL_DATA, base_offset + 8))];
					let smallvec1 = SmallVec::<[CustomType1; 14]>::from_iter(vec1.into_iter());
					
					let slice = &[CustomType1(_to_usize(GLOBAL_DATA, base_offset + 16)), CustomType1(_to_usize(GLOBAL_DATA, base_offset + 24))];
					let smallvec2 = SmallVec::<[CustomType1; 14]>::from_slice(slice);
					
					let slice_ref = slice;
					let deref_elem = &slice_ref[0];
					println!("{:?}", deref_elem.0);
					
					let to_small_vec: SmallVec<[CustomType1; 14]> = slice_ref.to_smallvec();
					println!("{:?}", to_small_vec.len());
					
					let eq_result = smallvec1.eq(&smallvec2);
					println!("{:?}", eq_result);
				},
				8 => {
					let mut smallvec = SmallVec::<[CustomType1; 24]>::new();
					for i in 0..(_to_u8(GLOBAL_DATA, base_offset) % 20) {
						smallvec.push(CustomType1(_to_usize(GLOBAL_DATA, base_offset + 1 + i as usize)));
					}
					
					if !smallvec.is_empty() {
						let remove_index = _to_usize(GLOBAL_DATA, base_offset + 21) % smallvec.len();
						let removed = smallvec.remove(remove_index);
						println!("{:?}", removed.0);
					}
					
					smallvec.shrink_to_fit();
					println!("{:?}", smallvec.capacity());
					
					let truncate_len = _to_usize(GLOBAL_DATA, base_offset + 29);
					smallvec.truncate(truncate_len);
					println!("{:?}", smallvec.len());
				},
				_ => {
					let mut smallvec = SmallVec::<[CustomType1; 26]>::new();
					let slice_to_extend = &[CustomType1(_to_usize(GLOBAL_DATA, base_offset)), CustomType1(_to_usize(GLOBAL_DATA, base_offset + 8))];
					smallvec.extend_from_slice(slice_to_extend);
					
					let mut other_vec = SmallVec::<[CustomType1; 26]>::new();
					for i in 0..(_to_u8(GLOBAL_DATA, base_offset + 16) % 10) {
						other_vec.push(CustomType1(_to_usize(GLOBAL_DATA, base_offset + 17 + i as usize)));
					}
					
					smallvec.append(&mut other_vec);
					println!("{:?}", smallvec.len());
					println!("{:?}", other_vec.len());
					
					let as_ref_slice = smallvec.as_ref();
					if !as_ref_slice.is_empty() {
						let ref_elem = &as_ref_slice[0];
						println!("{:?}", ref_elem.0);
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