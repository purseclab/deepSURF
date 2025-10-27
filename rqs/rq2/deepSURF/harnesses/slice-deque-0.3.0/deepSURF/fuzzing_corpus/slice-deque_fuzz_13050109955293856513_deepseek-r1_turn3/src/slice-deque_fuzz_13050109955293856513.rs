#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use slice_deque::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

#[derive(Debug, PartialEq, PartialOrd, Default)]
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
        CustomType0(String::from(t_3))
    }
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 8192 {return;}
        set_global_data(data);
        let global_data = get_global_data();
        
        let mut g_idx = 0;
        let GLOBAL_DATA = global_data.first_half;
        
        let op_count = _to_u8(GLOBAL_DATA, g_idx) % 20;
        g_idx += 1;
        
        let mut base_vec = Vec::with_capacity((_to_u8(GLOBAL_DATA, g_idx) % 65) as usize);
        g_idx += 1;
        for _ in 0..(_to_u8(GLOBAL_DATA, g_idx) % 32) {
            g_idx += 1;
            let s_len = _to_u8(GLOBAL_DATA, g_idx) % 17;
            g_idx += 1;
            let s = _to_str(GLOBAL_DATA, g_idx, g_idx + s_len as usize);
            g_idx += s_len as usize;
            base_vec.push(CustomType0(s.to_string()));
        }

        let mut deque1 = match _to_u8(GLOBAL_DATA, g_idx) % 4 {
            0 => SliceDeque::from(&base_vec[..]),
            1 => SliceDeque::with_capacity((_to_usize(GLOBAL_DATA, g_idx) % 65) as usize),
            2 => SliceDeque::new(),
            _ => SliceDeque::from_iter(base_vec.iter().cloned()),
        };
        g_idx += 8;

        let mut deque2 = match _to_u8(GLOBAL_DATA, g_idx) % 4 {
            0 => slice_deque::from_elem(CustomType0(String::new()), (_to_u8(GLOBAL_DATA, g_idx) % 65) as usize),
            1 => SliceDeque::from(base_vec.as_slice()),
            2 => SliceDeque::with_capacity((_to_usize(GLOBAL_DATA, g_idx) % 65) as usize),
            _ => SliceDeque::new(),
        };
        g_idx += 8;

        for _ in 0..op_count {
            match _to_u8(GLOBAL_DATA, g_idx) % 10 {
                0 => {
                    let s_len = _to_u8(GLOBAL_DATA, g_idx) % 17;
                    g_idx += 1;
                    let s = _to_str(GLOBAL_DATA, g_idx, g_idx + s_len as usize);
                    g_idx += s_len as usize;
                    deque1.push_front(CustomType0(s.to_string()));
                }
                1 => {
                    deque1.extend_from_slice(&base_vec[_to_usize(GLOBAL_DATA, g_idx).._to_usize(GLOBAL_DATA, g_idx+8)]);
                    g_idx += 16;
                }
                2 => {
                    deque1.truncate(_to_usize(GLOBAL_DATA, g_idx) % 65);
                    g_idx += 8;
                }
                3 => {
                    deque1.drain(_to_usize(GLOBAL_DATA, g_idx).._to_usize(GLOBAL_DATA, g_idx+8));
                    g_idx += 16;
                }
                4 => {
                    deque2.as_slice();
                    deque2.as_mut_slice();
                    deque2.append(&mut deque1);
                }
                5 => {
                    println!("Front: {:?}", deque1.front());
                    println!("Back: {:?}", deque1.back().map(|x| &x.0));
                }
                6 => {
                    let idx = _to_usize(GLOBAL_DATA, g_idx);
                    g_idx += 8;
                    let _ = deque1.get(idx).map(|x| println!("{:?}", x.0));
                }
                7 => {
                    let len = _to_usize(GLOBAL_DATA, g_idx);
                    g_idx += 8;
                    deque2.resize_default(len);
                    deque1.resize(len, CustomType0(String::new()));
                }
                8 => {
                    deque2.splice(0..0, deque1.drain(.._to_usize(GLOBAL_DATA, g_idx)));
                    g_idx += 8;
                }
                _ => {
                    println!("{:?}", deque1.as_slices());
                    println!("{:?}", deque2.as_mut_slices());
                }
            }
            g_idx += 1;
        }

        deque2.clone_from(&deque1);
        deque1.clone_from(&deque2);

        let _partial_cmp = deque1.partial_cmp(&deque2);
        let _front_mut = deque1.front_mut().map(|x| x.0.push_str("modified"));
        let _ = deque2.pop_back().map(|x| println!("{:?}", x));
        let _eq = deque1.eq(&deque2);
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