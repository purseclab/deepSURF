#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use toodee::*;
use global_data::*;

#[derive(Debug, Default, Clone)]
struct CustomType0(String);

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 256 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let constructor_choice = _to_u8(GLOBAL_DATA, 0) % 3;
        let mut t = match constructor_choice {
            0 => TooDee::<CustomType0>::with_capacity(_to_usize(GLOBAL_DATA, 8)),
            1 => {
                let cols = _to_usize(GLOBAL_DATA, 8);
                let rows = _to_usize(GLOBAL_DATA, 16);
                TooDee::<CustomType0>::new(cols, rows)
            }
            2 => {
                let cols = _to_usize(GLOBAL_DATA, 24);
                let rows = _to_usize(GLOBAL_DATA, 32);
                let data = (0..cols*rows).map(|_| CustomType0(String::new())).collect();
                TooDee::from_vec(cols, rows, data)
            }
            _ => unreachable!()
        };

        let ops = _to_usize(GLOBAL_DATA, 0) % 5;
        for _ in 0..ops {
            let op = _to_u8(GLOBAL_DATA, 0) % 4;
            match op {
                0 => {
                    let mut view_mut = t.view_mut(
                        (_to_usize(GLOBAL_DATA, 8), _to_usize(GLOBAL_DATA, 16)),
                        (_to_usize(GLOBAL_DATA, 24), _to_usize(GLOBAL_DATA, 32))
                    );
                    view_mut.swap_rows(_to_usize(GLOBAL_DATA, 40), _to_usize(GLOBAL_DATA, 48));
                }
                1 => {
                    let view = t.view(
                        (_to_usize(GLOBAL_DATA, 56), _to_usize(GLOBAL_DATA, 64)),
                        (_to_usize(GLOBAL_DATA, 72), _to_usize(GLOBAL_DATA, 80))
                    );
                    let rows = view.rows();
                    let _ = rows.last();
                }
                2 => {
                    let _ = t.pop_col();
                    let _ = t.remove_col(_to_usize(GLOBAL_DATA, 88));
                }
                3 => {
                    let col_idx = _to_usize(GLOBAL_DATA, 96);
                    let col_mut = t.col_mut(col_idx);
                    println!("{:?}", col_mut);
                    let _ = col_mut.last();
                }
                _ => ()
            }
        }

        let mut view_mut = t.view_mut((0, 0), (t.num_cols(), t.num_rows()));
        view_mut.swap_rows(0, view_mut.num_rows().saturating_sub(1));

        let mut rows_mut = t.rows_mut();
        let _ = rows_mut.nth(_to_usize(GLOBAL_DATA, 104));

        let view = t.view((0, 0), (t.num_cols(), t.num_rows()));
        let cells = view.cells().nth(_to_usize(GLOBAL_DATA, 112));
        println!("{:?}", cells);

        let removed = t.remove_col(_to_usize(GLOBAL_DATA, 120));
        for item in removed {
            let _ = item;
        }

        let pop = t.pop_col();
        if let Some(col) = pop {
            for elem in col {
                let _ = elem;
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