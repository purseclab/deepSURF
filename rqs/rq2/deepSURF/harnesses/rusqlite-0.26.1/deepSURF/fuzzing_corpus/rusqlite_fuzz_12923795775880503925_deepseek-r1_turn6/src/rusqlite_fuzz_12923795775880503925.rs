#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use rusqlite::*;
use global_data::*;
use std::str::FromStr;

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 512 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        
        let flags_selector = _to_u8(GLOBAL_DATA, 0) % 4;
        let flags = match flags_selector {
            0 => OpenFlags::empty(),
            1 => OpenFlags::all(),
            2 => OpenFlags::from_bits(_to_i32(GLOBAL_DATA, 1)).unwrap_or(OpenFlags::empty()),
            _ => OpenFlags::from_bits_truncate(_to_i32(GLOBAL_DATA, 1)),
        };

        let path_len = _to_u8(GLOBAL_DATA, 5) % 32;
        let vfs_len = _to_u8(GLOBAL_DATA, 6) % 32;
        let path_str = _to_str(GLOBAL_DATA, 7, 7 + path_len as usize);
        let vfs_str = _to_str(GLOBAL_DATA, 40, 40 + vfs_len as usize);
        
        let mut conn = match Connection::open_with_flags_and_vfs(path_str, flags, vfs_str) {
            Ok(c) => c,
            Err(_) => return,
        };

        let ops_count = _to_u8(GLOBAL_DATA, 100) % 65;
        for i in 0..ops_count {
            let op_type = _to_u8(&global_data.second_half, i as usize) % 7;
            
            match op_type {
                0 => {
                    let _ = conn.execute(
                        "CREATE TABLE IF NOT EXISTS fuzz (id INTEGER PRIMARY KEY, data BLOB)",
                        [],
                    );
                }
                1 => {
                    let mut stmt = conn.prepare("INSERT INTO fuzz (data) VALUES (?)").unwrap();
                    let data_len = _to_u8(GLOBAL_DATA, 150 + i as usize) % 64;
                    let data = _to_str(GLOBAL_DATA, 200 + i as usize, 200 + i as usize + data_len as usize);
                    let _ = stmt.execute([data]);
                }
                2 => {
                    let mut stmt = conn.prepare("SELECT data FROM fuzz WHERE id = ?").unwrap();
                    let id = _to_i64(GLOBAL_DATA, 300 + i as usize);
                    let _row: Result<String, _> = stmt.query_row([id], |r| r.get(0));
                }
                3 => {
                    let mut tx = conn.transaction().unwrap();
                    let _ = tx.execute("DELETE FROM fuzz WHERE id = 1", []);
                    {
                        let mut sp = tx.savepoint().unwrap();
                        let _ = sp.execute("UPDATE fuzz SET data = 'x'", []);
                        if _to_bool(GLOBAL_DATA, 400 + i as usize) {
                            sp.commit().unwrap();
                        } else {
                            sp.rollback().unwrap();
                        }
                    }
                    tx.commit().unwrap();
                }
                4 => {
                    let mut batch = Batch::new(&conn, "SELECT name FROM sqlite_master");
                    while let Some(mut stmt) = batch.next().unwrap() {
                        let _: Result<String, _> = stmt.query_row([], |r| r.get(0));
                    }
                }
                5 => {
                    conn.pragma_update(None::<DatabaseName>, "journal_mode", "WAL").unwrap();
                }
                _ => {
                    let handle = conn.get_interrupt_handle();
                    handle.interrupt();
                    println!("Last insert rowid: {}", conn.last_insert_rowid());
                }
            };
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