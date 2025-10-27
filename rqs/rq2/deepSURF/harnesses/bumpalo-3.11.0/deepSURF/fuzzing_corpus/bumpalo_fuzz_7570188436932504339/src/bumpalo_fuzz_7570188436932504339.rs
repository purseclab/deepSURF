//DefId(0:158 ~ bumpalo[9aa9]::{impl#8}::alloc_slice_clone)
#[macro_use]
extern crate afl;

use bumpalo::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

struct CustomType0(String);

impl core::clone::Clone for CustomType0 {
	
	fn clone(&self) -> Self {
		let global_data = get_global_data();
		let GLOBAL_DATA = global_data.first_half;
		let custom_impl_num = _to_usize(GLOBAL_DATA, 1);
		let custom_impl_inst_num = self.0.len();
		let selector = (custom_impl_num + custom_impl_inst_num) % 3;
		if selector == 0{
			panic!("INTENTIONAL PANIC!");
		}
		let GLOBAL_DATA = match selector{
				1 => global_data.first_half,
				_ => global_data.second_half,
		};
		let mut t_4 = _to_u8(GLOBAL_DATA, 9) % 17;
		let t_5 = _to_str(GLOBAL_DATA, 10, 10 + t_4 as usize);
		let t_6 = String::from(t_5);
		let t_7 = CustomType0(t_6);
		return t_7;
	}
}

