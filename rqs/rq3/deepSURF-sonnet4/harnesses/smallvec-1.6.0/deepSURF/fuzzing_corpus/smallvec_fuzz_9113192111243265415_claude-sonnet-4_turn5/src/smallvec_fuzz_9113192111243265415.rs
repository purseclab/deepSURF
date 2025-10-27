#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd)]
struct CustomType1(usize);

fn main (){
	fuzz_nohook!(|data: &[u8]| {
		if data.len() < 2500 {return;}
		set_global_data(data);
		let global_data = get_global_data();
		let GLOBAL_DATA = global_data.first_half;
		
		let operation_count = _to_u8(GLOBAL_DATA, 0) % 20;
		for _ in 0..operation_count {
			let op_selector = _to_u8(GLOBAL_DATA, 1) % 15;
			
			match op_selector {
				0 => {
					let mut vec_size = _to_u8(GLOBAL_DATA, 10) % 65;
					let mut vec_items = std::vec::Vec::with_capacity(64);
					for i in 0..vec_size {
						let item_value = _to_usize(GLOBAL_DATA, 20 + i as usize * 8);
						vec_items.push(CustomType1(item_value));
					}
					
					let constructor_choice = _to_u8(GLOBAL_DATA, 200) % 6;
					let sv1 = match constructor_choice {
						0 => smallvec::SmallVec::<[CustomType1; 16]>::new(),
						1 => {
							let capacity = _to_usize(GLOBAL_DATA, 210);
							smallvec::SmallVec::<[CustomType1; 16]>::with_capacity(capacity)
						},
						2 => smallvec::SmallVec::<[CustomType1; 16]>::from_vec(vec_items.clone()),
						3 => smallvec::SmallVec::<[CustomType1; 16]>::from_slice(&vec_items),
						4 => {
							let elem = CustomType1(_to_usize(GLOBAL_DATA, 220));
							let count = _to_usize(GLOBAL_DATA, 230);
							smallvec::SmallVec::<[CustomType1; 16]>::from_elem(elem, count)
						},
						_ => smallvec::SmallVec::<[CustomType1; 16]>::from_iter(vec_items.clone())
					};
					
					let mut vec_size2 = _to_u8(GLOBAL_DATA, 400) % 65;
					let mut vec_items2 = std::vec::Vec::with_capacity(64);
					for i in 0..vec_size2 {
						let item_value = _to_usize(GLOBAL_DATA, 450 + i as usize * 8);
						vec_items2.push(CustomType1(item_value));
					}
					
					let constructor_choice2 = _to_u8(GLOBAL_DATA, 700) % 6;
					let sv2 = match constructor_choice2 {
						0 => smallvec::SmallVec::<[CustomType1; 16]>::new(),
						1 => {
							let capacity = _to_usize(GLOBAL_DATA, 710);
							smallvec::SmallVec::<[CustomType1; 16]>::with_capacity(capacity)
						},
						2 => smallvec::SmallVec::<[CustomType1; 16]>::from_vec(vec_items2.clone()),
						3 => smallvec::SmallVec::<[CustomType1; 16]>::from_slice(&vec_items2),
						4 => {
							let elem = CustomType1(_to_usize(GLOBAL_DATA, 720));
							let count = _to_usize(GLOBAL_DATA, 730);
							smallvec::SmallVec::<[CustomType1; 16]>::from_elem(elem, count)
						},
						_ => smallvec::SmallVec::<[CustomType1; 16]>::from_iter(vec_items2.clone())
					};
					
					let sv1_ref = &sv1;
					let sv2_ref = &sv2;
					let result = sv1_ref.partial_cmp(sv2_ref);
					println!("{:?}", result);
					
					let cmp_result = sv1.cmp(&sv2);
					println!("{:?}", cmp_result);
					
					let eq_result = sv1.eq(&sv2);
					println!("{:?}", eq_result);
				},
				1 => {
					let mut sv = smallvec::SmallVec::<[CustomType1; 32]>::new();
					let push_count = _to_u8(GLOBAL_DATA, 100) % 20;
					for i in 0..push_count {
						let item = CustomType1(_to_usize(GLOBAL_DATA, 120 + i as usize * 8));
						sv.push(item);
					}
					
					let len_val = sv.len();
					println!("{}", len_val);
					
					let capacity_val = sv.capacity();
					println!("{}", capacity_val);
					
					if !sv.is_empty() {
						let pop_result = sv.pop();
						if let Some(val) = pop_result {
							println!("{}", val.0);
						}
					}
					
					let slice_ref = sv.as_slice();
					for item in slice_ref {
						println!("{}", item.0);
						break;
					}
				},
				2 => {
					let mut sv = smallvec::SmallVec::<[CustomType1; 12]>::new();
					let item = CustomType1(_to_usize(GLOBAL_DATA, 300));
					sv.push(item);
					
					let slice_ref = sv.as_slice();
					println!("{}", slice_ref.len());
					for item_ref in slice_ref {
						println!("{}", item_ref.0);
					}
					
					let mut_slice_ref = sv.as_mut_slice();
					println!("{}", mut_slice_ref.len());
					for item_mut_ref in mut_slice_ref {
						println!("{}", item_mut_ref.0);
					}
					
					let deref_result = sv.deref();
					println!("{}", deref_result.len());
					for deref_item in deref_result {
						println!("{}", deref_item.0);
					}
					
					let as_ref_result = sv.as_ref();
					println!("{}", as_ref_result.len());
					for as_ref_item in as_ref_result {
						println!("{}", as_ref_item.0);
					}
				},
				3 => {
					let mut sv1 = smallvec::SmallVec::<[CustomType1; 12]>::new();
					let mut sv2 = smallvec::SmallVec::<[CustomType1; 12]>::new();
					
					let item1 = CustomType1(_to_usize(GLOBAL_DATA, 500));
					let item2 = CustomType1(_to_usize(GLOBAL_DATA, 510));
					sv1.push(item1);
					sv2.push(item2);
					
					sv1.append(&mut sv2);
					println!("{}", sv1.len());
					
					sv1.extend_from_slice(&[item1, item2]);
					println!("{}", sv1.len());
				},
				4 => {
					let mut sv = smallvec::SmallVec::<[CustomType1; 20]>::new();
					let reserve_amount = _to_usize(GLOBAL_DATA, 600);
					sv.reserve(reserve_amount);
					
					let try_reserve_amount = _to_usize(GLOBAL_DATA, 610);
					let _ = sv.try_reserve(try_reserve_amount);
					
					sv.shrink_to_fit();
					
					let reserve_exact_amount = _to_usize(GLOBAL_DATA, 620);
					sv.reserve_exact(reserve_exact_amount);
					
					let try_reserve_exact_amount = _to_usize(GLOBAL_DATA, 630);
					let _ = sv.try_reserve_exact(try_reserve_exact_amount);
				},
				5 => {
					let mut sv = smallvec::SmallVec::<[CustomType1; 24]>::new();
					let item = CustomType1(_to_usize(GLOBAL_DATA, 800));
					sv.push(item);
					
					let index = _to_usize(GLOBAL_DATA, 810);
					let insert_item = CustomType1(_to_usize(GLOBAL_DATA, 820));
					sv.insert(index, insert_item);
					
					if !sv.is_empty() {
						let remove_index = _to_usize(GLOBAL_DATA, 830);
						let _ = sv.remove(remove_index);
					}
					
					if !sv.is_empty() {
						let swap_remove_index = _to_usize(GLOBAL_DATA, 840);
						let _ = sv.swap_remove(swap_remove_index);
					}
				},
				6 => {
					let mut sv = smallvec::SmallVec::<[CustomType1; 28]>::new();
					let new_len = _to_usize(GLOBAL_DATA, 900);
					let fill_value = CustomType1(_to_usize(GLOBAL_DATA, 910));
					sv.resize(new_len, fill_value);
					
					let truncate_len = _to_usize(GLOBAL_DATA, 920);
					sv.truncate(truncate_len);
					
					sv.clear();
				},
				7 => {
					let slice_data = vec![
						CustomType1(_to_usize(GLOBAL_DATA, 1000)),
						CustomType1(_to_usize(GLOBAL_DATA, 1010)),
						CustomType1(_to_usize(GLOBAL_DATA, 1020))
					];
					let sv = smallvec::SmallVec::<[CustomType1; 16]>::from_slice(&slice_data);
					
					let cloned_sv = sv.clone();
					println!("{}", cloned_sv.len());
					
					let vec_result = sv.into_vec();
					println!("{}", vec_result.len());
				},
				8 => {
					let mut sv1 = smallvec::SmallVec::<[CustomType1; 15]>::new();
					let mut sv2 = smallvec::SmallVec::<[CustomType1; 15]>::new();
					
					sv1.push(CustomType1(_to_usize(GLOBAL_DATA, 1100)));
					sv2.push(CustomType1(_to_usize(GLOBAL_DATA, 1110)));
					
					let eq_result = sv1.eq(&sv2);
					println!("{}", eq_result);
					
					let cmp_result = sv1.cmp(&sv2);
					println!("{:?}", cmp_result);
					
					let drain_range = 0..1;
					let mut drain_iter = sv1.drain(drain_range);
					if let Some(drained_item) = drain_iter.next() {
						println!("{}", drained_item.0);
					}
				},
				9 => {
					let item_count = _to_u8(GLOBAL_DATA, 1200) % 10;
					let items: Vec<CustomType1> = (0..item_count)
						.map(|i| CustomType1(_to_usize(GLOBAL_DATA, 1210 + i as usize * 8)))
						.collect();
					
					let sv = smallvec::SmallVec::<[CustomType1; 16]>::from_iter(items);
					
					let iter = sv.into_iter();
					for item in iter {
						println!("{}", item.0);
						break;
					}
				},
				10 => {
					let mut sv = smallvec::SmallVec::<[CustomType1; 14]>::new();
					let elem = CustomType1(_to_usize(GLOBAL_DATA, 1300));
					sv.push(elem);
					
					let index_value = _to_usize(GLOBAL_DATA, 1310);
					if !sv.is_empty() {
						let sv_len = sv.len();
						let indexed_ref = &sv[index_value % sv_len];
						println!("{}", indexed_ref.0);
					}
					
					if !sv.is_empty() {
						let sv_len = sv.len();
						let indexed_mut_ref = &mut sv[index_value % sv_len];
						println!("{}", indexed_mut_ref.0);
					}
				},
				11 => {
					let mut sv = smallvec::SmallVec::<[CustomType1; 18]>::new();
					let items_to_add = vec![
						CustomType1(_to_usize(GLOBAL_DATA, 1400)),
						CustomType1(_to_usize(GLOBAL_DATA, 1410)),
						CustomType1(_to_usize(GLOBAL_DATA, 1420))
					];
					sv.extend(items_to_add);
					
					let insert_index = _to_usize(GLOBAL_DATA, 1430);
					let items_to_insert = vec![
						CustomType1(_to_usize(GLOBAL_DATA, 1440)),
						CustomType1(_to_usize(GLOBAL_DATA, 1450))
					];
					sv.insert_many(insert_index, items_to_insert);
					
					let insert_from_slice_index = _to_usize(GLOBAL_DATA, 1460);
					let slice_to_insert = &[CustomType1(_to_usize(GLOBAL_DATA, 1470))];
					sv.insert_from_slice(insert_from_slice_index, slice_to_insert);
				},
				12 => {
					let mut sv = smallvec::SmallVec::<[CustomType1; 22]>::new();
					let items = vec![
						CustomType1(_to_usize(GLOBAL_DATA, 1500)),
						CustomType1(_to_usize(GLOBAL_DATA, 1501)),
						CustomType1(_to_usize(GLOBAL_DATA, 1502)),
						CustomType1(_to_usize(GLOBAL_DATA, 1503))
					];
					sv.extend(items);
					
					sv.dedup();
					
					let retain_condition = _to_u8(GLOBAL_DATA, 1510) % 2;
					sv.retain(|item| item.0 % 2 == retain_condition as usize);
					
					let spilled_status = sv.spilled();
					println!("{}", spilled_status);
				},
				13 => {
					let sv = smallvec::SmallVec::<[CustomType1; 26]>::new();
					let as_ptr_result = sv.as_ptr();
					println!("{:?}", as_ptr_result);
					
					let mut sv_mut = smallvec::SmallVec::<[CustomType1; 26]>::new();
					let as_mut_ptr_result = sv_mut.as_mut_ptr();
					println!("{:?}", as_mut_ptr_result);
					
					let into_boxed_result = sv.into_boxed_slice();
					println!("{}", into_boxed_result.len());
				},
				_ => {
					let items = vec![
						CustomType1(_to_usize(GLOBAL_DATA, 1600)),
						CustomType1(_to_usize(GLOBAL_DATA, 1610)),
						CustomType1(_to_usize(GLOBAL_DATA, 1620))
					];
					let sv = smallvec::SmallVec::<[CustomType1; 30]>::from_vec(items);
					
					let into_inner_result = sv.into_inner();
					match into_inner_result {
						Ok(array) => {
							println!("Into inner succeeded");
						},
						Err(vec_back) => {
							println!("Into inner failed, got vec back with len: {}", vec_back.len());
						}
					}
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