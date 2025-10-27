#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

struct CustomType1(String);
struct CustomType0(String);

fn main (){
	fuzz_nohook!(|data: &[u8]| {
		if data.len() < 120 {return;}
		set_global_data(data);
		let global_data = get_global_data();
		let GLOBAL_DATA = global_data.first_half;
		
		let num_operations = _to_u8(GLOBAL_DATA, 0) % 65;
		for i in 0..num_operations {
			let op_index = (i as usize + 1) * 3;
			if op_index + 8 >= GLOBAL_DATA.len() { break; }
			
			let operation = _to_u8(GLOBAL_DATA, op_index) % 6;
			match operation {
				0 => {
					let constructor_choice = _to_u8(GLOBAL_DATA, op_index + 1) % 10;
					let smallvec = match constructor_choice {
						0 => SmallVec::<[u32; 16]>::new(),
						1 => {
							let cap = _to_usize(GLOBAL_DATA, op_index + 2);
							SmallVec::<[u32; 16]>::with_capacity(cap)
						},
						2 => {
							let elem = _to_u32(GLOBAL_DATA, op_index + 2);
							let count = _to_usize(GLOBAL_DATA, op_index + 6);
							SmallVec::<[u32; 16]>::from_elem(elem, count)
						},
						3 => {
							let slice = &[_to_u32(GLOBAL_DATA, op_index + 2), _to_u32(GLOBAL_DATA, op_index + 6)];
							SmallVec::<[u32; 16]>::from_slice(slice)
						},
						4 => {
							let vec = vec![_to_u32(GLOBAL_DATA, op_index + 2), _to_u32(GLOBAL_DATA, op_index + 6)];
							SmallVec::<[u32; 16]>::from_vec(vec)
						},
						5 => {
							let arr = [_to_u32(GLOBAL_DATA, op_index + 2), _to_u32(GLOBAL_DATA, op_index + 6), 
									   _to_u32(GLOBAL_DATA, op_index + 7), 0u32, 0u32, 0u32, 0u32, 0u32, 
									   0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32];
							SmallVec::<[u32; 16]>::from_buf(arr)
						},
						6 => {
							let arr = [_to_u32(GLOBAL_DATA, op_index + 2), _to_u32(GLOBAL_DATA, op_index + 6), 
									   _to_u32(GLOBAL_DATA, op_index + 7), 0u32, 0u32, 0u32, 0u32, 0u32, 
									   0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32];
							let len = _to_usize(GLOBAL_DATA, op_index + 3);
							SmallVec::<[u32; 16]>::from_buf_and_len(arr, len)
						},
						7 => {
							let slice = &[_to_u32(GLOBAL_DATA, op_index + 2), _to_u32(GLOBAL_DATA, op_index + 6)];
							slice.to_smallvec()
						},
						8 => {
							let iter = vec![_to_u32(GLOBAL_DATA, op_index + 2), _to_u32(GLOBAL_DATA, op_index + 6)].into_iter();
							SmallVec::<[u32; 16]>::from_iter(iter)
						},
						_ => SmallVec::<[u32; 16]>::new(),
					};
					
					let cloned = smallvec.clone();
					println!("{:?}", cloned);
					
					let ref_sv = &smallvec;
					let cloned_ref = ref_sv.clone();
					println!("{:?}", cloned_ref);
					
					if !smallvec.is_empty() {
						let element_ref = &smallvec[0];
						println!("{}", *element_ref);
					}
					
					let as_slice_ref = smallvec.as_slice();
					for elem in as_slice_ref {
						println!("{}", *elem);
					}
				},
				1 => {
					let mut smallvec = SmallVec::<[i16; 24]>::new();
					let push_count = _to_u8(GLOBAL_DATA, op_index + 1) % 10;
					for j in 0..push_count {
						let val = _to_i16(GLOBAL_DATA, op_index + 2 + (j as usize * 2));
						smallvec.push(val);
					}
					
					let capacity = smallvec.capacity();
					println!("{}", capacity);
					
					let len = smallvec.len();
					println!("{}", len);
					
					let is_empty = smallvec.is_empty();
					println!("{}", is_empty);
					
					let as_slice_ref = smallvec.as_slice();
					println!("{:?}", as_slice_ref);
					
					let ref_sv = &smallvec;
					let cloned = ref_sv.clone();
					println!("{:?}", cloned);
					
					if !smallvec.is_empty() {
						let element_ref = &smallvec[0];
						println!("{}", *element_ref);
					}
					
					if smallvec.len() > 1 {
						let element_ref = &smallvec[smallvec.len() - 1];
						println!("{}", *element_ref);
					}
					
					let as_ptr = smallvec.as_ptr();
					println!("{:?}", as_ptr);
					
					let mut_as_mut_ptr = smallvec.as_mut_ptr();
					println!("{:?}", mut_as_mut_ptr);
				},
				2 => {
					let mut smallvec1 = SmallVec::<[u8; 32]>::new();
					let mut smallvec2 = SmallVec::<[u8; 32]>::new();
					
					let count1 = _to_u8(GLOBAL_DATA, op_index + 1) % 5;
					for j in 0..count1 {
						smallvec1.push(_to_u8(GLOBAL_DATA, op_index + 2 + j as usize));
					}
					
					let count2 = _to_u8(GLOBAL_DATA, op_index + 6) % 5;
					for j in 0..count2 {
						smallvec2.push(_to_u8(GLOBAL_DATA, op_index + 7 + j as usize));
					}
					
					let ordering = smallvec1.cmp(&smallvec2);
					println!("{:?}", ordering);
					
					let partial_ordering = smallvec1.partial_cmp(&smallvec2);
					if let Some(ord) = partial_ordering {
						println!("{:?}", ord);
					}
					
					let are_equal = smallvec1.eq(&smallvec2);
					println!("{}", are_equal);
					
					let ref_sv1 = &smallvec1;
					let cloned1 = ref_sv1.clone();
					println!("{:?}", cloned1);
					
					let ref_sv2 = &smallvec2;
					let cloned2 = ref_sv2.clone();
					println!("{:?}", cloned2);
					
					if !smallvec1.is_empty() {
						let element_ref = &smallvec1[0];
						println!("{}", *element_ref);
					}
					
					if !smallvec2.is_empty() {
						let element_ref = &smallvec2[0];
						println!("{}", *element_ref);
					}
					
					smallvec1.append(&mut smallvec2);
					
					let spilled = smallvec1.spilled();
					println!("{}", spilled);
				},
				3 => {
					let mut smallvec = SmallVec::<[f32; 12]>::new();
					let count = _to_u8(GLOBAL_DATA, op_index + 1) % 8;
					for j in 0..count {
						smallvec.push(_to_f32(GLOBAL_DATA, op_index + 2 + (j as usize * 4)));
					}
					
					if !smallvec.is_empty() {
						let index = _to_usize(GLOBAL_DATA, op_index + 2);
						if index < smallvec.len() {
							let indexed_ref = &smallvec[index];
							println!("{}", *indexed_ref);
						}
					}
					
					let as_mut_slice_ref = smallvec.as_mut_slice();
					println!("{:?}", as_mut_slice_ref);
					
					let ref_sv = &smallvec;
					let cloned = ref_sv.clone();
					println!("{:?}", cloned);
					
					let into_vec = smallvec.into_vec();
					let vec_ref = &into_vec;
					for elem in vec_ref {
						println!("{}", *elem);
					}
				},
				4 => {
					let mut smallvec = SmallVec::<[String; 16]>::new();
					let str_count = _to_u8(GLOBAL_DATA, op_index + 1) % 3;
					for j in 0..str_count {
						let start = op_index + 2 + (j as usize * 6);
						let len = (_to_u8(GLOBAL_DATA, start) % 5) + 1;
						if start + (len as usize) < GLOBAL_DATA.len() {
							let str_val = _to_str(GLOBAL_DATA, start, start + len as usize);
							smallvec.push(String::from(str_val));
						}
					}
					
					let drain_start = _to_usize(GLOBAL_DATA, op_index + 2);
					let drain_end = _to_usize(GLOBAL_DATA, op_index + 6);
					let drain_range = drain_start..drain_end;
					let drained = smallvec.drain(drain_range);
					for item in drained {
						println!("{}", item);
					}
					
					let ref_sv = &smallvec;
					let cloned = ref_sv.clone();
					println!("{:?}", cloned);
					
					if !smallvec.is_empty() {
						let element_ref = &smallvec[0];
						println!("{}", *element_ref);
					}
					
					smallvec.clear();
					
					let is_empty_after_clear = smallvec.is_empty();
					println!("{}", is_empty_after_clear);
				},
				5 => {
					let iter_vals = vec![_to_u64(GLOBAL_DATA, op_index + 1), 
										 _to_u64(GLOBAL_DATA, op_index + 5)];
					let smallvec = SmallVec::<[u64; 20]>::from_iter(iter_vals.into_iter());
					
					let into_iter = smallvec.into_iter();
					let into_iter_ref = &into_iter;
					let into_iter_cloned = into_iter_ref.clone();
					
					let slice_ref = into_iter_cloned.as_slice();
					println!("{:?}", slice_ref);
					
					for item in slice_ref {
						println!("{}", *item);
					}
					
					for item in into_iter {
						println!("{}", item);
					}
				},
				_ => {
					let mut smallvec = SmallVec::<[bool; 64]>::new();
					let bool_count = _to_u8(GLOBAL_DATA, op_index + 1) % 10;
					for j in 0..bool_count {
						let bool_val = _to_bool(GLOBAL_DATA, op_index + 2 + j as usize);
						smallvec.push(bool_val);
					}
					
					let ref_sv = &smallvec;
					let cloned = ref_sv.clone();
					println!("{:?}", cloned);
					
					if !smallvec.is_empty() {
						let element_ref = &smallvec[0];
						println!("{}", *element_ref);
					}
					
					let inline_size = smallvec.inline_size();
					println!("{}", inline_size);
					
					smallvec.shrink_to_fit();
					
					let capacity_after_shrink = smallvec.capacity();
					println!("{}", capacity_after_shrink);
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