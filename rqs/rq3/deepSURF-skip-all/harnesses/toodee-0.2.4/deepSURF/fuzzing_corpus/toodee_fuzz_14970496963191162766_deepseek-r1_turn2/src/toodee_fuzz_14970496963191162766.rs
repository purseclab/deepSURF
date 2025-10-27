#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use toodee::*;
use global_data::*;
use std::ops::{Deref, DerefMut, Index, IndexMut};

#[derive(Debug, Default, Clone)]
struct CustomType0(String);

fn main (){
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 2048 {return;}
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        
        let mut offset = 0;
        let constructor_type = _to_u8(GLOBAL_DATA, offset) % 3;
        offset += 8;
        
        let mut t = match constructor_type {
            0 => {
                let capacity = _to_usize(GLOBAL_DATA, offset);
                offset += 8;
                TooDee::<CustomType0>::with_capacity(capacity)
            },
            1 => {
                let cols = _to_usize(GLOBAL_DATA, offset);
                offset += 8;
                let rows = _to_usize(GLOBAL_DATA, offset);
                offset += 8;
                let mut vec = Vec::new();
                for _ in 0..cols.wrapping_mul(rows) {
                    vec.push(CustomType0(_to_str(GLOBAL_DATA, offset, offset+8).to_string()));
                    offset += 8;
                }
                TooDee::from_vec(cols, rows, vec)
            },
            2 => {
                let cols = _to_usize(GLOBAL_DATA, offset);
                offset += 8;
                let rows = _to_usize(GLOBAL_DATA, offset);
                offset += 8;
                TooDee::new(cols, rows)
            },
            _ => unreachable!()
        };

        let ops_count = _to_u8(GLOBAL_DATA, offset) % 5;
        offset += 8;
        
        for _ in 0..ops_count {
            let op_type = _to_u8(GLOBAL_DATA, offset) % 4;
            offset += 8;
            
            match op_type {
                0 => {
                    let start_col = _to_usize(GLOBAL_DATA, offset);
                    offset += 8;
                    let start_row = _to_usize(GLOBAL_DATA, offset);
                    offset += 8;
                    let end_col = _to_usize(GLOBAL_DATA, offset);
                    offset += 8;
                    let end_row = _to_usize(GLOBAL_DATA, offset);
                    offset += 8;
                    let view = t.view((start_col, start_row), (end_col, end_row));
                    println!("View dimensions: {}x{}", view.num_cols(), view.num_rows());
                },
                1 => {
                    let col_idx = _to_usize(GLOBAL_DATA, offset);
                    offset += 8;
                    let mut col = t.col_mut(col_idx);
                    let idx = _to_usize(GLOBAL_DATA, offset);
                    offset += 8;
                    let _ = col.nth_back(idx);
                },
                2 => {
                    let r1 = _to_usize(GLOBAL_DATA, offset);
                    offset += 8;
                    let r2 = _to_usize(GLOBAL_DATA, offset);
                    offset += 8;
                    t.swap_rows(r1, r2);
                },
                3 => {
                    let mut rows = t.rows_mut();
                    let row_idx = _to_usize(GLOBAL_DATA, offset) % 65;
                    offset += 8;
                    if let Some(row) = rows.nth_back(row_idx) {
                        println!("Row length: {}", row.len());
                    }
                },
                _ => unreachable!()
            }
        }

        let col_idx = _to_usize(GLOBAL_DATA, offset);
        offset += 8;
        let mut col = t.col_mut(col_idx);
        let idx = _to_usize(GLOBAL_DATA, offset);
        col.nth_back(idx);

        let push_col_data = (0.._to_usize(GLOBAL_DATA, offset) % 65)
            .map(|i| CustomType0(_to_str(GLOBAL_DATA, offset + i*8, offset + (i+1)*8).to_string()))
            .collect::<Vec<_>>();
        t.push_col(push_col_data.into_iter());

        let insert_idx = _to_usize(GLOBAL_DATA, offset + 64);
        let insert_data = (0.._to_usize(GLOBAL_DATA, offset + 72) % 65)
            .map(|i| CustomType0(_to_str(GLOBAL_DATA, offset + 80 + i*8, offset + 88 + i*8).to_string()))
            .collect::<Vec<_>>();
        t.insert_col(insert_idx, insert_data.into_iter());
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