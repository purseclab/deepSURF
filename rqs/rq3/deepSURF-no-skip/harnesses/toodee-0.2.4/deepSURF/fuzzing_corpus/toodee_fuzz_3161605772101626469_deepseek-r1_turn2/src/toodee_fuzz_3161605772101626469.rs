#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use std::ops::Index;
use toodee::*;
use global_data::*;

#[derive(Default, Clone)]
struct CustomType0(String);

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 8000 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let mut offset = 0;
        let mut ops_cnt = _to_u8(GLOBAL_DATA, offset) % 65;
        offset += 1;

        let t_0 = _to_usize(GLOBAL_DATA, offset);
        offset += 8;
        let t_1 = _to_usize(GLOBAL_DATA, offset);
        offset += 8;
        let init_val = _to_u8(GLOBAL_DATA, offset);
        offset += 1;

        let mut t_2 = Vec::with_capacity(64);
        for _ in 0..64 {
            if offset + 2 > GLOBAL_DATA.len() { break; }
            let len = _to_u8(GLOBAL_DATA, offset) % 17;
            offset += 1;
            let s = _to_str(GLOBAL_DATA, offset, offset + len as usize);
            offset += len as usize;
            t_2.push(CustomType0(s.to_string()));
        }

        let constructor_choice = _to_u8(GLOBAL_DATA, offset) % 4;
        offset += 1;
        let mut t_3 = match constructor_choice {
            0 => TooDee::from_vec(t_0 / 2, t_1 / 2, t_2),
            1 => TooDee::new(t_0, t_1),
            2 => TooDee::with_capacity(t_0),
            3 => TooDee::init(t_0, t_1, CustomType0(init_val.to_string())),
            _ => unreachable!(),
        };

        for _ in 0..ops_cnt {
            let op_byte = _to_u8(GLOBAL_DATA, offset) % 8;
            offset += 1;
            match op_byte {
                0 => {
                    let row_idx = _to_usize(GLOBAL_DATA, offset);
                    offset += 8;
                    println!("Row count: {:?}", t_3.rows().count());
                    let _ = t_3.rows_mut().nth(row_idx);
                }
                1 => {
                    let (c1, r1) = (
                        _to_usize(GLOBAL_DATA, offset),
                        _to_usize(GLOBAL_DATA, offset + 8),
                    );
                    offset += 16;
                    let (c2, r2) = (
                        _to_usize(GLOBAL_DATA, offset),
                        _to_usize(GLOBAL_DATA, offset + 8),
                    );
                    offset += 16;
                    let view = t_3.view_mut((c1, r1), (c2, r2));
                    let _ = &view;
                }
                2 => {
                    let view = t_3.view((0, 0), (t_3.num_cols(), t_3.num_rows()));
                    let col_idx = _to_usize(GLOBAL_DATA, offset);
                    offset += 8;
                    for item in view.col(col_idx) {
                        println!("Col item: {:?}", item.0);
                    }
                }
                3 => {
                    let c1 = _to_usize(GLOBAL_DATA, offset);
                    offset += 8;
                    let r1 = _to_usize(GLOBAL_DATA, offset);
                    offset += 8;
                    let c2 = _to_usize(GLOBAL_DATA, offset);
                    offset += 8;
                    let r2 = _to_usize(GLOBAL_DATA, offset);
                    offset += 8;
                    let view = t_3.view((c1, r1), (c2, r2));
                    let mut new_toodee = TooDee::from(view);
                    new_toodee.swap_rows(
                        _to_usize(GLOBAL_DATA, offset),
                        _to_usize(GLOBAL_DATA, offset + 8),
                    );
                    offset += 16;
                }
                4 => {
                    let idx = _to_usize(GLOBAL_DATA, offset);
                    offset += 8;
                    let mut col = t_3.col_mut(idx);
                    let _ = col.nth(_to_usize(GLOBAL_DATA, offset));
                    offset += 8;
                }
                5 => {
                    let idx = _to_usize(GLOBAL_DATA, offset);
                    offset += 8;
                    if let Some(row) = t_3.rows_mut().nth(idx) {
                        row.iter_mut().for_each(|v| *v = CustomType0(String::new()));
                    }
                }
                6 => {
                    let col_idx = _to_usize(GLOBAL_DATA, offset);
                    offset += 8;
                    t_3.fill(CustomType0("filled".to_string()));
                    let _ = t_3.col(col_idx);
                }
                7 => {
                    let idx = _to_usize(GLOBAL_DATA, offset);
                    offset += 8;
                    let t_4 = t_3.index(idx);
                    println!("Indexed row: {:?}", t_4.len());
                }
                _ => (),
            }
        }

        let view_mut = t_3.view_mut((0, 0), (t_3.num_cols(), t_3.num_rows()));
        let idx = _to_usize(GLOBAL_DATA, offset);
        let _ = view_mut.index(idx);
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