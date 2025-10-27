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
        if data.len() < 1024 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let gdata = global_data.first_half;
        
        let cap = _to_usize(gdata, 0);
        let mut cache = if cap == 0 {
            LruCache::<CustomType0, CustomType1>::unbounded()
        } else {
            LruCache::<CustomType0, CustomType1>::new(cap)
        };
        
        let num_ops = _to_u8(gdata, 74) % 10;
        let mut index = 75;
        
        for _ in 0..num_ops {
            if index >= gdata.len() { break; }
            let op = gdata[index] % 12;
            index += 1;
            
            match op {
                0 => {
                    if index + 2 > gdata.len() { continue; }
                    let k_len = _to_u8(gdata, index) % 17;
                    index += 1;
                    let k = _to_str(gdata, index, index + k_len as usize);
                    index += k_len as usize;
                    let v_len = _to_u8(gdata, index) % 17;
                    index += 1;
                    let v = _to_str(gdata, index, index + v_len as usize);
                    index += v_len as usize;
                    cache.put(CustomType0(k.to_string()), CustomType1(v.to_string()));
                }
                1 => {
                    if index + 1 > gdata.len() { continue; }
                    let k_len = _to_u8(gdata, index) % 17;
                    index += 1;
                    let k = _to_str(gdata, index, index + k_len as usize);
                    index += k_len as usize;
                    let _ = cache.get(&CustomType0(k.to_string()));
                }
                2 => {
                    if let Some((k, v)) = cache.peek_lru() {
                        println!("{:?} {:?}", k.0, v.0);
                    }
                }
                3 => {
                    let new_cap = _to_usize(gdata, index);
                    index += 8;
                    cache.resize(new_cap);
                }
                4 => {
                    let mut iter = cache.iter_mut();
                    while let Some((_k, v)) = iter.next() {
                        v.0.push('!');
                    }
                }
                5 => {
                    let mut count = 0;
                    let mut iter = cache.iter();
                    while let Some((_k, _v)) = iter.next() {
                        count += 1;
                    }
                }
                6 => {
                    if let Some((k, v)) = cache.pop_lru() {
                        println!("{:?} {:?}", k.0, v.0);
                    }
                }
                7 => {
                    cache.clear();
                }
                8 => {
                    let key = CustomType0("peek_key".into());
                    let _ = cache.peek(&key);
                }
                9 => {
                    let key = CustomType0("contains_key".into());
                    let _ = cache.contains(&key);
                }
                10 => {
                    if let Some(mut entry) = cache.peek_mut(&CustomType0("peek_mut".into())) {
                        entry.0.push('?');
                    }
                }
                11 => {
                    let _ = cache.len();
                    let _ = cache.cap();
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