//DefId(0:156 ~ bumpalo[9aa9]::{impl#8}::alloc_slice_copy)
#[macro_use]
extern crate afl;

use bumpalo::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

struct CustomType0(usize);

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
		let t_5 = _to_usize(GLOBAL_DATA, 9);
		let t_6 = CustomType0(t_5);
		return t_6;
	}
}

impl core::marker::Copy for CustomType0 {
}

fn main (){
	fuzz_nohook!(|data: &[u8]| {
		if data.len() < 546 {return;}
		set_global_data(data);
		let global_data = get_global_data();
		let GLOBAL_DATA = global_data.first_half;
		let t_0 = bumpalo::Bump::try_new();
		let t_1 = _unwrap_result(t_0);
		let t_2 = &t_1;
		let mut t_3 = _to_u8(GLOBAL_DATA, 0) % 33;
		// Start vector declaration.
		let mut t_4 = std::vec::Vec::with_capacity(32);
		let t_7 = _to_usize(GLOBAL_DATA, 17);
		let t_8 = CustomType0(t_7);
		t_4.push(t_8);
		let t_9 = _to_usize(GLOBAL_DATA, 25);
		let t_10 = CustomType0(t_9);
		t_4.push(t_10);
		let t_11 = _to_usize(GLOBAL_DATA, 33);
		let t_12 = CustomType0(t_11);
		t_4.push(t_12);
		let t_13 = _to_usize(GLOBAL_DATA, 41);
		let t_14 = CustomType0(t_13);
		t_4.push(t_14);
		let t_15 = _to_usize(GLOBAL_DATA, 49);
		let t_16 = CustomType0(t_15);
		t_4.push(t_16);
		let t_17 = _to_usize(GLOBAL_DATA, 57);
		let t_18 = CustomType0(t_17);
		t_4.push(t_18);
		let t_19 = _to_usize(GLOBAL_DATA, 65);
		let t_20 = CustomType0(t_19);
		t_4.push(t_20);
		let t_21 = _to_usize(GLOBAL_DATA, 73);
		let t_22 = CustomType0(t_21);
		t_4.push(t_22);
		let t_23 = _to_usize(GLOBAL_DATA, 81);
		let t_24 = CustomType0(t_23);
		t_4.push(t_24);
		let t_25 = _to_usize(GLOBAL_DATA, 89);
		let t_26 = CustomType0(t_25);
		t_4.push(t_26);
		let t_27 = _to_usize(GLOBAL_DATA, 97);
		let t_28 = CustomType0(t_27);
		t_4.push(t_28);
		let t_29 = _to_usize(GLOBAL_DATA, 105);
		let t_30 = CustomType0(t_29);
		t_4.push(t_30);
		let t_31 = _to_usize(GLOBAL_DATA, 113);
		let t_32 = CustomType0(t_31);
		t_4.push(t_32);
		let t_33 = _to_usize(GLOBAL_DATA, 121);
		let t_34 = CustomType0(t_33);
		t_4.push(t_34);
		let t_35 = _to_usize(GLOBAL_DATA, 129);
		let t_36 = CustomType0(t_35);
		t_4.push(t_36);
		let t_37 = _to_usize(GLOBAL_DATA, 137);
		let t_38 = CustomType0(t_37);
		t_4.push(t_38);
		let t_39 = _to_usize(GLOBAL_DATA, 145);
		let t_40 = CustomType0(t_39);
		t_4.push(t_40);
		let t_41 = _to_usize(GLOBAL_DATA, 153);
		let t_42 = CustomType0(t_41);
		t_4.push(t_42);
		let t_43 = _to_usize(GLOBAL_DATA, 161);
		let t_44 = CustomType0(t_43);
		t_4.push(t_44);
		let t_45 = _to_usize(GLOBAL_DATA, 169);
		let t_46 = CustomType0(t_45);
		t_4.push(t_46);
		let t_47 = _to_usize(GLOBAL_DATA, 177);
		let t_48 = CustomType0(t_47);
		t_4.push(t_48);
		let t_49 = _to_usize(GLOBAL_DATA, 185);
		let t_50 = CustomType0(t_49);
		t_4.push(t_50);
		let t_51 = _to_usize(GLOBAL_DATA, 193);
		let t_52 = CustomType0(t_51);
		t_4.push(t_52);
		let t_53 = _to_usize(GLOBAL_DATA, 201);
		let t_54 = CustomType0(t_53);
		t_4.push(t_54);
		let t_55 = _to_usize(GLOBAL_DATA, 209);
		let t_56 = CustomType0(t_55);
		t_4.push(t_56);
		let t_57 = _to_usize(GLOBAL_DATA, 217);
		let t_58 = CustomType0(t_57);
		t_4.push(t_58);
		let t_59 = _to_usize(GLOBAL_DATA, 225);
		let t_60 = CustomType0(t_59);
		t_4.push(t_60);
		let t_61 = _to_usize(GLOBAL_DATA, 233);
		let t_62 = CustomType0(t_61);
		t_4.push(t_62);
		let t_63 = _to_usize(GLOBAL_DATA, 241);
		let t_64 = CustomType0(t_63);
		t_4.push(t_64);
		let t_65 = _to_usize(GLOBAL_DATA, 249);
		let t_66 = CustomType0(t_65);
		t_4.push(t_66);
		let t_67 = _to_usize(GLOBAL_DATA, 257);
		let t_68 = CustomType0(t_67);
		t_4.push(t_68);
		let t_69 = _to_usize(GLOBAL_DATA, 265);
		let t_70 = CustomType0(t_69);
		t_4.push(t_70);
		t_4.truncate(t_3 as usize);
		// End vector declaration.
		let t_71 = &t_4[..];
		t_2.alloc_slice_copy(t_71);
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