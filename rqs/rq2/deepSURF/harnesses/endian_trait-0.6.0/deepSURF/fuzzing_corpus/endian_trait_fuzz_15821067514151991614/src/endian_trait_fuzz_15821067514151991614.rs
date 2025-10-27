//DefId(0:28 ~ endian_trait[144d]::slices::{impl#0}::from_be)
#[macro_use]
extern crate afl;

use endian_trait::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

struct CustomType0(String);

impl endian_trait::Endian for CustomType0 {
	
	fn to_be(self) -> Self {
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
		let mut t_2 = _to_u8(GLOBAL_DATA, 9) % 17;
		let t_3 = _to_str(GLOBAL_DATA, 10, 10 + t_2 as usize);
		let t_4 = String::from(t_3);
		let t_5 = CustomType0(t_4);
		return t_5;
	}
	
	fn to_le(self) -> Self {
		let global_data = get_global_data();
		let GLOBAL_DATA = global_data.first_half;
		let custom_impl_num = _to_usize(GLOBAL_DATA, 26);
		let custom_impl_inst_num = self.0.len();
		let selector = (custom_impl_num + custom_impl_inst_num) % 3;
		if selector == 0{
			panic!("INTENTIONAL PANIC!");
		}
		let GLOBAL_DATA = match selector{
				1 => global_data.first_half,
				_ => global_data.second_half,
		};
		let mut t_6 = _to_u8(GLOBAL_DATA, 34) % 17;
		let t_7 = _to_str(GLOBAL_DATA, 35, 35 + t_6 as usize);
		let t_8 = String::from(t_7);
		let t_9 = CustomType0(t_8);
		return t_9;
	}
	
	fn from_le(self) -> Self {
		let global_data = get_global_data();
		let GLOBAL_DATA = global_data.first_half;
		let custom_impl_num = _to_usize(GLOBAL_DATA, 51);
		let custom_impl_inst_num = self.0.len();
		let selector = (custom_impl_num + custom_impl_inst_num) % 3;
		if selector == 0{
			panic!("INTENTIONAL PANIC!");
		}
		let GLOBAL_DATA = match selector{
				1 => global_data.first_half,
				_ => global_data.second_half,
		};
		let mut t_10 = _to_u8(GLOBAL_DATA, 59) % 17;
		let t_11 = _to_str(GLOBAL_DATA, 60, 60 + t_10 as usize);
		let t_12 = String::from(t_11);
		let t_13 = CustomType0(t_12);
		return t_13;
	}
	
	fn from_be(self) -> Self {
		let global_data = get_global_data();
		let GLOBAL_DATA = global_data.first_half;
		let custom_impl_num = _to_usize(GLOBAL_DATA, 76);
		let custom_impl_inst_num = self.0.len();
		let selector = (custom_impl_num + custom_impl_inst_num) % 3;
		if selector == 0{
			panic!("INTENTIONAL PANIC!");
		}
		let GLOBAL_DATA = match selector{
				1 => global_data.first_half,
				_ => global_data.second_half,
		};
		let mut t_14 = _to_u8(GLOBAL_DATA, 84) % 17;
		let t_15 = _to_str(GLOBAL_DATA, 85, 85 + t_14 as usize);
		let t_16 = String::from(t_15);
		let t_17 = CustomType0(t_16);
		return t_17;
	}
}

