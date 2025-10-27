#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use rusqlite::*;
use rusqlite::types::Value;
use global_data::*;

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 16384 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let mut conn = match _to_u8(GLOBAL_DATA, 0) % 3 {
            0 => Connection::open_in_memory(),
            1 => Connection::open_in_memory_with_flags(OpenFlags::empty()),
            _ => Connection::open_in_memory_with_flags_and_vfs(OpenFlags::all(), ""),
        }.unwrap();

        let mut tx = conn.transaction().unwrap();
        let mut should_commit = false;
        {
            let mut savepoint = tx.savepoint().unwrap();

            let ops_count = _to_u8(GLOBAL_DATA, 1) % 8;
            for i in 0..ops_count {
                let op_type = _to_u8(GLOBAL_DATA, 2 + i as usize) % 7;
                match op_type {
                    0 => {
                        savepoint.execute_batch("CREATE TABLE IF NOT EXISTS t(x INTEGER);").unwrap();
                    }
                    1 => {
                        let params = params_from_iter((0.._to_u8(GLOBAL_DATA, 10) % 5).map(|i| _to_u8(GLOBAL_DATA, 20 + i as usize)));
                        savepoint.execute("INSERT INTO t VALUES (?1)", params).unwrap();
                    }
                    2 => {
                        let mut stmt = savepoint.prepare("SELECT x FROM t WHERE x > ?").unwrap();
                        let params = params_from_iter(vec![Value::from(_to_u8(GLOBAL_DATA, 100))]);
                        let _rows = stmt.query_map(params, |row| Ok(row.get_unwrap::<_, u8>(0))).unwrap();
                    }
                    3 => {
                        savepoint.pragma_update(None, "cache_size", _to_u8(GLOBAL_DATA, 200)).unwrap();
                    }
                    4 => {
                        let handle = savepoint.get_interrupt_handle();
                        handle.interrupt();
                    }
                    5 => {
                        let name = _to_str(GLOBAL_DATA, 300, 310);
                        let mut sp = savepoint.savepoint_with_name(name).unwrap();
                        sp.execute_batch("DELETE FROM t").unwrap();
                        if _to_u8(GLOBAL_DATA, 400) % 2 == 0 {
                            sp.commit().unwrap();
                        } else {
                            sp.rollback().unwrap();
                        }
                    }
                    _ => {
                        let mut batch = Batch::new(&savepoint, "SELECT x FROM t WHERE x > ?");
                        while let Ok(Some(mut stmt)) = batch.next() {
                            let val = _to_u8(GLOBAL_DATA, 500);
                            let _rows = stmt.query_map([val], |row| Ok(row.get_unwrap::<_, u8>(0))).unwrap();
                        }
                    }
                }
            }

            let query = _to_str(GLOBAL_DATA, 1000, 1000 + (_to_u8(GLOBAL_DATA, 999) % 50) as usize);
            let params: Vec<Value> = (0.._to_u8(GLOBAL_DATA, 1500) % 10)
                .map(|i| Value::from(_to_u8(GLOBAL_DATA, 1501 + i as usize)))
                .collect();
            
            let param_names: Vec<String> = (0..params.len()).map(|i| format!(":p{}", i)).collect();
            let param_refs: Vec<(&str, &dyn ToSql)> = param_names.iter()
                .map(|name| name.as_str())
                .zip(params.iter().map(|v| v as &dyn ToSql))
                .collect();

            let mapper = |row: &Row| -> Result<String> {
                let val: u8 = row.get(0)?;
                Ok(val.to_string())
            };

            {
                let mut stmt = savepoint.prepare_cached(query).unwrap();
                let _mapped = stmt.query_map_named(&param_refs[..], mapper).unwrap();
                for row in _mapped {
                    let _val = row.unwrap();
                    println!("{:?}", _val);
                }
            }

            if _to_u8(GLOBAL_DATA, 2000) % 2 == 0 {
                savepoint.commit().unwrap();
                should_commit = true;
            } else {
                savepoint.rollback().unwrap();
            }
        }

        if should_commit {
            tx.commit().unwrap();
        } else {
            tx.rollback().unwrap();
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