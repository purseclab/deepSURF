#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use toodee::*;
use global_data::*;
use std::ops::{Deref, DerefMut};

struct CustomType0(String);

fn main (){
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 4096 {return;}
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let mut offset = 0;

        let t_0 = _to_usize(GLOBAL_DATA, offset); offset += 8;
        let t_1 = _to_usize(GLOBAL_DATA, offset); offset += 8;
        let mut t_2 = _to_u8(GLOBAL_DATA, offset) % 33; offset += 1;

        let mut t_3 = Vec::with_capacity(32);
        for _ in 0..32 {
            let str_len = _to_u8(GLOBAL_DATA, offset) % 17; offset += 1;
            let s = _to_str(GLOBAL_DATA, offset, offset + str_len as usize);
            offset += str_len as usize;
            t_3.push(CustomType0(s.to_string()));
        }
        t_3.truncate(t_2 as usize);

        let mut t_132 = TooDee::from_vec(t_0, t_1, t_3);
        let view_start = (_to_usize(GLOBAL_DATA, offset), _to_usize(GLOBAL_DATA, offset+8)); offset += 16;
        let view_end = (_to_usize(GLOBAL_DATA, offset), _to_usize(GLOBAL_DATA, offset+8)); offset += 16;
        let mut view = t_132.view_mut(view_start, view_end);

        let ops = _to_u8(GLOBAL_DATA, offset) % 4; offset += 1;
        for _ in 0..ops {
            let op_type = _to_u8(GLOBAL_DATA, offset) % 3; offset += 1;
            match op_type {
                0 => {
                    let col_idx = _to_usize(GLOBAL_DATA, offset); offset += 8;
                    let mut col = view.col_mut(col_idx);
                    for i in 0..col.len() {
                        let str_len = _to_u8(GLOBAL_DATA, offset) % 17; offset += 1;
                        let s = _to_str(GLOBAL_DATA, offset, offset + str_len as usize);
                        offset += str_len as usize;
                        *col.nth(i).unwrap() = CustomType0(s.to_string());
                        println!("{:?}", col.nth(i).unwrap().0);
                    }
                }
                1 => {
                    let start = (_to_usize(GLOBAL_DATA, offset), _to_usize(GLOBAL_DATA, offset+8)); offset += 16;
                    let end = (_to_usize(GLOBAL_DATA, offset), _to_usize(GLOBAL_DATA, offset+8)); offset += 16;
                    let mut sub_view = view.view_mut(start, end);
                    let r1 = _to_usize(GLOBAL_DATA, offset); offset += 8;
                    let r2 = _to_usize(GLOBAL_DATA, offset); offset += 8;
                    sub_view.swap_rows(r1, r2);
                }
                2 => {
                    let r1 = _to_usize(GLOBAL_DATA, offset); offset += 8;
                    let r2 = _to_usize(GLOBAL_DATA, offset); offset += 8;
                    view.swap_rows(r1, r2);
                }
                _ => {
                    let mut rows = view.rows_mut();
                    while let Some(row) = rows.next() {
                        let idx = _to_usize(GLOBAL_DATA, offset) % row.len(); offset += 8;
                        let str_len = _to_u8(GLOBAL_DATA, offset) % 17; offset +=1;
                        let s = _to_str(GLOBAL_DATA, offset, offset + str_len as usize);
                        offset += str_len as usize;
                        row[idx] = CustomType0(s.to_string());
                    }
                }
            }
        }

        let final_r1 = _to_usize(GLOBAL_DATA, offset); offset += 8;
        let final_r2 = _to_usize(GLOBAL_DATA, offset); 
        view.swap_rows(final_r1, final_r2);
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