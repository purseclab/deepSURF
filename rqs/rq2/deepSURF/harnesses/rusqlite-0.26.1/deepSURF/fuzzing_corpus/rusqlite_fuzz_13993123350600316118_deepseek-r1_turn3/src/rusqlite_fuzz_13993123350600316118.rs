#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use rusqlite::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

fn _custom_fn0(_: &rusqlite::Row) -> Result<(), rusqlite::Error> {
    let global_data = get_global_data();
    let GLOBAL_DATA = global_data.first_half;
    let t_12 = _to_u8(GLOBAL_DATA, 46);
    if t_12 % 2 == 0 {
        panic!("INTENTIONAL PANIC!");
    }
    Ok(())
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 500 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let mut conn = match _to_u8(GLOBAL_DATA, 0) % 4 {
            0 => _unwrap_result(Connection::open_in_memory()),
            1 => _unwrap_result(Connection::open_with_flags(":memory:", OpenFlags::from_bits_truncate(_to_i32(GLOBAL_DATA, 1)))),
            2 => _unwrap_result(Connection::open_in_memory_with_flags(OpenFlags::from_bits_truncate(_to_i32(GLOBAL_DATA, 5)))),
            _ => _unwrap_result(Connection::open("test.db")),
        };

        {
            let mut batch = conn.prepare("CREATE TABLE IF NOT EXISTS t (id INTEGER PRIMARY KEY, data BLOB);").unwrap();
            _unwrap_result(batch.execute([]));
        }

        let ops_count = _to_u8(GLOBAL_DATA, 9) % 10;
        for i in 0..ops_count {
            let op_type = _to_u8(GLOBAL_DATA, 10 + i as usize) % 6;
            match op_type {
                0 => {
                    let mut tx = _unwrap_result(conn.transaction());
                    let mut sp = _unwrap_result(tx.savepoint());
                    _unwrap_result(sp.execute("INSERT INTO t (data) VALUES (?1)", params![_to_str(GLOBAL_DATA, 20, 30)]));
                }
                1 => {
                    let mut stmt = conn.prepare("SELECT * FROM t").unwrap();
                    let _: i64 = _unwrap_result(stmt.query_row([], |row| row.get(0)));
                }
                2 => {
                    let pragma_db = match _to_u8(GLOBAL_DATA, 50) % 3 {
                        0 => Some(DatabaseName::Main),
                        1 => Some(DatabaseName::Temp),
                        _ => None,
                    };
                    conn.pragma_query(pragma_db, "journal_mode", |r| {
                        println!("{:?}", r.get_ref(0).unwrap());
                        Ok(())
                    }).unwrap();
                }
                3 => {
                    let mut stmt = conn.prepare("DELETE FROM t WHERE id = ?").unwrap();
                    _unwrap_result(stmt.execute(params![_to_i32(GLOBAL_DATA, 60)]));
                }
                4 => {
                    let mut sp = _unwrap_result(conn.savepoint());
                    sp.execute("UPDATE t SET data = 'modified'", []).unwrap();
                }
                _ => {
                    let mut tx = _unwrap_result(conn.transaction_with_behavior(TransactionBehavior::Deferred));
                    tx.query_row("SELECT count(*) FROM t", [], |r| {
                        println!("COUNT: {:?}", r.get_ref_unwrap(0));
                        Ok(())
                    }).unwrap();
                }
            }
        }

        let t_5 = _to_usize(GLOBAL_DATA, 100) % 65;
        let db_name = match t_5 % 3 {
            0 => None,
            1 => Some(DatabaseName::Main),
            _ => {
                let len = _to_u8(GLOBAL_DATA, 104) % 65;
                Some(DatabaseName::Attached(_to_str(GLOBAL_DATA, 105, 105 + len as usize)))
            }
        };

        let pragma_str_len = _to_u8(GLOBAL_DATA, 200) % 65;
        let pragma_str = _to_str(GLOBAL_DATA, 201, 201 + pragma_str_len as usize);

        conn.pragma_query(db_name, pragma_str, _custom_fn0).ok();
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