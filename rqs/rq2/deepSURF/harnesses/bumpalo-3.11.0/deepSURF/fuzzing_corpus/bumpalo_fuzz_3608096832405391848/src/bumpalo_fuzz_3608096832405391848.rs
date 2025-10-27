//DefId(0:156 ~ bumpalo[9aa9]::{impl#8}::alloc_slice_copy)
#[macro_use]
extern crate afl;

use bumpalo::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

struct CustomType0(usize);

impl core::marker::Copy for CustomType0 {
}

impl core::clone::Clone for CustomType0 {
	
	fn clone(&self) -> Self {
		let global_data = get_global_data();
		let GLOBAL_DATA = global_data.first_half;
		let custom_impl_num = _to_usize(GLOBAL_DATA, 1);
		let custom_impl_inst_num = self.0;
		let selector = (custom_impl_num + custom_impl_inst_num) % 3;
		if selector == 0{
			panic!("INTENTIONAL PANIC!");
		}
		let GLOBAL_DATA = match selector{
				1 => global_data.first_half,
				_ => global_data.second_half,
		};
		let t_4 = _to_usize(GLOBAL_DATA, 9);
		let t_5 = CustomType0(t_4);
		return t_5;
	}
}

fn main (){
	fuzz_nohook!(|data: &[u8]| {
		if data.len() < 546 {return;}
		set_global_data(data);
		let global_data = get_global_data();
		let GLOBAL_DATA = global_data.first_half;
		let t_0 = bumpalo::Bump::new();
		let t_1 = &t_0;
		let mut t_2 = _to_u8(GLOBAL_DATA, 0) % 33;
		// Start vector declaration.
		let mut t_3 = std::vec::Vec::with_capacity(32);
		let t_6 = _to_usize(GLOBAL_DATA, 17);
		let t_7 = CustomType0(t_6);
		t_3.push(t_7);
		let t_8 = _to_usize(GLOBAL_DATA, 25);
		let t_9 = CustomType0(t_8);
		t_3.push(t_9);
		let t_10 = _to_usize(GLOBAL_DATA, 33);
		let t_11 = CustomType0(t_10);
		t_3.push(t_11);
		let t_12 = _to_usize(GLOBAL_DATA, 41);
		let t_13 = CustomType0(t_12);
		t_3.push(t_13);
		let t_14 = _to_usize(GLOBAL_DATA, 49);
		let t_15 = CustomType0(t_14);
		t_3.push(t_15);
		let t_16 = _to_usize(GLOBAL_DATA, 57);
		let t_17 = CustomType0(t_16);
		t_3.push(t_17);
		let t_18 = _to_usize(GLOBAL_DATA, 65);
		let t_19 = CustomType0(t_18);
		t_3.push(t_19);
		let t_20 = _to_usize(GLOBAL_DATA, 73);
		let t_21 = CustomType0(t_20);
		t_3.push(t_21);
		let t_22 = _to_usize(GLOBAL_DATA, 81);
		let t_23 = CustomType0(t_22);
		t_3.push(t_23);
		let t_24 = _to_usize(GLOBAL_DATA, 89);
		let t_25 = CustomType0(t_24);
		t_3.push(t_25);
		let t_26 = _to_usize(GLOBAL_DATA, 97);
		let t_27 = CustomType0(t_26);
		t_3.push(t_27);
		let t_28 = _to_usize(GLOBAL_DATA, 105);
		let t_29 = CustomType0(t_28);
		t_3.push(t_29);
		let t_30 = _to_usize(GLOBAL_DATA, 113);
		let t_31 = CustomType0(t_30);
		t_3.push(t_31);
		let t_32 = _to_usize(GLOBAL_DATA, 121);
		let t_33 = CustomType0(t_32);
		t_3.push(t_33);
		let t_34 = _to_usize(GLOBAL_DATA, 129);
		let t_35 = CustomType0(t_34);
		t_3.push(t_35);
		let t_36 = _to_usize(GLOBAL_DATA, 137);
		let t_37 = CustomType0(t_36);
		t_3.push(t_37);
		let t_38 = _to_usize(GLOBAL_DATA, 145);
		let t_39 = CustomType0(t_38);
		t_3.push(t_39);
		let t_40 = _to_usize(GLOBAL_DATA, 153);
		let t_41 = CustomType0(t_40);
		t_3.push(t_41);
		let t_42 = _to_usize(GLOBAL_DATA, 161);
		let t_43 = CustomType0(t_42);
		t_3.push(t_43);
		let t_44 = _to_usize(GLOBAL_DATA, 169);
		let t_45 = CustomType0(t_44);
		t_3.push(t_45);
		let t_46 = _to_usize(GLOBAL_DATA, 177);
		let t_47 = CustomType0(t_46);
		t_3.push(t_47);
		let t_48 = _to_usize(GLOBAL_DATA, 185);
		let t_49 = CustomType0(t_48);
		t_3.push(t_49);
		let t_50 = _to_usize(GLOBAL_DATA, 193);
		let t_51 = CustomType0(t_50);
		t_3.push(t_51);
		let t_52 = _to_usize(GLOBAL_DATA, 201);
		let t_53 = CustomType0(t_52);
		t_3.push(t_53);
		let t_54 = _to_usize(GLOBAL_DATA, 209);
		let t_55 = CustomType0(t_54);
		t_3.push(t_55);
		let t_56 = _to_usize(GLOBAL_DATA, 217);
		let t_57 = CustomType0(t_56);
		t_3.push(t_57);
		let t_58 = _to_usize(GLOBAL_DATA, 225);
		let t_59 = CustomType0(t_58);
		t_3.push(t_59);
		let t_60 = _to_usize(GLOBAL_DATA, 233);
		let t_61 = CustomType0(t_60);
		t_3.push(t_61);
		let t_62 = _to_usize(GLOBAL_DATA, 241);
		let t_63 = CustomType0(t_62);
		t_3.push(t_63);
		let t_64 = _to_usize(GLOBAL_DATA, 249);
		let t_65 = CustomType0(t_64);
		t_3.push(t_65);
		let t_66 = _to_usize(GLOBAL_DATA, 257);
		let t_67 = CustomType0(t_66);
		t_3.push(t_67);
		let t_68 = _to_usize(GLOBAL_DATA, 265);
		let t_69 = CustomType0(t_68);
		t_3.push(t_69);
		t_3.truncate(t_2 as usize);
		// End vector declaration.
		let t_70 = &t_3[..];
		t_1.alloc_slice_copy(t_70);
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