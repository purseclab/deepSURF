#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::ops::{Deref, DerefMut, Index, IndexMut};

struct CustomType2(String);
#[derive(Debug)]
struct CustomType1(String);
struct CustomType3(String);

impl core::iter::Iterator for CustomType3 {
    type Item = CustomType1;
    
    fn next(&mut self) -> Option<Self::Item> {
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
        let mut t_9 = _to_u8(GLOBAL_DATA, 42) % 17;
        let t_10 = _to_str(GLOBAL_DATA, 43, 43 + t_9 as usize);
        let t_11 = String::from(t_10);
        let t_12 = CustomType1(t_11);
        Some(t_12)
    }
    
    fn size_hint(&self) -> (usize, Option<usize>) {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 59);
        let custom_impl_inst_num = self.0.len();
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0 {
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector {
            1 => global_data.first_half,
            _ => global_data.second_half,
        };
        let t_14 = _to_usize(GLOBAL_DATA, 67);
        let t_15 = _to_usize(GLOBAL_DATA, 75);
        (t_14, Some(t_15))
    }
}

impl core::iter::IntoIterator for CustomType2 {
    type Item = CustomType1;
    type IntoIter = CustomType3;
    
    fn into_iter(self) -> Self::IntoIter {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 83);
        let custom_impl_inst_num = self.0.len();
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0 {
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector {
            1 => global_data.first_half,
            _ => global_data.second_half,
        };
        let mut t_18 = _to_u8(GLOBAL_DATA, 91) % 17;
        let t_19 = _to_str(GLOBAL_DATA, 92, 92 + t_18 as usize);
        CustomType3(t_19.to_string())
    }
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 1000 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let constructor_selector = _to_u8(GLOBAL_DATA, 0) % 4;
        let mut vec: SmallVec<[CustomType1; 32]> = match constructor_selector {
            0 => SmallVec::new(),
            1 => SmallVec::with_capacity(_to_usize(GLOBAL_DATA, 1) % 65),
            2 => {
                let elem_count = _to_u8(GLOBAL_DATA, 2) % 65;
                let mut items = Vec::new();
                let mut idx = 3;
                for _ in 0..elem_count {
                    let len = _to_u8(GLOBAL_DATA, idx) % 17;
                    idx += 1;
                    let s = _to_str(GLOBAL_DATA, idx, idx + len as usize);
                    idx += len as usize;
                    items.push(CustomType1(s.to_string()));
                }
                SmallVec::from_vec(items)
            }
            _ => SmallVec::from_iter((0.._to_u8(GLOBAL_DATA, 4) % 65).map(|i| {
                let len = _to_u8(GLOBAL_DATA, 5 + i as usize) % 17;
                let s = _to_str(GLOBAL_DATA, 6 + i as usize * 2, 6 + i as usize * 2 + len as usize);
                CustomType1(s.to_string())
            }))
        };

        let op_count = _to_usize(GLOBAL_DATA, 200) % 20;
        for op_idx in 0..op_count {
            let selector = _to_u8(GLOBAL_DATA, 201 + op_idx) % 8;
            match selector {
                0 if !vec.is_empty() => {
                    let idx = _to_usize(GLOBAL_DATA, 300 + op_idx) % vec.len();
                    println!("Dereferencing: {:?}", *vec.get(idx).unwrap());
                }
                1 => {
                    let elem = _to_str(GLOBAL_DATA, 400 + op_idx, 400 + op_idx + 10);
                    vec.push(CustomType1(elem.to_string()));
                }
                2 => {
                    if vec.spilled() {
                        println!("Vector capacity after spill: {}", vec.capacity());
                    }
                }
                3 => {
                    vec.truncate(_to_usize(GLOBAL_DATA, 500 + op_idx) % (vec.len() + 1));
                }
                4 if vec.capacity() > 0 => {
                    vec.reserve(_to_usize(GLOBAL_DATA, 600 + op_idx));
                }
                5 => {
                    let mut iter = vec.drain(..);
                    while let Some(elem) = iter.next() {
                        println!("Draining: {:?}", elem.0);
                    }
                }
                6 if !vec.is_empty() => {
                    vec.remove(_to_usize(GLOBAL_DATA, 700 + op_idx) % vec.len());
                }
                _ => {
                    let len = _to_u8(GLOBAL_DATA, 800 + op_idx) % 17;
                    let s = _to_str(GLOBAL_DATA, 801 + op_idx * 2, 801 + op_idx * 2 + len as usize);
                    vec.extend(CustomType2(s.to_string()));
                }
            }
        }

        let len_ext = _to_u8(GLOBAL_DATA, 950) % 17;
        let range_end = 951 + len_ext as usize;
        let s_final = _to_str(GLOBAL_DATA, 951, range_end);
        vec.extend(CustomType2(s_final.to_string()));
        println!("Final vector length: {}", vec.len());
    });
}

// The type converter functions are omitted as per directions.

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