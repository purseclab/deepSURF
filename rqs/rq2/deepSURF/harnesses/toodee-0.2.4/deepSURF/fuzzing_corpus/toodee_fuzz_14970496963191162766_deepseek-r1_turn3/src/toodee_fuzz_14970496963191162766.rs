#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use toodee::*;
use global_data::*;
use std::ops::{Index, IndexMut};

#[derive(Debug, Clone, Default)]
struct CustomType0(String);

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 256 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let constructor = _to_u8(GLOBAL_DATA, 0) % 4;
        let (rows, cols) = match constructor {
            0 => (_to_usize(GLOBAL_DATA, 8) % 65, _to_usize(GLOBAL_DATA, 16) % 65),
            1 => (_to_usize(GLOBAL_DATA, 8) % 65, _to_usize(GLOBAL_DATA, 16)),
            2 => (_to_usize(GLOBAL_DATA, 8), _to_usize(GLOBAL_DATA, 16)),
            _ => (_to_usize(GLOBAL_DATA, 8) % 32 + 1, _to_usize(GLOBAL_DATA, 16) % 32 + 1),
        };

        let mut toodee = match constructor {
            0 => TooDee::new(rows, cols),
            1 => TooDee::with_capacity(_to_usize(GLOBAL_DATA, 24)),
            2 => {
                let vec = vec![CustomType0(String::new()); rows * cols];
                TooDee::from_vec(rows, cols, vec)
            },
            3 => TooDee::init(rows, cols, CustomType0(String::from("Fuzzed"))),
            _ => unreachable!(),
        };

        let ops_count = _to_u8(GLOBAL_DATA, 32) % 10;
        let mut offset = 40;

        for _ in 0..ops_count {
            let op = _to_u8(GLOBAL_DATA, offset) % 7;
            offset += 1;

            match op {
                0 => {
                    let view = toodee.view((_to_usize(GLOBAL_DATA, offset), _to_usize(GLOBAL_DATA, offset + 8)),
                                          (_to_usize(GLOBAL_DATA, offset + 16), _to_usize(GLOBAL_DATA, offset + 24)));
                    let mut col = view.col(_to_usize(GLOBAL_DATA, offset + 32));
                    offset += 40;
                    let _unused = col.nth_back(_to_usize(GLOBAL_DATA, offset));
                    offset += 8;
                },
                1 => {
                    let mut view_mut = toodee.view_mut((_to_usize(GLOBAL_DATA, offset), _to_usize(GLOBAL_DATA, offset + 8)),
                                                     (_to_usize(GLOBAL_DATA, offset + 16), _to_usize(GLOBAL_DATA, offset + 24)));
                    view_mut.swap_rows(_to_usize(GLOBAL_DATA, offset + 32), _to_usize(GLOBAL_DATA, offset + 40));
                    offset += 48;
                },
                2 => {
                    let mut rows_mut = toodee.rows_mut();
                    let _ = rows_mut.nth_back(_to_usize(GLOBAL_DATA, offset));
                    offset += 8;
                },
                3 => {
                    let mut col_mut = toodee.col_mut(_to_usize(GLOBAL_DATA, offset));
                    offset += 8;
                    if let Some(item) = col_mut.nth_back(_to_usize(GLOBAL_DATA, offset)) {
                        println!("{:?}", item);
                    }
                    offset += 8;
                },
                4 => {
                    let mut rows = toodee.rows();
                    let _ = rows.nth(_to_usize(GLOBAL_DATA, offset));
                    offset += 8;
                },
                5 => {
                    let view = toodee.view((0, 0), (toodee.num_cols(), toodee.num_rows()));
                    let _cells = view.cells();
                },
                6 => {
                    if let Some(mut drain) = toodee.pop_col() {
                        let _ = drain.next_back();
                    }
                },
                _ => ()
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