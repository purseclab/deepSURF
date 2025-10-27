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
        if data.len() < 128 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let constructor_choice = _to_u8(GLOBAL_DATA, 0) % 3;
        let mut too_dee = match constructor_choice {
            0 => TooDee::with_capacity(_to_usize(GLOBAL_DATA, 8)),
            1 => {
                let rows = _to_usize(GLOBAL_DATA, 8);
                let cols = _to_usize(GLOBAL_DATA, 16);
                let mut t = TooDee::new(rows, cols);
                let data_str = _to_str(GLOBAL_DATA, 32, 64);
                t.push_col(vec![CustomType0(data_str.to_string()); cols].into_iter());
                t
            }
            _ => {
                let vec_data: Vec<CustomType0> = (0.._to_usize(GLOBAL_DATA, 8))
                    .map(|i| CustomType0(_to_str(GLOBAL_DATA, i*8 % 64, (i*8 + 8) % 64).to_string()))
                    .collect();
                TooDee::from_vec(_to_usize(GLOBAL_DATA, 16), _to_usize(GLOBAL_DATA, 24), vec_data)
            }
        };

        for i in 0..5 {
            let op_seed = _to_u8(GLOBAL_DATA, 32 + i);
            match op_seed % 6 {
                0 => {
                    let view = too_dee.view((0, 0), (too_dee.num_cols(), too_dee.num_rows()));
                    println!("View cols: {:?}", view.num_cols());
                }
                1 => {
                    let mut view_mut = too_dee.view_mut((0, 0), (too_dee.num_cols(), too_dee.num_rows()));
                    view_mut.swap_rows(
                        _to_usize(GLOBAL_DATA, 40) % view_mut.num_rows(),
                        _to_usize(GLOBAL_DATA, 48) % view_mut.num_rows(),
                    );
                }
                2 => {
                    let col_idx = _to_usize(GLOBAL_DATA, 56) % too_dee.num_cols();
                    let mut col = too_dee.col_mut(col_idx);
                    let _ = col.nth(_to_usize(GLOBAL_DATA, 64) % col.size_hint().0);
                }
                3 => {
                    too_dee.push_col(
                        vec![
                            CustomType0(_to_str(GLOBAL_DATA, 72, 80).to_string()),
                            CustomType0(_to_str(GLOBAL_DATA, 80, 88).to_string())
                        ].into_iter()
                    );
                }
                4 => {
                    let removed = too_dee.pop_col();
                    if let Some(mut drain) = removed {
                        drain.next();
                        drain.next_back();
                    }
                }
                _ => {
                    let mut rows = too_dee.rows_mut();
                    let _ = rows.nth_back(_to_usize(GLOBAL_DATA, 88) % rows.size_hint().0);
                }
            }
        }

        let col_remove_idx = _to_usize(GLOBAL_DATA, 96) % (too_dee.num_cols() + 1);
        let mut drain_col = too_dee.remove_col(col_remove_idx);
        drain_col.next_back();
        drain_col.next_back();
        println!("{:?}", drain_col.next().unwrap());
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