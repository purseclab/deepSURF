#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use lru::*;
use global_data::*;

#[derive(Debug, Hash, Eq, PartialEq)]
struct CustomType0(String);
#[derive(Debug)]
struct CustomType1(String);

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 2048 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let mut offset = 0;

        let constructor_sel = _to_u8(GLOBAL_DATA, offset) % 4;
        offset += 1;
        let mut cache = match constructor_sel {
            0 => {
                let cap = _to_usize(GLOBAL_DATA, offset);
                offset += 8;
                LruCache::new(cap)
            },
            1 => {
                let cap = _to_usize(GLOBAL_DATA, offset);
                offset += 8;
                LruCache::with_hasher(cap, lru::DefaultHasher::default())
            },
            2 => LruCache::unbounded(),
            _ => LruCache::unbounded_with_hasher(lru::DefaultHasher::default()),
        };

        let num_entries = _to_u8(GLOBAL_DATA, offset) % 65;
        offset += 1;
        for _ in 0..num_entries {
            let key_len = _to_u8(GLOBAL_DATA, offset) % 65;
            offset = offset.wrapping_add(1);
            let key = _to_str(GLOBAL_DATA, offset, offset + key_len as usize);
            offset = offset.wrapping_add(key_len as usize);
            let val_len = _to_u8(GLOBAL_DATA, offset) % 65;
            offset = offset.wrapping_add(1);
            let val = _to_str(GLOBAL_DATA, offset, offset + val_len as usize);
            offset = offset.wrapping_add(val_len as usize);
            cache.put(CustomType0(key.to_string()), CustomType1(val.to_string()));
        }

        let num_ops = _to_u8(GLOBAL_DATA, offset) % 65;
        offset += 1;
        for _ in 0..num_ops {
            let op = _to_u8(GLOBAL_DATA, offset) % 11;
            offset += 1;
            match op {
                0 => {
                    let key_len = _to_u8(GLOBAL_DATA, offset) % 65;
                    offset += 1;
                    let key = _to_str(GLOBAL_DATA, offset, offset + key_len as usize);
                    offset += key_len as usize;
                    let _ = cache.get(&CustomType0(key.to_string()));
                },
                1 => {
                    let key_len = _to_u8(GLOBAL_DATA, offset) % 65;
                    offset += 1;
                    let key = _to_str(GLOBAL_DATA, offset, offset + key_len as usize);
                    offset += key_len as usize;
                    let _ = cache.peek(&CustomType0(key.to_string()));
                },
                2 => {
                    let key_len = _to_u8(GLOBAL_DATA, offset) % 65;
                    offset += 1;
                    let key = _to_str(GLOBAL_DATA, offset, offset + key_len as usize);
                    offset += key_len as usize;
                    let _ = cache.get_mut(&CustomType0(key.to_string()));
                },
                3 => {
                    let _ = cache.pop_lru();
                },
                4 => {
                    let new_cap = _to_usize(GLOBAL_DATA, offset);
                    offset += 8;
                    cache.resize(new_cap);
                },
                5 => {
                    let key_len = _to_u8(GLOBAL_DATA, offset) % 65;
                    offset += 1;
                    let key = _to_str(GLOBAL_DATA, offset, offset + key_len as usize);
                    offset += key_len as usize;
                    let _ = cache.peek_mut(&CustomType0(key.to_string()));
                },
                6 => {
                    let key_len = _to_u8(GLOBAL_DATA, offset) % 65;
                    offset += 1;
                    let key = _to_str(GLOBAL_DATA, offset, offset + key_len as usize);
                    offset += key_len as usize;
                    let _ = cache.pop(&CustomType0(key.to_string()));
                },
                7 => {
                    let _ = cache.peek_lru();
                },
                8 => {
                    let _ = cache.len();
                },
                9 => {
                    let _ = cache.cap();
                },
                10 => {
                    cache.clear();
                },
                _ => (),
            }
        }

        let peek_entry = cache.peek_lru();
        let _len = cache.len();
        let _cap = cache.cap();

        let mut iter = cache.iter();
        for _ in 0.._to_u8(GLOBAL_DATA, offset) % 10 {
            if let Some((k, v)) = iter.next() {
                println!("{:?} {:?}", k.0, v.0);
            }
        }

        let mut iter_mut = cache.iter_mut();
        while let Some((k, v)) = iter_mut.next() {
            println!("{:?} {:?}", k.0, v.0);
        }

        if cache.is_empty() {
            panic!("Intentional panic to expose panic safety bugs");
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