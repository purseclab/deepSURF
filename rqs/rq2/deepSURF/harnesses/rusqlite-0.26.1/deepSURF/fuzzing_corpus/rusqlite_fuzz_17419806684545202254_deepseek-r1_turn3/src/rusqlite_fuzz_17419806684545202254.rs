#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use rusqlite::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 100 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let flags_choice = _to_u8(GLOBAL_DATA, 0);
        let flags = match flags_choice % 3 {
            0 => OpenFlags::empty(),
            1 => OpenFlags::SQLITE_OPEN_READ_WRITE,
            _ => OpenFlags::SQLITE_OPEN_MEMORY,
        };
        let mut conn = _unwrap_result(Connection::open_in_memory_with_flags(flags));
        
        _ = _unwrap_result(conn.execute("CREATE TABLE t (id INTEGER PRIMARY KEY, val TEXT)", []));

        let tx = _unwrap_result(conn.transaction());
        let ops_count = _to_u8(GLOBAL_DATA, 1) % 20;
        let mut data_offset = 2;

        for _ in 0..ops_count {
            let op_type = _to_u8(GLOBAL_DATA, data_offset) % 6;
            data_offset += 1;

            match op_type {
                0 => {
                    let param_count = _to_u8(GLOBAL_DATA, data_offset) % 5;
                    data_offset += 1;
                    let mut params = vec![];
                    for _ in 0..param_count {
                        params.push(_to_usize(GLOBAL_DATA, data_offset) as i32);
                        data_offset += 8;
                    }
                    let mut stmt = tx.prepare("INSERT INTO t (val) VALUES (?1)").unwrap();
                    let val = format!("val{}", _to_usize(GLOBAL_DATA, data_offset));
                    data_offset += 8;
                    _unwrap_result(stmt.execute(params![&val]));
                },
                1 => {
                    let stmt = tx.prepare("SELECT val FROM t").unwrap();
                    let status = match _to_u8(GLOBAL_DATA, data_offset) % 7 {
                        0 => StatementStatus::FullscanStep, 1 => StatementStatus::Sort,
                        2 => StatementStatus::Run, 3 => StatementStatus::RePrepare,
                        4 => StatementStatus::MemUsed, 5 => StatementStatus::AutoIndex,
                        _ => StatementStatus::VmStep,
                    };
                    stmt.reset_status(status);
                    data_offset += 1;
                },
                2 => {
                    let mut stmt = tx.prepare("UPDATE t SET val = ?1 WHERE id = ?2").unwrap();
                    let id = _to_usize(GLOBAL_DATA, data_offset) as i32;
                    data_offset += 8;
                    let param = _to_u32(GLOBAL_DATA, data_offset) as i32;
                    data_offset += 4;
                    _unwrap_result(stmt.execute(params![param.to_string(), id]));
                },
                3 => {
                    let mut stmt = tx.prepare("SELECT id, val FROM t").unwrap();
                    let mut rows = _unwrap_result(stmt.query(params![]));
                    while let Some(row) = _unwrap_result(rows.next()) {
                        let _id: i32 = row.get(0).unwrap();
                        let val: String = row.get(1).unwrap();
                        println!("Row: {}={}", _id, val);
                    }
                },
                4 => {
                    let handle = tx.get_interrupt_handle();
                    handle.interrupt();
                },
                5 => {
                    let index = _to_usize(GLOBAL_DATA, data_offset);
                    data_offset += 8;
                    let stmt = tx.prepare("SELECT id, val FROM t").unwrap();
                    let name = stmt.column_name(index).unwrap();
                    println!("Column name: {}", name);
                },
                _ => unreachable!()
            }
        }

        if _to_u8(GLOBAL_DATA, data_offset + 8) % 2 == 0 {
            _unwrap_result(tx.commit());
        } else {
            _unwrap_result(tx.rollback());
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