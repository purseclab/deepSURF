#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use stackvector::*;
use global_data::*;
use std::ops::{Deref, DerefMut, Index, IndexMut};

#[derive(Debug, Clone)]
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
        if selector == 0 {
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector {
            1 => global_data.first_half,
            _ => global_data.second_half,
        };
        let mut t_15 = _to_u8(GLOBAL_DATA, 98) % 17;
        let t_16 = _to_str(GLOBAL_DATA, 99, 99 + t_15 as usize);
        let t_17 = String::from(t_16);
        CustomType3(t_17)
    }
}

impl std::iter::Iterator for CustomType3 {
    type Item = CustomType1;

    fn size_hint(&self) -> (usize, Option<usize>) {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 41);
        let custom_impl_inst_num = self.0.len();
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0 {
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector {
            1 => global_data.first_half,
            _ => global_data.second_half,
        };
        let t_6 = _to_usize(GLOBAL_DATA, 49);
        let t_7 = _to_usize(GLOBAL_DATA, 57);
        (t_6, Some(t_7))
    }

    fn next(&mut self) -> Option<Self::Item> {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 65);
        let custom_impl_inst_num = self.0.len();
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0 {
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector {
            1 => global_data.first_half,
            _ => global_data.second_half,
        };
        let mut t_10 = _to_u8(GLOBAL_DATA, 73) % 17;
        let t_11 = _to_str(GLOBAL_DATA, 74, 74 + t_10 as usize);
        let t_12 = String::from(t_11);
        Some(CustomType1(t_12))
    }
}

#[derive(Debug)]
struct StackVecWrapper(StackVec<[CustomType1; 64]>);

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 2048 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let mut offset = 0;

        let constructor_selector = _to_u8(GLOBAL_DATA, offset) % 4;
        offset += 1;

        let mut stack_vec = match constructor_selector {
            0 => {
                let t_19 = _to_u8(GLOBAL_DATA, offset) % 17;
                offset += 1;
                let start_str = offset;
                offset += t_19 as usize;
                let t_20 = _to_str(GLOBAL_DATA, start_str, offset);
                StackVecWrapper(StackVec::from_iter(CustomType2(t_20.to_string())))
            },
            1 => {
                let slice_len = _to_usize(GLOBAL_DATA, offset) % 65;
                offset += 8;
                let mut elements = vec![];
                for _ in 0..slice_len {
                    let elem_len = _to_u8(GLOBAL_DATA, offset) % 17;
                    offset += 1;
                    let start = offset;
                    offset += elem_len as usize;
                    let s = _to_str(GLOBAL_DATA, start, offset);
                    elements.push(CustomType1(s.to_string()));
                }
                StackVecWrapper(StackVec::from_vec(elements))
            },
            2 => {
                let elem_len = _to_u8(GLOBAL_DATA, offset) % 17;
                offset += 1;
                let start = offset;
                offset += elem_len as usize;
                let s = _to_str(GLOBAL_DATA, start, offset);
                let count = _to_usize(GLOBAL_DATA, offset);
                offset += 8;
                StackVecWrapper(StackVec::from_elem(CustomType1(s.to_string()), count))
            },
            _ => StackVecWrapper(StackVec::new()),
        };

        let op_count = _to_usize(GLOBAL_DATA, offset) % 10;
        offset += 8;

        for _ in 0..op_count {
            let op_selector = _to_u8(GLOBAL_DATA, offset) % 8;
            offset += 1;

            match op_selector {
                0 => {
                    let elem_len = _to_u8(GLOBAL_DATA, offset) % 17;
                    offset += 1;
                    let start = offset;
                    offset += elem_len as usize;
                    let s = _to_str(GLOBAL_DATA, start, offset);
                    stack_vec.0.push(CustomType1(s.to_string()));
                },
                1 => {
                    stack_vec.0.pop();
                },
                2 => {
                    let index = _to_usize(GLOBAL_DATA, offset);
                    offset += 8;
                    let elem_len = _to_u8(GLOBAL_DATA, offset) % 17;
                    offset += 1;
                    let start = offset;
                    offset += elem_len as usize;
                    let s = _to_str(GLOBAL_DATA, start, offset);
                    stack_vec.0.insert(index, CustomType1(s.to_string()));
                },
                3 => {
                    let len = _to_usize(GLOBAL_DATA, offset);
                    offset += 8;
                    stack_vec.0.truncate(len);
                },
                4 => {
                    let mut drain = stack_vec.0.drain();
                    while let Some(item) = drain.next() {
                        println!("{:?}", item.0);
                    }
                },
                5 => {
                    let index = _to_usize(GLOBAL_DATA, offset);
                    offset += 8;
                    let _ = &stack_vec.0[index];
                },
                6 => {
                    let slice = stack_vec.0.as_slice();
                    println!("{:?}", slice);
                },
                _ => {
                    let mut new_vec = StackVec::new();
                    std::mem::swap(&mut stack_vec.0, &mut new_vec);
                },
            }
        }

        let _ = stack_vec.0.into_iter();
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