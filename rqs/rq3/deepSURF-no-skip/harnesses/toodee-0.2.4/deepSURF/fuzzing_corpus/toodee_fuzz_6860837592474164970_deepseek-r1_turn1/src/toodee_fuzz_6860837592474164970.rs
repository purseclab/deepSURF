#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use toodee::*;
use global_data::*;
use std::ops::Index;

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 8192 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let mut off = 0;
        let ops = _to_usize(GLOBAL_DATA, off) % 65;
        off += 8;

        let num_cols = _to_usize(GLOBAL_DATA, off);
        off += 8;
        let num_rows = _to_usize(GLOBAL_DATA, off);
        off += 8;
        let vec_len = (num_cols.saturating_mul(num_rows)) % 129;
        let mut data_vec = Vec::with_capacity(vec_len);
        for _ in 0..vec_len {
            if off + 1 > GLOBAL_DATA.len() { break; }
            data_vec.push(_to_u8(GLOBAL_DATA, off));
            off += 1;
        }
        let mut toodee = TooDee::from_vec(num_cols, num_rows, data_vec);

        for _ in 0..ops {
            if off >= GLOBAL_DATA.len() { break; }
            let op = _to_u8(GLOBAL_DATA, off) % 7;
            off += 1;

            match op {
                0 => {
                    let view = toodee.view((0, 0), (num_cols, num_rows));
                    let x = _to_usize(GLOBAL_DATA, off);
                    off += 8;
                    let y = _to_usize(GLOBAL_DATA, off);
                    off += 8;
                    let _ = view.index((x, y));
                }
                1 => {
                    let mut view_mut = toodee.view_mut((0, 0), (num_cols, num_rows));
                    let r1 = _to_usize(GLOBAL_DATA, off);
                    off += 8;
                    let r2 = _to_usize(GLOBAL_DATA, off);
                    off += 8;
                    view_mut.swap_rows(r1, r2);
                }
                2 => {
                    let col_idx = _to_usize(GLOBAL_DATA, off);
                    off += 8;
                    let mut col = toodee.col_mut(col_idx);
                    for _ in 0..col.len() {
                        if let Some(elem) = col.next() {
                            let _ = *elem;
                        }
                    }
                }
                3 => {
                    let x1 = _to_usize(GLOBAL_DATA, off);
                    off += 8;
                    let y1 = _to_usize(GLOBAL_DATA, off);
                    off += 8;
                    let x2 = _to_usize(GLOBAL_DATA, off);
                    off += 8;
                    let y2 = _to_usize(GLOBAL_DATA, off);
                    off += 8;
                    let view1 = toodee.view((x1, y1), (x2, y2));
                    let x = _to_usize(GLOBAL_DATA, off);
                    off += 8;
                    let y = _to_usize(GLOBAL_DATA, off);
                    off += 8;
                    let _ = view1.index((x, y));
                }
                4 => {
                    let mut rows_mut = toodee.rows_mut();
                    while let Some(row) = rows_mut.next() {
                        let idx = _to_usize(GLOBAL_DATA, off) % row.len();
                        off += 8;
                        let _ = &row[idx];
                    }
                }
                5 => {
                    let toodee_view = toodee.view((0, 0), (num_cols, num_rows));
                    let x = _to_usize(GLOBAL_DATA, off);
                    off += 8;
                    let y = _to_usize(GLOBAL_DATA, off);
                    off += 8;
                    let _ = toodee_view.index((x, y));
                }
                _ => {
                    let mut cloned = toodee.clone();
                    let _ = cloned.insert_col(0, Vec::new());
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