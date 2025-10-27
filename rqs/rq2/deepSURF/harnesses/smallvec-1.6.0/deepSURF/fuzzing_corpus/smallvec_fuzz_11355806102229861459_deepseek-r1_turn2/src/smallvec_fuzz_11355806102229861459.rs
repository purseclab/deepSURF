#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::ops::{Bound, RangeBounds};

#[derive(Debug)]
struct CustomType3(String);
#[derive(Debug)]
struct CustomType4(usize, usize);

impl core::clone::Clone for CustomType3 {
    fn clone(&self) -> Self {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 19);
        let custom_impl_inst_num = self.0.len();
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0 {
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector {
            1 => global_data.first_half,
            _ => global_data.second_half,
        };
        let mut t_6 = _to_u8(GLOBAL_DATA, 27) % 17;
        let t_7 = _to_str(GLOBAL_DATA, 28, 28 + t_6 as usize);
        let t_8 = String::from(t_7);
        CustomType3(t_8)
    }
}

impl RangeBounds<usize> for CustomType4 {
    fn start_bound(&self) -> Bound<&usize> {
        Bound::Excluded(&self.0)
    }

    fn end_bound(&self) -> Bound<&usize> {
        Bound::Included(&self.1)
    }
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 16384 {
            return;
        }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let constructor_num = _to_u8(GLOBAL_DATA, 0) % 3;
        let mut vec: SmallVec<[CustomType3; 32]> = match constructor_num {
            0 => {
                let capacity = _to_usize(GLOBAL_DATA, 1);
                SmallVec::with_capacity(capacity)
            }
            1 => {
                let elem_count = _to_usize(GLOBAL_DATA, 1) % 65;
                let mut v = SmallVec::new();
                for i in 0..elem_count {
                    let offset = 2 + i * 18;
                    let len = _to_u8(GLOBAL_DATA, offset) % 17;
                    let s = _to_str(GLOBAL_DATA, offset + 1, offset + 1 + len as usize);
                    v.push(CustomType3(s.to_string()));
                }
                v
            }
            _ => {
                let mut normal_vec = Vec::new();
                let elem_count = _to_usize(GLOBAL_DATA, 1) % 65;
                for i in 0..elem_count {
                    let offset = 2 + i * 18;
                    let len = _to_u8(GLOBAL_DATA, offset) % 17;
                    let s = _to_str(GLOBAL_DATA, offset + 1, offset + 1 + len as usize);
                    normal_vec.push(CustomType3(s.to_string()));
                }
                SmallVec::from_vec(normal_vec)
            }
        };

        let op_count = _to_u8(GLOBAL_DATA, 800) % 24;
        for i in 0..op_count {
            let op_selector = _to_u8(GLOBAL_DATA, 801 + i as usize) % 7;
            match op_selector {
                0 => {
                    let len = _to_u8(GLOBAL_DATA, 825 + i as usize) % 17;
                    let start = 826 + i as usize;
                    let s = _to_str(GLOBAL_DATA, start, start + len as usize);
                    vec.push(CustomType3(s.to_string()));
                }
                1 => {
                    if !vec.is_empty() {
                        vec.swap_remove(_to_usize(GLOBAL_DATA, 850 + i as usize) % vec.len());
                    }
                }
                2 => {
                    vec.reserve(_to_usize(GLOBAL_DATA, 900 + i as usize));
                }
                3 => {
                    vec.truncate(_to_usize(GLOBAL_DATA, 950 + i as usize) % 65);
                }
                4 => {
                    let drain_start = _to_usize(GLOBAL_DATA, 1000 + i as usize * 2) % (vec.len() + 1);
                    let drain_end = drain_start + _to_usize(GLOBAL_DATA, 1001 + i as usize * 2) % (vec.len() - drain_start + 1);
                    let mut drained = vec.drain(drain_start..drain_end);
                    let _ = drained.next();
                    let _ = drained.next_back();
                }
                5 => {
                    let as_slice = vec.as_slice();
                    if !as_slice.is_empty() {
                        let idx = _to_usize(GLOBAL_DATA, 1100 + i as usize) % as_slice.len();
                        let elem = &as_slice[idx];
                        println!("{:?}", elem);
                        let _ = elem.0.as_str();
                    }
                }
                6 => {
                    let mut new_elems = Vec::new();
                    for j in 0..3 {
                        let len = _to_u8(GLOBAL_DATA, 1200 + i as usize * 3 + j as usize) % 17;
                        let start = 1201 + i as usize * 3 + j as usize;
                        let s = _to_str(GLOBAL_DATA, start, start + len as usize);
                        new_elems.push(CustomType3(s.to_string()));
                    }
                    vec.extend(new_elems);
                }
                _ => {}
            }
        }

        let range_selector = _to_u8(GLOBAL_DATA, 1500) % 2;
        let mut drain = match range_selector {
            0 => {
                let range_start = _to_usize(GLOBAL_DATA, 1501);
                let range_end = _to_usize(GLOBAL_DATA, 1509);
                vec.drain(range_start..range_end)
            }
            _ => {
                let range_str = _to_str(GLOBAL_DATA, 1517, 1533);
                let parts: Vec<&str> = range_str.split(',').collect();
                let start = parts.get(0).and_then(|s| s.parse().ok()).unwrap_or(0);
                let end = parts.get(1).and_then(|s| s.parse().ok()).unwrap_or(0);
                vec.drain(CustomType4(start, end))
            }
        };

        for elem in drain.by_ref() {
            let _ = elem.0.capacity();
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