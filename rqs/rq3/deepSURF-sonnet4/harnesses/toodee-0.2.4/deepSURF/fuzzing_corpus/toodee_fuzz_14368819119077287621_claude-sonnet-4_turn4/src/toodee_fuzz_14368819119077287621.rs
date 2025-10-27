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
		if data.len() < 1024 {return;}
		set_global_data(data);
		let global_data = get_global_data();
		let GLOBAL_DATA = global_data.first_half;
		
		let num_operations = _to_usize(GLOBAL_DATA, 0) % 65;
		
		for op_idx in 0..num_operations {
			let base_offset = op_idx * 80;
			if base_offset + 80 >= GLOBAL_DATA.len() { break; }
			
			let operation = _to_u8(GLOBAL_DATA, base_offset) % 12;
			
			match operation {
				0 => {
					let constructor_choice = _to_u8(GLOBAL_DATA, base_offset + 1) % 4;
					let mut toodee = match constructor_choice {
						0 => {
							let capacity = _to_usize(GLOBAL_DATA, base_offset + 8);
							toodee::TooDee::<CustomType0>::with_capacity(capacity)
						},
						1 => {
							let cols = _to_usize(GLOBAL_DATA, base_offset + 8);
							let rows = _to_usize(GLOBAL_DATA, base_offset + 16);
							toodee::TooDee::<CustomType0>::new(cols, rows)
						},
						2 => {
							let cols = _to_usize(GLOBAL_DATA, base_offset + 8);
							let rows = _to_usize(GLOBAL_DATA, base_offset + 16);
							let init_val = CustomType0(String::from("init"));
							toodee::TooDee::<CustomType0>::init(cols, rows, init_val)
						},
						_ => {
							let cols = _to_usize(GLOBAL_DATA, base_offset + 8);
							let rows = _to_usize(GLOBAL_DATA, base_offset + 16);
							let vec_len = cols * rows;
							let mut vec_data = Vec::with_capacity(vec_len);
							for i in 0..vec_len {
								vec_data.push(CustomType0(format!("item_{}", i)));
							}
							toodee::TooDee::<CustomType0>::from_vec(cols, rows, vec_data)
						}
					};
					
					let ref_toodee = &toodee;
					let mut rows_iter = ref_toodee.rows();
					let nth_back_n = _to_usize(GLOBAL_DATA, base_offset + 24);
					if let Some(row_slice) = rows_iter.nth_back(nth_back_n) {
						println!("{:?}", row_slice);
					}
					
					let col_idx = _to_usize(GLOBAL_DATA, base_offset + 32);
					let col_iter = toodee.col(col_idx);
					for cell in col_iter {
						println!("{:?}", cell);
					}
					
					let drain_result = toodee.pop_col();
					if let Some(mut drain_col) = drain_result {
						for elem in drain_col {
							println!("{:?}", elem);
						}
					}
				},
				1 => {
					let vec_size = _to_usize(GLOBAL_DATA, base_offset + 8) % 65;
					let mut vec_data = Vec::with_capacity(vec_size);
					for i in 0..vec_size {
						vec_data.push(CustomType0(format!("elem_{}", i)));
					}
					let view_cols = _to_usize(GLOBAL_DATA, base_offset + 16);
					let view_rows = _to_usize(GLOBAL_DATA, base_offset + 24);
					let view = toodee::TooDeeView::<CustomType0>::new(view_cols, view_rows, &vec_data);
					let ref_view = &view;
					let mut rows_iter = ref_view.rows();
					let nth_back_n = _to_usize(GLOBAL_DATA, base_offset + 32);
					if let Some(row_slice) = rows_iter.nth_back(nth_back_n) {
						println!("{:?}", row_slice);
					}
					
					let col_idx = _to_usize(GLOBAL_DATA, base_offset + 40);
					let col_iter = view.col(col_idx);
					for cell in col_iter {
						println!("{:?}", cell);
					}
					
					let sub_view = view.view((_to_usize(GLOBAL_DATA, base_offset + 48), _to_usize(GLOBAL_DATA, base_offset + 56)), 
						(_to_usize(GLOBAL_DATA, base_offset + 64), _to_usize(GLOBAL_DATA, base_offset + 72)));
					for row in sub_view.rows() {
						println!("{:?}", row);
					}
				},
				2 => {
					let vec_size = _to_usize(GLOBAL_DATA, base_offset + 8) % 65;
					let mut vec_data = Vec::with_capacity(vec_size);
					for i in 0..vec_size {
						vec_data.push(CustomType0(format!("mut_elem_{}", i)));
					}
					let view_cols = _to_usize(GLOBAL_DATA, base_offset + 16);
					let view_rows = _to_usize(GLOBAL_DATA, base_offset + 24);
					let mut view_mut = toodee::TooDeeViewMut::<CustomType0>::new(view_cols, view_rows, &mut vec_data);
					let ref_view_mut = &view_mut;
					let mut rows_iter = ref_view_mut.rows();
					let nth_back_n = _to_usize(GLOBAL_DATA, base_offset + 32);
					if let Some(row_slice) = rows_iter.nth_back(nth_back_n) {
						println!("{:?}", row_slice);
					}
					
					let col_idx = _to_usize(GLOBAL_DATA, base_offset + 40);
					let mut col_mut_iter = view_mut.col_mut(col_idx);
					for elem in &mut col_mut_iter {
						*elem = CustomType0(String::from("modified"));
						println!("{:?}", elem);
					}
				},
				3 => {
					let cols = _to_usize(GLOBAL_DATA, base_offset + 8);
					let rows = _to_usize(GLOBAL_DATA, base_offset + 16);
					let mut toodee = toodee::TooDee::<CustomType0>::new(cols, rows);
					let col_idx = _to_usize(GLOBAL_DATA, base_offset + 24);
					let col_iter = toodee.col(col_idx);
					for cell in col_iter {
						println!("{:?}", cell);
					}
					let mut rows_iter = toodee.rows();
					let nth_back_n = _to_usize(GLOBAL_DATA, base_offset + 32);
					if let Some(row_slice) = rows_iter.nth_back(nth_back_n) {
						println!("{:?}", row_slice);
					}
					
					let start_coord = (_to_usize(GLOBAL_DATA, base_offset + 40), _to_usize(GLOBAL_DATA, base_offset + 48));
					let end_coord = (_to_usize(GLOBAL_DATA, base_offset + 56), _to_usize(GLOBAL_DATA, base_offset + 64));
					let view = toodee.view(start_coord, end_coord);
					println!("{:?}", view);
				},
				4 => {
					let cols = _to_usize(GLOBAL_DATA, base_offset + 8);
					let rows = _to_usize(GLOBAL_DATA, base_offset + 16);
					let mut toodee = toodee::TooDee::<CustomType0>::new(cols, rows);
					let mut rows_mut_iter = toodee.rows_mut();
					let nth_back_n = _to_usize(GLOBAL_DATA, base_offset + 24);
					if let Some(row_slice) = rows_mut_iter.nth_back(nth_back_n) {
						let mut row_vec = Vec::new();
						for elem in &mut *row_slice {
							*elem = CustomType0(String::from("mut_row"));
							row_vec.push(elem.clone());
						}
						println!("{:?}", row_vec);
					}
					
					let col_idx = _to_usize(GLOBAL_DATA, base_offset + 32);
					let col_iter = toodee.col(col_idx);
					for cell in col_iter {
						println!("{:?}", cell);
					}
					
					toodee.fill(CustomType0(String::from("filled")));
					println!("{:?}", toodee.data());
				},
				5 => {
					let cols = _to_usize(GLOBAL_DATA, base_offset + 8);
					let rows = _to_usize(GLOBAL_DATA, base_offset + 16);
					let mut toodee = toodee::TooDee::<CustomType0>::new(cols, rows);
					let col_idx = _to_usize(GLOBAL_DATA, base_offset + 24);
					let mut col_mut_iter = toodee.col_mut(col_idx);
					let nth_back_n = _to_usize(GLOBAL_DATA, base_offset + 32);
					if let Some(elem) = col_mut_iter.nth_back(nth_back_n) {
						*elem = CustomType0(String::from("nth_back_mut"));
						println!("{:?}", elem);
					}
					
					let r1 = _to_usize(GLOBAL_DATA, base_offset + 40);
					let r2 = _to_usize(GLOBAL_DATA, base_offset + 48);
					toodee.swap_rows(r1, r2);
					
					let c1 = _to_usize(GLOBAL_DATA, base_offset + 56);
					let c2 = _to_usize(GLOBAL_DATA, base_offset + 64);
					toodee.swap_cols(c1, c2);
				},
				6 => {
					let cols = _to_usize(GLOBAL_DATA, base_offset + 8);
					let rows = _to_usize(GLOBAL_DATA, base_offset + 16);
					let mut toodee = toodee::TooDee::<CustomType0>::new(cols, rows);
					let start_coord = (_to_usize(GLOBAL_DATA, base_offset + 24), _to_usize(GLOBAL_DATA, base_offset + 32));
					let end_coord = (_to_usize(GLOBAL_DATA, base_offset + 40), _to_usize(GLOBAL_DATA, base_offset + 48));
					let view = toodee.view(start_coord, end_coord);
					let mut rows_iter = view.rows();
					let nth_back_n = _to_usize(GLOBAL_DATA, base_offset + 56);
					if let Some(row_slice) = rows_iter.nth_back(nth_back_n) {
						println!("{:?}", row_slice);
					}
					
					for cells in toodee.cells() {
						println!("{:?}", cells);
					}
					
					let coord_access = (_to_usize(GLOBAL_DATA, base_offset + 64), _to_usize(GLOBAL_DATA, base_offset + 72));
					if toodee.num_cols() > 0 && toodee.num_rows() > 0 && coord_access.0 < toodee.num_cols() && coord_access.1 < toodee.num_rows() {
						let cell_ref = &toodee[coord_access];
						println!("{:?}", cell_ref);
					}
				},
				7 => {
					let cols = _to_usize(GLOBAL_DATA, base_offset + 8);
					let rows = _to_usize(GLOBAL_DATA, base_offset + 16);
					let mut toodee = toodee::TooDee::<CustomType0>::new(cols, rows);
					let start_coord = (_to_usize(GLOBAL_DATA, base_offset + 24), _to_usize(GLOBAL_DATA, base_offset + 32));
					let end_coord = (_to_usize(GLOBAL_DATA, base_offset + 40), _to_usize(GLOBAL_DATA, base_offset + 48));
					let mut view_mut = toodee.view_mut(start_coord, end_coord);
					let mut rows_mut_iter = view_mut.rows_mut();
					let nth_back_n = _to_usize(GLOBAL_DATA, base_offset + 56);
					if let Some(row_slice) = rows_mut_iter.nth_back(nth_back_n) {
						let mut row_vec = Vec::new();
						for elem in &mut *row_slice {
							*elem = CustomType0(String::from("view_mut"));
							row_vec.push(elem.clone());
						}
						println!("{:?}", row_vec);
					}
					
					let sub_start = (_to_usize(GLOBAL_DATA, base_offset + 64), _to_usize(GLOBAL_DATA, base_offset + 65));
					let sub_end = (_to_usize(GLOBAL_DATA, base_offset + 66), _to_usize(GLOBAL_DATA, base_offset + 67));
					let sub_view_mut = view_mut.view_mut(sub_start, sub_end);
					println!("{:?}", sub_view_mut);
				},
				8 => {
					let cols = _to_usize(GLOBAL_DATA, base_offset + 8);
					let rows = _to_usize(GLOBAL_DATA, base_offset + 16);
					let mut toodee = toodee::TooDee::<CustomType0>::new(cols, rows);
					
					let new_row_data: Vec<CustomType0> = (0.._to_usize(GLOBAL_DATA, base_offset + 24) % 65)
						.map(|i| CustomType0(format!("new_row_{}", i)))
						.collect();
					toodee.push_row(new_row_data);
					
					let remove_idx = _to_usize(GLOBAL_DATA, base_offset + 32);
					let drain_row = toodee.remove_row(remove_idx);
					for elem in drain_row {
						println!("{:?}", elem);
					}
					
					let mut cells_mut = toodee.cells_mut();
					for cell in cells_mut {
						*cell = CustomType0(String::from("cells_mut"));
					}
				},
				9 => {
					let cols = _to_usize(GLOBAL_DATA, base_offset + 8);
					let rows = _to_usize(GLOBAL_DATA, base_offset + 16);
					let mut toodee = toodee::TooDee::<CustomType0>::new(cols, rows);
					
					let new_col_data: Vec<CustomType0> = (0.._to_usize(GLOBAL_DATA, base_offset + 24) % 65)
						.map(|i| CustomType0(format!("new_col_{}", i)))
						.collect();
					toodee.push_col(new_col_data);
					
					let insert_idx = _to_usize(GLOBAL_DATA, base_offset + 32);
					let insert_data: Vec<CustomType0> = (0.._to_usize(GLOBAL_DATA, base_offset + 40) % 65)
						.map(|i| CustomType0(format!("insert_{}", i)))
						.collect();
					toodee.insert_col(insert_idx, insert_data);
					
					let remove_col_idx = _to_usize(GLOBAL_DATA, base_offset + 48);
					let drain_col = toodee.remove_col(remove_col_idx);
					for elem in drain_col {
						println!("{:?}", elem);
					}
				},
				10 => {
					let cols = _to_usize(GLOBAL_DATA, base_offset + 8);
					let rows = _to_usize(GLOBAL_DATA, base_offset + 16);
					let source_toodee = toodee::TooDee::<CustomType0>::init(cols, rows, CustomType0(String::from("source")));
					
					let mut dest_toodee = toodee::TooDee::<CustomType0>::new(cols, rows);
					dest_toodee.clone_from_toodee(&source_toodee);
					
					for row in dest_toodee.rows() {
						println!("{:?}", row);
					}
					
					let converted: TooDee<CustomType0> = TooDee::from(source_toodee.view(
						(_to_usize(GLOBAL_DATA, base_offset + 24), _to_usize(GLOBAL_DATA, base_offset + 32)),
						(_to_usize(GLOBAL_DATA, base_offset + 40), _to_usize(GLOBAL_DATA, base_offset + 48))
					));
					println!("{:?}", converted);
				},
				_ => {
					let cols = _to_usize(GLOBAL_DATA, base_offset + 8);
					let rows = _to_usize(GLOBAL_DATA, base_offset + 16);
					let mut toodee = toodee::TooDee::<CustomType0>::new(cols, rows);
					
					let row_idx = _to_usize(GLOBAL_DATA, base_offset + 24);
					if row_idx < toodee.num_rows() {
						let row_ref = &toodee[row_idx];
						println!("{:?}", row_ref);
					}
					
					let coord = (_to_usize(GLOBAL_DATA, base_offset + 32), _to_usize(GLOBAL_DATA, base_offset + 40));
					if toodee.num_cols() > 0 && toodee.num_rows() > 0 && coord.0 < toodee.num_cols() && coord.1 < toodee.num_rows() {
						let cell_ref = &toodee[coord];
						println!("{:?}", cell_ref);
					}
					
					let size = toodee.size();
					println!("Size: {:?}", size);
					
					let bounds = toodee.bounds();
					println!("Bounds: {:?}", bounds);
					
					let is_empty = toodee.is_empty();
					println!("Is empty: {}", is_empty);
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