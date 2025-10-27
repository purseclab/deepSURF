#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use rusqlite::*;
use global_data::*;
use std::str::FromStr;

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 100 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let flags_byte = _to_u8(GLOBAL_DATA, 0);
        let open_flags = match flags_byte % 4 {
            0 => OpenFlags::empty(),
            1 => OpenFlags::SQLITE_OPEN_READ_WRITE,
            2 => OpenFlags::SQLITE_OPEN_FULL_MUTEX,
            _ => OpenFlags::all(),
        };

        let mut conn = _unwrap_result(Connection::open_in_memory_with_flags(open_flags));
        let handle = conn.get_interrupt_handle();
        let mut tx = _unwrap_result(conn.transaction());

        let num_ops = _to_u8(GLOBAL_DATA, 1) % 5;
        let mut offset = 2;

        for _ in 0..num_ops {
            if offset >= GLOBAL_DATA.len() { break; }
            let sql_len = (_to_u8(GLOBAL_DATA, offset) % 50) as usize;
            offset += 1;
            let sql = _to_str(GLOBAL_DATA, offset, offset + sql_len);
            offset += sql_len;

            {
                let mut stmt = _unwrap_result(tx.prepare(sql));
                
                let param_count = _to_u8(GLOBAL_DATA, offset) % 3;
                offset += 1;
                let mut params = Vec::new();
                for _ in 0..param_count {
                    if offset >= GLOBAL_DATA.len() { break; }
                    let param_type = _to_u8(GLOBAL_DATA, offset) % 4;
                    offset += 1;
                    let param_val = match param_type {
                        0 => {
                            let v = _to_i32(GLOBAL_DATA, offset);
                            offset += 4;
                            v as i64
                        }
                        1 => {
                            let v = _to_bool(GLOBAL_DATA, offset);
                            offset += 1;
                            v as i64
                        }
                        2 => 0i64,
                        _ => {
                            let v = _to_f64(GLOBAL_DATA, offset);
                            offset += 8;
                            v as i64
                        }
                    };
                    params.push(param_val);
                }

                let _ = _unwrap_result(stmt.execute(params_from_iter(params.iter().map(|p| p as &dyn ToSql))));
                handle.interrupt();

                let mut rows = _unwrap_result(stmt.query(params_from_iter(params.iter().map(|p| p as &dyn ToSql))));
                while let Ok(Some(row)) = rows.next() {
                    let _: i32 = _unwrap_result(row.get(0));
                    let val_ref = _unwrap_result(row.get_ref(0));
                    println!("{:?}", val_ref.as_i64().unwrap());
                }
            }

            let mut savepoint = _unwrap_result(tx.savepoint());
            _unwrap_result(savepoint.commit());
        }

        if offset < GLOBAL_DATA.len() && _to_u8(GLOBAL_DATA, offset) % 2 == 0 {
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