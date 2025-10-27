#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use slice_deque::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

#[derive(Debug)]
struct CustomType0(String);
#[derive(Debug, Clone)]
struct CustomType1(String);
#[derive(Debug)]
struct CustomType2(String);

impl std::iter::IntoIterator for CustomType0 {
	type Item = CustomType1;
	type IntoIter = CustomType2;
	
	fn into_iter(self) -> Self::IntoIter {
		let global_data = get_global_data();
		let GLOBAL_DATA = global_data.first_half;
		let custom_impl_num = _to_usize(GLOBAL_DATA, 49);
		let custom_impl_inst_num = self.0.len();
		let selector = (custom_impl_num + custom_impl_inst_num) % 3;
		if selector == 0{
			panic!("INTENTIONAL PANIC!");
		}
		let GLOBAL_DATA = match selector{
				1 => global_data.first_half,
				_ => global_data.second_half,
		};
		let mut t_9 = _to_u8(GLOBAL_DATA, 57) % 17;
		let t_10 = _to_str(GLOBAL_DATA, 58, 58 + t_9 as usize);
		let t_11 = String::from(t_10);
		let t_12 = CustomType2(t_11);
		return t_12;
	}
}

impl std::iter::Iterator for CustomType2 {
	type Item = CustomType1;
	
	fn size_hint(&self) -> (usize, Option<usize>) {
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
		let t_0 = _to_usize(GLOBAL_DATA, 8);
		let t_1 = _to_usize(GLOBAL_DATA, 16);
		let t_2 = Some(t_1);
		let t_3 = (t_0, t_2);
		return t_3;
	}
	
	fn next(&mut self) -> Option<Self::Item> {
		let global_data = get_global_data();
		let GLOBAL_DATA = global_data.first_half;
		let custom_impl_num = _to_usize(GLOBAL_DATA, 24);
		let custom_impl_inst_num = self.0.len();
		let selector = (custom_impl_num + custom_impl_inst_num) % 3;
		if selector == 0{
			panic!("INTENTIONAL PANIC!");
		}
		let GLOBAL_DATA = match selector{
				1 => global_data.first_half,
				_ => global_data.second_half,
		};
		let mut t_4 = _to_u8(GLOBAL_DATA, 32) % 17;
		let t_5 = _to_str(GLOBAL_DATA, 33, 33 + t_4 as usize);
		let t_6 = String::from(t_5);
		let t_7 = CustomType1(t_6);
		let t_8 = Some(t_7);
		return t_8;
	}
}

fn main (){
	fuzz_nohook!(|data: &[u8]| {
		if data.len() < 4096 {return;}
		set_global_data(data);
		let global_data = get_global_data();
		let GLOBAL_DATA = global_data.first_half;

		let mut offset = _to_usize(GLOBAL_DATA, 0) % 256;
		let op_count = _to_u8(GLOBAL_DATA, offset) % 24;
		offset += 1;

		let mut deques = Vec::new();
		let mut current_deque = slice_deque::SliceDeque::new();

		for _ in 0..op_count {
			let op_selector = _to_u8(GLOBAL_DATA, offset) % 6;
			offset = (offset + 1) % 4096;

			match op_selector {
				0 => {
					let capacity = _to_usize(GLOBAL_DATA, offset) % 65;
					offset = (offset + 8) % 4096;
					current_deque = slice_deque::SliceDeque::with_capacity(capacity);
					deques.push(slice_deque::SliceDeque::from_iter(CustomType0("base".into())));
				}
				1 => {
					let elem_count = _to_usize(GLOBAL_DATA, offset) % 65;
					offset = (offset + 8) % 4096;
					let str_len = _to_u8(GLOBAL_DATA, offset) % 17;
					offset = (offset + 1) % 4096;
					let elem_str = _to_str(GLOBAL_DATA, offset, offset + str_len as usize);
					offset = (offset + str_len as usize) % 4096;
					let elem = CustomType1(elem_str.into());
					let new_deque = slice_deque::from_elem(elem, elem_count);
					deques.push(new_deque);
				}
				2 => {
					let splice_pos = _to_usize(GLOBAL_DATA, offset) % (current_deque.len() + 1);
					offset = (offset + 8) % 4096;
					let str_len = _to_u8(GLOBAL_DATA, offset) % 17;
					offset = (offset + 1) % 4096;
					let splice_str = _to_str(GLOBAL_DATA, offset, offset + str_len as usize);
					offset = (offset + str_len as usize) % 4096;
					current_deque.splice(splice_pos..splice_pos, CustomType0(splice_str.into()));
				}
				3 => {
					let drain_start = _to_usize(GLOBAL_DATA, offset) % (current_deque.len() + 1);
					offset = (offset + 8) % 4096;
					let drain_end = drain_start + (_to_usize(GLOBAL_DATA, offset) % (current_deque.len() - drain_start + 1));
					offset = (offset + 8) % 4096;
					let drained: Vec<_> = current_deque.drain(drain_start..drain_end).collect();
					if !drained.is_empty() {
						println!("{:?}", drained.last().unwrap());
					}
				}
				4 => {
					let filter_byte = _to_u8(GLOBAL_DATA, offset);
					offset = (offset + 1) % 4096;
					let mut filtered = current_deque.drain_filter(|_| (filter_byte % 2) == 0);
					while let Some(item) = filtered.next() {
						println!("{:?}", item);
					}
				}
				5 => {
					let slice = current_deque.as_slice();
					if !slice.is_empty() {
						println!("{:?}", &slice[slice.len()/2]);
					}
					let mut_slice = current_deque.as_mut_slice();
					if !mut_slice.is_empty() {
						println!("{:?}", &mut mut_slice[mut_slice.len()/2]);
					}
				}
				_ => {}
			}

			let ext_len = _to_u8(GLOBAL_DATA, offset) % 17;
			offset = (offset + 1) % 4096;
			let ext_data = _to_str(GLOBAL_DATA, offset, offset + ext_len as usize);
			offset = (offset + ext_len as usize) % 4096;
			current_deque.extend(CustomType0(ext_data.into()));
		}

		let mut final_ops = _to_u8(GLOBAL_DATA, offset) % 4;
		offset += 1;
		while final_ops > 0 {
			let trunc_pos = _to_usize(GLOBAL_DATA, offset) % (current_deque.len() + 1);
			current_deque.truncate(trunc_pos);
			
			let new_cap = _to_usize(GLOBAL_DATA, offset) % 129;
			current_deque.reserve(new_cap);
			
			final_ops -= 1;
			offset = (offset + 8) % 4096;
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