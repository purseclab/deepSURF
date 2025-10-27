#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use toodee::*;
use global_data::*;

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 16384 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let ops = _to_usize(GLOBAL_DATA, 0) % 32;
        let mut offset = 8;

        let mut base_cols = _to_usize(GLOBAL_DATA, offset) % 64 + 1;
        offset += 8;
        let mut base_rows = _to_usize(GLOBAL_DATA, offset) % 64 + 1;
        offset += 8;
        let vec_size = base_cols * base_rows;
        let vec_data = (0..vec_size).map(|i| _to_u8(GLOBAL_DATA, offset + i)).collect();
        offset += vec_size;

        let mut toodee = TooDee::from_vec(base_cols, base_rows, vec_data);

        for _ in 0..ops {
            let op = _to_u8(GLOBAL_DATA, offset) % 6;
            offset += 1;

            match op {
                0 => {
                    let idx = _to_usize(GLOBAL_DATA, offset);
                    let view = toodee.view((0, 0), (toodee.num_cols(), toodee.num_rows()));
                    let _ = view[idx];
                    offset += 8;
                }
                1 => {
                    let cols = _to_usize(GLOBAL_DATA, offset) % 64 + 1;
                    offset += 8;
                    let rows = _to_usize(GLOBAL_DATA, offset) % 64 + 1;
                    offset += 8;
                    let vec_size = cols * rows;
                    let new_data = (0..vec_size).map(|i| _to_u8(GLOBAL_DATA, offset + i)).collect();
                    offset += vec_size;
                    toodee = TooDee::from_vec(cols, rows, new_data);
                    base_cols = cols;
                    base_rows = rows;
                }
                2 => {
                    let view = toodee.view((0, 0), (toodee.num_cols(), toodee.num_rows()));
                    let r1 = _to_usize(GLOBAL_DATA, offset) % view.num_rows();
                    offset += 8;
                    let r2 = _to_usize(GLOBAL_DATA, offset) % view.num_rows();
                    offset += 8;
                    toodee.swap_rows(r1, r2);
                }
                3 => {
                    let view = toodee.view((0, 0), (toodee.num_cols(), toodee.num_rows()));
                    let col_idx = _to_usize(GLOBAL_DATA, offset) % view.num_cols();
                    offset += 8;
                    let col = view.col(col_idx);
                    for item in col.take(10) {
                        println!("{:?}", item);
                    }
                }
                4 => {
                    let view = toodee.view((0, 0), (toodee.num_cols(), toodee.num_rows()));
                    let c = _to_usize(GLOBAL_DATA, offset) % view.num_cols();
                    offset += 8;
                    let r = _to_usize(GLOBAL_DATA, offset) % view.num_rows();
                    offset += 8;
                    let _ = view[(c, r)];
                }
                5 => {
                    let cols = toodee.num_cols();
                    let rows = toodee.num_rows();
                    let mut view_mut = toodee.view_mut((0, 0), (cols, rows));
                    let col = _to_usize(GLOBAL_DATA, offset) % cols;
                    offset += 8;
                    let mut col_mut = view_mut.col_mut(col);
                    col_mut.nth(_to_usize(GLOBAL_DATA, offset) % rows);
                    offset += 8;
                }
                _ => ()
            }
        }

        let view = toodee.view((0, 0), (toodee.num_cols(), toodee.num_rows()));
        let idx = _to_usize(GLOBAL_DATA, GLOBAL_DATA.len() - 8);
        let _ = view[idx];
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