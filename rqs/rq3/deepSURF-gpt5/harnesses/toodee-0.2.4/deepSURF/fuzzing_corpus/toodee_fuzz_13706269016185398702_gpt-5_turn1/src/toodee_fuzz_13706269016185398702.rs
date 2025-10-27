#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use toodee::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

#[derive(Debug, Clone)]
struct CustomType0(String);
#[derive(Debug, Clone)]
struct CustomType2(String);
#[derive(Debug, Clone)]
struct CustomType1(String);

impl core::iter::IntoIterator for CustomType0 {
	type Item = CustomType1;
	type IntoIter = CustomType2;
	fn into_iter(self) -> Self::IntoIter {
		let global_data = get_global_data();
		let GLOBAL_DATA = global_data.first_half;
		let custom_impl_num = _to_usize(GLOBAL_DATA, 57);
		let custom_impl_inst_num = self.0.len();
		let selector = (custom_impl_num + custom_impl_inst_num) % 3;
		if selector == 0{
			panic!("INTENTIONAL PANIC!");
		}
		let GLOBAL_DATA = match selector{
			1 => global_data.first_half,
			_ => global_data.second_half,
		};
		let mut t_10 = _to_u8(GLOBAL_DATA, 65) % 17;
		let t_11 = _to_str(GLOBAL_DATA, 66, 66 + t_10 as usize);
		let t_12 = String::from(t_11);
		let t_13 = CustomType2(t_12);
		return t_13;
	}
}

impl core::iter::Iterator for CustomType2 {
	type Item = CustomType1;
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
		let mut t_4 = _to_u8(GLOBAL_DATA, 24) % 17;
		let t_5 = _to_str(GLOBAL_DATA, 25, 25 + t_4 as usize);
		let t_6 = String::from(t_5);
		let t_7 = CustomType1(t_6);
		let t_8 = Some(t_7);
		return t_8;
	}
}

impl core::iter::ExactSizeIterator for CustomType2 {
	fn len(&self) -> usize {
		let global_data = get_global_data();
		let GLOBAL_DATA = global_data.first_half;
		let custom_impl_num = _to_usize(GLOBAL_DATA, 41);
		let custom_impl_inst_num = self.0.len();
		let selector = (custom_impl_num + custom_impl_inst_num) % 3;
		if selector == 0{
			panic!("INTENTIONAL PANIC!");
		}
		let GLOBAL_DATA = match selector{
			1 => global_data.first_half,
			_ => global_data.second_half,
		};
		let t_9 = _to_usize(GLOBAL_DATA, 49);
		return t_9;
	}
}

