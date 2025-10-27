#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use toodee::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

#[derive(Debug)]
struct CustomType1(String);

fn main (){
	fuzz_nohook!(|data: &[u8]| {
		if data.len() < 2048 {return;}
		set_global_data(data);
		let global_data = get_global_data();
		let GLOBAL_DATA = global_data.first_half;
		
		let num_ops = _to_u8(GLOBAL_DATA, 0) % 65;
		let mut data_offset = 1;
		
		let constructor_selector = _to_u8(GLOBAL_DATA, data_offset) % 7;
		data_offset += 1;
		
		let mut t_1 = match constructor_selector {
			0 => {
				let t_0 = _to_usize(GLOBAL_DATA, data_offset);
				data_offset += 8;
				toodee::TooDee::with_capacity(t_0)
			},
			1 => {
				let cols = _to_usize(GLOBAL_DATA, data_offset);
				data_offset += 8;
				let rows = _to_usize(GLOBAL_DATA, data_offset);
				data_offset += 8;
				toodee::TooDee::new(cols, rows)
			},
			2 => {
				let cols = _to_usize(GLOBAL_DATA, data_offset);
				data_offset += 8;
				let rows = _to_usize(GLOBAL_DATA, data_offset);
				data_offset += 8;
				let mut t_14 = _to_u8(GLOBAL_DATA, data_offset) % 17;
				data_offset += 1;
				let t_15 = _to_str(GLOBAL_DATA, data_offset, data_offset + t_14 as usize);
				data_offset += t_14 as usize;
				let t_16 = String::from(t_15);
				toodee::TooDee::init(cols, rows, t_16)
			},
			3 => {
				let cols = _to_usize(GLOBAL_DATA, data_offset);
				data_offset += 8;
				let rows = _to_usize(GLOBAL_DATA, data_offset);
				data_offset += 8;
				let vec_size = _to_usize(GLOBAL_DATA, data_offset) % 65;
				data_offset += 8;
				let mut vec_data = Vec::new();
				for i in 0..vec_size {
					let mut str_len = _to_u8(GLOBAL_DATA, data_offset) % 17;
					data_offset += 1;
					let str_val = _to_str(GLOBAL_DATA, data_offset, data_offset + str_len as usize);
					data_offset += str_len as usize;
					vec_data.push(String::from(str_val));
				}
				toodee::TooDee::from_vec(cols, rows, vec_data)
			},
			4 => {
				let cols = _to_usize(GLOBAL_DATA, data_offset);
				data_offset += 8;
				let rows = _to_usize(GLOBAL_DATA, data_offset);
				data_offset += 8;
				let box_size = _to_usize(GLOBAL_DATA, data_offset) % 65;
				data_offset += 8;
				let mut box_data = Vec::new();
				for i in 0..box_size {
					let mut str_len = _to_u8(GLOBAL_DATA, data_offset) % 17;
					data_offset += 1;
					let str_val = _to_str(GLOBAL_DATA, data_offset, data_offset + str_len as usize);
					data_offset += str_len as usize;
					box_data.push(String::from(str_val));
				}
				toodee::TooDee::from_box(cols, rows, box_data.into_boxed_slice())
			},
			5 => {
				let view_cols = _to_usize(GLOBAL_DATA, data_offset);
				data_offset += 8;
				let view_rows = _to_usize(GLOBAL_DATA, data_offset);
				data_offset += 8;
				let view_size = view_cols * view_rows;
				let mut view_data = Vec::new();
				for i in 0..view_size {
					let mut str_len = _to_u8(GLOBAL_DATA, data_offset) % 17;
					data_offset += 1;
					let str_val = _to_str(GLOBAL_DATA, data_offset, data_offset + str_len as usize);
					data_offset += str_len as usize;
					view_data.push(String::from(str_val));
				}
				let temp_view = toodee::TooDeeView::new(view_cols, view_rows, &view_data);
				toodee::TooDee::from(temp_view)
			},
			_ => {
				let view_cols = _to_usize(GLOBAL_DATA, data_offset);
				data_offset += 8;
				let view_rows = _to_usize(GLOBAL_DATA, data_offset);
				data_offset += 8;
				let view_size = view_cols * view_rows;
				let mut view_data = Vec::new();
				for i in 0..view_size {
					let mut str_len = _to_u8(GLOBAL_DATA, data_offset) % 17;
					data_offset += 1;
					let str_val = _to_str(GLOBAL_DATA, data_offset, data_offset + str_len as usize);
					data_offset += str_len as usize;
					view_data.push(String::from(str_val));
				}
				let temp_view_mut = toodee::TooDeeViewMut::new(view_cols, view_rows, &mut view_data);
				toodee::TooDee::from(temp_view_mut)
			}
		};
		
		for op_idx in 0..num_ops {
			let GLOBAL_DATA = if op_idx % 2 == 0 { global_data.first_half } else { global_data.second_half };
			let operation = _to_u8(GLOBAL_DATA, data_offset);
			data_offset += 1;
			
			match operation % 20 {
				0 => {
					let row_idx = _to_usize(GLOBAL_DATA, data_offset);
					data_offset += 8;
					let vec_size = _to_u8(GLOBAL_DATA, data_offset) % 17;
					data_offset += 1;
					let mut custom_vec = Vec::new();
					for i in 0..vec_size {
						let mut str_len = _to_u8(GLOBAL_DATA, data_offset) % 17;
						data_offset += 1;
						let str_val = _to_str(GLOBAL_DATA, data_offset, data_offset + str_len as usize);
						data_offset += str_len as usize;
						custom_vec.push(String::from(str_val));
					}
					t_1.insert_row(row_idx, custom_vec);
				},
				1 => {
					let col_idx = _to_usize(GLOBAL_DATA, data_offset);
					data_offset += 8;
					let vec_size = _to_u8(GLOBAL_DATA, data_offset) % 17;
					data_offset += 1;
					let mut custom_vec = Vec::new();
					for i in 0..vec_size {
						let mut str_len = _to_u8(GLOBAL_DATA, data_offset) % 17;
						data_offset += 1;
						let str_val = _to_str(GLOBAL_DATA, data_offset, data_offset + str_len as usize);
						data_offset += str_len as usize;
						custom_vec.push(String::from(str_val));
					}
					t_1.insert_col(col_idx, custom_vec);
				},
				2 => {
					let vec_size = _to_u8(GLOBAL_DATA, data_offset) % 17;
					data_offset += 1;
					let mut custom_vec = Vec::new();
					for i in 0..vec_size {
						let mut str_len = _to_u8(GLOBAL_DATA, data_offset) % 17;
						data_offset += 1;
						let str_val = _to_str(GLOBAL_DATA, data_offset, data_offset + str_len as usize);
						data_offset += str_len as usize;
						custom_vec.push(String::from(str_val));
					}
					t_1.push_row(custom_vec);
				},
				3 => {
					let vec_size = _to_u8(GLOBAL_DATA, data_offset) % 17;
					data_offset += 1;
					let mut custom_vec = Vec::new();
					for i in 0..vec_size {
						let mut str_len = _to_u8(GLOBAL_DATA, data_offset) % 17;
						data_offset += 1;
						let str_val = _to_str(GLOBAL_DATA, data_offset, data_offset + str_len as usize);
						data_offset += str_len as usize;
						custom_vec.push(String::from(str_val));
					}
					t_1.push_col(custom_vec);
				},
				4 => {
					let popped_row = t_1.pop_row();
					if let Some(mut drain) = popped_row {
						let next_item = drain.next();
						if let Some(item) = next_item {
							println!("{:?}", item);
						}
					}
				},
				5 => {
					let popped_col = t_1.pop_col();
					if let Some(mut drain) = popped_col {
						let next_item = drain.next();
						if let Some(item) = next_item {
							println!("{:?}", item);
						}
					}
				},
				6 => {
					let remove_row_idx = _to_usize(GLOBAL_DATA, data_offset);
					data_offset += 8;
					let mut removed_row = t_1.remove_row(remove_row_idx);
					let first_item = removed_row.next();
					if let Some(item) = first_item {
						println!("{:?}", item);
					}
				},
				7 => {
					let remove_col_idx = _to_usize(GLOBAL_DATA, data_offset);
					data_offset += 8;
					let mut removed_col = t_1.remove_col(remove_col_idx);
					let first_item = removed_col.next();
					if let Some(item) = first_item {
						println!("{:?}", item);
					}
				},
				8 => {
					let start_coord = (_to_usize(GLOBAL_DATA, data_offset), _to_usize(GLOBAL_DATA, data_offset + 8));
					data_offset += 16;
					let end_coord = (_to_usize(GLOBAL_DATA, data_offset), _to_usize(GLOBAL_DATA, data_offset + 8));
					data_offset += 16;
					let view = t_1.view(start_coord, end_coord);
					let rows_iter = view.rows();
					for row in rows_iter {
						for item in row {
							println!("{:?}", item);
						}
					}
				},
				9 => {
					let start_coord = (_to_usize(GLOBAL_DATA, data_offset), _to_usize(GLOBAL_DATA, data_offset + 8));
					data_offset += 16;
					let end_coord = (_to_usize(GLOBAL_DATA, data_offset), _to_usize(GLOBAL_DATA, data_offset + 8));
					data_offset += 16;
					let mut view_mut = t_1.view_mut(start_coord, end_coord);
					let rows_mut_iter = view_mut.rows_mut();
					for row_mut in rows_mut_iter {
						for item in row_mut {
							println!("{:?}", item);
						}
					}
				},
				10 => {
					let col_idx = _to_usize(GLOBAL_DATA, data_offset);
					data_offset += 8;
					let col_iter = t_1.col(col_idx);
					for item in col_iter {
						println!("{:?}", item);
					}
				},
				11 => {
					let col_idx = _to_usize(GLOBAL_DATA, data_offset);
					data_offset += 8;
					let col_mut_iter = t_1.col_mut(col_idx);
					for item in col_mut_iter {
						println!("{:?}", item);
					}
				},
				12 => {
					let rows_iter = t_1.rows();
					for row in rows_iter {
						for item in row {
							println!("{:?}", item);
						}
					}
				},
				13 => {
					let rows_mut_iter = t_1.rows_mut();
					for row_mut in rows_mut_iter {
						for item in row_mut {
							println!("{:?}", item);
						}
					}
				},
				14 => {
					let row1 = _to_usize(GLOBAL_DATA, data_offset);
					data_offset += 8;
					let row2 = _to_usize(GLOBAL_DATA, data_offset);
					data_offset += 8;
					t_1.swap_rows(row1, row2);
				},
				15 => {
					let row_idx = _to_usize(GLOBAL_DATA, data_offset);
					data_offset += 8;
					let row_ref = &t_1[row_idx];
					for item in row_ref {
						println!("{:?}", item);
					}
				},
				16 => {
					let coord = (_to_usize(GLOBAL_DATA, data_offset), _to_usize(GLOBAL_DATA, data_offset + 8));
					data_offset += 16;
					let cell_ref = &t_1[coord];
					println!("{:?}", cell_ref);
				},
				17 => {
					let row_idx = _to_usize(GLOBAL_DATA, data_offset);
					data_offset += 8;
					let row_mut_ref = &mut t_1[row_idx];
					for item in row_mut_ref {
						println!("{:?}", item);
					}
				},
				18 => {
					let coord = (_to_usize(GLOBAL_DATA, data_offset), _to_usize(GLOBAL_DATA, data_offset + 8));
					data_offset += 16;
					let cell_mut_ref = &mut t_1[coord];
					println!("{:?}", cell_mut_ref);
				},
				_ => {
					let cells_iter = t_1.cells();
					for cell in cells_iter {
						println!("{:?}", cell);
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