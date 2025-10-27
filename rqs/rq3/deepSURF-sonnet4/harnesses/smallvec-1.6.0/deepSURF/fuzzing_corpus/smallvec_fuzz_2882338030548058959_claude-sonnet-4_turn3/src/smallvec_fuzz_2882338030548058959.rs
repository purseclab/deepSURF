#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

#[derive(Debug)]
struct CustomType1(String);

fn _custom_fn0() -> CustomType1 {
	let global_data = get_global_data();
	let GLOBAL_DATA = global_data.first_half;
	let t_10 = _to_u8(GLOBAL_DATA, 42);
	if t_10 % 2 == 0{
		panic!("INTENTIONAL PANIC!");
	}
	let mut t_11 = _to_u8(GLOBAL_DATA, 43) % 17;
	let t_12 = _to_str(GLOBAL_DATA, 44, 44 + t_11 as usize);
	let t_13 = String::from(t_12);
	let t_14 = CustomType1(t_13);
	return t_14;
}

fn _custom_fn1() -> String {
	let global_data = get_global_data();
	let GLOBAL_DATA = global_data.first_half;
	let t_20 = _to_u8(GLOBAL_DATA, 80);
	if t_20 % 3 == 0{
		panic!("INTENTIONAL PANIC!");
	}
	let mut t_21 = _to_u8(GLOBAL_DATA, 81) % 15;
	let t_22 = _to_str(GLOBAL_DATA, 82, 82 + t_21 as usize);
	String::from(t_22)
}

