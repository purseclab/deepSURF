#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use slice_deque::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

struct CustomType2(String);
struct CustomType1(String);
#[derive(Clone, Debug, PartialEq)]
struct CustomType3(String);
struct CustomType0(String);

impl std::iter::Iterator for CustomType2 {
    type Item = CustomType3;
    
    fn size_hint(&self) -> (usize, Option<usize>) {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 0);
        let custom_impl_inst_num = self.0.len();
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0{
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector{
            1 => global_data.first_half,
            _ => global_data.second_half,
        };
        let t_0 = _to_usize(GLOBAL_DATA, 8);
        let t_1 = _to_usize(GLOBAL_DATA, 16);
        let t_2 = Some(t_1);
        let t_3 = (t_0, t_2);
        return t_3;
    }
    
    fn next(&mut self) -> Option<Self::Item> {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 24);
        let custom_impl_inst_num = self.0.len();
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0{
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector{
            1 => global_data.first_half,
            _ => global_data.second_half,
        };
        let mut t_4 = _to_u8(GLOBAL_DATA, 32) % 17;
        let t_5 = _to_str(GLOBAL_DATA, 33, 33 + t_4 as usize);
        let t_6 = String::from(t_5);
        let t_7 = CustomType3(t_6);
        let t_8 = Some(t_7);
        return t_8;
    }
}

impl std::iter::IntoIterator for CustomType1 {
    type Item = CustomType3;
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

fn main (){
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 4096 {return;}
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        
        let constructor_sel = _to_u8(GLOBAL_DATA, 0) % 4;
        let mut t_13 = match constructor_sel {
            0 => SliceDeque::new(),
            1 => {
                let cap = _to_usize(GLOBAL_DATA, 1) % 65;
                SliceDeque::with_capacity(cap)
            },
            2 => SliceDeque::from_iter(CustomType1(String::new())),
            _ => {
                let elem = CustomType3(_to_str(GLOBAL_DATA, 2, 5).to_string());
                slice_deque::from_elem(elem, _to_usize(GLOBAL_DATA, 6) % 65)
            }
        };
        
        let mut offset = 24;
        for _ in 0..(_to_u8(GLOBAL_DATA, 8) % 64) {
            if offset + 3 > GLOBAL_DATA.len() { break; }
            let op = _to_u8(GLOBAL_DATA, offset) % 6;
            offset += 1;
            
            match op {
                0 => {
                    let len = _to_u8(GLOBAL_DATA, offset) % 17;
                    offset += 1;
                    let s = _to_str(GLOBAL_DATA, offset, offset + len as usize);
                    offset += len as usize;
                    t_13.push_back(CustomType3(s.to_string()));
                },
                1 => {
                    let len = _to_u8(GLOBAL_DATA, offset) % 17;
                    offset += 1;
                    let s = _to_str(GLOBAL_DATA, offset, offset + len as usize);
                    offset += len as usize;
                    t_13.push_front(CustomType3(s.to_string()));
                },
                2 => {
                    t_13.pop_back();
                },
                3 => {
                    t_13.pop_front();
                },
                4 => {
                    let new_len = _to_usize(GLOBAL_DATA, offset) % (t_13.len() + 1);
                    offset += 8;
                    t_13.truncate(new_len);
                },
                5 => {
                    let start = _to_usize(GLOBAL_DATA, offset) % (t_13.len() + 1);
                    let end = start + (_to_usize(GLOBAL_DATA, offset + 8) % (t_13.len() - start + 1));
                    offset += 16;
                    let _ = t_13.drain(start..end).next();
                },
                _ => ()
            }
            
            if let Some(front) = t_13.front_mut() {
                front.0.push_str("modified");
            }
            if let Some(back) = t_13.back() {
                println!("Deref back: {:?}", back.0);
            }
        }
        
        let mut vec = Vec::new();
        offset = 512;
        while vec.len() < 32 && offset + 2 < GLOBAL_DATA.len() {
            let len = _to_u8(GLOBAL_DATA, offset) % 17;
            offset += 1;
            let s = _to_str(GLOBAL_DATA, offset, offset + len as usize);
            offset += len as usize;
            vec.push(CustomType3(s.to_string()));
        }
        vec.truncate(25);
        let arr: [CustomType3; 25] = vec.try_into().unwrap();
        let arr_ref = &arr;
        
        let t_18 = &t_13;
        let _ = t_18 == arr_ref;
        
        let mut iter = t_13.iter_mut();
        while let Some(elem) = iter.next() {
            elem.0.push('!');
        }
        let _ = t_13.as_slice();
        let _ = t_13.as_mut_slice();
        println!("Capacity: {:?}", t_13.capacity());
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