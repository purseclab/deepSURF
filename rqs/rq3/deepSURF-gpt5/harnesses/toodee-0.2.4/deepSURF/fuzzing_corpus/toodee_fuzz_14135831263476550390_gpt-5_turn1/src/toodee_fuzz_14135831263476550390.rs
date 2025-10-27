#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use toodee::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

#[derive(Debug, Clone, Default)]
struct CustomType1(String);
struct CustomType2(String);
struct CustomType0(String);

impl core::iter::ExactSizeIterator for CustomType2 {
	fn len(&self) -> usize {
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
		let t_8 = _to_usize(GLOBAL_DATA, 41);
		return t_8;
	}
}

impl core::iter::Iterator for CustomType2 {
	type Item = CustomType1;
	
	fn next(&mut self) -> Option<Self::Item> {
		let global_data = get_global_data();
		let GLOBAL_DATA = global_data.first_half;
		let custom_impl_num = _to_usize(GLOBAL_DATA, 8);
		let custom_impl_inst_num = self.0.len();
		let selector = (custom_impl_num + custom_impl_inst_num) % 3;
		if selector == 0{
			panic!("INTENTIONAL PANIC!");
		}
		let GLOBAL_DATA = match selector{
				1 => global_data.first_half,
				_ => global_data.second_half,
		};
		let mut t_3 = _to_u8(GLOBAL_DATA, 16) % 17;
		let t_4 = _to_str(GLOBAL_DATA, 17, 17 + t_3 as usize);
		let t_5 = String::from(t_4);
		let t_6 = CustomType1(t_5);
		let t_7 = Some(t_6);
		return t_7;
	}
}

impl core::iter::DoubleEndedIterator for CustomType2 {
	fn next_back(&mut self) -> Option<Self::Item> {
		let global_data = get_global_data();
		let GLOBAL_DATA = global_data.second_half;
		let custom_impl_num = _to_usize(GLOBAL_DATA, 60);
		let custom_impl_inst_num = self.0.len();
		let selector = (custom_impl_num + custom_impl_inst_num) % 3;
		if selector == 0 {
			panic!("INTENTIONAL PANIC!");
		}
		let GLOBAL_DATA = match selector {
			1 => global_data.first_half,
			_ => global_data.second_half,
		};
		let mut t_a = _to_u8(GLOBAL_DATA, 61) % 17;
		let t_b = _to_str(GLOBAL_DATA, 62, 62 + t_a as usize);
		let s = String::from(t_b);
		let item = CustomType1(s);
		Some(item)
	}
}

impl core::iter::IntoIterator for CustomType0 {
	type Item = CustomType1;
	type IntoIter = CustomType2;
	
	fn into_iter(self) -> Self::IntoIter {
		let global_data = get_global_data();
		let GLOBAL_DATA = global_data.first_half;
		let custom_impl_num = _to_usize(GLOBAL_DATA, 49);
		let custom_impl_inst_num = self.0.len();
		let selector = (custom_impl_num + custom_impl_inst_num) % 3;
		if selector == 0{
			panic!("INTENTIONAL PANIC!");
		}
		let GLOBAL_DATA = match selector{
				1 => global_data.first_half,
				_ => global_data.second_half,
		};
		let mut t_9 = _to_u8(GLOBAL_DATA, 57) % 17;
		let t_10 = _to_str(GLOBAL_DATA, 58, 58 + t_9 as usize);
		let t_11 = String::from(t_10);
		let t_12 = CustomType2(t_11);
		return t_12;
	}
}

fn mk_iterable(offset: usize) -> CustomType0 {
	let global_data = get_global_data();
	let GLOBAL_DATA = global_data.first_half;
	let l = _to_u8(GLOBAL_DATA, offset) % 17;
	let s = _to_str(GLOBAL_DATA, offset + 1, offset + 1 + l as usize);
	CustomType0(String::from(s))
}

fn mk_value(offset: usize) -> CustomType1 {
	let global_data = get_global_data();
	let GLOBAL_DATA = global_data.second_half;
	let l = _to_u8(GLOBAL_DATA, offset) % 17;
	let s = _to_str(GLOBAL_DATA, offset + 1, offset + 1 + l as usize);
	CustomType1(String::from(s))
}

