#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};
use std::cmp::Ordering;

struct CustomType2(String);
struct CustomType1(usize);
struct CustomType0(String);
struct CustomType3(String);

impl core::clone::Clone for CustomType1 {
	
	fn clone(&self) -> Self {
		let global_data = get_global_data();
		let GLOBAL_DATA = global_data.first_half;
		let custom_impl_num = _to_usize(GLOBAL_DATA, 10);
		let custom_impl_inst_num = self.0;
		let selector = (custom_impl_num + custom_impl_inst_num) % 3;
		if selector == 0{
			panic!("INTENTIONAL PANIC!");
		}
		let GLOBAL_DATA = match selector{
				1 => global_data.first_half,
				_ => global_data.second_half,
		};
		let t_4 = _to_usize(GLOBAL_DATA, 18);
		let t_5 = CustomType1(t_4);
		return t_5;
	}
}

impl core::marker::Copy for CustomType1 {
}

impl core::fmt::Debug for CustomType1 {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		write!(f, "CustomType1({})", self.0)
	}
}

impl PartialEq for CustomType1 {
	fn eq(&self, other: &Self) -> bool {
		self.0 == other.0
	}
}

impl Eq for CustomType1 {}

impl PartialOrd for CustomType1 {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl Ord for CustomType1 {
	fn cmp(&self, other: &Self) -> Ordering {
		self.0.cmp(&other.0)
	}
}

fn main (){
	fuzz_nohook!(|data: &[u8]| {
		if data.len() < 1500 {return;}
		set_global_data(data);
		let global_data = get_global_data();
		let GLOBAL_DATA = global_data.first_half;
		
		let num_operations = _to_u8(GLOBAL_DATA, 0) % 65;
		
		for operation_idx in 0..num_operations {
			let mut offset = 20 + operation_idx as usize * 15;
			let operation_type = _to_u8(GLOBAL_DATA, offset) % 20;
			
			match operation_type {
				0 => {
					let mut t_2 = _to_u8(GLOBAL_DATA, offset + 1) % 33;
					let mut smallvec_items = std::vec::Vec::with_capacity(32);
					for i in 0..t_2 {
						let item_value = _to_usize(GLOBAL_DATA, offset + 2 + i as usize);
						smallvec_items.push(CustomType1(item_value));
					}
					
					let slice_ref = &smallvec_items[..];
					let mut target_smallvec = match _to_u8(GLOBAL_DATA, offset + 14) % 5 {
						0 => SmallVec::<[CustomType1; 16]>::new(),
						1 => SmallVec::<[CustomType1; 16]>::with_capacity(_to_usize(GLOBAL_DATA, offset + 3)),
						2 => SmallVec::<[CustomType1; 16]>::from_vec(smallvec_items.clone()),
						3 => SmallVec::<[CustomType1; 16]>::from_slice(slice_ref),
						_ => SmallVec::<[CustomType1; 16]>::from_elem(CustomType1(_to_usize(GLOBAL_DATA, offset + 5)), _to_usize(GLOBAL_DATA, offset + 6) % 65),
					};
					
					println!("{:?}", target_smallvec.len());
					println!("{:?}", target_smallvec.capacity());
					println!("{:?}", target_smallvec.is_empty());
					println!("{:?}", target_smallvec.spilled());
					
					let index_value = _to_usize(GLOBAL_DATA, offset + 8);
					
					if !target_smallvec.is_empty() {
						let actual_index = index_value % target_smallvec.len();
						let result = &mut target_smallvec[actual_index];
						println!("{:?}", *result);
					}
				}
				1 => {
					let capacity_val = _to_usize(GLOBAL_DATA, offset + 1);
					let mut target_smallvec = SmallVec::<[CustomType1; 24]>::with_capacity(capacity_val);
					
					for push_idx in 0..(_to_u8(GLOBAL_DATA, offset + 9) % 20) {
						let item_val = _to_usize(GLOBAL_DATA, offset + 10 + push_idx as usize);
						target_smallvec.push(CustomType1(item_val));
					}
					
					println!("{:?}", target_smallvec.as_slice().len());
					let mut_slice = target_smallvec.as_mut_slice();
					println!("{:?}", mut_slice.len());
					
					let index_val = _to_usize(GLOBAL_DATA, offset + 5);
					
					if !target_smallvec.is_empty() {
						let actual_index = index_val % target_smallvec.len();
						let indexed_result = &mut target_smallvec[actual_index];
						println!("{:?}", *indexed_result);
					}
				}
				2 => {
					let initial_size = _to_u8(GLOBAL_DATA, offset + 1) % 40;
					let mut base_vec = std::vec::Vec::with_capacity(initial_size as usize);
					for vec_idx in 0..initial_size {
						let val = _to_usize(GLOBAL_DATA, offset + 2 + vec_idx as usize);
						base_vec.push(CustomType1(val));
					}
					
					let mut target_smallvec = SmallVec::<[CustomType1; 32]>::from_vec(base_vec);
					
					let reserve_amount = _to_usize(GLOBAL_DATA, offset + 8);
					target_smallvec.reserve(reserve_amount);
					
					let truncate_len = _to_usize(GLOBAL_DATA, offset + 9) % 65;
					target_smallvec.truncate(truncate_len);
					
					println!("{:?}", target_smallvec.capacity());
					
					if !target_smallvec.is_empty() {
						let idx_val = _to_usize(GLOBAL_DATA, offset + 12);
						let actual_index = idx_val % target_smallvec.len();
						let mut_ref = &mut target_smallvec[actual_index];
						println!("{:?}", *mut_ref);
					}
				}
				3 => {
					let elem_count = _to_u8(GLOBAL_DATA, offset + 1) % 50;
					let elem_value = _to_usize(GLOBAL_DATA, offset + 2);
					let mut target_smallvec = SmallVec::<[CustomType1; 64]>::from_elem(CustomType1(elem_value), elem_count as usize);
					
					let insert_idx = _to_usize(GLOBAL_DATA, offset + 10);
					let insert_val = _to_usize(GLOBAL_DATA, offset + 11);
					if !target_smallvec.is_empty() && insert_idx < target_smallvec.len() {
						target_smallvec.insert(insert_idx, CustomType1(insert_val));
					}
					
					let grow_cap = _to_usize(GLOBAL_DATA, offset + 12);
					target_smallvec.grow(grow_cap);
					
					println!("{:?}", target_smallvec.len());
					
					if !target_smallvec.is_empty() {
						let access_idx = _to_usize(GLOBAL_DATA, offset + 13);
						let actual_index = access_idx % target_smallvec.len();
						let result_ref = &mut target_smallvec[actual_index];
						println!("{:?}", *result_ref);
					}
				}
				4 => {
					let mut target_smallvec = SmallVec::<[CustomType1; 128]>::new();
					
					let extend_count = _to_u8(GLOBAL_DATA, offset + 1) % 30;
					for ext_idx in 0..extend_count {
						let val = _to_usize(GLOBAL_DATA, offset + 2 + ext_idx as usize);
						target_smallvec.push(CustomType1(val));
					}
					
					let shrink_decision = _to_u8(GLOBAL_DATA, offset + 12);
					if shrink_decision % 2 == 0 {
						target_smallvec.shrink_to_fit();
					}
					
					let clear_decision = _to_u8(GLOBAL_DATA, offset + 13);
					if clear_decision % 3 == 0 {
						target_smallvec.clear();
					}
					
					if !target_smallvec.is_empty() {
						let idx_param = _to_usize(GLOBAL_DATA, offset + 14);
						let actual_index = idx_param % target_smallvec.len();
						let deref_result = &mut target_smallvec[actual_index];
						println!("{:?}", *deref_result);
					}
				}
				5 => {
					let initial_cap = _to_usize(GLOBAL_DATA, offset + 1);
					let mut target_smallvec = SmallVec::<[CustomType1; 256]>::with_capacity(initial_cap);
					
					let populate_count = _to_u8(GLOBAL_DATA, offset + 9) % 45;
					for pop_idx in 0..populate_count {
						let item_data = _to_usize(GLOBAL_DATA, offset + 10 + pop_idx as usize);
						target_smallvec.push(CustomType1(item_data));
					}
					
					let remove_idx = _to_usize(GLOBAL_DATA, offset + 2);
					if !target_smallvec.is_empty() && remove_idx < target_smallvec.len() {
						let removed_item = target_smallvec.remove(remove_idx);
						println!("{:?}", removed_item.0);
					}
					
					if !target_smallvec.is_empty() {
						let final_idx = _to_usize(GLOBAL_DATA, offset + 3);
						let actual_index = final_idx % target_smallvec.len();
						let final_result = &mut target_smallvec[actual_index];
						println!("{:?}", *final_result);
					}
				}
				6 => {
					let vec_size = _to_u8(GLOBAL_DATA, offset + 1) % 35;
					let mut data_vec = std::vec::Vec::with_capacity(vec_size as usize);
					for data_idx in 0..vec_size {
						let data_val = _to_usize(GLOBAL_DATA, offset + 2 + data_idx as usize);
						data_vec.push(CustomType1(data_val));
					}
					
					let mut target_smallvec = SmallVec::<[CustomType1; 512]>::from_vec(data_vec);
					
					let swap_remove_idx = _to_usize(GLOBAL_DATA, offset + 8);
					if !target_smallvec.is_empty() && swap_remove_idx < target_smallvec.len() {
						let swapped_item = target_smallvec.swap_remove(swap_remove_idx);
						println!("{:?}", swapped_item.0);
					}
					
					let resize_len = _to_usize(GLOBAL_DATA, offset + 9) % 65;
					let resize_val = _to_usize(GLOBAL_DATA, offset + 10);
					target_smallvec.resize(resize_len, CustomType1(resize_val));
					
					if !target_smallvec.is_empty() {
						let resize_idx = _to_usize(GLOBAL_DATA, offset + 11);
						let actual_index = resize_idx % target_smallvec.len();
						let resize_result = &mut target_smallvec[actual_index];
						println!("{:?}", *resize_result);
					}
				}
				7 => {
					let slice_len = _to_u8(GLOBAL_DATA, offset + 1) % 25;
					let mut slice_data = std::vec::Vec::with_capacity(slice_len as usize);
					for slice_idx in 0..slice_len {
						let slice_val = _to_usize(GLOBAL_DATA, offset + 2 + slice_idx as usize);
						slice_data.push(CustomType1(slice_val));
					}
					
					let slice_ref = &slice_data[..];
					let mut target_smallvec = SmallVec::<[CustomType1; 1024]>::from_slice(slice_ref);
					
					let drain_start = _to_usize(GLOBAL_DATA, offset + 10);
					let drain_end = _to_usize(GLOBAL_DATA, offset + 11);
					if !target_smallvec.is_empty() {
						let actual_start = drain_start.min(target_smallvec.len());
						let actual_end = drain_end.min(target_smallvec.len()).max(actual_start);
						let drained: Vec<_> = target_smallvec.drain(actual_start..actual_end).collect();
						println!("{:?}", drained.len());
					}
					
					if !target_smallvec.is_empty() {
						let drain_idx = _to_usize(GLOBAL_DATA, offset + 12);
						let actual_index = drain_idx % target_smallvec.len();
						let drain_result = &mut target_smallvec[actual_index];
						println!("{:?}", *drain_result);
					}
				}
				8 => {
					let mut target_smallvec = SmallVec::<[CustomType1; 96]>::new();
					
					let append_count = _to_u8(GLOBAL_DATA, offset + 1) % 20;
					for app_idx in 0..append_count {
						let app_val = _to_usize(GLOBAL_DATA, offset + 2 + app_idx as usize);
						target_smallvec.push(CustomType1(app_val));
					}
					
					let mut second_smallvec = SmallVec::<[CustomType1; 96]>::new();
					let second_count = _to_u8(GLOBAL_DATA, offset + 10) % 15;
					for sec_idx in 0..second_count {
						let sec_val = _to_usize(GLOBAL_DATA, offset + 11 + sec_idx as usize);
						second_smallvec.push(CustomType1(sec_val));
					}
					
					target_smallvec.append(&mut second_smallvec);
					
					println!("{:?}", target_smallvec.len());
					println!("{:?}", second_smallvec.len());
					
					if !target_smallvec.is_empty() {
						let append_idx = _to_usize(GLOBAL_DATA, offset + 13);
						let actual_index = append_idx % target_smallvec.len();
						let append_result = &mut target_smallvec[actual_index];
						println!("{:?}", *append_result);
					}
				}
				9 => {
					let elem_val = _to_usize(GLOBAL_DATA, offset + 1);
					let elem_count = _to_u8(GLOBAL_DATA, offset + 9) % 40;
					let mut target_smallvec = SmallVec::<[CustomType1; 2048]>::from_elem(CustomType1(elem_val), elem_count as usize);
					
					let insert_pos = _to_usize(GLOBAL_DATA, offset + 10);
					let insert_slice_len = _to_u8(GLOBAL_DATA, offset + 11) % 10;
					let mut insert_data = std::vec::Vec::with_capacity(insert_slice_len as usize);
					for ins_idx in 0..insert_slice_len {
						let ins_val = _to_usize(GLOBAL_DATA, offset + 12 + ins_idx as usize);
						insert_data.push(CustomType1(ins_val));
					}
					
					if insert_pos <= target_smallvec.len() {
						target_smallvec.insert_from_slice(insert_pos, &insert_data[..]);
					}
					
					if !target_smallvec.is_empty() {
						let insert_idx = _to_usize(GLOBAL_DATA, offset + 2);
						let actual_index = insert_idx % target_smallvec.len();
						let insert_result = &mut target_smallvec[actual_index];
						println!("{:?}", *insert_result);
					}
				}
				10 => {
					let initial_count = _to_u8(GLOBAL_DATA, offset + 1) % 30;
					let mut target_smallvec = SmallVec::<[CustomType1; 4096]>::new();
					for init_idx in 0..initial_count {
						let init_val = _to_usize(GLOBAL_DATA, offset + 2 + init_idx as usize);
						target_smallvec.push(CustomType1(init_val));
					}
					
					let extend_slice_len = _to_u8(GLOBAL_DATA, offset + 9) % 15;
					let mut extend_data = std::vec::Vec::with_capacity(extend_slice_len as usize);
					for ext_idx in 0..extend_slice_len {
						let ext_val = _to_usize(GLOBAL_DATA, offset + 10 + ext_idx as usize);
						extend_data.push(CustomType1(ext_val));
					}
					
					target_smallvec.extend_from_slice(&extend_data[..]);
					
					println!("{:?}", target_smallvec.len());
					println!("{:?}", target_smallvec.capacity());
					
					if !target_smallvec.is_empty() {
						let extend_idx = _to_usize(GLOBAL_DATA, offset + 12);
						let actual_index = extend_idx % target_smallvec.len();
						let extend_result = &mut target_smallvec[actual_index];
						println!("{:?}", *extend_result);
					}
				}
				11 => {
					let cap_val = _to_usize(GLOBAL_DATA, offset + 1);
					let mut target_smallvec = SmallVec::<[CustomType1; 8192]>::with_capacity(cap_val);
					
					let fill_count = _to_u8(GLOBAL_DATA, offset + 9) % 25;
					for fill_idx in 0..fill_count {
						let fill_val = _to_usize(GLOBAL_DATA, offset + 10 + fill_idx as usize);
						target_smallvec.push(CustomType1(fill_val));
					}
					
					let reserve_exact_amount = _to_usize(GLOBAL_DATA, offset + 2);
					target_smallvec.reserve_exact(reserve_exact_amount);
					
					if target_smallvec.len() > 0 {
						let pop_result = target_smallvec.pop();
						if let Some(popped) = pop_result {
							println!("{:?}", popped.0);
						}
					}
					
					if !target_smallvec.is_empty() {
						let reserve_idx = _to_usize(GLOBAL_DATA, offset + 3);
						let actual_index = reserve_idx % target_smallvec.len();
						let reserve_result = &mut target_smallvec[actual_index];
						println!("{:?}", *reserve_result);
					}
				}
				12 => {
					let base_count = _to_u8(GLOBAL_DATA, offset + 1) % 50;
					let mut target_smallvec = SmallVec::<[CustomType1; 16384]>::new();
					for base_idx in 0..base_count {
						let base_val = _to_usize(GLOBAL_DATA, offset + 2 + base_idx as usize);
						target_smallvec.push(CustomType1(base_val));
					}
					
					let clone_vec = target_smallvec.clone();
					println!("{:?}", clone_vec.len());
					
					let resize_with_len = _to_usize(GLOBAL_DATA, offset + 10) % 65;
					target_smallvec.resize_with(resize_with_len, || {
						let global_data = get_global_data();
						let val = _to_usize(global_data.second_half, offset + 11);
						CustomType1(val)
					});
					
					if !target_smallvec.is_empty() {
						let clone_idx = _to_usize(GLOBAL_DATA, offset + 12);
						let actual_index = clone_idx % target_smallvec.len();
						let clone_result = &mut target_smallvec[actual_index];
						println!("{:?}", *clone_result);
					}
				}
				13 => {
					let iter_count = _to_u8(GLOBAL_DATA, offset + 1) % 35;
					let mut iter_data = std::vec::Vec::with_capacity(iter_count as usize);
					for iter_idx in 0..iter_count {
						let iter_val = _to_usize(GLOBAL_DATA, offset + 2 + iter_idx as usize);
						iter_data.push(CustomType1(iter_val));
					}
					
					let target_smallvec = SmallVec::<[CustomType1; 32768]>::from_iter(iter_data.into_iter());
					
					println!("{:?}", target_smallvec.len());
					println!("{:?}", target_smallvec.spilled());
					
					let into_vec_result = target_smallvec.into_vec();
					println!("{:?}", into_vec_result.len());
				}
				14 => {
					let dedup_count = _to_u8(GLOBAL_DATA, offset + 1) % 20;
					let mut target_smallvec = SmallVec::<[CustomType1; 65536]>::new();
					for dedup_idx in 0..dedup_count {
						let dedup_val = _to_usize(GLOBAL_DATA, offset + 2 + dedup_idx as usize) % 5;
						target_smallvec.push(CustomType1(dedup_val));
					}
					
					let original_len = target_smallvec.len();
					target_smallvec.dedup_by(|a, b| a.0 == b.0);
					println!("{:?}", original_len);
					println!("{:?}", target_smallvec.len());
					
					if !target_smallvec.is_empty() {
						let dedup_idx = _to_usize(GLOBAL_DATA, offset + 10);
						let actual_index = dedup_idx % target_smallvec.len();
						let dedup_result = &mut target_smallvec[actual_index];
						println!("{:?}", *dedup_result);
					}
				}
				15 => {
					let retain_count = _to_u8(GLOBAL_DATA, offset + 1) % 30;
					let mut target_smallvec = SmallVec::<[CustomType1; 131072]>::new();
					for retain_idx in 0..retain_count {
						let retain_val = _to_usize(GLOBAL_DATA, offset + 2 + retain_idx as usize);
						target_smallvec.push(CustomType1(retain_val));
					}
					
					let original_len = target_smallvec.len();
					target_smallvec.retain(|item| item.0 % 2 == 0);
					println!("{:?}", original_len);
					println!("{:?}", target_smallvec.len());
					
					if !target_smallvec.is_empty() {
						let retain_idx = _to_usize(GLOBAL_DATA, offset + 10);
						let actual_index = retain_idx % target_smallvec.len();
						let retain_result = &mut target_smallvec[actual_index];
						println!("{:?}", *retain_result);
					}
				}
				16 => {
					let cmp_count = _to_u8(GLOBAL_DATA, offset + 1) % 15;
					let mut first_smallvec = SmallVec::<[CustomType1; 262144]>::new();
					let mut second_smallvec = SmallVec::<[CustomType1; 262144]>::new();
					
					for cmp_idx in 0..cmp_count {
						let first_val = _to_usize(GLOBAL_DATA, offset + 2 + cmp_idx as usize);
						let second_val = _to_usize(GLOBAL_DATA, offset + 3 + cmp_idx as usize);
						first_smallvec.push(CustomType1(first_val));
						second_smallvec.push(CustomType1(second_val));
					}
					
					let cmp_result = first_smallvec.cmp(&second_smallvec);
					println!("{:?}", cmp_result);
					
					let eq_result = first_smallvec.eq(&second_smallvec);
					println!("{:?}", eq_result);
					
					if !first_smallvec.is_empty() {
						let cmp_idx = _to_usize(GLOBAL_DATA, offset + 12);
						let actual_index = cmp_idx % first_smallvec.len();
						let cmp_result = &mut first_smallvec[actual_index];
						println!("{:?}", *cmp_result);
					}
				}
				17 => {
					let many_count = _to_u8(GLOBAL_DATA, offset + 1) % 20;
					let mut many_items = std::vec::Vec::with_capacity(many_count as usize);
					for many_idx in 0..many_count {
						let many_val = _to_usize(GLOBAL_DATA, offset + 2 + many_idx as usize);
						many_items.push(CustomType1(many_val));
					}
					
					let mut target_smallvec = SmallVec::<[CustomType1; 524288]>::new();
					let insert_pos = _to_usize(GLOBAL_DATA, offset + 10);
					
					if insert_pos <= target_smallvec.len() {
						target_smallvec.insert_many(insert_pos, many_items.into_iter());
					}
					
					println!("{:?}", target_smallvec.len());
					
					if !target_smallvec.is_empty() {
						let many_idx = _to_usize(GLOBAL_DATA, offset + 11);
						let actual_index = many_idx % target_smallvec.len();
						let many_result = &mut target_smallvec[actual_index];
						println!("{:?}", *many_result);
					}
				}
				18 => {
					let ptr_count = _to_u8(GLOBAL_DATA, offset + 1) % 25;
					let mut target_smallvec = SmallVec::<[CustomType1; 1048576]>::new();
					for ptr_idx in 0..ptr_count {
						let ptr_val = _to_usize(GLOBAL_DATA, offset + 2 + ptr_idx as usize);
						target_smallvec.push(CustomType1(ptr_val));
					}
					
					let ptr_result = target_smallvec.as_ptr();
					let mut_ptr_result = target_smallvec.as_mut_ptr();
					println!("{:?}", ptr_result as usize);
					println!("{:?}", mut_ptr_result as usize);
					
					if !target_smallvec.is_empty() {
						let ptr_idx = _to_usize(GLOBAL_DATA, offset + 10);
						let actual_index = ptr_idx % target_smallvec.len();
						let ptr_final = &mut target_smallvec[actual_index];
						println!("{:?}", *ptr_final);
					}
				}
				_ => {
					let default_count = _to_u8(GLOBAL_DATA, offset + 1) % 40;
					let mut target_smallvec = SmallVec::<[CustomType1; 1024]>::new();
					for def_idx in 0..default_count {
						let def_val = _to_usize(GLOBAL_DATA, offset + 2 + def_idx as usize);
						target_smallvec.push(CustomType1(def_val));
					}
					
					let boxed_slice = target_smallvec.into_boxed_slice();
					println!("{:?}", boxed_slice.len());
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