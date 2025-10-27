#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use simple_slab::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

#[derive(Debug)]
struct CustomType0(String);

#[derive(Debug)]
struct CustomType1(i32);

#[derive(Debug)]
struct CustomType2(Vec<u8>);

fn main (){
	fuzz_nohook!(|data: &[u8]| {
		if data.len() < 256 {return;}
		set_global_data(data);
		let global_data = get_global_data();
		let GLOBAL_DATA = global_data.first_half;
		
		let num_ops = _to_usize(GLOBAL_DATA, 0) % 65;
		for op_idx in 0..num_ops {
			let op_type = _to_u8(GLOBAL_DATA, op_idx * 8) % 6;
			match op_type {
				0 => {
					let capacity = _to_usize(GLOBAL_DATA, op_idx * 8 + 1);
					let mut slab = if _to_bool(GLOBAL_DATA, op_idx * 8 + 9) {
						Slab::<CustomType0>::new()
					} else {
						Slab::<CustomType0>::with_capacity(capacity)
					};
					
					let num_inserts = _to_usize(GLOBAL_DATA, op_idx * 8 + 10) % 32;
					for i in 0..num_inserts {
						let string_val = format!("item_{}", i);
						slab.insert(CustomType0(string_val));
					}
					
					if slab.len() > 0 {
						let iter = slab.iter();
						for item_ref in iter {
							println!("{:?}", *item_ref);
						}
					}
					
					drop(slab);
				},
				1 => {
					let capacity = _to_usize(GLOBAL_DATA, op_idx * 8 + 1);
					let mut slab = Slab::<CustomType1>::with_capacity(capacity);
					
					let num_inserts = _to_usize(GLOBAL_DATA, op_idx * 8 + 9) % 45;
					for i in 0..num_inserts {
						let int_val = _to_i32(GLOBAL_DATA, op_idx * 8 + 10 + i * 4);
						slab.insert(CustomType1(int_val));
					}
					
					if slab.len() > 0 {
						let mut iter_mut = slab.iter_mut();
						while let Some(item_ref) = iter_mut.next() {
							println!("{:?}", *item_ref);
						}
					}
					
					let len = slab.len();
					if len > 0 {
						let remove_idx = _to_usize(GLOBAL_DATA, op_idx * 8 + 200);
						if remove_idx < len {
							let removed = slab.remove(remove_idx);
							println!("{:?}", removed);
						}
					}
					
					drop(slab);
				},
				2 => {
					let mut slab = Slab::<CustomType2>::new();
					
					let num_inserts = _to_usize(GLOBAL_DATA, op_idx * 8 + 1) % 20;
					for i in 0..num_inserts {
						let vec_size = _to_usize(GLOBAL_DATA, op_idx * 8 + 9 + i) % 15;
						let mut vec_data = Vec::new();
						for j in 0..vec_size {
							vec_data.push(_to_u8(GLOBAL_DATA, op_idx * 8 + 50 + j));
						}
						slab.insert(CustomType2(vec_data));
					}
					
					if slab.len() > 0 {
						let index = _to_usize(GLOBAL_DATA, op_idx * 8 + 100);
						if index < slab.len() {
							let indexed_ref = &slab[index];
							println!("{:?}", *indexed_ref);
						}
					}
					
					let slab_iter = slab.into_iter();
					for item_ref in slab_iter {
						println!("{:?}", *item_ref);
					}
				},
				3 => {
					let capacity = _to_usize(GLOBAL_DATA, op_idx * 8 + 1);
					let mut slab1 = Slab::<String>::with_capacity(capacity);
					let mut slab2 = Slab::<i64>::new();
					
					let str_len = _to_usize(GLOBAL_DATA, op_idx * 8 + 9) % 20;
					for i in 0..str_len {
						let char_val = _to_char(GLOBAL_DATA, op_idx * 8 + 20 + i * 4);
						slab1.insert(char_val.to_string());
					}
					
					let num_count = _to_usize(GLOBAL_DATA, op_idx * 8 + 100) % 30;
					for i in 0..num_count {
						let num_val = _to_i64(GLOBAL_DATA, op_idx * 8 + 120 + i * 8);
						slab2.insert(num_val);
					}
					
					if slab1.len() > 0 {
						let mut iter1 = slab1.into_iter();
						while let Some(str_ref) = iter1.next() {
							println!("{:?}", *str_ref);
						}
					}
					
					if slab2.len() > 0 {
						let mut mut_iter = slab2.iter_mut();
						for num_ref in mut_iter {
							println!("{:?}", *num_ref);
						}
					}
					
					drop(slab2);
				},
				4 => {
					let mut slab = Slab::<f64>::new();
					
					let float_count = _to_usize(GLOBAL_DATA, op_idx * 8 + 1) % 25;
					for i in 0..float_count {
						let float_val = _to_f64(GLOBAL_DATA, op_idx * 8 + 10 + i * 8);
						slab.insert(float_val);
					}
					
					let len = slab.len();
					if len > 0 {
						let iter = slab.iter();
						for float_ref in iter {
							println!("{:?}", *float_ref);
						}
						
						let index1 = _to_usize(GLOBAL_DATA, op_idx * 8 + 200);
						let index2 = _to_usize(GLOBAL_DATA, op_idx * 8 + 208);
						
						if index1 < len {
							let val_ref = &slab[index1];
							println!("{:?}", *val_ref);
						}
						
						if index2 < len {
							let removed = slab.remove(index2);
							println!("{:?}", removed);
						}
					}
					
					drop(slab);
				},
				_ => {
					let mut slab = Slab::<bool>::with_capacity(_to_usize(GLOBAL_DATA, op_idx * 8 + 1));
					
					let bool_count = _to_usize(GLOBAL_DATA, op_idx * 8 + 9) % 40;
					for i in 0..bool_count {
						let bool_val = _to_bool(GLOBAL_DATA, op_idx * 8 + 20 + i);
						slab.insert(bool_val);
					}
					
					if slab.len() > 0 {
						let mut_iter = slab.into_iter();
						for bool_ref in mut_iter {
							println!("{:?}", *bool_ref);
						}
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