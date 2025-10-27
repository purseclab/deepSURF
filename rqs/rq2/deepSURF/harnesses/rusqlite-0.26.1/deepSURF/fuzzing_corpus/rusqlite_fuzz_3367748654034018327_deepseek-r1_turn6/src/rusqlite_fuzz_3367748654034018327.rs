#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use rusqlite::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};
use rusqlite::types::ValueRef;

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 1024 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let flags = OpenFlags::from_bits(_to_i32(GLOBAL_DATA, 0)).unwrap_or(OpenFlags::empty());
        let mut conn = _unwrap_result(Connection::open_in_memory_with_flags(flags));
        
        let ops_count = _to_u8(GLOBAL_DATA, 4) % 5;
        for i in 0..ops_count {
            let op_type = _to_u8(GLOBAL_DATA, 5 + i as usize) % 5;
            
            match op_type {
                0 => {
                    let mut offset = 10 + i as usize * 20;
                    let sql_len = _to_u8(GLOBAL_DATA, offset) % 65;
                    let sql = _to_str(GLOBAL_DATA, offset + 1, offset + 1 + sql_len as usize);
                    let mut stmt = _unwrap_result(conn.prepare(sql));
                    
                    let param_count = stmt.parameter_count();
                    for j in 0..param_count {
                        stmt.raw_bind_parameter(j + 1, _to_i32(GLOBAL_DATA, offset + j as usize)).ok();
                    }
                    let _ = stmt.execute([]);
                    let col_name = _to_str(GLOBAL_DATA, offset + 10, offset + 15);
                    let _ = stmt.column_index(col_name);
                    println!("{:?}", stmt.column_names());
                }
                1 => {
                    let mut tx = _unwrap_result(conn.transaction_with_behavior(TransactionBehavior::Deferred));
                    let mut savepoint = _unwrap_result(tx.savepoint());
                    let _ = savepoint.execute("CREATE TABLE test (id INTEGER PRIMARY KEY)", []);
                    
                    let sp_name = format!("sp{}", i);
                    {
                        let mut nested_sp = _unwrap_result(savepoint.savepoint_with_name(&sp_name));
                        let commit_flag = _to_u8(GLOBAL_DATA, 150 + i as usize) % 2;
                        if commit_flag == 0 {
                            let _ = nested_sp.commit();
                        } else {
                            let _ = nested_sp.rollback();
                        }
                    }
                    let _ = savepoint.execute("INSERT INTO test (id) VALUES (?)", [_to_i32(GLOBAL_DATA, 160 + i as usize)]);
                }
                2 => {
                    let mut offset = 50 + i as usize * 30;
                    let mut batch = Batch::new(&conn, "SELECT * FROM sqlite_master");
                    while let Ok(Some(mut stmt)) = batch.next() {
                        {
                            let mut rows = stmt.raw_query();
                            while let Ok(Some(row)) = rows.next() {
                                let val: ValueRef = row.get_ref(0).unwrap();
                                println!("{:?}", val);
                                let _ = row.get::<_, i32>(0);
                            }
                        }
                        let _ = stmt.execute([]);
                    }
                }
                3 => {
                    let offset = 200 + i as usize * 10;
                    let mut handle = conn.get_interrupt_handle();
                    handle.interrupt();
                    let _ = conn.query_row("SELECT 1", [], |r| r.get::<_, i32>(0));
                    let _ = conn.execute_batch("PRAGMA journal_mode=WAL");
                }
                4 => {
                    let offset = 300 + i as usize * 20;
                    let _ = conn.pragma_query_value(None::<DatabaseName>, "secure_delete", |row| {
                        let val: bool = row.get(0)?;
                        Ok(val)
                    });
                    let _ = conn.execute(
                        "INSERT INTO test (id) VALUES (?)", 
                        [_to_i32(GLOBAL_DATA, offset)]
                    );
                    let _ = conn.pragma_update(None::<DatabaseName>, "cache_size", -10000);
                    let mut cached_stmt = _unwrap_result(conn.prepare_cached("SELECT * FROM sqlite_master"));
                    let _ = cached_stmt.execute([]);
                }
                _ => unreachable!()
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