//DefId(0:418 ~ secp256k1[1a2a]::ecdsa::{impl#15}::verify)
#[macro_use]
extern crate afl;

use secp256k1::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};
use secp256k1_sys::*;

struct CustomType0(String);

impl secp256k1::ThirtyTwoByteHash for CustomType0 {
	
	fn into_32(self) -> [u8; 32] {
		let global_data = get_global_data();
		let GLOBAL_DATA = global_data.first_half;
		let custom_impl_num = _to_usize(GLOBAL_DATA, 33);
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
		let mut t_42 = std::vec::Vec::with_capacity(32);
		let t_43 = _to_u8(GLOBAL_DATA, 41);
		t_42.push(t_43);
		let t_44 = _to_u8(GLOBAL_DATA, 42);
		t_42.push(t_44);
		let t_45 = _to_u8(GLOBAL_DATA, 43);
		t_42.push(t_45);
		let t_46 = _to_u8(GLOBAL_DATA, 44);
		t_42.push(t_46);
		let t_47 = _to_u8(GLOBAL_DATA, 45);
		t_42.push(t_47);
		let t_48 = _to_u8(GLOBAL_DATA, 46);
		t_42.push(t_48);
		let t_49 = _to_u8(GLOBAL_DATA, 47);
		t_42.push(t_49);
		let t_50 = _to_u8(GLOBAL_DATA, 48);
		t_42.push(t_50);
		let t_51 = _to_u8(GLOBAL_DATA, 49);
		t_42.push(t_51);
		let t_52 = _to_u8(GLOBAL_DATA, 50);
		t_42.push(t_52);
		let t_53 = _to_u8(GLOBAL_DATA, 51);
		t_42.push(t_53);
		let t_54 = _to_u8(GLOBAL_DATA, 52);
		t_42.push(t_54);
		let t_55 = _to_u8(GLOBAL_DATA, 53);
		t_42.push(t_55);
		let t_56 = _to_u8(GLOBAL_DATA, 54);
		t_42.push(t_56);
		let t_57 = _to_u8(GLOBAL_DATA, 55);
		t_42.push(t_57);
		let t_58 = _to_u8(GLOBAL_DATA, 56);
		t_42.push(t_58);
		let t_59 = _to_u8(GLOBAL_DATA, 57);
		t_42.push(t_59);
		let t_60 = _to_u8(GLOBAL_DATA, 58);
		t_42.push(t_60);
		let t_61 = _to_u8(GLOBAL_DATA, 59);
		t_42.push(t_61);
		let t_62 = _to_u8(GLOBAL_DATA, 60);
		t_42.push(t_62);
		let t_63 = _to_u8(GLOBAL_DATA, 61);
		t_42.push(t_63);
		let t_64 = _to_u8(GLOBAL_DATA, 62);
		t_42.push(t_64);
		let t_65 = _to_u8(GLOBAL_DATA, 63);
		t_42.push(t_65);
		let t_66 = _to_u8(GLOBAL_DATA, 64);
		t_42.push(t_66);
		let t_67 = _to_u8(GLOBAL_DATA, 65);
		t_42.push(t_67);
		let t_68 = _to_u8(GLOBAL_DATA, 66);
		t_42.push(t_68);
		let t_69 = _to_u8(GLOBAL_DATA, 67);
		t_42.push(t_69);
		let t_70 = _to_u8(GLOBAL_DATA, 68);
		t_42.push(t_70);
		let t_71 = _to_u8(GLOBAL_DATA, 69);
		t_42.push(t_71);
		let t_72 = _to_u8(GLOBAL_DATA, 70);
		t_42.push(t_72);
		let t_73 = _to_u8(GLOBAL_DATA, 71);
		t_42.push(t_73);
		let t_74 = _to_u8(GLOBAL_DATA, 72);
		t_42.push(t_74);
		t_42.truncate(32);
		// End vector declaration.
		let t_75: [_; 32] = t_42.try_into().unwrap();
		return t_75;
	}
}

