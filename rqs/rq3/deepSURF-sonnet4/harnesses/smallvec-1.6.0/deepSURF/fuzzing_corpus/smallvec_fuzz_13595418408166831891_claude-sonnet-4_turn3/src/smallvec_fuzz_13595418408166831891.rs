#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

struct CustomType0(String);

impl core::clone::Clone for CustomType0 {
    fn clone(&self) -> Self {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let selector = _to_u8(GLOBAL_DATA, 5) % 3;
        if selector == 0 {
            panic!("INTENTIONAL PANIC!");
        }
        let string_len = _to_u8(GLOBAL_DATA, 6) % 20;
        let string_str = _to_str(GLOBAL_DATA, 7, 7 + string_len as usize);
        CustomType0(String::from(string_str))
    }
}

#[derive(Debug)]
struct CustomType1(usize);

impl core::clone::Clone for CustomType1 {
    fn clone(&self) -> Self {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 43);
        let custom_impl_inst_num = self.0;
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0{
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector{
                1 => global_data.first_half,
                _ => global_data.second_half,
        };
        let t_12 = _to_usize(GLOBAL_DATA, 51);
        let t_13 = CustomType1(t_12);
        return t_13;
    }
}

impl core::marker::Copy for CustomType1 {
}

impl PartialEq for CustomType1 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for CustomType1 {}

impl PartialOrd for CustomType1 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl Ord for CustomType1 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

