#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use toodee::*;
use global_data::*;
use std::ops::{DerefMut, IndexMut};

struct CustomType0(String);

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 4096 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let mut index = 0;
        let t0 = _to_usize(GLOBAL_DATA, index);
        index += 8;
        let t1 = _to_usize(GLOBAL_DATA, index);
        index += 8;
        let vec_len = _to_u8(GLOBAL_DATA, index) % 33;
        index += 1;

        let mut vec = Vec::with_capacity(vec_len as usize);
        for _ in 0..vec_len {
            let str_len = _to_u8(GLOBAL_DATA, index) % 17;
            index += 1;
            let s = _to_str(GLOBAL_DATA, index, index + str_len as usize);
            index += str_len as usize;
            vec.push(CustomType0(String::from(s)));
        }

        let mut toodee = TooDee::from_vec(t0, t1, vec);
        
        let op_count = _to_u8(GLOBAL_DATA, index) % 10;
        index += 1;

        for _ in 0..op_count {
            let op_type = _to_u8(GLOBAL_DATA, index) % 7;
            index += 1;

            match op_type {
                0 => {
                    let src_start = (_to_usize(GLOBAL_DATA, index), _to_usize(GLOBAL_DATA, index+8));
                    index += 16;
                    let src_end = (_to_usize(GLOBAL_DATA, index), _to_usize(GLOBAL_DATA, index+8));
                    index += 16;
                    
                    let mut view = toodee.view_mut(src_start, src_end);
                    println!("Created view @ {:?}-{:?}", src_start, src_end);
                    
                    let nested_start = (_to_usize(GLOBAL_DATA, index), _to_usize(GLOBAL_DATA, index+8));
                    index += 16;
                    let nested_end = (_to_usize(GLOBAL_DATA, index), _to_usize(GLOBAL_DATA, index+8));
                    index += 16;
                    
                    let nested = view.view_mut(nested_start, nested_end);
                    println!("Nested view cols: {}", nested.num_cols());
                }
                1 => {
                    let row1 = _to_usize(GLOBAL_DATA, index);
                    index += 8;
                    let row2 = _to_usize(GLOBAL_DATA, index);
                    index += 8;
                    toodee.swap_rows(row1, row2);
                }
                2 => {
                    let col_idx = _to_usize(GLOBAL_DATA, index);
                    index += 8;
                    let col_iter = toodee.col_mut(col_idx);
                    println!("Col {} length: {}", col_idx, col_iter.len());
                }
                3 => {
                    let mut rows = toodee.rows_mut();
                    if rows.len() > 0 {
                        println!("First row len: {}", rows.nth(0).unwrap().len());
                    }
                }
                4 => {
                    let idx = _to_usize(GLOBAL_DATA, index);
                    index += 8;
                    if let Some(mut drained) = toodee.pop_col() {
                        println!("Drained col @ {}: {}", idx, drained.len());
                    }
                }
                5 => {
                    let coord = (_to_usize(GLOBAL_DATA, index), _to_usize(GLOBAL_DATA, index+8));
                    index += 16;
                    println!("Cell value len: {}", toodee[coord].0.len());
                }
                6 => {
                    let t1 = (_to_usize(GLOBAL_DATA, index), _to_usize(GLOBAL_DATA, index+8));
                    index += 16;
                    let t2 = (_to_usize(GLOBAL_DATA, index), _to_usize(GLOBAL_DATA, index+8));
                    index += 16;
                    let mut v = toodee.view_mut(t1, t2);
                    v.swap_rows(0, v.num_rows()-1);
                }
                _ => {}
            }
        }

        let view1 = (_to_usize(GLOBAL_DATA, index), _to_usize(GLOBAL_DATA, index+8));
        index += 16;
        let view2 = (_to_usize(GLOBAL_DATA, index), _to_usize(GLOBAL_DATA, index+8));
        index += 16;
        let mut v1 = toodee.view_mut(view1, view2);

        let view3 = (_to_usize(GLOBAL_DATA, index), _to_usize(GLOBAL_DATA, index+8));
        index += 16;
        let view4 = (_to_usize(GLOBAL_DATA, index), _to_usize(GLOBAL_DATA, index+8));
        index += 16;
        v1.view_mut(view3, view4);
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