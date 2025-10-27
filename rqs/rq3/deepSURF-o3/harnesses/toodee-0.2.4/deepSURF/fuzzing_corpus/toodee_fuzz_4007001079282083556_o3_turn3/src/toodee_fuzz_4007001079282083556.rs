#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use toodee::*;
use global_data::*;
use std::ops::{Deref, DerefMut};
use std::ops::Index;

#[derive(Clone, Debug, Default)]
struct CustomType0(String);

impl Deref for CustomType0 {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for CustomType0 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 2048 {
            return;
        }
        set_global_data(data);
        let global_data = get_global_data();
        let GD = global_data.first_half;

        let choice = _to_u8(GD, 0) % 4;
        let cols = (_to_usize(GD, 1) % 33) + 1;
        let rows = (_to_usize(GD, 9) % 33) + 1;
        let capacity = cols * rows * 2 + 1;

        let mut base_vec = Vec::with_capacity((cols * rows).min(65));
        let mut idx = 17usize;
        while idx + 1 < GD.len() && base_vec.len() < 65 {
            let s_len = _to_u8(GD, idx) % 17;
            idx += 1;
            if idx + s_len as usize >= GD.len() {
                break;
            }
            let s = _to_str(GD, idx, idx + s_len as usize);
            idx += s_len as usize;
            base_vec.push(CustomType0(String::from(s)));
        }
        if base_vec.is_empty() {
            base_vec.push(CustomType0(String::from("fallback")));
        }

        let mut td: TooDee<CustomType0> = match choice {
            0 => TooDee::new(cols, rows),
            1 => TooDee::init(cols, rows, CustomType0(String::from("init"))),
            2 => {
                let mut tmp = TooDee::with_capacity(capacity);
                tmp.push_row(base_vec.clone());
                tmp
            }
            _ => {
                let mut v = base_vec.clone();
                while v.len() < cols * rows {
                    v.push(CustomType0(String::from("pad")));
                }
                v.truncate(cols * rows);
                TooDee::from_vec(cols, rows, v)
            }
        };

        let mut op_iter = (_to_u8(GD, idx % GD.len()) % 10) + 1;
        idx += 1;
        while op_iter > 0 {
            match _to_u8(GD, idx % GD.len()) % 5 {
                0 => {
                    let c = _to_usize(GD, idx % (GD.len() - 8)) % td.num_cols().max(1);
                    td.col_mut(c);
                }
                1 => {
                    let r1 = _to_usize(GD, idx % (GD.len() - 8)) % td.num_rows().max(1);
                    let r2 = _to_usize(GD, (idx + 4) % (GD.len() - 8)) % td.num_rows().max(1);
                    td.swap_rows(r1, r2);
                }
                2 => {
                    let c = _to_usize(GD, idx % (GD.len() - 8)) % td.num_cols().max(1);
                    let col_it = td.col(c);
                    println!("{:?}", col_it.size_hint());
                }
                3 => td.push_row(base_vec.clone()),
                _ => td.push_col(base_vec.clone()),
            }
            op_iter -= 1;
            idx = (idx + 5) % GD.len();
        }

        let start_coord = (
            _to_usize(GD, idx % (GD.len() - 8)) % td.num_cols().max(1),
            _to_usize(GD, (idx + 4) % (GD.len() - 8)) % td.num_rows().max(1),
        );
        idx = (idx + 8) % GD.len();
        let end_coord = (
            (_to_usize(GD, idx % (GD.len() - 8)) % td.num_cols().max(1)).saturating_add(start_coord.0),
            (_to_usize(GD, (idx + 4) % (GD.len() - 8)) % td.num_rows().max(1)).saturating_add(start_coord.1),
        );
        let mut view_mut = td.view_mut(start_coord, end_coord);

        let coord = (
            _to_usize(GD, (idx + 8) % (GD.len() - 8)) % view_mut.num_cols().max(1),
            _to_usize(GD, (idx + 12) % (GD.len() - 8)) % view_mut.num_rows().max(1),
        );

        for _ in 0..3 {
            let cell = view_mut.index(coord);
            println!("{:?}", cell);
        }

        let view_ro: TooDeeView<CustomType0> = view_mut.into();
        let _td2: TooDee<CustomType0> = TooDee::from(view_ro);
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