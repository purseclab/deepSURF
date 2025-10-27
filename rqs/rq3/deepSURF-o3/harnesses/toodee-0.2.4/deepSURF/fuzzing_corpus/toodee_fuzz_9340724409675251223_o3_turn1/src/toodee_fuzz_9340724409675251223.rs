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

fn build_row(count: usize) -> Vec<CustomType0> {
    let mut v = Vec::new();
    for _ in 0..count {
        v.push(CustomType0(String::from("row")));
    }
    v
}

fn build_col(count: usize) -> Vec<CustomType0> {
    let mut v = Vec::new();
    for _ in 0..count {
        v.push(CustomType0(String::from("col")));
    }
    v
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 64 {
            return;
        }
        set_global_data(data);
        let global_data = get_global_data();
        let first = global_data.first_half;
        let second = global_data.second_half;

        let constructor_sel = _to_u8(first, 0) % 5;
        let cols_raw = _to_usize(first, 1);
        let rows_raw = _to_usize(first, 9);
        let cols = cols_raw % 8 + 1;
        let rows = rows_raw % 8 + 1;
        let capacity = _to_usize(first, 17) % 65;

        let mut td: TooDee<CustomType0> = match constructor_sel {
            0 => TooDee::new(cols, rows),
            1 => TooDee::with_capacity(capacity),
            2 => {
                let init_val = CustomType0(String::from("init"));
                TooDee::init(cols, rows, init_val)
            }
            3 => {
                let total = cols * rows;
                let mut vec = Vec::new();
                for _ in 0..total {
                    vec.push(CustomType0(String::from("vec")));
                }
                TooDee::from_vec(cols, rows, vec)
            }
            _ => {
                let total = cols * rows;
                let mut vec = Vec::new();
                for _ in 0..total {
                    vec.push(CustomType0(String::from("box")));
                }
                let boxed = vec.into_boxed_slice();
                TooDee::from_box(cols, rows, boxed)
            }
        };

        let mut idx_second = 0usize;
        let ops = (second.len() % 20) + 1;
        for _ in 0..ops {
            if idx_second >= second.len() {
                break;
            }
            let op = second[idx_second] % 10;
            idx_second += 1;
            match op {
                0 => {
                    if td.num_rows() > 0 {
                        let idx = _to_usize(first, (idx_second % (first.len().saturating_sub(8))).max(1))
                            % td.num_rows();
                        let row_slice = td.index_mut(idx);
                        if !row_slice.is_empty() {
                            println!("{:?}", &row_slice[0]);
                        }
                    }
                }
                1 => {
                    let len = (td.num_cols().max(1) % 65) + 1;
                    td.push_row(build_row(len));
                }
                2 => {
                    let len = (td.num_rows().max(1) % 65) + 1;
                    td.push_col(build_col(len));
                }
                3 => {
                    td.pop_col();
                }
                4 => {
                    if td.num_rows() > 0 {
                        let r1 = _to_usize(first, (idx_second % (first.len().saturating_sub(8))).max(1))
                            % td.num_rows();
                        let r2 = _to_usize(first, ((idx_second + 8) % (first.len().saturating_sub(8))).max(1))
                            % td.num_rows();
                        td.swap_rows(r1, r2);
                    }
                }
                5 => {
                    let view = td.view((0, 0), (td.num_cols(), td.num_rows()));
                    println!("{:?}", view.num_cols());
                }
                6 => {
                    if td.num_cols() > 0 {
                        let c = _to_usize(first, (idx_second % (first.len().saturating_sub(8))).max(1))
                            % td.num_cols();
                        let mut col_iter = td.col_mut(c);
                        if let Some(cell) = col_iter.next() {
                            println!("{:?}", cell);
                        }
                    }
                }
                7 => {
                    let mut rows_iter = td.rows_mut();
                    rows_iter.next();
                }
                8 => {
                    if td.num_cols() > 0 {
                        let c = _to_usize(first, (idx_second % (first.len().saturating_sub(8))).max(1))
                            % td.num_cols();
                        let mut drain = td.remove_col(c);
                        drain.next();
                    }
                }
                _ => {
                    if td.num_cols() > 0 && td.num_rows() > 0 {
                        let mut view_mut = td.view_mut((0, 0), (td.num_cols(), td.num_rows()));
                        println!("{:?}", view_mut.num_rows());
                    }
                }
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