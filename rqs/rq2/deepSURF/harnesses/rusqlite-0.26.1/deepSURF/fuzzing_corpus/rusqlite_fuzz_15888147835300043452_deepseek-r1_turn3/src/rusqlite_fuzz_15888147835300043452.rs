#![forbid(unsafe_code)]

#[macro_use]
extern crate afl;

use rusqlite::*;
use rusqlite::types::ValueRef;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 128 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        
        let flags = OpenFlags::from_bits_truncate(_to_i32(GLOBAL_DATA, 0));
        let conn = match _to_u8(GLOBAL_DATA, 4) % 3 {
            0 => Connection::open_in_memory_with_flags(flags),
            1 => Connection::open_with_flags(":memory:", flags),
            _ => Connection::open_in_memory(),
        };
        let mut conn = _unwrap_result(conn);
        
        _unwrap_result(conn.execute("CREATE TABLE test (id INTEGER PRIMARY KEY, data TEXT)", []));
        
        let ops_count = _to_u8(GLOBAL_DATA, 8) % 8 + 1;
        let mut offset = 12;
        
        for _ in 0..ops_count {
            let op_type = _to_u8(GLOBAL_DATA, offset) % 6;
            offset += 1;
            
            match op_type {
                0 => {
                    let mut tx = _unwrap_result(conn.transaction());
                    let mut savepoint = _unwrap_result(tx.savepoint());
                    _unwrap_result(savepoint.execute("INSERT INTO test (data) VALUES (?1)", [&_to_str(GLOBAL_DATA, offset, offset + 8)]));
                    offset += 8;
                }
                1 => {
                    let sp = conn.savepoint();
                    let mut sp = _unwrap_result(sp);
                    _unwrap_result(sp.rollback());
                }
                2 => {
                    let mut stmt = _unwrap_result(conn.prepare("SELECT data FROM test WHERE id = ?"));
                    let mut rows = _unwrap_result(stmt.query([_to_i32(GLOBAL_DATA, offset)]));
                    offset += 4;
                    while let Some(row) = _unwrap_result(rows.next()) {
                        let val: ValueRef = row.get_ref(0).unwrap();
                        println!("{:?}", val);
                    }
                }
                3 => {
                    let mut batch = Batch::new(&conn, "INSERT INTO test (data) VALUES (?1)");
                    while let Some(mut stmt) = _unwrap_result(batch.next()) {
                        _unwrap_result(stmt.execute([_to_str(GLOBAL_DATA, offset, offset + 4)]));
                        offset += 4;
                    }
                }
                4 => {
                    let handle = conn.get_interrupt_handle();
                    handle.interrupt();
                }
                _ => {
                    let mut tx = _unwrap_result(conn.transaction_with_behavior(TransactionBehavior::Deferred));
                    let mut sp = _unwrap_result(tx.savepoint_with_name("mysavepoint"));
                    _unwrap_result(sp.execute("UPDATE test SET data = ?1 WHERE id = ?2", 
                        params![_to_str(GLOBAL_DATA, offset, offset + 8), _to_i32(GLOBAL_DATA, offset + 8)]));
                    offset += 12;
                }
            }
        }
        
        conn.savepoint().unwrap();
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