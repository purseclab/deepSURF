#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::SmallVec;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
struct CustomType1(String);
struct CustomType3(String);
struct CustomType2(String);

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
        let t_8 = Some(t_7);
        let t_9 = (t_6, t_8);
        return t_9;
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
        let t_13 = CustomType1(t_12);
        let t_14 = Some(t_13);
        return t_14;
    }
}

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
        let t_18 = CustomType3(t_17);
        return t_18;
    }
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 2500 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        
        let mut offset = 0;
        let op_count = _to_u8(GLOBAL_DATA, offset) % 65;
        offset += 1;

        let mut v1 = SmallVec::<[CustomType1; 16]>::new();
        let mut v2 = SmallVec::<[CustomType1; 16]>::new();

        match _to_u8(GLOBAL_DATA, offset) % 4 {
            0 => v1 = SmallVec::from_iter(CustomType2(String::new())),
            1 => {
                let elem = CustomType1(String::from(_to_str(GLOBAL_DATA, offset + 1, offset + 20)));
                v1 = SmallVec::from_elem(elem, _to_usize(GLOBAL_DATA, offset + 20) % 65);
            },
            2 => {
                let cap = _to_usize(GLOBAL_DATA, offset + 1) % 65;
                v1 = SmallVec::with_capacity(cap);
            },
            _ => {
                let mut tmp = Vec::new();
                let count = _to_u8(GLOBAL_DATA, offset + 1) % 65;
                for i in 0..count {
                    let s = _to_str(GLOBAL_DATA, offset + 2 + i as usize * 5, offset + 7 + i as usize * 5);
                    tmp.push(CustomType1(String::from(s)));
                }
                v1 = SmallVec::from_vec(tmp);
            }
        }
        offset += 50;

        match _to_u8(GLOBAL_DATA, offset) % 4 {
            0 => v2 = SmallVec::new(),
            1 => {
                let elem = CustomType1(String::from(_to_str(GLOBAL_DATA, offset + 1, offset + 20)));
                v2 = SmallVec::from_elem(elem, _to_usize(GLOBAL_DATA, offset + 20) % 65);
            },
            2 => {
                let mut tmp = Vec::new();
                let count = _to_u8(GLOBAL_DATA, offset + 1) % 65;
                for i in 0..count {
                    let s = _to_str(GLOBAL_DATA, offset + 2 + i as usize * 5, offset + 7 + i as usize * 5);
                    tmp.push(CustomType1(String::from(s)));
                }
                v2 = SmallVec::from_vec(tmp);
            },
            _ => v2 = SmallVec::new()
        }
        offset += 50;

        for _ in 0..op_count {
            let op = _to_u8(GLOBAL_DATA, offset) % 6;
            offset += 1;
            match op {
                0 => {
                    let s = _to_str(GLOBAL_DATA, offset, offset + 10);
                    v1.push(CustomType1(String::from(s)));
                    offset += 10;
                },
                1 => {
                    if !v1.is_empty() {
                        let _ = v1.pop();
                    }
                },
                2 => {
                    let idx = _to_usize(GLOBAL_DATA, offset);
                    let s = _to_str(GLOBAL_DATA, offset + 1, offset + 10);
                    v1.insert(idx, CustomType1(String::from(s)));
                    offset += 10;
                },
                3 => {
                    let len = _to_usize(GLOBAL_DATA, offset);
                    v1.truncate(len);
                    offset += 1;
                },
                4 => {
                    let _cap = v1.capacity();
                    v1.shrink_to_fit();
                },
                _ => {
                    let _ = v1.as_slice();
                    if !v1.is_empty() {
                        let _ = &v1[0];
                    }
                }
            }
        }

        if !v1.is_empty() {
            let _ = v1.as_mut_slice();
            let _ = &mut v1[0];
        }

        for _ in 0..op_count {
            let op = _to_u8(GLOBAL_DATA, offset) % 6;
            offset += 1;
            match op {
                0 => {
                    let s = _to_str(GLOBAL_DATA, offset, offset + 10);
                    v2.push(CustomType1(String::from(s)));
                    offset += 10;
                },
                1 => {
                    if !v2.is_empty() {
                        let _ = v2.pop();
                    }
                },
                2 => {
                    let idx = _to_usize(GLOBAL_DATA, offset);
                    let s = _to_str(GLOBAL_DATA, offset + 1, offset + 10);
                    v2.insert(idx, CustomType1(String::from(s)));
                    offset += 10;
                },
                3 => {
                    let len = _to_usize(GLOBAL_DATA, offset);
                    v2.truncate(len);
                    offset += 1;
                },
                4 => {
                    let _cap = v2.capacity();
                    v2.reserve(_to_usize(GLOBAL_DATA, offset) % 65);
                    offset += 1;
                },
                _ => {
                    let _ = v2.as_slice();
                    if !v2.is_empty() {
                        let _ = &v2[0];
                    }
                }
            }
        }

        let _cmp = v1.cmp(&v2);
        let _eq = v1.eq(&v2);
        v1.ne(&v2);
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