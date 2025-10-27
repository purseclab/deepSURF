#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use toodee::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

#[derive(Debug, Clone, Default)]
struct CustomType0(String);
#[derive(Debug)]
struct CustomType3(String);
#[derive(Debug)]
struct CustomType1(String);
#[derive(Debug)]
struct CustomType2(String);

impl core::iter::ExactSizeIterator for CustomType1 {
	
	fn len(&self) -> usize {
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
		let t_0 = _to_usize(GLOBAL_DATA, 8);
		return t_0;
	}
}

impl core::iter::Iterator for CustomType1 {
	type Item = CustomType0;
	
	fn next(&mut self) -> Option<Self::Item> {
		let global_data = get_global_data();
		let GLOBAL_DATA = global_data.first_half;
		let custom_impl_num = _to_usize(GLOBAL_DATA, 16);
		let custom_impl_inst_num = self.0.len();
		let selector = (custom_impl_num + custom_impl_inst_num) % 3;
		if selector == 0{
			panic!("INTENTIONAL PANIC!");
		}
		let GLOBAL_DATA = match selector{
				1 => global_data.first_half,
				_ => global_data.second_half,
		};
		let mut t_1 = _to_u8(GLOBAL_DATA, 24) % 17;
		let t_2 = _to_str(GLOBAL_DATA, 25, 25 + t_1 as usize);
		let t_3 = String::from(t_2);
		let t_4 = CustomType0(t_3);
		let t_5 = Some(t_4);
		return t_5;
	}
}

impl core::iter::IntoIterator for CustomType3 {
	type Item = CustomType0;
	type IntoIter = CustomType1;
	
	fn into_iter(self) -> Self::IntoIter {
		let global_data = get_global_data();
		let GLOBAL_DATA = global_data.first_half;
		let custom_impl_num = _to_usize(GLOBAL_DATA, 660);
		let custom_impl_inst_num = self.0.len();
		let selector = (custom_impl_num + custom_impl_inst_num) % 3;
		if selector == 0{
			panic!("INTENTIONAL PANIC!");
		}
		let GLOBAL_DATA = match selector{
				1 => global_data.first_half,
				_ => global_data.second_half,
		};
		let mut t_151 = _to_u8(GLOBAL_DATA, 668) % 17;
		let t_152 = _to_str(GLOBAL_DATA, 669, 669 + t_151 as usize);
		let t_153 = String::from(t_152);
		let t_154 = CustomType1(t_153);
		return t_154;
	}
}

impl core::iter::DoubleEndedIterator for CustomType1 {
	
	fn next_back(&mut self) -> Option<Self::Item> {
		let global_data = get_global_data();
		let GLOBAL_DATA = global_data.first_half;
		let custom_impl_num = _to_usize(GLOBAL_DATA, 66);
		let custom_impl_inst_num = self.0.len();
		let selector = (custom_impl_num + custom_impl_inst_num) % 3;
		if selector == 0{
			panic!("INTENTIONAL PANIC!");
		}
		let GLOBAL_DATA = match selector{
				1 => global_data.first_half,
				_ => global_data.second_half,
		};
		let mut t_11 = _to_u8(GLOBAL_DATA, 74) % 17;
		let t_12 = _to_str(GLOBAL_DATA, 75, 75 + t_11 as usize);
		let t_13 = String::from(t_12);
		let t_14 = CustomType0(t_13);
		let t_15 = Some(t_14);
		return t_15;
	}
}

