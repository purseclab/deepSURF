#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use toodee::*;
use global_data::*;
use std::ops::{Index, IndexMut};

#[derive(Debug, Default, Clone)]
struct CustomType0(String);

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 128 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let constructor = _to_u8(GLOBAL_DATA, 0) % 5;
        let mut t = match constructor {
            0 => {
                let cap = _to_usize(GLOBAL_DATA, 8);
                TooDee::<CustomType0>::with_capacity(cap)
            }
            1 => {
                let cols = _to_usize(GLOBAL_DATA, 8);
                let rows = _to_usize(GLOBAL_DATA, 16);
                TooDee::new(cols, rows)
            }
            2 => {
                let cols = _to_usize(GLOBAL_DATA, 8);
                let rows = _to_usize(GLOBAL_DATA, 16);
                let s = _to_str(GLOBAL_DATA, 24, 40);
                TooDee::init(cols, rows, CustomType0(s.to_string()))
            }
            3 => {
                let cols = _to_usize(GLOBAL_DATA, 8);
                let rows = _to_usize(GLOBAL_DATA, 16);
                let s = _to_str(GLOBAL_DATA, 24, 40);
                let v = vec![CustomType0(s.to_string()); cols * rows];
                TooDee::from_vec(cols, rows, v)
            }
            _ => {
                let view = TooDeeView::new(0, 0, &[]);
                TooDee::from(view)
            }
        };

        let ops = _to_u8(GLOBAL_DATA, 43) % 10;
        let mut offset = 44;
        for _ in 0..ops {
            let op_sel = _to_u8(GLOBAL_DATA, offset) % 16;
            offset += 1;

            match op_sel {
                0 => {
                    let coord = (_to_usize(GLOBAL_DATA, offset), _to_usize(GLOBAL_DATA, offset + 8));
                    let _ = t.index(coord);
                    offset += 16;
                }
                1 => {
                    let start = (_to_usize(GLOBAL_DATA, offset), _to_usize(GLOBAL_DATA, offset + 8));
                    let end = (_to_usize(GLOBAL_DATA, offset + 16), _to_usize(GLOBAL_DATA, offset + 24));
                    let view = t.view(start, end);
                    let cell = view.index((_to_usize(GLOBAL_DATA, offset + 32), _to_usize(GLOBAL_DATA, offset + 40)));
                    println!("{:?}", cell);
                    offset += 48;
                }
                2 => {
                    let start = (_to_usize(GLOBAL_DATA, offset), _to_usize(GLOBAL_DATA, offset + 8));
                    let end = (_to_usize(GLOBAL_DATA, offset + 16), _to_usize(GLOBAL_DATA, offset + 24));
                    let mut view_mut = t.view_mut(start, end);
                    view_mut.swap_rows(_to_usize(GLOBAL_DATA, offset + 32), _to_usize(GLOBAL_DATA, offset + 40));
                    let cell = view_mut.index_mut((_to_usize(GLOBAL_DATA, offset + 48), _to_usize(GLOBAL_DATA, offset + 56)));
                    println!("{:?}", cell);
                    offset += 64;
                }
                3 => {
                    let idx = _to_usize(GLOBAL_DATA, offset);
                    let mut col = t.col_mut(idx);
                    let elem = col.nth(_to_usize(GLOBAL_DATA, offset + 8));
                    println!("{:?}", elem);
                    offset += 16;
                }
                4 => {
                    if let Some(drain) = t.pop_col() {
                        for elem in drain {
                            println!("{:?}", elem);
                        }
                    }
                    offset += 8;
                }
                5 => {
                    let col = _to_usize(GLOBAL_DATA, offset);
                    let row = _to_usize(GLOBAL_DATA, offset + 8);
                    t.index_mut((col, row));
                    offset += 16;
                }
                6 => {
                    let cols = _to_usize(GLOBAL_DATA, offset);
                    let rows = _to_usize(GLOBAL_DATA, offset + 8);
                    let _ = t.view_mut((0, 0), (cols, rows));
                    offset += 16;
                }
                7 => {
                    let idx = _to_usize(GLOBAL_DATA, offset);
                    let _ = t.remove_col(idx);
                    offset += 8;
                }
                8 => {
                    let idx = _to_usize(GLOBAL_DATA, offset);
                    let mut rows = t.rows_mut();
                    let _ = rows.nth(idx);
                    offset += 8;
                }
                9 => {
                    let idx = _to_usize(GLOBAL_DATA, offset);
                    let _ = t.col(idx);
                    offset += 8;
                }
                10 => {
                    let mut rows = t.rows();
                    let _ = rows.last();
                    offset += 0;
                }
                11 => {
                    let idx = _to_usize(GLOBAL_DATA, offset);
                    let mut col = t.col_mut(idx);
                    let _ = col.next_back();
                    offset += 8;
                }
                12 => {
                    let idx = _to_usize(GLOBAL_DATA, offset);
                    let s = _to_str(GLOBAL_DATA, offset + 8, offset + 24);
                    let vec = vec![CustomType0(s.to_string())];
                    t.insert_col(idx, vec);
                    offset += 24;
                }
                13 => {
                    let idx = _to_usize(GLOBAL_DATA, offset);
                    let s = _to_str(GLOBAL_DATA, offset + 8, offset + 24);
                    let vec = vec![CustomType0(s.to_string())];
                    t.push_col(vec);
                    offset += 24;
                }
                14 => {
                    let r1 = _to_usize(GLOBAL_DATA, offset);
                    let r2 = _to_usize(GLOBAL_DATA, offset + 8);
                    t.swap_rows(r1, r2);
                    offset += 16;
                }
                _ => {
                    let mut cells = t.cells();
                    let _ = cells.nth(_to_usize(GLOBAL_DATA, offset));
                    offset += 8;
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