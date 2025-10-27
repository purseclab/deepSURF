#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::ops::{Deref, DerefMut, Index, IndexMut};

struct CustomType2(String);
struct CustomType3(String);

impl std::iter::IntoIterator for CustomType2 {
    type Item = String;
    type IntoIter = CustomType3;
    
    fn into_iter(self) -> Self::IntoIter {
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
        let mut t_15 = _to_u8(GLOBAL_DATA, 8) % 17;
        let t_16 = _to_str(GLOBAL_DATA, 9, 9 + t_15 as usize);
        CustomType3(String::from(t_16))
    }
}

impl std::iter::Iterator for CustomType3 {
    type Item = String;
    
    fn next(&mut self) -> Option<Self::Item> {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 50);
        let custom_impl_inst_num = self.0.len();
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0{
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector{
                    1 => global_data.first_half,
                    _ => global_data.second_half,
        };
        let mut t_6 = _to_u8(GLOBAL_DATA, 58) % 17;
        Some(_to_str(GLOBAL_DATA, 59, 59 + t_6 as usize).to_string())
    }
    
    fn size_hint(&self) -> (usize, Option<usize>) {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let t_11 = _to_usize(GLOBAL_DATA, 76);
        let t_12 = _to_usize(GLOBAL_DATA, 84);
        (t_11, Some(t_12))
    }
}

fn main (){
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 512 {return;}
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let ops = _to_usize(GLOBAL_DATA, 0) % 8;
        for idx in 0..ops {
            let op_selector = _to_u8(GLOBAL_DATA, idx * 8) % 6;
            let offset = idx * 8 + 8;

            match op_selector {
                0 => {
                    let mut sv = SmallVec::<[String; 64]>::new();
                    let elem_count = _to_usize(GLOBAL_DATA, offset) % 65;
                    for i in 0..elem_count {
                        let str_len = _to_u8(GLOBAL_DATA, offset + i + 1) % 17;
                        let s = _to_str(GLOBAL_DATA, offset + i + 2, offset + i + 2 + str_len as usize);
                        sv.push(s.to_string());
                    }
                }
                1 => {
                    let capacity = _to_usize(GLOBAL_DATA, offset);
                    let sv = SmallVec::<[u8; 64]>::with_capacity(capacity);
                    let _ = sv.capacity();
                }
                2 => {
                    let slice_len = _to_usize(GLOBAL_DATA, offset) % 65;
                    let slice = &global_data.second_half[..slice_len];
                    let sv = SmallVec::<[u8; 64]>::from_slice(slice);
                    let _ = sv.as_slice().len();
                }
                3 => {
                    let elem = _to_u8(GLOBAL_DATA, offset);
                    let count = _to_usize(GLOBAL_DATA, offset + 1) % 65;
                    let sv = SmallVec::<[u8; 64]>::from_elem(elem, count);
                    println!("{:?}", sv.last());
                }
                4 => {
                    let mut t_19 = _to_u8(GLOBAL_DATA, offset) % 17;
                    let t_22 = CustomType2(_to_str(GLOBAL_DATA, offset + 1, offset + 1 + t_19 as usize).to_string());
                    let sv = SmallVec::<[String; 64]>::from_iter(t_22);
                    let _ = sv.get(0).map(|s| println!("{}", s));
                }
                5 => {
                    let mut sv = SmallVec::<[usize; 64]>::new();
                    let elem = _to_usize(GLOBAL_DATA, offset);
                    sv.push(elem);
                    sv.truncate(_to_usize(GLOBAL_DATA, offset + 8) % (sv.len() + 1));
                }
                _ => {}
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