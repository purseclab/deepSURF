//DefId(0:165 ~ toodee[604e]::view::{impl#7}::index)
#[macro_use]
extern crate afl;

use toodee::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

struct CustomType0(String);

fn main (){
	fuzz_nohook!(|data: &[u8]| {
		if data.len() < 1202 {return;}
		set_global_data(data);
		let global_data = get_global_data();
		let GLOBAL_DATA = global_data.first_half;
		let t_0 = _to_usize(GLOBAL_DATA, 0);
		let t_1 = _to_usize(GLOBAL_DATA, 8);
		let mut t_2 = _to_u8(GLOBAL_DATA, 16) % 33;
		// Start vector declaration.
		let mut t_3 = std::vec::Vec::with_capacity(32);
		let mut t_4 = _to_u8(GLOBAL_DATA, 17) % 17;
		let t_5 = _to_str(GLOBAL_DATA, 18, 18 + t_4 as usize);
		let t_6 = String::from(t_5);
		let t_7 = CustomType0(t_6);
		t_3.push(t_7);
		let mut t_8 = _to_u8(GLOBAL_DATA, 34) % 17;
		let t_9 = _to_str(GLOBAL_DATA, 35, 35 + t_8 as usize);
		let t_10 = String::from(t_9);
		let t_11 = CustomType0(t_10);
		t_3.push(t_11);
		let mut t_12 = _to_u8(GLOBAL_DATA, 51) % 17;
		let t_13 = _to_str(GLOBAL_DATA, 52, 52 + t_12 as usize);
		let t_14 = String::from(t_13);
		let t_15 = CustomType0(t_14);
		t_3.push(t_15);
		let mut t_16 = _to_u8(GLOBAL_DATA, 68) % 17;
		let t_17 = _to_str(GLOBAL_DATA, 69, 69 + t_16 as usize);
		let t_18 = String::from(t_17);
		let t_19 = CustomType0(t_18);
		t_3.push(t_19);
		let mut t_20 = _to_u8(GLOBAL_DATA, 85) % 17;
		let t_21 = _to_str(GLOBAL_DATA, 86, 86 + t_20 as usize);
		let t_22 = String::from(t_21);
		let t_23 = CustomType0(t_22);
		t_3.push(t_23);
		let mut t_24 = _to_u8(GLOBAL_DATA, 102) % 17;
		let t_25 = _to_str(GLOBAL_DATA, 103, 103 + t_24 as usize);
		let t_26 = String::from(t_25);
		let t_27 = CustomType0(t_26);
		t_3.push(t_27);
		let mut t_28 = _to_u8(GLOBAL_DATA, 119) % 17;
		let t_29 = _to_str(GLOBAL_DATA, 120, 120 + t_28 as usize);
		let t_30 = String::from(t_29);
		let t_31 = CustomType0(t_30);
		t_3.push(t_31);
		let mut t_32 = _to_u8(GLOBAL_DATA, 136) % 17;
		let t_33 = _to_str(GLOBAL_DATA, 137, 137 + t_32 as usize);
		let t_34 = String::from(t_33);
		let t_35 = CustomType0(t_34);
		t_3.push(t_35);
		let mut t_36 = _to_u8(GLOBAL_DATA, 153) % 17;
		let t_37 = _to_str(GLOBAL_DATA, 154, 154 + t_36 as usize);
		let t_38 = String::from(t_37);
		let t_39 = CustomType0(t_38);
		t_3.push(t_39);
		let mut t_40 = _to_u8(GLOBAL_DATA, 170) % 17;
		let t_41 = _to_str(GLOBAL_DATA, 171, 171 + t_40 as usize);
		let t_42 = String::from(t_41);
		let t_43 = CustomType0(t_42);
		t_3.push(t_43);
		let mut t_44 = _to_u8(GLOBAL_DATA, 187) % 17;
		let t_45 = _to_str(GLOBAL_DATA, 188, 188 + t_44 as usize);
		let t_46 = String::from(t_45);
		let t_47 = CustomType0(t_46);
		t_3.push(t_47);
		let mut t_48 = _to_u8(GLOBAL_DATA, 204) % 17;
		let t_49 = _to_str(GLOBAL_DATA, 205, 205 + t_48 as usize);
		let t_50 = String::from(t_49);
		let t_51 = CustomType0(t_50);
		t_3.push(t_51);
		let mut t_52 = _to_u8(GLOBAL_DATA, 221) % 17;
		let t_53 = _to_str(GLOBAL_DATA, 222, 222 + t_52 as usize);
		let t_54 = String::from(t_53);
		let t_55 = CustomType0(t_54);
		t_3.push(t_55);
		let mut t_56 = _to_u8(GLOBAL_DATA, 238) % 17;
		let t_57 = _to_str(GLOBAL_DATA, 239, 239 + t_56 as usize);
		let t_58 = String::from(t_57);
		let t_59 = CustomType0(t_58);
		t_3.push(t_59);
		let mut t_60 = _to_u8(GLOBAL_DATA, 255) % 17;
		let t_61 = _to_str(GLOBAL_DATA, 256, 256 + t_60 as usize);
		let t_62 = String::from(t_61);
		let t_63 = CustomType0(t_62);
		t_3.push(t_63);
		let mut t_64 = _to_u8(GLOBAL_DATA, 272) % 17;
		let t_65 = _to_str(GLOBAL_DATA, 273, 273 + t_64 as usize);
		let t_66 = String::from(t_65);
		let t_67 = CustomType0(t_66);
		t_3.push(t_67);
		let mut t_68 = _to_u8(GLOBAL_DATA, 289) % 17;
		let t_69 = _to_str(GLOBAL_DATA, 290, 290 + t_68 as usize);
		let t_70 = String::from(t_69);
		let t_71 = CustomType0(t_70);
		t_3.push(t_71);
		let mut t_72 = _to_u8(GLOBAL_DATA, 306) % 17;
		let t_73 = _to_str(GLOBAL_DATA, 307, 307 + t_72 as usize);
		let t_74 = String::from(t_73);
		let t_75 = CustomType0(t_74);
		t_3.push(t_75);
		let mut t_76 = _to_u8(GLOBAL_DATA, 323) % 17;
		let t_77 = _to_str(GLOBAL_DATA, 324, 324 + t_76 as usize);
		let t_78 = String::from(t_77);
		let t_79 = CustomType0(t_78);
		t_3.push(t_79);
		let mut t_80 = _to_u8(GLOBAL_DATA, 340) % 17;
		let t_81 = _to_str(GLOBAL_DATA, 341, 341 + t_80 as usize);
		let t_82 = String::from(t_81);
		let t_83 = CustomType0(t_82);
		t_3.push(t_83);
		let mut t_84 = _to_u8(GLOBAL_DATA, 357) % 17;
		let t_85 = _to_str(GLOBAL_DATA, 358, 358 + t_84 as usize);
		let t_86 = String::from(t_85);
		let t_87 = CustomType0(t_86);
		t_3.push(t_87);
		let mut t_88 = _to_u8(GLOBAL_DATA, 374) % 17;
		let t_89 = _to_str(GLOBAL_DATA, 375, 375 + t_88 as usize);
		let t_90 = String::from(t_89);
		let t_91 = CustomType0(t_90);
		t_3.push(t_91);
		let mut t_92 = _to_u8(GLOBAL_DATA, 391) % 17;
		let t_93 = _to_str(GLOBAL_DATA, 392, 392 + t_92 as usize);
		let t_94 = String::from(t_93);
		let t_95 = CustomType0(t_94);
		t_3.push(t_95);
		let mut t_96 = _to_u8(GLOBAL_DATA, 408) % 17;
		let t_97 = _to_str(GLOBAL_DATA, 409, 409 + t_96 as usize);
		let t_98 = String::from(t_97);
		let t_99 = CustomType0(t_98);
		t_3.push(t_99);
		let mut t_100 = _to_u8(GLOBAL_DATA, 425) % 17;
		let t_101 = _to_str(GLOBAL_DATA, 426, 426 + t_100 as usize);
		let t_102 = String::from(t_101);
		let t_103 = CustomType0(t_102);
		t_3.push(t_103);
		let mut t_104 = _to_u8(GLOBAL_DATA, 442) % 17;
		let t_105 = _to_str(GLOBAL_DATA, 443, 443 + t_104 as usize);
		let t_106 = String::from(t_105);
		let t_107 = CustomType0(t_106);
		t_3.push(t_107);
		let mut t_108 = _to_u8(GLOBAL_DATA, 459) % 17;
		let t_109 = _to_str(GLOBAL_DATA, 460, 460 + t_108 as usize);
		let t_110 = String::from(t_109);
		let t_111 = CustomType0(t_110);
		t_3.push(t_111);
		let mut t_112 = _to_u8(GLOBAL_DATA, 476) % 17;
		let t_113 = _to_str(GLOBAL_DATA, 477, 477 + t_112 as usize);
		let t_114 = String::from(t_113);
		let t_115 = CustomType0(t_114);
		t_3.push(t_115);
		let mut t_116 = _to_u8(GLOBAL_DATA, 493) % 17;
		let t_117 = _to_str(GLOBAL_DATA, 494, 494 + t_116 as usize);
		let t_118 = String::from(t_117);
		let t_119 = CustomType0(t_118);
		t_3.push(t_119);
		let mut t_120 = _to_u8(GLOBAL_DATA, 510) % 17;
		let t_121 = _to_str(GLOBAL_DATA, 511, 511 + t_120 as usize);
		let t_122 = String::from(t_121);
		let t_123 = CustomType0(t_122);
		t_3.push(t_123);
		let mut t_124 = _to_u8(GLOBAL_DATA, 527) % 17;
		let t_125 = _to_str(GLOBAL_DATA, 528, 528 + t_124 as usize);
		let t_126 = String::from(t_125);
		let t_127 = CustomType0(t_126);
		t_3.push(t_127);
		let mut t_128 = _to_u8(GLOBAL_DATA, 544) % 17;
		let t_129 = _to_str(GLOBAL_DATA, 545, 545 + t_128 as usize);
		let t_130 = String::from(t_129);
		let t_131 = CustomType0(t_130);
		t_3.push(t_131);
		t_3.truncate(t_2 as usize);
		// End vector declaration.
		let mut t_132 = toodee::TooDee::from_vec(t_0, t_1, t_3);
		let mut t_133 = &mut t_132;
		let t_134 = _to_usize(GLOBAL_DATA, 561);
		let t_135 = _to_usize(GLOBAL_DATA, 569);
		let t_136 = (t_134, t_135);
		let t_137 = _to_usize(GLOBAL_DATA, 577);
		let t_138 = _to_usize(GLOBAL_DATA, 585);
		let t_139 = (t_137, t_138);
		let t_140 = toodee::TooDee::view_mut(t_133, t_136, t_139);
		let t_141 = &t_140;
		let t_142 = _to_usize(GLOBAL_DATA, 593);
		t_141.index(t_142);
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