#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use toodee::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

#[derive(Debug)]
struct CustomType0(String);

fn main (){
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 2048 {return;}
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let t_0 = _to_usize(GLOBAL_DATA, 0);
        let t_1 = _to_usize(GLOBAL_DATA, 8);
        let mut t_2 = _to_u8(GLOBAL_DATA, 16) % 65;
        let mut t_3 = std::vec::Vec::with_capacity(64);
        
        for i in 0..32 {
            let offset = 17 + i * 17;
            let str_len = _to_u8(GLOBAL_DATA, offset) % 17;
            let s = _to_str(GLOBAL_DATA, offset + 1, offset + 1 + str_len as usize);
            t_3.push(CustomType0(s.to_string()));
        }
        t_3.truncate(t_2 as usize);
        
        let mut t_132 = toodee::TooDee::from_vec(t_0, t_1, t_3);
        let mut offset = 561;
        
        let ops = _to_u8(GLOBAL_DATA, offset) % 8;
        offset += 1;
        
        for _ in 0..ops {
            let op_type = _to_u8(GLOBAL_DATA, offset) % 5;
            offset += 1;
            
            match op_type {
                0 => {
                    let col = _to_usize(GLOBAL_DATA, offset);
                    offset += 8;
                    t_132.swap_rows(col, _to_usize(GLOBAL_DATA, offset));
                    offset += 8;
                },
                1 => {
                    let view = t_132.view((_to_usize(GLOBAL_DATA, offset), _to_usize(GLOBAL_DATA, offset+8)), 
                                       (_to_usize(GLOBAL_DATA, offset+16), _to_usize(GLOBAL_DATA, offset+24)));
                    offset += 32;
                    println!("View: {}x{}", view.num_cols(), view.num_rows());
                },
                2 => {
                    let col = _to_usize(GLOBAL_DATA, offset);
                    offset += 8;
                    for _ in 0..3 {
                        let mut c = t_132.col_mut(col);
                        c.nth(_to_usize(GLOBAL_DATA, offset));
                        offset += 8;
                    }
                },
                3 => {
                    let mut view_mut = t_132.view_mut((0,0), (t_132.num_cols(), t_132.num_rows()));
                    let row = _to_usize(GLOBAL_DATA, offset);
                    offset +=8;
                    println!("Row: {:?}", &view_mut[row]);
                },
                _ => {
                    let col_idx = _to_usize(GLOBAL_DATA, offset);
                    offset +=8;
                    if let Some(mut col) = t_132.pop_col() {
                        for _ in 0..2 {
                            col.next();
                        }
                    }
                }
            }
        }

        let main_col = _to_usize(GLOBAL_DATA, offset);
        let mut col_iter = t_132.col_mut(main_col);
        col_iter.next();
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