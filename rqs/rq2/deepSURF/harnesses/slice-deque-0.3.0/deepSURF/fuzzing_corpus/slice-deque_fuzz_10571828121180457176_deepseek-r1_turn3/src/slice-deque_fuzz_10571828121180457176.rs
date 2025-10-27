#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use slice_deque::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut};

#[derive(Debug, PartialEq)]
struct CustomType0(String);

impl std::clone::Clone for CustomType0 {
    fn clone(&self) -> Self {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 1);
        let custom_impl_inst_num = self.0.len();
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0 {
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector {
            1 => global_data.first_half,
            _ => global_data.second_half,
        };
        let mut t_2 = _to_u8(GLOBAL_DATA, 9) % 17;
        let t_3 = _to_str(GLOBAL_DATA, 10, 10 + t_2 as usize);
        let t_4 = String::from(t_3);
        CustomType0(t_4)
    }
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 5000 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let mut offset = 0;

        let constructor_selector = _to_u8(GLOBAL_DATA, offset) % 4;
        offset += 1;
        let mut deq = match constructor_selector {
            0 => SliceDeque::new(),
            1 => {
                let cap = _to_usize(GLOBAL_DATA, offset) % 65;
                offset += 8;
                SliceDeque::with_capacity(cap)
            }
            2 => {
                let elem_count = _to_u8(GLOBAL_DATA, offset) % 32;
                offset += 1;
                let mut vec = Vec::new();
                for _ in 0..elem_count {
                    let len = _to_u8(GLOBAL_DATA, offset) % 17;
                    offset += 1;
                    let s = _to_str(GLOBAL_DATA, offset, offset + len as usize);
                    offset += len as usize;
                    vec.push(CustomType0(s.to_string()));
                }
                SliceDeque::from(vec.as_slice())
            }
            3 => {
                let len = _to_usize(GLOBAL_DATA, offset);
                offset += 8;
                let str_len = _to_u8(GLOBAL_DATA, offset) % 17;
                offset += 1;
                let s = _to_str(GLOBAL_DATA, offset, offset + str_len as usize);
                offset += str_len as usize;
                slice_deque::from_elem(CustomType0(s.to_string()), len)
            }
            _ => unreachable!(),
        };

        let op_count = _to_u8(GLOBAL_DATA, offset) % 20;
        offset += 1;

        for _ in 0..op_count {
            let op = _to_u8(GLOBAL_DATA, offset) % 9;
            offset += 1;

            match op {
                0 => {
                    let len = _to_u8(GLOBAL_DATA, offset) % 17;
                    offset += 1;
                    let s = _to_str(GLOBAL_DATA, offset, offset + len as usize);
                    offset += len as usize;
                    deq.push_back(CustomType0(s.to_string()));
                }
                1 => {
                    let len = _to_u8(GLOBAL_DATA, offset) % 17;
                    offset += 1;
                    let s = _to_str(GLOBAL_DATA, offset, offset + len as usize);
                    offset += len as usize;
                    deq.push_front(CustomType0(s.to_string()));
                }
                2 => {
                    deq.pop_back();
                }
                3 => {
                    deq.pop_front();
                }
                4 => {
                    let new_len = _to_usize(GLOBAL_DATA, offset) % (deq.len().saturating_add(1));
                    offset += 8;
                    deq.truncate(new_len);
                }
                5 => {
                    let new_len = _to_usize(GLOBAL_DATA, offset);
                    offset += 8;
                    let len = _to_u8(GLOBAL_DATA, offset) % 17;
                    offset += 1;
                    let s = _to_str(GLOBAL_DATA, offset, offset + len as usize);
                    offset += len as usize;
                    deq.resize(new_len, CustomType0(s.to_string()));
                }
                6 => {
                    if deq.len() >= 5 {
                        let mut vec = Vec::with_capacity(5);
                        for _ in 0..5 {
                            let len = _to_u8(GLOBAL_DATA, offset) % 17;
                            offset += 1;
                            let s = _to_str(GLOBAL_DATA, offset, offset + len as usize);
                            offset += len as usize;
                            vec.push(CustomType0(s.to_string()));
                        }
                        let arr: [CustomType0; 5] = vec.try_into().unwrap();
                        let _ = &deq == &arr;
                    }
                }
                7 => {
                    let (s1, s2) = deq.as_slices();
                    println!("{:?} {:?}", s1, s2);
                }
                8 => {
                    let mut slice = deq.as_mut_slice();
                    if !slice.is_empty() {
                        let idx = _to_usize(GLOBAL_DATA, offset) % slice.len();
                        offset += 8;
                        let len = _to_u8(GLOBAL_DATA, offset) % 17;
                        offset += 1;
                        let s = _to_str(GLOBAL_DATA, offset, offset + len as usize);
                        offset += len as usize;
                        slice[idx] = CustomType0(s.to_string());
                    }
                }
                _ => (),
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