fn mk_vec(n: usize, base: usize) -> Vec<CustomType1> {
	let mut v = Vec::new();
	for i in 0..(n % 65) {
		let off = base + (i % 30);
		v.push(mk_value(off));
	}
	v
}

fn deref_refs(td: &TooDee<CustomType1>) {
	let global_data = get_global_data();
	let GLOBAL_DATA = global_data.first_half;
	let r = _to_usize(GLOBAL_DATA, 200);
	let c = _to_usize(GLOBAL_DATA, 204);
	let row: &[CustomType1] = &td[r];
	let _ = println!("{:?}", row.len());
	let cell: &CustomType1 = &td[(c, r)];
	let _ = println!("{:?}", cell);
	let col_idx = _to_usize(GLOBAL_DATA, 208);
	let mut col_it = td.col(col_idx);
	let _ = col_it.nth(_to_usize(GLOBAL_DATA, 212)).map(|x| println!("{:?}", x));
	let _ = col_it.last().map(|x| println!("{:?}", x));
	let mut rows_it = td.rows();
	let _ = rows_it.nth(_to_usize(GLOBAL_DATA, 216)).map(|x| println!("{:?}", x.len()));
	let _ = rows_it.last().map(|x| println!("{:?}", x.len()));
}

fn view_ops(td: &TooDee<CustomType1>) {
	let global_data = get_global_data();
	let GLOBAL_DATA = global_data.second_half;
	let s0 = _to_usize(GLOBAL_DATA, 100);
	let s1 = _to_usize(GLOBAL_DATA, 104);
	let e0 = _to_usize(GLOBAL_DATA, 108);
	let e1 = _to_usize(GLOBAL_DATA, 112);
	let v = td.view((s0, s1), (e0, e1));
	let _ = println!("{:?}", v.num_cols());
	let _ = println!("{:?}", v.num_rows());
	let col = _to_usize(GLOBAL_DATA, 116);
	let mut c = v.col(col);
	let _ = c.nth(_to_usize(GLOBAL_DATA, 120)).map(|x| println!("{:?}", x));
	let mut rs = v.rows();
	let _ = rs.next().map(|x| println!("{:?}", x.len()));
	let td2 = TooDee::from(v);
	let it = mk_iterable(124);
	let mut t2 = td2;
	t2.push_row(it);
	let _ = println!("{:?}", t2.num_rows());
}

fn view_mut_ops(td: &mut TooDee<CustomType1>) {
	let global_data = get_global_data();
	let GLOBAL_DATA = global_data.first_half;
	let s0 = _to_usize(GLOBAL_DATA, 140);
	let s1 = _to_usize(GLOBAL_DATA, 144);
	let e0 = _to_usize(GLOBAL_DATA, 148);
	let e1 = _to_usize(GLOBAL_DATA, 152);
	let mut vm = td.view_mut((s0, s1), (e0, e1));
	let _ = println!("{:?}", vm.num_cols());
	let _ = println!("{:?}", vm.num_rows());
	let mut rms = vm.rows_mut();
	let _ = rms.next().map(|x| println!("{:?}", x.len()));
	let col = _to_usize(GLOBAL_DATA, 156);
	let mut cm = vm.col_mut(col);
	let _ = cm.nth(_to_usize(GLOBAL_DATA, 160)).map(|x| println!("{:?}", x));
	let ridx = _to_usize(GLOBAL_DATA, 164);
	let row_ref: &mut [CustomType1] = &mut vm[ridx];
	let _ = println!("{:?}", row_ref.len());
	let vm2 = vm.view((s0, s1), (e0, e1));
	let _ = println!("{:?}", vm2.num_cols());
}

fn col_drain_ops(td: &mut TooDee<CustomType1>) {
	let global_data = get_global_data();
	let GLOBAL_DATA = global_data.second_half;
	let do_pop = _to_bool(GLOBAL_DATA, 170);
	if do_pop {
		if let Some(mut dc) = td.pop_col() {
			let _ = dc.next().map(|x| println!("{:?}", x));
			let _ = dc.next_back().map(|x| println!("{:?}", x));
		}
	} else {
		let idx = _to_usize(GLOBAL_DATA, 171);
		let mut dc = td.remove_col(idx);
		let _ = dc.next().map(|x| println!("{:?}", x));
		let _ = dc.next_back().map(|x| println!("{:?}", x));
	}
}

