#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

#[derive(Debug)]
struct CustomType1(String);

impl core::cmp::PartialEq for CustomType1 {
    fn eq(&self, _: &Self) -> bool {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 0);
        let custom_impl_inst_num = self.0.len();
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0 {
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector {
            1 => global_data.first_half,
            _ => global_data.second_half,
        };
        let t_0 = _to_bool(GLOBAL_DATA, 8);
        t_0
    }
}

impl core::clone::Clone for CustomType1 {
    fn clone(&self) -> Self {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 28);
        let custom_impl_inst_num = self.0.len();
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0 {
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector {
            1 => global_data.first_half,
            _ => global_data.second_half,
        };
        let mut t_7 = _to_u8(GLOBAL_DATA, 36) % 17;
        let t_8 = _to_str(GLOBAL_DATA, 37, 37 + t_7 as usize);
        let t_9 = String::from(t_8);
        let t_10 = CustomType1(t_9);
        t_10
    }
}

impl core::cmp::Eq for CustomType1 {}

impl core::cmp::PartialOrd for CustomType1 {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl core::cmp::Ord for CustomType1 {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 600 {
            return;
        }
        set_global_data(data);
        let global = get_global_data();
        let first = global.first_half;
        let second = global.second_half;

        let mut vec1 = std::vec::Vec::new();
        let mut vec2 = std::vec::Vec::new();

        let num1 = (_to_u8(first, 12) % 65) as usize;
        for i in 0..num1 {
            let len = _to_u8(first, 30 + i) % 17;
            let start = (31 + i * 17) % (first.len() - 17);
            let s = _to_str(first, start, start + len as usize);
            vec1.push(CustomType1(String::from(s)));
        }

        let num2 = (_to_u8(second, 12) % 65) as usize;
        for i in 0..num2 {
            let len = _to_u8(second, 30 + i) % 17;
            let start = (31 + i * 17) % (second.len() - 17);
            let s = _to_str(second, start, start + len as usize);
            vec2.push(CustomType1(String::from(s)));
        }

        let sel1 = _to_u8(first, 4) % 4;
        let mut sv1: SmallVec<[CustomType1; 16]> = match sel1 {
            0 => SmallVec::new(),
            1 => SmallVec::with_capacity(_to_usize(first, 40) % 65),
            2 => SmallVec::from_vec(vec1.clone()),
            _ => SmallVec::from_elem(
                vec1.get(0).cloned().unwrap_or_else(|| CustomType1(String::from(""))),
                _to_usize(first, 50) % 65,
            ),
        };

        let sel2 = _to_u8(second, 4) % 4;
        let mut sv2: SmallVec<[CustomType1; 16]> = match sel2 {
            0 => SmallVec::from_vec(vec2.clone()),
            1 => SmallVec::from_iter(vec2.clone()),
            2 => SmallVec::new(),
            _ => SmallVec::with_capacity(_to_usize(second, 60) % 65),
        };

        let _ = (&sv1).eq(&sv2);

        let op_cnt = (_to_u8(first, 5) % 20) as usize;
        for i in 0..op_cnt {
            let op_byte = _to_u8(first, 100 + i);
            match op_byte % 12 {
                0 => {
                    let idx = _to_usize(first, 110 + i);
                    if idx <= sv1.len() {
                        let elem_str = _to_str(first, 200 + i, 201 + i);
                        let elem = CustomType1(String::from(elem_str));
                        sv1.insert(idx, elem);
                    }
                }
                1 => {
                    sv1.push(CustomType1(String::from("")));
                }
                2 => {
                    sv1.truncate(_to_usize(first, 210 + i));
                }
                3 => {
                    let _ = sv1.remove(_to_usize(first, 220 + i));
                }
                4 => {
                    sv1.extend(sv2.iter().cloned());
                }
                5 => {
                    sv1.retain(|item| {
                        let cond = _to_bool(first, 230 + i);
                        println!("{:?}", item.deref());
                        cond
                    });
                }
                6 => {
                    sv1.dedup();
                }
                7 => {
                    let _ = sv1.partial_cmp(&sv2);
                }
                8 => {
                    let _ = sv1.cmp(&sv2);
                }
                9 => {
                    if !sv1.is_empty() {
                        let _ = sv1.swap_remove(_to_usize(first, 240 + i));
                    }
                }
                10 => {
                    let _ = sv1.pop();
                }
                _ => {
                    let _ = (&sv1).eq(&sv2);
                }
            }
        }

        let _slice_ref = sv1.as_slice();
        let _ = sv1.capacity();
        let _ = (&sv1).eq(&sv2);
        let _ = sv1.len();
        let _ = sv1.is_empty();
        let _ = sv1.clone();
        let _ = sv2.into_vec();
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