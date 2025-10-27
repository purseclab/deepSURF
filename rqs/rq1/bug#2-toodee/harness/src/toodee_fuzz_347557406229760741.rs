#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use toodee::*;
use global_data::*;
use std::ops::{Index, IndexMut};

#[derive(Debug, Clone, Default)]
struct CustomType0(String);

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 256 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let op_count = _to_usize(GLOBAL_DATA, 0) % 4;
        
        for i in 0..op_count {
            let base = 8 + i * 24;
            match _to_u8(GLOBAL_DATA, base) % 4 {
                0 => {
                    let capacity = _to_usize(GLOBAL_DATA, base + 8);
                    let mut t1 = TooDee::<CustomType0>::with_capacity(capacity);
                    if capacity > 0 {
                        t1.push_row(vec![CustomType0(String::new()); _to_usize(GLOBAL_DATA, base + 16) % 65]);
                    }
                    let view = t1.view((0, 0), (1, 1));
                    let mut rows = view.rows();
                    let idx = _to_usize(GLOBAL_DATA, base + 24);
                    let _ = rows.nth(idx);
                    println!("{:?}", view[(0,0)]);
                }
                
                1 => {
                    let cols = _to_usize(GLOBAL_DATA, base + 8) % 65;
                    let rows = _to_usize(GLOBAL_DATA, base + 16) % 65;
                    let mut t2 = TooDee::init(cols, rows, CustomType0(String::new()));
                    let cols = t2.num_cols();
                    let rows = t2.num_rows();
                    let mut view_mut = t2.view_mut((0,0), (cols, rows));
                    view_mut.swap_rows(
                        _to_usize(GLOBAL_DATA, base + 24),
                        _to_usize(GLOBAL_DATA, base + 32)
                    );
                    let mut t3 = TooDee::from(view_mut);
                    let col_idx = _to_usize(GLOBAL_DATA, base + 40) % cols;
                    let _ = t3.pop_col();
                    let mut rows_mut = t3.rows_mut();
                    rows_mut.nth(_to_usize(GLOBAL_DATA, base + 48));
                }
                
                2 => {
                    let cols = _to_usize(GLOBAL_DATA, base + 8) % 65;
                    let rows = _to_usize(GLOBAL_DATA, base + 16) % 65;
                    let elements = vec![CustomType0(String::new()); cols*rows];
                    let mut t4 = TooDee::from_vec(cols, rows, elements);
                    let col_idx = _to_usize(GLOBAL_DATA, base + 24) % cols;
                    let row_idx = _to_usize(GLOBAL_DATA, base + 32) % rows;
                    let _ = t4.insert_col(col_idx, vec![CustomType0(String::new()); rows]);
                    let mut col_mut = t4.col_mut(col_idx);
                    col_mut.nth(row_idx);
                    let mut view = t4.view((0,0), (cols, rows));
                    let mut rows = view.rows();
                    rows.nth(_to_usize(GLOBAL_DATA, base + 40));
                }
                
                3 => {
                    let cols = _to_usize(GLOBAL_DATA, base + 8) % 65;
                    let rows = _to_usize(GLOBAL_DATA, base + 16) % 65;
                    let mut t5 = TooDee::new(cols, rows);
                    let col_idx = _to_usize(GLOBAL_DATA, base + 24) % cols;
                    let row_idx = _to_usize(GLOBAL_DATA, base + 32) % rows;
                    t5[(col_idx, row_idx)] = CustomType0(String::from(_to_str(GLOBAL_DATA, base + 40, base + 56)));
                    let _ = t5.remove_col(col_idx);
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