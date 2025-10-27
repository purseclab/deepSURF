#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use toodee::*;
use global_data::*;
use std::ops::{Index, IndexMut};

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 1024 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let mut offset = 0;

        let constructor_selector = _to_u8(global_data.first_half, offset) % 3;
        offset += 1;

        let num_ops = _to_usize(global_data.first_half, offset) % 5;
        offset += 8;

        let mut tdee = match constructor_selector {
            0 => TooDee::new(_to_usize(global_data.first_half, offset), _to_usize(global_data.first_half, offset + 8)),
            1 => TooDee::with_capacity(_to_usize(global_data.first_half, offset)),
            _ => {
                let cols = _to_usize(global_data.first_half, offset);
                let rows = _to_usize(global_data.first_half, offset + 8);
                let mut v = Vec::with_capacity(cols * rows);
                v.resize_with(cols * rows, || ());
                TooDee::from_vec(cols, rows, v)
            }
        };

        for _ in 0..num_ops {
            let op_selector = _to_u8(global_data.first_half, offset) % 6;
            offset += 1;

            match op_selector {
                0 => {
                    let src_cols = _to_usize(global_data.first_half, offset);
                    offset += 8;
                    let src_rows = _to_usize(global_data.first_half, offset);
                    offset += 8;
                    let mut data = vec![(); src_cols * src_rows];
                    let src = TooDee::from_vec(src_cols, src_rows, data);
                    tdee.clone_from_toodee(&src);
                },
                1 => {
                    let view = tdee.view(
                        (_to_usize(global_data.first_half, offset), _to_usize(global_data.first_half, offset + 8)),
                        (_to_usize(global_data.first_half, offset + 16), _to_usize(global_data.first_half, offset + 24))
                    );
                    let _ = view.rows().count();
                },
                2 => {
                    let r1 = _to_usize(global_data.first_half, offset);
                    offset += 8;
                    let r2 = _to_usize(global_data.first_half, offset);
                    offset += 8;
                    tdee.swap_rows(r1, r2);
                },
                3 => {
                    let col = _to_usize(global_data.first_half, offset);
                    offset += 8;
                    let _ = tdee.col_mut(col).nth(_to_usize(global_data.first_half, offset));
                },
                4 => {
                    let start = (_to_usize(global_data.first_half, offset), _to_usize(global_data.first_half, offset + 8));
                    offset += 16;
                    let end = (_to_usize(global_data.first_half, offset), _to_usize(global_data.first_half, offset + 8));
                    offset += 8;
                    let mut view = tdee.view_mut(start, end);
                    view.fill(&());
                },
                _ => {
                    let v = TooDeeView::new(
                        _to_usize(global_data.first_half, offset),
                        _to_usize(global_data.first_half, offset + 8),
                        &mut []
                    );
                    tdee.clone_from_toodee(&v);
                }
            };
        }

        let src = TooDee::new(
            _to_usize(global_data.second_half, 0),
            _to_usize(global_data.second_half, 8)
        );
        tdee.clone_from_toodee(&src);
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