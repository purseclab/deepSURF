#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use toodee::*;
use global_data::*;
use std::ops::{Index, IndexMut, Deref, DerefMut};

#[derive(Clone, Debug)]
struct CustomType0(String);

impl Default for CustomType0 {
    fn default() -> Self {
        CustomType0(String::new())
    }
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 1500 {
            return;
        }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let num_cols = (_to_usize(GLOBAL_DATA, 0) % 65).max(1);
        let num_rows = (_to_usize(GLOBAL_DATA, 8) % 65).max(1);
        let total_cells = num_cols * num_rows;

        let mut cell_vec: Vec<CustomType0> = Vec::with_capacity(total_cells);
        for i in 0..total_cells {
            let idx = 32 + i;
            if idx < GLOBAL_DATA.len() {
                let b = _to_u8(GLOBAL_DATA, idx);
                cell_vec.push(CustomType0(format!("{}", b)));
            } else {
                cell_vec.push(CustomType0(String::new()));
            }
        }

        let selector = _to_u8(GLOBAL_DATA, 16) % 4;
        let mut toodee_val: TooDee<CustomType0> = match selector {
            0 => TooDee::new(num_cols, num_rows),
            1 => TooDee::init(num_cols, num_rows, CustomType0("init".to_string())),
            2 => TooDee::from_box(num_cols, num_rows, cell_vec.clone().into_boxed_slice()),
            _ => TooDee::from_vec(num_cols, num_rows, cell_vec.clone()),
        };

        let op_cnt = _to_u8(GLOBAL_DATA, 24) % 10;
        for i in 0..op_cnt {
            let op_sel = _to_u8(GLOBAL_DATA, 25 + i as usize) % 6;
            match op_sel {
                0 => {
                    let mut r_iter = toodee_val.rows();
                    r_iter.next();
                    r_iter.last();
                }
                1 => {
                    let col_idx = _to_usize(GLOBAL_DATA, 40 + i as usize);
                    let mut c_iter = toodee_val.col(col_idx);
                    c_iter.next();
                    c_iter.last();
                }
                2 => {
                    if toodee_val.num_cols() > 0 {
                        let new_row: Vec<CustomType0> =
                            (0..toodee_val.num_cols()).map(|_| CustomType0("r".to_string())).collect();
                        toodee_val.push_row(new_row);
                    }
                }
                3 => {
                    if toodee_val.num_rows() > 0 {
                        let new_col: Vec<CustomType0> =
                            (0..toodee_val.num_rows()).map(|_| CustomType0("c".to_string())).collect();
                        toodee_val.push_col(new_col);
                    }
                }
                4 => {
                    if toodee_val.num_rows() > 1 {
                        let r1 = _to_usize(GLOBAL_DATA, 60 + i as usize) % toodee_val.num_rows();
                        let r2 = _to_usize(GLOBAL_DATA, 68 + i as usize) % toodee_val.num_rows();
                        toodee_val.swap_rows(r1, r2);
                    }
                }
                _ => {}
            }
        }

        let start_col = _to_usize(GLOBAL_DATA, 100);
        let start_row = _to_usize(GLOBAL_DATA, 108);
        let end_col = _to_usize(GLOBAL_DATA, 116);
        let end_row = _to_usize(GLOBAL_DATA, 124);

        let mut view_mut_val = TooDee::view_mut(
            &mut toodee_val,
            (start_col, start_row),
            (end_col, end_row),
        );

        {
            let view_ref = &view_mut_val;
            let row_idx = _to_usize(GLOBAL_DATA, 132);
            let row_slice = view_ref.index(row_idx);
            println!("{:?}", row_slice);
            let col_idx_after = _to_usize(GLOBAL_DATA, 140);
            let _ = view_ref.col(col_idx_after).last();
        }

        let mut rows_iter_after = view_mut_val.rows_mut();
        let _ = rows_iter_after.next();
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