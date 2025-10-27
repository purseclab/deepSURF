#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;    

use toodee::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};
fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 8192 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let SECOND_HALF = global_data.second_half;

        let num_cols = (_to_usize(GLOBAL_DATA, 0) % 65) as usize;
        let num_rows = (_to_usize(GLOBAL_DATA, 8) % 65) as usize;
        let mut toodee = TooDee::new(num_cols, num_rows);

        let op_count = _to_u8(GLOBAL_DATA, 16) % 5;
        for _ in 0..op_count {
            let op_selector = _to_u8(GLOBAL_DATA, 17) % 4;
            match op_selector {
                0 => {
                    let row_idx = _to_usize(GLOBAL_DATA, 18) % (num_rows + 1);
                    let row_data = &SECOND_HALF[_to_usize(GLOBAL_DATA, 26)..];
                    let row_len = (row_data.len() % 65).min(num_cols);
                    let row = row_data[..row_len].to_vec();
                    toodee.insert_row(row_idx, row);
                }
                1 => {
                    let col_idx = _to_usize(GLOBAL_DATA, 34) % (num_cols + 1);
                    let col_data = &SECOND_HALF[_to_usize(GLOBAL_DATA, 42)..];
                    let col_len = (col_data.len() % 65).min(num_rows);
                    let col = col_data[..col_len].to_vec();
                    toodee.insert_col(col_idx, col);
                }
                2 => {
                    let src_start_col = _to_usize(GLOBAL_DATA, 50);
                    let src_start_row = _to_usize(GLOBAL_DATA, 58);
                    let src_end_col = _to_usize(GLOBAL_DATA, 66);
                    let src_end_row = _to_usize(GLOBAL_DATA, 74);
                    let dest_col = _to_usize(GLOBAL_DATA, 82);
                    let dest_row = _to_usize(GLOBAL_DATA, 90);
                    toodee.copy_within(
                        ((src_start_col, src_start_row), (src_end_col, src_end_row)),
                        (dest_col, dest_row)
                    );
                }
                _ => {
                    let r1 = _to_usize(GLOBAL_DATA, 98) % num_rows;
                    let r2 = _to_usize(GLOBAL_DATA, 106) % num_rows;
                    toodee.swap_rows(r1, r2);
                }
            }
        }

        let src_len = num_cols * num_rows;
        let src_slice = &SECOND_HALF[_to_usize(GLOBAL_DATA, 114)..];
        let panic_trigger = _to_bool(GLOBAL_DATA, 122);
        toodee.sort_by_row(0, |a, b| {
            if panic_trigger { panic!("fuzzer induced panic") }
            a.cmp(b)
        });

        toodee.copy_from_slice(&src_slice[..src_len]);

        let view = toodee.view((0, 0), (num_cols, num_rows));
        for (idx, row) in view.rows().enumerate() {
            println!("Row {}: {:?}", idx, row);
            let cell = &row[_to_usize(GLOBAL_DATA, 123) % row.len()];
            let _ = *cell;
        }

        let mut view_mut = toodee.view_mut((0, 0), (num_cols, num_rows));
        for cell in view_mut.cells_mut() {
            *cell = _to_u8(SECOND_HALF, 131);
        }

        let col_idx = _to_usize(GLOBAL_DATA, 139) % num_cols;
        let col = toodee.col(col_idx);
        for (i, v) in col.enumerate() {
            println!("Col {}[{}] = {}", col_idx, i, v);
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