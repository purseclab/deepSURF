#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use rusqlite::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};
use rusqlite::types::Value;

struct CustomType2(String);
struct CustomType0(String);

fn _custom_fn0(_: &rusqlite::Row) -> Result<CustomType2, rusqlite::Error> {
    let global_data = get_global_data();
    let GLOBAL_DATA = global_data.first_half;
    let t_341 = _to_u8(GLOBAL_DATA, 1147);
    if t_341 % 2 == 0 {
        panic!("INTENTIONAL PANIC!");
    }
    let mut t_342 = _to_u8(GLOBAL_DATA, 1148) % 17;
    let t_343 = _to_str(GLOBAL_DATA, 1149, 1149 + t_342 as usize);
    let t_344 = String::from(t_343);
    let t_345 = CustomType2(t_344);
    Ok(t_345)
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 3500 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let mut conn = match _to_u8(GLOBAL_DATA, 0) % 4 {
            0 => Connection::open_in_memory(),
            1 => Connection::open_with_flags(":memory:", OpenFlags::empty()),
            2 => Connection::open_in_memory_with_flags(OpenFlags::all()),
            _ => Connection::open_in_memory_with_flags_and_vfs(OpenFlags::from_bits_truncate(_to_u32(GLOBAL_DATA, 1) as i32), "volatile")
        }.unwrap();

        let mut tx = conn.transaction().unwrap();
        let mut savepoint = tx.savepoint().unwrap();

        let ops_count = _to_u8(GLOBAL_DATA, 5) % 8;
        for i in 0..ops_count {
            let op_selector = _to_u8(GLOBAL_DATA, 6 + i as usize) % 6;
            match op_selector {
                0 => {
                    let mut stmt = savepoint.prepare("SELECT ?").unwrap();
                    let param: Value = Value::from(_to_u32(GLOBAL_DATA, 50 + i as usize));
                    stmt.execute(params![param]).unwrap();
                }
                1 => {
                    savepoint.execute_batch("CREATE TABLE test (id INTEGER PRIMARY KEY)").unwrap();
                }
                2 => {
                    let mut batch = savepoint.prepare("INSERT INTO test VALUES (?)").unwrap();
                    batch.execute(params![_to_i64(GLOBAL_DATA, 100 + i as usize)]).unwrap();
                }
                3 => {
                    let rows = savepoint.query_row("SELECT * FROM test", [], |row| {
                        Ok(row.get_unwrap::<_, i64>(0))
                    }).unwrap();
                    println!("{:?}", rows);
                }
                4 => {
                    let mut stmt = savepoint.prepare("SELECT * FROM test").unwrap();
                    let mut rows = stmt.query_map([], |row| {
                        row.get::<_, i64>(0)
                    }).unwrap();
                    let _ = rows.next();
                }
                _ => {
                    let mut stmt = savepoint.prepare("SELECT id FROM test").unwrap();
                    let mut named_rows = stmt.query_named(&[(":id", &_to_u32(GLOBAL_DATA, 200))]).unwrap();
                    let _ = named_rows.next();
                }
            }
        }

        {
            let mut main_stmt = savepoint.prepare("SELECT * FROM test").unwrap();
            let mut rows = main_stmt.raw_query();
            let mut mapped = rows.mapped(|row| _custom_fn0(row));
            
            let next_count = _to_u8(GLOBAL_DATA, 250) % 5;
            for _ in 0..next_count {
                let _ = mapped.next();
            }
        }

        savepoint.commit().unwrap();
        tx.commit().unwrap();
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