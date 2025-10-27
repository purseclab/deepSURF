#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use slice_deque::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

#[derive(Debug, PartialEq)]
struct CustomType2(String);
#[derive(Debug)]
struct CustomType0(String);
#[derive(Debug, PartialEq)]
struct CustomType3(String);
struct CustomType1(String);

impl std::clone::Clone for CustomType2 {
    fn clone(&self) -> Self {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 26);
        let custom_impl_inst_num = self.0.len();
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0 {
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector {
            1 => global_data.first_half,
            _ => global_data.second_half,
        };
        let mut t_7 = _to_u8(GLOBAL_DATA, 34) % 17;
        let t_8 = _to_str(GLOBAL_DATA, 35, 35 + t_7 as usize);
        let t_9 = String::from(t_8);
        CustomType2(t_9)
    }
}

impl std::iter::IntoIterator for CustomType0 {
    type Item = CustomType2;
    type IntoIter = std::vec::IntoIter<CustomType2>;
    
    fn into_iter(self) -> Self::IntoIter {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 687);
        let custom_impl_inst_num = self.0.len();
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0 {
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector {
            1 => global_data.first_half,
            _ => global_data.second_half,
        };
        let mut t_163 = _to_u8(GLOBAL_DATA, 695) % 17;
        let t_164 = _to_str(GLOBAL_DATA, 696, 696 + t_163 as usize);
        let t_165 = String::from(t_164);
        let elements: Vec<CustomType2> = t_165.chars().map(|c| CustomType2(c.to_string())).collect();
        elements.into_iter()
    }
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 4096 {return;}
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let constructor_selector = _to_u8(GLOBAL_DATA, 0) % 5;
        let mut deque = match constructor_selector {
            0 => {
                let mut vec = Vec::with_capacity(32);
                for i in 0..32 {
                    let offset = 1 + i * 2;
                    let str_len = _to_u8(GLOBAL_DATA, offset) % 17;
                    let s = _to_str(GLOBAL_DATA, offset + 1, offset + 1 + str_len as usize);
                    vec.push(CustomType2(s.to_string()));
                }
                SliceDeque::from(&vec[..])
            },
            1 => SliceDeque::new(),
            2 => SliceDeque::with_capacity(_to_usize(GLOBAL_DATA, 1) % 65),
            3 => {
                let elem = CustomType2("default".to_string());
                slice_deque::from_elem(elem, _to_usize(GLOBAL_DATA, 1) % 65)
            },
            4 => SliceDeque::from_iter(CustomType0("iterator_source".to_string())),
            _ => unreachable!(),
        };

        let num_operations = _to_usize(GLOBAL_DATA, 200) % 50;
        for op_idx in 0..num_operations {
            let op_selector = _to_u8(GLOBAL_DATA, 201 + op_idx) % 8;
            match op_selector {
                0 => {
                    let str_len = _to_u8(GLOBAL_DATA, 300 + op_idx * 2) % 17;
                    let s = _to_str(GLOBAL_DATA, 301 + op_idx * 2, 301 + op_idx * 2 + str_len as usize);
                    deque.push_back(CustomType2(s.to_string()));
                },
                1 => {
                    if !deque.is_empty() {
                        deque.pop_front();
                    }
                },
                2 => {
                    deque.truncate(_to_usize(GLOBAL_DATA, 400 + op_idx) % 65);
                },
                3 => {
                    let start = _to_usize(GLOBAL_DATA, 500 + op_idx * 2) % (deque.len() + 1);
                    let end = _to_usize(GLOBAL_DATA, 500 + op_idx * 2 + 2) % (deque.len() + 1);
                    let _ = deque.drain(start..end);
                },
                4 => {
                    let mut buf = Vec::new();
                    let count = _to_u8(GLOBAL_DATA, 600 + op_idx) % 5;
                    for _ in 0..count {
                        let str_len = _to_u8(GLOBAL_DATA, 601 + op_idx) % 17;
                        let s = _to_str(GLOBAL_DATA, 602 + op_idx, 602 + op_idx + str_len as usize);
                        buf.push(CustomType2(s.to_string()));
                    }
                    deque.extend_from_slice(&buf);
                },
                5 => {
                    let range_start = _to_usize(GLOBAL_DATA, 750 + op_idx * 2) % (deque.len() +1);
                    let range_end = _to_usize(GLOBAL_DATA, 751 + op_idx * 2) % (deque.len() +1);
                    let str_offset = 760 + op_idx * 2;
                    let str_len = _to_u8(GLOBAL_DATA, str_offset) % 17;
                    let replace_source = _to_str(GLOBAL_DATA, str_offset +1, str_offset +1 + str_len as usize);
                    let replace_iter = CustomType0(replace_source.to_string());
                    let mut splice = deque.splice(range_start..range_end, replace_iter);
                    splice.next();
                },
                6 => {
                    let str_len = _to_u8(GLOBAL_DATA, 700 + op_idx) % 17;
                    let s = _to_str(GLOBAL_DATA, 701 + op_idx, 701 + op_idx + str_len as usize);
                    deque.retain(|_| s.len() % 2 == 0);
                },
                7 => {
                    let other = SliceDeque::new();
                    println!("Comparison result: {:?}", deque == other);
                },
                _ => (),
            }
        }

        if let Some(front) = deque.front() {
            println!("Front element: {:?}", front);
        }
        if let Some(back) = deque.back_mut() {
            *back = CustomType2("modified_back".to_string());
        }

        let range_start = _to_usize(GLOBAL_DATA, 800) % (deque.len() + 1);
        let range_end = _to_usize(GLOBAL_DATA, 802) % (deque.len() + 1);
        let replace_source = _to_str(GLOBAL_DATA, 804, 804 + 12);
        let replace_iter = CustomType0(replace_source.to_string());
        let mut final_splice = deque.splice(range_start..range_end, replace_iter);
        final_splice.next();
        final_splice.next();
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