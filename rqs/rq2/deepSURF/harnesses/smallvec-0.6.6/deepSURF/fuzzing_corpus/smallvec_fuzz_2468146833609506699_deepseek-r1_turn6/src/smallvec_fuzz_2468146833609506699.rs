#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};
use std::borrow::BorrowMut;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct CustomType1(u8);
#[derive(Debug)]
struct CustomType2(u8);
#[derive(Debug)]
struct CustomType3(u8);

impl std::iter::IntoIterator for CustomType2 {
    type Item = CustomType1;
    type IntoIter = CustomType3;
    
    fn into_iter(self) -> Self::IntoIter {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 90);
        let selector = (custom_impl_num + self.0 as usize) % 3;
        if selector == 0 {
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector {
            1 => global_data.first_half,
            _ => global_data.second_half,
        };
        let val = _to_u8(GLOBAL_DATA, 98);
        CustomType3(val)
    }
}

impl std::iter::Iterator for CustomType3 {
    type Item = CustomType1;
    
    fn size_hint(&self) -> (usize, Option<usize>) {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 41);
        let selector = (custom_impl_num + self.0 as usize) % 3;
        if selector == 0 {
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector {
            1 => global_data.first_half,
            _ => global_data.second_half,
        };
        let min = _to_usize(GLOBAL_DATA, 49);
        let max = _to_usize(GLOBAL_DATA, 57);
        (min, Some(max))
    }
    
    fn next(&mut self) -> Option<Self::Item> {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 65);
        let selector = (custom_impl_num + self.0 as usize) % 3;
        if selector == 0 {
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector {
            1 => global_data.first_half,
            _ => global_data.second_half,
        };
        let elem = _to_u8(GLOBAL_DATA, 73);
        Some(CustomType1(elem))
    }
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 3000 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let mut offset = 0;
        
        let op_count = _to_usize(global_data.first_half, offset) % 65;
        offset += 8;

        let sv2_cap = _to_usize(global_data.first_half, offset) % 65;
        offset += 8;
        let mut sv2 = smallvec::SmallVec::<[CustomType1; 64]>::with_capacity(sv2_cap);

        let slice_len = _to_usize(global_data.second_half, 0) % 65;
        let mut slice_data = Vec::with_capacity(slice_len);
        for _ in 0..slice_len {
            if offset >= global_data.first_half.len() {
                offset = 0;
            }
            let elem = _to_u8(global_data.first_half, offset);
            offset += 1;
            slice_data.push(CustomType1(elem));
        }
        let sv3 = smallvec::SmallVec::<[CustomType1; 64]>::from_slice(&slice_data);
        let mut sv4 = smallvec::SmallVec::<[CustomType1; 64]>::from_vec(slice_data.clone());

        let mut sv1 = smallvec::SmallVec::<[CustomType1; 64]>::new();
        let mut sv5 = smallvec::SmallVec::from(sv3.clone());
        let sv6 = smallvec::SmallVec::<[CustomType1; 64]>::from_elem(CustomType1(0), _to_usize(global_data.first_half, offset) % 65);
        offset += 8;

        for _ in 0..op_count {
            let op = _to_u8(global_data.first_half, offset) % 15;
            offset += 1;

            match op {
                0 => {
                    if offset >= global_data.first_half.len() {
                        offset = 0;
                    }
                    let elem = _to_u8(global_data.first_half, offset);
                    offset += 1;
                    sv1.push(CustomType1(elem));
                    sv5.extend_from_slice(sv1.as_slice());
                }
                1 => { 
                    if !sv1.is_empty() { 
                        sv1.pop(); 
                        sv2.truncate(_to_usize(global_data.second_half, offset) % 65);
                    } 
                }
                2 => {
                    let idx = _to_usize(global_data.first_half, offset) % (sv1.len() + 1);
                    if offset >= global_data.first_half.len() {
                        offset = 0;
                    }
                    let elem = _to_u8(global_data.first_half, offset);
                    offset += 1;
                    sv1.insert(idx, CustomType1(elem));
                    sv4.insert_from_slice(0, sv3.as_slice());
                }
                3 => sv1.truncate(_to_usize(global_data.first_half, offset) % 65),
                4 => sv2.extend_from_slice(sv1.as_slice()),
                5 => sv1.extend(sv2.drain()),
                6 => {
                    let new_size = _to_usize(global_data.first_half, offset) % 65;
                    if offset >= global_data.first_half.len() {
                        offset = 0;
                    }
                    let elem = _to_u8(global_data.first_half, offset);
                    offset += 1;
                    sv1.resize(new_size, CustomType1(elem));
                    let _ = sv5.cmp(&sv6);
                }
                7 => sv1.dedup(),
                8 => sv1.retain(|_| _to_bool(global_data.second_half, offset)),
                9 => {
                    let pos = _to_usize(global_data.second_half, offset) % (sv2.len() + 1);
                    sv2.insert_from_slice(pos, &sv3);
                    let _ = sv4.partial_cmp(&sv5);
                }
                10 => {
                    sv5 = smallvec::SmallVec::from_iter(sv1.drain());
                    let _ = sv2.as_slice().get(0).unwrap();
                }
                11 => {
                    sv1.shrink_to_fit();
                    let cap = sv1.capacity();
                    sv1.reserve(cap + _to_usize(global_data.second_half, offset));
                }
                12 => {
                    let len = sv2.len();
                    if len > 0 {
                        sv2.swap_remove(_to_usize(global_data.second_half, offset) % len);
                    }
                }
                13 => {
                    let _: &mut [CustomType1] = sv2.borrow_mut();
                    let _: &[CustomType1] = sv3.as_slice();
                }
                14 => {
                    sv5.clear();
                    sv5.extend(sv4.drain());
                }
                _ => (),
            }
            offset += 4;

            println!("Capacity sv1: {} sv2: {}", sv1.capacity(), sv2.capacity());
            let slice: &[CustomType1] = sv1.as_slice();
            println!("Slice len: {}", slice.len());
        }

        sv1.insert_from_slice(_to_usize(global_data.second_half, 8) % (sv1.len() + 1), &sv3);
        sv4.extend_from_slice(sv2.as_slice());

        let mut borrow: &mut [CustomType1] = sv1.borrow_mut();
        borrow[0] = {
            if offset >= global_data.first_half.len() {
                offset = 0;
            }
            let elem = _to_u8(global_data.first_half, offset);
            offset += 1;
            CustomType1(elem)
        };
        let mut_mut_ref = &mut borrow[0];
        *mut_mut_ref = {
            if offset >= global_data.first_half.len() {
                offset = 0;
            }
            let elem = _to_u8(global_data.first_half, offset);
            offset += 1;
            CustomType1(elem)
        };
        println!("Borrow: {:?}", &borrow[0]);

        sv4.dedup();
        sv2.extend_from_slice(sv3.as_slice());
        let comparison = sv1.cmp(&sv4);
        let partial_cmp = sv2.partial_cmp(&sv3);
        
        let another_borrow: &[CustomType1] = sv3.as_slice();
        println!("Another slice: {:?}", &another_borrow[..2]);
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