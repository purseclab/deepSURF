#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use lru::*;
use global_data::*;
use std::ops::{Deref, DerefMut};

#[derive(Hash, Eq, PartialEq)]
struct CustomType0(String);
struct CustomType1(String);

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 512 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let first_half = global_data.first_half;
        let second_half = global_data.second_half;

        let constructor_selector = _to_u8(first_half, 0) % 4;
        let cap = _to_usize(first_half, 8);
        let hasher = lru::DefaultHasher::default();

        let mut cache = match constructor_selector {
            0 => LruCache::<CustomType0, CustomType1>::new(cap),
            1 => LruCache::<CustomType0, CustomType1>::unbounded(),
            2 => LruCache::<CustomType0, CustomType1>::with_hasher(cap, hasher),
            _ => LruCache::<CustomType0, CustomType1>::unbounded_with_hasher(hasher),
        };

        let num_ops = _to_u8(first_half, 1) % 10 + 1;
        let mut data_offset = 0;

        for _ in 0..num_ops {
            if data_offset >= second_half.len() { break; }
            let op_selector = _to_u8(second_half, data_offset) % 6;
            data_offset += 1;

            match op_selector {
                0 => {
                    let key_len = (_to_u8(second_half, data_offset) % 65) as usize;
                    data_offset += 1;
                    let key_str = _to_str(second_half, data_offset, data_offset + key_len);
                    data_offset += key_len;
                    let key = CustomType0(key_str.to_string());

                    let val_len = (_to_u8(second_half, data_offset) % 65) as usize;
                    data_offset += 1;
                    let val_str = _to_str(second_half, data_offset, data_offset + val_len);
                    data_offset += val_len;
                    let val = CustomType1(val_str.to_string());

                    let prev = cache.put(key, val);
                    if let Some(p) = prev { println!("{:?}", p.0); }
                }
                1 => {
                    let key_len = (_to_u8(second_half, data_offset) % 65) as usize;
                    data_offset += 1;
                    let key_str = _to_str(second_half, data_offset, data_offset + key_len);
                    data_offset += key_len;
                    let key = CustomType0(key_str.to_string());

                    if let Some(v) = cache.get(&key) { println!("{:?}", v.0); }
                }
                2 => {
                    let mut iter = cache.iter_mut();
                    while let Some((k, v)) = iter.next() {
                        println!("{:?} {:?}", k.0, v.0);
                        v.0.push('!');
                    }
                }
                3 => {
                    let iter = cache.iter();
                    for (k, v) in iter { println!("{:?} {:?}", k.0, v.0); }
                }
                4 => {
                    let iter = cache.into_iter();
                    let count = iter.count();
                    println!("{}", count);
                }
                5 => {
                    let new_cap = _to_usize(second_half, data_offset);
                    data_offset += 8;
                    cache.resize(new_cap);
                }
                _ => {}
            }

            if data_offset + 32 > second_half.len() { data_offset = second_half.len(); }
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