fn main (){
	fuzz_nohook!(|data: &[u8]| {
		if data.len() < 1140 {return;}
		set_global_data(data);
		let global_data = get_global_data();
		let GLOBAL_DATA = global_data.first_half;
		let t_0 = bumpalo::Bump::new();
		let t_1 = &t_0;
		let mut t_2 = _to_u8(GLOBAL_DATA, 0) % 33;
		// Start vector declaration.
		let mut t_3 = std::vec::Vec::with_capacity(32);
		let mut t_8 = _to_u8(GLOBAL_DATA, 26) % 17;
		let t_9 = _to_str(GLOBAL_DATA, 27, 27 + t_8 as usize);
		let t_10 = String::from(t_9);
		let t_11 = CustomType0(t_10);
		t_3.push(t_11);
		let mut t_12 = _to_u8(GLOBAL_DATA, 43) % 17;
		let t_13 = _to_str(GLOBAL_DATA, 44, 44 + t_12 as usize);
		let t_14 = String::from(t_13);
		let t_15 = CustomType0(t_14);
		t_3.push(t_15);
		let mut t_16 = _to_u8(GLOBAL_DATA, 60) % 17;
		let t_17 = _to_str(GLOBAL_DATA, 61, 61 + t_16 as usize);
		let t_18 = String::from(t_17);
		let t_19 = CustomType0(t_18);
		t_3.push(t_19);
		let mut t_20 = _to_u8(GLOBAL_DATA, 77) % 17;
		let t_21 = _to_str(GLOBAL_DATA, 78, 78 + t_20 as usize);
		let t_22 = String::from(t_21);
		let t_23 = CustomType0(t_22);
		t_3.push(t_23);
		let mut t_24 = _to_u8(GLOBAL_DATA, 94) % 17;
		let t_25 = _to_str(GLOBAL_DATA, 95, 95 + t_24 as usize);
		let t_26 = String::from(t_25);
		let t_27 = CustomType0(t_26);
		t_3.push(t_27);
		let mut t_28 = _to_u8(GLOBAL_DATA, 111) % 17;
		let t_29 = _to_str(GLOBAL_DATA, 112, 112 + t_28 as usize);
		let t_30 = String::from(t_29);
		let t_31 = CustomType0(t_30);
		t_3.push(t_31);
		let mut t_32 = _to_u8(GLOBAL_DATA, 128) % 17;
		let t_33 = _to_str(GLOBAL_DATA, 129, 129 + t_32 as usize);
		let t_34 = String::from(t_33);
		let t_35 = CustomType0(t_34);
		t_3.push(t_35);
		let mut t_36 = _to_u8(GLOBAL_DATA, 145) % 17;
		let t_37 = _to_str(GLOBAL_DATA, 146, 146 + t_36 as usize);
		let t_38 = String::from(t_37);
		let t_39 = CustomType0(t_38);
		t_3.push(t_39);
		let mut t_40 = _to_u8(GLOBAL_DATA, 162) % 17;
		let t_41 = _to_str(GLOBAL_DATA, 163, 163 + t_40 as usize);
		let t_42 = String::from(t_41);
		let t_43 = CustomType0(t_42);
		t_3.push(t_43);
		let mut t_44 = _to_u8(GLOBAL_DATA, 179) % 17;
		let t_45 = _to_str(GLOBAL_DATA, 180, 180 + t_44 as usize);
		let t_46 = String::from(t_45);
		let t_47 = CustomType0(t_46);
		t_3.push(t_47);
		let mut t_48 = _to_u8(GLOBAL_DATA, 196) % 17;
		let t_49 = _to_str(GLOBAL_DATA, 197, 197 + t_48 as usize);
		let t_50 = String::from(t_49);
		let t_51 = CustomType0(t_50);
		t_3.push(t_51);
		let mut t_52 = _to_u8(GLOBAL_DATA, 213) % 17;
		let t_53 = _to_str(GLOBAL_DATA, 214, 214 + t_52 as usize);
		let t_54 = String::from(t_53);
		let t_55 = CustomType0(t_54);
		t_3.push(t_55);
		let mut t_56 = _to_u8(GLOBAL_DATA, 230) % 17;
		let t_57 = _to_str(GLOBAL_DATA, 231, 231 + t_56 as usize);
		let t_58 = String::from(t_57);
		let t_59 = CustomType0(t_58);
		t_3.push(t_59);
		let mut t_60 = _to_u8(GLOBAL_DATA, 247) % 17;
		let t_61 = _to_str(GLOBAL_DATA, 248, 248 + t_60 as usize);
		let t_62 = String::from(t_61);
		let t_63 = CustomType0(t_62);
		t_3.push(t_63);
		let mut t_64 = _to_u8(GLOBAL_DATA, 264) % 17;
		let t_65 = _to_str(GLOBAL_DATA, 265, 265 + t_64 as usize);
		let t_66 = String::from(t_65);
		let t_67 = CustomType0(t_66);
		t_3.push(t_67);
		let mut t_68 = _to_u8(GLOBAL_DATA, 281) % 17;
		let t_69 = _to_str(GLOBAL_DATA, 282, 282 + t_68 as usize);
		let t_70 = String::from(t_69);
		let t_71 = CustomType0(t_70);
		t_3.push(t_71);
		let mut t_72 = _to_u8(GLOBAL_DATA, 298) % 17;
		let t_73 = _to_str(GLOBAL_DATA, 299, 299 + t_72 as usize);
		let t_74 = String::from(t_73);
		let t_75 = CustomType0(t_74);
		t_3.push(t_75);
		let mut t_76 = _to_u8(GLOBAL_DATA, 315) % 17;
		let t_77 = _to_str(GLOBAL_DATA, 316, 316 + t_76 as usize);
		let t_78 = String::from(t_77);
		let t_79 = CustomType0(t_78);
		t_3.push(t_79);
		let mut t_80 = _to_u8(GLOBAL_DATA, 332) % 17;
		let t_81 = _to_str(GLOBAL_DATA, 333, 333 + t_80 as usize);
		let t_82 = String::from(t_81);
		let t_83 = CustomType0(t_82);
		t_3.push(t_83);
		let mut t_84 = _to_u8(GLOBAL_DATA, 349) % 17;
		let t_85 = _to_str(GLOBAL_DATA, 350, 350 + t_84 as usize);
		let t_86 = String::from(t_85);
		let t_87 = CustomType0(t_86);
		t_3.push(t_87);
		let mut t_88 = _to_u8(GLOBAL_DATA, 366) % 17;
		let t_89 = _to_str(GLOBAL_DATA, 367, 367 + t_88 as usize);
		let t_90 = String::from(t_89);
		let t_91 = CustomType0(t_90);
		t_3.push(t_91);
		let mut t_92 = _to_u8(GLOBAL_DATA, 383) % 17;
		let t_93 = _to_str(GLOBAL_DATA, 384, 384 + t_92 as usize);
		let t_94 = String::from(t_93);
		let t_95 = CustomType0(t_94);
		t_3.push(t_95);
		let mut t_96 = _to_u8(GLOBAL_DATA, 400) % 17;
		let t_97 = _to_str(GLOBAL_DATA, 401, 401 + t_96 as usize);
		let t_98 = String::from(t_97);
		let t_99 = CustomType0(t_98);
		t_3.push(t_99);
		let mut t_100 = _to_u8(GLOBAL_DATA, 417) % 17;
		let t_101 = _to_str(GLOBAL_DATA, 418, 418 + t_100 as usize);
		let t_102 = String::from(t_101);
		let t_103 = CustomType0(t_102);
		t_3.push(t_103);
		let mut t_104 = _to_u8(GLOBAL_DATA, 434) % 17;
		let t_105 = _to_str(GLOBAL_DATA, 435, 435 + t_104 as usize);
		let t_106 = String::from(t_105);
		let t_107 = CustomType0(t_106);
		t_3.push(t_107);
		let mut t_108 = _to_u8(GLOBAL_DATA, 451) % 17;
		let t_109 = _to_str(GLOBAL_DATA, 452, 452 + t_108 as usize);
		let t_110 = String::from(t_109);
		let t_111 = CustomType0(t_110);
		t_3.push(t_111);
		let mut t_112 = _to_u8(GLOBAL_DATA, 468) % 17;
		let t_113 = _to_str(GLOBAL_DATA, 469, 469 + t_112 as usize);
		let t_114 = String::from(t_113);
		let t_115 = CustomType0(t_114);
		t_3.push(t_115);
		let mut t_116 = _to_u8(GLOBAL_DATA, 485) % 17;
		let t_117 = _to_str(GLOBAL_DATA, 486, 486 + t_116 as usize);
		let t_118 = String::from(t_117);
		let t_119 = CustomType0(t_118);
		t_3.push(t_119);
		let mut t_120 = _to_u8(GLOBAL_DATA, 502) % 17;
		let t_121 = _to_str(GLOBAL_DATA, 503, 503 + t_120 as usize);
		let t_122 = String::from(t_121);
		let t_123 = CustomType0(t_122);
		t_3.push(t_123);
		let mut t_124 = _to_u8(GLOBAL_DATA, 519) % 17;
		let t_125 = _to_str(GLOBAL_DATA, 520, 520 + t_124 as usize);
		let t_126 = String::from(t_125);
		let t_127 = CustomType0(t_126);
		t_3.push(t_127);
		let mut t_128 = _to_u8(GLOBAL_DATA, 536) % 17;
		let t_129 = _to_str(GLOBAL_DATA, 537, 537 + t_128 as usize);
		let t_130 = String::from(t_129);
		let t_131 = CustomType0(t_130);
		t_3.push(t_131);
		let mut t_132 = _to_u8(GLOBAL_DATA, 553) % 17;
		let t_133 = _to_str(GLOBAL_DATA, 554, 554 + t_132 as usize);
		let t_134 = String::from(t_133);
		let t_135 = CustomType0(t_134);
		t_3.push(t_135);
		t_3.truncate(t_2 as usize);
		// End vector declaration.
		let t_136 = &t_3[..];
		t_1.alloc_slice_clone(t_136);
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