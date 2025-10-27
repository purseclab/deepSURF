#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use slice_deque::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut};

struct CustomType0(String);

impl Clone for CustomType0 {
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
        let t_2 = _to_u8(GLOBAL_DATA, 9) % 17;
        let t_3 = _to_str(GLOBAL_DATA, 10, 10 + t_2 as usize);
        CustomType0(String::from(t_3))
    }
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 4096 {
            return;
        }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let mut constructor_selector = _to_u8(GLOBAL_DATA, 0) % 3;
        let mut deq = match constructor_selector {
            0 => {
                let cap = _to_u8(GLOBAL_DATA, 1) % 65;
                SliceDeque::with_capacity(cap as usize)
            }
            1 => SliceDeque::from_iter(vec![
                CustomType0(String::new()),
                CustomType0(String::new()),
            ]),
            _ => SliceDeque::new(),
        };

        let num_ops = _to_u8(GLOBAL_DATA, 2) % 65;
        let mut data_offset = 3;

        for _ in 0..num_ops {
            let op_selector = _to_u8(GLOBAL_DATA, data_offset) % 8;
            data_offset += 1;

            match op_selector {
                0 => {
                    let val = _to_u8(GLOBAL_DATA, data_offset) % 33;
                    let s = _to_str(GLOBAL_DATA, data_offset + 1, data_offset + 1 + val as usize);
                    deq.push_back(CustomType0(String::from(s)));
                    data_offset += 1 + val as usize;
                }
                1 => {
                    let val = _to_u8(GLOBAL_DATA, data_offset) % 33;
                    let s = _to_str(GLOBAL_DATA, data_offset + 1, data_offset + 1 + val as usize);
                    deq.push_front(CustomType0(String::from(s)));
                    data_offset += 1 + val as usize;
                }
                2 => {
                    let _ = deq.pop_back();
                }
                3 => {
                    let _ = deq.pop_front();
                }
                4 => {
                    let new_len = _to_u8(GLOBAL_DATA, data_offset) as usize % deq.len();
                    deq.truncate(new_len);
                    data_offset += 1;
                }
                5 => {
                    let (s1, s2) = deq.as_slices();
                    println!("{:?} {:?}", s1.as_ptr(), s2.as_ptr());
                }
                6 => {
                    let other_cap = _to_u8(GLOBAL_DATA, data_offset) % 65;
                    let mut other = SliceDeque::with_capacity(other_cap as usize);
                    let elems = _to_u8(GLOBAL_DATA, data_offset + 1) % 65;
                    data_offset += 2;
                    for _ in 0..elems {
                        let val = _to_u8(GLOBAL_DATA, data_offset) % 33;
                        let s = _to_str(GLOBAL_DATA, data_offset + 1, data_offset + 1 + val as usize);
                        other.push_back(CustomType0(String::from(s)));
                        data_offset += 1 + val as usize;
                    }
                    deq.append(&mut other);
                }
                _ => {
                    let elems = _to_u8(GLOBAL_DATA, data_offset) % 65;
                    data_offset += 1;
                    let mut v = Vec::with_capacity(elems as usize);
                    for _ in 0..elems {
                        let val = _to_u8(GLOBAL_DATA, data_offset) % 33;
                        let s = _to_str(GLOBAL_DATA, data_offset + 1, data_offset + 1 + val as usize);
                        v.push(CustomType0(String::from(s)));
                        data_offset += 1 + val as usize;
                    }
                    if let Some(slice) = v.get(..) {
                        deq.extend_from_slice(slice);
                    }
                }
            }
        }

        let slice_len = _to_u8(GLOBAL_DATA, data_offset) % 65;
        data_offset += 1;
        let mut ext_vec = Vec::with_capacity(slice_len as usize);
        for _ in 0..slice_len {
            let val = _to_u8(GLOBAL_DATA, data_offset) % 33;
            let s = _to_str(GLOBAL_DATA, data_offset + 1, data_offset + 1 + val as usize);
            ext_vec.push(CustomType0(String::from(s)));
            data_offset += 1 + val as usize;
        }

        if data_offset < GLOBAL_DATA.len() {
            let ext_slice = &ext_vec[..];
            deq.extend_from_slice(ext_slice);
        }

        if let Some(front) = deq.front() {
            println!("{:?}", front.0.as_ptr());
        }
        if let Some(back) = deq.back() {
            println!("{:?}", back.0.as_ptr());
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