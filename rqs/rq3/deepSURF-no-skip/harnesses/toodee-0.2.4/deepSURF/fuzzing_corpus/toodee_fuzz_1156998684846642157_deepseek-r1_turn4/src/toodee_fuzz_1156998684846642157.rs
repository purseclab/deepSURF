#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use toodee::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

#[derive(Debug, Clone, Default)]
struct CustomType0(String);

fn main (){
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 4096 {return;}
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        
        let mut offset = 0;
        let rows = _to_usize(GLOBAL_DATA, offset) % 256;
        offset += 8;
        let cols = _to_usize(GLOBAL_DATA, offset) % 256;
        offset += 8;
        
        let vec_cap = _to_u8(GLOBAL_DATA, offset) % 65;
        offset += 1;
        let mut t_3 = Vec::with_capacity(vec_cap as usize);
        
        for _ in 0..vec_cap {
            if offset + 1 >= GLOBAL_DATA.len() { break; }
            let str_len = _to_u8(GLOBAL_DATA, offset) % 17;
            offset += 1;
            let s = _to_str(GLOBAL_DATA, offset, offset + str_len as usize);
            t_3.push(CustomType0(String::from(s)));
            offset += str_len as usize;
        }
        
        let construct_choice = _to_u8(GLOBAL_DATA, offset) % 3;
        offset += 1;
        
        let mut t_132 = match construct_choice {
            0 => TooDee::from_vec(rows, cols, t_3),
            1 => {
                let mut t = TooDee::with_capacity(rows.max(cols) % 256);
                for row in t_3.chunks(cols) {
                    t.push_row(row.to_vec());
                }
                t
            }
            2 => {
                let mut t = TooDee::new(rows % 256, cols % 256);
                for (i, item) in t_3.into_iter().enumerate() {
                    let coord = (i % cols, i / cols);
                    t[coord] = item;
                }
                t
            }
            _ => unreachable!()
        };

        let ops = _to_u8(GLOBAL_DATA, offset) % 65;
        offset += 1;
        
        for _ in 0..ops {
            let op_type = _to_u8(GLOBAL_DATA, offset) % 7;
            offset += 1;
            
            match op_type {
                0 => {
                    let mut view = t_132.view_mut((0,0), (rows/2, cols/2));
                    let mut rows_mut = view.rows_mut();
                    if let Some(row) = rows_mut.next_back() {
                        println!("{:?}", row);
                    }
                }
                1 => {
                    let col_idx = _to_usize(GLOBAL_DATA, offset) % cols;
                    offset += 8;
                    let mut col = t_132.col_mut(col_idx);
                    while let Some(item) = col.next() {
                        println!("{:?}", item);
                    }
                }
                2 => {
                    let mut rows_mut = t_132.rows_mut();
                    let nth = _to_usize(GLOBAL_DATA, offset) % rows;
                    offset += 8;
                    if let Some(row) = rows_mut.nth_back(nth) {
                        println!("{:?}", row);
                    }
                }
                3 => {
                    let row1 = _to_usize(GLOBAL_DATA, offset) % rows;
                    offset += 8;
                    let row2 = _to_usize(GLOBAL_DATA, offset) % rows;
                    offset += 8;
                    t_132.swap_rows(row1, row2);
                }
                4 => {
                    let col_idx = _to_usize(GLOBAL_DATA, offset) % cols;
                    offset += 8;
                    let mut drain = t_132.remove_col(col_idx);
                    while let Some(item) = drain.next() {
                        println!("{:?}", item);
                    }
                }
                5 => {
                    let view = t_132.view((0,0), (rows, cols));
                    let mut cloned = TooDee::from(view);
                    cloned.swap_rows(0, cloned.num_rows().saturating_sub(1));
                }
                6 => {
                    let mut col = _to_usize(GLOBAL_DATA, offset) % cols;
                    offset += 8;
                    let view = t_132.view_mut((0, col), (rows, col + 1));
                    let _ = &view.rows().count();
                }
                _ => unreachable!()
            }
        }

        let mut view_mut = t_132.view_mut((0,0), (rows, cols));
        let mut new_toodee = TooDee::from(view_mut);
        let mut rows_mut = new_toodee.rows_mut();
        while let Some(row) = rows_mut.next_back() {
            println!("{:?}", row);
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