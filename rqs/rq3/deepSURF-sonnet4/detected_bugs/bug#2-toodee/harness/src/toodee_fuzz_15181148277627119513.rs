#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use toodee::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

#[derive(Debug, Clone, Default)]
struct CustomType1(String);
#[derive(Debug, Clone, Default)]
struct CustomType2(String);
#[derive(Debug, Clone, Default)]
struct CustomType3(String);
#[derive(Debug, Clone, Default)]
struct CustomType0(String);

impl core::iter::Iterator for CustomType1 {
	type Item = CustomType0;
	
	fn next(&mut self) -> Option<Self::Item> {
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
		let t_4 = CustomType0(t_3);
		let t_5 = Some(t_4);
		return t_5;
	}
}

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
		let t_14 = CustomType0(t_13);
		let t_15 = Some(t_14);
		return t_15;
	}
}

impl core::iter::IntoIterator for CustomType3 {
	type Item = CustomType0;
	type IntoIter = CustomType1;
	
	fn into_iter(self) -> Self::IntoIter {
		let global_data = get_global_data();
		let GLOBAL_DATA = global_data.first_half;
		let custom_impl_num = _to_usize(GLOBAL_DATA, 652);
		let custom_impl_inst_num = self.0.len();
		let selector = (custom_impl_num + custom_impl_inst_num) % 3;
		if selector == 0{
			panic!("INTENTIONAL PANIC!");
		}
		let GLOBAL_DATA = match selector{
				1 => global_data.first_half,
				_ => global_data.second_half,
		};
		let mut t_150 = _to_u8(GLOBAL_DATA, 660) % 17;
		let t_151 = _to_str(GLOBAL_DATA, 661, 661 + t_150 as usize);
		let t_152 = String::from(t_151);
		let t_153 = CustomType1(t_152);
		return t_153;
	}
}

