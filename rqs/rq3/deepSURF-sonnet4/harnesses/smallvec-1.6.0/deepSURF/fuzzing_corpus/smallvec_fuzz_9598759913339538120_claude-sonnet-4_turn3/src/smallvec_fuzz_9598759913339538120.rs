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
		if data.len() < 300 {return;}
		set_global_data(data);
		let global_data = get_global_data();
		let GLOBAL_DATA = global_data.first_half;
		
		let num_operations = _to_u8(GLOBAL_DATA, 0) % 65;
		
		for i in 0..num_operations {
			let base_index = (i as usize) * 4;
			if base_index + 20 >= GLOBAL_DATA.len() { break; }
			
			match _to_u8(GLOBAL_DATA, base_index) % 8 {
				0 => {
					let constructor_choice = _to_u8(GLOBAL_DATA, base_index + 1) % 6;
					let mut small_vec = match constructor_choice {
						0 => smallvec::SmallVec::<[u32; 16]>::new(),
						1 => {
							let capacity = _to_usize(GLOBAL_DATA, base_index + 2);
							smallvec::SmallVec::<[u32; 16]>::with_capacity(capacity)
						},
						2 => {
							let elem = _to_u32(GLOBAL_DATA, base_index + 2);
							let count = _to_usize(GLOBAL_DATA, base_index + 10) % 65;
							smallvec::SmallVec::<[u32; 16]>::from_elem(elem, count)
						},
						3 => {
							let vec_size = _to_usize(GLOBAL_DATA, base_index + 2) % 65;
							let mut vec = Vec::new();
							for j in 0..vec_size {
								vec.push(_to_u32(GLOBAL_DATA, base_index + 10 + (j % 4)));
							}
							smallvec::SmallVec::<[u32; 16]>::from_vec(vec)
						},
						4 => {
							let slice_data = &[_to_u32(GLOBAL_DATA, base_index + 2), _to_u32(GLOBAL_DATA, base_index + 6), _to_u32(GLOBAL_DATA, base_index + 10)];
							smallvec::SmallVec::<[u32; 16]>::from_slice(slice_data)
						},
						_ => {
							let iter_size = _to_usize(GLOBAL_DATA, base_index + 2) % 65;
							let iter_data: Vec<u32> = (0..iter_size).map(|k| _to_u32(GLOBAL_DATA, base_index + 10 + (k % 4))).collect();
							smallvec::SmallVec::<[u32; 16]>::from_iter(iter_data)
						}
					};
					
					let grow_size = _to_usize(GLOBAL_DATA, base_index + 18);
					let result = small_vec.try_grow(grow_size);
					println!("{:?}", result);
					
					let capacity_val = small_vec.capacity();
					println!("{}", capacity_val);
					small_vec.reserve(_to_usize(GLOBAL_DATA, base_index + 14));
					small_vec.shrink_to_fit();
					
					if small_vec.len() > 0 {
						if let Some(popped) = small_vec.pop() {
							println!("{}", popped);
						}
					}
				},
				1 => {
					let mut small_vec = smallvec::SmallVec::<[String; 12]>::new();
					let push_count = _to_u8(GLOBAL_DATA, base_index + 1) % 65;
					for j in 0..push_count {
						let j_usize = j as usize;
						let str_len = (_to_u8(GLOBAL_DATA, base_index + 2 + j_usize) % 10) as usize + 1;
						if base_index + 3 + j_usize + str_len < GLOBAL_DATA.len() {
							let s = _to_str(GLOBAL_DATA, base_index + 3 + j_usize, base_index + 3 + j_usize + str_len);
							small_vec.push(String::from(s));
						}
					}
					
					let capacity_val = _to_usize(GLOBAL_DATA, base_index + 15);
					println!("{}", small_vec.capacity());
					let slice_ref = small_vec.as_slice();
					for elem in slice_ref {
						println!("{:?}", elem);
					}
					small_vec.reserve(_to_usize(GLOBAL_DATA, base_index + 16));
					small_vec.shrink_to_fit();
					
					let clone_vec = small_vec.clone();
					println!("{}", clone_vec.len());
				},
				2 => {
					let arr = [_to_i64(GLOBAL_DATA, base_index + 1), _to_i64(GLOBAL_DATA, base_index + 9), _to_i64(GLOBAL_DATA, base_index + 17)];
					let mut small_vec = smallvec::SmallVec::from_buf(arr);
					
					if let Some(popped) = small_vec.pop() {
						println!("{}", popped);
					}
					
					let index = _to_usize(GLOBAL_DATA, base_index + 2);
					if index < small_vec.len() {
						let removed = small_vec.remove(index);
						println!("{}", removed);
					}
					
					small_vec.insert(_to_usize(GLOBAL_DATA, base_index + 3), _to_i64(GLOBAL_DATA, base_index + 11));
					small_vec.clear();
					
					let ptr = small_vec.as_ptr();
					println!("{:?}", ptr);
					let mut_ptr = small_vec.as_mut_ptr();
					println!("{:?}", mut_ptr);
				},
				3 => {
					let mut small_vec1 = smallvec::SmallVec::<[f32; 20]>::with_capacity(_to_usize(GLOBAL_DATA, base_index + 1));
					let mut small_vec2 = smallvec::SmallVec::<[f32; 20]>::new();
					
					for k in 0..5 {
						small_vec1.push(_to_f32(GLOBAL_DATA, base_index + 2 + k * 4));
						small_vec2.push(_to_f32(GLOBAL_DATA, base_index + 10 + k * 4));
					}
					
					small_vec1.append(&mut small_vec2);
					let mut_slice = small_vec1.as_mut_slice();
					for elem in mut_slice.iter() {
						println!("{:?}", elem);
					}
					
					let drain_range = 1..3;
					let mut drained: Vec<f32> = small_vec1.drain(drain_range).collect();
					for item in &mut drained {
						println!("{}", item);
					}
				},
				4 => {
					let mut small_vec = smallvec::SmallVec::<[bool; 30]>::new();
					let extend_size = _to_u8(GLOBAL_DATA, base_index + 1) % 65;
					
					for m in 0..extend_size {
						let m_usize = m as usize;
						small_vec.push(_to_bool(GLOBAL_DATA, base_index + 2 + m_usize));
					}
					
					let extend_slice = &[_to_bool(GLOBAL_DATA, base_index + 10), _to_bool(GLOBAL_DATA, base_index + 11)];
					small_vec.extend_from_slice(extend_slice);
					
					small_vec.retain(|&mut x| x);
					small_vec.dedup();
					
					let len_val = small_vec.len();
					println!("{}", len_val);
					let is_empty = small_vec.is_empty();
					println!("{}", is_empty);
					
					let spilled = small_vec.spilled();
					println!("{}", spilled);
				},
				5 => {
					let mut small_vec = smallvec::SmallVec::<[char; 25]>::new();
					let resize_size = _to_usize(GLOBAL_DATA, base_index + 1) % 65;
					let fill_char = _to_char(GLOBAL_DATA, base_index + 9);
					
					small_vec.resize(resize_size, fill_char);
					
					let truncate_size = _to_usize(GLOBAL_DATA, base_index + 13);
					small_vec.truncate(truncate_size);
					
					let into_vec = small_vec.into_vec();
					println!("{:?}", into_vec);
				},
				6 => {
					let constructor_choice = _to_u8(GLOBAL_DATA, base_index + 1) % 3;
					let mut small_vec = match constructor_choice {
						0 => smallvec::SmallVec::<[u8; 64]>::new(),
						1 => {
							let capacity = _to_usize(GLOBAL_DATA, base_index + 2);
							smallvec::SmallVec::<[u8; 64]>::with_capacity(capacity)
						},
						_ => {
							let elem = _to_u8(GLOBAL_DATA, base_index + 2);
							let count = _to_usize(GLOBAL_DATA, base_index + 6) % 65;
							smallvec::SmallVec::<[u8; 64]>::from_elem(elem, count)
						}
					};
					
					small_vec.try_reserve(_to_usize(GLOBAL_DATA, base_index + 10));
					small_vec.try_reserve_exact(_to_usize(GLOBAL_DATA, base_index + 14));
					
					if small_vec.len() > 1 {
						let swap_index = _to_usize(GLOBAL_DATA, base_index + 18) % small_vec.len();
						let removed = small_vec.swap_remove(swap_index);
						println!("{}", removed);
					}
					
					let index = _to_usize(GLOBAL_DATA, base_index + 3);
					if index < small_vec.len() {
						let indexed_val = &small_vec[index];
						println!("{}", indexed_val);
					}
				},
				_ => {
					let mut small_vec1 = smallvec::SmallVec::<[u16; 32]>::new();
					let mut small_vec2 = smallvec::SmallVec::<[u16; 32]>::new();
					
					for k in 0..10 {
						small_vec1.push(_to_u16(GLOBAL_DATA, base_index + k * 2));
						small_vec2.push(_to_u16(GLOBAL_DATA, base_index + 10 + k * 2));
					}
					
					let eq_result = small_vec1.eq(&small_vec2);
					println!("{}", eq_result);
					
					let cmp_result = small_vec1.cmp(&small_vec2);
					println!("{:?}", cmp_result);
					
					let partial_cmp = small_vec1.partial_cmp(&small_vec2);
					println!("{:?}", partial_cmp);
					
					small_vec1.insert_many(_to_usize(GLOBAL_DATA, base_index + 15), small_vec2.iter().cloned());
					
					let boxed = small_vec1.into_boxed_slice();
					println!("{}", boxed.len());
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