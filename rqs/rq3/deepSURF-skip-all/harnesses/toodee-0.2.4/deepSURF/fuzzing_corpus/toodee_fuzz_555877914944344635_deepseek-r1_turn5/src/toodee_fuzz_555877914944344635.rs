#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use toodee::*;
use global_data::*;
use std::ops::{Deref, DerefMut, Index, IndexMut};

#[derive(Debug, Clone, Default)]
struct CustomType0(String);

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 248 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let constructor_selector = _to_usize(GLOBAL_DATA, 0) % 4;
        let mut t = match constructor_selector {
            0 => TooDee::with_capacity(_to_usize(GLOBAL_DATA, 8)),
            1 => TooDee::new(_to_usize(GLOBAL_DATA, 8), _to_usize(GLOBAL_DATA, 16)),
            2 => {
                let s = _to_str(GLOBAL_DATA, 24, 32);
                TooDee::init(_to_usize(GLOBAL_DATA, 8), _to_usize(GLOBAL_DATA, 16), CustomType0(s.to_string()))
            }
            _ => TooDee::from_vec(_to_usize(GLOBAL_DATA, 8), _to_usize(GLOBAL_DATA, 16), vec![CustomType0("".into()); 64]),
        };

        let num_ops = _to_usize(GLOBAL_DATA, 40) % 5 + 3;
        for i in 0..num_ops {
            let offset = 48 + i * 32;
            match _to_usize(GLOBAL_DATA, offset) % 7 {
                0 => {
                    let mut view_mut = t.view_mut((_to_usize(GLOBAL_DATA, offset+8), _to_usize(GLOBAL_DATA, offset+16)), 
                                             (_to_usize(GLOBAL_DATA, offset+24), _to_usize(GLOBAL_DATA, offset+32)));
                    view_mut.swap_rows(_to_usize(GLOBAL_DATA, offset+8) % 16, _to_usize(GLOBAL_DATA, offset+16) % 16);
                    let num_rows = view_mut.num_rows();
                    let num_cols = view_mut.num_cols();
                    let row = _to_usize(GLOBAL_DATA, offset+24) % num_rows;
                    let cell = &mut view_mut[row][_to_usize(GLOBAL_DATA, offset+32) % num_cols];
                    *cell = CustomType0(_to_str(GLOBAL_DATA, offset+40, offset+48).to_string());
                }
                1 => {
                    let col_idx = _to_usize(GLOBAL_DATA, offset+8) % 32;
                    let col_mut = t.col_mut(col_idx);
                    let rev_col = col_mut.rev();
                    for (i, item) in rev_col.enumerate() {
                        if i % 4 == 0 {
                            *item = CustomType0(format!("{}", _to_usize(GLOBAL_DATA, offset+i*8)));
                        }
                    }
                }
                2 => {
                    let rows = t.rows().nth(_to_usize(GLOBAL_DATA, offset+8) % 8).unwrap();
                    let rev_row = rows.iter().rev();
                    println!("Reversed row contains {} elements", rev_row.count());
                }
                3 => {
                    t.insert_col(_to_usize(GLOBAL_DATA, offset+8) % 16, 
                                vec![CustomType0("fuzz".into()); _to_usize(GLOBAL_DATA, offset+16) % 8].into_iter());
                }
                4 => {
                    let mut view = t.view_mut((0,0), (t.num_cols(), t.num_rows()));
                    view.translate_with_wrap((_to_usize(GLOBAL_DATA, offset+8) % 4, _to_usize(GLOBAL_DATA, offset+16) % 4));
                    println!("Rotated view dimensions: {}x{}", view.num_cols(), view.num_rows());
                }
                5 => {
                    let col_idx = _to_usize(GLOBAL_DATA, offset+8) % t.num_cols();
                    let removed = t.remove_col(col_idx);
                    for item in removed {
                        println!("Removed: {:?}", item);
                    }
                }
                _ => {
                    let num_rows = t.num_rows();
                    let mut rows_mut = t.rows_mut();
                    let row_idx = _to_usize(GLOBAL_DATA, offset+8) % num_rows;
                    if let Some(row) = rows_mut.nth(row_idx) {
                        row.fill(CustomType0(_to_str(GLOBAL_DATA, offset+16, offset+24).to_string()));
                    }
                }
            }
        }

        let main_view = t.view(
            (_to_usize(GLOBAL_DATA, 168), _to_usize(GLOBAL_DATA, 176)),
            (_to_usize(GLOBAL_DATA, 184), _to_usize(GLOBAL_DATA, 192))
        );
        let col_idx = _to_usize(GLOBAL_DATA, 200) % 8;
        let mut transformed = TooDee::from(main_view);
        if let Some(row) = transformed.rows().nth(col_idx % transformed.num_rows()) {
            println!("Row {} first element: {:?}", col_idx, row[0]);
        }

        let mut view_mut_post = transformed.view_mut((0,0), (transformed.num_cols(), transformed.num_rows()));
        view_mut_post.swap_rows(1, 2);
        view_mut_post.translate_with_wrap((2, 3));
        view_mut_post.fill(CustomType0("modified".into()));

        let mut drain = t.remove_col(_to_usize(GLOBAL_DATA, 208) % t.num_cols());
        while let Some(item) = drain.next() {
            println!("Drained: {:?}", item);
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