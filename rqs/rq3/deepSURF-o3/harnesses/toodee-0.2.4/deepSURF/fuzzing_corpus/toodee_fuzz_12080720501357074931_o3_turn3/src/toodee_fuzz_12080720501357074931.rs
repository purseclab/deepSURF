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

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 64 {
            return;
        }

        set_global_data(data);
        let gd = get_global_data();
        let first = gd.first_half;
        let second = gd.second_half;

        let choice = _to_u8(first, 0) as usize % 4;
        let cols = _to_usize(first, 1);
        let rows = _to_usize(first, 9);
        let capacity = _to_usize(first, 17);

        let vec_len = (_to_u8(first, 25) as usize % 65) + 1;
        let mut seed_vec: Vec<CustomType0> = Vec::new();
        for i in 0..vec_len {
            let idx = 24 + (i % 4);
            let ch = _to_char(first, idx);
            seed_vec.push(CustomType0(ch.to_string()));
        }

        let mut td: TooDee<CustomType0> = match choice {
            0 => TooDee::with_capacity(capacity),
            1 => TooDee::new(cols, rows),
            2 => {
                let target = cols.saturating_mul(rows);
                let mut v = seed_vec.clone();
                while v.len() < target {
                    v.extend(seed_vec.clone());
                }
                v.truncate(target);
                TooDee::from_vec(cols, rows, v)
            }
            _ => TooDee::init(cols, rows, CustomType0(String::from("init"))),
        };

        let row_len = (_to_u8(second, 0) as usize % 65) + 1;
        let mut row_vec: Vec<CustomType0> = Vec::new();
        for i in 0..row_len {
            let idx = 24 + (i % 4);
            let ch = _to_char(second, idx);
            row_vec.push(CustomType0(ch.to_string()));
        }

        td.push_row(row_vec.clone());
        td.push_col(row_vec.clone());

        let r1 = _to_usize(second, 8);
        let r2 = _to_usize(second, 16);
        td.swap_rows(r1, r2);

        let start = (_to_usize(first, 24), _to_usize(first, 24));
        let end = (_to_usize(first, 24), _to_usize(first, 24));

        {
            let view = td.view(start, end);
            let mut rows_iter = view.rows();
            let _ = rows_iter.next();
            let mut col_iter = view.col(0);
            let _ = col_iter.next_back();
        }

        {
            let mut view_mut = td.view_mut(start, end);
            let mut rows_mut_iter = view_mut.rows_mut();
            let _ = rows_mut_iter.nth_back(0);
            let mut col_mut_iter = view_mut.col_mut(0);
            let _ = col_mut_iter.next();
        }

        let idx = _to_usize(second, 24);
        {
            let mut drain = td.remove_col(idx);
            let _ = drain.next();
            let _ = drain.next_back();
        }

        td.insert_col(idx, row_vec.clone());
        td.insert_row(idx, row_vec.clone());

        {
            if let Some(mut drain2) = td.pop_col() {
                let _ = drain2.next();
            }
        }

        println!("{:?}", td.num_cols());
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