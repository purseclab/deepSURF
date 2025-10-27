#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use lru::*;
use lru::DefaultHasher;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut};

#[derive(Hash, Eq, PartialEq)]
struct CustomType0(String);
struct CustomType1(String);

impl std::fmt::Debug for CustomType0 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::fmt::Debug for CustomType1 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 2048 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let first_half = global_data.first_half;
        let mut index = 0;

        let mut cache = match _to_u8(first_half, 0) % 4 {
            0 => LruCache::new(_to_usize(first_half, 1)),
            1 => LruCache::with_hasher(_to_usize(first_half, 1), DefaultHasher::default()),
            2 => LruCache::unbounded(),
            3 => LruCache::unbounded_with_hasher(DefaultHasher::default()),
            _ => unreachable!(),
        };
        index += 9;

        for _ in 0..(_to_u8(first_half, index) % 65) {
            index +=1;
            let key_len = _to_u8(first_half, index) % 50;
            let key = CustomType0(_to_str(first_half, index+1, index+1+key_len as usize).to_string());
            index += 1 + key_len as usize;
            let val_len = _to_u8(first_half, index) % 50;
            let value = CustomType1(_to_str(first_half, index+1, index+1+val_len as usize).to_string());
            index += 1 + val_len as usize;
            cache.put(key, value);
        }

        for _ in 0..(_to_u8(first_half, index) % 65) {
            index +=1;
            match _to_u8(first_half, index) % 13 {
                0 => {
                    let key_len = _to_u8(first_half, index+1) % 50;
                    let key = CustomType0(_to_str(first_half, index+2, index+2+key_len as usize).to_string());
                    index += 2 + key_len as usize;
                    let _ = cache.get(&key).inspect(|v| println!("{:?}", *v));
                }
                1 => {
                    let key_len = _to_u8(first_half, index+1) % 50;
                    let key = CustomType0(_to_str(first_half, index+2, index+2+key_len as usize).to_string());
                    index += 2 + key_len as usize;
                    let _ = cache.get_mut(&key).map(|v| *v = CustomType1("MODIFIED".into()));
                }
                2 => { let _ = cache.pop_lru().inspect(|(k, v)| println!("{:?} {:?}", k, v)); }
                3 => cache.resize(_to_usize(first_half, index+1)),
                4 => cache.clear(),
                5 => { let _ = cache.peek_lru().inspect(|(k, v)| println!("{:?} {:?}", k, v)); }
                6 => {
                    let key_len = _to_u8(first_half, index+1) % 50;
                    let key = CustomType0(_to_str(first_half, index+2, index+2+key_len as usize).to_string());
                    index += 2 + key_len as usize;
                    let _ = cache.peek(&key).inspect(|v| println!("{:?}", *v));
                }
                7 => {
                    let key_len = _to_u8(first_half, index+1) % 50;
                    let key = CustomType0(_to_str(first_half, index+2, index+2+key_len as usize).to_string());
                    index += 2 + key_len as usize;
                    let _ = cache.peek_mut(&key).map(|v| *v = CustomType1("ALTERED".into()));
                }
                8 => {
                    let key_len = _to_u8(first_half, index+1) % 50;
                    let key = CustomType0(_to_str(first_half, index+2, index+2+key_len as usize).to_string());
                    index += 2 + key_len as usize;
                    let _ = cache.pop(&key).inspect(|v| println!("Removed {:?}", v));
                }
                9 => println!("Cache len: {:?} cap: {:?}", cache.len(), cache.cap()),
                10 => {
                    let key = CustomType0(_to_str(first_half, index+1, index+11).to_string());
                    index += 10;
                    let _ = cache.contains(&key);
                }
                11 => {
                    for (k, v) in cache.iter() {
                        println!("Iter: {:?} => {:?}", k, v);
                    }
                }
                12 => {
                    let new_cap = _to_usize(first_half, index+1) % 256;
                    cache.resize(new_cap);
                    index += 8;
                }
                _ => (),
            }
        }

        let mut iter = cache.iter_mut();
        let _ = iter.next().map(|(k, v)| {
            println!("Mutating: {:?}", k);
            *v = CustomType1("FIRST".into());
        });

        for _ in 0..(_to_u8(first_half, index) % 65) {
            index +=1;
            let key_len = _to_u8(first_half, index) % 30;
            let key = CustomType0(_to_str(first_half, index+1, index+1+key_len as usize).to_string());
            index += 1 + key_len as usize;
            let val_len = _to_u8(first_half, index) % 30;
            let value = CustomType1(_to_str(first_half, index+1, index+1+val_len as usize).to_string());
            index += 1 + val_len as usize;
            cache.put(key, value);
        }

        let mut iter_mut = cache.iter_mut();
        while let Some((k, v)) = iter_mut.next_back() {
            println!("Reversing: {:?}", k);
            *v = CustomType1("CHANGED".into());
        }

        let _evicted = cache.put(CustomType0("FUZZ".into()), CustomType1("TEST".into()));
        let _contains = cache.contains(&CustomType0("FUZZ".into()));
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