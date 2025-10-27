#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::ops::IndexMut;

#[derive(Debug)]
struct CustomType1(usize);

impl core::marker::Copy for CustomType1 {}

impl core::clone::Clone for CustomType1 {
    fn clone(&self) -> Self {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 10);
        let custom_impl_inst_num = self.0;
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0 {
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = if selector == 1 {
            global_data.first_half
        } else {
            global_data.second_half
        };
        let t_4 = _to_usize(GLOBAL_DATA, 18);
        CustomType1(t_4)
    }
}

impl core::cmp::PartialEq for CustomType1 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl core::cmp::Eq for CustomType1 {}

impl core::cmp::PartialOrd for CustomType1 {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.0.cmp(&other.0))
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
        let elem_count = (_to_u8(GLOBAL_DATA, 2) % 65) as usize;
        let mut base_vec = std::vec::Vec::with_capacity(65);
        for i in 0..elem_count {
            let val = _to_usize(GLOBAL_DATA, 32 + i * 8);
            base_vec.push(CustomType1(val));
        }
        let mut small_vec: SmallVec<[CustomType1; 32]> = match _to_u8(GLOBAL_DATA, 1) % 6 {
            0 => {
                let mut buf = [CustomType1(0); 32];
                for i in 0..32 {
                    let v = _to_usize(GLOBAL_DATA, 256 + i * 8);
                    buf[i] = CustomType1(v);
                }
                SmallVec::from_buf(buf)
            }
            1 => SmallVec::new(),
            2 => {
                let cap = _to_usize(GLOBAL_DATA, 520) % 65;
                SmallVec::with_capacity(cap)
            }
            3 => SmallVec::from_vec(base_vec.clone()),
            4 => {
                let elem = CustomType1(_to_usize(GLOBAL_DATA, 600));
                let cnt = (_to_u8(GLOBAL_DATA, 608) % 65) as usize;
                SmallVec::from_elem(elem, cnt)
            }
            _ => SmallVec::from_slice(&base_vec),
        };
        let op_cnt = (_to_u8(GLOBAL_DATA, 700) % 16) as usize;
        for i in 0..op_cnt {
            match _to_u8(GLOBAL_DATA, 710 + i) % 8 {
                0 => {
                    let v = CustomType1(_to_usize(GLOBAL_DATA, 730 + i * 8));
                    small_vec.push(v);
                }
                1 => {
                    small_vec.pop();
                }
                2 => {
                    let len = small_vec.len();
                    if len > 0 {
                        let idx = _to_usize(GLOBAL_DATA, 800 + i * 8) % len;
                        small_vec.remove(idx);
                    }
                }
                3 => {
                    let new_len = (_to_u8(GLOBAL_DATA, 860 + i) % 65) as usize;
                    small_vec.truncate(new_len);
                }
                4 => {
                    let additional = _to_usize(GLOBAL_DATA, 900 + i * 4) % 65;
                    small_vec.reserve(additional);
                }
                5 => {
                    let slice = small_vec.as_slice();
                    println!("{:?}", slice.len());
                }
                6 => {
                    let cloned = small_vec.clone();
                    let _ = small_vec.partial_cmp(&cloned);
                }
                _ => {
                    small_vec.clear();
                }
            }
        }
        match _to_u8(GLOBAL_DATA, 950) % 4 {
            0 => {
                let idx = if small_vec.is_empty() {
                    0
                } else {
                    _to_usize(GLOBAL_DATA, 960) % small_vec.len()
                };
                let r = small_vec.index_mut(idx);
                println!("{:?}", *r);
            }
            1 => {
                let range = 0..small_vec.len();
                let s = small_vec.index_mut(range);
                println!("{:?}", s.len());
            }
            2 => {
                let start = if small_vec.is_empty() {
                    0
                } else {
                    _to_usize(GLOBAL_DATA, 970) % small_vec.len()
                };
                let s = small_vec.index_mut(start..);
                println!("{:?}", s.len());
            }
            _ => {
                let s = small_vec.index_mut(..);
                println!("{:?}", s.len());
            }
        }
        let _ = small_vec.len();
        let _ = small_vec.capacity();
        let _ = small_vec.is_empty();
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