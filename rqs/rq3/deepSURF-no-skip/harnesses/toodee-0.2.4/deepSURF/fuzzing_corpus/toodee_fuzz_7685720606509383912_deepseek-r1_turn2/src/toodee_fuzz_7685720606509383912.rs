#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use toodee::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

#[derive(Debug, Default, Clone)]
struct CustomType0(String);

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 256 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let mut idx = 0;

        if GLOBAL_DATA.len() < 9 { return; }
        let ctor_type = _to_u8(GLOBAL_DATA, idx);
        idx += 1;

        let capacity = _to_usize(GLOBAL_DATA, idx);
        idx += 8;
        let mut t = match ctor_type % 3 {
            0 => TooDee::<CustomType0>::with_capacity(capacity),
            1 => {
                let num_cols = _to_usize(GLOBAL_DATA, idx) % 65;
                idx += 8;
                let num_rows = _to_usize(GLOBAL_DATA, idx) % 65;
                idx += 8;
                let cells = num_cols.checked_mul(num_rows).unwrap_or(0);
                let mut vec = Vec::with_capacity(cells);
                vec.resize_with(cells, || CustomType0(String::new()));
                TooDee::from_vec(num_cols, num_rows, vec)
            }
            _ => {
                let num_cols = _to_usize(GLOBAL_DATA, idx) % 65;
                idx += 8;
                let num_rows = _to_usize(GLOBAL_DATA, idx) % 65;
                idx += 8;
                let default_val = CustomType0(String::new());
                TooDee::init(num_cols, num_rows, default_val)
            }
        };

        let num_ops = _to_u8(GLOBAL_DATA, idx);
        idx += 1;

        for _ in 0..num_ops {
            if idx >= GLOBAL_DATA.len() { break; }
            let op_select = _to_u8(GLOBAL_DATA, idx);
            idx += 1;

            match op_select % 11 {
                0 => {
                    let start = (_to_usize(GLOBAL_DATA, idx), _to_usize(GLOBAL_DATA, idx + 8));
                    idx += 16;
                    let end = (_to_usize(GLOBAL_DATA, idx), _to_usize(GLOBAL_DATA, idx + 8));
                    idx += 16;
                    let view_mut = t.view_mut(start, end);
                    let mut new_t = TooDee::from(view_mut);
                    let col_idx = _to_usize(GLOBAL_DATA, idx);
                    idx += 8;
                    let _ = new_t.col_mut(col_idx);
                }
                1 => {
                    let coord_start = (_to_usize(GLOBAL_DATA, idx), _to_usize(GLOBAL_DATA, idx + 8));
                    idx += 16;
                    let coord_end = (_to_usize(GLOBAL_DATA, idx), _to_usize(GLOBAL_DATA, idx + 8));
                    idx += 16;
                    let view = t.view(coord_start, coord_end);
                    let _ = view.col(0);
                }
                2 => {
                    let r1 = _to_usize(GLOBAL_DATA, idx);
                    idx += 8;
                    let r2 = _to_usize(GLOBAL_DATA, idx);
                    idx += 8;
                    t.swap_rows(r1, r2);
                }
                3 => {
                    let _drain = t.remove_col(_to_usize(GLOBAL_DATA, idx));
                    idx += 8;
                }
                4 => {
                    let col = _to_usize(GLOBAL_DATA, idx);
                    idx += 8;
                    let _col_mut = t.col_mut(col);
                }
                5 => {
                    let rows = _to_usize(GLOBAL_DATA, idx);
                    idx += 8;
                    let _rows_mut = t.rows_mut().nth(rows);
                }
                6 => {
                    let col = _to_usize(GLOBAL_DATA, idx);
                    idx += 8;
                    let _ = t.pop_col();
                }
                7 => {
                    let row = _to_usize(GLOBAL_DATA, idx);
                    idx += 8;
                    let _ = t.remove_row(row);
                }
                8 => {
                    let idx_val = _to_usize(GLOBAL_DATA, idx);
                    idx += 8;
                    let _ = t.index(idx_val);
                }
                9 => {
                    let coord = (_to_usize(GLOBAL_DATA, idx), _to_usize(GLOBAL_DATA, idx + 8));
                    idx += 16;
                    t.view_mut(coord, (coord.0 + 1, coord.1 + 1));
                }
                _ => {
                    let coord = (_to_usize(GLOBAL_DATA, idx), _to_usize(GLOBAL_DATA, idx + 8));
                    idx += 16;
                    t.index_mut(coord);
                }
            }
        }
    });
}

// Type converter functions follow but are excluded as per the directions

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