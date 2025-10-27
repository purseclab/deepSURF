#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use toodee::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

#[derive(Clone, Debug)]
struct CustomType0(String);

fn main (){
	fuzz_nohook!(|data: &[u8]| {
		if data.len() < 2400 {return;}
		set_global_data(data);
		let global_data = get_global_data();
		let GLOBAL_DATA = global_data.first_half;
		
		let num_ops = _to_u8(GLOBAL_DATA, 0) % 65;
		
		for op_idx in 0..num_ops {
			let base_offset = 16 + (op_idx as usize * 32);
			if base_offset + 32 > GLOBAL_DATA.len() { break; }
			
			let op_type = _to_u8(GLOBAL_DATA, base_offset) % 10;
			
			match op_type {
				0 => {
					let constructor_choice = _to_u8(GLOBAL_DATA, base_offset + 1) % 6;
					let rows = _to_usize(GLOBAL_DATA, base_offset + 2);
					let cols = _to_usize(GLOBAL_DATA, base_offset + 10);
					
					let mut toodee = match constructor_choice {
						0 => toodee::TooDee::init(cols, rows, CustomType0(String::from("init"))),
						1 => {
							let mut td = toodee::TooDee::with_capacity(cols.wrapping_mul(rows));
							for _ in 0..rows {
								let row_data: Vec<CustomType0> = (0..cols).map(|i| CustomType0(format!("cap_{}", i))).collect();
								td.push_row(row_data);
							}
							td
						},
						2 => {
							let mut vec_data = Vec::with_capacity(cols.wrapping_mul(rows));
							for i in 0..cols.wrapping_mul(rows) {
								vec_data.push(CustomType0(format!("vec_{}", i)));
							}
							toodee::TooDee::from_vec(cols, rows, vec_data)
						},
						3 => {
							let slice_data: Box<[CustomType0]> = (0..cols.wrapping_mul(rows))
								.map(|i| CustomType0(format!("box_{}", i)))
								.collect::<Vec<_>>()
								.into_boxed_slice();
							toodee::TooDee::from_box(cols, rows, slice_data)
						},
						4 => {
							let view_data: Vec<CustomType0> = (0..cols.wrapping_mul(rows))
								.map(|i| CustomType0(format!("view_{}", i)))
								.collect();
							let temp_toodee = toodee::TooDee::from_vec(cols, rows, view_data);
							let view = temp_toodee.view((0, 0), (cols, rows));
							toodee::TooDee::from(view)
						},
						_ => {
							let mut data_vec: Vec<CustomType0> = (0..cols.wrapping_mul(rows))
								.map(|i| CustomType0(format!("viewmut_{}", i)))
								.collect();
							let mut temp_toodee = toodee::TooDee::from_vec(cols, rows, data_vec);
							let view_mut = temp_toodee.view_mut((0, 0), (cols, rows));
							toodee::TooDee::from(view_mut)
						}
					};
					
					let view_ops = _to_u8(GLOBAL_DATA, base_offset + 18) % 3;
					for _ in 0..view_ops {
						let start_coord = (_to_usize(GLOBAL_DATA, base_offset + 19), _to_usize(GLOBAL_DATA, base_offset + 27));
						let end_coord = (_to_usize(GLOBAL_DATA, base_offset + 35), _to_usize(GLOBAL_DATA, base_offset + 43));
						let view = toodee.view(start_coord, end_coord);
						println!("{:?}", view.num_cols());
						println!("{:?}", view.num_rows());
						
						let view_coord = (_to_usize(GLOBAL_DATA, base_offset + 51), _to_usize(GLOBAL_DATA, base_offset + 59));
						let cell_ref = view.index(view_coord);
						println!("{:?}", cell_ref);
					}
					
					let start_coord = (_to_usize(GLOBAL_DATA, base_offset + 67), _to_usize(GLOBAL_DATA, base_offset + 75));
					let end_coord = (_to_usize(GLOBAL_DATA, base_offset + 83), _to_usize(GLOBAL_DATA, base_offset + 91));
					let mut view_mut = toodee.view_mut(start_coord, end_coord);
					
					let target_coord = (_to_usize(GLOBAL_DATA, base_offset + 99), _to_usize(GLOBAL_DATA, base_offset + 107));
					let result = view_mut.index_mut(target_coord);
					println!("{:?}", result);
				},
				1 => {
					let rows = _to_usize(GLOBAL_DATA, base_offset + 1);
					let cols = _to_usize(GLOBAL_DATA, base_offset + 9);
					let mut toodee = toodee::TooDee::init(cols, rows, CustomType0(String::from("test")));
					
					let row_idx = _to_usize(GLOBAL_DATA, base_offset + 17);
					let coord_idx = (_to_usize(GLOBAL_DATA, base_offset + 25), _to_usize(GLOBAL_DATA, base_offset + 33));
					
					let row_result = toodee.index_mut(row_idx);
					println!("{:?}", row_result);
					
					let coord_result = toodee.index_mut(coord_idx);
					println!("{:?}", coord_result);
					
					let bounds = toodee.bounds();
					println!("{:?}", bounds);
					
					let cells_iter = toodee.cells();
					for cell in cells_iter.take(5) {
						println!("{:?}", cell);
					}
				},
				2 => {
					let rows = _to_usize(GLOBAL_DATA, base_offset + 1);
					let cols = _to_usize(GLOBAL_DATA, base_offset + 9);
					let mut toodee = toodee::TooDee::init(cols, rows, CustomType0(String::from("rows")));
					
					let mut rows_mut = toodee.rows_mut();
					if let Some(first_row) = rows_mut.next() {
						println!("{:?}", first_row);
					}
					if let Some(last_row) = rows_mut.last() {
						println!("{:?}", last_row);
					}
					
					let rows_read = toodee.rows();
					for (i, row) in rows_read.enumerate().take(3) {
						println!("{:?}", row);
					}
				},
				3 => {
					let rows = _to_usize(GLOBAL_DATA, base_offset + 1);
					let cols = _to_usize(GLOBAL_DATA, base_offset + 9);
					let mut toodee = toodee::TooDee::init(cols, rows, CustomType0(String::from("cols")));
					
					let col_idx = _to_usize(GLOBAL_DATA, base_offset + 17);
					let mut col_mut = toodee.col_mut(col_idx);
					if let Some(first_elem) = col_mut.next() {
						println!("{:?}", first_elem);
					}
					if let Some(nth_elem) = col_mut.nth(_to_usize(GLOBAL_DATA, base_offset + 25)) {
						println!("{:?}", nth_elem);
					}
					
					let col_read = toodee.col(col_idx);
					for cell in col_read.take(3) {
						println!("{:?}", cell);
					}
				},
				4 => {
					let rows = _to_usize(GLOBAL_DATA, base_offset + 1);
					let cols = _to_usize(GLOBAL_DATA, base_offset + 9);
					let mut toodee = toodee::TooDee::init(cols, rows, CustomType0(String::from("drain")));
					
					if let Some(drain_col) = toodee.pop_col() {
						for item in drain_col {
							println!("{:?}", item);
						}
					}
					
					let remove_col_idx = _to_usize(GLOBAL_DATA, base_offset + 17);
					let drain_col_2 = toodee.remove_col(remove_col_idx);
					for item in drain_col_2 {
						println!("{:?}", item);
					}
					
					let pop_row = toodee.pop_row();
					if let Some(row_drain) = pop_row {
						for item in row_drain {
							println!("{:?}", item);
						}
					}
				},
				5 => {
					let rows = _to_usize(GLOBAL_DATA, base_offset + 1);
					let cols = _to_usize(GLOBAL_DATA, base_offset + 9);
					let mut toodee = toodee::TooDee::init(cols, rows, CustomType0(String::from("view")));
					
					let start_coord = (_to_usize(GLOBAL_DATA, base_offset + 17), _to_usize(GLOBAL_DATA, base_offset + 25));
					let end_coord = (_to_usize(GLOBAL_DATA, base_offset + 33), _to_usize(GLOBAL_DATA, base_offset + 41));
					
					let view = toodee.view(start_coord, end_coord);
					println!("{:?}", view.num_cols());
					println!("{:?}", view.num_rows());
					
					let view_coord = (_to_usize(GLOBAL_DATA, base_offset + 49), _to_usize(GLOBAL_DATA, base_offset + 57));
					let cell_ref = view.index(view_coord);
					println!("{:?}", cell_ref);
					
					let size = view.size();
					println!("{:?}", size);
					
					let is_empty = view.is_empty();
					println!("{:?}", is_empty);
				},
				6 => {
					let rows = _to_usize(GLOBAL_DATA, base_offset + 1);
					let cols = _to_usize(GLOBAL_DATA, base_offset + 9);
					let mut toodee = toodee::TooDee::init(cols, rows, CustomType0(String::from("swap")));
					
					let r1 = _to_usize(GLOBAL_DATA, base_offset + 17);
					let r2 = _to_usize(GLOBAL_DATA, base_offset + 25);
					toodee.swap_rows(r1, r2);
					
					let c1 = _to_usize(GLOBAL_DATA, base_offset + 33);
					let c2 = _to_usize(GLOBAL_DATA, base_offset + 41);
					toodee.swap_cols(c1, c2);
					
					let start_coord = (_to_usize(GLOBAL_DATA, base_offset + 49), _to_usize(GLOBAL_DATA, base_offset + 57));
					let end_coord = (_to_usize(GLOBAL_DATA, base_offset + 65), _to_usize(GLOBAL_DATA, base_offset + 73));
					let mut view_mut = toodee.view_mut(start_coord, end_coord);
					
					let target_coord = (_to_usize(GLOBAL_DATA, base_offset + 81), _to_usize(GLOBAL_DATA, base_offset + 89));
					let result = view_mut.index_mut(target_coord);
					println!("{:?}", result);
				},
				7 => {
					let rows = _to_usize(GLOBAL_DATA, base_offset + 1);
					let cols = _to_usize(GLOBAL_DATA, base_offset + 9);
					let mut toodee = toodee::TooDee::init(cols, rows, CustomType0(String::from("nested")));
					
					let start1 = (_to_usize(GLOBAL_DATA, base_offset + 17), _to_usize(GLOBAL_DATA, base_offset + 25));
					let end1 = (_to_usize(GLOBAL_DATA, base_offset + 33), _to_usize(GLOBAL_DATA, base_offset + 41));
					let mut view1 = toodee.view_mut(start1, end1);
					
					let start2 = (_to_usize(GLOBAL_DATA, base_offset + 49), _to_usize(GLOBAL_DATA, base_offset + 57));
					let end2 = (_to_usize(GLOBAL_DATA, base_offset + 65), _to_usize(GLOBAL_DATA, base_offset + 73));
					let mut view2 = view1.view_mut(start2, end2);
					
					let final_coord = (_to_usize(GLOBAL_DATA, base_offset + 81), _to_usize(GLOBAL_DATA, base_offset + 89));
					let result = view2.index_mut(final_coord);
					println!("{:?}", result);
				},
				8 => {
					let rows = _to_usize(GLOBAL_DATA, base_offset + 1);
					let cols = _to_usize(GLOBAL_DATA, base_offset + 9);
					let mut toodee = toodee::TooDee::init(cols, rows, CustomType0(String::from("insert")));
					
					let insert_row_data: Vec<CustomType0> = (0..cols).map(|i| CustomType0(format!("new_row_{}", i))).collect();
					let insert_row_idx = _to_usize(GLOBAL_DATA, base_offset + 17);
					toodee.insert_row(insert_row_idx, insert_row_data);
					
					let insert_col_data: Vec<CustomType0> = (0..rows).map(|i| CustomType0(format!("new_col_{}", i))).collect();
					let insert_col_idx = _to_usize(GLOBAL_DATA, base_offset + 25);
					toodee.insert_col(insert_col_idx, insert_col_data);
					
					let remove_row_idx = _to_usize(GLOBAL_DATA, base_offset + 33);
					let removed_row = toodee.remove_row(remove_row_idx);
					for item in removed_row {
						println!("{:?}", item);
					}
				},
				_ => {
					let rows = _to_usize(GLOBAL_DATA, base_offset + 1);
					let cols = _to_usize(GLOBAL_DATA, base_offset + 9);
					let mut toodee = toodee::TooDee::init(cols, rows, CustomType0(String::from("fill")));
					
					let fill_value = CustomType0(String::from("filled"));
					toodee.fill(&fill_value);
					
					let cells_mut = toodee.cells_mut();
					for cell in cells_mut.take(5) {
						println!("{:?}", cell);
					}
					
					let capacity = toodee.capacity();
					println!("{:?}", capacity);
					
					toodee.clear();
					println!("{:?}", toodee.num_cols());
					println!("{:?}", toodee.num_rows());
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