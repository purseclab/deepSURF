#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use toodee::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

#[derive(Clone, Default, Debug)]
struct CustomType0(String);

impl From<&str> for CustomType0 {
    fn from(s: &str) -> Self {
        CustomType0(s.to_owned())
    }
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 2048 {
            return;
        }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let vec_len = (_to_u8(GLOBAL_DATA, 0) % 65 + 1) as usize;
        let mut elements = Vec::with_capacity(vec_len);
        let avail = GLOBAL_DATA.len();
        for i in 0..vec_len {
            let start = (1 + i * 3) % (avail - 1);
            let len_byte = _to_u8(GLOBAL_DATA, start % avail) % 17 + 1;
            let end = (start + len_byte as usize).min(avail);
            let s = _to_str(GLOBAL_DATA, start, end);
            elements.push(CustomType0::from(s));
        }

        let total = elements.len();
        if total == 0 {
            return;
        }
        let num_cols = (_to_u8(GLOBAL_DATA, 37) as usize % total) + 1;
        let num_rows = total / num_cols;
        let size = num_cols * num_rows;
        if size == 0 {
            return;
        }
        elements.truncate(size);

        let view_slice = &elements[..];
        let tview0 = TooDeeView::new(num_cols, num_rows, view_slice);

        let tdee0 = TooDee::from_vec(num_cols, num_rows, elements.clone());
        let tview1 = tdee0.view((0, 0), (num_cols - 1, num_rows - 1));

        let mut tdee1 = TooDee::new(num_cols, num_rows);
        tdee1.fill(CustomType0::default());
        let mut tviewmut0 = tdee1.view_mut((0, 0), (num_cols - 1, num_rows - 1));

        let op_count = (_to_u8(GLOBAL_DATA, 53) % 20) as usize;
        for i in 0..=op_count {
            let selector =
                _to_u8(GLOBAL_DATA, (54 + i) % GLOBAL_DATA.len()) % 8;
            match selector {
                0 => {
                    let off = (200 + i * 8) % (GLOBAL_DATA.len() - 8);
                    let idx = _to_usize(GLOBAL_DATA, off) % num_rows;
                    let row = tview0.index(idx);
                    println!("{:?}", row.len());
                }
                1 => {
                    let off = (400 + i * 8) % (GLOBAL_DATA.len() - 8);
                    let idx = _to_usize(GLOBAL_DATA, off) % num_rows;
                    let row = tview1.index(idx);
                    if !row.is_empty() {
                        println!("{:?}", &row[0]);
                    }
                }
                2 => {
                    let mut r_iter = tview0.rows();
                    let _ = r_iter.next();
                }
                3 => {
                    let off = (600 + i * 8) % (GLOBAL_DATA.len() - 8);
                    let col_idx = _to_usize(GLOBAL_DATA, off) % num_cols;
                    let mut c_iter = tdee0.col(col_idx);
                    let _ = c_iter.next();
                }
                4 => {
                    let off = (800 + i * 8) % (GLOBAL_DATA.len() - 8);
                    let row_idx = _to_usize(GLOBAL_DATA, off) % num_rows;
                    let row_mut = tviewmut0.index_mut(row_idx);
                    if !row_mut.is_empty() {
                        row_mut[0] = CustomType0::default();
                    }
                }
                5 => {
                    let off = (1000 + i * 8) % (GLOBAL_DATA.len() - 8);
                    let col_idx = _to_usize(GLOBAL_DATA, off) % num_cols;
                    let mut c_mut_iter = tviewmut0.col_mut(col_idx);
                    let _ = c_mut_iter.next();
                }
                6 => {
                    let _ = TooDee::from(tview0.clone());
                }
                _ => {
                    let _ = TooDee::from(tview1.clone());
                }
            }
        }

        let final_off = 1500 % (GLOBAL_DATA.len() - 8);
        let final_idx = _to_usize(GLOBAL_DATA, final_off) % num_rows;
        let final_row = tview0.index(final_idx);
        if !final_row.is_empty() {
            println!("{:?}", &final_row[0]);
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