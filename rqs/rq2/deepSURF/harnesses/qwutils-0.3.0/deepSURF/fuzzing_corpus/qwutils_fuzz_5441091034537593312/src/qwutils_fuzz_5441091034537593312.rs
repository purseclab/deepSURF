//DefId(0:23 ~ qwutils[d7d7]::imp::vec::{impl#0}::insert_slice_copy)
#[macro_use]
extern crate afl;

use qwutils::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

struct CustomType0(usize);

impl std::clone::Clone for CustomType0 {
	
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
		let t_2 = _to_usize(GLOBAL_DATA, 9);
		let t_3 = CustomType0(t_2);
		return t_3;
	}
}

impl std::marker::Copy for CustomType0 {
}

fn main (){
	fuzz_nohook!(|data: &[u8]| {
		if data.len() < 1076 {return;}
		set_global_data(data);
		let global_data = get_global_data();
		let GLOBAL_DATA = global_data.first_half;
		let mut t_0 = _to_u8(GLOBAL_DATA, 0) % 33;
		// Start vector declaration.
		let mut t_1 = std::vec::Vec::with_capacity(32);
		let t_4 = _to_usize(GLOBAL_DATA, 17);
		let t_5 = CustomType0(t_4);
		t_1.push(t_5);
		let t_6 = _to_usize(GLOBAL_DATA, 25);
		let t_7 = CustomType0(t_6);
		t_1.push(t_7);
		let t_8 = _to_usize(GLOBAL_DATA, 33);
		let t_9 = CustomType0(t_8);
		t_1.push(t_9);
		let t_10 = _to_usize(GLOBAL_DATA, 41);
		let t_11 = CustomType0(t_10);
		t_1.push(t_11);
		let t_12 = _to_usize(GLOBAL_DATA, 49);
		let t_13 = CustomType0(t_12);
		t_1.push(t_13);
		let t_14 = _to_usize(GLOBAL_DATA, 57);
		let t_15 = CustomType0(t_14);
		t_1.push(t_15);
		let t_16 = _to_usize(GLOBAL_DATA, 65);
		let t_17 = CustomType0(t_16);
		t_1.push(t_17);
		let t_18 = _to_usize(GLOBAL_DATA, 73);
		let t_19 = CustomType0(t_18);
		t_1.push(t_19);
		let t_20 = _to_usize(GLOBAL_DATA, 81);
		let t_21 = CustomType0(t_20);
		t_1.push(t_21);
		let t_22 = _to_usize(GLOBAL_DATA, 89);
		let t_23 = CustomType0(t_22);
		t_1.push(t_23);
		let t_24 = _to_usize(GLOBAL_DATA, 97);
		let t_25 = CustomType0(t_24);
		t_1.push(t_25);
		let t_26 = _to_usize(GLOBAL_DATA, 105);
		let t_27 = CustomType0(t_26);
		t_1.push(t_27);
		let t_28 = _to_usize(GLOBAL_DATA, 113);
		let t_29 = CustomType0(t_28);
		t_1.push(t_29);
		let t_30 = _to_usize(GLOBAL_DATA, 121);
		let t_31 = CustomType0(t_30);
		t_1.push(t_31);
		let t_32 = _to_usize(GLOBAL_DATA, 129);
		let t_33 = CustomType0(t_32);
		t_1.push(t_33);
		let t_34 = _to_usize(GLOBAL_DATA, 137);
		let t_35 = CustomType0(t_34);
		t_1.push(t_35);
		let t_36 = _to_usize(GLOBAL_DATA, 145);
		let t_37 = CustomType0(t_36);
		t_1.push(t_37);
		let t_38 = _to_usize(GLOBAL_DATA, 153);
		let t_39 = CustomType0(t_38);
		t_1.push(t_39);
		let t_40 = _to_usize(GLOBAL_DATA, 161);
		let t_41 = CustomType0(t_40);
		t_1.push(t_41);
		let t_42 = _to_usize(GLOBAL_DATA, 169);
		let t_43 = CustomType0(t_42);
		t_1.push(t_43);
		let t_44 = _to_usize(GLOBAL_DATA, 177);
		let t_45 = CustomType0(t_44);
		t_1.push(t_45);
		let t_46 = _to_usize(GLOBAL_DATA, 185);
		let t_47 = CustomType0(t_46);
		t_1.push(t_47);
		let t_48 = _to_usize(GLOBAL_DATA, 193);
		let t_49 = CustomType0(t_48);
		t_1.push(t_49);
		let t_50 = _to_usize(GLOBAL_DATA, 201);
		let t_51 = CustomType0(t_50);
		t_1.push(t_51);
		let t_52 = _to_usize(GLOBAL_DATA, 209);
		let t_53 = CustomType0(t_52);
		t_1.push(t_53);
		let t_54 = _to_usize(GLOBAL_DATA, 217);
		let t_55 = CustomType0(t_54);
		t_1.push(t_55);
		let t_56 = _to_usize(GLOBAL_DATA, 225);
		let t_57 = CustomType0(t_56);
		t_1.push(t_57);
		let t_58 = _to_usize(GLOBAL_DATA, 233);
		let t_59 = CustomType0(t_58);
		t_1.push(t_59);
		let t_60 = _to_usize(GLOBAL_DATA, 241);
		let t_61 = CustomType0(t_60);
		t_1.push(t_61);
		let t_62 = _to_usize(GLOBAL_DATA, 249);
		let t_63 = CustomType0(t_62);
		t_1.push(t_63);
		let t_64 = _to_usize(GLOBAL_DATA, 257);
		let t_65 = CustomType0(t_64);
		t_1.push(t_65);
		let t_66 = _to_usize(GLOBAL_DATA, 265);
		let t_67 = CustomType0(t_66);
		t_1.push(t_67);
		t_1.truncate(t_0 as usize);
		// End vector declaration.
		let mut t_68 = &mut t_1;
		let t_69 = _to_usize(GLOBAL_DATA, 273);
		let mut t_70 = _to_u8(GLOBAL_DATA, 281) % 33;
		// Start vector declaration.
		let mut t_71 = std::vec::Vec::with_capacity(32);
		let t_72 = _to_usize(GLOBAL_DATA, 282);
		let t_73 = CustomType0(t_72);
		t_71.push(t_73);
		let t_74 = _to_usize(GLOBAL_DATA, 290);
		let t_75 = CustomType0(t_74);
		t_71.push(t_75);
		let t_76 = _to_usize(GLOBAL_DATA, 298);
		let t_77 = CustomType0(t_76);
		t_71.push(t_77);
		let t_78 = _to_usize(GLOBAL_DATA, 306);
		let t_79 = CustomType0(t_78);
		t_71.push(t_79);
		let t_80 = _to_usize(GLOBAL_DATA, 314);
		let t_81 = CustomType0(t_80);
		t_71.push(t_81);
		let t_82 = _to_usize(GLOBAL_DATA, 322);
		let t_83 = CustomType0(t_82);
		t_71.push(t_83);
		let t_84 = _to_usize(GLOBAL_DATA, 330);
		let t_85 = CustomType0(t_84);
		t_71.push(t_85);
		let t_86 = _to_usize(GLOBAL_DATA, 338);
		let t_87 = CustomType0(t_86);
		t_71.push(t_87);
		let t_88 = _to_usize(GLOBAL_DATA, 346);
		let t_89 = CustomType0(t_88);
		t_71.push(t_89);
		let t_90 = _to_usize(GLOBAL_DATA, 354);
		let t_91 = CustomType0(t_90);
		t_71.push(t_91);
		let t_92 = _to_usize(GLOBAL_DATA, 362);
		let t_93 = CustomType0(t_92);
		t_71.push(t_93);
		let t_94 = _to_usize(GLOBAL_DATA, 370);
		let t_95 = CustomType0(t_94);
		t_71.push(t_95);
		let t_96 = _to_usize(GLOBAL_DATA, 378);
		let t_97 = CustomType0(t_96);
		t_71.push(t_97);
		let t_98 = _to_usize(GLOBAL_DATA, 386);
		let t_99 = CustomType0(t_98);
		t_71.push(t_99);
		let t_100 = _to_usize(GLOBAL_DATA, 394);
		let t_101 = CustomType0(t_100);
		t_71.push(t_101);
		let t_102 = _to_usize(GLOBAL_DATA, 402);
		let t_103 = CustomType0(t_102);
		t_71.push(t_103);
		let t_104 = _to_usize(GLOBAL_DATA, 410);
		let t_105 = CustomType0(t_104);
		t_71.push(t_105);
		let t_106 = _to_usize(GLOBAL_DATA, 418);
		let t_107 = CustomType0(t_106);
		t_71.push(t_107);
		let t_108 = _to_usize(GLOBAL_DATA, 426);
		let t_109 = CustomType0(t_108);
		t_71.push(t_109);
		let t_110 = _to_usize(GLOBAL_DATA, 434);
		let t_111 = CustomType0(t_110);
		t_71.push(t_111);
		let t_112 = _to_usize(GLOBAL_DATA, 442);
		let t_113 = CustomType0(t_112);
		t_71.push(t_113);
		let t_114 = _to_usize(GLOBAL_DATA, 450);
		let t_115 = CustomType0(t_114);
		t_71.push(t_115);
		let t_116 = _to_usize(GLOBAL_DATA, 458);
		let t_117 = CustomType0(t_116);
		t_71.push(t_117);
		let t_118 = _to_usize(GLOBAL_DATA, 466);
		let t_119 = CustomType0(t_118);
		t_71.push(t_119);
		let t_120 = _to_usize(GLOBAL_DATA, 474);
		let t_121 = CustomType0(t_120);
		t_71.push(t_121);
		let t_122 = _to_usize(GLOBAL_DATA, 482);
		let t_123 = CustomType0(t_122);
		t_71.push(t_123);
		let t_124 = _to_usize(GLOBAL_DATA, 490);
		let t_125 = CustomType0(t_124);
		t_71.push(t_125);
		let t_126 = _to_usize(GLOBAL_DATA, 498);
		let t_127 = CustomType0(t_126);
		t_71.push(t_127);
		let t_128 = _to_usize(GLOBAL_DATA, 506);
		let t_129 = CustomType0(t_128);
		t_71.push(t_129);
		let t_130 = _to_usize(GLOBAL_DATA, 514);
		let t_131 = CustomType0(t_130);
		t_71.push(t_131);
		let t_132 = _to_usize(GLOBAL_DATA, 522);
		let t_133 = CustomType0(t_132);
		t_71.push(t_133);
		let t_134 = _to_usize(GLOBAL_DATA, 530);
		let t_135 = CustomType0(t_134);
		t_71.push(t_135);
		t_71.truncate(t_70 as usize);
		// End vector declaration.
		let t_136 = &t_71[..];
		t_68.insert_slice_copy(t_69, t_136);
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