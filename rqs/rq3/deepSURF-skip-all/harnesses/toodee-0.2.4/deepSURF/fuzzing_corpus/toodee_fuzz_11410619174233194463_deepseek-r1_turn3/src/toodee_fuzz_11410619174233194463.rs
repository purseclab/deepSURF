#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use toodee::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

#[derive(Debug, Default, Clone)]
struct CustomType0(String);

fn main (){
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 1024 {return;}
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let DATA = global_data.second_half;
        let mut offset = 0;
        
        let mut t1 = match _to_u8(GLOBAL_DATA, 0) % 3 {
            0 => {
                let cap = _to_usize(GLOBAL_DATA, 8);
                offset = 16;
                TooDee::<CustomType0>::with_capacity(cap)
            }
            1 => {
                let cols = _to_usize(GLOBAL_DATA, 8) % 65;
                let rows = _to_usize(GLOBAL_DATA, 16) % 65;
                offset = 24;
                TooDee::new(cols, rows)
            }
            _ => {
                let cols = _to_usize(GLOBAL_DATA, 8) % 65;
                let rows = _to_usize(GLOBAL_DATA, 16) % 65;
                offset = 24;
                let mut v = Vec::with_capacity(cols * rows);
                for _ in 0..cols * rows {
                    let len = _to_usize(DATA, v.len() * 8) % (DATA.len() / 8);
                    let start = v.len() * 8 + 8;
                    let end = start + len;
                    if end > DATA.len() { break; }
                    let s = _to_str(DATA, start, end);
                    v.push(CustomType0(s.to_string()));
                }
                TooDee::from_vec(cols, rows, v)
            }
        };

        let ops = _to_usize(GLOBAL_DATA, offset) % 65;
        offset += 8;

        for _ in 0..ops {
            if offset + 1 > GLOBAL_DATA.len() { break; }
            match _to_u8(GLOBAL_DATA, offset) % 4 {
                0 => {
                    offset += 1;
                    let start = (_to_usize(GLOBAL_DATA, offset), _to_usize(GLOBAL_DATA, offset + 8));
                    let end = (_to_usize(GLOBAL_DATA, offset + 16), _to_usize(GLOBAL_DATA, offset + 24));
                    offset += 32;
                    let view = t1.view(start, end);
                    let t2 = TooDee::from(view);
                    let mut r = t2.rows();
                    println!("{:?}", r.next_back());
                }
                1 => {
                    offset += 1;
                    let col = _to_usize(GLOBAL_DATA, offset);
                    offset += 8;
                    let mut c = t1.col_mut(col);
                    println!("{:?}", c.next());
                }
                2 => {
                    offset += 1;
                    let r1 = _to_usize(GLOBAL_DATA, offset);
                    offset += 8;
                    let r2 = _to_usize(GLOBAL_DATA, offset);
                    offset += 8;
                    t1.swap_rows(r1, r2);
                }
                _ => {
                    offset += 1;
                    let mut view_mut = t1.view_mut((0, 0), (t1.num_cols(), t1.num_rows()));
                    let mut rows_mut = view_mut.rows_mut();
                    let _ = rows_mut.next_back();
                }
            }
        }

        let mut rows = t1.rows();
        let _ = rows.next_back();
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