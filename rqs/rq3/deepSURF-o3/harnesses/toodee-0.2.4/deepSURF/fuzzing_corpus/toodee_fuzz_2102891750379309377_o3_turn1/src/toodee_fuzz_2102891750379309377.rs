#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use toodee::*;
use global_data::*;
use std::ops::{Deref, DerefMut, Index, IndexMut};

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 600 {
            return;
        }
        set_global_data(data);
        let global_data = get_global_data();
        let g = global_data.first_half;

        /* --- construct TooDee in several different ways --- */
        let num_cols = _to_u8(g, 0) as usize + 1;
        let num_rows = _to_u8(g, 1) as usize + 1;

        let selector = _to_u8(g, 2);
        let mut table = match selector % 4 {
            0 => {
                /* TooDee::new */
                TooDee::<String>::new(num_cols, num_rows)
            }
            1 => {
                /* TooDee::init */
                let init_str = String::from(_to_str(g, 3, 20));
                TooDee::init(num_cols, num_rows, init_str)
            }
            2 => {
                /* TooDee::from_vec */
                let mut backing: Vec<String> = Vec::with_capacity(65);
                let mut cur = 20usize;
                let want = _to_u8(g, 19) as usize % 65;
                for _ in 0..want {
                    let len = _to_u8(g, cur) as usize % 17;
                    let s = String::from(_to_str(g, cur + 1, cur + 1 + len));
                    backing.push(s);
                    cur += 1 + len;
                    if cur + 17 >= g.len() {
                        break;
                    }
                }
                TooDee::from_vec(num_cols, num_rows, backing)
            }
            _ => {
                /* TooDee::with_capacity then push_row */
                let mut t = TooDee::<String>::with_capacity(32);
                let mut cur = 20usize;
                let rows_to_add = (_to_u8(g, 19) % 5) as usize;
                for _ in 0..rows_to_add {
                    let mut row: Vec<String> = Vec::with_capacity(num_cols);
                    for _ in 0..num_cols {
                        let len = _to_u8(g, cur) as usize % 17;
                        let s = String::from(_to_str(g, cur + 1, cur + 1 + len));
                        row.push(s);
                        cur += 1 + len;
                        if cur + 17 >= g.len() {
                            break;
                        }
                    }
                    t.push_row(row);
                    if cur + 17 >= g.len() {
                        break;
                    }
                }
                t
            }
        };

        /* --- perform a series of operations steering towards ColMut::last --- */
        let ops = _to_u8(g, 40) % 16;
        let mut cursor = 41usize;
        for _ in 0..ops {
            if cursor + 16 >= g.len() {
                break;
            }
            match _to_u8(g, cursor) % 7 {
                0 => {
                    /* view and access */
                    let start = (_to_u8(g, cursor + 1) as usize, _to_u8(g, cursor + 2) as usize);
                    let end = (_to_u8(g, cursor + 3) as usize, _to_u8(g, cursor + 4) as usize);
                    let view = table.view(start, end);
                    let rows_it = view.rows();
                    println!("{:?}", rows_it.size_hint());
                }
                1 => {
                    /* rows_mut manipulation */
                    if let Some(mut row_slice) = table.rows_mut().last() {
                        println!("{:?}", row_slice.len());
                    }
                }
                2 => {
                    /* col iterator */
                    let idx = _to_usize(g, cursor + 5);
                    let col_it = table.col(idx);
                    println!("{:?}", col_it.size_hint());
                }
                3 => {
                    /* push_row */
                    let mut new_row: Vec<String> = Vec::with_capacity(num_cols);
                    let mut c = cursor + 6;
                    for _ in 0..num_cols {
                        let len = _to_u8(g, c) as usize % 17;
                        let s = String::from(_to_str(g, c + 1, c + 1 + len));
                        new_row.push(s);
                        c += 1 + len;
                        if c + 17 >= g.len() {
                            break;
                        }
                    }
                    table.push_row(new_row);
                }
                4 => {
                    /* remove_col then iterate */
                    let idx = _to_usize(g, cursor + 5);
                    let mut drain = table.remove_col(idx);
                    let _ = drain.next_back();
                }
                5 => {
                    /* swap_rows */
                    let r1 = _to_usize(g, cursor + 5);
                    let r2 = _to_usize(g, cursor + 9);
                    table.swap_rows(r1, r2);
                }
                _ => {
                    /* TARGET: ColMut::last */
                    let idx = _to_usize(g, cursor + 5);
                    let mut cm = table.col_mut(idx);
                    let _ = cm.last();
                }
            }
            cursor += 16;
        }

        /* --- final mandatory invocation of URAPI --- */
        let final_idx = _to_usize(g, 512);
        let mut final_col = table.col_mut(final_idx);
        let _ = final_col.last();
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