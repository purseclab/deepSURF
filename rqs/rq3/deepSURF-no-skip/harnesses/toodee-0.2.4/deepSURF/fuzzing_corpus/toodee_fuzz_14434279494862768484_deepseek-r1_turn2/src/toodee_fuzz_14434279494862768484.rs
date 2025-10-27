#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use toodee::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

#[derive(Clone, Default)]
struct CustomType0(String);

fn main (){
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 2048 {return;}
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let mut t_0 = _to_usize(GLOBAL_DATA, 0);
        let mut t_1 = _to_usize(GLOBAL_DATA, 8);
        let mut t_2 = _to_u8(GLOBAL_DATA, 16) % 65;

        let mut t_3 = Vec::with_capacity(64);
        for i in 0..32 {
            let offset = 24 + i * 64;
            let len = _to_u8(GLOBAL_DATA, offset) as usize;
            let s = _to_str(GLOBAL_DATA, offset + 1, offset + 1 + len);
            t_3.push(CustomType0(s.to_string()));
        }
        t_3.truncate(t_2 as usize);

        let ops = _to_u8(GLOBAL_DATA, 1120) % 8;
        let mut view = TooDeeViewMut::new(t_0, t_1, &mut t_3[..]);

        for i in 0..ops {
            let op_type = _to_u8(GLOBAL_DATA, 1121 + i as usize);
            match op_type % 5 {
                0 => {
                    let r1 = _to_usize(GLOBAL_DATA, 1200 + i as usize * 16);
                    let r2 = _to_usize(GLOBAL_DATA, 1208 + i as usize * 16);
                    view.swap_rows(r1, r2);
                    println!("{:?}", view[r1].as_ptr());
                }
                1 => {
                    let col = _to_usize(GLOBAL_DATA, 1300 + i as usize * 8);
                    let mut col_iter = view.col_mut(col);
                    let elem = col_iter.nth(_to_usize(GLOBAL_DATA, 1308 + i as usize * 8));
                    println!("{:?}", elem.unwrap().0);
                }
                2 => {
                    let (w, h) = (view.num_cols(), view.num_rows());
                    let mut new_toodee = TooDee::new(w, h);
                    new_toodee.clone_from_toodee(&view);
                }
                3 => {
                    let mut rows_iter = view.rows_mut();
                    let row = rows_iter.nth(_to_usize(GLOBAL_DATA, 1400 + i as usize * 8));
                    println!("{:?}", row.unwrap().as_ptr());
                }
                4 => {
                    let from_dee = TooDee::from(view.view_mut((0,0), (t_0, t_1)));
                    let _ = from_dee.rows().last();
                }
                _ => {}
            }
        }

        let new_toodee = TooDee::from(view);
        println!("{:?}", new_toodee.num_cols());
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