#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use toodee::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

#[derive(Clone, Debug)]
struct CustomType0(String);

impl Default for CustomType0 {
    fn default() -> Self {
        CustomType0(String::new())
    }
}

impl Deref for CustomType0 {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for CustomType0 {
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
        let GLOBAL_DATA = global_data.first_half;

        let selector = _to_u8(GLOBAL_DATA, 0) as usize;
        let mut td: TooDee<CustomType0>;

        match selector % 4 {
            0 => {
                let cols = _to_usize(GLOBAL_DATA, 8) % 65 + 1;
                let rows = _to_usize(GLOBAL_DATA, 16) % 65 + 1;
                td = TooDee::new(cols, rows);
            }
            1 => {
                let cols = _to_usize(GLOBAL_DATA, 8) % 65 + 1;
                let rows = _to_usize(GLOBAL_DATA, 16) % 65 + 1;
                let s = _to_str(GLOBAL_DATA, 24, 40).to_string();
                td = TooDee::init(cols, rows, CustomType0(s));
            }
            2 => {
                let capacity = _to_usize(GLOBAL_DATA, 8);
                td = TooDee::with_capacity(capacity);
            }
            _ => {
                let cols = _to_usize(GLOBAL_DATA, 8) % 65 + 1;
                let rows = _to_usize(GLOBAL_DATA, 16) % 65 + 1;
                let mut v = Vec::with_capacity(cols * rows);
                for i in 0..(cols * rows) {
                    let b = _to_u8(GLOBAL_DATA, 24 + (i % (GLOBAL_DATA.len() - 24)));
                    v.push(CustomType0((b as char).to_string()));
                }
                td = TooDee::from_vec(cols, rows, v);
            }
        }

        let ops = (_to_u8(GLOBAL_DATA, 4) as usize) % 30 + 1;
        for i in 0..ops {
            let op = _to_u8(GLOBAL_DATA, 32 + i) % 10;
            match op {
                0 => {
                    let col_off = (i * 7 + 40) % (GLOBAL_DATA.len() - 8);
                    let row_off = (i * 11 + 60) % (GLOBAL_DATA.len() - 8);
                    let col = _to_usize(GLOBAL_DATA, col_off);
                    let row = _to_usize(GLOBAL_DATA, row_off);
                    let cell = td.index_mut((col, row));
                    println!("{:?}", cell);
                }
                1 => {
                    let s_col_off = (i * 5 + 100) % (GLOBAL_DATA.len() - 8);
                    let s_row_off = (i * 9 + 110) % (GLOBAL_DATA.len() - 8);
                    let e_col_off = (i * 3 + 120) % (GLOBAL_DATA.len() - 8);
                    let e_row_off = (i * 13 + 130) % (GLOBAL_DATA.len() - 8);
                    let start = (
                        _to_usize(GLOBAL_DATA, s_col_off),
                        _to_usize(GLOBAL_DATA, s_row_off),
                    );
                    let end = (
                        _to_usize(GLOBAL_DATA, e_col_off),
                        _to_usize(GLOBAL_DATA, e_row_off),
                    );
                    let view = td.view(start, end);
                    for r in view.rows() {
                        println!("{:?}", r.len());
                        break;
                    }
                }
                2 => {
                    let s_col_off = (i * 15 + 140) % (GLOBAL_DATA.len() - 8);
                    let s_row_off = (i * 17 + 150) % (GLOBAL_DATA.len() - 8);
                    let e_col_off = (i * 19 + 160) % (GLOBAL_DATA.len() - 8);
                    let e_row_off = (i * 21 + 170) % (GLOBAL_DATA.len() - 8);
                    let start = (
                        _to_usize(GLOBAL_DATA, s_col_off),
                        _to_usize(GLOBAL_DATA, s_row_off),
                    );
                    let end = (
                        _to_usize(GLOBAL_DATA, e_col_off),
                        _to_usize(GLOBAL_DATA, e_row_off),
                    );
                    let mut view_mut = td.view_mut(start, end);
                    let cell = view_mut.index_mut((0, 0));
                    println!("{:?}", cell);
                }
                3 => {
                    let mut rows = td.rows_mut();
                    if let Some(r) = rows.next() {
                        println!("{:?}", r.len());
                    }
                }
                4 => {
                    let col_off = (i * 23 + 180) % (GLOBAL_DATA.len() - 8);
                    let col_idx = _to_usize(GLOBAL_DATA, col_off);
                    let mut col_iter = td.col_mut(col_idx);
                    if let Some(c) = col_iter.next() {
                        println!("{:?}", c);
                    }
                }
                5 => {
                    let cols = td.num_cols();
                    if cols > 0 {
                        let row_len = if cols > 65 { 65 } else { cols };
                        let mut row_vec = Vec::with_capacity(row_len);
                        for j in 0..row_len {
                            let b = _to_u8(GLOBAL_DATA, 200 + j);
                            row_vec.push(CustomType0((b as char).to_string()));
                        }
                        td.push_row(row_vec);
                    }
                }
                6 => {
                    let rows_n = td.num_rows();
                    if rows_n > 0 {
                        let len = if rows_n > 65 { 65 } else { rows_n };
                        let mut col_vec = Vec::with_capacity(len);
                        for j in 0..len {
                            let b = _to_u8(GLOBAL_DATA, 240 + j);
                            col_vec.push(CustomType0((b as char).to_string()));
                        }
                        td.push_col(col_vec);
                    }
                }
                7 => {
                    let r1 = _to_usize(GLOBAL_DATA, 300);
                    let r2 = _to_usize(GLOBAL_DATA, 308);
                    td.swap_rows(r1, r2);
                }
                8 => {
                    let idx = _to_usize(GLOBAL_DATA, 316);
                    let mut drain = td.remove_col(idx);
                    if let Some(item) = drain.next() {
                        println!("{:?}", item);
                    }
                }
                _ => {
                    td.pop_col();
                }
            }
        }

        let final_col = _to_usize(GLOBAL_DATA, 64);
        let final_row = _to_usize(GLOBAL_DATA, 72);
        let cell = td.index_mut((final_col, final_row));
        println!("{:?}", cell);
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