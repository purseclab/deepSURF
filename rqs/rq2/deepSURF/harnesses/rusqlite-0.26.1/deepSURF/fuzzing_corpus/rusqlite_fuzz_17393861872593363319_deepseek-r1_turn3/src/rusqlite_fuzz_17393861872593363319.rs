#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use rusqlite::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 256 {return;}
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let flags_selector = _to_u8(GLOBAL_DATA, 0) % 3;
        let mut conn = match flags_selector {
            0 => _unwrap_result(Connection::open_in_memory()),
            1 => _unwrap_result(Connection::open_in_memory_with_flags(OpenFlags::empty())),
            _ => _unwrap_result(Connection::open_in_memory_with_flags(OpenFlags::all())),
        };

        _unwrap_result(conn.execute_batch("CREATE TABLE IF NOT EXISTS t(x INTEGER);"));

        let tx_behavior = if _to_bool(GLOBAL_DATA, 1) {
            TransactionBehavior::Deferred
        } else {
            TransactionBehavior::Immediate
        };

        let mut tx = _unwrap_result(conn.transaction_with_behavior(tx_behavior));

        let ops_count = _to_u8(GLOBAL_DATA, 2) % 5;
        for i in 0..ops_count {
            let mut stmt = _unwrap_result(tx.prepare("INSERT INTO t VALUES (?1)"));
            let val = _to_i32(GLOBAL_DATA, 3 + i as usize *4);
            _unwrap_result(stmt.execute(params![val]));
        }

        let sp_count = _to_u8(GLOBAL_DATA, 23) % 3 + 1;
        for _ in 0..sp_count {
            let initial_name_len = _to_u8(GLOBAL_DATA, 24) % 16 + 1;
            let initial_str = _to_str(GLOBAL_DATA, 25, 25 + initial_name_len as usize).to_string();
            let custom_impl_num = _to_usize(GLOBAL_DATA, 0);
            let custom_impl_inst_num = initial_str.len();
            let selector = (custom_impl_num + custom_impl_inst_num) % 3;
            if selector == 0 {
                panic!("INTENTIONAL PANIC!");
            }
            let selected_data = match selector {
                1 => global_data.first_half,
                _ => global_data.second_half,
            };
            let mut t_4 = _to_u8(selected_data, 8) % 17;
            let t_5 = _to_str(selected_data, 9, 9 + t_4 as usize);
            let sp_name = String::from(t_5);

            let mut sp = _unwrap_result(tx.savepoint_with_name(sp_name));

            {
                let mut query = _unwrap_result(sp.prepare("SELECT x FROM t"));
                let mut rows = _unwrap_result(query.query([]));
                while let Some(row) = _unwrap_result(rows.next()) {
                    let x: i32 = row.get_unwrap(0);
                    println!("Value: {}", x);
                }
            }

            if _to_bool(GLOBAL_DATA, 60) {
                _unwrap_result(sp.rollback());
            } else {
                _unwrap_result(sp.commit());
            }
        }

        let initial_final_str = _to_str(GLOBAL_DATA, 80, 80 + 16).to_string();
        let custom_impl_num = _to_usize(GLOBAL_DATA, 0);
        let custom_impl_inst_num = initial_final_str.len();
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0 {
            panic!("INTENTIONAL PANIC!");
        }
        let selected_data = match selector {
            1 => global_data.first_half,
            _ => global_data.second_half,
        };
        let mut t_4 = _to_u8(selected_data, 8) % 17;
        let t_5 = _to_str(selected_data, 9, 9 + t_4 as usize);
        let final_sp_name = String::from(t_5);
        tx.savepoint_with_name(final_sp_name).ok();
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