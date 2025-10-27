#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use toodee::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

#[derive(Debug, Clone, Default)]
struct CustomType0(String);

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 2048 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let num_ops = _to_u8(GLOBAL_DATA, 0) % 10;
        let mut pos = 1;

        for _ in 0..num_ops {
            let primary = _to_usize(GLOBAL_DATA, pos);
            pos += 8;
            let secondary = _to_usize(GLOBAL_DATA, pos);
            pos += 8;

            let constructor_choice = _to_u8(GLOBAL_DATA, pos) % 4;
            pos += 1;

            let mut td = match constructor_choice {
                0 => {
                    let cols = primary;
                    let rows = secondary;
                    let mut vec = Vec::with_capacity(cols * rows);
                    for _ in 0..cols * rows {
                        let str_len = _to_u8(GLOBAL_DATA, pos) % 17;
                        pos += 1;
                        let s = _to_str(GLOBAL_DATA, pos, pos + str_len as usize);
                        pos += str_len as usize;
                        vec.push(CustomType0(String::from(s)));
                    }
                    TooDee::from_vec(cols, rows, vec)
                }
                1 => TooDee::with_capacity(primary),
                2 => {
                    let cols = primary;
                    let rows = secondary;
                    TooDee::new(cols, rows)
                }
                _ => {
                    let cols = primary;
                    let rows = secondary;
                    let mut vec = Vec::with_capacity(cols * rows);
                    for _ in 0..cols * rows {
                        let str_len = _to_u8(GLOBAL_DATA, pos) % 17;
                        pos += 1;
                        let s = _to_str(GLOBAL_DATA, pos, pos + str_len as usize);
                        pos += str_len as usize;
                        vec.push(CustomType0(String::from(s)));
                    }
                    TooDee::from_box(cols, rows, vec.into_boxed_slice())
                }
            };

            let mut view_mut = td.view_mut((0, 0), (primary, secondary));
            let row_count = view_mut.num_rows();
            let col_idx = primary % view_mut.num_cols();

            let mut rows = view_mut.rows_mut();
            let _ = rows.nth(_to_usize(GLOBAL_DATA, pos) % row_count);
            pos += 8;

            let mut col = view_mut.col_mut(col_idx);
            println!("{:?}", col.nth(0));

            let view: TooDeeView<_> = view_mut.into();

            let sub_view = view.view((_to_usize(GLOBAL_DATA, pos), _to_usize(GLOBAL_DATA, pos + 8)), 
                                   (_to_usize(GLOBAL_DATA, pos + 16), _to_usize(GLOBAL_DATA, pos + 24)));
            pos += 32;

            println!("{:?}", sub_view.rows().last());

            let col_sub_idx = _to_usize(GLOBAL_DATA, pos) % sub_view.num_cols();
            let mut col_sub = sub_view.col(col_sub_idx);
            println!("{:?}", col_sub.next());
            pos += 8;

            let mut rev_rows = sub_view.rows().rev();
            let _ = rev_rows.nth(_to_usize(GLOBAL_DATA, pos) % sub_view.num_rows());
            pos += 8;

            td.pop_col();
            td.remove_row(_to_usize(GLOBAL_DATA, pos) % td.num_rows());
            pos += 8;

            let insert_col_idx = _to_usize(GLOBAL_DATA, pos) % (td.num_cols() + 1);
            let _ = td.insert_col(insert_col_idx, vec![CustomType0("fuzz".into())]);
            pos += 8;
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