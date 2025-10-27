#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use toodee::*;
use global_data::*;
use std::ops::{Index, IndexMut};

#[derive(Default, Clone)]
struct CustomType0(String);

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 1024 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        
        let num_ops = _to_usize(GLOBAL_DATA, 0) % 65;
        let mut idx = 8;
        let mut containers = Vec::new();

        for _ in 0..num_ops {
            if idx + 8 > GLOBAL_DATA.len() { break; }
            let op = _to_usize(GLOBAL_DATA, idx) % 17;
            idx += 8;

            match op {
                0 => {
                    let param0 = _to_usize(GLOBAL_DATA, idx);
                    idx += 8;
                    let param1 = _to_usize(GLOBAL_DATA, idx);
                    let num_cols = (param0 % 65) as usize;
                    let num_rows = (param1 % 65) as usize;
                    let v = (0..num_cols*num_rows).map(|_| CustomType0(String::new())).collect();
                    containers.push(TooDee::from_vec(num_cols, num_rows, v));
                }
                1 => {
                    if containers.is_empty() { continue; }
                    let src = _to_usize(GLOBAL_DATA, idx) % containers.len();
                    idx += 8;
                    let dst = _to_usize(GLOBAL_DATA, idx) % containers.len();
                    idx += 8;
                    let c1 = containers.swap_remove(src);
                    containers.push(c1);
                    let last_idx = containers.len() - 1;
                    containers.swap(dst, last_idx);
                }
                2 => {
                    let idx_t = _to_usize(GLOBAL_DATA, idx) % containers.len();
                    idx += 8;
                    if let Some(t) = containers.get_mut(idx_t) {
                        let col = _to_usize(GLOBAL_DATA, idx) % (t.num_cols() + 1);
                        idx += 8;
                        t.insert_col(col, vec![CustomType0(String::new()); t.num_rows()].into_iter());
                    }
                }
                3 => {
                    let col = _to_usize(GLOBAL_DATA, idx) % 65;
                    idx += 8;
                    if let Some(c) = containers.last_mut() {
                        let col_idx = col % c.num_cols();
                        let mut col_mut = c.col_mut(col_idx);
                        let _ = col_mut.nth(_to_usize(GLOBAL_DATA, idx) % (col_mut.len() + 1));
                    }
                    idx += 8;
                }
                4 => {
                    let row = _to_usize(GLOBAL_DATA, idx) % 65;
                    idx += 8;
                    if let Some(c) = containers.last_mut() {
                        let row_idx = row % c.num_rows();
                        let row_mut = &mut c[row_idx];
                        let idx_elem = _to_usize(GLOBAL_DATA, idx) % row_mut.len();
                        idx += 8;
                        let _ = &row_mut[idx_elem];
                    }
                }
                5 => {
                    if let Some(c) = containers.last() {
                        let view = c.view((_to_usize(GLOBAL_DATA, idx) % c.num_cols(), _to_usize(GLOBAL_DATA, idx+8) % c.num_rows()), 
                                        (_to_usize(GLOBAL_DATA, idx+16) % (c.num_cols()+1), _to_usize(GLOBAL_DATA, idx+24) % (c.num_rows()+1)));
                        let mut rows = view.rows();
                        let n = _to_usize(GLOBAL_DATA, idx+32) % (rows.size_hint().0 + 1);
                        let _ = rows.nth_back(n);
                    }
                    idx += 40;
                }
                6 => {
                    if let Some(c) = containers.last_mut() {
                        let c2 = TooDee::from(c.view((0,0), (c.num_cols(), c.num_rows())));
                        containers.push(c2);
                    }
                }
                7 => {
                    if let Some(c) = containers.last_mut() {
                        let num_rows = c.num_rows();
                        let num_cols = c.num_cols();
                        let mut view_mut = c.view_mut((0,0), (num_cols, num_rows));
                        let r1 = _to_usize(GLOBAL_DATA, idx) % num_rows;
                        let r2 = _to_usize(GLOBAL_DATA, idx+8) % num_rows;
                        view_mut.swap_rows(r1, r2);
                    }
                    idx += 16;
                }
                8 => {
                    if let Some(mut c) = containers.pop() {
                        let drained = c.remove_col(_to_usize(GLOBAL_DATA, idx) % c.num_cols());
                        let _ = drained.collect::<Vec<_>>();
                    }
                    idx += 8;
                }
                9 => {
                    if let Some(c) = containers.last_mut() {
                        c.push_col(vec![CustomType0(String::new()); c.num_rows()].into_iter());
                    }
                }
                10 => {
                    if let Some(c) = containers.last() {
                        let mut cells = c.cells();
                        let _ = cells.nth(_to_usize(GLOBAL_DATA, idx) % (cells.len() + 1));
                    }
                    idx += 8;
                }
                11 => {
                    if let Some(c) = containers.last_mut() {
                        let mut cells = c.cells_mut();
                        let _ = cells.nth_back(_to_usize(GLOBAL_DATA, idx) % (cells.len() + 1));
                    }
                    idx += 8;
                }
                12 => {
                    let idx_t = _to_usize(GLOBAL_DATA, idx) % containers.len();
                    idx += 8;
                    if let Some(t) = containers.get(idx_t) {
                        let col = _to_usize(GLOBAL_DATA, idx) % t.num_cols();
                        idx += 8;
                        let row = _to_usize(GLOBAL_DATA, idx) % t.num_rows();
                        idx += 8;
                        println!("{:?}", t[(col, row)].0);
                    }
                }
                13 => {
                    let param = _to_usize(GLOBAL_DATA, idx);
                    idx += 8;
                    let capacity = (param % 65) as usize;
                    containers.push(TooDee::with_capacity(capacity));
                }
                14 => {
                    if let Some(c) = containers.last_mut() {
                        let mut rows_mut = c.rows_mut();
                        let n = _to_usize(GLOBAL_DATA, idx) % (rows_mut.size_hint().0 + 1);
                        let _ = rows_mut.nth_back(n);
                    }
                    idx += 8;
                }
                15 => {
                    let src = _to_usize(GLOBAL_DATA, idx) % (containers.len() + 1);
                    idx += 8;
                    if src < containers.len() {
                        let c = containers[src].clone();
                        containers.push(c);
                    }
                }
                _ => {
                    if let Some(c) = containers.last() {
                        let mut rows = c.rows();
                        let n = _to_usize(GLOBAL_DATA, idx) % (rows.size_hint().0 + 1);
                        let _ = rows.nth_back(n);
                        if let Some(r) = rows.next_back() {
                            println!("{:?}", r.len());
                        }
                    }
                    idx += 8;
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