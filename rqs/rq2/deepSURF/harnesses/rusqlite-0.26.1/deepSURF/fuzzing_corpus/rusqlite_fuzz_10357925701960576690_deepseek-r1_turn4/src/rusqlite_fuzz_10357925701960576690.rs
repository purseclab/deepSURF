#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use rusqlite::*;
use rusqlite::config::DbConfig;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 128 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let flags_choice = _to_u8(GLOBAL_DATA, 0) % 4;
        let flags = match flags_choice {
            0 => OpenFlags::empty(),
            1 => OpenFlags::all(),
            2 => OpenFlags::from_bits_truncate(_to_i32(GLOBAL_DATA, 1)),
            _ => OpenFlags::from_bits_truncate(OpenFlags::SQLITE_OPEN_READ_WRITE.bits()),
        };

        let mut conn = _unwrap_result(Connection::open_in_memory_with_flags(flags));
        let tx_behavior = _to_u8(GLOBAL_DATA, 5) % 4;
        let mut tx = _unwrap_result(match tx_behavior {
            0 => conn.transaction_with_behavior(TransactionBehavior::Deferred),
            1 => conn.transaction_with_behavior(TransactionBehavior::Immediate),
            2 => conn.transaction_with_behavior(TransactionBehavior::Exclusive),
            _ => conn.transaction()
        });

        for i in 0..(_to_u8(GLOBAL_DATA, 6) % 16) {
            let op_type = _to_u8(GLOBAL_DATA, 7 + i as usize) % 8;
            match op_type {
                0 => {
                    let stmt_len = _to_u8(GLOBAL_DATA, 15 + i as usize) % 32;
                    let sql = _to_str(GLOBAL_DATA, 20 + i as usize * 2, 20 + i as usize * 2 + stmt_len as usize);
                    let mut stmt = _unwrap_result(tx.prepare(sql));
                    let param_count = _to_u8(GLOBAL_DATA, 50 + i as usize) % 16;
                    for p in 0..param_count {
                        let value = _to_i32(GLOBAL_DATA, 70 + p as usize);
                        _unwrap_result(stmt.raw_bind_parameter(p as usize + 1, value));
                    }
                    let _ = _unwrap_result(stmt.raw_execute());
                }
                1 => {
                    let idx = _to_usize(GLOBAL_DATA, 90 + i as usize) % 32;
                    let stmt = _unwrap_result(tx.prepare("SELECT ?"));
                    println!("{:?}", stmt.parameter_name(idx));
                }
                2 => {
                    let savepoint_name = _to_str(GLOBAL_DATA, 100 + i as usize, 110 + i as usize);
                    let sp = _unwrap_result(tx.savepoint_with_name(savepoint_name));
                    _unwrap_result(sp.commit());
                }
                3 => {
                    let mut stmt = _unwrap_result(tx.prepare("SELECT name FROM sqlite_master"));
                    let rows = _unwrap_result(stmt.query([]));
                    let _ = rows.mapped(|row| {
                        let _: String = row.get_unwrap(0);
                        Ok(())
                    });
                }
                4 => {
                    let idx = _to_usize(GLOBAL_DATA, 120 + i as usize) % 16;
                    let _ = tx.deref().db_config(DbConfig::SQLITE_DBCONFIG_LEGACY_ALTER_TABLE).unwrap_or(false);
                }
                5 => {
                    let sql = _to_str(GLOBAL_DATA, 130 + i as usize, 140 + i as usize);
                    let _ = _unwrap_result(tx.execute_batch(sql));
                }
                6 => {
                    let param = _to_i32(GLOBAL_DATA, 150 + i as usize);
                    let _ = _unwrap_result(tx.execute("INSERT INTO test VALUES (?)", [param]));
                    let _ = tx.deref().last_insert_rowid();
                }
                _ => {
                    let mut batch = Batch::new(tx.deref(), "SELECT ?");
                    while let Some(mut stmt) = _unwrap_result(batch.next()) {
                        _unwrap_result(stmt.raw_bind_parameter(1, _to_i32(GLOBAL_DATA, 160)));
                        let _ = _unwrap_result(stmt.raw_execute());
                    }
                    let handle = tx.deref().get_interrupt_handle();
                    handle.interrupt();
                }
            }
        }

        {
            let final_stmt = _unwrap_result(tx.prepare(_to_str(GLOBAL_DATA, 180, 210)));
            let param_idx = _to_usize(GLOBAL_DATA, 210) % 32;
            final_stmt.parameter_name(param_idx);
        }
        _unwrap_result(tx.commit());
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