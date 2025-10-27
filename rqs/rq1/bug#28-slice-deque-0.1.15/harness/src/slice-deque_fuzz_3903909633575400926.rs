#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use slice_deque::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

#[derive(Debug)]
struct CustomType0(String);
#[derive(Debug)]
struct CustomType1(String);
#[derive(Debug, Clone, PartialEq)]
struct CustomType2(String);
#[derive(Debug)]
struct CustomType4(String);
#[derive(Debug)]
struct CustomType3(String);

impl std::cmp::PartialEq for CustomType1 {
    fn eq(&self, _: &Self) -> bool {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 0);
        let custom_impl_inst_num = self.0.len();
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0 {
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector {
            1 => global_data.first_half,
            _ => global_data.second_half,
        };
        let t_0 = _to_bool(GLOBAL_DATA, 8);
        return t_0;
    }
}

impl std::iter::IntoIterator for CustomType3 {
    type Item = CustomType2;
    type IntoIter = CustomType4;
    
    fn into_iter(self) -> Self::IntoIter {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 58);
        let custom_impl_inst_num = self.0.len();
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0 {
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector {
            1 => global_data.first_half,
            _ => global_data.second_half,
        };
        let mut t_10 = _to_u8(GLOBAL_DATA, 66) % 17;
        let t_11 = _to_str(GLOBAL_DATA, 67, 67 + t_10 as usize);
        let t_12 = String::from(t_11);
        let t_13 = CustomType4(t_12);
        return t_13;
    }
}

impl std::iter::Iterator for CustomType4 {
    type Item = CustomType2;
    
    fn next(&mut self) -> Option<Self::Item> {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 9);
        let custom_impl_inst_num = self.0.len();
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0 {
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector {
            1 => global_data.first_half,
            _ => global_data.second_half,
        };
        let mut t_1 = _to_u8(GLOBAL_DATA, 17) % 17;
        let t_2 = _to_str(GLOBAL_DATA, 18, 18 + t_1 as usize);
        let t_3 = String::from(t_2);
        let t_4 = CustomType2(t_3);
        let t_5 = Some(t_4);
        return t_5;
    }
    
    fn size_hint(&self) -> (usize, Option<usize>) {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 34);
        let custom_impl_inst_num = self.0.len();
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0 {
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector {
            1 => global_data.first_half,
            _ => global_data.second_half,
        };
        let t_6 = _to_usize(GLOBAL_DATA, 42);
        let t_7 = _to_usize(GLOBAL_DATA, 50);
        let t_8 = Some(t_7);
        let t_9 = (t_6, t_8);
        return t_9;
    }
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 2048 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let mut construct_selector = _to_u8(GLOBAL_DATA, 0) % 3;
        let mut deque = match construct_selector {
            0 => SliceDeque::new(),
            1 => SliceDeque::with_capacity(_to_usize(GLOBAL_DATA, 1) % 65),
            _ => {
                let t_14 = _to_u8(GLOBAL_DATA, 2) % 17;
                let t_15 = _to_str(GLOBAL_DATA, 3, 3 + t_14 as usize);
                SliceDeque::from_iter(CustomType3(t_15.to_string()))
            }
        };

        let ops_count = _to_usize(GLOBAL_DATA, 20) % 10;
        for i in 0..ops_count {
            let op_byte = _to_u8(GLOBAL_DATA, 21 + i) % 6;
            match op_byte {
                0 => {
                    let elem_len = _to_u8(GLOBAL_DATA, 30 + i) % 17;
                    let elem_data = _to_str(GLOBAL_DATA, 40 + i, 40 + i + elem_len as usize);
                    deque.push_front(CustomType2(elem_data.to_string()));
                }
                1 => {
                    let elem_len = _to_u8(GLOBAL_DATA, 50 + i) % 17;
                    let elem_data = _to_str(GLOBAL_DATA, 60 + i, 60 + i + elem_len as usize);
                    deque.push_back(CustomType2(elem_data.to_string()));
                }
                2 => { deque.pop_front(); }
                3 => { deque.pop_back(); }
                4 => deque.reserve(_to_usize(GLOBAL_DATA, 70 + i) % 100),
                5 => deque.truncate(_to_usize(GLOBAL_DATA, 80 + i) % (deque.len() + 1)),
                _ => unreachable!(),
            }
        }

        let mut array_elems = Vec::new();
        for idx in 0..32 {
            let base = 100 + idx * 20;
            let len = _to_u8(GLOBAL_DATA, base) % 17;
            let s = _to_str(GLOBAL_DATA, base + 1, base + 1 + len as usize);
            array_elems.push(CustomType2(s.to_string()));
        }
        let arr: [_; 32] = array_elems.try_into().unwrap();

        let (s1, s2) = deque.as_slices();
        println!("{:?} {:?}", s1.first(), s2.last());
        
        let _ = deque.eq(&arr);
        let _ = slice_deque::from_elem(CustomType2("".to_string()), _to_usize(GLOBAL_DATA, 500) % 65);
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