#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use rusqlite::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

struct CustomType0(String);

fn _custom_fn0(_: &rusqlite::Row) -> Result<CustomType0, rusqlite::Error> {
    let global_data = get_global_data();
    let GLOBAL_DATA = global_data.first_half;
    let t_12 = _to_u8(GLOBAL_DATA, 46);
    if t_12 % 2 == 0 {
        panic!("INTENTIONAL PANIC!");
    }
    let mut t_13 = _to_u8(GLOBAL_DATA, 47) % 17;
    let t_14 = _to_str(GLOBAL_DATA, 48, 48 + t_13 as usize);
    let t_15 = String::from(t_14);
    let t_16 = CustomType0(t_15);
    let t_17 = Ok(t_16);
    return t_17;
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 1024 {
            return;
        }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let mut conn = match _to_u8(GLOBAL_DATA, 0) % 3 {
            0 => _unwrap_result(Connection::open_in_memory()),
            1 => _unwrap_result(Connection::open_with_flags(":memory:", OpenFlags::from_bits_truncate(_to_i32(GLOBAL_DATA, 4)))),
            _ => _unwrap_result(Connection::open_in_memory_with_flags(OpenFlags::from_bits_truncate(_to_i32(GLOBAL_DATA, 8)))),
        };

        let mut offset = 16;
        let ops_count = _to_u8(GLOBAL_DATA, offset) % 8;
        offset += 1;

        for _ in 0..ops_count {
            let op_type = _to_u8(GLOBAL_DATA, offset) % 6;
            offset += 1;

            match op_type {
                0 => {
                    let mut tx = _unwrap_result(conn.transaction());
                    let mut sp = _unwrap_result(tx.savepoint_with_name(_to_str(GLOBAL_DATA, offset, offset + 8)));
                    offset += 8;
                    let _ = sp.commit();
                }
                1 => {
                    let name = DatabaseName::Attached(_to_str(GLOBAL_DATA, offset, offset + 8));
                    offset += 8;
                    let val = _to_str(GLOBAL_DATA, offset, offset + 16);
                    offset += 16;
                    let _ = conn.pragma_update(Some(name), "secure_delete", &val);
                }
                2 => {
                    let mut stmt = _unwrap_result(conn.prepare("SELECT * FROM sqlite_master"));
                    let _rows = _unwrap_result(stmt.query(NO_PARAMS));
                }
                3 => {
                    let pragma_name = _to_str(GLOBAL_DATA, offset, offset + 16);
                    offset += 16;
                    conn.pragma_query_value(None::<DatabaseName>, pragma_name, |row| {
                        _custom_fn0(row)
                    }).unwrap();
                }
                4 => {
                    let mut sp = _unwrap_result(conn.savepoint());
                    let _ = sp.rollback();
                }
                5 => {
                    let mut batch = Batch::new(&conn, _to_str(GLOBAL_DATA, offset, offset + 16));
                    offset += 16;
                    loop {
                        match _unwrap_result(batch.next()) {
                            Some(mut stmt) => {
                                let _ = stmt.execute(NO_PARAMS);
                            }
                            None => break,
                        }
                    }
                }
                _ => unreachable!(),
            }
        }

        let t_5 = _to_usize(GLOBAL_DATA, offset);
        offset += 4;
        let database_name = match (t_5 % 3) {
            0 => DatabaseName::Temp,
            1 => DatabaseName::Main,
            _ => {
                let len = _to_u8(GLOBAL_DATA, offset) % 17;
                offset += 1;
                DatabaseName::Attached(_to_str(GLOBAL_DATA, offset, offset + len as usize))
            }
        };

        let param_len = _to_u8(GLOBAL_DATA, offset) % 65;
        offset += 1;
        let pragma_param = _to_str(GLOBAL_DATA, offset, offset + param_len as usize);
        offset += param_len as usize;

        conn.pragma_query_value(Some(database_name), pragma_param, _custom_fn0).unwrap();
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