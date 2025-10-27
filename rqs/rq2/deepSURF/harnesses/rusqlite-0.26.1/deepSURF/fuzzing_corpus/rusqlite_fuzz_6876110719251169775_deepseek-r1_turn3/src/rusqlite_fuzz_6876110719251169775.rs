#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use rusqlite::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 256 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let conn_flags = match _to_u8(GLOBAL_DATA, 0) % 4 {
            0 => OpenFlags::empty(),
            1 => OpenFlags::SQLITE_OPEN_READ_WRITE,
            2 => OpenFlags::SQLITE_OPEN_CREATE,
            3 => OpenFlags::SQLITE_OPEN_MEMORY,
            _ => unreachable!(),
        };

        let mut conn = _unwrap_result(Connection::open_in_memory_with_flags(conn_flags));

        for i in 0..(_to_usize(GLOBAL_DATA, 1) % 4) {
            let operation = _to_u8(GLOBAL_DATA, 2 + i as usize) % 6;
            match operation {
                0 => {
                    let mut tx = _unwrap_result(conn.transaction());
                    let stmt = _unwrap_result(tx.prepare(_to_str(GLOBAL_DATA, 5, 125)));
                    let status_type = match _to_u8(GLOBAL_DATA, 125) % 7 {
                        0 => StatementStatus::FullscanStep,
                        1 => StatementStatus::Sort,
                        2 => StatementStatus::Run,
                        3 => StatementStatus::RePrepare,
                        4 => StatementStatus::MemUsed,
                        5 => StatementStatus::AutoIndex,
                        6 => StatementStatus::VmStep,
                        _ => unreachable!(),
                    };
                    println!("{:?}", stmt.get_status(status_type));
                }
                1 => {
                    let mut tx = _unwrap_result(conn.transaction());
                    let mut savepoint = _unwrap_result(tx.savepoint());
                    let sql = _to_str(GLOBAL_DATA, 130, 134);
                    let param = _to_i32(GLOBAL_DATA, 134);
                    let _ = savepoint.execute_named(sql, &[(":value", &param)]);
                    let _ = savepoint.commit();
                }
                2 => {
                    let mut batch = Batch::new(&conn, _to_str(GLOBAL_DATA, 134, 180));
                    while let Some(_) = _unwrap_result(batch.next()) {}
                }
                3 => {
                    let mut tx = _unwrap_result(conn.transaction());
                    let _ = tx.execute_batch(_to_str(GLOBAL_DATA, 180, 200));
                }
                4 => {
                    let mut sp = _unwrap_result(conn.savepoint());
                    let _ = sp.rollback();
                }
                5 => {
                    let mut stmt = _unwrap_result(conn.prepare_cached(_to_str(GLOBAL_DATA, 200, 220)));
                    let _ = stmt.query(params![_to_u32(GLOBAL_DATA, 220)]);
                }
                _ => unreachable!(),
            }
        }

        let mut tx = _unwrap_result(conn.transaction());
        let mut stmt = _unwrap_result(tx.prepare(_to_str(GLOBAL_DATA, 230, 250)));
        let final_status = match _to_u8(GLOBAL_DATA, 250) % 7 {
            0 => StatementStatus::FullscanStep,
            1 => StatementStatus::Sort,
            2 => StatementStatus::Run,
            3 => StatementStatus::RePrepare,
            4 => StatementStatus::MemUsed,
            5 => StatementStatus::AutoIndex,
            6 => StatementStatus::VmStep,
            _ => unreachable!(),
        };
        stmt.get_status(final_status);
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