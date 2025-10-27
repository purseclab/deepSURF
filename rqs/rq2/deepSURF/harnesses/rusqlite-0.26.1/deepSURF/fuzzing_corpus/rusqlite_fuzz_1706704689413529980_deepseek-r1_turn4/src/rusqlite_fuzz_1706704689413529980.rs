#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use rusqlite::*;
use rusqlite::types::{ValueRef, Value};
use rusqlite::config::DbConfig;
use global_data::*;
use std::str::FromStr;

fn main(){
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 4096 {return;}
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        
        let connection_type = _to_u8(GLOBAL_DATA, 0) % 5;
        let mut conn = match connection_type {
            0 => Connection::open_in_memory(),
            1 => {
                let mut flags = OpenFlags::empty();
                flags.insert(OpenFlags::SQLITE_OPEN_READ_WRITE);
                Connection::open_with_flags(":memory:", flags)
            }
            2 => Connection::open_in_memory_with_flags(OpenFlags::from_bits_truncate(_to_i32(GLOBAL_DATA, 1))),
            3 => Connection::open("fuzz.db"),
            _ => Connection::open_in_memory_with_flags_and_vfs(OpenFlags::default(), "")
        }.unwrap();

        let batch_iterations = _to_u8(GLOBAL_DATA, 3) % 5;
        for _ in 0..batch_iterations {
            let mut stmt = conn.prepare("SELECT ?1, ?2").unwrap();
            let param1 = _to_i64(GLOBAL_DATA, 4);
            let param2 = _to_str(GLOBAL_DATA, 5, 10);
            stmt.raw_bind_parameter(1, param1).unwrap();
            stmt.raw_bind_parameter(2, param2).unwrap();
            let mut rows = stmt.raw_query();
            let _ = rows.next();
        }

        let tx_behavior = match _to_u8(GLOBAL_DATA, 2) % 3 {
            0 => TransactionBehavior::Deferred,
            1 => TransactionBehavior::Immediate,
            _ => TransactionBehavior::Exclusive
        };
        let mut transaction = conn.transaction_with_behavior(tx_behavior).unwrap();
        
        let mut ops_count = _to_u8(GLOBAL_DATA, 100) % 15;
        for i in 0..ops_count {
            let op_selector = _to_u8(GLOBAL_DATA, 101 + i as usize) % 10;
            match op_selector {
                0 => transaction.execute_batch("CREATE TABLE IF NOT EXISTS t (id INTEGER PRIMARY KEY);").unwrap(),
                1 => {
                    let mut savepoint = transaction.savepoint().unwrap();
                    savepoint.execute("INSERT INTO t VALUES (?)", [_to_i64(GLOBAL_DATA, 205)]).unwrap();
                    savepoint.commit().unwrap();
                }
                2 => {
                    let flags = OpenFlags::from_iter([OpenFlags::SQLITE_OPEN_READ_ONLY]);
                    let _ = Connection::open_in_memory_with_flags(flags);
                }
                3 => {
                    let mut stmt = transaction.prepare("SELECT * FROM t").unwrap();
                    let _rows = stmt.query_map([], |row| Ok(row.get::<_, i64>(0))).unwrap();
                }
                4 => {
                    transaction.set_db_config(DbConfig::SQLITE_DBCONFIG_ENABLE_FKEY, true).unwrap();
                }
                5 => {
                    let handle = transaction.get_interrupt_handle();
                    handle.interrupt();
                }
                6 => {
                    let _ = transaction.execute("DELETE FROM t", []).unwrap();
                }
                7 => {
                    let _ = transaction.query_row("SELECT COUNT(*) FROM t", [], |row| row.get::<_, i64>(0)).unwrap();
                }
                8 => {
                    let param = _to_i32(GLOBAL_DATA, 305);
                    let _ = transaction.execute("INSERT INTO t VALUES (?)", [param]).unwrap();
                }
                _ => {
                    let _ = transaction.pragma_update(None::<DatabaseName>, "journal_mode", &_to_str(GLOBAL_DATA, 310, 315)).unwrap();
                }
            }
        }

        transaction.commit().unwrap();

        let mut named_params = vec![];
        for i in 0..5 {
            named_params.push((":param".to_string() + &i.to_string(), Value::Integer(_to_i64(GLOBAL_DATA, 200 + i as usize))));
        }

        let mut stmt = conn.prepare("SELECT * FROM t WHERE id > ?").unwrap();
        let mut rows = stmt.query([_to_i64(GLOBAL_DATA, 300)]).unwrap();
        while let Some(row) = rows.next().unwrap() {
            let val: i64 = row.get(0).unwrap();
            println!("Row: {}", val);
        }

        let blob_data = &GLOBAL_DATA[400..400 + 64];
        conn.execute("CREATE TABLE IF NOT EXISTS b (data BLOB)", []).unwrap();
        conn.execute("INSERT INTO b VALUES (?)", [blob_data]).unwrap();

        let mut cache_stmt = conn.prepare_cached("SELECT data FROM b WHERE length(data) > ?").unwrap();
        let _cached_rows = cache_stmt.query([_to_i32(GLOBAL_DATA, 350)]).unwrap();
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