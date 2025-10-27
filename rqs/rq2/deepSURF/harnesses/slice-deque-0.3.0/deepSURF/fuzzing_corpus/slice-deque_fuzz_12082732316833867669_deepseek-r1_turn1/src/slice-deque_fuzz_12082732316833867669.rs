#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use slice_deque::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

#[derive(Debug)]
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
        if data.len() < 4000 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        
        let constructor_selector = _to_u8(GLOBAL_DATA, 0);
        let mut t_135 = match constructor_selector % 3 {
            0 => {
                let mut t_1 = Vec::new();
                for i in 0..32 {
                    let offset = 26 + i * 17;
                    let t_len = _to_u8(GLOBAL_DATA, offset) % 17;
                    let s_start = offset + 1;
                    let s_str = _to_str(GLOBAL_DATA, s_start, s_start + t_len as usize);
                    t_1.push(CustomType0(String::from(s_str)));
                }
                slice_deque::SliceDeque::from(&mut t_1[..])
            }
            1 => {
                let capacity = _to_usize(GLOBAL_DATA, 1) % 65;
                let mut deque = slice_deque::SliceDeque::with_capacity(capacity);
                for i in 0..capacity {
                    let offset = 200 + i * 17;
                    let t_len = _to_u8(GLOBAL_DATA, offset) % 17;
                    let s_str = _to_str(GLOBAL_DATA, offset + 1, offset + 1 + t_len as usize);
                    deque.push_back(CustomType0(String::from(s_str)));
                }
                deque
            }
            _ => {
                let elem = {
                    let t_len = _to_u8(GLOBAL_DATA, 100) % 17;
                    let s_str = _to_str(GLOBAL_DATA, 101, 101 + t_len as usize);
                    CustomType0(String::from(s_str))
                };
                let count = _to_usize(GLOBAL_DATA, 150) % 65;
                slice_deque::from_elem(elem, count)
            }
        };

        let op_count = _to_usize(GLOBAL_DATA, 2) % 20;
        for op_idx in 0..op_count {
            let op_byte = _to_u8(GLOBAL_DATA, 3 + op_idx) % 8;
            match op_byte {
                0 => {
                    let s_len = _to_u8(GLOBAL_DATA, 500 + op_idx) % 17;
                    let s_str = _to_str(GLOBAL_DATA, 501 + 500 + op_idx, 501 + 500 + op_idx + s_len as usize);
                    t_135.push_back(CustomType0(String::from(s_str)));
                }
                1 => {
                    t_135.pop_front();
                }
                2 => {
                    let new_len = _to_usize(GLOBAL_DATA, 600 + op_idx) % (t_135.len() + 1);
                    t_135.truncate(new_len);
                }
                3 => {
                    let slice = t_135.as_mut();
                    for elem in slice {
                        println!("{:?}", elem);
                    }
                }
                4 => {
                    let s_idx = _to_usize(GLOBAL_DATA, 700 + op_idx) % (t_135.len() + 1);
                    let e_idx = s_idx + _to_usize(GLOBAL_DATA, 701 + op_idx) % (t_135.len() - s_idx + 1);
                    let new_elems = (0..2).map(|i| {
                        let offset = 800 + op_idx * 10 + i * 17;
                        let s_len = _to_u8(GLOBAL_DATA, offset) % 17;
                        let s_str = _to_str(GLOBAL_DATA, offset + 1, offset + 1 + s_len as usize);
                        CustomType0(String::from(s_str))
                    }).collect::<Vec<_>>();
                    t_135.splice(s_idx..e_idx, new_elems);
                }
                5 => {
                    let filter = |x: &mut CustomType0| x.0.len() % 2 == 0;
                    let _drain = t_135.drain_filter(filter);
                }
                6 => {
                    let mut new_deque = slice_deque::SliceDeque::new();
                    new_deque.extend(t_135.as_slice().iter().cloned());
                    t_135.append(&mut new_deque);
                }
                _ => {
                    let idx = _to_usize(GLOBAL_DATA, 900 + op_idx) % (t_135.len() + 1);
                    if !t_135.is_empty() {
                        t_135.remove(idx % t_135.len());
                    }
                }
            }
        }

        let _ = t_135.as_mut();
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