#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use toodee::*;
use global_data::*;
use std::ops::{Deref, DerefMut, Index, IndexMut};
use std::str::FromStr;

#[derive(Clone)]
struct CustomType0(String);
struct CustomType1(String);
struct CustomType2(String);
struct CustomType3(String);

impl core::iter::ExactSizeIterator for CustomType1 {
    fn len(&self) -> usize {
        let global_data = get_global_data();
        let selector = (_to_usize(global_data.first_half, 0) + self.0.len()) % 3;
        if selector == 0 {
            panic!("INTENTIONAL PANIC!");
        }
        _to_usize(global_data.second_half, 3)
    }
}

impl core::iter::Iterator for CustomType1 {
    type Item = CustomType0;
    fn next(&mut self) -> Option<Self::Item> {
        let global_data = get_global_data();
        let selector = (_to_usize(global_data.first_half, 11) + self.0.len()) % 3;
        if selector == 0 {
            panic!("INTENTIONAL PANIC!");
        }
        let len = _to_u8(global_data.second_half, 18) % 17;
        let start = 19;
        if start + len as usize > global_data.second_half.len() {
            return None;
        }
        let s = _to_str(global_data.second_half, start, start + len as usize);
        Some(CustomType0(String::from(s)))
    }
}

impl core::iter::DoubleEndedIterator for CustomType1 {
    fn next_back(&mut self) -> Option<Self::Item> {
        let global_data = get_global_data();
        let selector = (_to_usize(global_data.first_half, 40) + self.0.len()) % 3;
        if selector == 0 {
            panic!("INTENTIONAL PANIC!");
        }
        let len = _to_u8(global_data.second_half, 47) % 17;
        let start = 48;
        if start + len as usize > global_data.second_half.len() {
            return None;
        }
        let s = _to_str(global_data.second_half, start, start + len as usize);
        Some(CustomType0(String::from(s)))
    }
}

impl core::iter::IntoIterator for CustomType3 {
    type Item = CustomType0;
    type IntoIter = CustomType1;
    fn into_iter(self) -> Self::IntoIter {
        CustomType1(self.0)
    }
}

struct RowIter {
    remaining: usize,
}

impl core::iter::Iterator for RowIter {
    type Item = CustomType0;
    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            return None;
        }
        self.remaining -= 1;
        let global_data = get_global_data();
        let idx = self.remaining % (global_data.first_half.len().saturating_sub(2));
        let len = (_to_u8(global_data.first_half, idx) % 13) as usize;
        let end = (idx + len).min(global_data.first_half.len());
        let s = _to_str(global_data.first_half, idx, end);
        Some(CustomType0(String::from(s)))
    }
}

impl core::iter::ExactSizeIterator for RowIter {
    fn len(&self) -> usize {
        self.remaining
    }
}

impl core::iter::DoubleEndedIterator for RowIter {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.next()
    }
}

fn build_vec() -> Vec<CustomType0> {
    let global_data = get_global_data();
    let mut v = Vec::with_capacity(32);
    let mut idx = 0usize;
    let iterations = (_to_u8(global_data.first_half, 60) % 65) as usize;
    for _ in 0..iterations {
        if idx + 2 >= global_data.first_half.len() {
            break;
        }
        let len = (_to_u8(global_data.first_half, idx) % 17) as usize;
        let start = (idx + 1) % (global_data.first_half.len().saturating_sub(len));
        let end = start + len;
        let s = _to_str(global_data.first_half, start, end);
        v.push(CustomType0(String::from(s)));
        idx += 3;
    }
    v
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 256 {
            return;
        }
        set_global_data(data);
        let global_data = get_global_data();
        let num_cols = _to_usize(global_data.first_half, 70);
        let num_rows = _to_usize(global_data.first_half, 78);
        let selector = _to_u8(global_data.first_half, 86) % 3;
        let mut td = match selector {
            0 => {
                let v = build_vec();
                TooDee::from_vec(num_cols, num_rows, v)
            }
            1 => {
                let v = build_vec().into_boxed_slice();
                TooDee::from_box(num_cols, num_rows, v)
            }
            _ => TooDee::with_capacity(128),
        };
        let iterations = (_to_u8(global_data.first_half, 94) % 20) as usize;
        for i in 0..iterations {
            let op = _to_u8(global_data.first_half, 100 + i) % 7;
            match op {
                0 => {
                    let col_idx = _to_usize(global_data.second_half, 120 + i);
                    let c = td.col_mut(col_idx);
                    println!("{:?}", c.len());
                }
                1 => {
                    let r1 = _to_usize(global_data.second_half, 140 + i);
                    let r2 = _to_usize(global_data.second_half, 148 + i);
                    td.swap_rows(r1, r2);
                }
                2 => {
                    let ri = RowIter {
                        remaining: 5,
                    };
                    td.push_row(ri);
                }
                3 => {
                    let ri = RowIter {
                        remaining: 4,
                    };
                    td.push_col(ri);
                }
                4 => {
                    td.pop_col();
                }
                5 => {
                    let col_idx = _to_usize(global_data.second_half, 160 + i);
                    td.remove_col(col_idx);
                }
                _ => {
                    let start = (
                        _to_usize(global_data.first_half, 170 + i),
                        _to_usize(global_data.first_half, 178 + i),
                    );
                    let end = (
                        _to_usize(global_data.first_half, 186 + i),
                        _to_usize(global_data.first_half, 194 + i),
                    );
                    let v = td.view(start, end);
                    println!("{:?}", v.num_cols());
                }
            }
        }
        let insert_idx = _to_usize(global_data.first_half, 200);
        let col_iter_seed = _to_str(global_data.first_half, 208, 220);
        let col_iter = CustomType3(String::from(col_iter_seed));
        td.insert_col(insert_idx, col_iter);
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