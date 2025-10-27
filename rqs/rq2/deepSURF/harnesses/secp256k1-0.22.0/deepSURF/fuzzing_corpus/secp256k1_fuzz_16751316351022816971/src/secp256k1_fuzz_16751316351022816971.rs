//DefId(0:359 ~ secp256k1[1a2a]::ecdsa::{impl#1}::fmt)
#[macro_use]
extern crate afl;

use secp256k1::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};
use secp256k1_sys::*;


fn main (){
	fuzz_nohook!(|data: &[u8]| {
		if data.len() < 102 {return;}
		set_global_data(data);
		let global_data = get_global_data();
		let GLOBAL_DATA = global_data.first_half;
		let mut t_0 = _to_u8(GLOBAL_DATA, 0) % 33;
		// Start vector declaration.
		let mut t_1 = std::vec::Vec::with_capacity(32);
		let t_2 = secp256k1_sys::types::AlignedType::zeroed();
		t_1.push(t_2);
		let t_3 = secp256k1_sys::types::AlignedType::zeroed();
		t_1.push(t_3);
		let t_4 = secp256k1_sys::types::AlignedType::zeroed();
		t_1.push(t_4);
		let t_5 = secp256k1_sys::types::AlignedType::zeroed();
		t_1.push(t_5);
		let t_6 = secp256k1_sys::types::AlignedType::zeroed();
		t_1.push(t_6);
		let t_7 = secp256k1_sys::types::AlignedType::zeroed();
		t_1.push(t_7);
		let t_8 = secp256k1_sys::types::AlignedType::zeroed();
		t_1.push(t_8);
		let t_9 = secp256k1_sys::types::AlignedType::zeroed();
		t_1.push(t_9);
		let t_10 = secp256k1_sys::types::AlignedType::zeroed();
		t_1.push(t_10);
		let t_11 = secp256k1_sys::types::AlignedType::zeroed();
		t_1.push(t_11);
		let t_12 = secp256k1_sys::types::AlignedType::zeroed();
		t_1.push(t_12);
		let t_13 = secp256k1_sys::types::AlignedType::zeroed();
		t_1.push(t_13);
		let t_14 = secp256k1_sys::types::AlignedType::zeroed();
		t_1.push(t_14);
		let t_15 = secp256k1_sys::types::AlignedType::zeroed();
		t_1.push(t_15);
		let t_16 = secp256k1_sys::types::AlignedType::zeroed();
		t_1.push(t_16);
		let t_17 = secp256k1_sys::types::AlignedType::zeroed();
		t_1.push(t_17);
		let t_18 = secp256k1_sys::types::AlignedType::zeroed();
		t_1.push(t_18);
		let t_19 = secp256k1_sys::types::AlignedType::zeroed();
		t_1.push(t_19);
		let t_20 = secp256k1_sys::types::AlignedType::zeroed();
		t_1.push(t_20);
		let t_21 = secp256k1_sys::types::AlignedType::zeroed();
		t_1.push(t_21);
		let t_22 = secp256k1_sys::types::AlignedType::zeroed();
		t_1.push(t_22);
		let t_23 = secp256k1_sys::types::AlignedType::zeroed();
		t_1.push(t_23);
		let t_24 = secp256k1_sys::types::AlignedType::zeroed();
		t_1.push(t_24);
		let t_25 = secp256k1_sys::types::AlignedType::zeroed();
		t_1.push(t_25);
		let t_26 = secp256k1_sys::types::AlignedType::zeroed();
		t_1.push(t_26);
		let t_27 = secp256k1_sys::types::AlignedType::zeroed();
		t_1.push(t_27);
		let t_28 = secp256k1_sys::types::AlignedType::zeroed();
		t_1.push(t_28);
		let t_29 = secp256k1_sys::types::AlignedType::zeroed();
		t_1.push(t_29);
		let t_30 = secp256k1_sys::types::AlignedType::zeroed();
		t_1.push(t_30);
		let t_31 = secp256k1_sys::types::AlignedType::zeroed();
		t_1.push(t_31);
		let t_32 = secp256k1_sys::types::AlignedType::zeroed();
		t_1.push(t_32);
		let t_33 = secp256k1_sys::types::AlignedType::zeroed();
		t_1.push(t_33);
		t_1.truncate(t_0 as usize);
		// End vector declaration.
		let t_34 = &mut t_1[..];
		let t_35 = secp256k1::Secp256k1::preallocated_signing_only(t_34);
		let t_36 = _unwrap_result(t_35);
		let t_37 = &t_36;
		let mut t_38 = _to_u8(GLOBAL_DATA, 1) % 33;
		// Start vector declaration.
		let mut t_39 = std::vec::Vec::with_capacity(32);
		let t_40 = _to_u8(GLOBAL_DATA, 2);
		t_39.push(t_40);
		let t_41 = _to_u8(GLOBAL_DATA, 3);
		t_39.push(t_41);
		let t_42 = _to_u8(GLOBAL_DATA, 4);
		t_39.push(t_42);
		let t_43 = _to_u8(GLOBAL_DATA, 5);
		t_39.push(t_43);
		let t_44 = _to_u8(GLOBAL_DATA, 6);
		t_39.push(t_44);
		let t_45 = _to_u8(GLOBAL_DATA, 7);
		t_39.push(t_45);
		let t_46 = _to_u8(GLOBAL_DATA, 8);
		t_39.push(t_46);
		let t_47 = _to_u8(GLOBAL_DATA, 9);
		t_39.push(t_47);
		let t_48 = _to_u8(GLOBAL_DATA, 10);
		t_39.push(t_48);
		let t_49 = _to_u8(GLOBAL_DATA, 11);
		t_39.push(t_49);
		let t_50 = _to_u8(GLOBAL_DATA, 12);
		t_39.push(t_50);
		let t_51 = _to_u8(GLOBAL_DATA, 13);
		t_39.push(t_51);
		let t_52 = _to_u8(GLOBAL_DATA, 14);
		t_39.push(t_52);
		let t_53 = _to_u8(GLOBAL_DATA, 15);
		t_39.push(t_53);
		let t_54 = _to_u8(GLOBAL_DATA, 16);
		t_39.push(t_54);
		let t_55 = _to_u8(GLOBAL_DATA, 17);
		t_39.push(t_55);
		let t_56 = _to_u8(GLOBAL_DATA, 18);
		t_39.push(t_56);
		let t_57 = _to_u8(GLOBAL_DATA, 19);
		t_39.push(t_57);
		let t_58 = _to_u8(GLOBAL_DATA, 20);
		t_39.push(t_58);
		let t_59 = _to_u8(GLOBAL_DATA, 21);
		t_39.push(t_59);
		let t_60 = _to_u8(GLOBAL_DATA, 22);
		t_39.push(t_60);
		let t_61 = _to_u8(GLOBAL_DATA, 23);
		t_39.push(t_61);
		let t_62 = _to_u8(GLOBAL_DATA, 24);
		t_39.push(t_62);
		let t_63 = _to_u8(GLOBAL_DATA, 25);
		t_39.push(t_63);
		let t_64 = _to_u8(GLOBAL_DATA, 26);
		t_39.push(t_64);
		let t_65 = _to_u8(GLOBAL_DATA, 27);
		t_39.push(t_65);
		let t_66 = _to_u8(GLOBAL_DATA, 28);
		t_39.push(t_66);
		let t_67 = _to_u8(GLOBAL_DATA, 29);
		t_39.push(t_67);
		let t_68 = _to_u8(GLOBAL_DATA, 30);
		t_39.push(t_68);
		let t_69 = _to_u8(GLOBAL_DATA, 31);
		t_39.push(t_69);
		let t_70 = _to_u8(GLOBAL_DATA, 32);
		t_39.push(t_70);
		let t_71 = _to_u8(GLOBAL_DATA, 33);
		t_39.push(t_71);
		t_39.truncate(t_38 as usize);
		// End vector declaration.
		let t_72 = &t_39[..];
		let t_73 = secp256k1::Message::from_slice(t_72);
		let t_74 = _unwrap_result(t_73);
		let t_75 = &t_74;
		let mut t_76 = _to_u8(GLOBAL_DATA, 34) % 17;
		let t_77 = _to_str(GLOBAL_DATA, 35, 35 + t_76 as usize);
		let t_78 = secp256k1::KeyPair::from_str(t_77);
		let t_79 = _unwrap_result(t_78);
		let t_80 = &t_79;
		let t_81 = secp256k1::SecretKey::from(t_80);
		let t_82 = &t_81;
		let t_83 = secp256k1::Secp256k1::sign_ecdsa(t_37, t_75, t_82);
		let t_84 = &t_83;
		println!("{}", t_84);
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