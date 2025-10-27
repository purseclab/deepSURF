#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;

use std::ops::{Deref, DerefMut, Index, IndexMut};
use std::str::FromStr;

#[derive(Clone, Debug)]
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
        _to_bool(GLOBAL_DATA, 8)
    }
}

impl Deref for CustomType1 {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for CustomType1 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 200 {
            return;
        }
        set_global_data(data);
        let global_data = get_global_data();
        let FIRST = global_data.first_half;
        let SECOND = global_data.second_half;

        let base_len = _to_u8(FIRST, 10) % 32;
        let base_str = _to_str(FIRST, 11, 11 + base_len as usize);
        let base_elem = CustomType1(String::from(base_str));

        use std::convert::TryInto;
        let buffer: [CustomType1; 16] = vec![base_elem.clone(); 16]
            .try_into()
            .unwrap_or_else(|_| panic!("Failed to build buffer"));

        let mut vec_items = Vec::new();
        let vec_len = (_to_u8(FIRST, 15) % 16) as usize;
        for _ in 0..vec_len {
            vec_items.push(base_elem.clone());
        }

        let selector = _to_u8(FIRST, 2) % 5;
        let mut sv: SmallVec<[CustomType1; 16]> = match selector {
            0 => SmallVec::new(),
            1 => SmallVec::with_capacity(_to_usize(FIRST, 3) % 65),
            2 => SmallVec::from_elem(base_elem.clone(), (_to_u8(FIRST, 5) % 65) as usize),
            3 => SmallVec::from_iter(vec_items.clone().into_iter()),
            _ => SmallVec::from_buf_and_len(buffer, (_to_u8(FIRST, 6) % 17) as usize),
        };

        for (i, code) in SECOND.iter().enumerate() {
            match code % 16 {
                0 => sv.push(base_elem.clone()),
                1 => {
                    sv.pop();
                }
                2 => {
                    sv.dedup();
                }
                3 => {
                    sv.dedup_by(|a, b| {
                        let idx = i % FIRST.len();
                        let flag = _to_bool(FIRST, idx);
                        if flag {
                            a.0 == b.0
                        } else {
                            a.0.len() == b.0.len()
                        }
                    });
                }
                4 => {
                    if sv.len() > 0 {
                        let idx = _to_usize(FIRST, 7);
                        sv.remove(idx);
                    }
                }
                5 => {
                    let idx = _to_usize(FIRST, 9);
                    sv.insert(idx, base_elem.clone());
                }
                6 => {
                    let idx = _to_usize(FIRST, 11);
                    sv.swap_remove(idx);
                }
                7 => {
                    sv.retain(|x| {
                        let idx = i % FIRST.len();
                        let keep = _to_bool(FIRST, idx);
                        keep && !x.0.is_empty()
                    });
                }
                8 => {
                    sv.extend(vec_items.clone());
                }
                9 => {
                    let new_len = _to_usize(FIRST, 20) % 65;
                    sv.resize(new_len, base_elem.clone());
                }
                10 => {
                    sv.reserve(_to_usize(FIRST, 22) % 65);
                }
                11 => {
                    sv.truncate(_to_usize(FIRST, 24) % 65);
                }
                12 => {
                    if _to_bool(FIRST, 26) {
                        sv.clear();
                    }
                }
                13 => {
                    let _c = sv.capacity();
                    let _l = sv.len();
                }
                14 => {
                    if sv.len() > 0 {
                        let first_ref = &sv[0];
                        let _val = first_ref.deref();
                    }
                }
                _ => {
                    let sv2 = sv.clone();
                    let _ = sv == sv2;
                }
            }
        }

        sv.dedup();
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