fn main (){
	fuzz_nohook!(|data: &[u8]| {
		if data.len() < 298 {return;}
		set_global_data(data);
		let global_data = get_global_data();
		let GLOBAL_DATA = global_data.first_half;
		let t_0 = secp256k1::Secp256k1::new();
		let t_1 = &t_0;
		let mut t_2 = _to_u8(GLOBAL_DATA, 0) % 33;
		// Start vector declaration.
		let mut t_3 = std::vec::Vec::with_capacity(32);
		let t_4 = _to_u8(GLOBAL_DATA, 1);
		t_3.push(t_4);
		let t_5 = _to_u8(GLOBAL_DATA, 2);
		t_3.push(t_5);
		let t_6 = _to_u8(GLOBAL_DATA, 3);
		t_3.push(t_6);
		let t_7 = _to_u8(GLOBAL_DATA, 4);
		t_3.push(t_7);
		let t_8 = _to_u8(GLOBAL_DATA, 5);
		t_3.push(t_8);
		let t_9 = _to_u8(GLOBAL_DATA, 6);
		t_3.push(t_9);
		let t_10 = _to_u8(GLOBAL_DATA, 7);
		t_3.push(t_10);
		let t_11 = _to_u8(GLOBAL_DATA, 8);
		t_3.push(t_11);
		let t_12 = _to_u8(GLOBAL_DATA, 9);
		t_3.push(t_12);
		let t_13 = _to_u8(GLOBAL_DATA, 10);
		t_3.push(t_13);
		let t_14 = _to_u8(GLOBAL_DATA, 11);
		t_3.push(t_14);
		let t_15 = _to_u8(GLOBAL_DATA, 12);
		t_3.push(t_15);
		let t_16 = _to_u8(GLOBAL_DATA, 13);
		t_3.push(t_16);
		let t_17 = _to_u8(GLOBAL_DATA, 14);
		t_3.push(t_17);
		let t_18 = _to_u8(GLOBAL_DATA, 15);
		t_3.push(t_18);
		let t_19 = _to_u8(GLOBAL_DATA, 16);
		t_3.push(t_19);
		let t_20 = _to_u8(GLOBAL_DATA, 17);
		t_3.push(t_20);
		let t_21 = _to_u8(GLOBAL_DATA, 18);
		t_3.push(t_21);
		let t_22 = _to_u8(GLOBAL_DATA, 19);
		t_3.push(t_22);
		let t_23 = _to_u8(GLOBAL_DATA, 20);
		t_3.push(t_23);
		let t_24 = _to_u8(GLOBAL_DATA, 21);
		t_3.push(t_24);
		let t_25 = _to_u8(GLOBAL_DATA, 22);
		t_3.push(t_25);
		let t_26 = _to_u8(GLOBAL_DATA, 23);
		t_3.push(t_26);
		let t_27 = _to_u8(GLOBAL_DATA, 24);
		t_3.push(t_27);
		let t_28 = _to_u8(GLOBAL_DATA, 25);
		t_3.push(t_28);
		let t_29 = _to_u8(GLOBAL_DATA, 26);
		t_3.push(t_29);
		let t_30 = _to_u8(GLOBAL_DATA, 27);
		t_3.push(t_30);
		let t_31 = _to_u8(GLOBAL_DATA, 28);
		t_3.push(t_31);
		let t_32 = _to_u8(GLOBAL_DATA, 29);
		t_3.push(t_32);
		let t_33 = _to_u8(GLOBAL_DATA, 30);
		t_3.push(t_33);
		let t_34 = _to_u8(GLOBAL_DATA, 31);
		t_3.push(t_34);
		let t_35 = _to_u8(GLOBAL_DATA, 32);
		t_3.push(t_35);
		t_3.truncate(t_2 as usize);
		// End vector declaration.
		let t_36 = &t_3[..];
		let t_37 = secp256k1::Message::from_slice(t_36);
		let t_38 = _unwrap_result(t_37);
		let t_39 = &t_38;
		let t_40 = secp256k1::Secp256k1::new();
		let t_41 = &t_40;
		let mut t_76 = _to_u8(GLOBAL_DATA, 73) % 17;
		let t_77 = _to_str(GLOBAL_DATA, 74, 74 + t_76 as usize);
		let t_78 = String::from(t_77);
		let t_79 = CustomType0(t_78);
		let t_80 = secp256k1::Message::from(t_79);
		let t_81 = &t_80;
		let t_82 = secp256k1::Secp256k1::new();
		let t_83 = &t_82;
		let mut t_84 = _to_u8(GLOBAL_DATA, 90) % 33;
		// Start vector declaration.
		let mut t_85 = std::vec::Vec::with_capacity(32);
		let t_86 = _to_u8(GLOBAL_DATA, 91);
		t_85.push(t_86);
		let t_87 = _to_u8(GLOBAL_DATA, 92);
		t_85.push(t_87);
		let t_88 = _to_u8(GLOBAL_DATA, 93);
		t_85.push(t_88);
		let t_89 = _to_u8(GLOBAL_DATA, 94);
		t_85.push(t_89);
		let t_90 = _to_u8(GLOBAL_DATA, 95);
		t_85.push(t_90);
		let t_91 = _to_u8(GLOBAL_DATA, 96);
		t_85.push(t_91);
		let t_92 = _to_u8(GLOBAL_DATA, 97);
		t_85.push(t_92);
		let t_93 = _to_u8(GLOBAL_DATA, 98);
		t_85.push(t_93);
		let t_94 = _to_u8(GLOBAL_DATA, 99);
		t_85.push(t_94);
		let t_95 = _to_u8(GLOBAL_DATA, 100);
		t_85.push(t_95);
		let t_96 = _to_u8(GLOBAL_DATA, 101);
		t_85.push(t_96);
		let t_97 = _to_u8(GLOBAL_DATA, 102);
		t_85.push(t_97);
		let t_98 = _to_u8(GLOBAL_DATA, 103);
		t_85.push(t_98);
		let t_99 = _to_u8(GLOBAL_DATA, 104);
		t_85.push(t_99);
		let t_100 = _to_u8(GLOBAL_DATA, 105);
		t_85.push(t_100);
		let t_101 = _to_u8(GLOBAL_DATA, 106);
		t_85.push(t_101);
		let t_102 = _to_u8(GLOBAL_DATA, 107);
		t_85.push(t_102);
		let t_103 = _to_u8(GLOBAL_DATA, 108);
		t_85.push(t_103);
		let t_104 = _to_u8(GLOBAL_DATA, 109);
		t_85.push(t_104);
		let t_105 = _to_u8(GLOBAL_DATA, 110);
		t_85.push(t_105);
		let t_106 = _to_u8(GLOBAL_DATA, 111);
		t_85.push(t_106);
		let t_107 = _to_u8(GLOBAL_DATA, 112);
		t_85.push(t_107);
		let t_108 = _to_u8(GLOBAL_DATA, 113);
		t_85.push(t_108);
		let t_109 = _to_u8(GLOBAL_DATA, 114);
		t_85.push(t_109);
		let t_110 = _to_u8(GLOBAL_DATA, 115);
		t_85.push(t_110);
		let t_111 = _to_u8(GLOBAL_DATA, 116);
		t_85.push(t_111);
		let t_112 = _to_u8(GLOBAL_DATA, 117);
		t_85.push(t_112);
		let t_113 = _to_u8(GLOBAL_DATA, 118);
		t_85.push(t_113);
		let t_114 = _to_u8(GLOBAL_DATA, 119);
		t_85.push(t_114);
		let t_115 = _to_u8(GLOBAL_DATA, 120);
		t_85.push(t_115);
		let t_116 = _to_u8(GLOBAL_DATA, 121);
		t_85.push(t_116);
		let t_117 = _to_u8(GLOBAL_DATA, 122);
		t_85.push(t_117);
		t_85.truncate(t_84 as usize);
		// End vector declaration.
		let t_118 = &t_85[..];
		let t_119 = secp256k1::KeyPair::from_seckey_slice(t_83, t_118);
		let t_120 = _unwrap_result(t_119);
		let t_121 = &t_120;
		let t_122 = secp256k1::SecretKey::from(t_121);
		let t_123 = &t_122;
		let t_124 = _to_usize(GLOBAL_DATA, 123);
		let t_125 = secp256k1::Secp256k1::sign_grind_r(t_41, t_81, t_123, t_124);
		let t_126 = &t_125;
		let mut t_127 = _to_u8(GLOBAL_DATA, 131) % 33;
		// Start vector declaration.
		let mut t_128 = std::vec::Vec::with_capacity(32);
		let t_129 = secp256k1_sys::types::AlignedType::zeroed();
		t_128.push(t_129);
		let t_130 = secp256k1_sys::types::AlignedType::zeroed();
		t_128.push(t_130);
		let t_131 = secp256k1_sys::types::AlignedType::zeroed();
		t_128.push(t_131);
		let t_132 = secp256k1_sys::types::AlignedType::zeroed();
		t_128.push(t_132);
		let t_133 = secp256k1_sys::types::AlignedType::zeroed();
		t_128.push(t_133);
		let t_134 = secp256k1_sys::types::AlignedType::zeroed();
		t_128.push(t_134);
		let t_135 = secp256k1_sys::types::AlignedType::zeroed();
		t_128.push(t_135);
		let t_136 = secp256k1_sys::types::AlignedType::zeroed();
		t_128.push(t_136);
		let t_137 = secp256k1_sys::types::AlignedType::zeroed();
		t_128.push(t_137);
		let t_138 = secp256k1_sys::types::AlignedType::zeroed();
		t_128.push(t_138);
		let t_139 = secp256k1_sys::types::AlignedType::zeroed();
		t_128.push(t_139);
		let t_140 = secp256k1_sys::types::AlignedType::zeroed();
		t_128.push(t_140);
		let t_141 = secp256k1_sys::types::AlignedType::zeroed();
		t_128.push(t_141);
		let t_142 = secp256k1_sys::types::AlignedType::zeroed();
		t_128.push(t_142);
		let t_143 = secp256k1_sys::types::AlignedType::zeroed();
		t_128.push(t_143);
		let t_144 = secp256k1_sys::types::AlignedType::zeroed();
		t_128.push(t_144);
		let t_145 = secp256k1_sys::types::AlignedType::zeroed();
		t_128.push(t_145);
		let t_146 = secp256k1_sys::types::AlignedType::zeroed();
		t_128.push(t_146);
		let t_147 = secp256k1_sys::types::AlignedType::zeroed();
		t_128.push(t_147);
		let t_148 = secp256k1_sys::types::AlignedType::zeroed();
		t_128.push(t_148);
		let t_149 = secp256k1_sys::types::AlignedType::zeroed();
		t_128.push(t_149);
		let t_150 = secp256k1_sys::types::AlignedType::zeroed();
		t_128.push(t_150);
		let t_151 = secp256k1_sys::types::AlignedType::zeroed();
		t_128.push(t_151);
		let t_152 = secp256k1_sys::types::AlignedType::zeroed();
		t_128.push(t_152);
		let t_153 = secp256k1_sys::types::AlignedType::zeroed();
		t_128.push(t_153);
		let t_154 = secp256k1_sys::types::AlignedType::zeroed();
		t_128.push(t_154);
		let t_155 = secp256k1_sys::types::AlignedType::zeroed();
		t_128.push(t_155);
		let t_156 = secp256k1_sys::types::AlignedType::zeroed();
		t_128.push(t_156);
		let t_157 = secp256k1_sys::types::AlignedType::zeroed();
		t_128.push(t_157);
		let t_158 = secp256k1_sys::types::AlignedType::zeroed();
		t_128.push(t_158);
		let t_159 = secp256k1_sys::types::AlignedType::zeroed();
		t_128.push(t_159);
		let t_160 = secp256k1_sys::types::AlignedType::zeroed();
		t_128.push(t_160);
		t_128.truncate(t_127 as usize);
		// End vector declaration.
		let t_161 = &mut t_128[..];
		let t_162 = secp256k1::Secp256k1::preallocated_signing_only(t_161);
		let t_163 = _unwrap_result(t_162);
		let t_164 = &t_163;
		let mut t_165 = _to_u8(GLOBAL_DATA, 132) % 17;
		let t_166 = _to_str(GLOBAL_DATA, 133, 133 + t_165 as usize);
		let t_167 = secp256k1::KeyPair::from_seckey_str(t_164, t_166);
		let t_168 = _unwrap_result(t_167);
		let t_169 = &t_168;
		let t_170 = secp256k1::PublicKey::from_keypair(t_169);
		let t_171 = &t_170;
		t_1.verify(t_39, t_126, t_171);
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