fn main (){
	fuzz_nohook!(|data: &[u8]| {
		if data.len() < 1290 {return;}
		set_global_data(data);
		let global_data = get_global_data();
		let GLOBAL_DATA = global_data.first_half;
		let mut t_0 = _to_u8(GLOBAL_DATA, 0) % 33;
		// Start vector declaration.
		let mut t_1 = std::vec::Vec::with_capacity(32);
		let mut t_18 = _to_u8(GLOBAL_DATA, 101) % 17;
		let t_19 = _to_str(GLOBAL_DATA, 102, 102 + t_18 as usize);
		let t_20 = String::from(t_19);
		let t_21 = CustomType0(t_20);
		t_1.push(t_21);
		let mut t_22 = _to_u8(GLOBAL_DATA, 118) % 17;
		let t_23 = _to_str(GLOBAL_DATA, 119, 119 + t_22 as usize);
		let t_24 = String::from(t_23);
		let t_25 = CustomType0(t_24);
		t_1.push(t_25);
		let mut t_26 = _to_u8(GLOBAL_DATA, 135) % 17;
		let t_27 = _to_str(GLOBAL_DATA, 136, 136 + t_26 as usize);
		let t_28 = String::from(t_27);
		let t_29 = CustomType0(t_28);
		t_1.push(t_29);
		let mut t_30 = _to_u8(GLOBAL_DATA, 152) % 17;
		let t_31 = _to_str(GLOBAL_DATA, 153, 153 + t_30 as usize);
		let t_32 = String::from(t_31);
		let t_33 = CustomType0(t_32);
		t_1.push(t_33);
		let mut t_34 = _to_u8(GLOBAL_DATA, 169) % 17;
		let t_35 = _to_str(GLOBAL_DATA, 170, 170 + t_34 as usize);
		let t_36 = String::from(t_35);
		let t_37 = CustomType0(t_36);
		t_1.push(t_37);
		let mut t_38 = _to_u8(GLOBAL_DATA, 186) % 17;
		let t_39 = _to_str(GLOBAL_DATA, 187, 187 + t_38 as usize);
		let t_40 = String::from(t_39);
		let t_41 = CustomType0(t_40);
		t_1.push(t_41);
		let mut t_42 = _to_u8(GLOBAL_DATA, 203) % 17;
		let t_43 = _to_str(GLOBAL_DATA, 204, 204 + t_42 as usize);
		let t_44 = String::from(t_43);
		let t_45 = CustomType0(t_44);
		t_1.push(t_45);
		let mut t_46 = _to_u8(GLOBAL_DATA, 220) % 17;
		let t_47 = _to_str(GLOBAL_DATA, 221, 221 + t_46 as usize);
		let t_48 = String::from(t_47);
		let t_49 = CustomType0(t_48);
		t_1.push(t_49);
		let mut t_50 = _to_u8(GLOBAL_DATA, 237) % 17;
		let t_51 = _to_str(GLOBAL_DATA, 238, 238 + t_50 as usize);
		let t_52 = String::from(t_51);
		let t_53 = CustomType0(t_52);
		t_1.push(t_53);
		let mut t_54 = _to_u8(GLOBAL_DATA, 254) % 17;
		let t_55 = _to_str(GLOBAL_DATA, 255, 255 + t_54 as usize);
		let t_56 = String::from(t_55);
		let t_57 = CustomType0(t_56);
		t_1.push(t_57);
		let mut t_58 = _to_u8(GLOBAL_DATA, 271) % 17;
		let t_59 = _to_str(GLOBAL_DATA, 272, 272 + t_58 as usize);
		let t_60 = String::from(t_59);
		let t_61 = CustomType0(t_60);
		t_1.push(t_61);
		let mut t_62 = _to_u8(GLOBAL_DATA, 288) % 17;
		let t_63 = _to_str(GLOBAL_DATA, 289, 289 + t_62 as usize);
		let t_64 = String::from(t_63);
		let t_65 = CustomType0(t_64);
		t_1.push(t_65);
		let mut t_66 = _to_u8(GLOBAL_DATA, 305) % 17;
		let t_67 = _to_str(GLOBAL_DATA, 306, 306 + t_66 as usize);
		let t_68 = String::from(t_67);
		let t_69 = CustomType0(t_68);
		t_1.push(t_69);
		let mut t_70 = _to_u8(GLOBAL_DATA, 322) % 17;
		let t_71 = _to_str(GLOBAL_DATA, 323, 323 + t_70 as usize);
		let t_72 = String::from(t_71);
		let t_73 = CustomType0(t_72);
		t_1.push(t_73);
		let mut t_74 = _to_u8(GLOBAL_DATA, 339) % 17;
		let t_75 = _to_str(GLOBAL_DATA, 340, 340 + t_74 as usize);
		let t_76 = String::from(t_75);
		let t_77 = CustomType0(t_76);
		t_1.push(t_77);
		let mut t_78 = _to_u8(GLOBAL_DATA, 356) % 17;
		let t_79 = _to_str(GLOBAL_DATA, 357, 357 + t_78 as usize);
		let t_80 = String::from(t_79);
		let t_81 = CustomType0(t_80);
		t_1.push(t_81);
		let mut t_82 = _to_u8(GLOBAL_DATA, 373) % 17;
		let t_83 = _to_str(GLOBAL_DATA, 374, 374 + t_82 as usize);
		let t_84 = String::from(t_83);
		let t_85 = CustomType0(t_84);
		t_1.push(t_85);
		let mut t_86 = _to_u8(GLOBAL_DATA, 390) % 17;
		let t_87 = _to_str(GLOBAL_DATA, 391, 391 + t_86 as usize);
		let t_88 = String::from(t_87);
		let t_89 = CustomType0(t_88);
		t_1.push(t_89);
		let mut t_90 = _to_u8(GLOBAL_DATA, 407) % 17;
		let t_91 = _to_str(GLOBAL_DATA, 408, 408 + t_90 as usize);
		let t_92 = String::from(t_91);
		let t_93 = CustomType0(t_92);
		t_1.push(t_93);
		let mut t_94 = _to_u8(GLOBAL_DATA, 424) % 17;
		let t_95 = _to_str(GLOBAL_DATA, 425, 425 + t_94 as usize);
		let t_96 = String::from(t_95);
		let t_97 = CustomType0(t_96);
		t_1.push(t_97);
		let mut t_98 = _to_u8(GLOBAL_DATA, 441) % 17;
		let t_99 = _to_str(GLOBAL_DATA, 442, 442 + t_98 as usize);
		let t_100 = String::from(t_99);
		let t_101 = CustomType0(t_100);
		t_1.push(t_101);
		let mut t_102 = _to_u8(GLOBAL_DATA, 458) % 17;
		let t_103 = _to_str(GLOBAL_DATA, 459, 459 + t_102 as usize);
		let t_104 = String::from(t_103);
		let t_105 = CustomType0(t_104);
		t_1.push(t_105);
		let mut t_106 = _to_u8(GLOBAL_DATA, 475) % 17;
		let t_107 = _to_str(GLOBAL_DATA, 476, 476 + t_106 as usize);
		let t_108 = String::from(t_107);
		let t_109 = CustomType0(t_108);
		t_1.push(t_109);
		let mut t_110 = _to_u8(GLOBAL_DATA, 492) % 17;
		let t_111 = _to_str(GLOBAL_DATA, 493, 493 + t_110 as usize);
		let t_112 = String::from(t_111);
		let t_113 = CustomType0(t_112);
		t_1.push(t_113);
		let mut t_114 = _to_u8(GLOBAL_DATA, 509) % 17;
		let t_115 = _to_str(GLOBAL_DATA, 510, 510 + t_114 as usize);
		let t_116 = String::from(t_115);
		let t_117 = CustomType0(t_116);
		t_1.push(t_117);
		let mut t_118 = _to_u8(GLOBAL_DATA, 526) % 17;
		let t_119 = _to_str(GLOBAL_DATA, 527, 527 + t_118 as usize);
		let t_120 = String::from(t_119);
		let t_121 = CustomType0(t_120);
		t_1.push(t_121);
		let mut t_122 = _to_u8(GLOBAL_DATA, 543) % 17;
		let t_123 = _to_str(GLOBAL_DATA, 544, 544 + t_122 as usize);
		let t_124 = String::from(t_123);
		let t_125 = CustomType0(t_124);
		t_1.push(t_125);
		let mut t_126 = _to_u8(GLOBAL_DATA, 560) % 17;
		let t_127 = _to_str(GLOBAL_DATA, 561, 561 + t_126 as usize);
		let t_128 = String::from(t_127);
		let t_129 = CustomType0(t_128);
		t_1.push(t_129);
		let mut t_130 = _to_u8(GLOBAL_DATA, 577) % 17;
		let t_131 = _to_str(GLOBAL_DATA, 578, 578 + t_130 as usize);
		let t_132 = String::from(t_131);
		let t_133 = CustomType0(t_132);
		t_1.push(t_133);
		let mut t_134 = _to_u8(GLOBAL_DATA, 594) % 17;
		let t_135 = _to_str(GLOBAL_DATA, 595, 595 + t_134 as usize);
		let t_136 = String::from(t_135);
		let t_137 = CustomType0(t_136);
		t_1.push(t_137);
		let mut t_138 = _to_u8(GLOBAL_DATA, 611) % 17;
		let t_139 = _to_str(GLOBAL_DATA, 612, 612 + t_138 as usize);
		let t_140 = String::from(t_139);
		let t_141 = CustomType0(t_140);
		t_1.push(t_141);
		let mut t_142 = _to_u8(GLOBAL_DATA, 628) % 17;
		let t_143 = _to_str(GLOBAL_DATA, 629, 629 + t_142 as usize);
		let t_144 = String::from(t_143);
		let t_145 = CustomType0(t_144);
		t_1.push(t_145);
		t_1.truncate(t_0 as usize);
		// End vector declaration.
		let t_146 = &mut t_1[..];
		t_146.from_be();
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