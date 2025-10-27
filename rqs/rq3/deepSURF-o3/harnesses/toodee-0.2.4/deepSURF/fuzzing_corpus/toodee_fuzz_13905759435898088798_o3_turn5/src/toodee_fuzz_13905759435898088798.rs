#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use toodee::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

struct CustomType0(String);

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 600 {
            return;
        }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let num_cols = (_to_usize(GLOBAL_DATA, 0) % 33).max(1);
        let num_rows = (_to_usize(GLOBAL_DATA, 8) % 33).max(1);

        let vec_len = ((num_cols * num_rows) % 65).max(1);
        if 16 + vec_len >= GLOBAL_DATA.len() {
            return;
        }

        let mut raw_vec: Vec<u8> = (0..vec_len)
            .map(|i| _to_u8(GLOBAL_DATA, 16 + i))
            .collect();

        let selector = _to_u8(GLOBAL_DATA, 32) % 4;
        let mut toodee = match selector {
            0 => TooDee::<u8>::new(num_cols, num_rows),
            1 => {
                let init_val = _to_u8(GLOBAL_DATA, 33);
                TooDee::<u8>::init(num_cols, num_rows, init_val)
            }
            2 => {
                let v = raw_vec.clone();
                TooDee::<u8>::from_vec(num_cols, num_rows, v)
            }
            _ => {
                let cap = (_to_usize(GLOBAL_DATA, 40) % 65).max(1);
                TooDee::<u8>::with_capacity(cap)
            }
        };

        if !_to_bool(GLOBAL_DATA, 48) && raw_vec.len() == num_cols {
            toodee.push_row(raw_vec.clone());
        } else if raw_vec.len() == num_rows {
            toodee.push_col(raw_vec.clone());
        }

        let mut backing_slice: Vec<u8> = (0..(num_cols * num_rows).max(1))
            .map(|i| _to_u8(GLOBAL_DATA, (50 + i) % GLOBAL_DATA.len()))
            .collect();
        let slice_cols = (num_cols % 33).max(1);
        let slice_rows = (num_rows % 33).max(1);
        let mut view_mut = TooDeeViewMut::new(slice_cols, slice_rows, &mut backing_slice[..]);

        let ops_total = _to_u8(GLOBAL_DATA, 56) % 20;
        let mut cursor = 57usize;
        for _ in 0..ops_total {
            if cursor >= GLOBAL_DATA.len() {
                break;
            }
            match _to_u8(GLOBAL_DATA, cursor) % 8 {
                0 => {
                    let (c1, c2) = (
                        _to_usize(GLOBAL_DATA, cursor + 1) % slice_rows,
                        _to_usize(GLOBAL_DATA, cursor + 9) % slice_rows,
                    );
                    view_mut.swap_rows(c1, c2);
                }
                1 => {
                    let col_idx = _to_usize(GLOBAL_DATA, cursor + 1) % slice_cols;
                    let col_iter = view_mut.col_mut(col_idx);
                    let mut col_iter_rev = col_iter.rev();
                    let _ = col_iter_rev.next();
                }
                2 => {
                    let mut flat_iter = view_mut.cells_mut();
                    let _ = flat_iter.next();
                }
                3 => {
                    let (start_c, start_r) = (
                        _to_usize(GLOBAL_DATA, cursor + 1) % slice_cols,
                        _to_usize(GLOBAL_DATA, cursor + 9) % slice_rows,
                    );
                    let (end_c, end_r) = (
                        _to_usize(GLOBAL_DATA, cursor + 17) % slice_cols,
                        _to_usize(GLOBAL_DATA, cursor + 25) % slice_rows,
                    );
                    let v = view_mut.view((start_c, start_r), (end_c, end_r));
                    let _ = v.rows().last();
                }
                4 => {
                    let mut t_view = view_mut.view_mut((0, 0), (slice_cols - 1, slice_rows - 1));
                    let _row = _unwrap_option(t_view.rows_mut().next());
                    println!("{:?}", &_row[0]);
                }
                5 => {
                    let col_idx = _to_usize(GLOBAL_DATA, cursor + 1) % num_cols;
                    let _ = toodee.col(col_idx).next_back();
                }
                6 => {
                    if let Some(mut drain) = toodee.pop_col() {
                        let _ = drain.next();
                    }
                }
                _ => {
                    let _ = toodee.rows().next();
                }
            }
            cursor += 33;
        }

        let final_choice = _to_u8(GLOBAL_DATA, 400) % 2;
        match final_choice {
            0 => {
                let _new_td = TooDee::<u8>::from(view_mut);
                let _ = _new_td.data();
            }
            _ => {
                let _view: TooDeeView<u8> = view_mut.into();
                println!("{:?}", _view.num_cols());
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