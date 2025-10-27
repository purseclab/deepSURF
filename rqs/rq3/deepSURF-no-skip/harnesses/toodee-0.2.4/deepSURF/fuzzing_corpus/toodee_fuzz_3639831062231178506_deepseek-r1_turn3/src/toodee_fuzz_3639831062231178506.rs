#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use toodee::*;
use global_data::*;

#[derive(Debug, Clone, Default)]
struct CustomType0(String);

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 3072 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let mut data_index = 0;
        let ctor_choice = _to_u8(GLOBAL_DATA, data_index) % 3;
        data_index += 1;

        let cols = _to_usize(GLOBAL_DATA, data_index) % 64;
        data_index += 8;
        let rows = _to_usize(GLOBAL_DATA, data_index) % 64;
        data_index += 8;

        let mut too_dee = match ctor_choice {
            0 => {
                let mut vec_data = Vec::with_capacity(cols * rows);
                for _ in 0..(cols * rows) {
                    let len = _to_u8(GLOBAL_DATA, data_index) % 17;
                    data_index += 1;
                    let s = _to_str(GLOBAL_DATA, data_index, data_index + len as usize);
                    data_index += len as usize;
                    vec_data.push(CustomType0(s.to_string()));
                }
                TooDee::from_vec(cols, rows, vec_data)
            },
            1 => {
                let mut td = TooDee::new(cols, rows);
                for _ in 0..3 {
                    let len = _to_u8(GLOBAL_DATA, data_index) % 17;
                    data_index += 1;
                    let s = _to_str(GLOBAL_DATA, data_index, data_index + len as usize);
                    data_index += len as usize;
                    td.push_row(vec![CustomType0(s.to_string())].into_iter());
                }
                td
            },
            2 => {
                let mut vec_data = Vec::with_capacity(cols * rows);
                for _ in 0..(cols * rows) {
                    let len = _to_u8(GLOBAL_DATA, data_index) % 17;
                    data_index += 1;
                    let s = _to_str(GLOBAL_DATA, data_index, data_index + len as usize);
                    data_index += len as usize;
                    vec_data.push(CustomType0(s.to_string()));
                }
                let mut view = TooDeeViewMut::new(cols, rows, &mut vec_data);
                TooDee::from(view)
            },
            _ => unreachable!()
        };

        let num_ops = _to_u8(GLOBAL_DATA, data_index) % 8;
        data_index += 1;

        for _ in 0..num_ops {
            let op_select = _to_u8(GLOBAL_DATA, data_index) % 6;
            data_index += 1;

            match op_select {
                0 => {
                    let col_id = _to_usize(GLOBAL_DATA, data_index);
                    data_index += 8;
                    let mut col = too_dee.col_mut(col_id);
                    let last = col.last();
                    println!("{:?}", last);
                },
                1 => {
                    let start = (_to_usize(GLOBAL_DATA, data_index), _to_usize(GLOBAL_DATA, data_index + 8));
                    data_index += 16;
                    let end = (_to_usize(GLOBAL_DATA, data_index), _to_usize(GLOBAL_DATA, data_index + 8));
                    data_index += 16;
                    too_dee.view_mut(start, end);
                },
                2 => {
                    let r1 = _to_usize(GLOBAL_DATA, data_index);
                    data_index += 8;
                    let r2 = _to_usize(GLOBAL_DATA, data_index);
                    data_index += 8;
                    too_dee.swap_rows(r1, r2);
                },
                3 => {
                    let col_idx = _to_usize(GLOBAL_DATA, data_index);
                    data_index += 8;
                    too_dee.insert_col(col_idx, vec![CustomType0(String::new())].into_iter());
                },
                4 => {
                    let row_view = too_dee.rows_mut().last().unwrap();
                    println!("{:?}", row_view);
                },
                5 => {
                    let col_idx = _to_usize(GLOBAL_DATA, data_index);
                    data_index += 8;
                    let mut dc = too_dee.remove_col(col_idx);
                    while let Some(elem) = dc.next() {
                        println!("Drained: {:?}", elem);
                    }
                },
                _ => {}
            }
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