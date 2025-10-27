#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use toodee::*;
use global_data::*;
use std::ops::{Deref, DerefMut, Index, IndexMut};

#[derive(Clone, Debug, Default)]
struct CustomType0(String);

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 256 {
            return;
        }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let mut cursor: usize = 0;

        let cols = _to_usize(GLOBAL_DATA, cursor);
        cursor += 8;
        let rows = _to_usize(GLOBAL_DATA, cursor);
        cursor += 8;

        let vec_cap = (_to_u8(GLOBAL_DATA, cursor) % 65) as usize + 1;
        cursor += 1;

        let mut base_vec: Vec<CustomType0> = Vec::with_capacity(vec_cap);
        for _ in 0..vec_cap {
            if cursor >= GLOBAL_DATA.len() {
                break;
            }
            let slen = (_to_u8(GLOBAL_DATA, cursor) % 17) as usize;
            cursor += 1;
            if cursor + slen >= GLOBAL_DATA.len() {
                break;
            }
            let s = _to_str(GLOBAL_DATA, cursor, cursor + slen);
            cursor += slen;
            base_vec.push(CustomType0(String::from(s)));
        }

        if base_vec.is_empty() {
            base_vec.push(CustomType0(String::new()));
        }

        if cursor >= GLOBAL_DATA.len() {
            return;
        }
        let ctor_choice = _to_u8(GLOBAL_DATA, cursor);
        cursor += 1;

        let mut td: TooDee<CustomType0> = match ctor_choice % 3 {
            0 => TooDee::new(cols, rows),
            1 => TooDee::from_vec(cols, rows, base_vec.clone()),
            _ => TooDee::init(cols, rows, base_vec[0].clone()),
        };

        if cursor >= GLOBAL_DATA.len() {
            return;
        }
        let ops = (_to_u8(GLOBAL_DATA, cursor) % 16) as usize;
        cursor += 1;

        for _ in 0..ops {
            if cursor >= GLOBAL_DATA.len() {
                break;
            }
            let op_code = _to_u8(GLOBAL_DATA, cursor);
            cursor += 1;
            match op_code % 8 {
                0 => {
                    if cursor + 16 >= GLOBAL_DATA.len() {
                        break;
                    }
                    let r1 = _to_usize(GLOBAL_DATA, cursor);
                    cursor += 8;
                    let r2 = _to_usize(GLOBAL_DATA, cursor);
                    cursor += 8;
                    td.swap_rows(r1, r2);
                }
                1 => {
                    if cursor + 8 >= GLOBAL_DATA.len() {
                        break;
                    }
                    let c = _to_usize(GLOBAL_DATA, cursor);
                    cursor += 8;
                    let mut col_iter = td.col_mut(c);
                    col_iter.next_back();
                }
                2 => {
                    if cursor + 32 >= GLOBAL_DATA.len() {
                        break;
                    }
                    let s = (
                        _to_usize(GLOBAL_DATA, cursor),
                        _to_usize(GLOBAL_DATA, cursor + 8),
                    );
                    let e = (
                        _to_usize(GLOBAL_DATA, cursor + 16),
                        _to_usize(GLOBAL_DATA, cursor + 24),
                    );
                    cursor += 32;
                    let mut vmut = td.view_mut(s, e);
                    let mut rows_it = vmut.rows_mut();
                    if let Some(r) = rows_it.next_back() {
                        println!("{:?}", r.len());
                    }
                }
                3 => {
                    td.pop_col();
                }
                4 => {
                    td.push_col(base_vec.clone());
                }
                5 => {
                    if cursor + 16 >= GLOBAL_DATA.len() {
                        break;
                    }
                    let r1 = _to_usize(GLOBAL_DATA, cursor);
                    cursor += 8;
                    let r2 = _to_usize(GLOBAL_DATA, cursor);
                    cursor += 8;
                    let mut vmut = td.view_mut((0, 0), (cols, rows));
                    vmut.swap_rows(r1, r2);
                }
                6 => {
                    if cursor + 8 >= GLOBAL_DATA.len() {
                        break;
                    }
                    let n = _to_usize(GLOBAL_DATA, cursor);
                    cursor += 8;
                    let mut rows_it = td.rows_mut();
                    rows_it.nth_back(n);
                }
                _ => {}
            }
        }

        let mut target_iter = td.rows_mut();
        if let Some(row) = target_iter.next_back() {
            println!("{:?}", row.deref().len());
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