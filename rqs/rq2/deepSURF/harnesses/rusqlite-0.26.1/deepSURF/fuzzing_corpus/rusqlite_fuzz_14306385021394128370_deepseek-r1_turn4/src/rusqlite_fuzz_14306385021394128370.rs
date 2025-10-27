#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use rusqlite::*;
use global_data::*;
use std::str::FromStr;

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 200 {
            return;
        }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        
        let op_count = _to_u8(GLOBAL_DATA, 0) % 5 + 1;
        let mut offset = 1;
        
        for _ in 0..op_count {
            let op_selector = _to_u8(GLOBAL_DATA, offset) % 7;
            offset = (offset + 1) % 128;

            match op_selector {
                0 => {
                    let path_len = _to_u8(GLOBAL_DATA, offset) % 65;
                    offset = (offset + 1) % 128;
                    let path_start = offset;
                    let path_end = path_start + path_len as usize;
                    offset = (offset + path_len as usize) % 128;
                    let t_6 = _to_str(GLOBAL_DATA, path_start, path_end);
                    let _ = Connection::open(t_6);
                }
                1 => {
                    let flags = OpenFlags::from_bits_truncate(_to_u8(GLOBAL_DATA, offset) as i32);
                    offset = (offset + 1) % 128;
                    let path_len = _to_u8(GLOBAL_DATA, offset) % 65;
                    offset = (offset + 1) % 128;
                    let path_start = offset;
                    let path_end = path_start + path_len as usize;
                    offset = (offset + path_len as usize) % 128;
                    let t_6 = _to_str(GLOBAL_DATA, path_start, path_end);
                    let _ = Connection::open_with_flags(t_6, flags);
                }
                2 => {
                    let _ = Connection::open_in_memory();
                }
                3 => {
                    if let Ok(conn) = Connection::open_in_memory() {
                        let sql_len = _to_u8(GLOBAL_DATA, offset) % 65;
                        offset = (offset + 1) % 128;
                        let sql_start = offset;
                        let sql_end = sql_start + sql_len as usize;
                        offset = (offset + sql_len as usize) % 128;
                        let sql = _to_str(GLOBAL_DATA, sql_start, sql_end);
                        let param = _to_u8(GLOBAL_DATA, offset);
                        offset = (offset + 1) % 128;
                        let _ = conn.execute(sql, [param as i64]);
                        let _ = conn.last_insert_rowid();
                    }
                }
                4 => {
                    if let Ok(mut conn) = Connection::open_in_memory() {
                        let _tx = conn.transaction().expect("Transaction failed");
                        let mut value = _to_u8(GLOBAL_DATA, offset) % 2;
                        offset = (offset + 1) % 128;
                        if value == 0 {
                            let _ = _tx.commit();
                        } else {
                            let _ = _tx.rollback();
                        }
                    }
                }
                5 => {
                    if let Ok(mut conn) = Connection::open_in_memory() {
                        let _savepoint = conn.savepoint().expect("Savepoint failed");
                        let sql_len = _to_u8(GLOBAL_DATA, offset) % 65;
                        offset = (offset + 1) % 128;
                        let sql_start = offset;
                        let sql_end = sql_start + sql_len as usize;
                        offset = (offset + sql_len as usize) % 128;
                        let sql = _to_str(GLOBAL_DATA, sql_start, sql_end);
                        let param = _to_u8(GLOBAL_DATA, offset);
                        offset = (offset + 1) % 128;
                        let _ = _savepoint.execute(sql, [param as i64]);
                    }
                }
                6 => {
                    if let Ok(conn) = Connection::open_in_memory() {
                        let sql_len = _to_u8(GLOBAL_DATA, offset) % 65;
                        offset = (offset + 1) % 128;
                        let sql_start = offset;
                        let sql_end = sql_start + sql_len as usize;
                        offset = (offset + sql_len as usize) % 128;
                        let sql = _to_str(GLOBAL_DATA, sql_start, sql_end);
                        if let Ok(mut stmt) = conn.prepare(sql) {
                            let param = _to_u8(GLOBAL_DATA, offset);
                            offset = (offset + 1) % 128;
                            let _ = stmt.query([param as i64]).map(|mut rows| {
                                while let Ok(Some(row)) = rows.next() {
                                    let _: i32 = row.get(0).unwrap_or(0);
                                }
                            });
                        }
                    }
                }
                _ => {}
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