fn main (){
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 2000 {return;}
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        
        let outer_loop_count = _to_u8(GLOBAL_DATA, 0) % 65;
        
        for i in 0..outer_loop_count {
            let base_offset = i as usize * 20;
            if base_offset + 20 >= GLOBAL_DATA.len() { break; }
            
            let operation_selector = _to_u8(GLOBAL_DATA, base_offset) % 10;
            
            match operation_selector {
                0 | 3 | 6 => {
                    let constructor_selector = _to_u8(GLOBAL_DATA, base_offset + 1) % 6;
                    let mut sv = match constructor_selector {
                        0 => SmallVec::<[CustomType1; 16]>::new(),
                        1 => {
                            let capacity = _to_usize(GLOBAL_DATA, base_offset + 2);
                            SmallVec::<[CustomType1; 16]>::with_capacity(capacity)
                        },
                        2 => {
                            let elem = CustomType1(_to_usize(GLOBAL_DATA, base_offset + 10));
                            let count = _to_usize(GLOBAL_DATA, base_offset + 18);
                            SmallVec::<[CustomType1; 16]>::from_elem(elem, count)
                        },
                        3 => {
                            let mut vec = Vec::new();
                            let vec_size = _to_u8(GLOBAL_DATA, base_offset + 19) % 65;
                            for j in 0..vec_size {
                                vec.push(CustomType1(_to_usize(GLOBAL_DATA, (base_offset + j as usize) % GLOBAL_DATA.len())));
                            }
                            SmallVec::<[CustomType1; 16]>::from_vec(vec)
                        },
                        4 => {
                            let slice_size = _to_u8(GLOBAL_DATA, base_offset + 15) % 20;
                            let mut temp_vec = Vec::new();
                            for k in 0..slice_size {
                                temp_vec.push(CustomType1(_to_usize(GLOBAL_DATA, (base_offset + k as usize) % GLOBAL_DATA.len())));
                            }
                            SmallVec::<[CustomType1; 16]>::from_slice(&temp_vec[..])
                        },
                        _ => {
                            let slice_size = _to_u8(GLOBAL_DATA, base_offset + 15) % 20;
                            let mut temp_vec = Vec::new();
                            for k in 0..slice_size {
                                temp_vec.push(CustomType1(_to_usize(GLOBAL_DATA, (base_offset + k as usize) % GLOBAL_DATA.len())));
                            }
                            temp_vec[..].to_smallvec()
                        },
                    };
                    
                    let insert_index = _to_usize(GLOBAL_DATA, base_offset + 2);
                    let slice_len = _to_u8(GLOBAL_DATA, base_offset + 10) % 25;
                    let mut insert_vec = Vec::new();
                    for j in 0..slice_len {
                        insert_vec.push(CustomType1(_to_usize(GLOBAL_DATA, (base_offset + j as usize + 11) % GLOBAL_DATA.len())));
                    }
                    let insert_slice = &insert_vec[..];
                    
                    sv.insert_from_slice(insert_index, insert_slice);
                    
                    let slice_ref = sv.as_slice();
                    if !slice_ref.is_empty() {
                        let first_elem = &slice_ref[0];
                        println!("{:?}", *first_elem);
                    }
                    
                    let sv_len = sv.len();
                    let grow_amount = _to_usize(GLOBAL_DATA, base_offset + 16);
                    sv.grow(grow_amount);
                    
                    if !sv.is_empty() {
                        let pop_result = sv.pop();
                        if let Some(popped) = pop_result {
                            println!("{:?}", popped);
                        }
                    }
                    
                    let spilled = sv.spilled();
                    println!("Spilled: {}", spilled);
                },
                1 => {
                    let constructor_selector = _to_u8(GLOBAL_DATA, base_offset + 1) % 3;
                    let mut sv = match constructor_selector {
                        0 => SmallVec::<[CustomType1; 32]>::new(),
                        1 => {
                            let capacity = _to_usize(GLOBAL_DATA, base_offset + 2);
                            SmallVec::<[CustomType1; 32]>::with_capacity(capacity)
                        },
                        _ => {
                            let mut vec = Vec::new();
                            let vec_size = _to_u8(GLOBAL_DATA, base_offset + 10) % 15;
                            for j in 0..vec_size {
                                vec.push(CustomType1(_to_usize(GLOBAL_DATA, (base_offset + j as usize) % GLOBAL_DATA.len())));
                            }
                            SmallVec::<[CustomType1; 32]>::from_vec(vec)
                        },
                    };
                    
                    let push_count = _to_u8(GLOBAL_DATA, base_offset + 15) % 10;
                    for j in 0..push_count {
                        sv.push(CustomType1(_to_usize(GLOBAL_DATA, (base_offset + j as usize + 16) % GLOBAL_DATA.len())));
                    }
                    
                    let len_before = sv.len();
                    let extend_slice_len = _to_u8(GLOBAL_DATA, base_offset + 17) % 12;
                    let mut extend_vec = Vec::new();
                    for j in 0..extend_slice_len {
                        extend_vec.push(CustomType1(_to_usize(GLOBAL_DATA, (base_offset + j as usize + 18) % GLOBAL_DATA.len())));
                    }
                    sv.extend_from_slice(&extend_vec[..]);
                    
                    if sv.len() > len_before {
                        let new_elem = &sv[len_before];
                        println!("{:?}", *new_elem);
                    }
                    
                    let vec_result = sv.into_vec();
                    let vec_len = vec_result.len();
                    println!("Vec length: {}", vec_len);
                },
                2 => {
                    let mut sv1 = SmallVec::<[CustomType1; 12]>::new();
                    let mut sv2 = SmallVec::<[CustomType1; 12]>::new();
                    
                    let count1 = _to_u8(GLOBAL_DATA, base_offset + 5) % 8;
                    for j in 0..count1 {
                        sv1.push(CustomType1(_to_usize(GLOBAL_DATA, (base_offset + j as usize + 6) % GLOBAL_DATA.len())));
                    }
                    
                    let count2 = _to_u8(GLOBAL_DATA, base_offset + 14) % 8;
                    for j in 0..count2 {
                        sv2.push(CustomType1(_to_usize(GLOBAL_DATA, (base_offset + j as usize + 15) % GLOBAL_DATA.len())));
                    }
                    
                    let ordering = sv1.cmp(&sv2);
                    println!("{:?}", ordering);
                    
                    let partial_ordering = sv1.partial_cmp(&sv2);
                    if let Some(ord) = partial_ordering {
                        println!("{:?}", ord);
                    }
                    
                    sv1.append(&mut sv2);
                    let combined_len = sv1.len();
                    println!("Combined length: {}", combined_len);
                },
                4 => {
                    let mut sv = SmallVec::<[CustomType1; 20]>::new();
                    let initial_count = _to_u8(GLOBAL_DATA, base_offset + 3) % 15;
                    for j in 0..initial_count {
                        sv.push(CustomType1(_to_usize(GLOBAL_DATA, (base_offset + j as usize + 4) % GLOBAL_DATA.len())));
                    }
                    
                    let drain_start = _to_usize(GLOBAL_DATA, base_offset + 19);
                    let drain_end = _to_usize(GLOBAL_DATA, base_offset + 15);
                    
                    if !sv.is_empty() {
                        let actual_start = drain_start % sv.len();
                        let actual_end = if drain_end > actual_start { 
                            std::cmp::min(drain_end, sv.len()) 
                        } else { 
                            actual_start + 1 
                        };
                        
                        let mut drain_iter = sv.drain(actual_start..actual_end);
                        if let Some(first_drained) = drain_iter.next() {
                            println!("{:?}", first_drained);
                        }
                    }
                    
                    let boxed_slice = sv.into_boxed_slice();
                    println!("Boxed slice length: {}", boxed_slice.len());
                },
                5 => {
                    let mut sv = SmallVec::<[CustomType1; 24]>::new();
                    let count = _to_u8(GLOBAL_DATA, base_offset + 2) % 20;
                    for j in 0..count {
                        sv.push(CustomType1(_to_usize(GLOBAL_DATA, (base_offset + j as usize + 3) % GLOBAL_DATA.len())));
                    }
                    
                    let reserve_amount = _to_usize(GLOBAL_DATA, base_offset + 10);
                    sv.reserve(reserve_amount);
                    
                    let capacity_after = sv.capacity();
                    println!("Capacity: {}", capacity_after);
                    
                    if !sv.is_empty() {
                        let remove_index = _to_usize(GLOBAL_DATA, base_offset + 18) % sv.len();
                        let removed = sv.remove(remove_index);
                        println!("{:?}", removed);
                    }
                    
                    let truncate_len = _to_usize(GLOBAL_DATA, base_offset + 12);
                    sv.truncate(truncate_len);
                    
                    let len_after_truncate = sv.len();
                    println!("Length after truncate: {}", len_after_truncate);
                },
                7 => {
                    let mut sv = SmallVec::<[CustomType1; 28]>::new();
                    let count = _to_u8(GLOBAL_DATA, base_offset + 1) % 25;
                    for j in 0..count {
                        sv.push(CustomType1(_to_usize(GLOBAL_DATA, (base_offset + j as usize + 2) % GLOBAL_DATA.len())));
                    }
                    
                    let clone_sv = sv.clone();
                    let eq_result = sv.eq(&clone_sv);
                    println!("Equal: {}", eq_result);
                    
                    sv.clear();
                    let is_empty_result = sv.is_empty();
                    println!("Is empty: {}", is_empty_result);
                    
                    let iter = clone_sv.into_iter();
                    let iter_len = iter.size_hint().0;
                    println!("Iterator length hint: {}", iter_len);
                },
                8 => {
                    let mut sv = SmallVec::<[CustomType1; 30]>::new();
                    let count = _to_u8(GLOBAL_DATA, base_offset + 4) % 20;
                    for j in 0..count {
                        sv.push(CustomType1(_to_usize(GLOBAL_DATA, (base_offset + j as usize + 5) % GLOBAL_DATA.len())));
                    }
                    
                    let retain_threshold = _to_usize(GLOBAL_DATA, base_offset + 8);
                    sv.retain(|item| item.0 > retain_threshold);
                    
                    sv.dedup();
                    
                    let deduped_len = sv.len();
                    println!("Deduped length: {}", deduped_len);
                },
                9 => {
                    let mut sv = SmallVec::<[CustomType1; 36]>::new();
                    let count = _to_u8(GLOBAL_DATA, base_offset + 3) % 30;
                    for j in 0..count {
                        sv.push(CustomType1(_to_usize(GLOBAL_DATA, (base_offset + j as usize + 4) % GLOBAL_DATA.len())));
                    }
                    
                    let resize_len = _to_usize(GLOBAL_DATA, base_offset + 8);
                    let resize_value = CustomType1(_to_usize(GLOBAL_DATA, base_offset + 16));
                    sv.resize(resize_len, resize_value);
                    
                    if !sv.is_empty() {
                        let swap_remove_index = _to_usize(GLOBAL_DATA, base_offset + 12) % sv.len();
                        let swap_removed = sv.swap_remove(swap_remove_index);
                        println!("{:?}", swap_removed);
                    }
                    
                    let final_len = sv.len();
                    println!("Final length: {}", final_len);
                },
                _ => {
                    let mut sv = SmallVec::<[CustomType1; 32]>::new();
                    let count = _to_u8(GLOBAL_DATA, base_offset + 4) % 30;
                    for j in 0..count {
                        sv.push(CustomType1(_to_usize(GLOBAL_DATA, (base_offset + j as usize + 5) % GLOBAL_DATA.len())));
                    }
                    
                    let insert_index = _to_usize(GLOBAL_DATA, base_offset + 6);
                    let insert_value = CustomType1(_to_usize(GLOBAL_DATA, base_offset + 14));
                    sv.insert(insert_index, insert_value);
                    
                    let mut_slice = sv.as_mut_slice();
                    if !mut_slice.is_empty() {
                        let first_mut = &mut mut_slice[0];
                        println!("{:?}", *first_mut);
                    }
                    
                    sv.shrink_to_fit();
                    let final_capacity = sv.capacity();
                    println!("Final capacity: {}", final_capacity);
                    
                    let as_ptr = sv.as_ptr();
                    let as_mut_ptr = sv.as_mut_ptr();
                    println!("Pointers equal: {}", as_ptr == as_mut_ptr);
                },
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