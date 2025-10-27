#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use toodee::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 4096 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let ctor_selector = _to_u8(GLOBAL_DATA, 0) % 3;
        let mut toodee = match ctor_selector {
            0 => {
                let cols = _to_usize(GLOBAL_DATA, 8) % 65;
                let rows = _to_usize(GLOBAL_DATA, 16) % 65;
                TooDee::new(cols, rows)
            }
            1 => {
                let cap = _to_usize(GLOBAL_DATA, 8);
                TooDee::with_capacity(cap)
            }
            _ => {
                let cols = _to_usize(GLOBAL_DATA, 8) % 65;
                let rows = _to_usize(GLOBAL_DATA, 16) % 65;
                let vec_data: Vec<String> = (0..cols*rows).map(|i| {
                    _to_str(GLOBAL_DATA, 24 + i*4, 24 + i*4 + 4).to_string()
                }).collect();
                TooDee::from_vec(cols, rows, vec_data)
            }
        };

        let op_count = _to_usize(GLOBAL_DATA, 128) % 10;
        let mut data_ptr = 256;

        for _ in 0..op_count {
            let op_byte = _to_u8(GLOBAL_DATA, data_ptr) % 6;
            data_ptr += 1;

            match op_byte {
                0 => {
                    let view = toodee.view(
                        (_to_usize(GLOBAL_DATA, data_ptr), _to_usize(GLOBAL_DATA, data_ptr + 8)),
                        (_to_usize(GLOBAL_DATA, data_ptr + 16), _to_usize(GLOBAL_DATA, data_ptr + 24))
                    );
                    data_ptr += 32;
                    let col = view.col(_to_usize(GLOBAL_DATA, data_ptr));
                    col.last();
                    data_ptr += 8;
                }
                1 => {
                    if toodee.num_cols() > 0 {
                        let col_idx = _to_usize(GLOBAL_DATA, data_ptr) % toodee.num_cols();
                        let col = toodee.col_mut(col_idx);
                        println!("{:?}", col.last());
                    }
                    data_ptr += 8;
                }
                2 => {
                    let start = (_to_usize(GLOBAL_DATA, data_ptr), _to_usize(GLOBAL_DATA, data_ptr + 8));
                    let end = (_to_usize(GLOBAL_DATA, data_ptr + 16), _to_usize(GLOBAL_DATA, data_ptr + 24));
                    let mut view = toodee.view_mut(start, end);
                    view.swap_rows(
                        _to_usize(GLOBAL_DATA, data_ptr + 32) % view.num_rows(),
                        _to_usize(GLOBAL_DATA, data_ptr + 40) % view.num_rows()
                    );
                    data_ptr += 48;
                }
                3 => {
                    let pop = toodee.pop_col();
                    if let Some(mut drain) = pop {
                        while let Some(_) = drain.next() {}
                    }
                }
                4 => {
                    let col_idx = _to_usize(GLOBAL_DATA, data_ptr);
                    let row_idx = _to_usize(GLOBAL_DATA, data_ptr + 8);
                    if row_idx < toodee.num_rows() {
                        let row = &toodee[row_idx];
                        let _ = row.get(col_idx);
                    }
                    data_ptr += 16;
                }
                _ => {
                    let view = toodee.view((0, 0), (toodee.num_cols(), toodee.num_rows()));
                    let reversed_cols = (0..view.num_cols()).rev();
                    for c in reversed_cols {
                        let col = view.col(c);
                        col.last();
                    }
                }
            }
        }

        let final_col = _to_usize(GLOBAL_DATA, data_ptr) % (toodee.num_cols() + 1);
        let _ = toodee.col(final_col).last();
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