#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use toodee::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

#[derive(Debug, Clone, Default)]
struct CustomType0(String);

fn main (){
	fuzz_nohook!(|data: &[u8]| {
		if data.len() < 200 {return;}
		set_global_data(data);
		let global_data = get_global_data();
		let GLOBAL_DATA = global_data.first_half;
		
		let num_operations = _to_usize(GLOBAL_DATA, 0) % 65;
		
		for op_idx in 0..num_operations {
			let base_offset = 8 + (op_idx * 56);
			if base_offset + 56 > GLOBAL_DATA.len() { break; }
			
			let operation = _to_u8(GLOBAL_DATA, base_offset) % 10;
			
			match operation {
				0 => {
					let constructor_choice = _to_u8(GLOBAL_DATA, base_offset + 1) % 7;
					let mut toodee = match constructor_choice {
						0 => {
							let capacity = _to_usize(GLOBAL_DATA, base_offset + 2);
							toodee::TooDee::<CustomType0>::with_capacity(capacity)
						},
						1 => {
							let cols = _to_usize(GLOBAL_DATA, base_offset + 2);
							let rows = _to_usize(GLOBAL_DATA, base_offset + 10);
							toodee::TooDee::<CustomType0>::new(cols, rows)
						},
						2 => {
							let cols = _to_usize(GLOBAL_DATA, base_offset + 2);
							let rows = _to_usize(GLOBAL_DATA, base_offset + 10);
							let init_val = CustomType0(String::from("test"));
							toodee::TooDee::init(cols, rows, init_val)
						},
						3 => {
							let vec_size = _to_usize(GLOBAL_DATA, base_offset + 2) % 65;
							let mut vec = Vec::with_capacity(vec_size);
							for i in 0..vec_size {
								vec.push(CustomType0(format!("item_{}", i)));
							}
							let cols = _to_usize(GLOBAL_DATA, base_offset + 10);
							let rows = _to_usize(GLOBAL_DATA, base_offset + 18);
							toodee::TooDee::from_vec(cols, rows, vec)
						},
						4 => {
							let slice_size = _to_usize(GLOBAL_DATA, base_offset + 2) % 65;
							let mut vec = Vec::with_capacity(slice_size);
							for i in 0..slice_size {
								vec.push(CustomType0(format!("box_{}", i)));
							}
							let boxed = vec.into_boxed_slice();
							let cols = _to_usize(GLOBAL_DATA, base_offset + 10);
							let rows = _to_usize(GLOBAL_DATA, base_offset + 18);
							toodee::TooDee::from_box(cols, rows, boxed)
						},
						5 => {
							let view_size = _to_usize(GLOBAL_DATA, base_offset + 2) % 65;
							let mut slice = Vec::with_capacity(view_size);
							for i in 0..view_size {
								slice.push(CustomType0(format!("view_{}", i)));
							}
							let cols = _to_usize(GLOBAL_DATA, base_offset + 10);
							let rows = _to_usize(GLOBAL_DATA, base_offset + 18);
							let view = toodee::TooDeeView::new(cols, rows, &slice);
							toodee::TooDee::from(view)
						},
						_ => {
							let mut viewmut_size = _to_usize(GLOBAL_DATA, base_offset + 2) % 65;
							let mut slice = Vec::with_capacity(viewmut_size);
							for i in 0..viewmut_size {
								slice.push(CustomType0(format!("viewmut_{}", i)));
							}
							let cols = _to_usize(GLOBAL_DATA, base_offset + 10);
							let rows = _to_usize(GLOBAL_DATA, base_offset + 18);
							let viewmut = toodee::TooDeeViewMut::new(cols, rows, &mut slice);
							toodee::TooDee::from(viewmut)
						}
					};
					
					if toodee.num_cols() > 0 {
						let col_idx = _to_usize(GLOBAL_DATA, base_offset + 26);
						let mut col_iter = toodee.col(col_idx);
						let nth_idx = _to_usize(GLOBAL_DATA, base_offset + 34);
						let result = col_iter.nth(nth_idx);
						if let Some(val) = result {
							println!("{:?}", *val);
						}
					}
					
					let drain_choice = _to_u8(GLOBAL_DATA, base_offset + 42) % 3;
					match drain_choice {
						0 => {
							if let Some(mut drain_col) = toodee.pop_col() {
								let drain_nth = _to_usize(GLOBAL_DATA, base_offset + 43);
								if let Some(item) = drain_col.nth(drain_nth) {
									println!("{:?}", item);
								}
							}
						},
						1 => {
							if toodee.num_cols() > 0 {
								let remove_col_idx = _to_usize(GLOBAL_DATA, base_offset + 43);
								let mut drain_col = toodee.remove_col(remove_col_idx);
								let drain_nth = _to_usize(GLOBAL_DATA, base_offset + 51);
								if let Some(item) = drain_col.nth(drain_nth) {
									println!("{:?}", item);
								}
							}
						},
						_ => {
							let idx = _to_usize(GLOBAL_DATA, base_offset + 43);
							if idx < toodee.data().len() {
								let elem_ref = &toodee.data()[idx];
								println!("{:?}", *elem_ref);
							}
						}
					}
				},
				1 => {
					let mut toodee = toodee::TooDee::<CustomType0>::with_capacity(_to_usize(GLOBAL_DATA, base_offset + 1));
					let mut rows_iter = toodee.rows();
					let nth_rows = _to_usize(GLOBAL_DATA, base_offset + 9);
					let row_result = rows_iter.nth(nth_rows);
					if let Some(row) = row_result {
						println!("{:?}", row.len());
						for (i, elem) in row.iter().enumerate() {
							println!("{:?}", *elem);
							if i >= 5 { break; }
						}
					}
					
					if toodee.num_cols() > 0 {
						let col_idx = _to_usize(GLOBAL_DATA, base_offset + 17);
						let mut col_iter = toodee.col(col_idx);
						let nth_col = _to_usize(GLOBAL_DATA, base_offset + 25);
						let col_result = col_iter.nth(nth_col);
						if let Some(cell) = col_result {
							println!("{:?}", *cell);
						}
					}
					
					let view_start_col = _to_usize(GLOBAL_DATA, base_offset + 33);
					let view_start_row = _to_usize(GLOBAL_DATA, base_offset + 41);
					let view_end_col = _to_usize(GLOBAL_DATA, base_offset + 49);
					let view_end_row = _to_usize(GLOBAL_DATA, base_offset + 57);
					if view_start_col < view_end_col && view_start_row < view_end_row {
						let view = toodee.view((view_start_col, view_start_row), (view_end_col, view_end_row));
						println!("{:?}", view.num_cols());
					}
				},
				2 => {
					let cols = _to_usize(GLOBAL_DATA, base_offset + 1);
					let rows = _to_usize(GLOBAL_DATA, base_offset + 9);
					let mut toodee = toodee::TooDee::<CustomType0>::new(cols, rows);
					
					let mut rows_mut_iter = toodee.rows_mut();
					let nth_row_mut = _to_usize(GLOBAL_DATA, base_offset + 17);
					let row_mut_result = rows_mut_iter.nth(nth_row_mut);
					if let Some(row_mut) = row_mut_result {
						println!("{:?}", row_mut.len());
						for elem in row_mut.iter() {
							println!("{:?}", *elem);
						}
					}
					
					if toodee.num_cols() > 0 {
						let col_mut_idx = _to_usize(GLOBAL_DATA, base_offset + 25);
						let mut col_mut_iter = toodee.col_mut(col_mut_idx);
						let nth_col_mut = _to_usize(GLOBAL_DATA, base_offset + 33);
						let col_mut_result = col_mut_iter.nth(nth_col_mut);
						if let Some(cell_mut) = col_mut_result {
							println!("{:?}", *cell_mut);
						}
					}
					
					let swap_r1 = _to_usize(GLOBAL_DATA, base_offset + 41);
					let swap_r2 = _to_usize(GLOBAL_DATA, base_offset + 49);
					if toodee.num_rows() > 1 {
						toodee.swap_rows(swap_r1, swap_r2);
					}
				},
				3 => {
					let view_cols = _to_usize(GLOBAL_DATA, base_offset + 1);
					let view_rows = _to_usize(GLOBAL_DATA, base_offset + 9);
					let slice_size = _to_usize(GLOBAL_DATA, base_offset + 17) % 65;
					let mut slice = Vec::with_capacity(slice_size);
					for i in 0..slice_size {
						slice.push(CustomType0(format!("view_test_{}", i)));
					}
					
					let view = toodee::TooDeeView::new(view_cols, view_rows, &slice);
					let mut rows_view = view.rows();
					let nth_view_row = _to_usize(GLOBAL_DATA, base_offset + 25);
					let view_row_result = rows_view.nth(nth_view_row);
					if let Some(view_row) = view_row_result {
						println!("{:?}", view_row.len());
						for elem in view_row.iter() {
							println!("{:?}", *elem);
						}
					}
					
					if view.num_cols() > 0 {
						let view_col_idx = _to_usize(GLOBAL_DATA, base_offset + 33);
						let mut view_col = view.col(view_col_idx);
						let nth_view_col = _to_usize(GLOBAL_DATA, base_offset + 41);
						let view_col_result = view_col.nth(nth_view_col);
						if let Some(view_cell) = view_col_result {
							println!("{:?}", *view_cell);
						}
					}
					
					let index_row = _to_usize(GLOBAL_DATA, base_offset + 49);
					if index_row < view.num_rows() {
						let row_slice = &view[index_row];
						for elem in row_slice.iter() {
							println!("{:?}", *elem);
						}
					}
				},
				4 => {
					let viewmut_cols = _to_usize(GLOBAL_DATA, base_offset + 1);
					let viewmut_rows = _to_usize(GLOBAL_DATA, base_offset + 9);
					let slice_size = _to_usize(GLOBAL_DATA, base_offset + 17) % 65;
					let mut slice = Vec::with_capacity(slice_size);
					for i in 0..slice_size {
						slice.push(CustomType0(format!("viewmut_test_{}", i)));
					}
					
					let mut viewmut = toodee::TooDeeViewMut::new(viewmut_cols, viewmut_rows, &mut slice);
					let mut rows_viewmut = viewmut.rows_mut();
					let nth_viewmut_row = _to_usize(GLOBAL_DATA, base_offset + 25);
					let viewmut_row_result = rows_viewmut.nth(nth_viewmut_row);
					if let Some(viewmut_row) = viewmut_row_result {
						println!("{:?}", viewmut_row.len());
						for elem in viewmut_row.iter() {
							println!("{:?}", *elem);
						}
					}
					
					if viewmut.num_cols() > 0 {
						let viewmut_col_idx = _to_usize(GLOBAL_DATA, base_offset + 33);
						let mut viewmut_col = viewmut.col_mut(viewmut_col_idx);
						let nth_viewmut_col = _to_usize(GLOBAL_DATA, base_offset + 41);
						let viewmut_col_result = viewmut_col.nth(nth_viewmut_col);
						if let Some(viewmut_cell) = viewmut_col_result {
							println!("{:?}", *viewmut_cell);
						}
					}
					
					let mut viewmut_bounds = viewmut.bounds();
					println!("{:?}", viewmut_bounds);
				},
				5 => {
					let cols = _to_usize(GLOBAL_DATA, base_offset + 1);
					let rows = _to_usize(GLOBAL_DATA, base_offset + 9);
					let mut source_toodee = toodee::TooDee::<CustomType0>::new(cols, rows);
					
					let target_cols = _to_usize(GLOBAL_DATA, base_offset + 17);
					let target_rows = _to_usize(GLOBAL_DATA, base_offset + 25);
					let mut target_toodee = toodee::TooDee::<CustomType0>::new(target_cols, target_rows);
					
					if source_toodee.num_cols() > 0 && source_toodee.num_rows() > 0 {
						target_toodee.clone_from_toodee(&source_toodee);
					}
					
					let push_vec_size = _to_usize(GLOBAL_DATA, base_offset + 33) % 65;
					let mut push_vec = Vec::with_capacity(push_vec_size);
					for i in 0..push_vec_size {
						push_vec.push(CustomType0(format!("push_{}", i)));
					}
					if !push_vec.is_empty() && target_toodee.num_cols() == push_vec.len() {
						target_toodee.push_row(push_vec);
					}
				},
				6 => {
					let vec_size = _to_usize(GLOBAL_DATA, base_offset + 1) % 65;
					let mut vec = Vec::with_capacity(vec_size);
					for i in 0..vec_size {
						vec.push(CustomType0(format!("cells_{}", i)));
					}
					let cols = _to_usize(GLOBAL_DATA, base_offset + 9);
					let rows = _to_usize(GLOBAL_DATA, base_offset + 17);
					let toodee = toodee::TooDee::from_vec(cols, rows, vec);
					
					let mut cells_iter = toodee.cells();
					let cells_nth = _to_usize(GLOBAL_DATA, base_offset + 25);
					if let Some(cell) = cells_iter.nth(cells_nth) {
						println!("{:?}", *cell);
					}
					
					let last_cell = cells_iter.last();
					if let Some(cell) = last_cell {
						println!("{:?}", *cell);
					}
				},
				7 => {
					let cols = _to_usize(GLOBAL_DATA, base_offset + 1);
					let rows = _to_usize(GLOBAL_DATA, base_offset + 9);
					let mut toodee = toodee::TooDee::<CustomType0>::new(cols, rows);
					
					let insert_row_idx = _to_usize(GLOBAL_DATA, base_offset + 17);
					let insert_vec_size = _to_usize(GLOBAL_DATA, base_offset + 25) % 65;
					let mut insert_vec = Vec::with_capacity(insert_vec_size);
					for i in 0..insert_vec_size {
						insert_vec.push(CustomType0(format!("insert_{}", i)));
					}
					if !insert_vec.is_empty() && toodee.num_cols() == insert_vec.len() && insert_row_idx <= toodee.num_rows() {
						toodee.insert_row(insert_row_idx, insert_vec);
					}
					
					if toodee.num_rows() > 0 {
						let remove_row_idx = _to_usize(GLOBAL_DATA, base_offset + 33);
						if remove_row_idx < toodee.num_rows() {
							let mut drain_row = toodee.remove_row(remove_row_idx);
							let drain_nth = _to_usize(GLOBAL_DATA, base_offset + 41);
							if let Some(item) = drain_row.nth(drain_nth) {
								println!("{:?}", item);
							}
						}
					}
				},
				8 => {
					let cols = _to_usize(GLOBAL_DATA, base_offset + 1);
					let rows = _to_usize(GLOBAL_DATA, base_offset + 9);
					let init_val = CustomType0(format!("init_{}", _to_usize(GLOBAL_DATA, base_offset + 17)));
					let toodee = toodee::TooDee::init(cols, rows, init_val);
					
					let coord_col = _to_usize(GLOBAL_DATA, base_offset + 25);
					let coord_row = _to_usize(GLOBAL_DATA, base_offset + 33);
					if coord_col < toodee.num_cols() && coord_row < toodee.num_rows() {
						let cell_ref = &toodee[(coord_col, coord_row)];
						println!("{:?}", *cell_ref);
					}
					
					let start_col = _to_usize(GLOBAL_DATA, base_offset + 41);
					let start_row = _to_usize(GLOBAL_DATA, base_offset + 49);
					let end_col = start_col + 1;
					let end_row = start_row + 1;
					if end_col <= toodee.num_cols() && end_row <= toodee.num_rows() {
						let sub_view = toodee.view((start_col, start_row), (end_col, end_row));
						let bounds = sub_view.bounds();
						println!("{:?}", bounds);
					}
				},
				_ => {
					let capacity = _to_usize(GLOBAL_DATA, base_offset + 1);
					let mut toodee = toodee::TooDee::<CustomType0>::with_capacity(capacity);
					
					let fill_val = CustomType0(format!("fill_{}", _to_usize(GLOBAL_DATA, base_offset + 9)));
					if toodee.num_cols() > 0 && toodee.num_rows() > 0 {
						toodee.fill(&fill_val);
					}
					
					let col_vec_size = _to_usize(GLOBAL_DATA, base_offset + 17) % 65;
					let mut col_vec = Vec::with_capacity(col_vec_size);
					for i in 0..col_vec_size {
						col_vec.push(CustomType0(format!("col_{}", i)));
					}
					if !col_vec.is_empty() && toodee.num_rows() == col_vec.len() {
						toodee.push_col(col_vec);
					}
					
					if toodee.num_cols() > 1 {
						let swap_c1 = _to_usize(GLOBAL_DATA, base_offset + 25);
						let swap_c2 = _to_usize(GLOBAL_DATA, base_offset + 33);
						toodee.swap_cols(swap_c1, swap_c2);
					}
					
					println!("{:?}", toodee.capacity());
					toodee.shrink_to_fit();
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