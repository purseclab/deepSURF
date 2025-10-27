#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use lru::*;
use global_data::*;
use std::hash::Hasher;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

#[derive(Debug, Eq, Hash, PartialEq)]
struct CustomType0(String);
#[derive(Debug)]
struct CustomType1(String);

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 4096 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        
        let mut offset = 0;
        let constructor_sel = _to_u8(GLOBAL_DATA, offset);
        offset += 1;

        let mut cache = match constructor_sel % 4 {
            0 => {
                let cap = _to_usize(GLOBAL_DATA, offset);
                offset += 8;
                let hasher_len = (_to_u8(GLOBAL_DATA, offset) % 65) as usize;
                offset += 1;
                offset += hasher_len;
                LruCache::with_hasher(cap, lru::DefaultHasher::default())
            },
            1 => {
                let cap = _to_usize(GLOBAL_DATA, offset);
                offset += 8;
                LruCache::new(cap)
            },
            2 => LruCache::unbounded(),
            3 => LruCache::unbounded_with_hasher(lru::DefaultHasher::default()),
            _ => unreachable!(),
        };

        let n_ops = _to_u8(GLOBAL_DATA, offset) % 65;
        offset += 1;

        for _ in 0..n_ops {
            let op_ty = _to_u8(GLOBAL_DATA, offset) % 10;
            offset += 1;

            match op_ty {
                0 => {
                    let key_len = (_to_u8(GLOBAL_DATA, offset) % 65) as usize;
                    offset += 1;
                    let key_str = _to_str(GLOBAL_DATA, offset, offset + key_len);
                    offset += key_len;
                    let val_len = (_to_u8(GLOBAL_DATA, offset) % 65) as usize;
                    offset += 1;
                    let val_str = _to_str(GLOBAL_DATA, offset, offset + val_len);
                    offset += val_len;
                    cache.put(CustomType0(key_str.to_string()), CustomType1(val_str.to_string()));
                    cache.contains(&CustomType0(key_str.to_string()));
                    let _ = cache.len();
                }
                1 => {
                    let key_len = (_to_u8(GLOBAL_DATA, offset) % 65) as usize;
                    offset += 1;
                    let key_str = _to_str(GLOBAL_DATA, offset, offset + key_len);
                    offset += key_len;
                    if let Some(v) = cache.get_mut(&CustomType0(key_str.to_string())) {
                        v.0 = _to_str(GLOBAL_DATA, offset, offset + 8).to_string();
                        println!("{:?}", v);
                    }
                }
                2 => {
                    let key_len = (_to_u8(GLOBAL_DATA, offset) % 65) as usize;
                    offset += 1;
                    let key_str = _to_str(GLOBAL_DATA, offset, offset + key_len);
                    offset += key_len;
                    if let Some(v) = cache.peek(&CustomType0(key_str.to_string())) {
                        println!("{:?}", *v);
                    }
                    let _ = cache.pop(&CustomType0(key_str.to_string()));
                }
                3 => {
                    let mut iter = cache.iter_mut();
                    while let Some((k, v)) = iter.next() {
                        *v = CustomType1(_to_str(GLOBAL_DATA, offset, offset + 16).to_string());
                        println!("{:?}", *k);
                        offset = (offset + 16) % GLOBAL_DATA.len();
                    }
                }
                4 => {
                    if let Some((k, v)) = cache.peek_lru() {
                        println!("{:?} {}", k.0, v.0);
                    }
                    cache.resize(_to_usize(GLOBAL_DATA, offset));
                }
                5 => {
                    let new_cap = _to_usize(GLOBAL_DATA, offset);
                    offset += 8;
                    cache.resize(new_cap);
                    let _ = cache.cap();
                }
                6 => {
                    for (k, v) in cache.iter() {
                        println!("{:?} {:?}", *k, *v);
                    }
                }
                7 => {
                    if let Some(entry) = cache.pop_lru() {
                        println!("Evicted: {:?}", entry.0);
                    }
                    let _ = cache.is_empty();
                }
                8 => {
                    let key_len = (_to_u8(GLOBAL_DATA, offset) % 65) as usize;
                    offset += 1;
                    let key_str = _to_str(GLOBAL_DATA, offset, offset + key_len);
                    offset += key_len;
                    cache.put(CustomType0(key_str.to_string()), CustomType1(String::new()));
                    if let Some(v) = cache.get(&CustomType0(key_str.to_string())) {
                        let _ = v.0.as_str();
                    }
                }
                9 => {
                    let mut iter = cache.iter().rev();
                    while let Some((k, _)) = iter.next_back() {
                        println!("Reverse: {:?}", *k);
                    }
                }
                _ => unreachable!(),
            }
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