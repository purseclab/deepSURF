#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use toodee::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

struct CustomType3(String);
struct CustomType0(String);
struct CustomType2(String);
struct CustomType1(String);

impl core::iter::ExactSizeIterator for CustomType1 {
	
	fn len(&self) -> usize {
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
		return t_0;
	}
}

impl core::iter::DoubleEndedIterator for CustomType1 {
	
	fn next_back(&mut self) -> Option<Self::Item> {
		let global_data = get_global_data();
		let GLOBAL_DATA = global_data.first_half;
		let custom_impl_num = _to_usize(GLOBAL_DATA, 66);
		let custom_impl_inst_num = self.0.len();
		let selector = (custom_impl_num + custom_impl_inst_num) % 3;
		if selector == 0{
			panic!("INTENTIONAL PANIC!");
		}
		let GLOBAL_DATA = match selector{
				1 => global_data.first_half,
				_ => global_data.second_half,
		};
		let mut t_11 = _to_u8(GLOBAL_DATA, 74) % 17;
		let t_12 = _to_str(GLOBAL_DATA, 75, 75 + t_11 as usize);
		let t_13 = String::from(t_12);
		let t_15 = Some(t_13);
		return t_15;
	}
}

impl core::iter::Iterator for CustomType1 {
	type Item = String;
	
	fn rev(self) -> core::iter::Rev<CustomType1> {
		let global_data = get_global_data();
		let GLOBAL_DATA = global_data.first_half;
		let custom_impl_num = _to_usize(GLOBAL_DATA, 16);
		let custom_impl_inst_num = self.0.len();
		let selector = (custom_impl_num + custom_impl_inst_num) % 3;
		if selector == 0{
			panic!("INTENTIONAL PANIC!");
		}
		let GLOBAL_DATA = match selector{
				1 => global_data.first_half,
				_ => global_data.second_half,
		};
		let mut t_1 = _to_u8(GLOBAL_DATA, 24) % 17;
		let t_2 = _to_str(GLOBAL_DATA, 25, 25 + t_1 as usize);
		let t_3 = String::from(t_2);
		let t_4 = CustomType1(t_3);
		core::iter::Iterator::rev(t_4)
	}
	
	fn next(&mut self) -> Option<Self::Item> {
		let global_data = get_global_data();
		let GLOBAL_DATA = global_data.first_half;
		let custom_impl_num = _to_usize(GLOBAL_DATA, 41);
		let custom_impl_inst_num = self.0.len();
		let selector = (custom_impl_num + custom_impl_inst_num) % 3;
		if selector == 0{
			panic!("INTENTIONAL PANIC!");
		}
		let GLOBAL_DATA = match selector{
				1 => global_data.first_half,
				_ => global_data.second_half,
		};
		let mut t_6 = _to_u8(GLOBAL_DATA, 49) % 17;
		let t_7 = _to_str(GLOBAL_DATA, 50, 50 + t_6 as usize);
		let t_8 = String::from(t_7);
		let t_10 = Some(t_8);
		return t_10;
	}
}

impl core::iter::IntoIterator for CustomType3 {
	type Item = String;
	type IntoIter = CustomType1;
	
	fn into_iter(self) -> Self::IntoIter {
		let global_data = get_global_data();
		let GLOBAL_DATA = global_data.first_half;
		let custom_impl_num = _to_usize(GLOBAL_DATA, 660);
		let custom_impl_inst_num = self.0.len();
		let selector = (custom_impl_num + custom_impl_inst_num) % 3;
		if selector == 0{
			panic!("INTENTIONAL PANIC!");
		}
		let GLOBAL_DATA = match selector{
				1 => global_data.first_half,
				_ => global_data.second_half,
		};
		let mut t_151 = _to_u8(GLOBAL_DATA, 668) % 17;
		let t_152 = _to_str(GLOBAL_DATA, 669, 669 + t_151 as usize);
		let t_153 = String::from(t_152);
		let t_154 = CustomType1(t_153);
		return t_154;
	}
}

