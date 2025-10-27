//DefId(0:406 ~ secp256k1[1a2a]::ecdsa::{impl#14}::sign)
#[macro_use]
extern crate afl;

use secp256k1::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

struct CustomType0(String);

impl secp256k1::ThirtyTwoByteHash for CustomType0 {
	
	fn into_32(self) -> [u8; 32] {
		let global_data = get_global_data();
		let GLOBAL_DATA = global_data.first_half;
		let custom_impl_num = _to_usize(GLOBAL_DATA, 0);
		let custom_impl_inst_num = self.0.len();
		let selector = (custom_impl_num + custom_impl_inst_num) % 3;
		if selector == 0{
			panic!("INTENTIONAL PANIC!");
		}
		let GLOBAL_DATA = match selector{
				1 => global_data.first_half,
				_ => global_data.second_half,
		};
		// Start vector declaration.
		let mut t_2 = std::vec::Vec::with_capacity(32);
		let t_3 = _to_u8(GLOBAL_DATA, 8);
		t_2.push(t_3);
		let t_4 = _to_u8(GLOBAL_DATA, 9);
		t_2.push(t_4);
		let t_5 = _to_u8(GLOBAL_DATA, 10);
		t_2.push(t_5);
		let t_6 = _to_u8(GLOBAL_DATA, 11);
		t_2.push(t_6);
		let t_7 = _to_u8(GLOBAL_DATA, 12);
		t_2.push(t_7);
		let t_8 = _to_u8(GLOBAL_DATA, 13);
		t_2.push(t_8);
		let t_9 = _to_u8(GLOBAL_DATA, 14);
		t_2.push(t_9);
		let t_10 = _to_u8(GLOBAL_DATA, 15);
		t_2.push(t_10);
		let t_11 = _to_u8(GLOBAL_DATA, 16);
		t_2.push(t_11);
		let t_12 = _to_u8(GLOBAL_DATA, 17);
		t_2.push(t_12);
		let t_13 = _to_u8(GLOBAL_DATA, 18);
		t_2.push(t_13);
		let t_14 = _to_u8(GLOBAL_DATA, 19);
		t_2.push(t_14);
		let t_15 = _to_u8(GLOBAL_DATA, 20);
		t_2.push(t_15);
		let t_16 = _to_u8(GLOBAL_DATA, 21);
		t_2.push(t_16);
		let t_17 = _to_u8(GLOBAL_DATA, 22);
		t_2.push(t_17);
		let t_18 = _to_u8(GLOBAL_DATA, 23);
		t_2.push(t_18);
		let t_19 = _to_u8(GLOBAL_DATA, 24);
		t_2.push(t_19);
		let t_20 = _to_u8(GLOBAL_DATA, 25);
		t_2.push(t_20);
		let t_21 = _to_u8(GLOBAL_DATA, 26);
		t_2.push(t_21);
		let t_22 = _to_u8(GLOBAL_DATA, 27);
		t_2.push(t_22);
		let t_23 = _to_u8(GLOBAL_DATA, 28);
		t_2.push(t_23);
		let t_24 = _to_u8(GLOBAL_DATA, 29);
		t_2.push(t_24);
		let t_25 = _to_u8(GLOBAL_DATA, 30);
		t_2.push(t_25);
		let t_26 = _to_u8(GLOBAL_DATA, 31);
		t_2.push(t_26);
		let t_27 = _to_u8(GLOBAL_DATA, 32);
		t_2.push(t_27);
		let t_28 = _to_u8(GLOBAL_DATA, 33);
		t_2.push(t_28);
		let t_29 = _to_u8(GLOBAL_DATA, 34);
		t_2.push(t_29);
		let t_30 = _to_u8(GLOBAL_DATA, 35);
		t_2.push(t_30);
		let t_31 = _to_u8(GLOBAL_DATA, 36);
		t_2.push(t_31);
		let t_32 = _to_u8(GLOBAL_DATA, 37);
		t_2.push(t_32);
		let t_33 = _to_u8(GLOBAL_DATA, 38);
		t_2.push(t_33);
		let t_34 = _to_u8(GLOBAL_DATA, 39);
		t_2.push(t_34);
		t_2.truncate(32);
		// End vector declaration.
		let t_35: [_; 32] = t_2.try_into().unwrap();
		return t_35;
	}
}

fn main (){
	fuzz_nohook!(|data: &[u8]| {
		if data.len() < 148 {return;}
		set_global_data(data);
		let global_data = get_global_data();
		let GLOBAL_DATA = global_data.first_half;
		let t_0 = secp256k1::Secp256k1::new();
		let t_1 = &t_0;
		let mut t_36 = _to_u8(GLOBAL_DATA, 40) % 17;
		let t_37 = _to_str(GLOBAL_DATA, 41, 41 + t_36 as usize);
		let t_38 = String::from(t_37);
		let t_39 = CustomType0(t_38);
		let t_40 = secp256k1::Message::from(t_39);
		let t_41 = &t_40;
		let mut t_42 = _to_u8(GLOBAL_DATA, 57) % 17;
		let t_43 = _to_str(GLOBAL_DATA, 58, 58 + t_42 as usize);
		let t_44 = secp256k1::KeyPair::from_str(t_43);
		let t_45 = _unwrap_result(t_44);
		let t_46 = secp256k1::SecretKey::from(t_45);
		let t_47 = &t_46;
		t_1.sign(t_41, t_47);
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