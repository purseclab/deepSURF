#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use toodee::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

#[derive(Clone, Default, Debug)]
struct CustomType0(String);

fn main() {
	fuzz_nohook!(|data: &[u8]| {
		if data.len() < 1800 { return; }
		set_global_data(data);
		let global_data = get_global_data();
		let GLOBAL_DATA = global_data.first_half;
		let mut vec_len = (_to_u8(GLOBAL_DATA, 4) % 65) as usize;
		if vec_len == 0 { vec_len = 1; }
		let mut custom_vec: Vec<CustomType0> = Vec::with_capacity(vec_len);
		let mut str_offset = 200;
		for _ in 0..vec_len {
			let s_len = _to_u8(GLOBAL_DATA, str_offset) % 17;
			let s = _to_str(GLOBAL_DATA, str_offset + 1, str_offset + 1 + s_len as usize);
			custom_vec.push(CustomType0(String::from(s)));
			str_offset += 18;
		}
		let choice = _to_u8(GLOBAL_DATA, 0) % 5;
		let t_cols0 = _to_usize(GLOBAL_DATA, 8);
		let t_rows0 = _to_usize(GLOBAL_DATA, 16);
		let t_capacity = _to_usize(GLOBAL_DATA, 24);
		let mut td: TooDee<CustomType0>;
		match choice {
			0 => {
				td = TooDee::new(t_cols0, t_rows0);
			}
			1 => {
				let init_val = CustomType0(String::from("init"));
				td = TooDee::init(t_cols0, t_rows0, init_val);
			}
			2 => {
				td = TooDee::with_capacity(t_capacity);
				td.push_row(custom_vec.clone());
			}
			3 => {
				td = TooDee::from_vec(t_cols0, t_rows0, custom_vec.clone());
			}
			_ => {
				let bx: Box<[CustomType0]> = custom_vec.clone().into_boxed_slice();
				td = TooDee::from_box(t_cols0, t_rows0, bx);
			}
		}
		let mut rows_mut_main = td.rows_mut();
		rows_mut_main.nth(_to_usize(GLOBAL_DATA, 100));
		rows_mut_main.nth_back(_to_usize(GLOBAL_DATA, 108));
		rows_mut_main.next();
		let last_main = rows_mut_main.last();
		if let Some(r) = &last_main {
			println!("{:?}", r.deref());
		}
		td.swap_rows(_to_usize(GLOBAL_DATA, 120), _to_usize(GLOBAL_DATA, 128));
		let view = td.view(
			(_to_usize(GLOBAL_DATA, 136), _to_usize(GLOBAL_DATA, 144)),
			(_to_usize(GLOBAL_DATA, 152), _to_usize(GLOBAL_DATA, 160)),
		);
		let mut rows_view = view.rows();
		rows_view.last();
		let col_iter = td.col(_to_usize(GLOBAL_DATA, 168));
		col_iter.last();
		let mut view_mut = td.view_mut(
			(_to_usize(GLOBAL_DATA, 300), _to_usize(GLOBAL_DATA, 308)),
			(_to_usize(GLOBAL_DATA, 316), _to_usize(GLOBAL_DATA, 324)),
		);
		view_mut.swap_rows(_to_usize(GLOBAL_DATA, 332), _to_usize(GLOBAL_DATA, 340));
		let mut rows_mut_view = view_mut.rows_mut();
		let last_view = rows_mut_view.last();
		if let Some(r) = &last_view {
			println!("{:?}", r.deref());
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