#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use toodee::*;
use global_data::*;
use std::ops::{Index, IndexMut};

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 4096 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let cols = _to_usize(GLOBAL_DATA, 0) % 65;
        let rows = _to_usize(GLOBAL_DATA, 8) % 65;
        let total_cells = cols.saturating_mul(rows);

        let mut vec_data = Vec::with_capacity(total_cells);
        let mut offset = 16;

        for _ in 0..total_cells {
            if offset + 1 > GLOBAL_DATA.len() { break; }
            let len = _to_u8(GLOBAL_DATA, offset) as usize % 65;
            offset += 1;
            
            let end = std::cmp::min(offset + len, GLOBAL_DATA.len());
            let s = _to_str(GLOBAL_DATA, offset, end);
            vec_data.push(String::from(s));
            offset = end;
        }

        let toodee_constructor = _to_u8(GLOBAL_DATA, 160) % 4;
        let mut toodee = match toodee_constructor {
            0 => TooDee::from_vec(cols, rows, vec_data),
            1 => TooDee::with_capacity(cols),
            2 => TooDee::init(cols, rows, String::new()),
            _ => TooDee::new(cols, rows),
        };

        let ops_count = _to_u8(GLOBAL_DATA, offset) % 8;
        offset += 1;

        for _ in 0..ops_count {
            let op_choice = _to_u8(GLOBAL_DATA, offset) % 10;
            offset += 1;

            match op_choice {
                0 => {
                    let r1 = _to_usize(GLOBAL_DATA, offset) % toodee.num_rows();
                    let r2 = _to_usize(GLOBAL_DATA, offset + 8) % toodee.num_rows();
                    toodee.swap_rows(r1, r2);
                    let mut view = toodee.view_mut((0,0), (cols, rows));
                    view.swap_rows(r1, r2);
                    offset += 16;
                }
                1 => {
                    let idx = _to_usize(GLOBAL_DATA, offset) % toodee.num_cols();
                    let mut col = toodee.col_mut(idx);
                    let cell = col.nth(0);
                    for c in &mut col {
                        *c = String::new();
                    }
                    println!("Col {}: {:?}", idx, cell);
                    offset += 8;
                }
                2 => {
                    let start_col = _to_usize(GLOBAL_DATA, offset) % cols.saturating_add(1);
                    let start_row = _to_usize(GLOBAL_DATA, offset + 8) % rows.saturating_add(1);
                    let end_col = _to_usize(GLOBAL_DATA, offset + 16) % cols.saturating_add(1);
                    let end_row = _to_usize(GLOBAL_DATA, offset + 24) % rows.saturating_add(1);
                    let view = toodee.view((start_col, start_row), (end_col, end_row));
                    let first_row = view.rows().next();
                    println!("View: {:?}", first_row);
                    offset += 32;
                }
                3 => {
                    let idx = _to_usize(GLOBAL_DATA, offset) % (toodee.num_cols() + 1);
                    let col_idx = _to_usize(GLOBAL_DATA, offset + 8) % toodee.num_cols();
                    let items: Vec<_> = toodee.col(col_idx).cloned().collect();
                    toodee.insert_col(idx, items);
                    offset += 16;
                }
                4 => {
                    let r = _to_usize(GLOBAL_DATA, offset) % toodee.num_rows();
                    let row = &mut toodee[r];
                    println!("Row {} len: {}", r, row.len());
                    row.iter_mut().for_each(|c| *c = String::from("FUZZ"));
                    offset += 8;
                }
                5 => {
                    if let Some(mut drain) = toodee.pop_col() {
                        let drained: Vec<_> = drain.by_ref().collect();
                        println!("Draining {} elements", drained.len());
                    }
                    offset += 8;
                }
                6 => {
                    let start = (_to_usize(GLOBAL_DATA, offset), _to_usize(GLOBAL_DATA, offset + 8));
                    offset += 16;
                    let mut view = toodee.view_mut(start, (cols, rows));
                    view.flip_cols();
                    let new_toodee = TooDee::from(view);
                    println!("Flipped cols: {:?}", new_toodee.data().first());
                }
                7 => {
                    let row_idx = _to_usize(GLOBAL_DATA, offset) % toodee.num_rows();
                    let mut row_iter = toodee.rows_mut();
                    if let Some(row) = row_iter.nth(row_idx) {
                        row.fill(String::from("FUZZ2"));
                    }
                    offset += 8;
                }
                8 => {
                    let r1 = _to_usize(GLOBAL_DATA, offset);
                    let r2 = _to_usize(GLOBAL_DATA, offset + 8);
                    let mut view = toodee.view_mut((0,0), (cols, rows));
                    view.swap_rows(r1, r2);
                    println!("Swapped rows {} and {}", r1, r2);
                    offset += 16;
                }
                _ => {
                    let mut rotated = toodee.view_mut((0,0), (cols, rows));
                    rotated.translate_with_wrap((cols/2, rows/2));
                    let sample = rotated[(0,0)].clone();
                    println!("Translated: {:?}", sample);
                }
            }
        }

        let mut view_mut = toodee.view_mut((0, 0), (cols, rows));
        let r1 = _to_usize(GLOBAL_DATA, offset) % view_mut.num_rows();
        let r2 = _to_usize(GLOBAL_DATA, offset + 8) % view_mut.num_rows();
        view_mut.swap_rows(r1, r2);
        let final_toodee = TooDee::from(view_mut);
        println!("Final rows: {:?}", final_toodee.rows().count());
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