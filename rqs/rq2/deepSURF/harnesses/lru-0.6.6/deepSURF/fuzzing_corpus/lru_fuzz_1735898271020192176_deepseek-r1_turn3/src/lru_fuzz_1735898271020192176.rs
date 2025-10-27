#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use lru::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

#[derive(Eq, Hash, PartialEq)]
struct CustomType0(String);
struct CustomType1(String);

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 500 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        
        let mut pos = 0;
        let constructor_sel = _to_u8(GLOBAL_DATA, pos) % 4;
        pos += 1;

        let mut cache = match constructor_sel {
            0 => {
                let cap = _to_usize(GLOBAL_DATA, pos);
                pos += 8;
                LruCache::<CustomType0, CustomType1>::new(cap)
            }
            1 => {
                let cap = _to_usize(GLOBAL_DATA, pos);
                pos += 8;
                LruCache::with_hasher(cap, DefaultHasher::default())
            }
            2 => {
                LruCache::unbounded_with_hasher(DefaultHasher::default())
            }
            3 => LruCache::unbounded(),
            _ => unreachable!()
        };

        let op_count = _to_u8(GLOBAL_DATA, pos) % 10;
        pos += 1;

        for _ in 0..op_count {
            if pos >= GLOBAL_DATA.len() { break; }
            let op = _to_u8(GLOBAL_DATA, pos) % 8;
            pos += 1;

            match op {
                0 => {
                    if pos + 1 >= GLOBAL_DATA.len() { break; }
                    let k_len = _to_u8(GLOBAL_DATA, pos) % 65;
                    pos += 1;
                    if pos + k_len as usize > GLOBAL_DATA.len() { break; }
                    let k_str = _to_str(GLOBAL_DATA, pos, pos + k_len as usize);
                    pos += k_len as usize;

                    let v_len = _to_u8(GLOBAL_DATA, pos) % 65;
                    pos += 1;
                    if pos + v_len as usize > GLOBAL_DATA.len() { break; }
                    let v_str = _to_str(GLOBAL_DATA, pos, pos + v_len as usize);
                    pos += v_len as usize;

                    cache.put(CustomType0(k_str.to_string()), CustomType1(v_str.to_string()));
                }
                1 => {
                    if pos + 1 >= GLOBAL_DATA.len() { break; }
                    let k_len = _to_u8(GLOBAL_DATA, pos) % 65;
                    pos += 1;
                    if pos + k_len as usize > GLOBAL_DATA.len() { break; }
                    let k_str = _to_str(GLOBAL_DATA, pos, pos + k_len as usize);
                    pos += k_len as usize;

                    let _ = cache.get(&CustomType0(k_str.to_string()));
                }
                2 => {
                    if pos + 8 > GLOBAL_DATA.len() { break; }
                    let new_cap = _to_usize(GLOBAL_DATA, pos);
                    pos += 8;
                    cache.resize(new_cap);
                }
                3 => {
                    let mut iter = cache.iter_mut();
                    for (_, v) in &mut iter {
                        *v = CustomType1("modified".into());
                    }
                }
                4 => {
                    if let Some((k, v)) = cache.peek_lru() {
                        println!("{:?} {:?}", k.0, v.0);
                    }
                }
                5 => {
                    let _ = cache.pop_lru();
                }
                6 => {
                    cache.clear();
                }
                7 => {
                    if pos + 1 >= GLOBAL_DATA.len() { break; }
                    let k_len = _to_u8(GLOBAL_DATA, pos) % 65;
                    pos += 1;
                    if pos + k_len as usize > GLOBAL_DATA.len() { break; }
                    let k_str = _to_str(GLOBAL_DATA, pos, pos + k_len as usize);
                    pos += k_len as usize;

                    if let Some(v) = cache.get_mut(&CustomType0(k_str.to_string())) {
                        *v = CustomType1("mutated".into());
                    }
                }
                _ => {}
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