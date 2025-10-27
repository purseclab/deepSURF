#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;    

use toodee::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};
fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 105 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let t_0 = _to_u8(GLOBAL_DATA, 0);
        let t_1 = _to_u16(GLOBAL_DATA, 1);
        let t_2 = _to_u32(GLOBAL_DATA, 3);
        let t_3 = _to_u64(GLOBAL_DATA, 7);
        let t_4 = _to_u128(GLOBAL_DATA, 15);
        let t_5 = _to_usize(GLOBAL_DATA, 31);
        let t_6 = _to_i8(GLOBAL_DATA, 39);
        let t_7 = _to_i16(GLOBAL_DATA, 40);
        let t_8 = _to_i32(GLOBAL_DATA, 42);
        let t_9 = _to_i64(GLOBAL_DATA, 46);
        let t_10 = _to_i128(GLOBAL_DATA, 54);
        let t_11 = _to_isize(GLOBAL_DATA, 70);
        let t_12 = _to_f32(GLOBAL_DATA, 78);
        let t_13 = _to_f64(GLOBAL_DATA, 82);
        let t_14 = _to_char(GLOBAL_DATA, 90);
        let t_15 = _to_bool(GLOBAL_DATA, 94);
        let t_16 = _to_str(GLOBAL_DATA, 95, 105);
        let t_17 = String::from(t_16);

        let cols = (t_0 as usize % 64).max(1);
        let rows = (t_1 as usize % 64).max(1);
        let data_start = 105;
        let mut data_index = data_start;

        let mut toodee = TooDee::new(cols, rows);
        for r in 0..rows {
            for c in 0..cols {
                if data_index >= GLOBAL_DATA.len() { break; }
                toodee[(c, r)] = _to_u8(GLOBAL_DATA, data_index);
                data_index += 1;
            }
        }

        let ops = t_2 % 8;
        for _ in 0..ops {
            if data_index + 2 > GLOBAL_DATA.len() { break; }
            let op_sel = _to_u8(GLOBAL_DATA, data_index) % 6;
            data_index += 1;

            match op_sel {
                0 => {
                    toodee.flip_rows();
                }
                1 => {
                    let r1 = _to_usize(GLOBAL_DATA, data_index) % toodee.num_rows();
                    let r2 = _to_usize(GLOBAL_DATA, data_index + 8) % toodee.num_rows();
                    toodee.swap_rows(r1, r2);
                    data_index += 16;
                }
                2 => {
                    let new_cols = _to_u8(GLOBAL_DATA, data_index) % 64;
                    let new_data: Vec<u8> = (0..toodee.num_rows()).map(|i| _to_u8(GLOBAL_DATA, data_index + i)).collect();
                    toodee.push_col(new_data);
                    data_index += toodee.num_rows();
                }
                3 => {
                    let idx = _to_usize(GLOBAL_DATA, data_index) % (toodee.num_cols() + 1);
                    let new_row: Vec<u8> = (0..toodee.num_cols()).map(|i| _to_u8(GLOBAL_DATA, data_index + i)).collect();
                    toodee.insert_row(idx, new_row);
                    data_index += toodee.num_cols();
                }
                4 => {
                    let row = _to_usize(GLOBAL_DATA, data_index) % toodee.num_rows();
                    let dir = _to_bool(GLOBAL_DATA, data_index + 8);
                    toodee.sort_by_row(row, |a, b| if dir { a.cmp(b) } else { b.cmp(a) });
                    data_index += 9;
                }
                5 => {
                    let mid_col = _to_usize(GLOBAL_DATA, data_index) % toodee.num_cols();
                    let mid_row = _to_usize(GLOBAL_DATA, data_index + 8) % toodee.num_rows();
                    toodee.translate_with_wrap((mid_col, mid_row));
                    data_index += 16;
                }
                _ => ()
            }
        }

        if toodee.num_rows() > 0 && toodee.num_cols() > 0 {
            println!("{:?}", toodee[(0, 0)]);
            println!("{:?}", toodee.rows().next().unwrap());
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