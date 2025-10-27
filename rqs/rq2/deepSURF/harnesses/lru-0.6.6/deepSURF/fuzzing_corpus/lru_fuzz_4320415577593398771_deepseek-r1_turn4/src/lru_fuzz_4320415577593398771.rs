#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use lru::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

#[derive(Debug, Hash, Eq, PartialEq)]
struct CustomType0(String);
#[derive(Debug)]
struct CustomType1(String);

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 1024 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let mut cache = match _to_u8(GLOBAL_DATA,0) % 5 {
            0 => LruCache::<CustomType0, CustomType1>::new(_to_usize(GLOBAL_DATA,1)),
            1 => LruCache::with_hasher(_to_usize(GLOBAL_DATA,1), DefaultHasher::default()),
            2 => LruCache::unbounded_with_hasher(DefaultHasher::default()),
            3 => LruCache::unbounded(),
            _ => LruCache::<CustomType0, CustomType1>::new(_to_usize(GLOBAL_DATA,1))
        };

        let OP_DATA = global_data.second_half;
        let operations = _to_u8(OP_DATA,0) % 25;
        let mut idx = 1;

        for _ in 0..operations {
            match _to_u8(OP_DATA, idx) % 11 {
                0 => {
                    if idx + 2 >= OP_DATA.len() { break; }
                    let key_len = _to_u8(OP_DATA, idx+1) % 65;
                    let key = _to_str(OP_DATA, idx+2, idx+2+key_len as usize);
                    idx += 2 + key_len as usize;
                    
                    if idx + 2 >= OP_DATA.len() { break; }
                    let val_len = _to_u8(OP_DATA, idx+1) % 65;
                    let val = _to_str(OP_DATA, idx+2, idx+2+val_len as usize);
                    idx += 2 + val_len as usize;
                    
                    cache.put(CustomType0(key.to_string()), CustomType1(val.to_string()));
                },
                1 => {
                    if idx + 2 >= OP_DATA.len() { break; }
                    let key_len = _to_u8(OP_DATA, idx+1) % 65;
                    let key = _to_str(OP_DATA, idx+2, idx+2+key_len as usize);
                    idx += 2 + key_len as usize;
                    let k = CustomType0(key.to_string());
                    if let Some(v) = cache.get(&k) { println!("{:?}", *v); }
                    cache.peek(&k);
                },
                2 => {
                    if idx + 2 >= OP_DATA.len() { break; }
                    let key_len = _to_u8(OP_DATA, idx+1) % 65;
                    let key = _to_str(OP_DATA, idx+2, idx+2+key_len as usize);
                    idx += 2 + key_len as usize;
                    let k = CustomType0(key.to_string());
                    if let Some(v) = cache.get_mut(&k) { println!("{:?}", *v); }
                    cache.peek_mut(&k);
                },
                3 => {
                    if let Some((k, v)) = cache.pop_lru() {
                        println!("{:?} {:?}", k.0, v.0);
                    }
                },
                4 => {
                    cache.iter_mut().for_each(|(k, v)| {
                        println!("{:?} {:?}", *k, *v);
                    });
                },
                5 => {
                    if idx + 8 >= OP_DATA.len() { break; }
                    cache.resize(_to_usize(OP_DATA, idx+1));
                    idx += 8;
                },
                6 => {
                    if idx + 2 >= OP_DATA.len() { break; }
                    let key_len = _to_u8(OP_DATA, idx+1) % 65;
                    let key = _to_str(OP_DATA, idx+2, idx+2+key_len as usize);
                    idx += 2 + key_len as usize;
                    cache.pop(&CustomType0(key.to_string()));
                },
                7 => {
                    let len = cache.len();
                    println!("{}", len);
                },
                8 => {
                    let cap = cache.cap();
                    println!("{}", cap);
                },
                9 => {
                    cache.clear();
                },
                _ => {
                    if let Some((k, v)) = cache.peek_lru() {
                        println!("{:?} {:?}", *k, *v);
                    }
                }
            }
            idx += 1;
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