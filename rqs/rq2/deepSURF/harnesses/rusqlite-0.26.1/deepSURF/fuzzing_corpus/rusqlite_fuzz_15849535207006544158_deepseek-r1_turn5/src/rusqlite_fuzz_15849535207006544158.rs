#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use rusqlite::*;
use rusqlite::types::{ToSqlOutput, Value};
use global_data::*;

#[derive(Debug)]
struct CustomType0(String);

impl ToSql for CustomType0 {
    fn to_sql(&self) -> Result<ToSqlOutput, Error> {
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
        let t_1 = _to_usize(GLOBAL_DATA, 8);
        let t_2 = match t_1 % 7 {
            0 => Value::Text(String::from(_to_str(GLOBAL_DATA, 16, 32))),
            1 => Value::Real(_to_f64(GLOBAL_DATA, 32)),
            2 => Value::Integer(_to_i64(GLOBAL_DATA, 40)),
            3 => Value::Blob((0..16).map(|i| _to_u8(GLOBAL_DATA, 48 + i)).collect()),
            4 => Value::Null,
            5 => Value::from(_to_bool(GLOBAL_DATA, 64)),
            _ => Value::from(_to_i32(GLOBAL_DATA, 65)),
        };
        Ok(ToSqlOutput::Owned(t_2))
    }
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 2048 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let flags = OpenFlags::from_bits_truncate(_to_i32(GLOBAL_DATA, 0));
        let vfs = _to_str(GLOBAL_DATA, 4, 68);
        let mut conn = _unwrap_result(Connection::open_in_memory_with_flags_and_vfs(flags, vfs));
        
        let mut tx = _unwrap_result(conn.transaction());
        let mut sp = _unwrap_result(tx.savepoint_with_name(_to_str(GLOBAL_DATA, 68, 100)));
        
        let mut operation_count = _to_usize(GLOBAL_DATA, 132) % 8;

        for _ in 0..operation_count {
            let op_type = _to_u8(GLOBAL_DATA, 140) % 4;
            match op_type {
                0 => {
                    let mut params = Vec::with_capacity(14);
                    for i in 0..14 {
                        let param_type = _to_u8(GLOBAL_DATA, 141 + i) % 6;
                        let param: Box<dyn ToSql> = match param_type {
                            0 => Box::new(CustomType0(String::from(_to_str(GLOBAL_DATA, 155 + i*20, 155 + i*20 + 16)))),
                            1 => Box::new(Value::Integer(_to_i64(GLOBAL_DATA, 315 + i*8))),
                            2 => Box::new(Value::Real(_to_f64(GLOBAL_DATA, 427 + i*8))),
                            3 => Box::new(Value::Blob((0..16).map(|j| _to_u8(GLOBAL_DATA, 501 + i*16 + j)).collect())),
                            4 => Box::new(Value::Text(String::from(_to_str(GLOBAL_DATA, 765 + i*20, 765 + i*20 + 18)))),
                            _ => Box::new(Value::Null),
                        };
                        params.push(param);
                    }
                    let mut stmt = _unwrap_result(sp.prepare(_to_str(GLOBAL_DATA, 1024, 1056)));
                    params.iter().enumerate().for_each(|(i, p)| {
                        stmt.raw_bind_parameter(i + 1, p.as_ref()).unwrap();
                    });
                    let _ = stmt.raw_execute();
                }
                1 => {
                    let mut stmt = _unwrap_result(sp.prepare(_to_str(GLOBAL_DATA, 140, 172)));
                    let mut rows = _unwrap_result(stmt.query_map(params![], |row| { 
                        println!("{:?}", row.get_ref_unwrap(0));
                        Ok(())
                    }));
                    while let Some(row) = rows.next().transpose().unwrap() {}
                }
                2 => {
                    let mut stmt = _unwrap_result(sp.prepare(_to_str(GLOBAL_DATA, 1056, 1088)));
                    let _ = stmt.insert(params![
                        Value::from(String::from(_to_str(GLOBAL_DATA, 1088, 1120))),
                        Value::from(_to_i64(GLOBAL_DATA, 1120)),
                        Value::from(_to_f64(GLOBAL_DATA, 1128))
                    ]);
                }
                _ => {
                    let mut stmt = _unwrap_result(sp.prepare(_to_str(GLOBAL_DATA, 1136, 1168)));
                    let mut param = String::new();
                    param.push_str(_to_str(GLOBAL_DATA, 1168, 1200));
                    let params = params![param];
                    let mut rows = _unwrap_result(stmt.query(&*params));
                    while let Some(_row) = rows.next().unwrap() {}
                }
            }
        }

        println!("Last insert: {}", sp.last_insert_rowid());
        _unwrap_result(sp.commit());
        _unwrap_result(tx.commit());

        let mut batch = Batch::new(&conn, _to_str(GLOBAL_DATA, 100, 132));
        let mut batch_stmt = _unwrap_result(batch.next());
        while let Some(mut stmt) = batch_stmt {
            let mut custom_params = (0..14).map(|i| {
                let s = String::from(_to_str(GLOBAL_DATA, 1200 + i*32, 1200 + i*32 + 24));
                CustomType0(s)
            }).collect::<Vec<_>>();
            let param_refs: Vec<_> = custom_params.iter().collect();
            let param_array: [_; 14] = param_refs.try_into().unwrap();
            param_array.__bind_in(&mut stmt).unwrap();
            batch_stmt = _unwrap_result(batch.next());
        }
    });
}

// Type conversion functions omitted for brevity as per instructions

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