#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

struct CustomType1(String);

fn main (){
	fuzz_nohook!(|data: &[u8]| {
		if data.len() < 120 {return;}
		set_global_data(data);
		let global_data = get_global_data();
		let GLOBAL_DATA = global_data.first_half;
		
		let num_operations = _to_u8(GLOBAL_DATA, 0) % 65;
		
		for op_idx in 0..num_operations {
			let base_offset = 1 + (op_idx as usize * 15);
			if base_offset + 14 >= GLOBAL_DATA.len() { break; }
			
			let operation = _to_u8(GLOBAL_DATA, base_offset) % 8;
			
			match operation {
				0 => {
					let constructor_choice = _to_u8(GLOBAL_DATA, base_offset + 1) % 7;
					let mut sv = match constructor_choice {
						0 => SmallVec::<[u32; 12]>::new(),
						1 => {
							let cap = _to_usize(GLOBAL_DATA, base_offset + 2);
							SmallVec::<[u32; 12]>::with_capacity(cap)
						},
						2 => {
							let size = _to_u8(GLOBAL_DATA, base_offset + 3) % 20;
							let mut vec = Vec::new();
							for i in 0..size {
								vec.push(i as u32);
							}
							SmallVec::<[u32; 12]>::from_vec(vec)
						},
						3 => {
							let arr = [42u32; 12];
							SmallVec::from_buf(arr)
						},
						4 => {
							let size = _to_u8(GLOBAL_DATA, base_offset + 4) % 20;
							let mut slice_data = vec![99u32; size as usize];
							SmallVec::<[u32; 12]>::from_slice(&slice_data)
						},
						5 => {
							let elem = _to_u32(GLOBAL_DATA, base_offset + 5);
							let count = _to_usize(GLOBAL_DATA, base_offset + 9);
							SmallVec::<[u32; 12]>::from_elem(elem, count)
						},
						_ => {
							let iter_data = vec![1u32, 2u32, 3u32];
							SmallVec::<[u32; 12]>::from_iter(iter_data)
						}
					};
					
					let item = _to_u32(GLOBAL_DATA, base_offset + 13);
					sv.push(item);
					let _len = sv.len();
					println!("{:?}", _len);
					
					if sv.len() > 0 {
						let idx = _to_usize(GLOBAL_DATA, base_offset + 10);
						if idx < sv.len() {
							let _item_ref = &sv[idx];
							println!("{:?}", *_item_ref);
						}
						let _popped = sv.pop();
					}
					
					let result = sv.into_inner();
					match result {
						Ok(arr) => {
							println!("Got array back");
						},
						Err(sv_back) => {
							let _vec = sv_back.into_vec();
						}
					}
				},
				1 => {
					let mut sv1 = SmallVec::<[i32; 16]>::new();
					let mut sv2 = SmallVec::<[i32; 16]>::new();
					
					let val1 = _to_i32(GLOBAL_DATA, base_offset + 1);
					let val2 = _to_i32(GLOBAL_DATA, base_offset + 5);
					sv1.push(val1);
					sv2.push(val2);
					
					let eq_result = sv1.eq(&sv2);
					println!("{:?}", eq_result);
					
					let cmp_result = sv1.cmp(&sv2);
					println!("{:?}", cmp_result);
					
					let partial_cmp_result = sv1.partial_cmp(&sv2);
					if let Some(ord) = partial_cmp_result {
						println!("{:?}", ord);
					}
					
					let result = sv1.into_inner();
					match result {
						Ok(_) => {},
						Err(sv_back) => {
							let _vec = sv_back.into_vec();
						}
					}
				},
				2 => {
					let mut sv = SmallVec::<[f64; 20]>::new();
					let num_items = _to_u8(GLOBAL_DATA, base_offset + 1) % 30;
					
					for i in 0..num_items {
						let val = _to_f64(GLOBAL_DATA, base_offset + 2 + (i as usize % 8));
						sv.push(val);
					}
					
					let capacity = sv.capacity();
					println!("{:?}", capacity);
					
					let slice_ref = sv.as_slice();
					if slice_ref.len() > 0 {
						let first_elem = &slice_ref[0];
						println!("{:?}", *first_elem);
					}
					
					let mut_slice_ref = sv.as_mut_slice();
					if mut_slice_ref.len() > 0 {
						let first_elem_mut = &mut mut_slice_ref[0];
						println!("{:?}", *first_elem_mut);
					}
					
					let result = sv.into_inner();
					match result {
						Ok(_) => {},
						Err(sv_back) => {
							let _vec = sv_back.into_vec();
						}
					}
				},
				3 => {
					let mut sv = SmallVec::<[String; 8]>::new();
					let str_len = _to_u8(GLOBAL_DATA, base_offset + 1) % 10;
					let str_data = _to_str(GLOBAL_DATA, base_offset + 2, base_offset + 2 + str_len as usize);
					sv.push(String::from(str_data));
					
					let idx = _to_usize(GLOBAL_DATA, base_offset + 12);
					sv.insert(0, String::from("inserted"));
					
					if sv.len() > 1 {
						let removed = sv.remove(0);
						println!("{:?}", removed);
					}
					
					sv.clear();
					let is_empty = sv.is_empty();
					println!("{:?}", is_empty);
					
					let result = sv.into_inner();
					match result {
						Ok(_) => {},
						Err(sv_back) => {
							let _vec = sv_back.into_vec();
						}
					}
				},
				4 => {
					let mut sv = SmallVec::<[u8; 32]>::new();
					let data_len = _to_u8(GLOBAL_DATA, base_offset + 1) % 40;
					
					for i in 0..data_len {
						sv.push(i);
					}
					
					let range_start = _to_usize(GLOBAL_DATA, base_offset + 2);
					let range_end = _to_usize(GLOBAL_DATA, base_offset + 10);
					
					if range_start <= sv.len() && range_end <= sv.len() && range_start <= range_end {
						let mut drain = sv.drain(range_start..range_end);
						while let Some(item) = drain.next() {
							println!("{:?}", item);
						}
					}
					
					let result = sv.into_inner();
					match result {
						Ok(_) => {},
						Err(sv_back) => {
							let _vec = sv_back.into_vec();
						}
					}
				},
				5 => {
					let mut sv = SmallVec::<[CustomType1; 6]>::new();
					let str_len = _to_u8(GLOBAL_DATA, base_offset + 1) % 8;
					let str_data = _to_str(GLOBAL_DATA, base_offset + 2, base_offset + 2 + str_len as usize);
					sv.push(CustomType1(String::from(str_data)));
					
					let reserve_amount = _to_usize(GLOBAL_DATA, base_offset + 10);
					sv.reserve(reserve_amount);
					
					let try_reserve_result = sv.try_reserve(reserve_amount);
					match try_reserve_result {
						Ok(_) => {},
						Err(_) => {}
					}
					
					sv.shrink_to_fit();
					
					let result = sv.into_inner();
					match result {
						Ok(_) => {},
						Err(sv_back) => {
							let _vec = sv_back.into_vec();
						}
					}
				},
				6 => {
					let mut sv = SmallVec::<[bool; 24]>::new();
					let num_bools = _to_u8(GLOBAL_DATA, base_offset + 1) % 30;
					
					for i in 0..num_bools {
						let bool_val = _to_bool(GLOBAL_DATA, base_offset + 2 + (i as usize % 8));
						sv.push(bool_val);
					}
					
					let new_len = _to_usize(GLOBAL_DATA, base_offset + 10);
					sv.truncate(new_len);
					
					let resize_len = _to_usize(GLOBAL_DATA, base_offset + 11);
					let resize_val = _to_bool(GLOBAL_DATA, base_offset + 12);
					sv.resize(resize_len, resize_val);
					
					sv.dedup();
					
					let result = sv.into_inner();
					match result {
						Ok(_) => {},
						Err(sv_back) => {
							let _vec = sv_back.into_vec();
						}
					}
				},
				_ => {
					let constructor_choice = _to_u8(GLOBAL_DATA, base_offset + 1) % 4;
					let mut sv = match constructor_choice {
						0 => SmallVec::<[u64; 15]>::new(),
						1 => {
							let cap = _to_usize(GLOBAL_DATA, base_offset + 2);
							SmallVec::<[u64; 15]>::with_capacity(cap)
						},
						2 => {
							let arr = [777u64; 15];
							SmallVec::from_buf(arr)
						},
						_ => {
							let elem = _to_u64(GLOBAL_DATA, base_offset + 3);
							let count = _to_usize(GLOBAL_DATA, base_offset + 11);
							SmallVec::<[u64; 15]>::from_elem(elem, count)
						}
					};
					
					let val = _to_u64(GLOBAL_DATA, base_offset + 4);
					sv.push(val);
					
					let spilled = sv.spilled();
					println!("{:?}", spilled);
					
					let into_iter = sv.into_iter();
					let mut iter_clone = into_iter.clone();
					while let Some(item) = iter_clone.next() {
						println!("{:?}", item);
					}
					
					let remaining_slice = into_iter.as_slice();
					if remaining_slice.len() > 0 {
						let first = &remaining_slice[0];
						println!("{:?}", *first);
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