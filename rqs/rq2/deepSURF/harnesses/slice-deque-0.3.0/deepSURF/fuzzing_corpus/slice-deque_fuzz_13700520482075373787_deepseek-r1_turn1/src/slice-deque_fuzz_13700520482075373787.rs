#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use slice_deque::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

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
        if data.len() < 2048 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        
        let mut deque = match _to_u8(global_data.first_half, 0) % 3 {
            0 => {
                let cap = _to_usize(global_data.first_half, 1) % 65;
                SliceDeque::with_capacity(cap)
            },
            1 => SliceDeque::new(),
            _ => {
                let elements = global_data.first_half[2..114].chunks(17).map(|c| {
                    let len = c[0] % 17;
                    let s = _to_str(c, 1, 1 + len as usize);
                    CustomType0(s.to_string())
                }).collect::<Vec<_>>();
                SliceDeque::from(&elements[..])
            }
        };

        let ops_data = &global_data.second_half;
        let num_ops = _to_usize(ops_data, 0) % 65;
        let mut data_idx = 8;

        for _ in 0..num_ops {
            if data_idx >= ops_data.len() { break; }
            let op_selector = _to_u8(ops_data, data_idx) % 7;
            data_idx += 1;

            match op_selector {
                0 => {
                    let len = ops_data.get(data_idx).copied().unwrap_or(0) % 17;
                    data_idx = data_idx.saturating_add(1);
                    let str_data = ops_data.get(data_idx..data_idx + len as usize).unwrap_or(&[]);
                    let s = String::from_utf8_lossy(str_data);
                    deque.push_back(CustomType0(s.to_string()));
                    data_idx = data_idx.saturating_add(len as usize);
                },
                1 => {
                    let idx = _to_usize(ops_data, data_idx);
                    data_idx = data_idx.saturating_add(8);
                    if let Some(e) = deque.get(idx) {
                        println!("{:?}", e.0);
                    }
                },
                2 => {
                    deque.truncate(_to_usize(ops_data, data_idx));
                    data_idx = data_idx.saturating_add(8);
                },
                3 => {
                    let (s1, s2) = deque.as_slices();
                    println!("Slices: {:?} | {:?}", s1.len(), s2.len());
                },
                4 => {
                    let mut iter = deque.iter();
                    while let Some(e) = iter.next() {
                        println!("Iter: {}", e.0);
                    }
                },
                5 => {
                    let len = deque.len();
                    let drain_range = _to_usize(ops_data, data_idx) % len.saturating_add(1);
                    data_idx = data_idx.saturating_add(8);
                    let _ = deque.drain(..drain_range);
                },
                6 => {
                    let mut iter = SliceDeque::into_iter(deque);
                    iter.next();
                    iter.next();
                    deque = iter.collect();
                },
                _ => (),
            }
        }

        let mut iter = SliceDeque::into_iter(deque);
        let _ = iter.next();
        let _ = iter.next_back();
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