fn main (){
	fuzz_nohook!(|data: &[u8]| {
		if data.len() < 1800 {return;}
		set_global_data(data);
		let global_data = get_global_data();
		let GLOBAL_DATA = global_data.first_half;
		
		let num_operations = _to_u8(GLOBAL_DATA, 1300) % 20 + 1;
		
		for operation_idx in 0..num_operations {
			let operation_selector = _to_u8(GLOBAL_DATA, 1301 + operation_idx as usize) % 8;
			
			match operation_selector {
				0 => {
					let t_16 = _to_usize(GLOBAL_DATA, 91);
					let t_17 = _to_usize(GLOBAL_DATA, 99);
					let constructor_choice = _to_u8(GLOBAL_DATA, 90) % 3;
					
					let mut toodee = match constructor_choice {
						0 => {
							let init_val = _to_u8(GLOBAL_DATA, 695) % 17;
							let t_696 = _to_str(GLOBAL_DATA, 696, 696 + init_val as usize);
							let t_697 = String::from(t_696);
							let t_698 = CustomType0(t_697);
							toodee::TooDee::init(t_16, t_17, t_698)
						},
						1 => {
							toodee::TooDee::<CustomType0>::new(t_16, t_17)
						},
						_ => {
							let mut t_18 = _to_u8(GLOBAL_DATA, 107) % 33;
							let mut t_19 = std::vec::Vec::with_capacity(32);
							for i in 0..32 {
								let t_offset = 108 + i * 17;
								let mut t_str_len = _to_u8(GLOBAL_DATA, t_offset) % 17;
								let t_str = _to_str(GLOBAL_DATA, t_offset + 1, t_offset + 1 + t_str_len as usize);
								let t_custom = CustomType0(String::from(t_str));
								t_19.push(t_custom);
							}
							t_19.truncate(t_18 as usize);
							toodee::TooDee::from_vec(t_16, t_17, t_19)
						}
					};
					
					let mut t_154 = _to_u8(GLOBAL_DATA, 677) % 17;
					let t_155 = _to_str(GLOBAL_DATA, 678, 678 + t_154 as usize);
					let t_156 = String::from(t_155);
					let t_157 = CustomType3(t_156);
					toodee.push_col(t_157);
					
					let ref_rows = toodee.rows();
					for row in ref_rows {
						println!("{:?}", row);
					}
					
					if toodee.num_cols() > 0 {
						let col_idx = _to_usize(GLOBAL_DATA, 1320);
						let col_iter = toodee.col(col_idx);
						for item in col_iter {
							println!("{:?}", &*item);
						}
					}
				},
				
				1 => {
					let t_16 = _to_usize(GLOBAL_DATA, 700);
					let t_17 = _to_usize(GLOBAL_DATA, 708);
					let mut toodee = toodee::TooDee::<CustomType0>::with_capacity(t_16);
					
					let rows_to_push = _to_u8(GLOBAL_DATA, 716) % 10;
					for i in 0..rows_to_push {
						let start_idx = 720 + i as usize * 50;
						let row_size = _to_u8(GLOBAL_DATA, start_idx) % 15;
						let mut row_data = Vec::new();
						for j in 0..row_size {
							let str_len = _to_u8(GLOBAL_DATA, start_idx + 1 + j as usize) % 12;
							let str_data = _to_str(GLOBAL_DATA, start_idx + 16 + j as usize * 12, start_idx + 16 + j as usize * 12 + str_len as usize);
							row_data.push(CustomType0(String::from(str_data)));
						}
						toodee.push_row(row_data);
					}
					
					let view_start = (_to_usize(GLOBAL_DATA, 1200), _to_usize(GLOBAL_DATA, 1208));
					let view_end = (_to_usize(GLOBAL_DATA, 1216), _to_usize(GLOBAL_DATA, 1224));
					let view = toodee.view(view_start, view_end);
					println!("{:?}", view.bounds());
					
					let cells = view.cells();
					for cell in cells {
						println!("{:?}", &*cell);
					}
				},
				
				2 => {
					let t_16 = _to_usize(GLOBAL_DATA, 91);
					let t_17 = _to_usize(GLOBAL_DATA, 99);
					let mut toodee = toodee::TooDee::<CustomType0>::new(t_16, t_17);
					
					let cols_to_insert = _to_u8(GLOBAL_DATA, 1400) % 5;
					for i in 0..cols_to_insert {
						let start_idx = 1401 + i as usize * 100;
						let col_size = _to_u8(GLOBAL_DATA, start_idx) % 20;
						let mut col_data = Vec::new();
						for j in 0..col_size {
							let str_len = _to_u8(GLOBAL_DATA, start_idx + 1 + j as usize * 3) % 15;
							let str_start = start_idx + 30 + j as usize * 15;
							let str_data = _to_str(GLOBAL_DATA, str_start, str_start + str_len as usize);
							col_data.push(CustomType0(String::from(str_data)));
						}
						let insert_idx = _to_usize(GLOBAL_DATA, start_idx + 80);
						toodee.insert_col(insert_idx, col_data);
					}
					
					if let Some(drain) = toodee.pop_col() {
						for item in drain {
							println!("{:?}", item);
						}
					}
					
					let mut rows_mut = toodee.rows_mut();
					for row_mut in rows_mut {
						println!("{:?}", row_mut);
					}
				},
				
				3 => {
					let t_16 = _to_usize(GLOBAL_DATA, 91);
					let t_17 = _to_usize(GLOBAL_DATA, 99);
					let mut slice_data = vec![CustomType0(String::from("test")); t_16 * t_17];
					let view_mut = toodee::TooDeeViewMut::new(t_16, t_17, &mut slice_data);
					
					let view_start = (_to_usize(GLOBAL_DATA, 1500), _to_usize(GLOBAL_DATA, 1508));
					let view_end = (_to_usize(GLOBAL_DATA, 1516), _to_usize(GLOBAL_DATA, 1524));
					let sub_view = view_mut.view(view_start, view_end);
					
					let converted_toodee = toodee::TooDee::from(view_mut);
					println!("{:?}", converted_toodee.num_rows());
					println!("{:?}", converted_toodee.num_cols());
				},
				
				4 => {
					let t_16 = _to_usize(GLOBAL_DATA, 91);
					let t_17 = _to_usize(GLOBAL_DATA, 99);
					let mut toodee = toodee::TooDee::<CustomType0>::new(t_16, t_17);
					
					if toodee.num_cols() > 0 && toodee.num_rows() > 0 {
						let col_idx = _to_usize(GLOBAL_DATA, 1600);
						let mut col_mut = toodee.col_mut(col_idx);
						for item_mut in col_mut {
							println!("{:?}", &*item_mut);
						}
						
						let row_idx = _to_usize(GLOBAL_DATA, 1608);
						let row_ref = &toodee[row_idx];
						println!("{:?}", &*row_ref);
						
						let coord = (_to_usize(GLOBAL_DATA, 1616), _to_usize(GLOBAL_DATA, 1624));
						let cell_ref = &toodee[coord];
						println!("{:?}", &*cell_ref);
					}
				},
				
				5 => {
					let t_16 = _to_usize(GLOBAL_DATA, 91);
					let t_17 = _to_usize(GLOBAL_DATA, 99);
					let mut toodee = toodee::TooDee::<CustomType0>::new(t_16, t_17);
					
					let remove_idx = _to_usize(GLOBAL_DATA, 1650);
					let drain_col = toodee.remove_col(remove_idx);
					for item in drain_col {
						println!("{:?}", item);
					}
					
					let swap_r1 = _to_usize(GLOBAL_DATA, 1658);
					let swap_r2 = _to_usize(GLOBAL_DATA, 1666);
					toodee.swap_rows(swap_r1, swap_r2);
					
					let remove_row_idx = _to_usize(GLOBAL_DATA, 1674);
					let drain_row = toodee.remove_row(remove_row_idx);
					for item in drain_row {
						println!("{:?}", item);
					}
				},
				
				6 => {
					let t_16 = _to_usize(GLOBAL_DATA, 91);
					let t_17 = _to_usize(GLOBAL_DATA, 99);
					let slice_data = vec![CustomType0(String::from("view_test")); t_16 * t_17];
					let view = toodee::TooDeeView::new(t_16, t_17, &slice_data);
					
					let view_start = (_to_usize(GLOBAL_DATA, 1700), _to_usize(GLOBAL_DATA, 1708));
					let view_end = (_to_usize(GLOBAL_DATA, 1716), _to_usize(GLOBAL_DATA, 1724));
					let sub_view = view.view(view_start, view_end);
					
					let rows_iter = sub_view.rows();
					for row in rows_iter {
						println!("{:?}", row);
					}
					
					if sub_view.num_cols() > 0 {
						let col_idx = _to_usize(GLOBAL_DATA, 1732);
						let col_iter = sub_view.col(col_idx);
						for item in col_iter {
							println!("{:?}", &*item);
						}
					}
					
					let converted_toodee = toodee::TooDee::from(view);
					println!("{:?}", converted_toodee.size());
				},
				
				_ => {
					let t_16 = _to_usize(GLOBAL_DATA, 91);
					let t_17 = _to_usize(GLOBAL_DATA, 99);
					let mut toodee = toodee::TooDee::<CustomType0>::new(t_16, t_17);
					
					let mut t_154 = _to_u8(GLOBAL_DATA, 677) % 17;
					let t_155 = _to_str(GLOBAL_DATA, 678, 678 + t_154 as usize);
					let t_156 = String::from(t_155);
					let t_157 = CustomType3(t_156);
					toodee.push_col(t_157);
					
					let mut t_158 = _to_u8(GLOBAL_DATA, 1750) % 17;
					let t_159 = _to_str(GLOBAL_DATA, 1751, 1751 + t_158 as usize);
					let t_160 = String::from(t_159);
					let t_161 = CustomType3(t_160);
					toodee.push_col(t_161);
					
					println!("{:?}", toodee.capacity());
					println!("{:?}", toodee.data().len());
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