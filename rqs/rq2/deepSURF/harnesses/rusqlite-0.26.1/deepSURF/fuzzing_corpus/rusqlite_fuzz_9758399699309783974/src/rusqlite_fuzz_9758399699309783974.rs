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
		let custom_impl_num = _to_usize(GLOBAL_DATA, 25);
		let custom_impl_inst_num = self.0.len();
		let selector = (custom_impl_num + custom_impl_inst_num) % 3;
		if selector == 0{
			panic!("INTENTIONAL PANIC!");
		}
		let GLOBAL_DATA = match selector{
				1 => global_data.first_half,
				_ => global_data.second_half,
		};
		let t_13 = _to_usize(GLOBAL_DATA, 33);
		let t_14 = match (t_13 % 5usize) {
			0 => {
				let mut t_15 = _to_u8(GLOBAL_DATA, 41) % 17;
				let t_16 = _to_str(GLOBAL_DATA, 42, 42 + t_15 as usize);
				let t_17 = String::from(t_16);
				rusqlite::types::Value::Text(t_17)
			},
			1 => {
				let t_18 = _to_f64(GLOBAL_DATA, 58);
				rusqlite::types::Value::Real(t_18)
			},
			2 => {
				let t_19 = _to_i64(GLOBAL_DATA, 66);
				rusqlite::types::Value::Integer(t_19)
			},
			3 => {
				let mut t_20 = _to_u8(GLOBAL_DATA, 74) % 33;
				// Start vector declaration.
				let mut t_21 = std::vec::Vec::with_capacity(32);
				let t_22 = _to_u8(GLOBAL_DATA, 75);
				t_21.push(t_22);
				let t_23 = _to_u8(GLOBAL_DATA, 76);
				t_21.push(t_23);
				let t_24 = _to_u8(GLOBAL_DATA, 77);
				t_21.push(t_24);
				let t_25 = _to_u8(GLOBAL_DATA, 78);
				t_21.push(t_25);
				let t_26 = _to_u8(GLOBAL_DATA, 79);
				t_21.push(t_26);
				let t_27 = _to_u8(GLOBAL_DATA, 80);
				t_21.push(t_27);
				let t_28 = _to_u8(GLOBAL_DATA, 81);
				t_21.push(t_28);
				let t_29 = _to_u8(GLOBAL_DATA, 82);
				t_21.push(t_29);
				let t_30 = _to_u8(GLOBAL_DATA, 83);
				t_21.push(t_30);
				let t_31 = _to_u8(GLOBAL_DATA, 84);
				t_21.push(t_31);
				let t_32 = _to_u8(GLOBAL_DATA, 85);
				t_21.push(t_32);
				let t_33 = _to_u8(GLOBAL_DATA, 86);
				t_21.push(t_33);
				let t_34 = _to_u8(GLOBAL_DATA, 87);
				t_21.push(t_34);
				let t_35 = _to_u8(GLOBAL_DATA, 88);
				t_21.push(t_35);
				let t_36 = _to_u8(GLOBAL_DATA, 89);
				t_21.push(t_36);
				let t_37 = _to_u8(GLOBAL_DATA, 90);
				t_21.push(t_37);
				let t_38 = _to_u8(GLOBAL_DATA, 91);
				t_21.push(t_38);
				let t_39 = _to_u8(GLOBAL_DATA, 92);
				t_21.push(t_39);
				let t_40 = _to_u8(GLOBAL_DATA, 93);
				t_21.push(t_40);
				let t_41 = _to_u8(GLOBAL_DATA, 94);
				t_21.push(t_41);
				let t_42 = _to_u8(GLOBAL_DATA, 95);
				t_21.push(t_42);
				let t_43 = _to_u8(GLOBAL_DATA, 96);
				t_21.push(t_43);
				let t_44 = _to_u8(GLOBAL_DATA, 97);
				t_21.push(t_44);
				let t_45 = _to_u8(GLOBAL_DATA, 98);
				t_21.push(t_45);
				let t_46 = _to_u8(GLOBAL_DATA, 99);
				t_21.push(t_46);
				let t_47 = _to_u8(GLOBAL_DATA, 100);
				t_21.push(t_47);
				let t_48 = _to_u8(GLOBAL_DATA, 101);
				t_21.push(t_48);
				let t_49 = _to_u8(GLOBAL_DATA, 102);
				t_21.push(t_49);
				let t_50 = _to_u8(GLOBAL_DATA, 103);
				t_21.push(t_50);
				let t_51 = _to_u8(GLOBAL_DATA, 104);
				t_21.push(t_51);
				let t_52 = _to_u8(GLOBAL_DATA, 105);
				t_21.push(t_52);
				let t_53 = _to_u8(GLOBAL_DATA, 106);
				t_21.push(t_53);
				t_21.truncate(t_20 as usize);
				// End vector declaration.
				rusqlite::types::Value::Blob(t_21)
			},
			4 => {
				rusqlite::types::Value::Null
			},
			_ => unreachable!(),
		};
		let t_54 = rusqlite::types::ToSqlOutput::Owned(t_14);
		let t_55 = Ok(t_54);
		return t_55;
	}
}

fn main (){
	fuzz_nohook!(|data: &[u8]| {
		if data.len() < 248 {return;}
		set_global_data(data);
		let global_data = get_global_data();
		let GLOBAL_DATA = global_data.first_half;
		let t_0 = rusqlite::OpenFlags::empty();
		let t_1 = rusqlite::Connection::open_in_memory_with_flags(t_0);
		let t_2 = _unwrap_result(t_1);
		let t_3 = &t_2;
		let mut t_4 = _to_u8(GLOBAL_DATA, 0) % 17;
		let t_5 = _to_str(GLOBAL_DATA, 1, 1 + t_4 as usize);
		let mut t_6 = rusqlite::Batch::new(t_3, t_5);
		let mut t_7 = &mut t_6;
		let t_8 = rusqlite::Batch::next(t_7);
		let t_9 = _unwrap_result(t_8);
		let mut t_10 = _unwrap_option(t_9);
		let mut t_11 = &mut t_10;
		let t_12 = _to_usize(GLOBAL_DATA, 17);
		let mut t_56 = _to_u8(GLOBAL_DATA, 107) % 17;
		let t_57 = _to_str(GLOBAL_DATA, 108, 108 + t_56 as usize);
		let t_58 = String::from(t_57);
		let t_59 = CustomType0(t_58);
		t_11.raw_bind_parameter(t_12, t_59);
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