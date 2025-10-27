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
		if data.len() < 200 {return;}
		set_global_data(data);
		let global_data = get_global_data();
		let GLOBAL_DATA = global_data.first_half;
		let second_half = global_data.second_half;
		
		let num_operations = _to_u8(GLOBAL_DATA, 0) % 65;
		
		for op_idx in 0..num_operations {
			let base_offset = (op_idx as usize * 20) % (GLOBAL_DATA.len() - 20);
			let operation = _to_u8(GLOBAL_DATA, base_offset) % 10;
			
			match operation {
				0 => {
					let array_type = _to_u8(GLOBAL_DATA, base_offset + 1) % 4;
					match array_type {
						0 => {
							let str_len = _to_u8(second_half, base_offset) % 65;
							let str_data = _to_str(second_half, base_offset + 1, (base_offset + 1 + str_len as usize).min(second_half.len()));
							let items: Vec<String> = vec![String::from(str_data); str_len as usize % 65];
							let buf: [String; 16] = [String::new(), String::new(), String::new(), String::new(),
												   String::new(), String::new(), String::new(), String::new(),
												   String::new(), String::new(), String::new(), String::new(),
												   String::new(), String::new(), String::new(), String::new()];
							let len = _to_usize(GLOBAL_DATA, base_offset + 2);
							let sv = smallvec::SmallVec::from_buf_and_len(buf, len);
							println!("{:?}", sv);
						},
						1 => {
							let items: Vec<i32> = (0..(_to_u8(second_half, base_offset) % 65)).map(|i| _to_i32(second_half, (base_offset + i as usize * 4) % second_half.len())).collect();
							let buf: [i32; 32] = [0; 32];
							let len = _to_usize(GLOBAL_DATA, base_offset + 2);
							let sv = smallvec::SmallVec::from_buf_and_len(buf, len);
							println!("{:?}", sv);
						},
						2 => {
							let items: Vec<u8> = (0..(_to_u8(second_half, base_offset) % 65)).map(|i| _to_u8(second_half, (base_offset + i as usize) % second_half.len())).collect();
							let buf: [u8; 64] = [0; 64];
							let len = _to_usize(GLOBAL_DATA, base_offset + 2);
							let sv = smallvec::SmallVec::from_buf_and_len(buf, len);
							println!("{:?}", sv);
						},
						_ => {
							let items: Vec<f32> = (0..(_to_u8(second_half, base_offset) % 65)).map(|i| _to_f32(second_half, (base_offset + i as usize * 4) % (second_half.len() - 4))).collect();
							let buf: [f32; 20] = [0.0; 20];
							let len = _to_usize(GLOBAL_DATA, base_offset + 2);
							let sv = smallvec::SmallVec::from_buf_and_len(buf, len);
							println!("{:?}", sv);
						}
					}
				},
				1 => {
					let mut sv: SmallVec<[i32; 16]> = SmallVec::new();
					let push_count = _to_u8(GLOBAL_DATA, base_offset + 1) % 65;
					for i in 0..push_count {
						let value = _to_i32(GLOBAL_DATA, base_offset + 2 + (i as usize * 4));
						sv.push(value);
					}
					let slice = sv.as_slice();
					println!("{:?}", slice);
					let slice_ref = &slice[0];
					println!("{:?}", slice_ref);
					let len = sv.len();
					println!("{:?}", len);
					let capacity = sv.capacity();
					println!("{:?}", capacity);
				},
				2 => {
					let elem_count = _to_u8(GLOBAL_DATA, base_offset + 1) % 65;
					let elem_value = _to_u8(GLOBAL_DATA, base_offset + 2);
					let sv: SmallVec<[u8; 24]> = SmallVec::from_elem(elem_value, elem_count as usize);
					let mut_slice = sv.as_slice();
					println!("{:?}", mut_slice);
					let slice_ref = &mut_slice[0];
					println!("{:?}", slice_ref);
				},
				3 => {
					let iter_count = _to_u8(GLOBAL_DATA, base_offset + 1) % 65;
					let values: Vec<i16> = (0..iter_count).map(|i| _to_i16(GLOBAL_DATA, base_offset + 2 + (i as usize * 2))).collect();
					let sv: SmallVec<[i16; 12]> = SmallVec::from_iter(values);
					let vec_result = sv.into_vec();
					println!("{:?}", vec_result);
				},
				4 => {
					let slice_len = _to_u8(GLOBAL_DATA, base_offset + 1) % 65;
					let slice: Vec<f64> = (0..slice_len).map(|i| _to_f64(GLOBAL_DATA, base_offset + 2 + (i as usize * 8))).collect();
					let sv: SmallVec<[f64; 8]> = SmallVec::from_slice(&slice);
					let boxed = sv.into_boxed_slice();
					println!("{:?}", boxed);
				},
				5 => {
					let capacity = _to_usize(GLOBAL_DATA, base_offset + 1);
					let mut sv: SmallVec<[u32; 10]> = SmallVec::with_capacity(capacity);
					let insert_count = _to_u8(GLOBAL_DATA, base_offset + 9) % 65;
					for i in 0..insert_count {
						let value = _to_u32(GLOBAL_DATA, base_offset + 10 + (i as usize * 4));
						sv.push(value);
					}
					let pop_result = sv.pop();
					if let Some(val) = pop_result {
						println!("{:?}", val);
						println!("{:?}", &val);
					}
					sv.shrink_to_fit();
					sv.clear();
					println!("{:?}", sv.is_empty());
				},
				6 => {
					let vec_len = _to_u8(GLOBAL_DATA, base_offset + 1) % 65;
					let vec: Vec<char> = (0..vec_len).map(|i| _to_char(GLOBAL_DATA, base_offset + 2 + (i as usize * 4))).collect();
					let mut sv: SmallVec<[char; 15]> = SmallVec::from_vec(vec);
					
					if sv.len() > 0 {
						let index = _to_usize(GLOBAL_DATA, base_offset + 10);
						let removed = sv.remove(index);
						println!("{:?}", removed);
						println!("{:?}", &removed);
					}
					
					let insert_val = _to_char(GLOBAL_DATA, base_offset + 18);
					let insert_idx = _to_usize(GLOBAL_DATA, base_offset + 14);
					sv.insert(insert_idx, insert_val);
					
					let drain_start = _to_usize(GLOBAL_DATA, base_offset + 6);
					let drain_end = _to_usize(GLOBAL_DATA, base_offset + 11);
					let drain_iter = sv.drain(drain_start..drain_end);
					for item in drain_iter {
						println!("{:?}", item);
						println!("{:?}", &item);
					}
				},
				7 => {
					let mut sv1: SmallVec<[bool; 11]> = SmallVec::new();
					let mut sv2: SmallVec<[bool; 11]> = SmallVec::new();
					
					let count1 = _to_u8(GLOBAL_DATA, base_offset + 1) % 65;
					for i in 0..count1 {
						let val = _to_bool(GLOBAL_DATA, base_offset + 2 + i as usize);
						sv1.push(val);
					}
					
					let count2 = _to_u8(GLOBAL_DATA, base_offset + 10) % 65;
					for i in 0..count2 {
						let val = _to_bool(GLOBAL_DATA, base_offset + 11 + i as usize);
						sv2.push(val);
					}
					
					let eq_result = sv1.eq(&sv2);
					println!("{:?}", eq_result);
					
					let cmp_result = sv1.cmp(&sv2);
					println!("{:?}", cmp_result);
					
					let partial_cmp = sv1.partial_cmp(&sv2);
					if let Some(ord) = partial_cmp {
						println!("{:?}", ord);
						println!("{:?}", &ord);
					}
					
					sv1.append(&mut sv2);
					println!("{:?}", sv2.len());
				},
				8 => {
					let mut sv: SmallVec<[isize; 13]> = SmallVec::new();
					let push_count = _to_u8(GLOBAL_DATA, base_offset + 1) % 65;
					for i in 0..push_count {
						let val = _to_isize(GLOBAL_DATA, base_offset + 2 + (i as usize * 8));
						sv.push(val);
					}
					
					let reserve_amt = _to_usize(GLOBAL_DATA, base_offset + 10);
					sv.reserve(reserve_amt);
					
					let truncate_len = _to_usize(GLOBAL_DATA, base_offset + 18);
					sv.truncate(truncate_len);
					
					if sv.len() > 0 {
						let swap_idx = _to_usize(GLOBAL_DATA, base_offset + 14);
						let swapped = sv.swap_remove(swap_idx);
						println!("{:?}", swapped);
						println!("{:?}", &swapped);
					}
					
					let as_ptr = sv.as_ptr();
					println!("{:?}", as_ptr);
					
					let as_mut_ptr = sv.as_mut_ptr();
					println!("{:?}", as_mut_ptr);
				},
				_ => {
					let slice_len = _to_u8(GLOBAL_DATA, base_offset + 1) % 65;
					let slice_data: Vec<u16> = (0..slice_len).map(|i| _to_u16(GLOBAL_DATA, base_offset + 2 + (i as usize * 2))).collect();
					let mut sv: SmallVec<[u16; 18]> = SmallVec::from_slice(&slice_data);
					
					let extend_slice_len = _to_u8(GLOBAL_DATA, base_offset + 10) % 65;
					let extend_slice: Vec<u16> = (0..extend_slice_len).map(|i| _to_u16(GLOBAL_DATA, base_offset + 11 + (i as usize * 2))).collect();
					sv.extend_from_slice(&extend_slice);
					
					let resize_len = _to_usize(GLOBAL_DATA, base_offset + 15);
					let resize_val = _to_u16(GLOBAL_DATA, base_offset + 19);
					sv.resize(resize_len, resize_val);
					
					let retain_threshold = _to_u16(GLOBAL_DATA, base_offset + 17);
					sv.retain(|x| *x > retain_threshold);
					
					sv.dedup();
					
					let to_smallvec: SmallVec<[u16; 18]> = slice_data.as_slice().to_smallvec();
					println!("{:?}", to_smallvec.len());
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