#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use toodee::*;
use global_data::*;
use std::fmt::Debug;

#[derive(Debug, Default, Clone)]
struct CustomType0(String);

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 2048 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let op_count = _to_usize(GLOBAL_DATA, 0) % 10;
        for i in 0..op_count {
            let offset = i * 8;
            let op_choice = _to_u8(GLOBAL_DATA, offset) % 6;
            let cap = _to_usize(GLOBAL_DATA, offset + 8);
            let cols = _to_usize(GLOBAL_DATA, offset + 16) % 65;
            let rows = _to_usize(GLOBAL_DATA, offset + 24) % 65;

            match op_choice {
                0 => {
                    let t = TooDee::<CustomType0>::with_capacity(cap % 65);
                    let view = t.view((0, 0), (cols, rows));
                    let mut t_new = TooDee::from(view);
                    let mut view_mut = t_new.view_mut((0, 0), (cols.min(t_new.num_cols()), rows.min(t_new.num_rows())));
                    let swap1 = _to_usize(GLOBAL_DATA, offset + 32) % view_mut.num_rows();
                    let swap2 = _to_usize(GLOBAL_DATA, offset + 40) % view_mut.num_rows();
                    view_mut.swap_rows(swap1, swap2);
                }
                1 => {
                    let mut t = TooDee::<CustomType0>::new(cols, rows);
                    let pop_col = t.pop_col();
                    let _ = pop_col.map(|mut c| c.next());
                }
                2 => {
                    let t = TooDee::<CustomType0>::with_capacity(cap % 65);
                    let row_idx = _to_usize(GLOBAL_DATA, offset + 32) % t.num_rows();
                    println!("{:?}", &t[row_idx]);
                }
                3 => {
                    let mut t = TooDee::<CustomType0>::new(cols, rows);
                    let col_idx = _to_usize(GLOBAL_DATA, offset + 32) % t.num_cols();
                    let mut col = t.col_mut(col_idx);
                    let nth_idx = _to_usize(GLOBAL_DATA, offset + 40);
                    col.nth(nth_idx);
                }
                4 => {
                    let t = TooDee::<CustomType0>::with_capacity(cap % 65);
                    let mut rows_iter = t.rows();
                    let nth_idx = _to_usize(GLOBAL_DATA, offset + 32);
                    rows_iter.nth_back(nth_idx);
                }
                _ => {
                    let mut t = TooDee::<CustomType0>::new(cols, rows);
                    let insert_col_idx = _to_usize(GLOBAL_DATA, offset + 32) % (t.num_cols() + 1);
                    t.insert_col(insert_col_idx, vec![CustomType0(String::new()); rows]);
                }
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