fn main (){
	fuzz_nohook!(|data: &[u8]| {
		if data.len() < 520 {return;}
		set_global_data(data);
		let global_data = get_global_data();
		let GLOBAL_DATA = global_data.first_half;
		let sel = _to_u8(GLOBAL_DATA, 0) % 5;
		let mut td: TooDee<CustomType1> = match sel {
			0 => {
				let cap = _to_usize(GLOBAL_DATA, 1);
				TooDee::<CustomType1>::with_capacity(cap)
			},
			1 => {
				let nc = _to_usize(GLOBAL_DATA, 9);
				let nr = _to_usize(GLOBAL_DATA, 17);
				TooDee::<CustomType1>::new(nc, nr)
			},
			2 => {
				let nc = _to_usize(GLOBAL_DATA, 25);
				let nr = _to_usize(GLOBAL_DATA, 33);
				let init = mk_value(41);
				TooDee::<CustomType1>::init(nc, nr, init)
			},
			3 => {
				let n = _to_u8(GLOBAL_DATA, 50) as usize;
				let v = mk_vec(n as usize, 52);
				let nc = _to_usize(GLOBAL_DATA, 51);
				let nr = _to_usize(GLOBAL_DATA, 59);
				TooDee::<CustomType1>::from_vec(nc, nr, v)
			},
			_ => {
				let n = _to_u8(GLOBAL_DATA, 68) as usize;
				let v = mk_vec(n as usize, 70).into_boxed_slice();
				let nc = _to_usize(GLOBAL_DATA, 67);
				let nr = _to_usize(GLOBAL_DATA, 75);
				TooDee::<CustomType1>::from_box(nc, nr, v)
			},
		};
		let mut ops = (_to_u8(GLOBAL_DATA, 83) % 11) as usize;
		let mut i = 0usize;
		while i < ops {
			let op = _to_u8(GLOBAL_DATA, 84 + i) % 8;
			match op {
				0 => {
					let it = mk_iterable(85 + i);
					td.push_row(it);
					let _ = println!("{:?}", td.num_rows());
				},
				1 => {
					let idx = _to_usize(GLOBAL_DATA, 100 + i);
					let it = mk_iterable(101 + i);
					td.insert_row(idx, it);
				},
				2 => {
					let r1 = _to_usize(GLOBAL_DATA, 110 + i);
					let r2 = _to_usize(GLOBAL_DATA, 118 + i);
					td.swap_rows(r1, r2);
				},
				3 => {
					view_ops(&td);
				},
				4 => {
					view_mut_ops(&mut td);
				},
				5 => {
					let col_idx = _to_usize(GLOBAL_DATA, 180 + i);
					let mut cm = td.col_mut(col_idx);
					let _ = cm.next().map(|x| println!("{:?}", x));
					let _ = cm.nth_back(_to_usize(GLOBAL_DATA, 181 + i)).map(|x| println!("{:?}", x));
				},
				6 => {
					let mut rs = td.rows_mut();
					let _ = rs.next().map(|x| println!("{:?}", x.len()));
					let _ = rs.nth(_to_usize(GLOBAL_DATA, 182 + i)).map(|x| println!("{:?}", x.len()));
					let _ = rs.last().map(|x| println!("{:?}", x.len()));
				},
				_ => {
					col_drain_ops(&mut td);
				},
			}
			i += 1;
		}
		let it_row = mk_iterable(74);
		td.push_row(it_row);
		let it_col = mk_iterable(90);
		td.push_col(it_col);
		deref_refs(&td);
		let idx = _to_usize(GLOBAL_DATA, 220);
		let row_mut: &mut [CustomType1] = &mut td[idx];
		let _ = println!("{:?}", row_mut.len());
		let idxc = _to_usize(GLOBAL_DATA, 224);
		let ridx = _to_usize(GLOBAL_DATA, 228);
		let cell_mut: &mut CustomType1 = &mut td[(idxc, ridx)];
		let _ = println!("{:?}", cell_mut);
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