#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use toodee::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

#[derive(Clone, Debug, Default)]
struct CustomType0(pub String);

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 2048 {
            return;
        }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let mut cursor = 0usize;

        let num_cols = _to_usize(GLOBAL_DATA, cursor);
        cursor += 8;
        let num_rows = _to_usize(GLOBAL_DATA, cursor);
        cursor += 8;

        let vec_cap = (_to_u8(GLOBAL_DATA, cursor) % 65) as usize;
        cursor += 1;

        let mut base_vec: Vec<CustomType0> = Vec::with_capacity(vec_cap);
        for _ in 0..vec_cap {
            let slen = (_to_u8(GLOBAL_DATA, cursor) % 17) as usize;
            cursor += 1;
            let s = _to_str(GLOBAL_DATA, cursor, cursor + slen);
            cursor += slen;
            base_vec.push(CustomType0(String::from(s)));
        }

        let init_val = base_vec.get(0).cloned().unwrap_or_default();

        let constructor_choice = _to_u8(GLOBAL_DATA, cursor);
        cursor += 1;

        let mut td: TooDee<CustomType0> = match constructor_choice % 5 {
            0 => TooDee::new(num_cols, num_rows),
            1 => TooDee::init(num_cols, num_rows, init_val.clone()),
            2 => TooDee::from_vec(num_cols, num_rows, base_vec.clone()),
            3 => {
                let cap = _to_usize(GLOBAL_DATA, cursor);
                cursor += 8;
                TooDee::with_capacity(cap)
            }
            _ => TooDee::from_box(num_cols, num_rows, base_vec.clone().into_boxed_slice()),
        };

        let ops_count = (_to_u8(GLOBAL_DATA, cursor) % 10) as usize;
        cursor += 1;

        for _ in 0..ops_count {
            let op_code = _to_u8(GLOBAL_DATA, cursor);
            cursor += 1;
            match op_code % 8 {
                0 => {
                    let row_len = (_to_u8(GLOBAL_DATA, cursor) % 65) as usize;
                    cursor += 1;
                    let mut row_vec = Vec::with_capacity(row_len);
                    for _ in 0..row_len {
                        let sl = (_to_u8(GLOBAL_DATA, cursor) % 17) as usize;
                        cursor += 1;
                        let st = _to_str(GLOBAL_DATA, cursor, cursor + sl);
                        cursor += sl;
                        row_vec.push(CustomType0(String::from(st)));
                    }
                    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                        td.push_row(row_vec);
                    }));
                }
                1 => {
                    if td.num_rows() > 0 {
                        let mut col_vec = Vec::with_capacity(td.num_rows());
                        for _ in 0..td.num_rows() {
                            let sl = (_to_u8(GLOBAL_DATA, cursor) % 17) as usize;
                            cursor += 1;
                            let st = _to_str(GLOBAL_DATA, cursor, cursor + sl);
                            cursor += sl;
                            col_vec.push(CustomType0(String::from(st)));
                        }
                        let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                            td.push_col(col_vec);
                        }));
                    }
                }
                2 => {
                    if td.num_rows() > 1 {
                        let r1 = _to_usize(GLOBAL_DATA, cursor);
                        cursor += 8;
                        let r2 = _to_usize(GLOBAL_DATA, cursor);
                        cursor += 8;
                        let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                            td.swap_rows(r1, r2);
                        }));
                    }
                }
                3 => {
                    if td.num_cols() > 0 && td.num_rows() > 0 {
                        let start = (
                            _to_usize(GLOBAL_DATA, cursor),
                            _to_usize(GLOBAL_DATA, cursor + 8),
                        );
                        let end = (
                            _to_usize(GLOBAL_DATA, cursor + 16),
                            _to_usize(GLOBAL_DATA, cursor + 24),
                        );
                        cursor += 32;
                        let mut view_mut = td.view_mut(start, end);
                        let col_idx = _to_usize(GLOBAL_DATA, cursor);
                        cursor += 8;
                        let mut col_iter = view_mut.col_mut(col_idx);
                        let nth_idx = _to_usize(GLOBAL_DATA, cursor);
                        cursor += 8;
                        col_iter.nth(nth_idx);
                    }
                }
                4 => {
                    let mut rows_iter = td.rows_mut();
                    let nth_idx = _to_usize(GLOBAL_DATA, cursor);
                    cursor += 8;
                    let _ = rows_iter.nth(nth_idx);
                }
                5 => {
                    if td.num_cols() > 0 {
                        let col_idx = _to_usize(GLOBAL_DATA, cursor);
                        cursor += 8;
                        let mut drain = td.remove_col(col_idx);
                        let _ = drain.next();
                    }
                }
                6 => {
                    let col_idx = _to_usize(GLOBAL_DATA, cursor);
                    cursor += 8;
                    let mut col_iter = td.col_mut(col_idx);
                    let nth_idx = _to_usize(GLOBAL_DATA, cursor);
                    cursor += 8;
                    col_iter.nth(nth_idx);
                }
                _ => {
                    if !td.is_empty() {
                        let data_ref = td.data();
                        println!("{:?}", &data_ref[0]);
                    }
                }
            }
        }

        if !td.is_empty() {
            let data_ref = td.data();
            println!("{:?}", &data_ref[0]);
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