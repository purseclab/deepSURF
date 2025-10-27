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
        if data.len() < 4096 {return;}
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let mut offset = 0;

        let rows = _to_usize(GLOBAL_DATA, offset);
        offset += 8;
        let cols = _to_usize(GLOBAL_DATA, offset);
        offset += 8;

        let mut vec_data = Vec::new();
        for _ in 0..32 {
            if offset +1 > GLOBAL_DATA.len() {break;}
            let len = _to_u8(GLOBAL_DATA, offset) % 17;
            offset += 1;
            if offset + len as usize > GLOBAL_DATA.len() {break;}
            let s = _to_str(GLOBAL_DATA, offset, offset + len as usize);
            vec_data.push(CustomType0(s.to_string()));
            offset += len as usize;
        }
        vec_data.truncate(_to_u8(GLOBAL_DATA, 16) as usize % 33);

        let mut tdee = match _to_u8(GLOBAL_DATA, 24) % 3 {
            0 => TooDee::from_vec(rows, cols, vec_data),
            1 => {
                let boxed = vec_data.into_boxed_slice();
                TooDee::from_box(rows, cols, boxed)
            }
            _ => {
                let mut t = TooDee::new(rows, cols);
                for r in 0..rows {
                    let start = offset + r * cols;
                    let end = start + cols;
                    if end > GLOBAL_DATA.len() {break;}
                    let row_data: Vec<_> = (start..end).map(|i| CustomType0(_to_str(GLOBAL_DATA, i, i+1).to_string())).collect();
                    t.push_row(row_data);
                }
                t
            }
        };

        let op_count = _to_u8(GLOBAL_DATA, offset) % 10;
        offset += 1;
        for _ in 0..op_count {
            let op = _to_u8(GLOBAL_DATA, offset) % 5;
            offset += 1;
            match op {
                0 => {
                    let r1 = _to_usize(GLOBAL_DATA, offset);
                    offset += 8;
                    let r2 = _to_usize(GLOBAL_DATA, offset);
                    offset += 8;
                    tdee.swap_rows(r1, r2);
                }
                1 => {
                    let mut view = tdee.view_mut((0,0), (rows, cols));
                    for cell in view.rows_mut() {
                        for elem in cell {
                            elem.0.push('!');
                        }
                    }
                }
                2 => {
                    let c = _to_usize(GLOBAL_DATA, offset);
                    offset += 8;
                    let mut col = tdee.col_mut(c);
                    for item in &mut col {
                        println!("{}", item.0);
                    }
                }
                3 => {
                    let start = (_to_usize(GLOBAL_DATA, offset), _to_usize(GLOBAL_DATA, offset+8));
                    offset += 16;
                    let end = (_to_usize(GLOBAL_DATA, offset), _to_usize(GLOBAL_DATA, offset+8));
                    offset += 16;
                    let v = tdee.view(start, end);
                    let _ = TooDee::from(v);
                }
                _ => {
                    let c = _to_usize(GLOBAL_DATA, offset);
                    offset += 8;
                    let dc = tdee.remove_col(c);
                    for elem in dc {
                        println!("{:?}", elem.0);
                    }
                }
            }
        }

        let start = (_to_usize(GLOBAL_DATA, offset), _to_usize(GLOBAL_DATA, offset+8));
        offset += 16;
        let end = (_to_usize(GLOBAL_DATA, offset), _to_usize(GLOBAL_DATA, offset+8));
        let view = tdee.view_mut(start, end);
        let _ = TooDee::from(view);
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