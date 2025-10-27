#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::ops::{Deref, DerefMut};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct CustomType1(String);
struct CustomType2(String);
struct CustomType3(String);

impl core::iter::Iterator for CustomType3 {
    type Item = CustomType1;
    
    fn size_hint(&self) -> (usize, Option<usize>) {
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
        (_to_usize(GLOBAL_DATA, 17), Some(_to_usize(GLOBAL_DATA, 25)))
    }
    
    fn next(&mut self) -> Option<Self::Item> {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 33);
        let custom_impl_inst_num = self.0.len();
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0 {
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector {
            1 => global_data.first_half,
            _ => global_data.second_half,
        };
        let len = _to_u8(GLOBAL_DATA, 41) % 17;
        let s = _to_str(GLOBAL_DATA, 42, 42 + len as usize);
        Some(CustomType1(s.to_string()))
    }
}

impl core::iter::IntoIterator for CustomType2 {
    type Item = CustomType1;
    type IntoIter = CustomType3;
    
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
        let len = _to_u8(GLOBAL_DATA, 66) % 17;
        let s = _to_str(GLOBAL_DATA, 67, 67 + len as usize);
        CustomType3(s.to_string())
    }
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 1000 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let mut offset = 0;

        let num_operations = _to_u8(GLOBAL_DATA, offset) % 10;
        offset += 1;

        let mut vectors = Vec::new();
        for _ in 0..num_operations {
            let op_code = _to_u8(GLOBAL_DATA, offset) % 6;
            offset += 1;

            match op_code {
                0 => {
                    let capacity = _to_usize(GLOBAL_DATA, offset);
                    offset += 8;
                    vectors.push(SmallVec::<[CustomType1; 32]>::with_capacity(capacity));
                }
                1 => {
                    let count = _to_usize(GLOBAL_DATA, offset) % 65;
                    offset += 8;
                    let elem = CustomType1(_to_str(GLOBAL_DATA, offset, offset + 8).to_string());
                    offset += 8;
                    vectors.push(SmallVec::from_elem(elem, count));
                }
                2 => {
                    if let Some(mut sv) = vectors.pop() {
                        let push_count = _to_u8(GLOBAL_DATA, offset) % 10;
                        offset += 1;
                        for _ in 0..push_count {
                            let s = _to_str(GLOBAL_DATA, offset, offset + 8);
                            sv.push(CustomType1(s.to_string()));
                            offset += 8;
                        }
                        vectors.push(sv);
                    }
                }
                3 => {
                    if vectors.len() >= 2 {
                        let a = &vectors[0];
                        let b = &vectors[1];
                        let cmp_result = a.cmp(b);
                        println!("Comparison: {:?}", cmp_result);
                    }
                }
                4 => {
                    if let Some(mut sv) = vectors.pop() {
                        let index = _to_usize(GLOBAL_DATA, offset) % (sv.len() + 1);
                        offset += 8;
                        sv.truncate(index);
                        vectors.push(sv);
                    }
                }
                5 => {
                    let s = _to_str(GLOBAL_DATA, offset, offset + 16);
                    offset += 16;
                    let iter_source = CustomType2(s.to_string());
                    vectors.push(SmallVec::<[CustomType1; 32]>::from_iter(iter_source));
                }
                _ => {}
            }
        }

        let main_str = _to_str(GLOBAL_DATA, offset, offset + 64);
        let main_iter = CustomType2(main_str.to_string());
        let _ = SmallVec::<[CustomType1; 32]>::from_iter(main_iter);
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