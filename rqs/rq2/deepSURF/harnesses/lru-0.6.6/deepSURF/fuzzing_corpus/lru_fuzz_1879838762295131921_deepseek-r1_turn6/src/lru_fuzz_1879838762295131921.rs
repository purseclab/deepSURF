#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use lru::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};
use lru::DefaultHasher;

#[derive(Debug, Hash, Eq, PartialEq)]
struct CustomType0(String);
#[derive(Debug)]
struct CustomType1(String);

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 512 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let cap = _to_usize(GLOBAL_DATA, 0);
        let constructor_selector = _to_u8(GLOBAL_DATA, 8) % 4;
        let mut cache = match constructor_selector {
            0 => LruCache::<CustomType0, CustomType1>::new(cap),
            1 => LruCache::unbounded(),
            2 => {
                let hasher = DefaultHasher::default();
                LruCache::with_hasher(cap, hasher)
            }
            3 => {
                let hasher = DefaultHasher::default();
                LruCache::unbounded_with_hasher(hasher)
            }
            _ => unreachable!(),
        };

        let n_ops = _to_u8(GLOBAL_DATA, 112) % 65;
        let mut offset = 113;

        for _ in 0..n_ops {
            if offset + 1 > GLOBAL_DATA.len() { break; }
            let op = _to_u8(GLOBAL_DATA, offset) % 13;
            offset += 1;

            match op {
                0 => {
                    let key_len = _to_u8(GLOBAL_DATA, offset) % 65;
                    let key_str = _to_str(GLOBAL_DATA, offset + 1, offset + 1 + key_len as usize);
                    offset += 1 + key_len as usize;
                    let val_len = _to_u8(GLOBAL_DATA, offset) % 65;
                    let val_str = _to_str(GLOBAL_DATA, offset + 1, offset + 1 + val_len as usize);
                    offset += 1 + val_len as usize;
                    let prev = cache.put(CustomType0(key_str.to_string()), CustomType1(val_str.to_string()));
                    if let Some(p) = prev {
                        println!("Evicted: {:?}", p.0);
                    }
                }
                1 => {
                    if let Some((k, v)) = cache.peek_lru() {
                        println!("Peek LRU: {:?} => {:?}", *k, *v);
                        cache.get(k);
                    }
                }
                2 => {
                    let key_len = _to_u8(GLOBAL_DATA, offset) % 65;
                    let key_str = _to_str(GLOBAL_DATA, offset + 1, offset + 1 + key_len as usize);
                    offset += 1 + key_len as usize;
                    let key = CustomType0(key_str.to_string());
                    if let Some(v) = cache.get(&key) {
                        println!("Get: {:?}", *v);
                        cache.put(key, CustomType1("updated".into()));
                    }
                }
                3 => {
                    let key_len = _to_u8(GLOBAL_DATA, offset) % 65;
                    let key_str = _to_str(GLOBAL_DATA, offset + 1, offset + 1 + key_len as usize);
                    offset += 1 + key_len as usize;
                    let key = CustomType0(key_str.to_string());
                    if let Some(v) = cache.get_mut(&key) {
                        *v = CustomType1("modified".into());
                        println!("Mutated: {:?}", v.0);
                    }
                }
                4 => {
                    if let Some((k, v)) = cache.pop_lru() {
                        println!("Pop LRU: {:?} => {:?}", k.0, v.0);
                        cache.peek(&k);
                    }
                }
                5 => {
                    let new_cap = _to_usize(GLOBAL_DATA, offset);
                    offset += 8;
                    cache.resize(new_cap);
                    println!("New cap: {}", new_cap);
                }
                6 => {
                    let mut iter = cache.iter_mut();
                    while let Some((k, v)) = iter.next() {
                        *v = CustomType1(format!("{}-mut", v.0));
                        println!("IterMut: {:?} => {:?}", k.0, v.0);
                    }
                }
                7 => {
                    let key_len = _to_u8(GLOBAL_DATA, offset) % 65;
                    let key_str = _to_str(GLOBAL_DATA, offset + 1, offset + 1 + key_len as usize);
                    offset += 1 + key_len as usize;
                    let key = CustomType0(key_str.to_string());
                    if let Some(v) = cache.pop(&key) {
                        println!("Popped: {:?}", v.0);
                    }
                }
                8 => {
                    cache.clear();
                    println!("Cache cleared");
                }
                9 => {
                    let key_len = _to_u8(GLOBAL_DATA, offset) % 65;
                    let key_str = _to_str(GLOBAL_DATA, offset + 1, offset + 1 + key_len as usize);
                    offset += 1 + key_len as usize;
                    let key = CustomType0(key_str.to_string());
                    if cache.contains(&key) {
                        println!("Contains key: {}", key_str);
                    }
                }
                10 => {
                    let len = cache.len();
                    println!("Cache length: {}", len);
                }
                11 => {
                    let cap = cache.cap();
                    println!("Cache capacity: {}", cap);
                }
                12 => {
                    let mut iter = cache.iter();
                    while let Some((k, v)) = iter.next() {
                        println!("Iter: {:?} => {:?}", k.0, v.0);
                        let _len = k.0.len() + v.0.len();
                    }
                }
                _ => (),
            }
        }

        for (k, v) in cache.iter_mut() {
            *v = CustomType1(format!("{}-mut", v.0));
            println!("Final mutate: {:?} => {:?}", k.0, v.0);
        }

        let kv_count = _to_u8(GLOBAL_DATA, offset) % 65;
        for _ in 0..kv_count {
            if offset + 2 > GLOBAL_DATA.len() {
                break;
            }
            let key_len = _to_u8(GLOBAL_DATA, offset) % 65;
            let key_str = _to_str(GLOBAL_DATA, offset + 1, offset + 1 + key_len as usize);
            offset += 1 + key_len as usize;
            let val_len = _to_u8(GLOBAL_DATA, offset) % 65;
            let val_str = _to_str(GLOBAL_DATA, offset + 1, offset + 1 + val_len as usize);
            offset += 1 + val_len as usize;
            cache.put(
                CustomType0(key_str.to_string()),
                CustomType1(val_str.to_string()),
            );
        }

        if let Some((k, v)) = cache.peek_lru() {
            let _ = k.0.chars().chain(v.0.chars()).count();
            cache.get_mut(k);
        }

        let mut iter = cache.iter_mut();
        while let Some((k, v)) = iter.next_back() {
            *v = CustomType1("back-mutated".into());
            println!("ReverseMut: {:?} => {:?}", k.0, v.0);
        }

        cache.clear();
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