fn main() {
	fuzz_nohook!(|data: &[u8]| {
		if data.len() < 420 { return; }
		set_global_data(data);
		let global_data = get_global_data();
		let first = global_data.first_half;
		let second = global_data.second_half;

		let mut vec_ct1: Vec<CustomType1> = Vec::new();
		let vlen = (_to_u8(first, 2) % 65) as usize;
		let mut base = 3usize;
		for i in 0..vlen {
			let l = (_to_u8(first, (base + i) % (first.len() - 32)) % 17) as usize;
			let start = (base + i * 3) % (first.len() - 32);
			let end = start + l;
			let s = _to_str(first, start, end);
			vec_ct1.push(CustomType1(String::from(s)));
		}
		let bx_ct1 = vec_ct1.clone().into_boxed_slice();

		let sel = _to_u8(first, 0) % 4;
		let mut t_main: TooDee<CustomType1>;
		match sel {
			0 => {
				let cap = _to_usize(first, 8);
				t_main = TooDee::with_capacity(cap);
			}
			1 => {
				let c = _to_usize(first, 16);
				let r = _to_usize(first, 24);
				t_main = TooDee::from_vec(c, r, vec_ct1.clone());
			}
			2 => {
				let c = _to_usize(first, 32);
				let r = _to_usize(first, 40);
				t_main = TooDee::from_box(c, r, bx_ct1.clone());
			}
			_ => {
				let c = _to_usize(first, 48);
				let r = _to_usize(first, 56);
				let mut tmp = TooDee::from_vec(c, r, vec_ct1.clone());
				let v = tmp.view((_to_usize(first, 64), _to_usize(first, 72)), (_to_usize(first, 80), _to_usize(first, 88)));
				t_main = TooDee::from(v);
			}
		}

		let pre_n = (_to_u8(second, 0) % 3) as usize;
		for k in 0..pre_n {
			let sl = (_to_u8(second, 1 + k) % 17) as usize;
			let st = (2 + k * 5) % (second.len() - 32);
			let en = st + sl;
			let s = _to_str(second, st, en);
			let it = CustomType0(String::from(s));
			t_main.push_row(it);
		}

		{
			let mut rows_it = t_main.rows();
			if let Some(r0) = rows_it.next() {
				let _l = r0.len();
				println!("{}", _l);
				if _l > 0 {
					let c0 = &r0[0];
					println!("{:?}", c0);
				}
			}
		}

		let idx_ins1 = _to_usize(second, 8);
		let sl_a = (_to_u8(second, 9) % 17) as usize;
		let st_a = (10) % (second.len() - 32);
		let en_a = st_a + sl_a;
		let s_a = _to_str(second, st_a, en_a);
		let it_a = CustomType0(String::from(s_a));
		t_main.insert_row(idx_ins1, it_a);

		let mut ops = (_to_u8(first, 96) % 10) as usize;
		while ops > 0 {
			let kind = _to_u8(second, 100 + ops) % 8;
			match kind {
				0 => {
					let cidx = _to_usize(second, 16);
					{
						let c = t_main.col(cidx);
						if let Some(lastv) = c.last() {
							println!("{:?}", lastv);
						}
					}
				}
				1 => {
					let cidxm = _to_usize(second, 24);
					let nthn = _to_usize(second, 32);
					let mut cm = t_main.col_mut(cidxm);
					let _ = cm.nth(nthn);
					if let Some(lb) = cm.last() {
						println!("{:?}", lb);
					}
				}
				2 => {
					let rc = _to_usize(second, 40);
					let mut dr = t_main.remove_col(rc);
					let _ = dr.next();
					let _ = dr.next_back();
				}
				3 => {
					if let Some(mut dc) = t_main.pop_col() {
						let _ = dc.next();
						let _ = dc.next_back();
					}
				}
				4 => {
					let sv = t_main.view((_to_usize(first, 104), _to_usize(first, 112)), (_to_usize(first, 120), _to_usize(first, 128)));
					{
						let mut rsv = sv.rows();
						let _ = rsv.next_back();
						if let Some(rr) = rsv.last() {
							println!("{}", rr.len());
						}
					}
					let cr = (_to_usize(first, 136), _to_usize(first, 144));
					let _ = &sv[cr];
				}
				5 => {
					let mut vm = t_main.view_mut((_to_usize(second, 48), _to_usize(second, 56)), (_to_usize(second, 64), _to_usize(second, 72)));
					{
						let mut rm = vm.rows_mut();
						let _ = rm.next();
						let _ = rm.nth_back(_to_usize(second, 80));
					}
					vm.swap_rows(_to_usize(second, 88), _to_usize(second, 96));
					let vv: TooDeeView<_> = vm.into();
					let _ = vv.rows().nth(_to_usize(second, 104));
				}
				6 => {
					let rix = _to_usize(first, 152);
					let _row_ref = &t_main[rix];
					println!("{}", _row_ref.len());
					let cx = (_to_usize(first, 160), _to_usize(first, 168));
					let _ = &t_main[cx];
				}
				_ => {
					let idx_ins2 = _to_usize(first, 176);
					let sl_b = (_to_u8(first, 184) % 17) as usize;
					let st_b = (188) % (first.len() - 32);
					let en_b = st_b + sl_b;
					let s_b = _to_str(first, st_b, en_b);
					let it_b = CustomType0(String::from(s_b));
					t_main.insert_row(idx_ins2, it_b);
				}
			}
			ops -= 1;
		}

		let sl_c = (_to_u8(first, 192) % 17) as usize;
		let st_c = (196) % (first.len() - 32);
		let en_c = st_c + sl_c;
		let s_c = _to_str(first, st_c, en_c);
		let it_c = CustomType0(String::from(s_c));
		let idx_final = _to_usize(second, 192);
		t_main.insert_row(idx_final, it_c);
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