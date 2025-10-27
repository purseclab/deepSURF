#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use toodee::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

#[derive(Default, Clone, Debug)]
struct CustomType0(String);

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 4096 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let mut offset = 0;
        let ctor_choice = _to_u8(GLOBAL_DATA, offset);
        offset += 1;

        let mut dee = if ctor_choice % 3 == 0 {
            let cap = _to_usize(GLOBAL_DATA, offset);
            offset += 8;
            TooDee::<CustomType0>::with_capacity(cap)
        } else if ctor_choice % 3 == 1 {
            let cols = _to_usize(GLOBAL_DATA, offset);
            offset += 8;
            let rows = _to_usize(GLOBAL_DATA, offset);
            offset += 8;
            TooDee::new(cols, rows)
        } else {
            let cols = _to_usize(GLOBAL_DATA, offset);
            offset += 8;
            let rows = _to_usize(GLOBAL_DATA, offset);
            offset += 8;
            let mut v = Vec::with_capacity(cols * rows);
            v.resize_with(cols * rows, || CustomType0(String::new()));
            TooDee::from_vec(cols, rows, v)
        };

        let op_count = _to_u8(GLOBAL_DATA, offset) % 65;
        offset += 1;

        for _ in 0..op_count {
            let op_type = _to_u8(GLOBAL_DATA, offset) % 9;
            offset += 1;

            match op_type {
                0 => {
                    let r1 = _to_usize(GLOBAL_DATA, offset);
                    offset += 8;
                    let r2 = _to_usize(GLOBAL_DATA, offset);
                    offset += 8;
                    dee.swap_rows(r1, r2);
                }
                1 => {
                    let s_col = _to_usize(GLOBAL_DATA, offset);
                    offset += 8;
                    let s_row = _to_usize(GLOBAL_DATA, offset);
                    offset += 8;
                    let e_col = _to_usize(GLOBAL_DATA, offset);
                    offset += 8;
                    let e_row = _to_usize(GLOBAL_DATA, offset);
                    offset += 8;
                    let view = dee.view_mut((s_col, s_row), (e_col, e_row));
                    let _ = TooDee::from(view);
                }
                2 => {
                    let col = _to_usize(GLOBAL_DATA, offset);
                    offset += 8;
                    if let Some(c) = dee.col_mut(col).next() {
                        *c = CustomType0("modified".into());
                        println!("{:?}", *c);
                    }
                }
                3 => {
                    let idx = _to_usize(GLOBAL_DATA, offset);
                    offset += 8;
                    let _ = dee.remove_col(idx);
                }
                4 => {
                    let cnt = _to_usize(GLOBAL_DATA, offset) % 65;
                    offset += 8;
                    let elements = (0..cnt).map(|i| CustomType0(i.to_string())).collect::<Vec<_>>();
                    dee.push_row(elements);
                }
                5 => {
                    let idx = _to_usize(GLOBAL_DATA, offset);
                    offset += 8;
                    let cnt = _to_usize(GLOBAL_DATA, offset) % 65;
                    offset += 8;
                    let elements = (0..cnt).map(|i| CustomType0(i.to_string())).collect::<Vec<_>>();
                    dee.insert_row(idx, elements);
                }
                6 => {
                    let s_col = _to_usize(GLOBAL_DATA, offset);
                    offset += 8;
                    let s_row = _to_usize(GLOBAL_DATA, offset);
                    offset += 8;
                    let _ = dee.view((s_col, s_row), (s_col + 1, s_row + 1));
                }
                7 => {
                    let row_idx = _to_usize(GLOBAL_DATA, offset);
                    offset += 8;
                    let _ = dee.rows().nth(row_idx);
                }
                8 => {
                    let col_idx = _to_usize(GLOBAL_DATA, offset);
                    offset += 8;
                    let _ = dee.col_mut(col_idx).last();
                }
                _ => {}
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