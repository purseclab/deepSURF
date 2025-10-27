//DefId(0:244 ~ qwutils[d7d7]::arc_slice::imp::{impl#0}::insert_slice)
#[macro_use]
extern crate afl;

use qwutils::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

struct CustomType0(String);

impl std::clone::Clone for CustomType0 {
	
	fn clone(&self) -> Self {
		let global_data = get_global_data();
		let GLOBAL_DATA = global_data.first_half;
		let custom_impl_num = _to_usize(GLOBAL_DATA, 9);
		let custom_impl_inst_num = self.0.len();
		let selector = (custom_impl_num + custom_impl_inst_num) % 3;
		if selector == 0{
			panic!("INTENTIONAL PANIC!");
		}
		let GLOBAL_DATA = match selector{
				1 => global_data.first_half,
				_ => global_data.second_half,
		};
		let mut t_5 = _to_u8(GLOBAL_DATA, 17) % 17;
		let t_6 = _to_str(GLOBAL_DATA, 18, 18 + t_5 as usize);
		let t_7 = String::from(t_6);
		let t_8 = CustomType0(t_7);
		return t_8;
	}
}

fn main (){
	fuzz_nohook!(|data: &[u8]| {
		if data.len() < 1156 {return;}
		set_global_data(data);
		let global_data = get_global_data();
		let GLOBAL_DATA = global_data.first_half;
		let mut t_0 = qwutils::arc_slice::ArcSlice::new();
		let mut t_1 = &mut t_0;
		let t_2 = _to_usize(GLOBAL_DATA, 0);
		let mut t_3 = _to_u8(GLOBAL_DATA, 8) % 33;
		// Start vector declaration.
		let mut t_4 = std::vec::Vec::with_capacity(32);
		let mut t_9 = _to_u8(GLOBAL_DATA, 34) % 17;
		let t_10 = _to_str(GLOBAL_DATA, 35, 35 + t_9 as usize);
		let t_11 = String::from(t_10);
		let t_12 = CustomType0(t_11);
		t_4.push(t_12);
		let mut t_13 = _to_u8(GLOBAL_DATA, 51) % 17;
		let t_14 = _to_str(GLOBAL_DATA, 52, 52 + t_13 as usize);
		let t_15 = String::from(t_14);
		let t_16 = CustomType0(t_15);
		t_4.push(t_16);
		let mut t_17 = _to_u8(GLOBAL_DATA, 68) % 17;
		let t_18 = _to_str(GLOBAL_DATA, 69, 69 + t_17 as usize);
		let t_19 = String::from(t_18);
		let t_20 = CustomType0(t_19);
		t_4.push(t_20);
		let mut t_21 = _to_u8(GLOBAL_DATA, 85) % 17;
		let t_22 = _to_str(GLOBAL_DATA, 86, 86 + t_21 as usize);
		let t_23 = String::from(t_22);
		let t_24 = CustomType0(t_23);
		t_4.push(t_24);
		let mut t_25 = _to_u8(GLOBAL_DATA, 102) % 17;
		let t_26 = _to_str(GLOBAL_DATA, 103, 103 + t_25 as usize);
		let t_27 = String::from(t_26);
		let t_28 = CustomType0(t_27);
		t_4.push(t_28);
		let mut t_29 = _to_u8(GLOBAL_DATA, 119) % 17;
		let t_30 = _to_str(GLOBAL_DATA, 120, 120 + t_29 as usize);
		let t_31 = String::from(t_30);
		let t_32 = CustomType0(t_31);
		t_4.push(t_32);
		let mut t_33 = _to_u8(GLOBAL_DATA, 136) % 17;
		let t_34 = _to_str(GLOBAL_DATA, 137, 137 + t_33 as usize);
		let t_35 = String::from(t_34);
		let t_36 = CustomType0(t_35);
		t_4.push(t_36);
		let mut t_37 = _to_u8(GLOBAL_DATA, 153) % 17;
		let t_38 = _to_str(GLOBAL_DATA, 154, 154 + t_37 as usize);
		let t_39 = String::from(t_38);
		let t_40 = CustomType0(t_39);
		t_4.push(t_40);
		let mut t_41 = _to_u8(GLOBAL_DATA, 170) % 17;
		let t_42 = _to_str(GLOBAL_DATA, 171, 171 + t_41 as usize);
		let t_43 = String::from(t_42);
		let t_44 = CustomType0(t_43);
		t_4.push(t_44);
		let mut t_45 = _to_u8(GLOBAL_DATA, 187) % 17;
		let t_46 = _to_str(GLOBAL_DATA, 188, 188 + t_45 as usize);
		let t_47 = String::from(t_46);
		let t_48 = CustomType0(t_47);
		t_4.push(t_48);
		let mut t_49 = _to_u8(GLOBAL_DATA, 204) % 17;
		let t_50 = _to_str(GLOBAL_DATA, 205, 205 + t_49 as usize);
		let t_51 = String::from(t_50);
		let t_52 = CustomType0(t_51);
		t_4.push(t_52);
		let mut t_53 = _to_u8(GLOBAL_DATA, 221) % 17;
		let t_54 = _to_str(GLOBAL_DATA, 222, 222 + t_53 as usize);
		let t_55 = String::from(t_54);
		let t_56 = CustomType0(t_55);
		t_4.push(t_56);
		let mut t_57 = _to_u8(GLOBAL_DATA, 238) % 17;
		let t_58 = _to_str(GLOBAL_DATA, 239, 239 + t_57 as usize);
		let t_59 = String::from(t_58);
		let t_60 = CustomType0(t_59);
		t_4.push(t_60);
		let mut t_61 = _to_u8(GLOBAL_DATA, 255) % 17;
		let t_62 = _to_str(GLOBAL_DATA, 256, 256 + t_61 as usize);
		let t_63 = String::from(t_62);
		let t_64 = CustomType0(t_63);
		t_4.push(t_64);
		let mut t_65 = _to_u8(GLOBAL_DATA, 272) % 17;
		let t_66 = _to_str(GLOBAL_DATA, 273, 273 + t_65 as usize);
		let t_67 = String::from(t_66);
		let t_68 = CustomType0(t_67);
		t_4.push(t_68);
		let mut t_69 = _to_u8(GLOBAL_DATA, 289) % 17;
		let t_70 = _to_str(GLOBAL_DATA, 290, 290 + t_69 as usize);
		let t_71 = String::from(t_70);
		let t_72 = CustomType0(t_71);
		t_4.push(t_72);
		let mut t_73 = _to_u8(GLOBAL_DATA, 306) % 17;
		let t_74 = _to_str(GLOBAL_DATA, 307, 307 + t_73 as usize);
		let t_75 = String::from(t_74);
		let t_76 = CustomType0(t_75);
		t_4.push(t_76);
		let mut t_77 = _to_u8(GLOBAL_DATA, 323) % 17;
		let t_78 = _to_str(GLOBAL_DATA, 324, 324 + t_77 as usize);
		let t_79 = String::from(t_78);
		let t_80 = CustomType0(t_79);
		t_4.push(t_80);
		let mut t_81 = _to_u8(GLOBAL_DATA, 340) % 17;
		let t_82 = _to_str(GLOBAL_DATA, 341, 341 + t_81 as usize);
		let t_83 = String::from(t_82);
		let t_84 = CustomType0(t_83);
		t_4.push(t_84);
		let mut t_85 = _to_u8(GLOBAL_DATA, 357) % 17;
		let t_86 = _to_str(GLOBAL_DATA, 358, 358 + t_85 as usize);
		let t_87 = String::from(t_86);
		let t_88 = CustomType0(t_87);
		t_4.push(t_88);
		let mut t_89 = _to_u8(GLOBAL_DATA, 374) % 17;
		let t_90 = _to_str(GLOBAL_DATA, 375, 375 + t_89 as usize);
		let t_91 = String::from(t_90);
		let t_92 = CustomType0(t_91);
		t_4.push(t_92);
		let mut t_93 = _to_u8(GLOBAL_DATA, 391) % 17;
		let t_94 = _to_str(GLOBAL_DATA, 392, 392 + t_93 as usize);
		let t_95 = String::from(t_94);
		let t_96 = CustomType0(t_95);
		t_4.push(t_96);
		let mut t_97 = _to_u8(GLOBAL_DATA, 408) % 17;
		let t_98 = _to_str(GLOBAL_DATA, 409, 409 + t_97 as usize);
		let t_99 = String::from(t_98);
		let t_100 = CustomType0(t_99);
		t_4.push(t_100);
		let mut t_101 = _to_u8(GLOBAL_DATA, 425) % 17;
		let t_102 = _to_str(GLOBAL_DATA, 426, 426 + t_101 as usize);
		let t_103 = String::from(t_102);
		let t_104 = CustomType0(t_103);
		t_4.push(t_104);
		let mut t_105 = _to_u8(GLOBAL_DATA, 442) % 17;
		let t_106 = _to_str(GLOBAL_DATA, 443, 443 + t_105 as usize);
		let t_107 = String::from(t_106);
		let t_108 = CustomType0(t_107);
		t_4.push(t_108);
		let mut t_109 = _to_u8(GLOBAL_DATA, 459) % 17;
		let t_110 = _to_str(GLOBAL_DATA, 460, 460 + t_109 as usize);
		let t_111 = String::from(t_110);
		let t_112 = CustomType0(t_111);
		t_4.push(t_112);
		let mut t_113 = _to_u8(GLOBAL_DATA, 476) % 17;
		let t_114 = _to_str(GLOBAL_DATA, 477, 477 + t_113 as usize);
		let t_115 = String::from(t_114);
		let t_116 = CustomType0(t_115);
		t_4.push(t_116);
		let mut t_117 = _to_u8(GLOBAL_DATA, 493) % 17;
		let t_118 = _to_str(GLOBAL_DATA, 494, 494 + t_117 as usize);
		let t_119 = String::from(t_118);
		let t_120 = CustomType0(t_119);
		t_4.push(t_120);
		let mut t_121 = _to_u8(GLOBAL_DATA, 510) % 17;
		let t_122 = _to_str(GLOBAL_DATA, 511, 511 + t_121 as usize);
		let t_123 = String::from(t_122);
		let t_124 = CustomType0(t_123);
		t_4.push(t_124);
		let mut t_125 = _to_u8(GLOBAL_DATA, 527) % 17;
		let t_126 = _to_str(GLOBAL_DATA, 528, 528 + t_125 as usize);
		let t_127 = String::from(t_126);
		let t_128 = CustomType0(t_127);
		t_4.push(t_128);
		let mut t_129 = _to_u8(GLOBAL_DATA, 544) % 17;
		let t_130 = _to_str(GLOBAL_DATA, 545, 545 + t_129 as usize);
		let t_131 = String::from(t_130);
		let t_132 = CustomType0(t_131);
		t_4.push(t_132);
		let mut t_133 = _to_u8(GLOBAL_DATA, 561) % 17;
		let t_134 = _to_str(GLOBAL_DATA, 562, 562 + t_133 as usize);
		let t_135 = String::from(t_134);
		let t_136 = CustomType0(t_135);
		t_4.push(t_136);
		t_4.truncate(t_3 as usize);
		// End vector declaration.
		let t_137 = &t_4[..];
		t_1.insert_slice(t_2, t_137);
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