#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use toodee::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

#[derive(Debug, Clone, Default)]
struct CustomType1(String);
#[derive(Debug)]
struct CustomType2(String);
#[derive(Debug)]
struct CustomType0(String);

impl core::iter::ExactSizeIterator for CustomType2 {
    fn len(&self) -> usize {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 8);
        let custom_impl_inst_num = self.0.len();
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0 {
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector {
            1 => global_data.first_half,
            _ => global_data.second_half,
        };
        let t_3 = _to_usize(GLOBAL_DATA, 16);
        return t_3;
    }
}

impl core::iter::Iterator for CustomType2 {
    type Item = CustomType1;

    fn next(&mut self) -> Option<Self::Item> {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 24);
        let custom_impl_inst_num = self.0.len();
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0 {
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector {
            1 => global_data.first_half,
            _ => global_data.second_half,
        };
        let mut t_4 = _to_u8(GLOBAL_DATA, 32) % 17;
        let t_5 = _to_str(GLOBAL_DATA, 33, 33 + t_4 as usize);
        let t_6 = String::from(t_5);
        let t_7 = CustomType1(t_6);
        let t_8 = Some(t_7);
        return t_8;
    }
}

impl core::iter::IntoIterator for CustomType0 {
    type Item = CustomType1;
    type IntoIter = CustomType2;

    fn into_iter(self) -> Self::IntoIter {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 49);
        let custom_impl_inst_num = self.0.len();
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0 {
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector {
            1 => global_data.first_half,
            _ => global_data.second_half,
        };
        let mut t_9 = _to_u8(GLOBAL_DATA, 57) % 17;
        let t_10 = _to_str(GLOBAL_DATA, 58, 58 + t_9 as usize);
        let t_11 = String::from(t_10);
        let t_12 = CustomType2(t_11);
        return t_12;
    }
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 4096 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let mut offset = 0;

        let constructor_selector = _to_u8(GLOBAL_DATA, offset) % 4;
        offset += 1;
        let mut t_1 = match constructor_selector {
            0 => {
                let cap = _to_usize(GLOBAL_DATA, offset);
                offset += 8;
                toodee::TooDee::with_capacity(cap)
            }
            1 => {
                let cols = _to_usize(GLOBAL_DATA, offset);
                offset += 8;
                let rows = _to_usize(GLOBAL_DATA, offset);
                offset += 8;
                toodee::TooDee::new(cols, rows)
            }
            2 => {
                let cols = _to_usize(GLOBAL_DATA, offset);
                offset += 8;
                let rows = _to_usize(GLOBAL_DATA, offset);
                offset += 8;
                let len = _to_u8(GLOBAL_DATA, offset) as usize;
                offset += 1;
                let s = _to_str(GLOBAL_DATA, offset, offset + len);
                offset += len;
                toodee::TooDee::init(cols, rows, CustomType1(s.to_string()))
            }
            3 => {
                let cols = _to_usize(GLOBAL_DATA, offset);
                offset += 8;
                let rows = _to_usize(GLOBAL_DATA, offset);
                offset += 8;
                let total = cols * rows;
                let mut vec = Vec::with_capacity(total);
                for _ in 0..total {
                    let len = _to_u8(GLOBAL_DATA, offset) as usize;
                    offset += 1;
                    let s = _to_str(GLOBAL_DATA, offset, offset + len);
                    offset += len;
                    vec.push(CustomType1(s.to_string()));
                }
                toodee::TooDee::from_vec(cols, rows, vec)
            }
            _ => unreachable!(),
        };

        let num_ops = _to_u8(GLOBAL_DATA, offset) % 16;
        offset += 1;

        for _ in 0..num_ops {
            if offset + 1 >= GLOBAL_DATA.len() { break; }
            let op = _to_u8(GLOBAL_DATA, offset) % 7;
            offset += 1;

            match op {
                0 => {
                    let len = _to_u8(GLOBAL_DATA, offset) as usize;
                    offset += 1;
                    if offset + len > GLOBAL_DATA.len() { break; }
                    let s = _to_str(GLOBAL_DATA, offset, offset + len);
                    offset += len;
                    let val = CustomType0(s.to_string());
                    t_1.push_row(val);
                }
                1 => {
                    if offset + 16 > GLOBAL_DATA.len() { break; }
                    let start_col = _to_usize(GLOBAL_DATA, offset);
                    offset += 8;
                    let start_row = _to_usize(GLOBAL_DATA, offset);
                    offset += 8;
                    let end_col = _to_usize(GLOBAL_DATA, offset);
                    offset += 8;
                    let end_row = _to_usize(GLOBAL_DATA, offset);
                    offset += 8;
                    let mut view = t_1.view_mut((start_col, start_row), (end_col, end_row));
                    let row = &mut view[0];
                    println!("{:?}", row);
                }
                2 => {
                    if offset + 8 > GLOBAL_DATA.len() { break; }
                    let col = _to_usize(GLOBAL_DATA, offset);
                    offset += 8;
                    let _ = t_1.remove_col(col);
                }
                3 => {
                    if offset + 16 > GLOBAL_DATA.len() { break; }
                    let r1 = _to_usize(GLOBAL_DATA, offset);
                    offset += 8;
                    let r2 = _to_usize(GLOBAL_DATA, offset);
                    offset += 8;
                    t_1.swap_rows(r1, r2);
                }
                4 => {
                    let idx = _to_usize(GLOBAL_DATA, offset);
                    offset += 8;
                    let len = _to_u8(GLOBAL_DATA, offset) as usize;
                    offset += 1;
                    if offset + len > GLOBAL_DATA.len() { break; }
                    let s = _to_str(GLOBAL_DATA, offset, offset + len);
                    offset += len;
                    t_1.insert_row(idx, CustomType0(s.to_string()));
                }
                5 => {
                    if let Some(mut col) = t_1.pop_col() {
                        while let Some(item) = col.next() {
                            println!("{:?}", item);
                        }
                    }
                }
                6 => {
                    let num_rows = t_1.num_rows();
                    let row_selector = _to_usize(GLOBAL_DATA, offset) % num_rows;
                    let mut rows_mut = t_1.rows_mut();
                    if let Some(row) = rows_mut.nth(row_selector) {
                        println!("{:?}", row);
                    }
                    offset += 8;
                }
                _ => (),
            }
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