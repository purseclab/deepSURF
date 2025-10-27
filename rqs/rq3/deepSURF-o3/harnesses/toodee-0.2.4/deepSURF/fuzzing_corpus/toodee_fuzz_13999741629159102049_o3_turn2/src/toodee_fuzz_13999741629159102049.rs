#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use toodee::*;
use global_data::*;
use std::ops::{Deref, DerefMut};

#[derive(Debug, Clone, Default)]
struct CustomType0(String);

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 600 {
            return;
        }
        set_global_data(data);
        let global = get_global_data();
        let G = global.first_half;

        let cols = _to_usize(G, 0) % 65 + 1;
        let rows = _to_usize(G, 8) % 65 + 1;
        let elem_count = (_to_u8(G, 16) % 65) as usize;

        let max_idx = G.len().saturating_sub(20);
        let mut vec_data: Vec<CustomType0> = Vec::with_capacity(elem_count);
        for i in 0..elem_count {
            let offset = (20 + i * 3) % max_idx;
            let slice_len = (_to_u8(G, offset) % 17) as usize;
            let end = (offset + slice_len).min(G.len());
            let s = _to_str(G, offset, end);
            vec_data.push(CustomType0(String::from(s)));
        }

        let mode = _to_u8(G, 4);
        let mut td = match mode % 4 {
            0 => TooDee::from_vec(cols, rows, vec_data.clone()),
            1 => TooDee::new(cols, rows),
            2 => {
                let init_val = vec_data.get(0).cloned().unwrap_or_else(CustomType0::default);
                TooDee::init(cols, rows, init_val)
            }
            _ => {
                let mut t = TooDee::with_capacity(cols * rows + 5);
                t.reserve(cols * rows);
                t
            }
        };

        let c1 = _to_usize(G, 24) % cols;
        let r1 = _to_usize(G, 32) % rows;
        let c2 = _to_usize(G, 40) % (cols + 1);
        let r2 = _to_usize(G, 48) % (rows + 1);

        let inner_c1_seed = _to_usize(G, 56);
        let inner_r1_seed = _to_usize(G, 64);
        let inner_c2_seed = _to_usize(G, 72);
        let inner_r2_seed = _to_usize(G, 80);

        let op_count = (_to_u8(G, 88) % 10) as usize;

        {
            let view_main = td.view((c1, r1), (c2, r2));
            println!("{:?}", view_main.bounds());

            let inner_c1 = inner_c1_seed % view_main.num_cols();
            let inner_r1 = inner_r1_seed % view_main.num_rows();
            let inner_c2 = inner_c2_seed % (view_main.num_cols() + 1);
            let inner_r2 = inner_r2_seed % (view_main.num_rows() + 1);

            let sub_view = view_main.view((inner_c1, inner_r1), (inner_c2, inner_r2));
            println!("{:?}", sub_view.size());
        }

        for i in 0..op_count {
            match _to_u8(G, 89 + i) % 6 {
                0 => {
                    let view_main = td.view((c1, r1), (c2, r2));
                    let inner_c1 = inner_c1_seed % view_main.num_cols();
                    let inner_r1 = inner_r1_seed % view_main.num_rows();
                    let inner_c2 = inner_c2_seed % (view_main.num_cols() + 1);
                    let inner_r2 = inner_r2_seed % (view_main.num_rows() + 1);
                    let sub_view = view_main.view((inner_c1, inner_r1), (inner_c2, inner_r2));
                    let col_idx = _to_usize(G, 100 + i) % sub_view.num_cols();
                    let col_iter = sub_view.col(col_idx);
                    println!("{:?}", col_iter.size_hint());
                }
                1 => {
                    let view_main = td.view((c1, r1), (c2, r2));
                    let inner_c1 = inner_c1_seed % view_main.num_cols();
                    let inner_r1 = inner_r1_seed % view_main.num_rows();
                    let inner_c2 = inner_c2_seed % (view_main.num_cols() + 1);
                    let inner_r2 = inner_r2_seed % (view_main.num_rows() + 1);
                    let sub_view = view_main.view((inner_c1, inner_r1), (inner_c2, inner_r2));
                    let row_iter = sub_view.rows();
                    println!("{:?}", row_iter.size_hint());
                }
                2 => {
                    let mut td_view_mut = td.view_mut((0, 0), (cols, rows));
                    let r_a = _to_usize(G, 110 + i) % rows;
                    let r_b = _to_usize(G, 118 + i) % rows;
                    td_view_mut.swap_rows(r_a, r_b);
                }
                3 => {
                    let mut col_mut = td.col_mut(_to_usize(G, 126 + i) % cols);
                    let _ = col_mut.next_back();
                }
                4 => {
                    let _ = td.pop_col();
                }
                _ => {
                    let _ = td.rows().last();
                }
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