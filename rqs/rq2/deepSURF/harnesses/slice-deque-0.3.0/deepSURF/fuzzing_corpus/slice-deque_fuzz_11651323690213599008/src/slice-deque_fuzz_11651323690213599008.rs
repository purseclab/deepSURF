//DefId(0:263 ~ slice_deque[05d1]::{impl#18}::partial_cmp)
#[macro_use]
extern crate afl;

use slice_deque::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

struct CustomType0(String);

impl std::clone::Clone for CustomType0 {
	
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
		let mut t_2 = _to_u8(GLOBAL_DATA, 9) % 17;
		let t_3 = _to_str(GLOBAL_DATA, 10, 10 + t_2 as usize);
		let t_4 = String::from(t_3);
		let t_5 = CustomType0(t_4);
		return t_5;
	}
}

impl std::cmp::PartialEq for CustomType0 {
	
	fn eq(&self, _: &Self) -> bool {
		let global_data = get_global_data();
		let GLOBAL_DATA = global_data.first_half;
		let custom_impl_num = _to_usize(GLOBAL_DATA, 571);
		let custom_impl_inst_num = self.0.len();
		let selector = (custom_impl_num + custom_impl_inst_num) % 3;
		if selector == 0{
			panic!("INTENTIONAL PANIC!");
		}
		let GLOBAL_DATA = match selector{
				1 => global_data.first_half,
				_ => global_data.second_half,
		};
		let t_139 = _to_bool(GLOBAL_DATA, 579);
		return t_139;
	}
}

impl std::cmp::PartialOrd for CustomType0 {
	
	fn partial_cmp(&self, _: &Self) -> Option<std::cmp::Ordering> {
		let global_data = get_global_data();
		let GLOBAL_DATA = global_data.first_half;
		let custom_impl_num = _to_usize(GLOBAL_DATA, 580);
		let custom_impl_inst_num = self.0.len();
		let selector = (custom_impl_num + custom_impl_inst_num) % 3;
		if selector == 0{
			panic!("INTENTIONAL PANIC!");
		}
		let GLOBAL_DATA = match selector{
				1 => global_data.first_half,
				_ => global_data.second_half,
		};
		let t_140 = _to_usize(GLOBAL_DATA, 588);
		let t_141 = match (t_140 % 3usize) {
			0 => {
				std::cmp::Ordering::Less
			},
			1 => {
				std::cmp::Ordering::Greater
			},
			2 => {
				std::cmp::Ordering::Equal
			},
			_ => unreachable!(),
		};
		let t_142 = Some(t_141);
		return t_142;
	}
}

