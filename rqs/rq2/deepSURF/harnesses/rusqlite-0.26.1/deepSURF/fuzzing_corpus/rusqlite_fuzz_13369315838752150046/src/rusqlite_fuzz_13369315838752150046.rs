//DefId(0:560 ~ rusqlite[8877]::statement::{impl#0}::raw_bind_parameter)
#[macro_use]
extern crate afl;

use rusqlite::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

struct CustomType0(String);

impl rusqlite::types::ToSql for CustomType0 {
	
	fn to_sql(&self) -> Result<rusqlite::types::ToSqlOutput, rusqlite::Error> {
		let global_data = get_global_data();
		let GLOBAL_DATA = global_data.first_half;
		let custom_impl_num = _to_usize(GLOBAL_DATA, 42);
		let custom_impl_inst_num = self.0.len();
		let selector = (custom_impl_num + custom_impl_inst_num) % 3;
		if selector == 0{
			panic!("INTENTIONAL PANIC!");
		}
		let GLOBAL_DATA = match selector{
				1 => global_data.first_half,
				_ => global_data.second_half,
		};
		let t_12 = _to_usize(GLOBAL_DATA, 50);
		let t_13 = match (t_12 % 3usize) {
			0 => {
				let t_14 = _to_f64(GLOBAL_DATA, 58);
				rusqlite::types::ValueRef::Real(t_14)
			},
			1 => {
				rusqlite::types::ValueRef::Null
			},
			2 => {
				let t_15 = _to_i64(GLOBAL_DATA, 66);
				rusqlite::types::ValueRef::Integer(t_15)
			},
			_ => unreachable!(),
		};
		let t_16 = rusqlite::types::ToSqlOutput::Borrowed(t_13);
		let t_17 = Ok(t_16);
		return t_17;
	}
}

fn main (){
	fuzz_nohook!(|data: &[u8]| {
		if data.len() < 182 {return;}
		set_global_data(data);
		let global_data = get_global_data();
		let GLOBAL_DATA = global_data.first_half;
		let t_0 = rusqlite::OpenFlags::all();
		let mut t_1 = _to_u8(GLOBAL_DATA, 0) % 17;
		let t_2 = _to_str(GLOBAL_DATA, 1, 1 + t_1 as usize);
		let t_3 = rusqlite::Connection::open_in_memory_with_flags_and_vfs(t_0, t_2);
		let t_4 = _unwrap_result(t_3);
		let t_5 = &t_4;
		let mut t_6 = _to_u8(GLOBAL_DATA, 17) % 17;
		let t_7 = _to_str(GLOBAL_DATA, 18, 18 + t_6 as usize);
		let t_8 = rusqlite::Connection::prepare(t_5, t_7);
		let mut t_9 = _unwrap_result(t_8);
		let mut t_10 = &mut t_9;
		let t_11 = _to_usize(GLOBAL_DATA, 34);
		let mut t_18 = _to_u8(GLOBAL_DATA, 74) % 17;
		let t_19 = _to_str(GLOBAL_DATA, 75, 75 + t_18 as usize);
		let t_20 = String::from(t_19);
		let t_21 = CustomType0(t_20);
		t_10.raw_bind_parameter(t_11, t_21);
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