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
        if data.len() < 128 {
            return;
        }
        set_global_data(data);
        let global_data = get_global_data();
        let gd = global_data.first_half;
        let gd2 = global_data.second_half;
        let cols = (_to_usize(gd, 1) % 65) + 1;
        let rows = (_to_usize(gd, 9) % 65) + 1;
        let ctor_choice = _to_u8(gd, 17) % 5;
        let init_val = CustomType0::default();
        let total = cols * rows;
        let mut td: TooDee<CustomType0> = match ctor_choice {
            0 => TooDee::new(cols, rows),
            1 => TooDee::with_capacity(_to_usize(gd, 25)),
            2 => TooDee::init(cols, rows, init_val.clone()),
            3 => {
                let v = vec![init_val.clone(); total];
                TooDee::from_vec(cols, rows, v)
            }
            _ => {
                let b: Box<[CustomType0]> = vec![init_val.clone(); total].into_boxed_slice();
                TooDee::from_box(cols, rows, b)
            }
        };
        let op_count = _to_u8(gd, 33) % 20;
        for i in 0..op_count {
            let opcode = _to_u8(gd, 34 + i as usize);
            match opcode % 9 {
                0 => {
                    td.capacity();
                }
                1 => {
                    let mut rows_iter = td.rows();
                    rows_iter.nth(_to_usize(gd2, i as usize * 8));
                }
                2 => {
                    let mut rows_mut_iter = td.rows_mut();
                    rows_mut_iter.next();
                }
                3 => {
                    td.swap_rows(_to_usize(gd, 40), _to_usize(gd, 48));
                }
                4 => {
                    let col_index = _to_usize(gd2, i as usize * 8 + 4);
                    let mut col_iter = td.col(col_index);
                    let nth_idx = _to_usize(gd, 56);
                    if let Some(val) = col_iter.nth(nth_idx) {
                        println!("{:?}", val);
                    }
                    col_iter.last();
                }
                5 => {
                    if let Some(mut drain) = td.pop_col() {
                        drain.next();
                        drain.next_back();
                    }
                }
                6 => {
                    let view = td.view(
                        (_to_usize(gd, 64), _to_usize(gd, 72)),
                        (_to_usize(gd, 80), _to_usize(gd, 88)),
                    );
                    let _ = view.rows().last();
                }
                7 => {
                    let mut view_mut = td.view_mut(
                        (_to_usize(gd, 96), _to_usize(gd, 104)),
                        (_to_usize(gd, 112), _to_usize(gd, 120)),
                    );
                    let mut col_mut_iter = view_mut.col_mut(_to_usize(gd2, i as usize));
                    col_mut_iter.nth_back(_to_usize(gd, 124));
                }
                _ => {
                    let coord = (_to_usize(gd, 40), _to_usize(gd, 48));
                    let cell = &td[coord];
                    println!("{:?}", cell);
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