fn main (){
	fuzz_nohook!(|data: &[u8]| {
		if data.len() < 1404 {return;}
		set_global_data(data);
		let global_data = get_global_data();
		let GLOBAL_DATA = global_data.first_half;
		let t_cols = _to_usize(GLOBAL_DATA, 91);
		let t_rows = _to_usize(GLOBAL_DATA, 99);
		let mut n_init = _to_u8(GLOBAL_DATA, 107) % 33;
		let mut vbase: Vec<CustomType0> = std::vec::Vec::with_capacity(64);
		let mut i = 0u8;
		while (i as usize) < (n_init as usize) && (i as usize) < 20 {
			let len_b = _to_u8(GLOBAL_DATA, 108 + (i as usize) * 17) % 17;
			let sidx = 109 + (i as usize) * 17;
			let eidx = sidx + len_b as usize;
			let s = _to_str(GLOBAL_DATA, sidx, eidx);
			vbase.push(CustomType0(String::from(s)));
			i += 1;
		}
		let ctor_sel = _to_u8(GLOBAL_DATA, 652);
		let mut td: toodee::TooDee<CustomType0> = match ctor_sel % 5 {
			0 => {
				let vclone = vbase.clone();
				toodee::TooDee::from_vec(t_cols, t_rows, vclone)
			}
			1 => {
				toodee::TooDee::new(t_cols, t_rows)
			}
			2 => {
				let cap = _to_usize(GLOBAL_DATA, 659);
				toodee::TooDee::with_capacity(cap)
			}
			3 => {
				let vclone = vbase.clone().into_boxed_slice();
				toodee::TooDee::from_box(t_cols, t_rows, vclone)
			}
			_ => {
				let l = _to_u8(GLOBAL_DATA, 666) % 17;
				let s = _to_str(GLOBAL_DATA, 667, 667 + l as usize);
				let initv = CustomType0(String::from(s));
				toodee::TooDee::init(t_cols, t_rows, initv)
			}
		};
		let capq = td.capacity();
		let _ = capq;
		td.reserve(_to_usize(GLOBAL_DATA, 675));
		td.reserve_exact(_to_usize(GLOBAL_DATA, 683));
		let mut ops = (_to_u8(GLOBAL_DATA, 690) % 16) as usize;
		if ops == 0 { ops = 1; }
		let mut step = 0usize;
		while step < ops {
			let sel = _to_u8(GLOBAL_DATA, 691 + step) % 10;
			match sel {
				0 => {
					let mut vv: Vec<CustomType0> = Vec::with_capacity(64);
					let mut j = 0usize;
					while j < 16 {
						let lb = _to_u8(GLOBAL_DATA, 700 + j) % 17;
						let sidx = 716 + j * 7;
						let eidx = sidx + lb as usize;
						let s = _to_str(GLOBAL_DATA, sidx, eidx);
						vv.push(CustomType0(String::from(s)));
						j += 1;
					}
					let idx = _to_usize(GLOBAL_DATA, 740 + step);
					td.insert_col(idx, vv);
				}
				1 => {
					let mut vv: Vec<CustomType0> = Vec::with_capacity(32);
					let mut j = 0usize;
					while j < 12 {
						let lb = _to_u8(GLOBAL_DATA, 760 + j) % 17;
						let sidx = 780 + j * 5;
						let eidx = sidx + lb as usize;
						let s = _to_str(GLOBAL_DATA, sidx, eidx);
						vv.push(CustomType0(String::from(s)));
						j += 1;
					}
					td.push_col(vv);
				}
				2 => {
					let idx = _to_usize(GLOBAL_DATA, 840 + step);
					let mut dc = td.remove_col(idx);
					let _ = dc.next();
					let _ = dc.next_back();
				}
				3 => {
					if let Some(mut dc) = td.pop_col() {
						let _ = dc.next();
						let _ = dc.next_back();
					}
				}
				4 => {
					let mut r = td.rows();
					let _ = r.next();
					let _ = r.next_back();
				}
				5 => {
					let a = (_to_usize(GLOBAL_DATA, 860 + step), _to_usize(GLOBAL_DATA, 900 + step));
					let b = (_to_usize(GLOBAL_DATA, 880 + step), _to_usize(GLOBAL_DATA, 920 + step));
					let view = td.view(a, b);
					let mut rr = view.rows();
					let _ = rr.next();
					let c = view.col(_to_usize(GLOBAL_DATA, 940 + step));
					let mut c2 = td.col(_to_usize(GLOBAL_DATA, 960 + step));
					let _ = c2.next();
					let _ = c2.nth(_to_usize(GLOBAL_DATA, 980 + step));
				}
				6 => {
					let a = (_to_usize(GLOBAL_DATA, 1000 + step), _to_usize(GLOBAL_DATA, 1020 + step));
					let b = (_to_usize(GLOBAL_DATA, 1040 + step), _to_usize(GLOBAL_DATA, 1060 + step));
					let mut vm = td.view_mut(a, b);
					{
						let rc = vm.col(_to_usize(GLOBAL_DATA, 1080 + step));
						let _ = rc.into_iter().count();
					}
					{
						let mut cm = vm.col_mut(_to_usize(GLOBAL_DATA, 1100 + step));
						let _ = cm.next();
						let _ = cm.nth(_to_usize(GLOBAL_DATA, 1120 + step));
					}
					vm.swap_rows(_to_usize(GLOBAL_DATA, 1140 + step), _to_usize(GLOBAL_DATA, 1160 + step));
					let _v2: TooDeeView<CustomType0> = vm.into();
				}
				7 => {
					let mut cm = td.col_mut(_to_usize(GLOBAL_DATA, 1180 + step));
					let _ = cm.next_back();
					let _ = cm.last();
				}
				8 => {
					td.swap_rows(_to_usize(GLOBAL_DATA, 1200 + step), _to_usize(GLOBAL_DATA, 1220 + step));
				}
				_ => {
					let fill_len = _to_u8(GLOBAL_DATA, 1240 + step) % 17;
					let f = _to_str(GLOBAL_DATA, 1250 + step, 1250 + step + fill_len as usize);
					td.fill(CustomType0(String::from(f)));
				}
			}
			step += 1;
		}
		let dref = td.data();
		if !dref.is_empty() {
			let r = &dref[0];
			println!("{:?}", r);
		}
		let idx_final = _to_usize(GLOBAL_DATA, 660);
		let mut final_vec: Vec<CustomType0> = Vec::with_capacity(64);
		let mut j = 0usize;
		while j < 16 {
			let lb = _to_u8(GLOBAL_DATA, 668 + j) % 17;
			let sidx = 686 + j * 4;
			let eidx = sidx + lb as usize;
			let s = _to_str(GLOBAL_DATA, sidx, eidx);
			final_vec.push(CustomType0(String::from(s)));
			j += 1;
		}
		td.insert_col(idx_final, final_vec);
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