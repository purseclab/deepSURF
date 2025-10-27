#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

#[derive(Debug)]
struct CustomType1(String);
struct CustomType2(String);
struct CustomType3(String);

impl core::iter::IntoIterator for CustomType2 {
    type Item = CustomType1;
    type IntoIter = CustomType3;
    
    fn into_iter(self) -> Self::IntoIter {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 91);
        let custom_impl_inst_num = self.0.len();
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0 {
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector {
            1 => global_data.first_half,
            _ => global_data.second_half,
        };
        let mut t_19 = _to_u8(GLOBAL_DATA, 99) % 17;
        let t_20 = _to_str(GLOBAL_DATA, 100, 100 + t_19 as usize);
        let t_21 = String::from(t_20);
        let t_22 = CustomType3(t_21);
        return t_22;
    }
}

impl core::iter::Iterator for CustomType3 {
    type Item = CustomType1;
    
    fn size_hint(&self) -> (usize, Option<usize>) {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 42);
        let custom_impl_inst_num = self.0.len();
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0 {
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector {
            1 => global_data.first_half,
            _ => global_data.second_half,
        };
        let t_10 = _to_usize(GLOBAL_DATA, 50);
        let t_11 = _to_usize(GLOBAL_DATA, 58);
        let t_12 = Some(t_11);
        let t_13 = (t_10, t_12);
        return t_13;
    }
    
    fn next(&mut self) -> Option<Self::Item> {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 66);
        let custom_impl_inst_num = self.0.len();
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0 {
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector {
            1 => global_data.first_half,
            _ => global_data.second_half,
        };
        let mut t_14 = _to_u8(GLOBAL_DATA, 74) % 17;
        let t_15 = _to_str(GLOBAL_DATA, 75, 75 + t_14 as usize);
        let t_16 = String::from(t_15);
        let t_17 = CustomType1(t_16);
        let t_18 = Some(t_17);
        return t_18;
    }
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 2048 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let mut index = 0;
        let constructor_selector = _to_u8(GLOBAL_DATA, index) % 3;
        index += 1;

        let mut smallvec = match constructor_selector {
            0 => SmallVec::<[CustomType1; 8]>::with_capacity(_to_usize(GLOBAL_DATA, index)),
            1 => {
                index += 8;
                let count = _to_usize(GLOBAL_DATA, index) % 65;
                index += 8;
                let mut elements = Vec::new();
                for _ in 0..count {
                    let str_len = _to_u8(GLOBAL_DATA, index) % 17;
                    index += 1;
                    let start = index;
                    index += str_len as usize;
                    elements.push(CustomType1(String::from(_to_str(GLOBAL_DATA, start, index))));
                }
                SmallVec::from_vec(elements)
            }
            _ => {
                index += 8;
                let count = _to_usize(GLOBAL_DATA, index) % 65;
                index += 8;
                let mut sv = SmallVec::with_capacity(count);
                for _ in 0..count {
                    let str_len = _to_u8(GLOBAL_DATA, index) % 17;
                    index += 1;
                    let start = index;
                    index += str_len as usize;
                    sv.push(CustomType1(String::from(_to_str(GLOBAL_DATA, start, index))));
                }
                sv
            }
        };

        index += 8;
        for _ in 0..(_to_u8(GLOBAL_DATA, index) % 8) {
            index += 1;
            match _to_u8(GLOBAL_DATA, index) % 3 {
                0 => {
                    index += 1;
                    let str_len = _to_u8(GLOBAL_DATA, index) % 17;
                    index += 1;
                    smallvec.push(CustomType1(String::from(_to_str(GLOBAL_DATA, index, index + str_len as usize))));
                    index += str_len as usize;
                }
                1 => { smallvec.pop(); }
                _ => {
                    index += 8;
                    let idx = _to_usize(GLOBAL_DATA, index) % (smallvec.len() + 1);
                    index += 1;
                    let str_len = _to_u8(GLOBAL_DATA, index) % 17;
                    index += 1;
                    smallvec.insert(idx, CustomType1(String::from(_to_str(GLOBAL_DATA, index, index + str_len as usize))));
                    index += str_len as usize;
                }
            }
        }

        index += 8;
        let pos = _to_usize(GLOBAL_DATA, index) % (smallvec.len() + 1);
        index += 8;
        let iter_selector = _to_u8(GLOBAL_DATA, index) % 2;
        index += 1;

        let iter: Box<dyn Iterator<Item = CustomType1>> = match iter_selector {
            0 => {
                let str_len = _to_u8(GLOBAL_DATA, index) % 17;
                index += 1;
                let s = String::from(_to_str(GLOBAL_DATA, index, index + str_len as usize));
                index += str_len as usize;
                Box::new(CustomType2(s).into_iter())
            }
            _ => {
                let count = _to_usize(GLOBAL_DATA, index) % 65;
                index += 8;
                let mut items = Vec::new();
                for _ in 0..count {
                    let str_len = _to_u8(GLOBAL_DATA, index) % 17;
                    index += 1;
                    let s = String::from(_to_str(GLOBAL_DATA, index, index + str_len as usize));
                    index += str_len as usize;
                    items.push(CustomType1(s));
                }
                Box::new(items.into_iter())
            }
        };

        smallvec.insert_many(pos, iter);

        for _ in 0..(_to_u8(GLOBAL_DATA, index) % 4) {
            index += 1;
            match _to_u8(GLOBAL_DATA, index) % 3 {
                0 => {
                    index += 8;
                    let range = _to_usize(GLOBAL_DATA, index) % smallvec.len();
                    smallvec.drain(range..range + 1);
                }
                1 => smallvec.truncate(_to_usize(GLOBAL_DATA, index) % (smallvec.len() + 1)),
                _ => smallvec.reserve(_to_usize(GLOBAL_DATA, index))
            }
        }

        println!("{:?}", smallvec.as_slice());
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