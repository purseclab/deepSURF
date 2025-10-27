#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use toodee::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

#[derive(Clone)]
struct CustomType0(String);

fn main (){
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 2750 {return;}
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let t_0 = _to_usize(GLOBAL_DATA, 0);
        let t_1 = _to_usize(GLOBAL_DATA, 8);
        let mut t_2 = _to_u8(GLOBAL_DATA, 16) % 33;
        let mut t_3 = std::vec::Vec::with_capacity(32);
        t_3.truncate(t_2 as usize);
        let mut t_132 = toodee::TooDee::from_vec(t_0, t_1, t_3);
        
        let new_toodee = {
            let view_start = (_to_usize(GLOBAL_DATA, 1360), _to_usize(GLOBAL_DATA, 1368));
            let view_end = (_to_usize(GLOBAL_DATA, 1376), _to_usize(GLOBAL_DATA, 1384));
            let mut view = t_132.view_mut(view_start, view_end);
            view.swap_rows(_to_usize(GLOBAL_DATA, 1392), _to_usize(GLOBAL_DATA, 1400));
            
            {
                let mut view_col = view.col_mut(_to_usize(GLOBAL_DATA, 1408));
                view_col.nth(_to_usize(GLOBAL_DATA, 1416));
            }
            
            toodee::TooDee::from(view)
        };
        
        {
            let mut col_twist = t_132.col_mut(_to_usize(GLOBAL_DATA, 1424));
            col_twist.nth(_to_usize(GLOBAL_DATA, 1432));
        }
        
        {
            let mut new_toodee = new_toodee;
            let mut new_col = new_toodee.col_mut(_to_usize(GLOBAL_DATA, 1440));
            new_col.nth(_to_usize(GLOBAL_DATA, 1448));
        }
        
        {
            let mut drain_col = t_132.remove_col(_to_usize(GLOBAL_DATA, 1456));
            let drain_idx = _to_usize(GLOBAL_DATA, 1464);
            let _ = drain_col.nth(drain_idx);
        }
        
        {
            let mut rev_col = t_132.col_mut(_to_usize(GLOBAL_DATA, 1472)).rev();
            rev_col.nth(_to_usize(GLOBAL_DATA, 1480));
        }
        
        t_132.insert_col(_to_usize(GLOBAL_DATA, 1488), [CustomType0(String::new())]);
        t_132.push_col(vec![CustomType0(String::new())]);
        let popped = t_132.pop_col();
        println!("{:?}", popped.is_some());
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