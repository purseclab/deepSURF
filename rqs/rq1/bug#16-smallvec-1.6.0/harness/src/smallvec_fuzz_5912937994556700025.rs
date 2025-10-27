#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

#[derive(Debug)]
struct CustomType3(String);
#[derive(Debug, Clone)]
struct CustomType1(String);
#[derive(Debug)]
struct CustomType2(String);

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
        (t_10, Some(t_11))
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
        Some(CustomType1(String::from(t_15)))
    }
}

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
        CustomType3(String::from(t_20))
    }
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 1024 {
            return;
        }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let constructor_selector = _to_u8(GLOBAL_DATA, 0) % 4;
        let mut sv = match constructor_selector {
            0 => SmallVec::<[CustomType1; 32]>::new(),
            1 => SmallVec::with_capacity(_to_usize(GLOBAL_DATA, 1)),
            2 => {
                let elem_len = _to_u8(GLOBAL_DATA, 10) % 17;
                let elem_str = _to_str(GLOBAL_DATA, 11, 11 + elem_len as usize);
                let elem = CustomType1(String::from(elem_str));
                SmallVec::from_elem(elem, _to_usize(GLOBAL_DATA, 28) % 65)
            }
            _ => {
                let slice_len = _to_usize(GLOBAL_DATA, 50) % 65;
                let mut elements = Vec::with_capacity(slice_len);
                for i in 0..slice_len {
                    let offset = 60 + i * 8;
                    let len = _to_u8(GLOBAL_DATA, offset) % 17;
                    let s = _to_str(GLOBAL_DATA, offset + 1, offset + 1 + len as usize);
                    elements.push(CustomType1(String::from(s)));
                }
                SmallVec::from_vec(elements)
            }
        };

        let num_ops = _to_usize(GLOBAL_DATA, 200) % 10 + 1;
        for i in 0..num_ops {
            let op_sel = _to_u8(GLOBAL_DATA, 201 + i) % 10;
            match op_sel {
                0 => {
                    let len = _to_u8(GLOBAL_DATA, 220 + i) % 17;
                    let s = _to_str(GLOBAL_DATA, 230 + i, 230 + i + len as usize);
                    sv.push(CustomType1(String::from(s)));
                }
                1 => {
                    let _ = sv.pop();
                }
                2 => {
                    let idx = _to_usize(GLOBAL_DATA, 250 + i);
                    let len = _to_u8(GLOBAL_DATA, 260 + i) % 17;
                    let s = _to_str(GLOBAL_DATA, 270 + i, 270 + i + len as usize);
                    sv.insert(idx, CustomType1(String::from(s)));
                }
                3 => {
                    let len = _to_usize(GLOBAL_DATA, 300 + i);
                    sv.truncate(len);
                }
                4 => sv.reserve(_to_usize(GLOBAL_DATA, 320 + i)),
                5 => {
                    let s = _to_str(GLOBAL_DATA, 340 + i, 340 + i + 32);
                    sv.extend(vec![CustomType1(String::from(s))]);
                }
                6 => {
                    let start = _to_usize(GLOBAL_DATA, 380 + i);
                    let end = _to_usize(GLOBAL_DATA, 390 + i);
                    let _ = sv.drain(start..end);
                }
                7 => {
                    let idx = _to_usize(GLOBAL_DATA, 400 + i);
                    let len = _to_u8(GLOBAL_DATA, 410 + i) % 17;
                    let s = _to_str(GLOBAL_DATA, 420 + i, 420 + i + len as usize);
                    let v = vec![CustomType1(String::from(s)); _to_usize(GLOBAL_DATA, 430 + i) % 8];
                    sv.insert_many(idx, v);
                }
                8 => {
                    let idx = _to_usize(GLOBAL_DATA, 500 + i);
                    let len = _to_u8(GLOBAL_DATA, 510 + i) % 17;
                    let s = _to_str(GLOBAL_DATA, 520 + i, 520 + i + len as usize);
                    let data = CustomType2(String::from(s));
                    sv.insert_many(idx, data);
                }
                _ => {
                    let new_cap = _to_usize(GLOBAL_DATA, 600 + i);
                    let _ = sv.try_grow(new_cap);
                }
            }
        }

        let idx = _to_usize(GLOBAL_DATA, 900);
        let len = _to_u8(GLOBAL_DATA, 901) % 17;
        let s = _to_str(GLOBAL_DATA, 902, 902 + len as usize);
        sv.insert_many(idx, CustomType2(String::from(s)));

        println!("Final vector state: {:?}", sv.as_slice());
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