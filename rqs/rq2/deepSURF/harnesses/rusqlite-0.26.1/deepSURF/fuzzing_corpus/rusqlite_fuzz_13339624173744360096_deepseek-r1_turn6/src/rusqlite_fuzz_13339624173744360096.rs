#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use rusqlite::*;
use global_data::*;
use std::str::FromStr;

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 306 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let flags_choice = _to_u8(GLOBAL_DATA, 0);
        let flags = if flags_choice % 2 == 0 {
            OpenFlags::empty()
        } else {
            OpenFlags::all()
        };

        let mut conn = _unwrap_result(Connection::open_in_memory_with_flags(flags));

        let ops_count = _to_u8(GLOBAL_DATA, 1) % 8 + 2;
        let mut base_idx = 2;

        for _ in 0..ops_count {
            let op_type = _to_u8(GLOBAL_DATA, base_idx) % 6;
            base_idx += 1;

            match op_type {
                0 => {
                    let stmt_len = _to_u8(GLOBAL_DATA, base_idx) % 64 + 1;
                    base_idx += 1;
                    let sql = _to_str(GLOBAL_DATA, base_idx as usize, (base_idx + stmt_len as usize) as usize);
                    base_idx += usize::from(stmt_len);
                    let stmt = _unwrap_result(conn.prepare(sql));
                    println!("{:?}", stmt.parameter_count());
                }
                1 => {
                    let param_len = _to_u8(GLOBAL_DATA, base_idx) % 32;
                    base_idx += 1;
                    let param_name = _to_str(GLOBAL_DATA, base_idx as usize, (base_idx + param_len as usize) as usize);
                    base_idx += usize::from(param_len);
                    let mut cached_stmt = _unwrap_result(conn.prepare_cached("SELECT ?"));
                    let idx = cached_stmt.parameter_index(param_name);
                    println!("{:?}", idx);
                }
                2 => {
                    let batch_sql = _to_str(GLOBAL_DATA, base_idx as usize, (base_idx + 16) as usize);
                    base_idx += 16;
                    let mut batch = Batch::new(&conn, batch_sql);
                    while let Some(mut stmt) = _unwrap_result(batch.next()) {
                        _unwrap_result(stmt.execute([]));
                    }
                }
                3 => {
                    let name_len = _to_u8(GLOBAL_DATA, base_idx) % 32;
                    base_idx += 1;
                    let sp_name = _to_str(GLOBAL_DATA, base_idx as usize, (base_idx + name_len as usize) as usize);
                    base_idx += usize::from(name_len);
                    let mut savepoint = _unwrap_result(conn.savepoint_with_name(sp_name));
                    _unwrap_result(savepoint.rollback());
                }
                4 => {
                    let mut stmt = _unwrap_result(conn.prepare("SELECT 1"));
                    let mut rows = _unwrap_result(stmt.query([]));
                    while let Some(row) = _unwrap_result(rows.next()) {
                        println!("{:?}", row.get_ref_unwrap(0));
                    }
                }
                5 => {
                    let param_len = _to_u8(GLOBAL_DATA, base_idx) % 32;
                    base_idx += 1;
                    let param_name = _to_str(GLOBAL_DATA, base_idx as usize, (base_idx + param_len as usize) as usize);
                    base_idx += usize::from(param_len);
                    let mut stmt = _unwrap_result(conn.prepare("INSERT INTO test VALUES (:name)"));
                    let idx = stmt.parameter_index(param_name);
                    println!("{:?}", idx);
                }
                _ => unreachable!()
            }
        }

        let mut sp = _unwrap_result(conn.savepoint_with_name("fuzz_sp"));
        _unwrap_result(sp.commit());

        let mut final_stmt = _unwrap_result(conn.prepare("SELECT name FROM sqlite_master"));
        let param_len = _to_u8(GLOBAL_DATA, base_idx) % 32;
        let param_name = _to_str(GLOBAL_DATA, base_idx as usize, (base_idx + param_len as usize) as usize);
        final_stmt.parameter_index(param_name);
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