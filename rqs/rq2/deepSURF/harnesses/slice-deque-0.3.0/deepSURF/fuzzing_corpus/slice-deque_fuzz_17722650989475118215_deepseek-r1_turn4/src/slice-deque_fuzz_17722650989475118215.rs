#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use slice_deque::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

#[derive(Debug, PartialEq, PartialOrd)]
struct CustomType2(String);
#[derive(Debug, PartialEq, PartialOrd)]
struct CustomType0(String);
#[derive(Debug, PartialEq, PartialOrd)]
struct CustomType1(String);

impl std::clone::Clone for CustomType0 {
    fn clone(&self) -> Self {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 0);
        let custom_impl_inst_num = self.0.len();
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0 { panic!("INTENTIONAL PANIC!"); }
        let GLOBAL_DATA = match selector {
            1 => global_data.first_half,
            _ => global_data.second_half,
        };
        let mut t_0 = _to_u8(GLOBAL_DATA, 8) % 17;
        let t_1 = _to_str(GLOBAL_DATA, 9, 9 + t_0 as usize);
        let t_2 = String::from(t_1);
        CustomType0(t_2)
    }
}

impl std::iter::IntoIterator for CustomType1 {
    type Item = CustomType0;
    type IntoIter = CustomType2;
    
    fn into_iter(self) -> Self::IntoIter {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 99);
        let custom_impl_inst_num = self.0.len();
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0 { panic!("INTENTIONAL PANIC!"); }
        let GLOBAL_DATA = match selector {
            1 => global_data.first_half,
            _ => global_data.second_half,
        };
        let mut t_20 = _to_u8(GLOBAL_DATA, 107) % 17;
        let t_21 = _to_str(GLOBAL_DATA, 108, 108 + t_20 as usize);
        let t_22 = String::from(t_21);
        CustomType2(t_22)
    }
}

impl std::iter::Iterator for CustomType2 {
    type Item = CustomType0;
    
    fn size_hint(&self) -> (usize, Option<usize>) {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 50);
        let custom_impl_inst_num = self.0.len();
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0 { panic!("INTENTIONAL PANIC!"); }
        let GLOBAL_DATA = match selector {
            1 => global_data.first_half,
            _ => global_data.second_half,
        };
        let t_11 = _to_usize(GLOBAL_DATA, 58);
        let t_12 = _to_usize(GLOBAL_DATA, 66);
        (t_11, Some(t_12))
    }
    
    fn next(&mut self) -> Option<Self::Item> {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 74);
        let custom_impl_inst_num = self.0.len();
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0 { panic!("INTENTIONAL PANIC!"); }
        let GLOBAL_DATA = match selector {
            1 => global_data.first_half,
            _ => global_data.second_half,
        };
        let mut t_15 = _to_u8(GLOBAL_DATA, 82) % 17;
        let t_16 = _to_str(GLOBAL_DATA, 83, 83 + t_15 as usize);
        let t_17 = String::from(t_16);
        Some(CustomType0(t_17))
    }
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 2048 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let mut offset = 0;
        let op_count = _to_u8(GLOBAL_DATA, offset) % 65;
        offset += 1;

        let mut construct_selector = _to_u8(GLOBAL_DATA, offset) % 3;
        offset += 1;
        let mut deque1 = match construct_selector {
            0 => {
                let cap = _to_usize(GLOBAL_DATA, offset) % 65;
                offset += 8;
                SliceDeque::with_capacity(cap)
            },
            1 => {
                let elem_count = _to_usize(GLOBAL_DATA, offset) % 65;
                offset += 8;
                let elem = CustomType0(String::from(_to_str(GLOBAL_DATA, offset, offset + 16)));
                offset += 16;
                slice_deque::from_elem(elem, elem_count)
            },
            _ => SliceDeque::new()
        };

        construct_selector = _to_u8(GLOBAL_DATA, offset) % 3;
        offset += 1;
        let mut deque2 = match construct_selector {
            0 => {
                let cap = _to_usize(GLOBAL_DATA, offset) % 65;
                offset += 8;
                SliceDeque::with_capacity(cap)
            },
            1 => {
                let iter = CustomType1(String::from(_to_str(GLOBAL_DATA, offset, offset + 32)));
                offset += 32;
                SliceDeque::from_iter(iter)
            },
            _ => SliceDeque::new()
        };

        for _ in 0..op_count {
            let op = _to_u8(GLOBAL_DATA, offset) % 6;
            offset += 1;
            match op {
                0 => {
                    let elem = CustomType0(String::from(_to_str(GLOBAL_DATA, offset, offset + 16)));
                    offset += 16;
                    deque1.push_front(elem.clone());
                    deque2.push_back(elem);
                },
                1 => {
                    if let Some(front) = deque1.front() {
                        println!("{:?}", front);
                    }
                    if let Some(back) = deque2.back_mut() {
                        *back = CustomType0(String::from("modified"));
                    }
                },
                2 => {
                    let index = _to_usize(GLOBAL_DATA, offset);
                    offset += 8;
                    if !deque1.is_empty() {
                        deque1.truncate(index % deque1.len());
                    }
                    deque2.reserve(index % 65);
                },
                3 => {
                    let drain_range = _to_usize(GLOBAL_DATA, offset) % deque1.len().max(1);
                    offset += 8;
                    let _ = deque1.drain(..drain_range);
                    let _ = deque2.pop_back();
                },
                4 => {
                    let elem = CustomType0(String::from(_to_str(GLOBAL_DATA, offset, offset + 16)));
                    offset += 16;
                    let _ = SliceDeque::from_iter(vec![elem]);
                },
                _ => {
                    let slice = deque1.as_slice();
                    if !slice.is_empty() {
                        println!("{:?}", &slice[0]);
                    }
                    let _ = deque2.as_mut_slice();
                }
            }
        }

        let partial_cmp_result1 = deque1.partial_cmp(&deque2);
        let partial_cmp_result2 = deque2.partial_cmp(&deque1);
        println!("{:?} {:?}", partial_cmp_result1, partial_cmp_result2);
    });
}

// Conversion functions are omitted as per directions.
// Include them here if needed.

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