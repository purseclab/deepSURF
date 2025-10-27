#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use rusqlite::*;
use global_data::*;
use std::str::FromStr;

struct CustomType0(String);

fn _custom_fn0(_: &rusqlite::Row) -> Result<CustomType0, rusqlite::Error> {
    let global_data = get_global_data();
    let GLOBAL_DATA = global_data.first_half;
    let t_1295 = _to_u8(GLOBAL_DATA, 1618);
    if t_1295 % 2 == 0 {
        panic!("INTENTIONAL PANIC!");
    }
    let mut t_1296 = _to_u8(GLOBAL_DATA, 1619) % 17;
    let t_1297 = _to_str(GLOBAL_DATA, 1620, 1620 + t_1296 as usize);
    let t_1298 = String::from(t_1297);
    let t_1299 = CustomType0(t_1298);
    Ok(t_1299)
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 4096 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let mut conn = match _to_u8(GLOBAL_DATA, 0) % 3 {
            0 => _unwrap_result(Connection::open_in_memory()),
            1 => _unwrap_result(Connection::open_in_memory_with_flags(OpenFlags::empty())),
            _ => _unwrap_result(Connection::open_with_flags(":memory:", OpenFlags::default())),
        };

        let mut offset = 1;
        let op_count = _to_u8(GLOBAL_DATA, offset) % 7;
        offset += 1;

        for _ in 0..op_count {
            let op_type = _to_u8(GLOBAL_DATA, offset) % 5;
            offset += 1;

            match op_type {
                0 => {
                    let mut tx = _unwrap_result(conn.transaction());
                    {
                        let sp = _unwrap_result(tx.savepoint());
                        _unwrap_result(sp.execute_batch("CREATE TABLE IF NOT EXISTS t (a TEXT)"));
                    }
                    let mut batch = Batch::new(&tx, "INSERT INTO t VALUES (?1)");
                    while let Some(stmt) = _unwrap_result(batch.next()) {
                        let mut stmt = stmt;
                        stmt.execute(&[(":param", &format!("{}", _to_u8(GLOBAL_DATA, offset)))]).ok();
                        offset += 1;
                    }
                }
                1 => {
                    let mut stmt = _unwrap_result(conn.prepare("SELECT * FROM t"));
                    let mut rows = _unwrap_result(stmt.query([]));
                    while let Some(row) = _unwrap_result(rows.next()) {
                        let _: String = row.get(0).unwrap();
                    }
                }
                2 => {
                    let param_count = _to_u8(GLOBAL_DATA, offset) % 15;
                    offset += 1;
                    let mut param_names = Vec::new();
                    let mut param_values = Vec::new();
                    for _ in 0..param_count {
                        let s = format!("p{}", _to_u8(GLOBAL_DATA, offset));
                        param_names.push(s);
                        let val = _to_u8(GLOBAL_DATA, offset + 1) as i32;
                        param_values.push(val);
                        offset += 2;
                    }
                    let params: Vec<(&str, &dyn ToSql)> = param_names
                        .iter()
                        .zip(param_values.iter())
                        .map(|(name, val)| (name.as_str(), val as &dyn ToSql))
                        .collect();

                    let mut stmt = conn.prepare("SELECT * FROM t WHERE a = :param").unwrap();
                    stmt.query_row_named(&params, |row| {
                        println!("{:?}", row.get_ref(0));
                        _custom_fn0(row)
                    }).ok();
                }
                3 => {
                    let tx = _unwrap_result(conn.transaction());
                    tx.execute_batch("DELETE FROM t WHERE a = 'test'").ok();
                    _unwrap_result(tx.commit());
                }
                4 => {
                    let mut stmt = _unwrap_result(conn.prepare_cached("SELECT COUNT(*) FROM t"));
                    let count: i32 = _unwrap_result(stmt.query_row([], |r| r.get(0)));
                    println!("Count: {}", count);
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