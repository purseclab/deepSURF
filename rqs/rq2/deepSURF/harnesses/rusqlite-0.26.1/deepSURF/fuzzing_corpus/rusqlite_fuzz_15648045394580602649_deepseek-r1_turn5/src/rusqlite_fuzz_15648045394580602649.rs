#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use rusqlite::*;
use rusqlite::types::Value;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 512 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let flags_byte = _to_u8(GLOBAL_DATA, 0);
        let flags = match flags_byte % 4 {
            0 => OpenFlags::empty(),
            1 => OpenFlags::from_bits_truncate(_to_i32(GLOBAL_DATA, 1)),
            2 => OpenFlags::all(),
            _ => OpenFlags::from_iter(vec![OpenFlags::SQLITE_OPEN_READ_WRITE, OpenFlags::SQLITE_OPEN_CREATE]),
        };

        let mut conn = _unwrap_result(Connection::open_in_memory_with_flags(flags));

        let mut tx = _unwrap_result(conn.transaction_with_behavior(TransactionBehavior::Deferred));
        let mut sp = _unwrap_result(tx.savepoint_with_name(_to_str(GLOBAL_DATA, 2, 10)));

        let ops_count = _to_u8(GLOBAL_DATA, 10) % 8;
        for i in 0..ops_count {
            let op_type = _to_u8(GLOBAL_DATA, 11 + i as usize) % 7;
            match op_type {
                0 => {
                    let mut stmt = _unwrap_result(sp.prepare(_to_str(GLOBAL_DATA, 20 + i as usize * 10, 30 + i as usize * 10)));
                    let _ = _unwrap_result(stmt.execute([]));
                    let _ = stmt.finalize();
                },
                1 => {
                    let mut batch = Batch::new(&sp, _to_str(GLOBAL_DATA, 100 + i as usize * 20, 120 + i as usize * 20));
                    while let Ok(Some(mut inner_stmt)) = batch.next() {
                        let param = _to_i32(GLOBAL_DATA, 400 + i as usize * 4);
                        let _ = inner_stmt.execute([&param as &dyn ToSql]);
                    }
                },
                2 => {
                    let mut stmt = _unwrap_result(sp.prepare(_to_str(GLOBAL_DATA, 200, 250)));
                    let rows = _unwrap_result(stmt.query([]));
                    let _mapped = rows.mapped(|row| Ok(row.get_unwrap::<_, i32>(0)));
                },
                3 => {
                    let value_ref = sp.query_row(
                        "SELECT name FROM sqlite_master",
                        [],
                        |r| Ok(Value::from(r.get_ref_unwrap(0)))
                    ).unwrap();
                    println!("{:?}", value_ref);
                },
                4 => {
                    let mut stmt = _unwrap_result(sp.prepare_cached(_to_str(GLOBAL_DATA, 300, 350)));
                    let _ = _unwrap_result(stmt.insert([&_to_i32(GLOBAL_DATA, 400) as &dyn ToSql]));
                },
                5 => {
                    let _ = sp.execute_batch(_to_str(GLOBAL_DATA, 450, 480)).unwrap();
                },
                _ => {
                    let _ = _unwrap_result(sp.query_row(
                        "SELECT count(*) FROM sqlite_master",
                        [],
                        |r| Ok(r.get_unwrap::<_, i64>(0))
                    ));
                }
            }
        }

        let mut final_stmt = _unwrap_result(sp.prepare(_to_str(GLOBAL_DATA, 500, 512)));
        final_stmt.raw_query();
        final_stmt.finalize();

        let _ = _unwrap_result(sp.commit());
        let _ = tx.commit();
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