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
		if data.len() < 2500 {return;}
		set_global_data(data);
		let global_data = get_global_data();
		let GLOBAL_DATA = global_data.first_half;
		let SECOND_DATA = global_data.second_half;
		
		let num_ops = _to_u8(GLOBAL_DATA, 0) % 65;
		
		let mut base_data = Vec::with_capacity(64);
		for i in 0..64 {
			let str_len = _to_u8(GLOBAL_DATA, 1 + i) % 17;
			let start_idx = 65 + i * 17;
			let end_idx = start_idx + str_len as usize;
			let content = _to_str(GLOBAL_DATA, start_idx, end_idx);
			base_data.push(CustomType0(String::from(content)));
		}
		
		let constructor_choice = _to_u8(GLOBAL_DATA, 1153) % 7;
		let mut toodee = match constructor_choice {
			0 => {
				let cols = _to_usize(GLOBAL_DATA, 1154);
				let rows = _to_usize(GLOBAL_DATA, 1162);
				TooDee::from_vec(cols, rows, base_data)
			},
			1 => {
				let cols = _to_usize(GLOBAL_DATA, 1170);
				let rows = _to_usize(GLOBAL_DATA, 1178);
				let val = CustomType0(String::from("init"));
				TooDee::init(cols, rows, val)
			},
			2 => {
				let capacity = _to_usize(GLOBAL_DATA, 1186);
				TooDee::with_capacity(capacity)
			},
			3 => {
				let cols = _to_usize(GLOBAL_DATA, 1194);
				let rows = _to_usize(GLOBAL_DATA, 1202);
				let boxed = base_data.into_boxed_slice();
				TooDee::from_box(cols, rows, boxed)
			},
			4 => {
				let cols = _to_usize(GLOBAL_DATA, 1210);
				let rows = _to_usize(GLOBAL_DATA, 1218);
				TooDee::new(cols, rows)
			},
			5 => {
				let slice_data = Vec::new();
				let view = TooDeeView::new(4, 4, &slice_data);
				TooDee::from(view)
			},
			_ => {
				let mut slice_data = Vec::new();
				slice_data.resize(16, CustomType0(String::new()));
				let view_mut = TooDeeViewMut::new(4, 4, &mut slice_data);
				TooDee::from(view_mut)
			}
		};
		
		for op_index in 0..num_ops {
			let op_type = _to_u8(SECOND_DATA, op_index as usize) % 20;
			let base_offset = op_index as usize * 50;
			
			match op_type {
				0 => {
					let coord_row = _to_usize(SECOND_DATA, base_offset);
					let coord_col = _to_usize(SECOND_DATA, base_offset + 8);
					let target_coord = (coord_col, coord_row);
					if toodee.num_cols() > 0 && toodee.num_rows() > 0 {
						let view_start = (coord_col, coord_row);
						let view_end = (coord_col + 1, coord_row + 1);
						let view = toodee.view(view_start, view_end);
						let view_result = view.index(target_coord);
						println!("{:?}", view_result);
					}
				},
				1 => {
					if toodee.num_cols() > 0 && toodee.num_rows() > 0 {
						let coord_row = _to_usize(SECOND_DATA, base_offset + 16);
						let coord_col = _to_usize(SECOND_DATA, base_offset + 24);
						let target_coord = (coord_col, coord_row);
						let view_start = (coord_col, coord_row);
						let view_end = (coord_col + 2, coord_row + 2);
						let mut view_mut = toodee.view_mut(view_start, view_end);
						let indexed_ref = view_mut.index(target_coord);
						println!("{:?}", indexed_ref);
					}
				},
				2 => {
					let row_idx = _to_usize(SECOND_DATA, base_offset + 32);
					if toodee.num_rows() > 0 {
						let row_slice = toodee.index(row_idx);
						println!("{}", row_slice.len());
					}
				},
				3 => {
					let col_idx = _to_usize(SECOND_DATA, base_offset + 40);
					if toodee.num_cols() > 0 {
						let col_iter = toodee.col(col_idx);
						for cell_ref in col_iter {
							println!("{:?}", cell_ref);
							break;
						}
					}
				},
				4 => {
					let rows_iter = toodee.rows();
					for row_ref in rows_iter {
						println!("{}", row_ref.len());
						break;
					}
				},
				5 => {
					if toodee.num_cols() > 0 {
						let col_idx = _to_usize(SECOND_DATA, base_offset + 8);
						let mut col_mut = toodee.col_mut(col_idx);
						for cell_mut_ref in col_mut {
							println!("{:?}", cell_mut_ref);
							break;
						}
					}
				},
				6 => {
					let mut rows_mut = toodee.rows_mut();
					for row_mut_ref in rows_mut {
						println!("{}", row_mut_ref.len());
						break;
					}
				},
				7 => {
					let drain_result = toodee.pop_col();
					if let Some(mut drain) = drain_result {
						for drained_item in drain {
							println!("{:?}", drained_item);
							break;
						}
					}
				},
				8 => {
					let remove_col_idx = _to_usize(SECOND_DATA, base_offset + 16);
					if toodee.num_cols() > 0 {
						let mut drain_col = toodee.remove_col(remove_col_idx);
						for drained_item in drain_col {
							println!("{:?}", drained_item);
							break;
						}
					}
				},
				9 => {
					if toodee.num_rows() > 1 {
						let r1 = _to_usize(SECOND_DATA, base_offset + 24);
						let r2 = _to_usize(SECOND_DATA, base_offset + 32);
						toodee.swap_rows(r1, r2);
					}
				},
				10 => {
					if toodee.num_cols() > 1 {
						let c1 = _to_usize(SECOND_DATA, base_offset + 40);
						let c2 = _to_usize(SECOND_DATA, base_offset + 48);
						toodee.swap_cols(c1, c2);
					}
				},
				11 => {
					if toodee.num_cols() > 0 && toodee.num_rows() > 0 {
						let coord_row = _to_usize(SECOND_DATA, base_offset);
						let coord_col = _to_usize(SECOND_DATA, base_offset + 8);
						let target_coord = (coord_col, coord_row);
						let start_coord = (coord_col, coord_row);
						let end_coord = (coord_col + 1, coord_row + 1);
						let view = toodee.view(start_coord, end_coord);
						let indexed_value = view.index(target_coord);
						println!("{:?}", indexed_value);
					}
				},
				12 => {
					if toodee.num_cols() > 0 && toodee.num_rows() > 0 {
						let coord_row = _to_usize(SECOND_DATA, base_offset + 16);
						let coord_col = _to_usize(SECOND_DATA, base_offset + 24);
						let target_coord = (coord_col, coord_row);
						let start_coord = (coord_col, coord_row);
						let end_coord = (coord_col + 2, coord_row + 2);
						let view = toodee.view(start_coord, end_coord);
						let sub_view = view.view(target_coord, end_coord);
						println!("{}", sub_view.num_cols());
					}
				},
				13 => {
					if toodee.num_cols() > 0 && toodee.num_rows() > 0 {
						let start_row = _to_usize(SECOND_DATA, base_offset + 32);
						let start_col = _to_usize(SECOND_DATA, base_offset + 40);
						let end_row = _to_usize(SECOND_DATA, base_offset + 48);
						let end_col = _to_usize(SECOND_DATA, base_offset + 8);
						let start_coord = (start_col, start_row);
						let end_coord = (end_col, end_row);
						let mut view_mut = toodee.view_mut(start_coord, end_coord);
						let subview_mut = view_mut.view_mut(start_coord, end_coord);
						println!("{}", subview_mut.num_rows());
					}
				},
				14 => {
					if toodee.num_cols() > 0 && toodee.num_rows() > 0 {
						let coord_row = _to_usize(SECOND_DATA, base_offset + 16);
						let coord_col = _to_usize(SECOND_DATA, base_offset + 24);
						let start_coord = (coord_col, coord_row);
						let end_coord = (coord_col + 3, coord_row + 3);
						let mut view_mut = toodee.view_mut(start_coord, end_coord);
						if view_mut.num_rows() > 1 {
							let row1 = _to_usize(SECOND_DATA, base_offset + 32);
							let row2 = _to_usize(SECOND_DATA, base_offset + 40);
							view_mut.swap_rows(row1, row2);
						}
					}
				},
				15 => {
					if toodee.num_cols() > 0 && toodee.num_rows() > 0 {
						let coord_row = _to_usize(SECOND_DATA, base_offset);
						let coord_col = _to_usize(SECOND_DATA, base_offset + 8);
						let start_coord = (coord_col, coord_row);
						let end_coord = (coord_col + 2, coord_row + 2);
						let view = toodee.view(start_coord, end_coord);
						let col_idx = _to_usize(SECOND_DATA, base_offset + 16);
						let col_iter = view.col(col_idx);
						for cell_ref in col_iter {
							println!("{:?}", cell_ref);
							break;
						}
					}
				},
				16 => {
					if toodee.num_cols() > 0 && toodee.num_rows() > 0 {
						let coord_row = _to_usize(SECOND_DATA, base_offset + 24);
						let coord_col = _to_usize(SECOND_DATA, base_offset + 32);
						let start_coord = (coord_col, coord_row);
						let end_coord = (coord_col + 3, coord_row + 3);
						let mut view_mut = toodee.view_mut(start_coord, end_coord);
						let col_idx = _to_usize(SECOND_DATA, base_offset + 40);
						let mut col_mut = view_mut.col_mut(col_idx);
						for cell_mut_ref in col_mut {
							println!("{:?}", cell_mut_ref);
							break;
						}
					}
				},
				17 => {
					if toodee.num_cols() > 0 && toodee.num_rows() > 0 {
						let coord_row = _to_usize(SECOND_DATA, base_offset + 8);
						let coord_col = _to_usize(SECOND_DATA, base_offset + 16);
						let start_coord = (coord_col, coord_row);
						let end_coord = (coord_col + 2, coord_row + 2);
						let view = toodee.view(start_coord, end_coord);
						let rows_iter = view.rows();
						for row_ref in rows_iter {
							println!("{}", row_ref.len());
							break;
						}
					}
				},
				18 => {
					if toodee.num_cols() > 0 && toodee.num_rows() > 0 {
						let coord_row = _to_usize(SECOND_DATA, base_offset + 24);
						let coord_col = _to_usize(SECOND_DATA, base_offset + 32);
						let start_coord = (coord_col, coord_row);
						let end_coord = (coord_col + 3, coord_row + 3);
						let mut view_mut = toodee.view_mut(start_coord, end_coord);
						let mut rows_mut = view_mut.rows_mut();
						for row_mut_ref in rows_mut {
							println!("{}", row_mut_ref.len());
							break;
						}
					}
				},
				_ => {
					if toodee.num_cols() > 0 && toodee.num_rows() > 0 {
						let coord_row = _to_usize(SECOND_DATA, base_offset + 40);
						let coord_col = _to_usize(SECOND_DATA, base_offset + 48);
						let target_coord = (coord_col, coord_row);
						let start_coord = (coord_col, coord_row);
						let end_coord = (coord_col + 1, coord_row + 1);
						let mut view_mut = toodee.view_mut(start_coord, end_coord);
						let indexed_mut_ref = view_mut.index_mut(target_coord);
						println!("{:?}", indexed_mut_ref);
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