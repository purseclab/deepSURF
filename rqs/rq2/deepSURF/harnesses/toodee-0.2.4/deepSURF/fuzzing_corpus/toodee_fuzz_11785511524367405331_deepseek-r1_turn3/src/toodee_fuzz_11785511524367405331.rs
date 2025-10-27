#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use toodee::*;
use global_data::*;
use std::ops::{IndexMut, Index};

#[derive(Debug, Clone)]
struct CustomType0(String);

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 4000 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let mut data_idx = 0;
    
        let rows = _to_usize(GLOBAL_DATA, data_idx);
        data_idx += 8;
        let cols = _to_usize(GLOBAL_DATA, data_idx);
        data_idx += 8;

        let mut vec_data = Vec::with_capacity(64);
        for _ in 0..64 {
            let str_len = _to_u8(GLOBAL_DATA, data_idx) % 17;
            data_idx += 1;
            let s = _to_str(GLOBAL_DATA, data_idx, data_idx + str_len as usize);
            vec_data.push(CustomType0(s.to_string()));
            data_idx += str_len as usize;
        }

        let constructor_choice = _to_u8(GLOBAL_DATA, data_idx) % 3;
        data_idx += 1;
        let mut too_dee = match constructor_choice {
            0 => TooDee::from_vec(cols, rows, vec_data),
            1 => {
                let boxed = vec_data.into_boxed_slice();
                TooDee::from_box(cols, rows, boxed)
            },
            _ => TooDee::init(cols, rows, CustomType0(String::new()))
        };

        let ops_count = _to_usize(GLOBAL_DATA, data_idx) % 8;
        data_idx += 8;

        for _ in 0..ops_count {
            let op_type = _to_u8(GLOBAL_DATA, data_idx) % 5;
            data_idx += 1;
            
            match op_type {
                0 => {
                    let start = (_to_usize(GLOBAL_DATA, data_idx), _to_usize(GLOBAL_DATA, data_idx + 8));
                    data_idx += 16;
                    let end = (_to_usize(GLOBAL_DATA, data_idx), _to_usize(GLOBAL_DATA, data_idx + 8));
                    data_idx += 16;
                    let mut view = too_dee.view_mut(start, end);
                    let col_idx = _to_usize(GLOBAL_DATA, data_idx);
                    data_idx += 8;
                    let mut col = view.col_mut(col_idx);
                    let idx = _to_usize(GLOBAL_DATA, data_idx);
                    data_idx += 8;
                    let _ = col.nth(idx);
                    let count = _to_usize(GLOBAL_DATA, data_idx) % 5;
                    data_idx += 8;
                    for _ in 0..count {
                        let _ = view.rows_mut().next();
                    }
                }
                1 => {
                    let start = (_to_usize(GLOBAL_DATA, data_idx), _to_usize(GLOBAL_DATA, data_idx + 8));
                    data_idx += 16;
                    let end = (_to_usize(GLOBAL_DATA, data_idx), _to_usize(GLOBAL_DATA, data_idx + 8));
                    data_idx += 16;
                    let mut view = too_dee.view_mut(start, end);
                    let r1 = _to_usize(GLOBAL_DATA, data_idx);
                    data_idx += 8;
                    let r2 = _to_usize(GLOBAL_DATA, data_idx);
                    data_idx += 8;
                    view.swap_rows(r1, r2);
                    let _row = view.rows().next_back();
                }
                2 => {
                    let start = (_to_usize(GLOBAL_DATA, data_idx), _to_usize(GLOBAL_DATA, data_idx + 8));
                    data_idx += 16;
                    let end = (_to_usize(GLOBAL_DATA, data_idx), _to_usize(GLOBAL_DATA, data_idx + 8));
                    data_idx += 16;
                    let mut outer_view = too_dee.view_mut(start, end);
                    let start_inner = (_to_usize(GLOBAL_DATA, data_idx), _to_usize(GLOBAL_DATA, data_idx + 8));
                    data_idx += 16;
                    let end_inner = (_to_usize(GLOBAL_DATA, data_idx), _to_usize(GLOBAL_DATA, data_idx + 8));
                    data_idx += 16;
                    let mut inner_view = outer_view.view_mut(start_inner, end_inner);
                    let idx = _to_usize(GLOBAL_DATA, data_idx);
                    data_idx += 8;
                    let cell = inner_view.index_mut(idx);
                    println!("{:?}", cell);
                    let col = _to_usize(GLOBAL_DATA, data_idx) % inner_view.num_cols();
                    data_idx += 8;
                    let _ = inner_view.col_mut(col).nth_back(0);
                }
                3 => {
                    let mut view = too_dee.view_mut((0, 0), (cols, rows));
                    let row_idx = _to_usize(GLOBAL_DATA, data_idx);
                    data_idx += 8;
                    let row = view.index_mut(row_idx);
                    let cell_idx = _to_usize(GLOBAL_DATA, data_idx);
                    data_idx += 8;
                    let _ = &row[cell_idx];
                    if let Some(mut c) = too_dee.pop_col() {
                        let _ = c.next();
                    }
                }
                _ => {
                    let col_idx = _to_usize(GLOBAL_DATA, data_idx) % too_dee.num_cols();
                    data_idx += 8;
                    let _ = too_dee.col_mut(col_idx).nth(0);
                    let insert_pos = _to_usize(GLOBAL_DATA, data_idx) % (too_dee.num_rows() + 1);
                    data_idx += 8;
                    too_dee.insert_row(insert_pos, vec![CustomType0(String::new())]);
                }
            }
        }

        let final_view_start = (_to_usize(GLOBAL_DATA, data_idx), _to_usize(GLOBAL_DATA, data_idx + 8));
        data_idx += 16;
        let final_view_end = (_to_usize(GLOBAL_DATA, data_idx), _to_usize(GLOBAL_DATA, data_idx + 8));
        data_idx += 16;
        let mut final_view = too_dee.view_mut(final_view_start, final_view_end);
        let target_idx = _to_usize(GLOBAL_DATA, data_idx);
        let cell = final_view.index_mut(target_idx);
        println!("Final cell: {:?}", cell);
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