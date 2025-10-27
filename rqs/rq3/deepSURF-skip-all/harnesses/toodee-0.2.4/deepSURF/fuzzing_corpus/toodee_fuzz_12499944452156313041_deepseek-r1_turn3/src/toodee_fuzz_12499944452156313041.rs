#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use toodee::*;
use global_data::*;
use std::ops::{Deref, DerefMut, Index, IndexMut};

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 128 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        
        let mut idx = 0;
        let ctor_selector = _to_u8(GLOBAL_DATA, idx) % 4;
        idx += 1;

        let mut t = match ctor_selector {
            0 => {
                let cap = _to_usize(GLOBAL_DATA, idx);
                idx += 8;
                toodee::TooDee::<String>::with_capacity(cap)
            },
            1 => {
                let cols = _to_usize(GLOBAL_DATA, idx);
                idx += 8;
                let rows = _to_usize(GLOBAL_DATA, idx);
                idx += 8;
                toodee::TooDee::new(cols, rows)
            },
            2 => {
                let cols = _to_usize(GLOBAL_DATA, idx) % 20;
                idx += 8;
                let rows = _to_usize(GLOBAL_DATA, idx) % 20;
                idx +=8;
                let mut base = toodee::TooDee::new(cols.max(1), rows.max(1));
                let view = base.view((0,0), (cols, rows));
                toodee::TooDee::from(view)
            },
            3 => {
                let cols = _to_usize(GLOBAL_DATA, idx) % 20 + 1;
                idx += 8;
                let rows = _to_usize(GLOBAL_DATA, idx) % 20 + 1;
                idx +=8;
                let data = (0..cols*rows).map(|i| format!("{i}")).collect();
                toodee::TooDee::from_vec(cols, rows, data)
            },
            _ => unreachable!(),
        };

        let op_count = _to_usize(GLOBAL_DATA, idx) % 7;
        idx +=8;

        for _ in 0..op_count {
            if idx >= GLOBAL_DATA.len() { break; }
            let op = _to_u8(GLOBAL_DATA, idx) % 9;
            idx +=1;

            match op {
                0 => {
                    let n = _to_usize(GLOBAL_DATA, idx);
                    idx +=8;
                    let mut rows = t.rows();
                    println!("{:?}", rows.nth_back(n));
                },
                1 => {
                    let n = _to_usize(GLOBAL_DATA, idx);
                    idx +=8;
                    let mut rows = t.rows_mut();
                    if let Some(row) = rows.nth_back(n) {
                        println!("{:?}", row);
                    }
                },
                2 => {
                    let c = _to_usize(GLOBAL_DATA, idx) % t.num_cols().max(1);
                    idx +=8;
                    t.insert_col(c, (0..t.num_rows()).map(|_| String::new()));
                },
                3 => {
                    let col = _to_usize(GLOBAL_DATA, idx) % t.num_cols().max(1);
                    idx +=8;
                    let mut col = t.col_mut(col);
                    let n = _to_usize(GLOBAL_DATA, idx);
                    idx +=8;
                    println!("{:?}", col.nth_back(n));
                },
                4 => {
                    let c = _to_usize(GLOBAL_DATA, idx) % t.num_cols().max(1);
                    idx +=8;
                    if let Some(drain) = t.pop_col() {
                        for item in drain.rev() {
                            println!("{:?}", item);
                        }
                    }
                },
                5 => {
                    let start = (_to_usize(GLOBAL_DATA, idx) % t.num_cols(), _to_usize(GLOBAL_DATA, idx+8) % t.num_rows());
                    idx +=16;
                    let end = (_to_usize(GLOBAL_DATA, idx) % (t.num_cols()+1), _to_usize(GLOBAL_DATA, idx+8) % (t.num_rows()+1));
                    idx +=16;
                    let mut view = t.view_mut(start, end);
                    let _ = view.rows_mut().nth_back(0);
                },
                6 => {
                    let c = _to_usize(GLOBAL_DATA, idx) % t.num_cols().max(1);
                    idx +=8;
                    let cap = _to_usize(GLOBAL_DATA, idx) % 65;
                    idx +=8;
                    let mut new = toodee::TooDee::<String>::with_capacity(cap);
                    new.clone_from_toodee(&t.view((c, 0), (t.num_cols(), t.num_rows())));
                },
                7 => {
                    let row = _to_usize(GLOBAL_DATA, idx) % t.num_rows().max(1);
                    idx +=8;
                    let val = format!("{}", _to_u8(GLOBAL_DATA, idx) as char);
                    idx +=1;
                    t[row].fill(val);
                },
                8 => {
                    let r1 = _to_usize(GLOBAL_DATA, idx) % t.num_rows().max(1);
                    idx +=8;
                    let r2 = _to_usize(GLOBAL_DATA, idx) % t.num_rows().max(1);
                    idx +=8;
                    let (row0, row1) = t.row_pair_mut(r1, r2);
                    for (d, s) in row0.iter_mut().zip(row1) {
                        *d = s.clone();
                    }
                },
                _ => (),
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