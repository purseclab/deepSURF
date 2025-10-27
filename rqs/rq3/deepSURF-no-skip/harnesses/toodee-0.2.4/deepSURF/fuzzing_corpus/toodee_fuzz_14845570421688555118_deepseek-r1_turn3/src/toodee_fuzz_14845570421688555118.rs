#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use toodee::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

#[derive(Debug)]
struct CustomType0(String);

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 8192 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        
        let num_cols = _to_usize(GLOBAL_DATA, 0);
        let num_rows = _to_usize(GLOBAL_DATA, 8);
        let capacity = _to_u8(GLOBAL_DATA, 16) % 65;
        let mut vec_data = Vec::with_capacity(capacity as usize);
        
        let mut data_idx = 17;
        while data_idx + 18 < GLOBAL_DATA.len() && vec_data.len() < capacity as usize {
            let str_len = _to_u8(GLOBAL_DATA, data_idx) % 17;
            data_idx += 1;
            let s = _to_str(GLOBAL_DATA, data_idx, data_idx + str_len as usize);
            vec_data.push(CustomType0(s.to_string()));
            data_idx += str_len as usize;
        }

        let mut toodee = TooDee::from_vec(num_cols, num_rows, vec_data);
        let mut op_idx = data_idx;
        let op_count = _to_u8(GLOBAL_DATA, op_idx) % 16;
        op_idx += 1;

        for _ in 0..op_count {
            if op_idx + 8 >= GLOBAL_DATA.len() { break; }
            
            let op_type = _to_u8(GLOBAL_DATA, op_idx) % 6;
            op_idx += 1;

            match op_type {
                0 => {
                    let col = _to_usize(GLOBAL_DATA, op_idx);
                    op_idx += 8;
                    let mut col_iter = toodee.col_mut(col);
                    col_iter.next_back();
                }
                1 => {
                    let r1 = _to_usize(GLOBAL_DATA, op_idx);
                    op_idx += 8;
                    let r2 = _to_usize(GLOBAL_DATA, op_idx);
                    op_idx += 8;
                    toodee.swap_rows(r1, r2);
                }
                2 => {
                    let view_col = _to_usize(GLOBAL_DATA, op_idx);
                    op_idx += 8;
                    let view = toodee.view((0, 0), (toodee.num_cols(), toodee.num_rows()));
                    if let Some(v) = view.rows().last() {
                        println!("{:?}", v);
                    }
                }
                3 => {
                    let idx = _to_usize(GLOBAL_DATA, op_idx);
                    op_idx += 8;
                    let num_cols = toodee.num_cols();
                    let num_rows = toodee.num_rows();
                    let mut view_mut = toodee.view_mut((0, 0), (num_cols, num_rows));
                    let mut rows_mut = view_mut.rows_mut();
                    if let Some(row) = rows_mut.nth_back(idx % num_rows) {
                        println!("{:?}", row);
                    }
                }
                4 => {
                    let col = _to_usize(GLOBAL_DATA, op_idx);
                    op_idx += 8;
                    let mut column = toodee.col_mut(col);
                    let _ = (0..column.len()).map(|_| column.next_back());
                }
                5 => {
                    let start = (_to_usize(GLOBAL_DATA, op_idx), _to_usize(GLOBAL_DATA, op_idx + 8));
                    op_idx += 16;
                    let end = (_to_usize(GLOBAL_DATA, op_idx), _to_usize(GLOBAL_DATA, op_idx + 8));
                    op_idx += 16;
                    let mut nested_view = toodee.view_mut(start, end);
                    let rev_iter = nested_view.rows_mut().rev();
                    for r in rev_iter {
                        println!("{:?}", r);
                    }
                }
                _ => {}
            }
        }

        let target_col = _to_usize(GLOBAL_DATA, op_idx);
        let mut final_col = toodee.col_mut(target_col);
        while let Some(item) = final_col.next_back() {
            println!("{:?}", item.0);
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