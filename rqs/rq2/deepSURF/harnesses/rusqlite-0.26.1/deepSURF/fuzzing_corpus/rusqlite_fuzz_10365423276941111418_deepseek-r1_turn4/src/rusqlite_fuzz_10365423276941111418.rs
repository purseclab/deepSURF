#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use rusqlite::*;
use rusqlite::types::ValueRef;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

struct CustomType1(String);
struct CustomType2(String);
struct CustomType3(String);
struct CustomType0(String);

impl std::iter::Iterator for CustomType2 {
    type Item = OpenFlags;
    
    fn next(&mut self) -> Option<Self::Item> {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 0);
        let custom_impl_inst_num = self.0.len();
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0 {
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector {
            1 => global_data.first_half,
            _ => global_data.second_half,
        };
        let flag_bits = _to_u32(GLOBAL_DATA, custom_impl_inst_num * 4).wrapping_sub(custom_impl_inst_num as u32) as i32;
        Some(OpenFlags::from_bits_truncate(flag_bits))
    }
}

impl std::iter::IntoIterator for CustomType0 {
    type Item = OpenFlags;
    type IntoIter = CustomType2;
    
    fn into_iter(self) -> Self::IntoIter {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 25);
        let custom_impl_inst_num = self.0.len();
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0 {
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector {
            1 => global_data.first_half,
            _ => global_data.second_half,
        };
        let t_10 = CustomType2(_to_str(GLOBAL_DATA, 34, 34 + (custom_impl_num % 16)).to_string());
        t_10
    }
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 350 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        
        let op_count = _to_usize(GLOBAL_DATA, 0) % 8;
        for i in 0..op_count {
            let op_selector = _to_u8(GLOBAL_DATA, i * 4) % 6;
            
            match op_selector {
                0 => {
                    let flags_selector = _to_u8(GLOBAL_DATA, i * 4 + 1) % 3;
                    let mut flags = match flags_selector {
                        0 => OpenFlags::all(),
                        1 => OpenFlags::from_bits(_to_u32(GLOBAL_DATA, i * 4 + 2).try_into().unwrap()).unwrap_or(OpenFlags::empty()),
                        _ => OpenFlags::from_bits_truncate(_to_u32(GLOBAL_DATA, i * 4 + 2).try_into().unwrap()),
                    };
                    let custom_data = CustomType0(_to_str(GLOBAL_DATA, i * 4 + 7, i * 4 + 7 + (i % 17) as usize).to_string());
                    flags.extend(custom_data);
                    println!("{:?}", flags);
                },
                1 => {
                    let mut conn = Connection::open_in_memory_with_flags(OpenFlags::all()).unwrap();
                    let tx = conn.transaction().unwrap();
                    let _ = tx.execute("CREATE TABLE test (id INTEGER PRIMARY KEY)", []);
                    tx.commit().unwrap();
                    let _ = conn.execute("INSERT INTO test (id) VALUES (?1)", [1i32]);
                },
                2 => {
                    let conn = Connection::open_in_memory().unwrap();
                    let mut stmt = conn.prepare("SELECT * FROM sqlite_master").unwrap();
                    let mut rows = stmt.query([]).unwrap();
                    while let Some(row) = rows.next().unwrap() {
                        let _val: ValueRef = row.get_ref(0).unwrap();
                        println!("{:?}", _val);
                    }
                },
                3 => {
                    let mut db = Connection::open_with_flags(
                        ":memory:",
                        OpenFlags::from_bits(_to_u32(GLOBAL_DATA, i * 4 + 2).try_into().unwrap()).unwrap()
                    ).unwrap();
                    {
                        let mut stmt = db.prepare("PRAGMA schema_version").unwrap();
                        let _: i32 = stmt.query_row([], |r| r.get(0)).unwrap();
                    }
                    let _handle = db.get_interrupt_handle();
                    
                    let mut tx = db.transaction().unwrap();
                    tx.execute_batch("BEGIN; COMMIT;").unwrap();
                    let mut sp = tx.savepoint().unwrap();
                    sp.rollback().unwrap();
                },
                4 => {
                    let conn = Connection::open_in_memory().unwrap();
                    let mut batch = Batch::new(&conn, "SELECT ?1, ?2");
                    let _ = batch.next();
                    let conn2 = Connection::open_in_memory().unwrap();
                    let _ = conn2.execute("CREATE TABLE temp_table (data TEXT)", []);
                },
                5 => {
                    let mut conn = Connection::open_in_memory().unwrap();
                    let mut sp = conn.savepoint_with_name("test_savepoint").unwrap();
                    sp.commit().unwrap();
                },
                _ => panic!("INTENTIONAL PANIC!"),
            }
        }
        
        let mut final_flags = OpenFlags::from_bits(_to_u32(GLOBAL_DATA, 300).try_into().unwrap()).unwrap_or_default();
        let custom_data = CustomType0(_to_str(GLOBAL_DATA, 304, 320).to_string());
        final_flags.extend(custom_data);
        let _ = Connection::open_in_memory().unwrap().execute("PRAGMA quick_check", []);
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