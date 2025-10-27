//DefId(0:471 ~ secp256k1[1a2a]::schnorr::{impl#5}::schnorrsig_verify)
#[macro_use]
extern crate afl;

use secp256k1::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};
use secp256k1_sys::*;


fn main (){
	fuzz_nohook!(|data: &[u8]| {
		if data.len() < 266 {return;}
		set_global_data(data);
		let global_data = get_global_data();
		let GLOBAL_DATA = global_data.first_half;
		let t_0 = secp256k1::Secp256k1::new();
		let t_1 = &t_0;
		let t_2 = secp256k1::Secp256k1::new();
		let t_3 = &t_2;
		let mut t_4 = _to_u8(GLOBAL_DATA, 0) % 33;
		// Start vector declaration.
		let mut t_5 = std::vec::Vec::with_capacity(32);
		let t_6 = _to_u8(GLOBAL_DATA, 1);
		t_5.push(t_6);
		let t_7 = _to_u8(GLOBAL_DATA, 2);
		t_5.push(t_7);
		let t_8 = _to_u8(GLOBAL_DATA, 3);
		t_5.push(t_8);
		let t_9 = _to_u8(GLOBAL_DATA, 4);
		t_5.push(t_9);
		let t_10 = _to_u8(GLOBAL_DATA, 5);
		t_5.push(t_10);
		let t_11 = _to_u8(GLOBAL_DATA, 6);
		t_5.push(t_11);
		let t_12 = _to_u8(GLOBAL_DATA, 7);
		t_5.push(t_12);
		let t_13 = _to_u8(GLOBAL_DATA, 8);
		t_5.push(t_13);
		let t_14 = _to_u8(GLOBAL_DATA, 9);
		t_5.push(t_14);
		let t_15 = _to_u8(GLOBAL_DATA, 10);
		t_5.push(t_15);
		let t_16 = _to_u8(GLOBAL_DATA, 11);
		t_5.push(t_16);
		let t_17 = _to_u8(GLOBAL_DATA, 12);
		t_5.push(t_17);
		let t_18 = _to_u8(GLOBAL_DATA, 13);
		t_5.push(t_18);
		let t_19 = _to_u8(GLOBAL_DATA, 14);
		t_5.push(t_19);
		let t_20 = _to_u8(GLOBAL_DATA, 15);
		t_5.push(t_20);
		let t_21 = _to_u8(GLOBAL_DATA, 16);
		t_5.push(t_21);
		let t_22 = _to_u8(GLOBAL_DATA, 17);
		t_5.push(t_22);
		let t_23 = _to_u8(GLOBAL_DATA, 18);
		t_5.push(t_23);
		let t_24 = _to_u8(GLOBAL_DATA, 19);
		t_5.push(t_24);
		let t_25 = _to_u8(GLOBAL_DATA, 20);
		t_5.push(t_25);
		let t_26 = _to_u8(GLOBAL_DATA, 21);
		t_5.push(t_26);
		let t_27 = _to_u8(GLOBAL_DATA, 22);
		t_5.push(t_27);
		let t_28 = _to_u8(GLOBAL_DATA, 23);
		t_5.push(t_28);
		let t_29 = _to_u8(GLOBAL_DATA, 24);
		t_5.push(t_29);
		let t_30 = _to_u8(GLOBAL_DATA, 25);
		t_5.push(t_30);
		let t_31 = _to_u8(GLOBAL_DATA, 26);
		t_5.push(t_31);
		let t_32 = _to_u8(GLOBAL_DATA, 27);
		t_5.push(t_32);
		let t_33 = _to_u8(GLOBAL_DATA, 28);
		t_5.push(t_33);
		let t_34 = _to_u8(GLOBAL_DATA, 29);
		t_5.push(t_34);
		let t_35 = _to_u8(GLOBAL_DATA, 30);
		t_5.push(t_35);
		let t_36 = _to_u8(GLOBAL_DATA, 31);
		t_5.push(t_36);
		let t_37 = _to_u8(GLOBAL_DATA, 32);
		t_5.push(t_37);
		t_5.truncate(t_4 as usize);
		// End vector declaration.
		let t_38 = &t_5[..];
		let t_39 = secp256k1::Message::from_slice(t_38);
		let t_40 = _unwrap_result(t_39);
		let t_41 = &t_40;
		let mut t_42 = _to_u8(GLOBAL_DATA, 33) % 33;
		// Start vector declaration.
		let mut t_43 = std::vec::Vec::with_capacity(32);
		let t_44 = secp256k1_sys::types::AlignedType::zeroed();
		t_43.push(t_44);
		let t_45 = secp256k1_sys::types::AlignedType::zeroed();
		t_43.push(t_45);
		let t_46 = secp256k1_sys::types::AlignedType::zeroed();
		t_43.push(t_46);
		let t_47 = secp256k1_sys::types::AlignedType::zeroed();
		t_43.push(t_47);
		let t_48 = secp256k1_sys::types::AlignedType::zeroed();
		t_43.push(t_48);
		let t_49 = secp256k1_sys::types::AlignedType::zeroed();
		t_43.push(t_49);
		let t_50 = secp256k1_sys::types::AlignedType::zeroed();
		t_43.push(t_50);
		let t_51 = secp256k1_sys::types::AlignedType::zeroed();
		t_43.push(t_51);
		let t_52 = secp256k1_sys::types::AlignedType::zeroed();
		t_43.push(t_52);
		let t_53 = secp256k1_sys::types::AlignedType::zeroed();
		t_43.push(t_53);
		let t_54 = secp256k1_sys::types::AlignedType::zeroed();
		t_43.push(t_54);
		let t_55 = secp256k1_sys::types::AlignedType::zeroed();
		t_43.push(t_55);
		let t_56 = secp256k1_sys::types::AlignedType::zeroed();
		t_43.push(t_56);
		let t_57 = secp256k1_sys::types::AlignedType::zeroed();
		t_43.push(t_57);
		let t_58 = secp256k1_sys::types::AlignedType::zeroed();
		t_43.push(t_58);
		let t_59 = secp256k1_sys::types::AlignedType::zeroed();
		t_43.push(t_59);
		let t_60 = secp256k1_sys::types::AlignedType::zeroed();
		t_43.push(t_60);
		let t_61 = secp256k1_sys::types::AlignedType::zeroed();
		t_43.push(t_61);
		let t_62 = secp256k1_sys::types::AlignedType::zeroed();
		t_43.push(t_62);
		let t_63 = secp256k1_sys::types::AlignedType::zeroed();
		t_43.push(t_63);
		let t_64 = secp256k1_sys::types::AlignedType::zeroed();
		t_43.push(t_64);
		let t_65 = secp256k1_sys::types::AlignedType::zeroed();
		t_43.push(t_65);
		let t_66 = secp256k1_sys::types::AlignedType::zeroed();
		t_43.push(t_66);
		let t_67 = secp256k1_sys::types::AlignedType::zeroed();
		t_43.push(t_67);
		let t_68 = secp256k1_sys::types::AlignedType::zeroed();
		t_43.push(t_68);
		let t_69 = secp256k1_sys::types::AlignedType::zeroed();
		t_43.push(t_69);
		let t_70 = secp256k1_sys::types::AlignedType::zeroed();
		t_43.push(t_70);
		let t_71 = secp256k1_sys::types::AlignedType::zeroed();
		t_43.push(t_71);
		let t_72 = secp256k1_sys::types::AlignedType::zeroed();
		t_43.push(t_72);
		let t_73 = secp256k1_sys::types::AlignedType::zeroed();
		t_43.push(t_73);
		let t_74 = secp256k1_sys::types::AlignedType::zeroed();
		t_43.push(t_74);
		let t_75 = secp256k1_sys::types::AlignedType::zeroed();
		t_43.push(t_75);
		t_43.truncate(t_42 as usize);
		// End vector declaration.
		let t_76 = &mut t_43[..];
		let t_77 = secp256k1::Secp256k1::preallocated_signing_only(t_76);
		let t_78 = _unwrap_result(t_77);
		let t_79 = &t_78;
		let mut t_80 = _to_u8(GLOBAL_DATA, 34) % 33;
		// Start vector declaration.
		let mut t_81 = std::vec::Vec::with_capacity(32);
		let t_82 = _to_u8(GLOBAL_DATA, 35);
		t_81.push(t_82);
		let t_83 = _to_u8(GLOBAL_DATA, 36);
		t_81.push(t_83);
		let t_84 = _to_u8(GLOBAL_DATA, 37);
		t_81.push(t_84);
		let t_85 = _to_u8(GLOBAL_DATA, 38);
		t_81.push(t_85);
		let t_86 = _to_u8(GLOBAL_DATA, 39);
		t_81.push(t_86);
		let t_87 = _to_u8(GLOBAL_DATA, 40);
		t_81.push(t_87);
		let t_88 = _to_u8(GLOBAL_DATA, 41);
		t_81.push(t_88);
		let t_89 = _to_u8(GLOBAL_DATA, 42);
		t_81.push(t_89);
		let t_90 = _to_u8(GLOBAL_DATA, 43);
		t_81.push(t_90);
		let t_91 = _to_u8(GLOBAL_DATA, 44);
		t_81.push(t_91);
		let t_92 = _to_u8(GLOBAL_DATA, 45);
		t_81.push(t_92);
		let t_93 = _to_u8(GLOBAL_DATA, 46);
		t_81.push(t_93);
		let t_94 = _to_u8(GLOBAL_DATA, 47);
		t_81.push(t_94);
		let t_95 = _to_u8(GLOBAL_DATA, 48);
		t_81.push(t_95);
		let t_96 = _to_u8(GLOBAL_DATA, 49);
		t_81.push(t_96);
		let t_97 = _to_u8(GLOBAL_DATA, 50);
		t_81.push(t_97);
		let t_98 = _to_u8(GLOBAL_DATA, 51);
		t_81.push(t_98);
		let t_99 = _to_u8(GLOBAL_DATA, 52);
		t_81.push(t_99);
		let t_100 = _to_u8(GLOBAL_DATA, 53);
		t_81.push(t_100);
		let t_101 = _to_u8(GLOBAL_DATA, 54);
		t_81.push(t_101);
		let t_102 = _to_u8(GLOBAL_DATA, 55);
		t_81.push(t_102);
		let t_103 = _to_u8(GLOBAL_DATA, 56);
		t_81.push(t_103);
		let t_104 = _to_u8(GLOBAL_DATA, 57);
		t_81.push(t_104);
		let t_105 = _to_u8(GLOBAL_DATA, 58);
		t_81.push(t_105);
		let t_106 = _to_u8(GLOBAL_DATA, 59);
		t_81.push(t_106);
		let t_107 = _to_u8(GLOBAL_DATA, 60);
		t_81.push(t_107);
		let t_108 = _to_u8(GLOBAL_DATA, 61);
		t_81.push(t_108);
		let t_109 = _to_u8(GLOBAL_DATA, 62);
		t_81.push(t_109);
		let t_110 = _to_u8(GLOBAL_DATA, 63);
		t_81.push(t_110);
		let t_111 = _to_u8(GLOBAL_DATA, 64);
		t_81.push(t_111);
		let t_112 = _to_u8(GLOBAL_DATA, 65);
		t_81.push(t_112);
		let t_113 = _to_u8(GLOBAL_DATA, 66);
		t_81.push(t_113);
		t_81.truncate(t_80 as usize);
		// End vector declaration.
		let t_114 = &t_81[..];
		let t_115 = secp256k1::SecretKey::from_slice(t_114);
		let t_116 = _unwrap_result(t_115);
		let t_117 = secp256k1::KeyPair::from_secret_key(t_79, t_116);
		let t_118 = &t_117;
		let t_119 = secp256k1::Secp256k1::sign_schnorr_no_aux_rand(t_3, t_41, t_118);
		let t_120 = &t_119;
		let mut t_121 = _to_u8(GLOBAL_DATA, 67) % 33;
		// Start vector declaration.
		let mut t_122 = std::vec::Vec::with_capacity(32);
		let t_123 = _to_u8(GLOBAL_DATA, 68);
		t_122.push(t_123);
		let t_124 = _to_u8(GLOBAL_DATA, 69);
		t_122.push(t_124);
		let t_125 = _to_u8(GLOBAL_DATA, 70);
		t_122.push(t_125);
		let t_126 = _to_u8(GLOBAL_DATA, 71);
		t_122.push(t_126);
		let t_127 = _to_u8(GLOBAL_DATA, 72);
		t_122.push(t_127);
		let t_128 = _to_u8(GLOBAL_DATA, 73);
		t_122.push(t_128);
		let t_129 = _to_u8(GLOBAL_DATA, 74);
		t_122.push(t_129);
		let t_130 = _to_u8(GLOBAL_DATA, 75);
		t_122.push(t_130);
		let t_131 = _to_u8(GLOBAL_DATA, 76);
		t_122.push(t_131);
		let t_132 = _to_u8(GLOBAL_DATA, 77);
		t_122.push(t_132);
		let t_133 = _to_u8(GLOBAL_DATA, 78);
		t_122.push(t_133);
		let t_134 = _to_u8(GLOBAL_DATA, 79);
		t_122.push(t_134);
		let t_135 = _to_u8(GLOBAL_DATA, 80);
		t_122.push(t_135);
		let t_136 = _to_u8(GLOBAL_DATA, 81);
		t_122.push(t_136);
		let t_137 = _to_u8(GLOBAL_DATA, 82);
		t_122.push(t_137);
		let t_138 = _to_u8(GLOBAL_DATA, 83);
		t_122.push(t_138);
		let t_139 = _to_u8(GLOBAL_DATA, 84);
		t_122.push(t_139);
		let t_140 = _to_u8(GLOBAL_DATA, 85);
		t_122.push(t_140);
		let t_141 = _to_u8(GLOBAL_DATA, 86);
		t_122.push(t_141);
		let t_142 = _to_u8(GLOBAL_DATA, 87);
		t_122.push(t_142);
		let t_143 = _to_u8(GLOBAL_DATA, 88);
		t_122.push(t_143);
		let t_144 = _to_u8(GLOBAL_DATA, 89);
		t_122.push(t_144);
		let t_145 = _to_u8(GLOBAL_DATA, 90);
		t_122.push(t_145);
		let t_146 = _to_u8(GLOBAL_DATA, 91);
		t_122.push(t_146);
		let t_147 = _to_u8(GLOBAL_DATA, 92);
		t_122.push(t_147);
		let t_148 = _to_u8(GLOBAL_DATA, 93);
		t_122.push(t_148);
		let t_149 = _to_u8(GLOBAL_DATA, 94);
		t_122.push(t_149);
		let t_150 = _to_u8(GLOBAL_DATA, 95);
		t_122.push(t_150);
		let t_151 = _to_u8(GLOBAL_DATA, 96);
		t_122.push(t_151);
		let t_152 = _to_u8(GLOBAL_DATA, 97);
		t_122.push(t_152);
		let t_153 = _to_u8(GLOBAL_DATA, 98);
		t_122.push(t_153);
		let t_154 = _to_u8(GLOBAL_DATA, 99);
		t_122.push(t_154);
		t_122.truncate(t_121 as usize);
		// End vector declaration.
		let t_155 = &t_122[..];
		let t_156 = secp256k1::Message::from_slice(t_155);
		let t_157 = _unwrap_result(t_156);
		let t_158 = &t_157;
		let t_159 = secp256k1::Secp256k1::new();
		let t_160 = &t_159;
		let mut t_161 = _to_u8(GLOBAL_DATA, 100) % 33;
		// Start vector declaration.
		let mut t_162 = std::vec::Vec::with_capacity(32);
		let t_163 = _to_u8(GLOBAL_DATA, 101);
		t_162.push(t_163);
		let t_164 = _to_u8(GLOBAL_DATA, 102);
		t_162.push(t_164);
		let t_165 = _to_u8(GLOBAL_DATA, 103);
		t_162.push(t_165);
		let t_166 = _to_u8(GLOBAL_DATA, 104);
		t_162.push(t_166);
		let t_167 = _to_u8(GLOBAL_DATA, 105);
		t_162.push(t_167);
		let t_168 = _to_u8(GLOBAL_DATA, 106);
		t_162.push(t_168);
		let t_169 = _to_u8(GLOBAL_DATA, 107);
		t_162.push(t_169);
		let t_170 = _to_u8(GLOBAL_DATA, 108);
		t_162.push(t_170);
		let t_171 = _to_u8(GLOBAL_DATA, 109);
		t_162.push(t_171);
		let t_172 = _to_u8(GLOBAL_DATA, 110);
		t_162.push(t_172);
		let t_173 = _to_u8(GLOBAL_DATA, 111);
		t_162.push(t_173);
		let t_174 = _to_u8(GLOBAL_DATA, 112);
		t_162.push(t_174);
		let t_175 = _to_u8(GLOBAL_DATA, 113);
		t_162.push(t_175);
		let t_176 = _to_u8(GLOBAL_DATA, 114);
		t_162.push(t_176);
		let t_177 = _to_u8(GLOBAL_DATA, 115);
		t_162.push(t_177);
		let t_178 = _to_u8(GLOBAL_DATA, 116);
		t_162.push(t_178);
		let t_179 = _to_u8(GLOBAL_DATA, 117);
		t_162.push(t_179);
		let t_180 = _to_u8(GLOBAL_DATA, 118);
		t_162.push(t_180);
		let t_181 = _to_u8(GLOBAL_DATA, 119);
		t_162.push(t_181);
		let t_182 = _to_u8(GLOBAL_DATA, 120);
		t_162.push(t_182);
		let t_183 = _to_u8(GLOBAL_DATA, 121);
		t_162.push(t_183);
		let t_184 = _to_u8(GLOBAL_DATA, 122);
		t_162.push(t_184);
		let t_185 = _to_u8(GLOBAL_DATA, 123);
		t_162.push(t_185);
		let t_186 = _to_u8(GLOBAL_DATA, 124);
		t_162.push(t_186);
		let t_187 = _to_u8(GLOBAL_DATA, 125);
		t_162.push(t_187);
		let t_188 = _to_u8(GLOBAL_DATA, 126);
		t_162.push(t_188);
		let t_189 = _to_u8(GLOBAL_DATA, 127);
		t_162.push(t_189);
		let t_190 = _to_u8(GLOBAL_DATA, 128);
		t_162.push(t_190);
		let t_191 = _to_u8(GLOBAL_DATA, 129);
		t_162.push(t_191);
		let t_192 = _to_u8(GLOBAL_DATA, 130);
		t_162.push(t_192);
		let t_193 = _to_u8(GLOBAL_DATA, 131);
		t_162.push(t_193);
		let t_194 = _to_u8(GLOBAL_DATA, 132);
		t_162.push(t_194);
		t_162.truncate(t_161 as usize);
		// End vector declaration.
		let t_195 = &t_162[..];
		let t_196 = secp256k1::SecretKey::from_slice(t_195);
		let t_197 = _unwrap_result(t_196);
		let t_198 = secp256k1::KeyPair::from_secret_key(t_160, t_197);
		let t_199 = secp256k1::PublicKey::from(t_198);
		let t_200 = secp256k1::XOnlyPublicKey::from(t_199);
		let t_201 = &t_200;
		t_1.schnorrsig_verify(t_120, t_158, t_201);
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