fn main (){
	fuzz_nohook!(|data: &[u8]| {
		if data.len() < 2280 {return;}
		set_global_data(data);
		let global_data = get_global_data();
		let GLOBAL_DATA = global_data.first_half;
		let mut t_0 = _to_u8(GLOBAL_DATA, 0) % 33;
		// Start vector declaration.
		let mut t_1 = std::vec::Vec::with_capacity(32);
		let mut t_6 = _to_u8(GLOBAL_DATA, 26) % 17;
		let t_7 = _to_str(GLOBAL_DATA, 27, 27 + t_6 as usize);
		let t_8 = String::from(t_7);
		let t_9 = CustomType0(t_8);
		t_1.push(t_9);
		let mut t_10 = _to_u8(GLOBAL_DATA, 43) % 17;
		let t_11 = _to_str(GLOBAL_DATA, 44, 44 + t_10 as usize);
		let t_12 = String::from(t_11);
		let t_13 = CustomType0(t_12);
		t_1.push(t_13);
		let mut t_14 = _to_u8(GLOBAL_DATA, 60) % 17;
		let t_15 = _to_str(GLOBAL_DATA, 61, 61 + t_14 as usize);
		let t_16 = String::from(t_15);
		let t_17 = CustomType0(t_16);
		t_1.push(t_17);
		let mut t_18 = _to_u8(GLOBAL_DATA, 77) % 17;
		let t_19 = _to_str(GLOBAL_DATA, 78, 78 + t_18 as usize);
		let t_20 = String::from(t_19);
		let t_21 = CustomType0(t_20);
		t_1.push(t_21);
		let mut t_22 = _to_u8(GLOBAL_DATA, 94) % 17;
		let t_23 = _to_str(GLOBAL_DATA, 95, 95 + t_22 as usize);
		let t_24 = String::from(t_23);
		let t_25 = CustomType0(t_24);
		t_1.push(t_25);
		let mut t_26 = _to_u8(GLOBAL_DATA, 111) % 17;
		let t_27 = _to_str(GLOBAL_DATA, 112, 112 + t_26 as usize);
		let t_28 = String::from(t_27);
		let t_29 = CustomType0(t_28);
		t_1.push(t_29);
		let mut t_30 = _to_u8(GLOBAL_DATA, 128) % 17;
		let t_31 = _to_str(GLOBAL_DATA, 129, 129 + t_30 as usize);
		let t_32 = String::from(t_31);
		let t_33 = CustomType0(t_32);
		t_1.push(t_33);
		let mut t_34 = _to_u8(GLOBAL_DATA, 145) % 17;
		let t_35 = _to_str(GLOBAL_DATA, 146, 146 + t_34 as usize);
		let t_36 = String::from(t_35);
		let t_37 = CustomType0(t_36);
		t_1.push(t_37);
		let mut t_38 = _to_u8(GLOBAL_DATA, 162) % 17;
		let t_39 = _to_str(GLOBAL_DATA, 163, 163 + t_38 as usize);
		let t_40 = String::from(t_39);
		let t_41 = CustomType0(t_40);
		t_1.push(t_41);
		let mut t_42 = _to_u8(GLOBAL_DATA, 179) % 17;
		let t_43 = _to_str(GLOBAL_DATA, 180, 180 + t_42 as usize);
		let t_44 = String::from(t_43);
		let t_45 = CustomType0(t_44);
		t_1.push(t_45);
		let mut t_46 = _to_u8(GLOBAL_DATA, 196) % 17;
		let t_47 = _to_str(GLOBAL_DATA, 197, 197 + t_46 as usize);
		let t_48 = String::from(t_47);
		let t_49 = CustomType0(t_48);
		t_1.push(t_49);
		let mut t_50 = _to_u8(GLOBAL_DATA, 213) % 17;
		let t_51 = _to_str(GLOBAL_DATA, 214, 214 + t_50 as usize);
		let t_52 = String::from(t_51);
		let t_53 = CustomType0(t_52);
		t_1.push(t_53);
		let mut t_54 = _to_u8(GLOBAL_DATA, 230) % 17;
		let t_55 = _to_str(GLOBAL_DATA, 231, 231 + t_54 as usize);
		let t_56 = String::from(t_55);
		let t_57 = CustomType0(t_56);
		t_1.push(t_57);
		let mut t_58 = _to_u8(GLOBAL_DATA, 247) % 17;
		let t_59 = _to_str(GLOBAL_DATA, 248, 248 + t_58 as usize);
		let t_60 = String::from(t_59);
		let t_61 = CustomType0(t_60);
		t_1.push(t_61);
		let mut t_62 = _to_u8(GLOBAL_DATA, 264) % 17;
		let t_63 = _to_str(GLOBAL_DATA, 265, 265 + t_62 as usize);
		let t_64 = String::from(t_63);
		let t_65 = CustomType0(t_64);
		t_1.push(t_65);
		let mut t_66 = _to_u8(GLOBAL_DATA, 281) % 17;
		let t_67 = _to_str(GLOBAL_DATA, 282, 282 + t_66 as usize);
		let t_68 = String::from(t_67);
		let t_69 = CustomType0(t_68);
		t_1.push(t_69);
		let mut t_70 = _to_u8(GLOBAL_DATA, 298) % 17;
		let t_71 = _to_str(GLOBAL_DATA, 299, 299 + t_70 as usize);
		let t_72 = String::from(t_71);
		let t_73 = CustomType0(t_72);
		t_1.push(t_73);
		let mut t_74 = _to_u8(GLOBAL_DATA, 315) % 17;
		let t_75 = _to_str(GLOBAL_DATA, 316, 316 + t_74 as usize);
		let t_76 = String::from(t_75);
		let t_77 = CustomType0(t_76);
		t_1.push(t_77);
		let mut t_78 = _to_u8(GLOBAL_DATA, 332) % 17;
		let t_79 = _to_str(GLOBAL_DATA, 333, 333 + t_78 as usize);
		let t_80 = String::from(t_79);
		let t_81 = CustomType0(t_80);
		t_1.push(t_81);
		let mut t_82 = _to_u8(GLOBAL_DATA, 349) % 17;
		let t_83 = _to_str(GLOBAL_DATA, 350, 350 + t_82 as usize);
		let t_84 = String::from(t_83);
		let t_85 = CustomType0(t_84);
		t_1.push(t_85);
		let mut t_86 = _to_u8(GLOBAL_DATA, 366) % 17;
		let t_87 = _to_str(GLOBAL_DATA, 367, 367 + t_86 as usize);
		let t_88 = String::from(t_87);
		let t_89 = CustomType0(t_88);
		t_1.push(t_89);
		let mut t_90 = _to_u8(GLOBAL_DATA, 383) % 17;
		let t_91 = _to_str(GLOBAL_DATA, 384, 384 + t_90 as usize);
		let t_92 = String::from(t_91);
		let t_93 = CustomType0(t_92);
		t_1.push(t_93);
		let mut t_94 = _to_u8(GLOBAL_DATA, 400) % 17;
		let t_95 = _to_str(GLOBAL_DATA, 401, 401 + t_94 as usize);
		let t_96 = String::from(t_95);
		let t_97 = CustomType0(t_96);
		t_1.push(t_97);
		let mut t_98 = _to_u8(GLOBAL_DATA, 417) % 17;
		let t_99 = _to_str(GLOBAL_DATA, 418, 418 + t_98 as usize);
		let t_100 = String::from(t_99);
		let t_101 = CustomType0(t_100);
		t_1.push(t_101);
		let mut t_102 = _to_u8(GLOBAL_DATA, 434) % 17;
		let t_103 = _to_str(GLOBAL_DATA, 435, 435 + t_102 as usize);
		let t_104 = String::from(t_103);
		let t_105 = CustomType0(t_104);
		t_1.push(t_105);
		let mut t_106 = _to_u8(GLOBAL_DATA, 451) % 17;
		let t_107 = _to_str(GLOBAL_DATA, 452, 452 + t_106 as usize);
		let t_108 = String::from(t_107);
		let t_109 = CustomType0(t_108);
		t_1.push(t_109);
		let mut t_110 = _to_u8(GLOBAL_DATA, 468) % 17;
		let t_111 = _to_str(GLOBAL_DATA, 469, 469 + t_110 as usize);
		let t_112 = String::from(t_111);
		let t_113 = CustomType0(t_112);
		t_1.push(t_113);
		let mut t_114 = _to_u8(GLOBAL_DATA, 485) % 17;
		let t_115 = _to_str(GLOBAL_DATA, 486, 486 + t_114 as usize);
		let t_116 = String::from(t_115);
		let t_117 = CustomType0(t_116);
		t_1.push(t_117);
		let mut t_118 = _to_u8(GLOBAL_DATA, 502) % 17;
		let t_119 = _to_str(GLOBAL_DATA, 503, 503 + t_118 as usize);
		let t_120 = String::from(t_119);
		let t_121 = CustomType0(t_120);
		t_1.push(t_121);
		let mut t_122 = _to_u8(GLOBAL_DATA, 519) % 17;
		let t_123 = _to_str(GLOBAL_DATA, 520, 520 + t_122 as usize);
		let t_124 = String::from(t_123);
		let t_125 = CustomType0(t_124);
		t_1.push(t_125);
		let mut t_126 = _to_u8(GLOBAL_DATA, 536) % 17;
		let t_127 = _to_str(GLOBAL_DATA, 537, 537 + t_126 as usize);
		let t_128 = String::from(t_127);
		let t_129 = CustomType0(t_128);
		t_1.push(t_129);
		let mut t_130 = _to_u8(GLOBAL_DATA, 553) % 17;
		let t_131 = _to_str(GLOBAL_DATA, 554, 554 + t_130 as usize);
		let t_132 = String::from(t_131);
		let t_133 = CustomType0(t_132);
		t_1.push(t_133);
		t_1.truncate(t_0 as usize);
		// End vector declaration.
		let t_134 = &mut t_1[..];
		let t_135 = slice_deque::SliceDeque::from(t_134);
		let t_136 = &t_135;
		let mut t_137 = _to_u8(GLOBAL_DATA, 570) % 33;
		// Start vector declaration.
		let mut t_138 = std::vec::Vec::with_capacity(32);
		let mut t_143 = _to_u8(GLOBAL_DATA, 596) % 17;
		let t_144 = _to_str(GLOBAL_DATA, 597, 597 + t_143 as usize);
		let t_145 = String::from(t_144);
		let t_146 = CustomType0(t_145);
		t_138.push(t_146);
		let mut t_147 = _to_u8(GLOBAL_DATA, 613) % 17;
		let t_148 = _to_str(GLOBAL_DATA, 614, 614 + t_147 as usize);
		let t_149 = String::from(t_148);
		let t_150 = CustomType0(t_149);
		t_138.push(t_150);
		let mut t_151 = _to_u8(GLOBAL_DATA, 630) % 17;
		let t_152 = _to_str(GLOBAL_DATA, 631, 631 + t_151 as usize);
		let t_153 = String::from(t_152);
		let t_154 = CustomType0(t_153);
		t_138.push(t_154);
		let mut t_155 = _to_u8(GLOBAL_DATA, 647) % 17;
		let t_156 = _to_str(GLOBAL_DATA, 648, 648 + t_155 as usize);
		let t_157 = String::from(t_156);
		let t_158 = CustomType0(t_157);
		t_138.push(t_158);
		let mut t_159 = _to_u8(GLOBAL_DATA, 664) % 17;
		let t_160 = _to_str(GLOBAL_DATA, 665, 665 + t_159 as usize);
		let t_161 = String::from(t_160);
		let t_162 = CustomType0(t_161);
		t_138.push(t_162);
		let mut t_163 = _to_u8(GLOBAL_DATA, 681) % 17;
		let t_164 = _to_str(GLOBAL_DATA, 682, 682 + t_163 as usize);
		let t_165 = String::from(t_164);
		let t_166 = CustomType0(t_165);
		t_138.push(t_166);
		let mut t_167 = _to_u8(GLOBAL_DATA, 698) % 17;
		let t_168 = _to_str(GLOBAL_DATA, 699, 699 + t_167 as usize);
		let t_169 = String::from(t_168);
		let t_170 = CustomType0(t_169);
		t_138.push(t_170);
		let mut t_171 = _to_u8(GLOBAL_DATA, 715) % 17;
		let t_172 = _to_str(GLOBAL_DATA, 716, 716 + t_171 as usize);
		let t_173 = String::from(t_172);
		let t_174 = CustomType0(t_173);
		t_138.push(t_174);
		let mut t_175 = _to_u8(GLOBAL_DATA, 732) % 17;
		let t_176 = _to_str(GLOBAL_DATA, 733, 733 + t_175 as usize);
		let t_177 = String::from(t_176);
		let t_178 = CustomType0(t_177);
		t_138.push(t_178);
		let mut t_179 = _to_u8(GLOBAL_DATA, 749) % 17;
		let t_180 = _to_str(GLOBAL_DATA, 750, 750 + t_179 as usize);
		let t_181 = String::from(t_180);
		let t_182 = CustomType0(t_181);
		t_138.push(t_182);
		let mut t_183 = _to_u8(GLOBAL_DATA, 766) % 17;
		let t_184 = _to_str(GLOBAL_DATA, 767, 767 + t_183 as usize);
		let t_185 = String::from(t_184);
		let t_186 = CustomType0(t_185);
		t_138.push(t_186);
		let mut t_187 = _to_u8(GLOBAL_DATA, 783) % 17;
		let t_188 = _to_str(GLOBAL_DATA, 784, 784 + t_187 as usize);
		let t_189 = String::from(t_188);
		let t_190 = CustomType0(t_189);
		t_138.push(t_190);
		let mut t_191 = _to_u8(GLOBAL_DATA, 800) % 17;
		let t_192 = _to_str(GLOBAL_DATA, 801, 801 + t_191 as usize);
		let t_193 = String::from(t_192);
		let t_194 = CustomType0(t_193);
		t_138.push(t_194);
		let mut t_195 = _to_u8(GLOBAL_DATA, 817) % 17;
		let t_196 = _to_str(GLOBAL_DATA, 818, 818 + t_195 as usize);
		let t_197 = String::from(t_196);
		let t_198 = CustomType0(t_197);
		t_138.push(t_198);
		let mut t_199 = _to_u8(GLOBAL_DATA, 834) % 17;
		let t_200 = _to_str(GLOBAL_DATA, 835, 835 + t_199 as usize);
		let t_201 = String::from(t_200);
		let t_202 = CustomType0(t_201);
		t_138.push(t_202);
		let mut t_203 = _to_u8(GLOBAL_DATA, 851) % 17;
		let t_204 = _to_str(GLOBAL_DATA, 852, 852 + t_203 as usize);
		let t_205 = String::from(t_204);
		let t_206 = CustomType0(t_205);
		t_138.push(t_206);
		let mut t_207 = _to_u8(GLOBAL_DATA, 868) % 17;
		let t_208 = _to_str(GLOBAL_DATA, 869, 869 + t_207 as usize);
		let t_209 = String::from(t_208);
		let t_210 = CustomType0(t_209);
		t_138.push(t_210);
		let mut t_211 = _to_u8(GLOBAL_DATA, 885) % 17;
		let t_212 = _to_str(GLOBAL_DATA, 886, 886 + t_211 as usize);
		let t_213 = String::from(t_212);
		let t_214 = CustomType0(t_213);
		t_138.push(t_214);
		let mut t_215 = _to_u8(GLOBAL_DATA, 902) % 17;
		let t_216 = _to_str(GLOBAL_DATA, 903, 903 + t_215 as usize);
		let t_217 = String::from(t_216);
		let t_218 = CustomType0(t_217);
		t_138.push(t_218);
		let mut t_219 = _to_u8(GLOBAL_DATA, 919) % 17;
		let t_220 = _to_str(GLOBAL_DATA, 920, 920 + t_219 as usize);
		let t_221 = String::from(t_220);
		let t_222 = CustomType0(t_221);
		t_138.push(t_222);
		let mut t_223 = _to_u8(GLOBAL_DATA, 936) % 17;
		let t_224 = _to_str(GLOBAL_DATA, 937, 937 + t_223 as usize);
		let t_225 = String::from(t_224);
		let t_226 = CustomType0(t_225);
		t_138.push(t_226);
		let mut t_227 = _to_u8(GLOBAL_DATA, 953) % 17;
		let t_228 = _to_str(GLOBAL_DATA, 954, 954 + t_227 as usize);
		let t_229 = String::from(t_228);
		let t_230 = CustomType0(t_229);
		t_138.push(t_230);
		let mut t_231 = _to_u8(GLOBAL_DATA, 970) % 17;
		let t_232 = _to_str(GLOBAL_DATA, 971, 971 + t_231 as usize);
		let t_233 = String::from(t_232);
		let t_234 = CustomType0(t_233);
		t_138.push(t_234);
		let mut t_235 = _to_u8(GLOBAL_DATA, 987) % 17;
		let t_236 = _to_str(GLOBAL_DATA, 988, 988 + t_235 as usize);
		let t_237 = String::from(t_236);
		let t_238 = CustomType0(t_237);
		t_138.push(t_238);
		let mut t_239 = _to_u8(GLOBAL_DATA, 1004) % 17;
		let t_240 = _to_str(GLOBAL_DATA, 1005, 1005 + t_239 as usize);
		let t_241 = String::from(t_240);
		let t_242 = CustomType0(t_241);
		t_138.push(t_242);
		let mut t_243 = _to_u8(GLOBAL_DATA, 1021) % 17;
		let t_244 = _to_str(GLOBAL_DATA, 1022, 1022 + t_243 as usize);
		let t_245 = String::from(t_244);
		let t_246 = CustomType0(t_245);
		t_138.push(t_246);
		let mut t_247 = _to_u8(GLOBAL_DATA, 1038) % 17;
		let t_248 = _to_str(GLOBAL_DATA, 1039, 1039 + t_247 as usize);
		let t_249 = String::from(t_248);
		let t_250 = CustomType0(t_249);
		t_138.push(t_250);
		let mut t_251 = _to_u8(GLOBAL_DATA, 1055) % 17;
		let t_252 = _to_str(GLOBAL_DATA, 1056, 1056 + t_251 as usize);
		let t_253 = String::from(t_252);
		let t_254 = CustomType0(t_253);
		t_138.push(t_254);
		let mut t_255 = _to_u8(GLOBAL_DATA, 1072) % 17;
		let t_256 = _to_str(GLOBAL_DATA, 1073, 1073 + t_255 as usize);
		let t_257 = String::from(t_256);
		let t_258 = CustomType0(t_257);
		t_138.push(t_258);
		let mut t_259 = _to_u8(GLOBAL_DATA, 1089) % 17;
		let t_260 = _to_str(GLOBAL_DATA, 1090, 1090 + t_259 as usize);
		let t_261 = String::from(t_260);
		let t_262 = CustomType0(t_261);
		t_138.push(t_262);
		let mut t_263 = _to_u8(GLOBAL_DATA, 1106) % 17;
		let t_264 = _to_str(GLOBAL_DATA, 1107, 1107 + t_263 as usize);
		let t_265 = String::from(t_264);
		let t_266 = CustomType0(t_265);
		t_138.push(t_266);
		let mut t_267 = _to_u8(GLOBAL_DATA, 1123) % 17;
		let t_268 = _to_str(GLOBAL_DATA, 1124, 1124 + t_267 as usize);
		let t_269 = String::from(t_268);
		let t_270 = CustomType0(t_269);
		t_138.push(t_270);
		t_138.truncate(t_137 as usize);
		// End vector declaration.
		let t_271 = &t_138[..];
		let t_272 = &t_271;
		t_136.partial_cmp(t_272);
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