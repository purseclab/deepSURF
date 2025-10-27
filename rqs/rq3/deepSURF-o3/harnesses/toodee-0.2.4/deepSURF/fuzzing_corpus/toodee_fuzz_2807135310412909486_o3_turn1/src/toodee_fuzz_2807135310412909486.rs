#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use toodee::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};
use toodee::TooDeeOps;

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 600 {
            return;
        }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let dim_cols = (_to_u8(GLOBAL_DATA, 0) % 8 + 1) as usize;
        let dim_rows = (_to_u8(GLOBAL_DATA, 1) % 8 + 1) as usize;
        let total_elems = dim_cols * dim_rows;

        let mut raw_vec = Vec::with_capacity(total_elems);
        for i in 0..total_elems {
            let idx = 10 + (i * 4) % (GLOBAL_DATA.len() - 4);
            raw_vec.push(_to_u32(GLOBAL_DATA, idx));
        }

        let init_val = _to_u32(GLOBAL_DATA, 2);

        let ctor_sel = _to_u8(GLOBAL_DATA, 3) % 3;
        let mut td: TooDee<u32> = match ctor_sel {
            0 => TooDee::from_vec(dim_cols, dim_rows, raw_vec.clone()),
            1 => TooDee::init(dim_cols, dim_rows, init_val),
            _ => TooDee::new(dim_cols, dim_rows),
        };

        let view_sel = _to_u8(GLOBAL_DATA, 4) % 3;
        let view: TooDeeView<u32> = match view_sel {
            0 => td.view((0, 0), (dim_cols, dim_rows)),
            1 => {
                let mut vmut = td.view_mut((0, 0), (dim_cols, dim_rows));
                let mut rows_iter = vmut.rows_mut();
                let _ = rows_iter.next_back();
                vmut.into()
            }
            _ => {
                let slice_ref = td.data();
                TooDeeView::new(dim_cols, dim_rows, slice_ref)
            }
        };

        println!("{:?}", view.size());

        let mut rows_iter = view.rows();
        let _ = rows_iter.next();
        let _ = rows_iter.next_back();

        let _ = view.col(0).last();

        let col_idx = _to_usize(GLOBAL_DATA, 8);
        let row_idx = _to_usize(GLOBAL_DATA, 16);
        let cell_ref = view.index((col_idx, row_idx));
        println!("{:?}", *cell_ref);

        let sub_view = view.view((0, 0), (dim_cols.min(2), dim_rows.min(2)));
        println!("{:?}", sub_view.size());

        let op_count = (_to_u8(GLOBAL_DATA, 24) % 5 + 1) as usize;
        for i in 0..op_count {
            match _to_u8(GLOBAL_DATA, 25 + i) % 4 {
                0 => {
                    let c = _to_usize(GLOBAL_DATA, 32 + i);
                    let r = _to_usize(GLOBAL_DATA, 64 + i);
                    let val = view.index((c, r));
                    println!("{:?}", *val);
                }
                1 => {
                    let mut it = view.rows();
                    let _ = it.nth(_to_usize(GLOBAL_DATA, 96 + i) % 4);
                }
                2 => {
                    let mut it = view.col(0);
                    let _ = it.next();
                }
                _ => {
                    println!("{:?}", view.size());
                }
            }
        }

        let td_clone: TooDee<u32> = TooDee::from(view.clone());
        let clone_val = td_clone.index((col_idx, row_idx));
        println!("{:?}", *clone_val);
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