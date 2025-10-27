#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use toodee::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

#[derive(Debug, Default, Clone)]
struct CustomType0(String);
#[derive(Debug, Default, Clone)]
struct CustomType1(String);
#[derive(Debug, Default, Clone)]
struct CustomType2(String);

impl core::iter::ExactSizeIterator for CustomType2 {
	
	fn len(&self) -> usize {
		let global_data = get_global_data();
		let GLOBAL_DATA = global_data.first_half;
		let custom_impl_num = _to_usize(GLOBAL_DATA, 8);
		let custom_impl_inst_num = self.0.len();
		let selector = (custom_impl_num + custom_impl_inst_num) % 3;
		if selector == 0{
			panic!("INTENTIONAL PANIC!");
		}
		let GLOBAL_DATA = match selector{
				1 => global_data.first_half,
				_ => global_data.second_half,
		};
		let t_3 = _to_usize(GLOBAL_DATA, 16);
		return t_3;
	}
}

impl core::iter::Iterator for CustomType2 {
	type Item = CustomType0;
	
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
		let t_7 = CustomType0(t_6);
		let t_8 = Some(t_7);
		return t_8;
	}
}

impl core::iter::IntoIterator for CustomType0 {
	type Item = CustomType0;
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

fn main (){
	fuzz_nohook!(|data: &[u8]| {
		if data.len() < 1024 {return;}
		set_global_data(data);
		let global_data = get_global_data();
		let GLOBAL_DATA = global_data.first_half;
		
		let num_operations = _to_u8(GLOBAL_DATA, 0) % 65;
		
		for op_idx in 0..num_operations {
			let op_type = _to_u8(GLOBAL_DATA, 1 + op_idx as usize) % 10;
			
			match op_type {
				0 => {
					let t_0 = _to_usize(GLOBAL_DATA, 100);
					let mut t_1: toodee::TooDee<CustomType0> = toodee::TooDee::with_capacity(t_0);
					let mut t_2 = &mut t_1;
					let mut t_13 = _to_u8(GLOBAL_DATA, 74) % 17;
					let t_14 = _to_str(GLOBAL_DATA, 75, 75 + t_13 as usize);
					let t_15 = String::from(t_14);
					let t_16 = CustomType0(t_15);
					t_2.push_row(vec![t_16]);
					
					let t_17 = &*t_2;
					let row_ref = &t_17[0];
					println!("{:?}", row_ref);
					
					let rows_iter = t_17.rows();
					for row in rows_iter {
						println!("{:?}", row);
					}
					
					let cells_iter = t_17.cells();
					for cell in cells_iter {
						println!("{:?}", cell);
					}
				},
				1 => {
					let cols = _to_usize(GLOBAL_DATA, 108);
					let rows = _to_usize(GLOBAL_DATA, 116);
					let mut toodee: toodee::TooDee<CustomType0> = toodee::TooDee::new(cols, rows);
					
					let mut t_18 = _to_u8(GLOBAL_DATA, 124) % 17;
					let t_19 = _to_str(GLOBAL_DATA, 125, 125 + t_18 as usize);
					let t_20 = String::from(t_19);
					let t_21 = CustomType0(t_20);
					toodee.push_row(vec![t_21]);
					
					let view = toodee.view((_to_usize(GLOBAL_DATA, 142), _to_usize(GLOBAL_DATA, 150)), (_to_usize(GLOBAL_DATA, 158), _to_usize(GLOBAL_DATA, 166)));
					println!("{:?}", view.num_cols());
					
					let view_rows = view.rows();
					for row in view_rows {
						println!("{:?}", row);
					}
				},
				2 => {
					let cols = _to_usize(GLOBAL_DATA, 174);
					let rows = _to_usize(GLOBAL_DATA, 182);
					let mut toodee: toodee::TooDee<CustomType1> = toodee::TooDee::init(cols, rows, CustomType1(String::new()));
					
					let mut t_22 = _to_u8(GLOBAL_DATA, 222) % 17;
					let t_23 = _to_str(GLOBAL_DATA, 223, 223 + t_22 as usize);
					let t_24 = String::from(t_23);
					let t_25 = CustomType1(t_24);
					
					toodee.push_row(vec![t_25]);
					
					let col_iter = toodee.col(_to_usize(GLOBAL_DATA, 240));
					for cell in col_iter {
						println!("{:?}", cell);
					}
					
					let mut view_mut = toodee.view_mut((_to_usize(GLOBAL_DATA, 190), _to_usize(GLOBAL_DATA, 198)), (_to_usize(GLOBAL_DATA, 206), _to_usize(GLOBAL_DATA, 214)));
					let rows_mut_iter = view_mut.rows_mut();
					for row in rows_mut_iter {
						println!("{:?}", row);
					}
				},
				3 => {
					let capacity = _to_usize(GLOBAL_DATA, 248);
					let mut toodee: toodee::TooDee<CustomType1> = toodee::TooDee::with_capacity(capacity);
					
					let vec_size = _to_u8(GLOBAL_DATA, 256) % 65;
					let mut vec_data = Vec::new();
					for i in 0..vec_size {
						let mut str_len = _to_u8(GLOBAL_DATA, 257 + i as usize) % 17;
						let str_data = _to_str(GLOBAL_DATA, 258 + i as usize, 258 + i as usize + str_len as usize);
						vec_data.push(CustomType1(String::from(str_data)));
					}
					
					let cols = _to_usize(GLOBAL_DATA, 324);
					let rows = _to_usize(GLOBAL_DATA, 332);
					
					let custom_toodee: toodee::TooDee<CustomType1> = toodee::TooDee::from_vec(cols, rows, vec_data);
					
					let mut final_toodee = custom_toodee;
					
					let mut t_26 = _to_u8(GLOBAL_DATA, 340) % 17;
					let t_27 = _to_str(GLOBAL_DATA, 341, 341 + t_26 as usize);
					let t_28 = String::from(t_27);
					let t_29 = CustomType1(t_28);
					final_toodee.push_row(vec![t_29]);
					
					let drained_col = final_toodee.pop_col();
					if let Some(drain) = drained_col {
						for elem in drain {
							println!("{:?}", elem);
						}
					}
				},
				4 => {
					let cols = _to_usize(GLOBAL_DATA, 358);
					let rows = _to_usize(GLOBAL_DATA, 366);
					let mut toodee: toodee::TooDee<CustomType0> = toodee::TooDee::new(cols, rows);
					
					let mut t_30 = _to_u8(GLOBAL_DATA, 374) % 17;
					let t_31 = _to_str(GLOBAL_DATA, 375, 375 + t_30 as usize);
					let t_32 = String::from(t_31);
					let t_33 = CustomType0(t_32);
					toodee.push_row(vec![t_33]);
					
					let mut col_mut = toodee.col_mut(_to_usize(GLOBAL_DATA, 392));
					let cell_ref = &mut col_mut[_to_usize(GLOBAL_DATA, 400)];
					println!("{:?}", cell_ref);
					
					toodee.swap_rows(_to_usize(GLOBAL_DATA, 408), _to_usize(GLOBAL_DATA, 416));
					
					let col_ref = toodee.col(_to_usize(GLOBAL_DATA, 424));
					for cell in col_ref {
						println!("{:?}", cell);
					}
				},
				5 => {
					let cols = _to_usize(GLOBAL_DATA, 424);
					let rows = _to_usize(GLOBAL_DATA, 432);
					let mut toodee: toodee::TooDee<CustomType0> = toodee::TooDee::new(cols, rows);
					
					let insert_row_idx = _to_usize(GLOBAL_DATA, 440);
					let mut t_34 = _to_u8(GLOBAL_DATA, 448) % 17;
					let t_35 = _to_str(GLOBAL_DATA, 449, 449 + t_34 as usize);
					let t_36 = String::from(t_35);
					let t_37 = CustomType0(t_36);
					
					toodee.insert_row(insert_row_idx, vec![t_37]);
					
					let remove_col_idx = _to_usize(GLOBAL_DATA, 466);
					let drain_col = toodee.remove_col(remove_col_idx);
					for elem in drain_col {
						println!("{:?}", elem);
					}
					
					let mut t_38 = _to_u8(GLOBAL_DATA, 474) % 17;
					let t_39 = _to_str(GLOBAL_DATA, 475, 475 + t_38 as usize);
					let t_40 = String::from(t_39);
					let t_41 = CustomType0(t_40);
					toodee.push_row(vec![t_41]);
					
					let insert_col_idx = _to_usize(GLOBAL_DATA, 492);
					let col_data = vec![CustomType0::default(); _to_usize(GLOBAL_DATA, 500) % 65];
					toodee.insert_col(insert_col_idx, col_data);
				},
				6 => {
					let cols = _to_usize(GLOBAL_DATA, 492);
					let rows = _to_usize(GLOBAL_DATA, 500);
					let vec_data = vec![CustomType1(String::new()); 15];
					let view = toodee::TooDeeView::new(cols, rows, &vec_data);
					
					let converted_toodee: toodee::TooDee<CustomType1> = toodee::TooDee::from(view);
					let mut final_toodee = converted_toodee;
					
					let mut t_42 = _to_u8(GLOBAL_DATA, 508) % 17;
					let t_43 = _to_str(GLOBAL_DATA, 509, 509 + t_42 as usize);
					let t_44 = String::from(t_43);
					let t_45 = CustomType1(t_44);
					final_toodee.push_row(vec![t_45]);
					
					let rows_mut_iter = final_toodee.rows_mut();
					for row in rows_mut_iter {
						println!("{:?}", row);
					}
					
					let mut mutable_view = final_toodee.view_mut((_to_usize(GLOBAL_DATA, 526), _to_usize(GLOBAL_DATA, 534)), (_to_usize(GLOBAL_DATA, 542), _to_usize(GLOBAL_DATA, 550)));
					let col_mut = mutable_view.col_mut(_to_usize(GLOBAL_DATA, 558));
					for cell in col_mut {
						println!("{:?}", cell);
					}
				},
				7 => {
					let mut data_vec = Vec::new();
					let vec_size = _to_u8(GLOBAL_DATA, 566) % 65;
					for i in 0..vec_size {
						data_vec.push(CustomType1(String::new()));
					}
					
					let cols = _to_usize(GLOBAL_DATA, 574);
					let rows = _to_usize(GLOBAL_DATA, 582);
					let boxed_slice = data_vec.into_boxed_slice();
					let mut toodee: toodee::TooDee<CustomType1> = toodee::TooDee::from_box(cols, rows, boxed_slice);
					
					let mut t_46 = _to_u8(GLOBAL_DATA, 590) % 17;
					let t_47 = _to_str(GLOBAL_DATA, 591, 591 + t_46 as usize);
					let t_48 = String::from(t_47);
					let t_49 = CustomType1(t_48);
					toodee.push_row(vec![t_49]);
					
					let coord = (_to_usize(GLOBAL_DATA, 608), _to_usize(GLOBAL_DATA, 616));
					let cell_ref = &toodee[coord];
					println!("{:?}", cell_ref);
					
					let push_col_data = vec![CustomType1(String::new()); _to_usize(GLOBAL_DATA, 624) % 65];
					toodee.push_col(push_col_data);
					
					let row_ref = &toodee[_to_usize(GLOBAL_DATA, 632)];
					println!("{:?}", row_ref);
				},
				8 => {
					let cols = _to_usize(GLOBAL_DATA, 640);
					let rows = _to_usize(GLOBAL_DATA, 648);
					let mut toodee: toodee::TooDee<CustomType1> = toodee::TooDee::init(cols, rows, CustomType1(String::new()));
					
					if let Some(drain) = toodee.pop_row() {
						for elem in drain {
							println!("{:?}", elem);
						}
					}
					
					let remove_row_idx = _to_usize(GLOBAL_DATA, 656);
					let drain_row = toodee.remove_row(remove_row_idx);
					for elem in drain_row {
						println!("{:?}", elem);
					}
					
					let row_pair = toodee.row_pair_mut(_to_usize(GLOBAL_DATA, 664), _to_usize(GLOBAL_DATA, 672));
					println!("{:?}", row_pair.0);
					println!("{:?}", row_pair.1);
					
					let mutable_cell = &mut toodee[(_to_usize(GLOBAL_DATA, 680), _to_usize(GLOBAL_DATA, 688))];
					println!("{:?}", mutable_cell);
				},
				_ => {
					let cols = _to_usize(GLOBAL_DATA, 696);
					let rows = _to_usize(GLOBAL_DATA, 704);
					let mut toodee: toodee::TooDee<CustomType1> = toodee::TooDee::new(cols, rows);
					
					let fill_value = CustomType1(String::from("fill"));
					toodee.fill(&fill_value);
					
					let data_ref = toodee.data();
					for elem in data_ref {
						println!("{:?}", elem);
					}
					
					let data_mut_ref = toodee.data_mut();
					for elem in data_mut_ref {
						println!("{:?}", elem);
					}
					
					let size = toodee.size();
					println!("{:?}", size);
					
					let bounds = toodee.bounds();
					println!("{:?}", bounds);
					
					let is_empty = toodee.is_empty();
					println!("{:?}", is_empty);
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