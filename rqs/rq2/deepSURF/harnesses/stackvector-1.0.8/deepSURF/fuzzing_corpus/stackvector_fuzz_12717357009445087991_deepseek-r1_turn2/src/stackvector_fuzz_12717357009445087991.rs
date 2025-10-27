#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use stackvector::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

struct CustomType3(String);

#[derive(Clone, Debug, PartialEq, PartialOrd)]
struct CustomType1(String);

struct CustomType2(String);

impl Iterator for CustomType3 {
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
        Some(CustomType1(t_11.to_string()))
    }
}

impl IntoIterator for CustomType2 {
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
        CustomType3(_to_str(GLOBAL_DATA, 99, 99 + t_15 as usize).to_string())
    }
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 4096 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        
        let mut stack_vec = match _to_u8(global_data.first_half, 0) % 4 {
            0 => StackVec::<[CustomType1; 64]>::new(),
            1 => {
                let count = _to_usize(global_data.first_half, 1) % 65;
                let mut items = Vec::new();
                for i in 0..count {
                    let len = _to_u8(global_data.first_half, 10 + i * 12) % 17;
                    let start = 10 + i * 12 + 1;
                    let s = _to_str(global_data.first_half, start, start + len as usize);
                    items.push(CustomType1(s.to_string()));
                }
                StackVec::from_vec(items)
            }
            2 => {
                let elem_len = _to_u8(global_data.first_half, 200) % 17;
                let elem = CustomType1(_to_str(global_data.first_half, 201, 201 + elem_len as usize).to_string());
                StackVec::from_elem(elem, _to_usize(global_data.first_half, 300) % 65)
            }
            _ => {
                let iter_len = _to_u8(global_data.first_half, 400) % 17;
                let iter_str = _to_str(global_data.first_half, 401, 401 + iter_len as usize).to_string();
                StackVec::from_iter(CustomType2(iter_str))
            }
        };

        let op_count = _to_usize(global_data.second_half, 0) % 65;
        for i in 0..op_count {
            let op_selector = _to_u8(global_data.second_half, 1 + i * 8) % 8;
            match op_selector {
                0 => {
                    let len = _to_u8(global_data.second_half, 2 + i * 8) % 17;
                    let start = 3 + i * 8;
                    let s = _to_str(global_data.second_half, start, start + len as usize);
                    stack_vec.push(CustomType1(s.to_string()));
                    println!("{:?}", &*stack_vec);
                }
                1 => { stack_vec.pop(); }
                2 => stack_vec.truncate(_to_usize(global_data.second_half, 4 + i * 8)),
                3 => {
                    stack_vec.insert(
                        _to_usize(global_data.second_half, 5 + i * 8),
                        CustomType1(_to_str(global_data.second_half, 6 + i * 8, 7 + i * 8 + 12).to_string())
                    );
                }
                4 => { stack_vec.swap_remove(_to_usize(global_data.second_half, 6 + i * 8)); }
                5 => {
                    let mut cloned = stack_vec.clone();
                    let _ = stack_vec.partial_cmp(&cloned);
                }
                6 => {
                    let mut drain = stack_vec.drain();
                    while let Some(item) = drain.next() {
                        println!("{:?}", item.0);
                    }
                }
                _ => {
                    let slice = stack_vec.as_mut_slice();
                    if !slice.is_empty() {
                        println!("First element: {:?}", slice[0].0);
                    }
                }
            }
        }

        let final_push_len = _to_u8(global_data.first_half, 500) % 17;
        let final_push_str = _to_str(global_data.first_half, 501, 501 + final_push_len as usize);
        stack_vec.push(CustomType1(final_push_str.to_string()));
        println!("Final len: {}", stack_vec.len());
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