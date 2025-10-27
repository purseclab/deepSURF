#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use lru::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

struct CustomType1(String);

fn main (){
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 512 {return;}
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        
        let constructor_selector = _to_u8(GLOBAL_DATA, 0) % 4;
        let mut cache = match constructor_selector {
            0 => LruCache::new(_to_usize(GLOBAL_DATA, 1)),
            1 => LruCache::unbounded(),
            2 => {
                let cap = _to_usize(GLOBAL_DATA, 1);
                LruCache::with_hasher(cap, DefaultHasher::default())
            },
            3 => {
                LruCache::unbounded_with_hasher(DefaultHasher::default())
            },
            _ => unreachable!()
        };

        let ops_data = global_data.second_half;
        let mut offset = 0;
        let num_ops = if ops_data.is_empty() {0} else {_to_u8(ops_data, 0) % 16};
        offset += 1;

        for _ in 0..num_ops {
            if offset >= ops_data.len() {break;}
            let op_selector = _to_u8(ops_data, offset) % 12;
            offset += 1;

            match op_selector {
                0 => {
                    if offset + 2 > ops_data.len() {continue;}
                    let key_len = _to_u8(ops_data, offset) % 65;
                    offset += 1;
                    let key_str = _to_str(ops_data, offset, offset + key_len as usize);
                    offset += key_len as usize;
                    let val_len = _to_u8(ops_data, offset) % 65;
                    offset += 1;
                    let val_str = _to_str(ops_data, offset, offset + val_len as usize);
                    offset += val_len as usize;
                    cache.put(key_str.to_string(), CustomType1(val_str.to_string()));
                }
                1 => {
                    if offset >= ops_data.len() {continue;}
                    let key_len = _to_u8(ops_data, offset) % 65;
                    offset += 1;
                    let key_str = _to_str(ops_data, offset, offset + key_len as usize);
                    let key = key_str.to_string();
                    offset += key_len as usize;
                    if let Some(v) = cache.get(&key) {
                        println!("{:?}", v.0);
                    }
                }
                2 => {
                    if offset >= ops_data.len() {continue;}
                    let key_len = _to_u8(ops_data, offset) % 65;
                    offset += 1;
                    let key_str = _to_str(ops_data, offset, offset + key_len as usize);
                    let key = key_str.to_string();
                    offset += key_len as usize;
                    if let Some(v) = cache.get_mut(&key) {
                        v.0.push_str("_mut");
                    }
                }
                3 => {
                    if let Some((k, v)) = cache.peek_lru() {
                        println!("LRU: {:?} {:?}", k, v.0);
                    }
                }
                4 => {
                    if let Some((k, v)) = cache.pop_lru() {
                        println!("Popped: {:?} {:?}", k, v.0);
                    }
                }
                5 => {
                    if offset + 8 > ops_data.len() {continue;}
                    cache.resize(_to_usize(ops_data, offset));
                    offset += 8;
                }
                6 => cache.clear(),
                7 => {
                    for (k, v) in cache.iter() {
                        println!("Entry: {:?} {:?}", k, v.0);
                    }
                }
                8 => {
                    for (k, v) in cache.iter_mut() {
                        v.0.push_str("_mod");
                        println!("Mut Entry: {:?}", k);
                    }
                }
                9 => {
                    if offset + 8 > ops_data.len() {continue;}
                    println!("Capacity: {}", cache.cap());
                }
                10 => {
                    if offset + 8 > ops_data.len() {continue;}
                    println!("Entries: {}", cache.len());
                }
                11 => {
                    let c = cache.iter().count();
                    println!("Iter count: {}", c);
                }
                _ => {}
            }
        }

        for (k, v) in cache.iter() {
            println!("Final Entry: {:?} {:?}", k, v.0);
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