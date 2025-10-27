#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use rusqlite::*;
use rusqlite::types::Value;
use global_data::*;

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 4096 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let op_count = _to_usize(GLOBAL_DATA, 0) % 16;
        let mut conn = _unwrap_result(Connection::open_in_memory());
        let mut tx = _unwrap_result(conn.transaction());

        for i in 0..op_count {
            let op_selector = _to_usize(GLOBAL_DATA, i * 4) % 8;
            match op_selector {
                0 => {
                    let sql = _to_str(GLOBAL_DATA, 100 + i * 32, 132 + i * 32);
                    let _ = tx.execute(sql, params_from_iter::<Vec<Value>>(vec![]));
                }
                1 => {
                    let mut batch = Batch::new(&tx, _to_str(GLOBAL_DATA, 200 + i * 16, 216 + i * 16));
                    let _ = _unwrap_result(batch.next());
                }
                2 => {
                    let mut savepoint = _unwrap_result(tx.savepoint());
                    let _ = savepoint.rollback();
                }
                3 => {
                    let mut stmt = _unwrap_result(tx.prepare(_to_str(GLOBAL_DATA, 300 + i * 16, 316 + i * 16)));
                    let mut rows = _unwrap_result(stmt.query([]));
                    while let Ok(Some(row)) = rows.next() {
                        let _val: i32 = row.get(0).unwrap();
                    }
                }
                4 => {
                    let mut params = Vec::with_capacity(32);
                    for j in 0..32 {
                        params.push(_to_i32(GLOBAL_DATA, 400 + j * 4));
                    }
                    tx.execute("INSERT INTO test VALUES (?1)", params_from_iter(params)).ok();
                }
                5 => {
                    let flags = OpenFlags::from_bits_truncate(_to_i32(GLOBAL_DATA, 500 + i * 4));
                    let _ = Connection::open_in_memory_with_flags(flags);
                }
                6 => {
                    let mut stmt = _unwrap_result(tx.prepare("SELECT * FROM test"));
                    for j in 0..32 {
                        let val = _to_str(GLOBAL_DATA, 600 + j * 16, 616 + j * 16);
                        stmt.raw_bind_parameter(j + 1, &Value::Text(val.to_string())).ok();
                    }
                }
                _ => {
                    let mut batch = Batch::new(&tx, "SELECT ?1");
                    let mut stmt = _unwrap_option(_unwrap_result(batch.next()));
                    for j in 0..32 {
                        stmt.raw_bind_parameter(j + 1, &Value::Null).ok();
                    }
                    println!("{}", stmt.column_count());
                }
            }
        }

        {
            let mut batch = Batch::new(&tx, "UPDATE test SET col=?1");
            let mut stmt = _unwrap_option(_unwrap_result(batch.next()));
            for j in 0..32 {
                stmt.raw_bind_parameter(j + 1, &_to_i32(GLOBAL_DATA, 1000 + j * 4)).ok();
            }
        }
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