fn main() {
	fuzz_nohook!(|data: &[u8]| {
		if data.len() < 3200 {return;}
		set_global_data(data);
		let global_data = get_global_data();
		let GLOBAL_DATA = global_data.first_half;
		
		let num_operations = (_to_u8(GLOBAL_DATA, 0) % 15) + 1;
		
		for op_idx in 0..num_operations {
			let operation = _to_u8(GLOBAL_DATA, (op_idx * 200 + 1) as usize) % 20;
			let base_offset = (op_idx * 200 + 2) as usize;
			
			match operation {
				0..=2 => {
					let constructor_choice = _to_u8(GLOBAL_DATA, base_offset) % 7;
					
					let new_toodee: TooDee<String> = match constructor_choice {
						0 => {
							let rows = _to_usize(GLOBAL_DATA, base_offset + 2);
							let cols = _to_usize(GLOBAL_DATA, base_offset + 10);
							TooDee::init(cols, rows, String::new())
						},
						1 => {
							let capacity = _to_usize(GLOBAL_DATA, base_offset + 18);
							TooDee::with_capacity(capacity)
						},
						2 => {
							let rows = _to_usize(GLOBAL_DATA, base_offset + 26);
							let cols = _to_usize(GLOBAL_DATA, base_offset + 34);
							let init_str_len = (_to_u8(GLOBAL_DATA, base_offset + 42) % 16) + 1;
							let init_str = _to_str(GLOBAL_DATA, base_offset + 43, base_offset + 43 + init_str_len as usize);
							TooDee::init(cols, rows, String::from(init_str))
						},
						3 => {
							let rows = _to_usize(GLOBAL_DATA, base_offset + 60);
							let cols = _to_usize(GLOBAL_DATA, base_offset + 68);
							let vec_size = _to_u8(GLOBAL_DATA, base_offset + 76) % 65;
							let mut vec_data = Vec::with_capacity(vec_size as usize);
							for i in 0..vec_size {
								let str_len = (_to_u8(GLOBAL_DATA, base_offset + 77 + i as usize) % 16) + 1;
								let str_data = _to_str(GLOBAL_DATA, base_offset + 93 + i as usize * 17, base_offset + 93 + i as usize * 17 + str_len as usize);
								vec_data.push(String::from(str_data));
							}
							TooDee::from_vec(cols, rows, vec_data)
						},
						_ => TooDee::init(1, 1, String::new())
					};
					
					let col_idx = _to_usize(GLOBAL_DATA, base_offset + 1);
					let iter_items = _to_u8(GLOBAL_DATA, base_offset + 9) % 65;
					
					let mut iter_data = Vec::new();
					for i in 0..iter_items {
						let str_len = (_to_u8(GLOBAL_DATA, base_offset + 10 + i as usize) % 16) + 1;
						let str_data = _to_str(GLOBAL_DATA, base_offset + 75 + i as usize * 17, base_offset + 75 + i as usize * 17 + str_len as usize);
						iter_data.push(String::from(str_data));
					}
					
					let mut toodee = new_toodee;
					toodee.insert_col(col_idx, iter_data);
					
					let cells = toodee.cells();
					for cell in cells {
						println!("{:?}", *cell);
					}
				},
				3 => {
					let mut toodee: TooDee<String> = TooDee::init(1, 1, String::new());
					let drain_col = toodee.pop_col();
					if let Some(mut drain) = drain_col {
						while let Some(item) = drain.next() {
							println!("{:?}", item);
						}
					}
				},
				4 => {
					let mut toodee: TooDee<String> = TooDee::init(3, 3, String::new());
					let col_idx = _to_usize(GLOBAL_DATA, base_offset + 1);
					let drain_col = toodee.remove_col(col_idx);
					for item in drain_col {
						println!("{:?}", item);
					}
				},
				5 => {
					let mut toodee: TooDee<String> = TooDee::init(3, 3, String::new());
					let row_idx = _to_usize(GLOBAL_DATA, base_offset + 1);
					let iter_items = _to_u8(GLOBAL_DATA, base_offset + 9) % 65;
					
					let mut iter_data = Vec::new();
					for i in 0..iter_items {
						let str_len = (_to_u8(GLOBAL_DATA, base_offset + 10 + i as usize) % 16) + 1;
						let str_data = _to_str(GLOBAL_DATA, base_offset + 75 + i as usize * 17, base_offset + 75 + i as usize * 17 + str_len as usize);
						iter_data.push(String::from(str_data));
					}
					
					toodee.insert_row(row_idx, iter_data);
					
					let rows_iter = toodee.rows();
					for row in rows_iter {
						for item in row {
							println!("{:?}", *item);
						}
					}
				},
				6 => {
					let toodee: TooDee<String> = TooDee::init(3, 3, String::new());
					let start_col = _to_usize(GLOBAL_DATA, base_offset + 1);
					let start_row = _to_usize(GLOBAL_DATA, base_offset + 9);
					let end_col = _to_usize(GLOBAL_DATA, base_offset + 17);
					let end_row = _to_usize(GLOBAL_DATA, base_offset + 25);
					
					let view = toodee.view((start_col, start_row), (end_col, end_row));
					let col_idx = _to_usize(GLOBAL_DATA, base_offset + 33);
					let col_iter = view.col(col_idx);
					for item in col_iter {
						println!("{:?}", *item);
					}
				},
				7 => {
					let mut toodee: TooDee<String> = TooDee::init(3, 3, String::new());
					let start_col = _to_usize(GLOBAL_DATA, base_offset + 1);
					let start_row = _to_usize(GLOBAL_DATA, base_offset + 9);
					let end_col = _to_usize(GLOBAL_DATA, base_offset + 17);
					let end_row = _to_usize(GLOBAL_DATA, base_offset + 25);
					
					let mut view_mut = toodee.view_mut((start_col, start_row), (end_col, end_row));
					let col_idx = _to_usize(GLOBAL_DATA, base_offset + 33);
					let mut col_mut = view_mut.col_mut(col_idx);
					while let Some(item) = col_mut.next() {
						println!("{:?}", *item);
					}
				},
				8 => {
					let toodee: TooDee<String> = TooDee::init(3, 3, String::new());
					let col_idx = _to_usize(GLOBAL_DATA, base_offset + 1);
					let col_iter = toodee.col(col_idx);
					for item in col_iter {
						println!("{:?}", *item);
					}
				},
				9 => {
					let mut toodee: TooDee<String> = TooDee::init(3, 3, String::new());
					let col_idx = _to_usize(GLOBAL_DATA, base_offset + 1);
					let mut col_mut = toodee.col_mut(col_idx);
					while let Some(item) = col_mut.next() {
						println!("{:?}", *item);
					}
				},
				10 => {
					let toodee: TooDee<String> = TooDee::init(3, 3, String::new());
					let row_idx = _to_usize(GLOBAL_DATA, base_offset + 1);
					let row_ref = &toodee[row_idx];
					for item in row_ref {
						println!("{:?}", *item);
					}
				},
				11 => {
					let toodee: TooDee<String> = TooDee::init(3, 3, String::new());
					let col = _to_usize(GLOBAL_DATA, base_offset + 1);
					let row = _to_usize(GLOBAL_DATA, base_offset + 9);
					let cell_ref = &toodee[(col, row)];
					println!("{:?}", *cell_ref);
				},
				12 => {
					let mut toodee: TooDee<String> = TooDee::init(3, 3, String::new());
					let col = _to_usize(GLOBAL_DATA, base_offset + 1);
					let row = _to_usize(GLOBAL_DATA, base_offset + 9);
					let cell_mut_ref = &mut toodee[(col, row)];
					println!("{:?}", *cell_mut_ref);
				},
				13 => {
					let mut toodee: TooDee<String> = TooDee::init(3, 3, String::new());
					let r1 = _to_usize(GLOBAL_DATA, base_offset + 1);
					let r2 = _to_usize(GLOBAL_DATA, base_offset + 9);
					toodee.swap_rows(r1, r2);
					
					let cells = toodee.cells();
					for cell in cells {
						println!("{:?}", *cell);
					}
				},
				14 => {
					let mut toodee: TooDee<String> = TooDee::init(3, 3, String::new());
					let rows_mut = toodee.rows_mut();
					for row in rows_mut {
						for item in row {
							println!("{:?}", *item);
						}
					}
				},
				15 => {
					let toodee: TooDee<String> = TooDee::init(3, 3, String::new());
					let cells = toodee.cells();
					for cell in cells {
						println!("{:?}", *cell);
					}
				},
				16 => {
					let custom_str_len = (_to_u8(GLOBAL_DATA, base_offset) % 16) + 1;
					let custom_str = _to_str(GLOBAL_DATA, base_offset + 1, base_offset + 1 + custom_str_len as usize);
					let custom_type = CustomType3(String::from(custom_str));
					
					let mut toodee: TooDee<String> = TooDee::init(1, 1, String::new());
					let col_idx = _to_usize(GLOBAL_DATA, base_offset + 18);
					
					toodee.insert_col(col_idx, custom_type);
					
					let cells = toodee.cells();
					for cell in cells {
						println!("{:?}", *cell);
					}
				},
				17 => {
					let toodee: TooDee<String> = TooDee::init(5, 4, String::new());
					let start_col = _to_usize(GLOBAL_DATA, base_offset + 1);
					let start_row = _to_usize(GLOBAL_DATA, base_offset + 9);
					let end_col = _to_usize(GLOBAL_DATA, base_offset + 17);
					let end_row = _to_usize(GLOBAL_DATA, base_offset + 25);
					
					let view = toodee.view((start_col, start_row), (end_col, end_row));
					let rows_iter = view.rows();
					for row in rows_iter {
						for item in row {
							println!("{:?}", *item);
						}
					}
				},
				18 => {
					let mut toodee = TooDee::init(4, 3, String::from("test"));
					let row_idx = _to_usize(GLOBAL_DATA, base_offset + 1);
					let drain_row = toodee.remove_row(row_idx);
					for item in drain_row {
						println!("{:?}", item);
					}
				},
				_ => {
					let mut toodee: TooDee<String> = TooDee::init(4, 4, String::new());
					let str_len = (_to_u8(GLOBAL_DATA, base_offset) % 16) + 1;
					let str_data = _to_str(GLOBAL_DATA, base_offset + 1, base_offset + 1 + str_len as usize);
					let iter_data = vec![String::from(str_data); 4];
					
					toodee.push_row(iter_data);
					
					let rows_iter = toodee.rows();
					for row in rows_iter {
						for item in row {
							println!("{:?}", *item);
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