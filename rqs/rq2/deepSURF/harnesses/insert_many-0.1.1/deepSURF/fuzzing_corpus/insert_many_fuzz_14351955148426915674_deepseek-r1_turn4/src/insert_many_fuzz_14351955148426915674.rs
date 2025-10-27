#![forbid(unsafe_code)]
#![feature(rustc_private)]

#[macro_use]
extern crate afl;
extern crate smallvec;

use insert_many::*;
use global_data::*;
use smallvec::SmallVec;
use std::ops::{Deref, DerefMut, Index, IndexMut};

#[derive(Debug)]
struct CustomType0(String);
#[derive(Debug)]
struct CustomType2(String);
#[derive(Debug)]
struct CustomType1(String);

impl Iterator for CustomType2 {
    type Item = CustomType0;

    fn next(&mut self) -> Option<Self::Item> {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 569);
        let custom_impl_inst_num = self.0.len();
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0 {
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector {
            1 => global_data.first_half,
            _ => global_data.second_half,
        };
        let t_133 = _to_u8(GLOBAL_DATA, 577) % 17;
        let t_134 = _to_str(GLOBAL_DATA, 578, 578 + t_133 as usize);
        Some(CustomType0(t_134.to_string()))
    }
}

impl ExactSizeIterator for CustomType2 {
    fn len(&self) -> usize {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 553);
        let custom_impl_inst_num = self.0.len();
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0 {
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector {
            1 => global_data.first_half,
            _ => global_data.second_half,
        };
        _to_usize(GLOBAL_DATA, 561)
    }
}

impl IntoIterator for CustomType1 {
    type Item = CustomType0;
    type IntoIter = CustomType2;

    fn into_iter(self) -> Self::IntoIter {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 594);
        let custom_impl_inst_num = self.0.len();
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0 {
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector {
            1 => global_data.first_half,
            _ => global_data.second_half,
        };
        let t_138 = _to_u8(GLOBAL_DATA, 602) % 17;
        let t_139 = _to_str(GLOBAL_DATA, 603, 603 + t_138 as usize);
        CustomType2(t_139.to_string())
    }
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 4096 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let operations = _to_usize(global_data.first_half, 0) % 65;
        let mut offset = std::mem::size_of::<usize>();

        for _ in 0..operations {
            if offset + 3 > global_data.first_half.len() { break; }
            let container_type = _to_u8(global_data.first_half, offset) % 2;
            offset += 1;
            let init_elements = _to_usize(global_data.first_half, offset) % 65;
            offset += 1;

            match container_type {
                0 => {
                    let mut vec_container: Vec<CustomType0> = Vec::with_capacity(init_elements);
                    for _ in 0..init_elements {
                        if offset + 2 > global_data.first_half.len() { break; }
                        let len = _to_u8(global_data.first_half, offset) % 17;
                        offset += 1;
                        let start = offset;
                        let end = start + len as usize;
                        if end > global_data.first_half.len() { break; }
                        let s = _to_str(global_data.first_half, start, end);
                        vec_container.push(CustomType0(s.to_string()));
                        offset = end;
                    }

                    let trunc_idx = _to_usize(global_data.first_half, offset) % (vec_container.len() + 1);
                    offset += 1;
                    vec_container.truncate(trunc_idx);

                    match _to_u8(global_data.first_half, offset) % 2 {
                        0 => {
                            offset += 1;
                            let iter_len = _to_u8(global_data.first_half, offset) % 17;
                            offset += 1;
                            let start = offset;
                            let end = start + iter_len as usize;
                            if end > global_data.first_half.len() { break; }
                            let iter = CustomType1(_to_str(global_data.first_half, start, end).to_string()).into_iter();
                            offset = end;
                            let idx = if vec_container.is_empty() { 0 } else { _to_usize(global_data.first_half, offset) % vec_container.len() };
                            offset += 1;
                            vec_container.insert_many(idx, iter);
                        }
                        _ => {
                            offset += 1;
                            let count = _to_usize(global_data.first_half, offset) % 17;
                            offset += 1;
                            let mut items = Vec::new();
                            for _ in 0..count {
                                let len = _to_u8(global_data.first_half, offset) % 17;
                                offset += 1;
                                let start = offset;
                                let end = start + len as usize;
                                if end > global_data.first_half.len() { break; }
                                items.push(CustomType0(_to_str(global_data.first_half, start, end).to_string()));
                                offset = end;
                            }
                            let idx = if vec_container.is_empty() { 0 } else { _to_usize(global_data.first_half, offset) % vec_container.len() };
                            offset += 1;
                            vec_container.insert_many(idx, items.into_iter());
                        }
                    }

                    if !vec_container.is_empty() {
                        let access_idx = _to_usize(global_data.first_half, offset) % vec_container.len();
                        offset += 1;
                        println!("{:?}", vec_container.get(access_idx));
                    }

                    let new_capacity = _to_usize(global_data.first_half, offset) % 129;
                    offset += 1;
                    vec_container.reserve(new_capacity);
                }
                _ => {
                    let mut smallvec_container: SmallVec<[CustomType0; 32]> = SmallVec::new();
                    for _ in 0..init_elements {
                        if offset + 2 > global_data.first_half.len() { break; }
                        let len = _to_u8(global_data.first_half, offset) % 17;
                        offset += 1;
                        let start = offset;
                        let end = start + len as usize;
                        if end > global_data.first_half.len() { break; }
                        let s = _to_str(global_data.first_half, start, end);
                        smallvec_container.push(CustomType0(s.to_string()));
                        offset = end;
                    }

                    match _to_u8(global_data.first_half, offset) % 2 {
                        0 => {
                            offset += 1;
                            let iter_len = _to_u8(global_data.first_half, offset) % 17;
                            offset += 1;
                            let start = offset;
                            let end = start + iter_len as usize;
                            if end > global_data.first_half.len() { break; }
                            let iter = CustomType1(_to_str(global_data.first_half, start, end).to_string()).into_iter();
                            offset = end;
                            let idx = if smallvec_container.is_empty() { 0 } else { _to_usize(global_data.first_half, offset) % smallvec_container.len() };
                            offset += 1;
                            smallvec_container.insert_many(idx, iter);
                        }
                        _ => {
                            offset += 1;
                            let count = _to_usize(global_data.first_half, offset) % 17;
                            offset += 1;
                            let mut items = Vec::new();
                            for _ in 0..count {
                                let len = _to_u8(global_data.first_half, offset) % 17;
                                offset += 1;
                                let start = offset;
                                let end = start + len as usize;
                                if end > global_data.first_half.len() { break; }
                                items.push(CustomType0(_to_str(global_data.first_half, start, end).to_string()));
                                offset = end;
                            }
                            let idx = if smallvec_container.is_empty() { 0 } else { _to_usize(global_data.first_half, offset) % smallvec_container.len() };
                            offset += 1;
                            smallvec_container.insert_many(idx, items.into_iter());
                        }
                    }

                    if !smallvec_container.is_empty() {
                        let access_idx = _to_usize(global_data.first_half, offset) % smallvec_container.len();
                        offset += 1;
                        println!("{:?}", smallvec_container.get(access_idx));
                    }

                    let new_capacity = _to_usize(global_data.first_half, offset) % 129;
                    offset += 1;
                    smallvec_container.reserve(new_capacity);
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