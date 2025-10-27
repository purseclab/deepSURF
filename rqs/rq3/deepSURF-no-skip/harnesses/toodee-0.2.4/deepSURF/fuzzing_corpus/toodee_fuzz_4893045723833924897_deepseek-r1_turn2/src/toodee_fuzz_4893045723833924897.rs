#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use toodee::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

#[derive(Debug, Default, Clone)]
struct CustomType0(String);

fn main (){
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 128 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        
        let rows = _to_usize(GLOBAL_DATA, 0) % 65;
        let cols = _to_usize(GLOBAL_DATA, 4) % 65;
        
        let mut t1 = match _to_u8(GLOBAL_DATA, 8) % 3 {
            0 => TooDee::<CustomType0>::new(cols, rows),
            1 => TooDee::<CustomType0>::with_capacity(_to_usize(GLOBAL_DATA, 12)),
            _ => TooDee::from_vec(cols, rows, (0..rows*cols)
                .map(|i| CustomType0(_to_str(GLOBAL_DATA, i%64, (i%64)+1).to_string()))
                .collect())
        };
        
        let op_count = _to_usize(GLOBAL_DATA, 16) % 5;
        for i in 0..op_count {
            let op_byte = _to_u8(GLOBAL_DATA, 20 + i);
            match op_byte % 7 {
                0 => {
                    let mut view = t1.view_mut((0,0), (rows, cols));
                    view.swap_rows(_to_usize(GLOBAL_DATA, 24 + i*4) % rows, _to_usize(GLOBAL_DATA, 28 + i*4) % rows);
                    let _ = TooDee::from(view);
                }
                1 => {
                    let col_idx = _to_usize(GLOBAL_DATA, 24 + i*4) % (t1.num_cols() + 1);
                    t1.insert_col(col_idx, (0..rows).map(|j| CustomType0(_to_str(GLOBAL_DATA, j%64, (j%64)+1).to_string())));
                }
                2 => {
                    let view = t1.view((0,0), (t1.num_rows(), t1.num_cols()));
                    let _ = TooDee::from(view).pop_col();
                }
                3 => {
                    let col = t1.col_mut(_to_usize(GLOBAL_DATA, 24 + i*4) % t1.num_cols());
                    for c in col {
                        println!("{:?}", c);
                    }
                }
                4 => {
                    t1.push_col((0..rows).map(|j| CustomType0(_to_str(GLOBAL_DATA, j%64, (j%64)+1).to_string())));
                }
                5 => {
                    t1.pop_col();
                }
                _ => {
                    let mut rows_iter = t1.rows_mut();
                    while let Some(row) = rows_iter.next() {
                        for elem in row {
                            println!("{:?}", elem);
                        }
                    }
                }
            }
        }
        
        let mut t2 = t1;
        t2.pop_col();
        
        let col_idx = _to_usize(GLOBAL_DATA, 50) % t2.num_cols().max(1);
        let mut drain = t2.remove_col(col_idx);
        while let Some(elem) = drain.next() {
            println!("{:?}", elem);
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