fn main (){
	fuzz_nohook!(|data: &[u8]| {
		if data.len() < 300 {return;}
		set_global_data(data);
		let global_data = get_global_data();
		let GLOBAL_DATA = global_data.first_half;
		
		let num_operations = _to_u8(GLOBAL_DATA, 0) % 20 + 5;
		
		for i in 0..num_operations {
			let op_choice = _to_u8(GLOBAL_DATA, 1 + i as usize * 10) % 8;
			
			match op_choice {
				0 => {
					let constructor_choice = _to_u8(GLOBAL_DATA, 10 + i as usize * 10) % 4;
					let mut sv: SmallVec<[String; 15]> = match constructor_choice {
						0 => SmallVec::new(),
						1 => {
							let cap = _to_usize(GLOBAL_DATA, 11 + i as usize * 10);
							SmallVec::with_capacity(cap)
						},
						2 => {
							let slice_data = vec![_custom_fn1(), _custom_fn1(), _custom_fn1()];
							SmallVec::from_vec(slice_data)
						},
						_ => {
							let elem = _custom_fn1();
							let count = _to_usize(GLOBAL_DATA, 12 + i as usize * 10) % 65;
							SmallVec::from_elem(elem, count)
						}
					};
					
					sv.push(_custom_fn1());
					sv.push(_custom_fn1());
					
					let new_len = _to_usize(GLOBAL_DATA, 13 + i as usize * 10);
					sv.resize_with(new_len, || _custom_fn1());
					
					let slice_ref = sv.as_slice();
					println!("{:?}", slice_ref);
					
					let mut_slice_ref = sv.as_mut_slice();
					println!("{:?}", mut_slice_ref);
					
					if !sv.is_empty() {
						let idx = _to_usize(GLOBAL_DATA, 14 + i as usize * 10) % sv.len();
						let item_ref = &sv[idx];
						println!("{:?}", item_ref);
						
						let item_mut_ref = &mut sv[idx];
						println!("{:?}", item_mut_ref);
					}
					
					sv.reserve(_to_usize(GLOBAL_DATA, 15 + i as usize * 10));
					sv.shrink_to_fit();
					sv.clear();
				},
				1 => {
					let mut sv: SmallVec<[CustomType1; 20]> = SmallVec::new();
					
					for _ in 0..5 {
						sv.push(_custom_fn0());
					}
					
					let new_len = _to_usize(GLOBAL_DATA, 20 + i as usize * 10);
					sv.resize_with(new_len, _custom_fn0);
					
					if !sv.is_empty() {
						let pop_result = sv.pop();
						if let Some(item) = pop_result {
							println!("{:?}", item.0);
						}
					}
					
					let len = sv.len();
					println!("{}", len);
					
					let cap = sv.capacity();
					println!("{}", cap);
					
					sv.truncate(_to_usize(GLOBAL_DATA, 21 + i as usize * 10));
				},
				2 => {
					let constructor_choice = _to_u8(GLOBAL_DATA, 30 + i as usize * 10) % 3;
					let mut sv: SmallVec<[u32; 32]> = match constructor_choice {
						0 => SmallVec::new(),
						1 => {
							let vec_data = vec![1u32, 2u32, 3u32, 4u32];
							SmallVec::from_vec(vec_data)
						},
						_ => {
							let slice_data = &[10u32, 20u32, 30u32];
							SmallVec::from_slice(slice_data)
						}
					};
					
					sv.extend_from_slice(&[100u32, 200u32, 300u32]);
					
					let new_len = _to_usize(GLOBAL_DATA, 31 + i as usize * 10);
					sv.resize_with(new_len, || _to_u32(GLOBAL_DATA, 32 + i as usize * 10));
					
					if sv.len() > 2 {
						let drain_start = _to_usize(GLOBAL_DATA, 33 + i as usize * 10) % (sv.len() - 1);
						let drain_end = drain_start + 1;
						let mut drain_iter = sv.drain(drain_start..drain_end);
						while let Some(item) = drain_iter.next() {
							println!("{}", item);
						}
					}
					
					sv.reserve_exact(_to_usize(GLOBAL_DATA, 34 + i as usize * 10));
				},
				3 => {
					let mut sv1: SmallVec<[i64; 16]> = SmallVec::new();
					let mut sv2: SmallVec<[i64; 16]> = SmallVec::new();
					
					sv1.push(_to_i64(GLOBAL_DATA, 40 + i as usize * 10));
					sv2.push(_to_i64(GLOBAL_DATA, 48 + i as usize * 10));
					
					let new_len = _to_usize(GLOBAL_DATA, 41 + i as usize * 10);
					sv1.resize_with(new_len, || _to_i64(GLOBAL_DATA, 42 + i as usize * 10));
					
					sv1.append(&mut sv2);
					
					let cmp_result = sv1.cmp(&sv2);
					println!("{:?}", cmp_result);
					
					let partial_cmp_result = sv1.partial_cmp(&sv2);
					println!("{:?}", partial_cmp_result);
					
					let eq_result = sv1.eq(&sv2);
					println!("{}", eq_result);
				},
				4 => {
					let mut sv: SmallVec<[f64; 12]> = SmallVec::new();
					
					sv.push(_to_f64(GLOBAL_DATA, 50 + i as usize * 10));
					sv.push(_to_f64(GLOBAL_DATA, 58 + i as usize * 10));
					
					let new_len = _to_usize(GLOBAL_DATA, 51 + i as usize * 10);
					sv.resize_with(new_len, || _to_f64(GLOBAL_DATA, 52 + i as usize * 10));
					
					if !sv.is_empty() {
						let insert_idx = _to_usize(GLOBAL_DATA, 53 + i as usize * 10) % (sv.len() + 1);
						sv.insert(insert_idx, _to_f64(GLOBAL_DATA, 54 + i as usize * 10));
					}
					
					let into_iter = sv.into_iter();
					for item in into_iter {
						println!("{}", item);
					}
				},
				5 => {
					let mut sv: SmallVec<[bool; 25]> = SmallVec::new();
					
					for j in 0..3 {
						sv.push(_to_bool(GLOBAL_DATA, 60 + i as usize * 10 + j));
					}
					
					let new_len = _to_usize(GLOBAL_DATA, 61 + i as usize * 10);
					sv.resize_with(new_len, || _to_bool(GLOBAL_DATA, 62 + i as usize * 10));
					
					if sv.len() > 1 {
						let remove_idx = _to_usize(GLOBAL_DATA, 63 + i as usize * 10) % sv.len();
						let removed = sv.remove(remove_idx);
						println!("{}", removed);
					}
					
					sv.dedup();
					
					sv.retain(|x| *x);
				},
				6 => {
					let mut sv: SmallVec<[char; 30]> = SmallVec::new();
					
					sv.push(_to_char(GLOBAL_DATA, 70 + i as usize * 10));
					sv.push(_to_char(GLOBAL_DATA, 74 + i as usize * 10));
					
					let new_len = _to_usize(GLOBAL_DATA, 71 + i as usize * 10);
					sv.resize_with(new_len, || _to_char(GLOBAL_DATA, 72 + i as usize * 10));
					
					if sv.len() > 1 {
						let swap_idx = _to_usize(GLOBAL_DATA, 73 + i as usize * 10) % sv.len();
						let swapped = sv.swap_remove(swap_idx);
						println!("{}", swapped);
					}
					
					let cloned_sv = sv.clone();
					println!("{:?}", cloned_sv);
				},
				_ => {
					let constructor_choice = _to_u8(GLOBAL_DATA, 90 + i as usize * 10) % 2;
					let mut sv: SmallVec<[usize; 32]> = match constructor_choice {
						0 => {
							let iter_data = vec![1usize, 2usize, 3usize].into_iter();
							SmallVec::from_iter(iter_data)
						},
						_ => {
							let slice_data = &[10usize, 20usize, 30usize];
							slice_data.to_smallvec()
						}
					};
					
					let new_len = _to_usize(GLOBAL_DATA, 91 + i as usize * 10);
					sv.resize_with(new_len, || _to_usize(GLOBAL_DATA, 92 + i as usize * 10));
					
					let grow_size = _to_usize(GLOBAL_DATA, 93 + i as usize * 10);
					sv.grow(grow_size);
					
					let as_ptr = sv.as_ptr();
					println!("{:?}", as_ptr);
					
					let as_mut_ptr = sv.as_mut_ptr();
					println!("{:?}", as_mut_ptr);
					
					let spilled = sv.spilled();
					println!("{}", spilled);
					
					let vec = sv.into_vec();
					println!("{:?}", vec);
				}
			}
		}
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