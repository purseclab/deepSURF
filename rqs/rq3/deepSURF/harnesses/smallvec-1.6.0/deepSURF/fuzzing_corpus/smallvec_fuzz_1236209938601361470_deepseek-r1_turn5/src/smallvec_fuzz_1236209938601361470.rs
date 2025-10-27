#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::ops::{Deref, DerefMut, Index, IndexMut};

#[derive(Debug, Clone)]
struct CustomType1(String);
#[derive(Debug)]
struct CustomType2(String);
#[derive(Debug)]
struct CustomType3(String);

impl core::iter::IntoIterator for CustomType2 {
    type Item = CustomType1;
    type IntoIter = CustomType3;

    fn into_iter(self) -> Self::IntoIter {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let selector = _to_usize(GLOBAL_DATA, 83) % 3;
        if selector == 0 { panic!("INTENTIONAL PANIC!"); }
        let ctx_len = self.0.len();
        let offset = _to_usize(GLOBAL_DATA, 91 + ctx_len) % GLOBAL_DATA.len();
        let elem_len = _to_u8(GLOBAL_DATA, offset) as usize % 17;
        let s = _to_str(GLOBAL_DATA, offset + 1, offset + 1 + elem_len);
        CustomType3(s.to_string())
    }
}

impl core::iter::Iterator for CustomType3 {
    type Item = CustomType1;

    fn next(&mut self) -> Option<Self::Item> {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.second_half;
        let elem_len = _to_u8(GLOBAL_DATA, self.0.len()) as usize % 17;
        let s = _to_str(GLOBAL_DATA, self.0.len() + 1, self.0.len() + 1 + elem_len);
        Some(CustomType1(s.to_string()))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(_to_usize(get_global_data().first_half, 42) % 5))
    }
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 1024 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let mut ops_idx = 0;

        let constructor_sel = _to_u8(GLOBAL_DATA, ops_idx) % 4;
        ops_idx += 1;
        let mut sv = match constructor_sel {
            0 => SmallVec::<[CustomType1; 16]>::new(),
            1 => SmallVec::<[CustomType1; 16]>::with_capacity(_to_usize(GLOBAL_DATA, ops_idx)),
            2 => {
                let elem_count = _to_usize(GLOBAL_DATA, ops_idx) % 65;
                ops_idx += elem_count * 3;
                let mut v = Vec::new();
                for _ in 0..elem_count {
                    let len = _to_u8(GLOBAL_DATA, ops_idx) as usize % 17;
                    ops_idx += 1;
                    v.push(CustomType1(_to_str(GLOBAL_DATA, ops_idx, ops_idx + len).to_string()));
                    ops_idx += len;
                }
                SmallVec::<[CustomType1; 16]>::from_vec(v)
            }
            _ => SmallVec::<[CustomType1; 16]>::from_elem(CustomType1("".into()), _to_usize(GLOBAL_DATA, ops_idx) % 65),
        };

        let num_ops = _to_usize(GLOBAL_DATA, ops_idx) % 15;
        ops_idx += 1;
        for _ in 0..num_ops {
            if ops_idx + 3 >= GLOBAL_DATA.len() { break; }
            let op = _to_u8(GLOBAL_DATA, ops_idx) % 8;
            ops_idx += 1;

            match op {
                0 => {
                    let len = _to_u8(GLOBAL_DATA, ops_idx) as usize % 17;
                    ops_idx += 1;
                    sv.push(CustomType1(_to_str(GLOBAL_DATA, ops_idx, ops_idx + len).to_string()));
                    ops_idx += len;
                }
                1 => {
                    if !sv.is_empty() {
                        let idx = _to_usize(GLOBAL_DATA, ops_idx) % sv.len();
                        ops_idx += 1;
                        sv.remove(idx);
                    }
                }
                2 => {
                    let idx = _to_usize(GLOBAL_DATA, ops_idx) % (sv.len() + 1);
                    ops_idx += 1;
                    let len = _to_u8(GLOBAL_DATA, ops_idx) as usize % 17;
                    ops_idx += 1;
                    sv.insert(idx, CustomType1(_to_str(GLOBAL_DATA, ops_idx, ops_idx + len).to_string()));
                    ops_idx += len;
                }
                3 => {
                    let capacity = _to_usize(GLOBAL_DATA, ops_idx);
                    ops_idx += 1;
                    sv.reserve(capacity);
                }
                4 => {
                    let start = _to_usize(GLOBAL_DATA, ops_idx) % sv.len();
                    let end = start + _to_usize(GLOBAL_DATA, ops_idx + 1) % (sv.len() - start + 1);
                    ops_idx += 2;
                    let _ = sv.drain(start..end).collect::<Vec<_>>();
                }
                5 => {
                    let amt = _to_usize(GLOBAL_DATA, ops_idx);
                    ops_idx += 1;
                    match _to_u8(GLOBAL_DATA, ops_idx) % 2 {
                        0 => { sv.reserve_exact(amt); },
                        _ => { sv.try_reserve(amt).ok(); },
                    }
                }
                6 => {
                    let len = _to_u8(GLOBAL_DATA, ops_idx) as usize % 17;
                    ops_idx += 1;
                    let iter = CustomType2(_to_str(GLOBAL_DATA, ops_idx, ops_idx + len).to_string());
                    ops_idx += len;
                    sv.extend(iter);
                }
                7 => {
                    let slice = sv.as_slice();
                    println!("{:?}", slice);
                    let mut_slice = sv.as_mut_slice();
                    if !mut_slice.is_empty() {
                        mut_slice[0] = CustomType1("modified".into());
                    }
                }
                _ => unreachable!(),
            }
        }

        let _ = sv.capacity();
        let _ = sv.into_vec();
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