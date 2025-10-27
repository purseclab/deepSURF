#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

fn _gen_byte0() -> u8 {
	let global_data = get_global_data();
	let a = global_data.first_half;
	let b0 = _to_u8(a, 0);
	if b0 % 2 == 0 {
		panic!("INTENTIONAL PANIC!");
	}
	let b1 = _to_u8(a, 1);
	let b2 = _to_u8(a, 2);
	b1 ^ b2
}

fn _gen_byte1() -> u8 {
	let global_data = get_global_data();
	let a = global_data.second_half;
	let b0 = _to_u8(a, 0);
	if b0 % 2 == 0 {
		panic!("INTENTIONAL PANIC!");
	}
	let b1 = _to_u8(a, 1);
	let b2 = _to_u8(a, 3);
	b1.wrapping_add(b2)
}

fn make_buf32() -> [u8; 32] {
	let global_data = get_global_data();
	let a = global_data.first_half;
	let mut buf = [0u8; 32];
	let mut i = 0usize;
	while i < 32 {
		buf[i] = _to_u8(a, 32 + i);
		i += 1;
	}
	buf
}

fn bounded_vec_from_half(half: &[u8], n: usize) -> Vec<u8> {
	let target_len = n % 65;
	let mut out = Vec::new();
	let mut i = 0usize;
	while i < target_len {
		out.push(half[i % half.len()]);
		i += 1;
	}
	out
}

fn main() {
	fuzz_nohook!(|data: &[u8]| {
		if data.len() < 160 { return; }
		set_global_data(data);
		let global_data = get_global_data();
		let first = global_data.first_half;
		let second = global_data.second_half;

		let selector = _to_u8(first, 3);
		let buf32 = make_buf32();
		let mut sv: SmallVec<[u8; 32]> = match selector % 7 {
			0 => SmallVec::<[u8; 32]>::new(),
			1 => SmallVec::<[u8; 32]>::with_capacity(_to_usize(first, 10)),
			2 => SmallVec::<[u8; 32]>::from_elem(_to_u8(first, 20), _to_usize(first, 21)),
			3 => {
				let tmp = bounded_vec_from_half(first, _to_u8(first, 31) as usize);
				SmallVec::<[u8; 32]>::from_slice(&tmp)
			}
			4 => SmallVec::<[u8; 32]>::from_buf_and_len(buf32, _to_usize(first, 62)),
			5 => {
				let tmp = bounded_vec_from_half(second, _to_u8(first, 64) as usize);
				SmallVec::<[u8; 32]>::from_vec(tmp)
			}
			_ => SmallVec::<[u8; 32]>::from_buf(buf32),
		};

		println!("{:?}", sv.capacity());
		println!("{:?}", sv.len());
		println!("{:?}", sv.is_empty());
		{
			let s = sv.as_slice();
			println!("{:?}", s);
		}
		{
			let m = sv.as_mut_slice();
			println!("{:?}", m);
		}

		let mut other = SmallVec::<[u8; 32]>::from_elem(_to_u8(second, 10), _to_usize(second, 12));
		let ocmp = SmallVec::<[u8; 32]>::partial_cmp(&sv, &other);
		if let Some(ord) = ocmp {
			println!("{:?}", ord);
		}
		let ord2 = SmallVec::<[u8; 32]>::cmp(&sv, &other);
		println!("{:?}", ord2);

		let new_len0 = _to_usize(first, 70);
		let mut g0 = _gen_byte0;
		sv.resize_with(new_len0, g0);

		sv.grow(_to_usize(second, 18));
		sv.reserve(_to_usize(second, 54));
		let _ = sv.try_reserve(_to_usize(second, 62));
		sv.reserve_exact(_to_usize(second, 56));
		let _ = sv.try_reserve_exact(_to_usize(second, 60));

		let ops = (_to_u8(first, 8) % 10) as usize;
		let mut i = 0usize;
		while i < ops {
			let op = _to_u8(first, 9 + i);
			match op % 12 {
				0 => {
					let v = _to_u8(second, 14 + i);
					sv.push(v);
				}
				1 => {
					let idx = _to_usize(second, 30);
					let val = _to_u8(second, 16 + i);
					sv.insert(idx, val);
				}
				2 => {
					let _ = sv.pop();
				}
				3 => {
					let idx = _to_usize(second, 38);
					let _ = sv.remove(idx);
				}
				4 => {
					let idx = _to_usize(second, 46);
					let _ = sv.swap_remove(idx);
				}
				5 => {
					let tmp = bounded_vec_from_half(second, _to_u8(second, 8) as usize);
					sv.extend_from_slice(&tmp);
				}
				6 => {
					let mut f = |a: &mut u8, b: &mut u8| {
						println!("{:?}", (*a, *b));
						let t = _to_u8(first, 0);
						if t % 2 == 0 {
							panic!("INTENTIONAL PANIC!");
						}
						*a == *b
					};
					sv.dedup_by(&mut f);
				}
				7 => {
					let mut f = |x: &mut u8| {
						println!("{:?}", *x);
						let t = _to_u8(second, 2);
						if t % 2 == 0 {
							panic!("INTENTIONAL PANIC!");
						}
						*x
					};
					sv.dedup_by_key(&mut f);
				}
				8 => {
					let mut f = |x: &mut u8| {
						println!("{:?}", *x);
						let t = _to_u8(first, 4);
						if t % 2 == 0 { true } else { false }
					};
					sv.retain(&mut f);
				}
				9 => {
					let mut g = if _to_bool(first, 71) { _gen_byte0 } else { _gen_byte1 };
					let nl = _to_usize(second, 70);
					sv.resize_with(nl, g);
				}
				10 => {
					let mut dr = sv.drain(0.._to_usize(second, 22));
					if let Some(x) = dr.next() {
						println!("{:?}", x);
					}
					if let Some(y) = dr.next_back() {
						println!("{:?}", y);
					}
				}
				_ => {
					let slice = bounded_vec_from_half(first, _to_u8(first, 15) as usize);
					let idx = _to_usize(second, 34);
					sv.insert_from_slice(idx, &slice);
				}
			}
			i += 1;
		}

		{
			let d = sv.deref();
			println!("{:?}", d);
		}
		{
			let dm = sv.deref_mut();
			println!("{:?}", dm);
		}
		if sv.len() > 0 {
			let r = &sv[0];
			println!("{:?}", *r);
		}
		{
			let s2 = sv.as_slice();
			println!("{:?}", s2);
		}
		{
			let m2 = sv.as_mut_slice();
			println!("{:?}", m2);
		}

		let mut to_append = SmallVec::<[u8; 32]>::from_elem(_to_u8(first, 28), _to_usize(first, 29));
		sv.append(&mut to_append);

		let clone_for_vec = sv.clone();
		let v3 = clone_for_vec.into_vec();
		println!("{:?}", v3.len());

		sv.truncate(_to_usize(second, 26));
		sv.shrink_to_fit();
		sv.clear();

		let mut g1 = _gen_byte1;
		sv.resize_with(_to_usize(second, 72), g1);

		println!("{:?}", sv.len());
		println!("{:?}", sv.capacity());
		println!("{:p}", sv.as_ptr());
		let b = sv.into_boxed_slice();
		println!("{:?}", b.len());
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