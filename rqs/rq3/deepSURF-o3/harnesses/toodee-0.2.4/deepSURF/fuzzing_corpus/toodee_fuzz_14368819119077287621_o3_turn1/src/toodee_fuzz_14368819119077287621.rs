#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use toodee::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};
use std::iter::repeat;

#[derive(Clone, Debug)]
struct CustomType0(String);
impl Default for CustomType0 {
    fn default() -> Self {
        CustomType0(String::new())
    }
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 64 {
            return;
        }
        set_global_data(data);
        let global_data = get_global_data();
        let g = global_data.first_half;
        let h = global_data.second_half;

        let selector = _to_u8(g, 0) % 4;
        let cols = (_to_usize(g, 8) % 65).max(1);
        let rows = (_to_usize(g, 16) % 65).max(1);
        let mut toodee: TooDee<CustomType0> = match selector {
            0 => TooDee::new(cols, rows),
            1 => {
                let init_val = CustomType0(String::from_utf8_lossy(h).to_string());
                TooDee::init(cols, rows, init_val)
            }
            2 => TooDee::with_capacity(_to_usize(g, 24) % 65),
            _ => {
                let len = cols * rows;
                let v: Vec<CustomType0> = repeat(CustomType0::default()).take(len).collect();
                TooDee::from_vec(cols, rows, v)
            }
        };

        let op_count = (_to_u8(g, 32) % 10) as usize;
        for i in 0..op_count {
            let op_code = _to_u8(g, 33 + i) % 8;
            match op_code {
                0 => {
                    let mut rows_iter = toodee.rows();
                    let idx = _to_usize(g, 40 + i * 8);
                    let res = rows_iter.nth_back(idx);
                    if let Some(slice) = res {
                        println!("{:?}", slice.deref());
                    }
                }
                1 => {
                    let mut rows_iter = toodee.rows();
                    let idx = _to_usize(g, 40 + i * 8);
                    let _ = rows_iter.nth(idx);
                }
                2 => {
                    let mut rows_iter = toodee.rows();
                    let _ = rows_iter.next();
                }
                3 => {
                    let mut rows_iter = toodee.rows();
                    let _ = rows_iter.last();
                }
                4 => {
                    let mut rows_iter = toodee.rows();
                    let _ = rows_iter.next_back();
                }
                5 => {
                    let start = (_to_usize(g, 40 + i * 8) % cols, _to_usize(h, 0) % rows);
                    let end = (cols.saturating_sub(1), rows.saturating_sub(1));
                    let view = toodee.view(start, end);
                    let mut rows_iter = view.rows();
                    let idx = _to_usize(h, 8);
                    let _ = rows_iter.nth_back(idx);
                }
                6 => {
                    let col_idx = _to_usize(g, 40 + i * 8) % cols;
                    let mut col_iter = toodee.col(col_idx);
                    let idx = _to_usize(h, 16);
                    let _ = col_iter.nth_back(idx);
                }
                _ => {
                    let r1 = _to_usize(g, 40 + i * 8) % rows;
                    let r2 = _to_usize(h, 24) % rows;
                    toodee.swap_rows(r1, r2);
                }
            }
        }

        let mut final_rows = toodee.rows();
        let final_idx = _to_usize(g, 60);
        let _ = final_rows.nth_back(final_idx);
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