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
		if data.len() < 64 {return;}
		set_global_data(data);
		let global_data = get_global_data();
		let GLOBAL_DATA = global_data.first_half;
		
		let num_operations = _to_usize(GLOBAL_DATA, 0) % 65;
		
		for op_idx in 0..num_operations {
			let base_offset = (op_idx * 8) % GLOBAL_DATA.len().saturating_sub(8);
			if base_offset + 8 > GLOBAL_DATA.len() {break;}
			
			let operation = _to_u8(GLOBAL_DATA, base_offset) % 8;
			
			match operation {
				0 => {
					let capacity = _to_usize(GLOBAL_DATA, base_offset);
					let constructor_choice = _to_u8(GLOBAL_DATA, base_offset + 1) % 7;
					let toodee = match constructor_choice {
						0 => toodee::TooDee::<CustomType0>::with_capacity(capacity),
						1 => {
							let cols = _to_usize(GLOBAL_DATA, base_offset + 2);
							let rows = _to_usize(GLOBAL_DATA, base_offset + 3);
							toodee::TooDee::<CustomType0>::init(cols, rows, CustomType0(String::from("init")))
						},
						2 => {
							let cols = _to_usize(GLOBAL_DATA, base_offset + 2);
							let rows = _to_usize(GLOBAL_DATA, base_offset + 3);
							toodee::TooDee::<CustomType0>::new(cols, rows)
						},
						3 => {
							let vec_size = _to_usize(GLOBAL_DATA, base_offset + 2) % 65;
							let mut v = Vec::new();
							for i in 0..vec_size {
								v.push(CustomType0(format!("item_{}", i)));
							}
							let cols = _to_usize(GLOBAL_DATA, base_offset + 3);
							let rows = if cols > 0 { v.len() / cols } else { 0 };
							if cols * rows == v.len() {
								toodee::TooDee::<CustomType0>::from_vec(cols, rows, v)
							} else {
								toodee::TooDee::<CustomType0>::with_capacity(capacity)
							}
						},
						4 => {
							let vec_size = _to_usize(GLOBAL_DATA, base_offset + 2) % 65;
							let mut v = Vec::new();
							for i in 0..vec_size {
								v.push(CustomType0(format!("boxed_{}", i)));
							}
							let cols = _to_usize(GLOBAL_DATA, base_offset + 3);
							let rows = if cols > 0 { v.len() / cols } else { 0 };
							if cols * rows == v.len() {
								let boxed = v.into_boxed_slice();
								toodee::TooDee::<CustomType0>::from_box(cols, rows, boxed)
							} else {
								toodee::TooDee::<CustomType0>::with_capacity(capacity)
							}
						},
						5 => {
							let slice_size = _to_usize(GLOBAL_DATA, base_offset + 2) % 65;
							let mut v = Vec::new();
							for i in 0..slice_size {
								v.push(CustomType0(format!("view_{}", i)));
							}
							let cols = _to_usize(GLOBAL_DATA, base_offset + 3);
							let rows = if cols > 0 { v.len() / cols } else { 0 };
							if cols * rows == v.len() {
								let view = toodee::TooDeeView::new(cols, rows, &v);
								toodee::TooDee::<CustomType0>::from(view)
							} else {
								toodee::TooDee::<CustomType0>::with_capacity(capacity)
							}
						},
						_ => {
							let slice_size = _to_usize(GLOBAL_DATA, base_offset + 2) % 65;
							let mut v = Vec::new();
							for i in 0..slice_size {
								v.push(CustomType0(format!("viewmut_{}", i)));
							}
							let cols = _to_usize(GLOBAL_DATA, base_offset + 3);
							let rows = if cols > 0 { v.len() / cols } else { 0 };
							if cols * rows == v.len() {
								let mut view = toodee::TooDeeViewMut::new(cols, rows, &mut v);
								toodee::TooDee::<CustomType0>::from(view)
							} else {
								toodee::TooDee::<CustomType0>::with_capacity(capacity)
							}
						}
					};
					
					let rows_iter = toodee.rows();
					let last_result = rows_iter.last();
					if let Some(last_row) = last_result {
						println!("{:?}", last_row);
					}
				},
				1 => {
					let mut toodee = toodee::TooDee::<CustomType0>::new(3, 4);
					let view_mut = toodee.view_mut((0, 0), (2, 2));
					let rows_iter = view_mut.rows();
					let last_result = rows_iter.last();
					if let Some(last_row) = last_result {
						println!("{:?}", last_row);
					}
				},
				2 => {
					let mut toodee = toodee::TooDee::<CustomType0>::init(5, 3, CustomType0(String::from("test")));
					let col_iter = toodee.col(_to_usize(GLOBAL_DATA, base_offset + 1));
					let col_last = col_iter.last();
					if let Some(last_cell) = col_last {
						println!("{:?}", last_cell);
					}
				},
				3 => {
					let mut toodee = toodee::TooDee::<CustomType0>::with_capacity(_to_usize(GLOBAL_DATA, base_offset + 1));
					toodee.push_row(vec![CustomType0(String::from("row1")), CustomType0(String::from("row2"))]);
					let rows_mut_iter = toodee.rows_mut();
					let last_mut_row = rows_mut_iter.last();
					if let Some(last_row) = last_mut_row {
						println!("{:?}", last_row);
					}
				},
				4 => {
					let mut toodee = toodee::TooDee::<CustomType0>::init(4, 2, CustomType0(String::from("column_test")));
					let col_mut_iter = toodee.col_mut(_to_usize(GLOBAL_DATA, base_offset + 1));
					let col_mut_last = col_mut_iter.last();
					if let Some(last_cell) = col_mut_last {
						println!("{:?}", last_cell);
					}
				},
				5 => {
					let slice_data = [CustomType0(String::from("a")), CustomType0(String::from("b")), 
									 CustomType0(String::from("c")), CustomType0(String::from("d"))];
					let view = toodee::TooDeeView::new(2, 2, &slice_data);
					let rows_iter = view.rows();
					let last_result = rows_iter.last();
					if let Some(last_row) = last_result {
						println!("{:?}", last_row);
					}
				},
				6 => {
					let mut slice_data = [CustomType0(String::from("x")), CustomType0(String::from("y")), 
										 CustomType0(String::from("z")), CustomType0(String::from("w"))];
					let mut view = toodee::TooDeeViewMut::new(2, 2, &mut slice_data);
					let rows_iter = view.rows();
					let last_result = rows_iter.last();
					if let Some(last_row) = last_result {
						println!("{:?}", last_row);
					}
				},
				_ => {
					let mut toodee = toodee::TooDee::<CustomType0>::init(3, 3, CustomType0(String::from("multi")));
					let view = toodee.view((_to_usize(GLOBAL_DATA, base_offset + 1), _to_usize(GLOBAL_DATA, base_offset + 2)), 
										  (_to_usize(GLOBAL_DATA, base_offset + 3), _to_usize(GLOBAL_DATA, base_offset + 4)));
					let rows_iter = view.rows();
					let last_result = rows_iter.last();
					if let Some(last_row) = last_result {
						println!("{:?}", last_row);
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