#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use rusqlite::*;
use global_data::*;
use std::str::FromStr;

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 256 {
            return;
        }
        set_global_data(data);
        let global_data = get_global_data();
        let mut offset = 0;
        let op_count = _to_usize(global_data.first_half, offset) % 5 + 1;
        offset += 8;

        for _ in 0..op_count {
            let op_selector = _to_u8(global_data.first_half, offset) % 7;
            offset += 1;

            match op_selector {
                0 => {
                    let mut path_len = _to_u8(global_data.first_half, offset) % 65;
                    offset += 1;
                    let path = _to_str(global_data.first_half, offset, offset + path_len as usize);
                    offset += path_len as usize;
                    let mut conn = _unwrap_result(Connection::open_with_flags(path, OpenFlags::all()));
                    let tx = _unwrap_result(conn.transaction());
                    let _ = _unwrap_result(tx.execute_batch("CREATE TABLE test (x INTEGER)"));
                }
                1 => {
                    let flags = OpenFlags::from_bits_truncate(_to_i32(global_data.first_half, offset));
                    let conn = _unwrap_result(Connection::open_in_memory_with_flags(flags));
                    let mut stmt = _unwrap_result(conn.prepare("SELECT * FROM sqlite_master"));
                    let _rows = _unwrap_result(stmt.query([]));
                }
                2 => {
                    let conn = _unwrap_result(Connection::open_in_memory());
                    let mut batch = Batch::new(&conn, "SELECT 1; SELECT 2;");
                    while let Some(mut stmt) = _unwrap_result(batch.next()) {
                        let mut rows = _unwrap_result(stmt.query([]));
                        while let Some(row) = _unwrap_result(rows.next()) {
                            let _value: i32 = _unwrap_result(row.get(0));
                            println!("Row value: {:?}", _value);
                        }
                    }
                }
                3 => {
                    let mut vfs_len = _to_u8(global_data.first_half, offset) % 65;
                    offset += 1;
                    let vfs = _to_str(global_data.first_half, offset, offset + vfs_len as usize);
                    offset += vfs_len as usize;
                    let conn = _unwrap_result(Connection::open_with_flags_and_vfs(":memory:", OpenFlags::default(), vfs));
                    let _ = _unwrap_result(conn.execute_batch("PRAGMA journal_mode=WAL;"));
                }
                4 => {
                    let mut conn = _unwrap_result(Connection::open_in_memory());
                    let savepoint = _unwrap_result(conn.savepoint());
                    let _ = _unwrap_result(savepoint.execute_batch("CREATE TABLE test (id INTEGER PRIMARY KEY)"));
                    println!("{:?}", _unwrap_result(savepoint.commit()));
                }
                5 => {
                    let mut conn = _unwrap_result(Connection::open_in_memory());
                    let tx = _unwrap_result(conn.transaction_with_behavior(TransactionBehavior::Deferred));
                    {
                        let mut stmt = _unwrap_result(tx.prepare("INSERT INTO test VALUES (?)"));
                        let val = _to_i32(global_data.second_half, offset);
                        _unwrap_result(stmt.execute([val]));
                    }
                    let _ = _unwrap_result(tx.commit());
                }
                6 => {
                    let conn = _unwrap_result(Connection::open_in_memory());
                    let mut stmt = _unwrap_result(conn.prepare_cached("SELECT name FROM sqlite_master"));
                    let mut rows = _unwrap_result(stmt.query([]));
                    while let Some(row) = _unwrap_result(rows.next()) {
                        let _name: String = _unwrap_result(row.get(0));
                    }
                }
                _ => unreachable!(),
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