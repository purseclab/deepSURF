#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use stackvector::*;
use global_data::*;
use std::ops::{Deref, DerefMut};

type MyArray = [CustomType1; 256];

#[derive(Debug, Clone, PartialEq, PartialOrd)]
struct CustomType1(String);
struct CustomType2(String);
struct CustomType3(String);

impl std::iter::IntoIterator for CustomType2 {
    type Item = CustomType1;
    type IntoIter = CustomType3;
    
    fn into_iter(self) -> Self::IntoIter {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 90);
        let custom_impl_inst_num = self.0.len();
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0{
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector{
            1 => global_data.first_half,
            _ => global_data.second_half,
        };
        let mut t_15 = _to_u8(GLOBAL_DATA, 98) % 17;
        let t_16 = _to_str(GLOBAL_DATA, 99, 99 + t_15 as usize);
        let t_17 = String::from(t_16);
        let t_18 = CustomType3(t_17);
        t_18
    }
}

impl std::iter::Iterator for CustomType3 {
    type Item = CustomType1;
    
    fn next(&mut self) -> Option<Self::Item> {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 65);
        let custom_impl_inst_num = self.0.len();
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0{
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector{
            1 => global_data.first_half,
            _ => global_data.second_half,
        };
        let mut t_10 = _to_u8(GLOBAL_DATA, 73) % 17;
        let t_11 = _to_str(GLOBAL_DATA, 74, 74 + t_10 as usize);
        let t_12 = String::from(t_11);
        Some(CustomType1(t_12))
    }
}

fn _custom_fn0(a: &mut CustomType1, b: &mut CustomType1) -> bool {
    let global_data = get_global_data();
    let GLOBAL_DATA = global_data.first_half;
    let t_25 = _to_u8(GLOBAL_DATA, 132);
    if t_25 % 2 == 0{
        panic!("INTENTIONAL PANIC!");
    }
    let t_26 = _to_bool(GLOBAL_DATA, 133);
    t_26
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 1024 {return;}
        set_global_data(data);
        let global_data = get_global_data();
        let first_half = global_data.first_half;
        let second_half = global_data.second_half;

        let mut vec = match _to_u8(first_half, 0) % 4 {
            0 => StackVec::<MyArray>::new(),
            1 => {
                let count = _to_usize(first_half, 1) % 65;
                let mut items = Vec::new();
                let mut pos = 2;
                for _ in 0..count {
                    let len = _to_u8(first_half, pos) % 63;
                    let s = _to_str(first_half, pos+1, pos+1+len as usize);
                    items.push(CustomType1(s.to_string()));
                    pos += 1 + len as usize;
                }
                StackVec::from_vec(items)
            }
            2 => {
                let elem_len = _to_u8(first_half, 1) % 63;
                let elem_str = _to_str(first_half, 2, 2 + elem_len as usize);
                StackVec::from_elem(CustomType1(elem_str.to_string()), _to_usize(first_half, 2+elem_len as usize) % 65)
            }
            _ => {
                let str_len = _to_u8(first_half, 1) % 63;
                let s = _to_str(first_half, 2, 2+str_len as usize);
                StackVec::<MyArray>::from_iter(CustomType2(s.to_string()))
            }
        };

        let op_count = _to_u8(second_half, 0) % 16;
        let mut op_pos = 1;
        
        for _ in 0..op_count {
            if op_pos >= second_half.len() {break;}
            match second_half[op_pos] % 7 {
                0 => {
                    if let Some(x) = vec.pop() {
                        println!("{:?}", x);
                    }
                    op_pos += 1;
                }
                1 => {
                    let str_len = _to_u8(second_half, op_pos+1) % 63;
                    let s = _to_str(second_half, op_pos+2, op_pos+2+str_len as usize);
                    vec.push(CustomType1(s.to_string()));
                    op_pos += 2 + str_len as usize;
                }
                2 => {
                    let idx = _to_usize(second_half, op_pos+1);
                    let str_len = _to_u8(second_half, op_pos+9) % 63;
                    let s = _to_str(second_half, op_pos+10, op_pos+10+str_len as usize);
                    vec.insert(idx, CustomType1(s.to_string()));
                    op_pos += 10 + str_len as usize;
                }
                3 => {
                    let idx = _to_usize(second_half, op_pos+1);
                    if !vec.is_empty() {
                        vec.swap_remove(idx % vec.len());
                    }
                    op_pos += 9;
                }
                4 => {
                    vec.dedup_by(_custom_fn0);
                    op_pos += 1;
                }
                5 => {
                    let new_len = _to_usize(second_half, op_pos+1);
                    vec.truncate(new_len);
                    op_pos += 9;
                }
                _ => {
                    let other = vec.clone();
                    println!("{:?}", vec.partial_cmp(&other));
                    op_pos += 1;
                }
            }
        }

        vec.dedup_by(|a,b| _custom_fn0(a, b));
        println!("{:?}", vec.as_slice());
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