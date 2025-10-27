#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use id_map::*;
use global_data::*;
use id_set::IdSet;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

#[derive(Debug)]
struct CustomType0(String);

impl std::clone::Clone for CustomType0 {
    fn clone(&self) -> Self {
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
        let mut t_0 = _to_u8(GLOBAL_DATA, 8) % 17;
        let t_1 = _to_str(GLOBAL_DATA, 9, 9 + t_0 as usize);
        let t_2 = String::from(t_1);
        let t_3 = CustomType0(t_2);
        return t_3;
    }
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 100 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let constructor_selector = _to_u8(global_data.first_half, 0) % 3;
        let mut map = match constructor_selector {
            0 => IdMap::new(),
            1 => IdMap::with_capacity(_to_usize(global_data.first_half, 1)),
            _ => IdMap::from_iter((0..3).map(|i| {
                let start = 2 + i * 8;
                CustomType0(_to_str(global_data.first_half, start, start + 8).to_string())
            }))
        };

        let num_ops = _to_usize(global_data.second_half, 0) % 8;
        for i in 0..num_ops {
            let op_selector = _to_u8(global_data.second_half, i + 1) % 7;
            match op_selector {
                0 => {
                    let val = CustomType0(_to_str(global_data.second_half, 10 + i*5, 15 + i*5).to_string());
                    map.insert(val);
                },
                1 => {
                    let id = _to_usize(global_data.second_half, 20 + i*2);
                    if let Some(v) = map.get(id) {
                        println!("{:?}", *v);
                    }
                },
                2 => {
                    let id = _to_usize(global_data.second_half, 30 + i*3);
                    map.remove(id);
                },
                3 => {
                    let cloned = map.clone();
                    for (idx, item) in cloned.into_iter() {
                        println!("{:?}", item);
                    }
                },
                4 => {
                    let id_set = IdSet::new_filled(_to_usize(global_data.second_half, 40 + i));
                    map.remove_set(&id_set);
                },
                5 => {
                    let id = _to_usize(global_data.second_half, 50 + i*2);
                    map.get_or_insert_with(id, || panic!("Closure panic!"));
                },
                6 => {
                    let mut iter = map.iter_mut();
                    while let Some((_, val)) = iter.next() {
                        *val = CustomType0("modified".into());
                    }
                },
                _ => ()
            }
        }

        let mut values = map.values_mut();
        while let Some(val) = values.next() {
            *val = CustomType0(_to_str(global_data.first_half, 60, 68).to_string());
        }

        let _ = map.clone();
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