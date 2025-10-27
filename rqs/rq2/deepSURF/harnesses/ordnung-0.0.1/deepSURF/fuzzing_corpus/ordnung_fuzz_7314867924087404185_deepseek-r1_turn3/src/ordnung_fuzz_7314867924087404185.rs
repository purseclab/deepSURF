#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use ordnung::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
struct CustomType0(String);
#[derive(Debug, Clone)]
struct CustomType1(String);

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 512 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let first_half = global_data.first_half;
        let second_half = global_data.second_half;

        let constructor_selector = _to_u8(first_half, 0) % 3;
        let mut map = match constructor_selector {
            0 => ordnung::Map::new(),
            1 => {
                let capacity = _to_usize(first_half, 1);
                ordnung::Map::with_capacity(capacity)
            }
            2 => {
                let entries_len = _to_u8(first_half, 1) % 17;
                let mut entries = Vec::new();
                let mut pos = 2;
                for _ in 0..entries_len {
                    if pos >= first_half.len() { break; }
                    let key_len = first_half[pos] % 17;
                    pos += 1;
                    let key_str = if pos + key_len as usize <= first_half.len() {
                        _to_str(first_half, pos, pos + key_len as usize)
                    } else { break };
                    pos += key_len as usize;
                    
                    if pos >= first_half.len() { break; }
                    let val_len = first_half[pos] % 17;
                    pos += 1;
                    let val_str = if pos + val_len as usize <= first_half.len() {
                        _to_str(first_half, pos, pos + val_len as usize)
                    } else { break };
                    pos += val_len as usize;
                    
                    entries.push((CustomType0(key_str.to_string()), CustomType1(val_str.to_string())));
                }
                ordnung::Map::from_iter(entries.into_iter())
            }
            _ => unreachable!(),
        };

        let op_count = _to_u8(second_half, 0) % 65;
        let mut cursor = 1;

        for _ in 0..op_count {
            if cursor >= second_half.len() { break; }
            let op = second_half[cursor] % 11;
            cursor += 1;

            match op {
                0 => {
                    let key_len = second_half.get(cursor).map(|b| *b % 17).unwrap_or(0);
                    cursor += 1;
                    let key_str = if cursor + key_len as usize <= second_half.len() {
                        _to_str(second_half, cursor, cursor + key_len as usize)
                    } else { break };
                    cursor += key_len as usize;
                    
                    let val_len = second_half.get(cursor).map(|b| *b % 17).unwrap_or(0);
                    cursor += 1;
                    let val_str = if cursor + val_len as usize <= second_half.len() {
                        _to_str(second_half, cursor, cursor + val_len as usize)
                    } else { break };
                    cursor += val_len as usize;

                    map.insert(CustomType0(key_str.to_string()), CustomType1(val_str.to_string()));
                }
                1 => {
                    let key_len = second_half.get(cursor).map(|b| *b % 17).unwrap_or(0);
                    cursor += 1;
                    let key_str = if cursor + key_len as usize <= second_half.len() {
                        _to_str(second_half, cursor, cursor + key_len as usize)
                    } else { break };
                    cursor += key_len as usize;

                    let _ = map.get(&CustomType0(key_str.to_string()));
                }
                2 => {
                    let len = map.len();
                    println!("Len: {}", len);
                }
                3 => {
                    let key_len = second_half.get(cursor).map(|b| *b % 17).unwrap_or(0);
                    cursor += 1;
                    let key_str = if cursor + key_len as usize <= second_half.len() {
                        _to_str(second_half, cursor, cursor + key_len as usize)
                    } else { break };
                    cursor += key_len as usize;

                    let _ = map.remove(&CustomType0(key_str.to_string()));
                }
                4 => {
                    let key_len = second_half.get(cursor).map(|b| *b % 17).unwrap_or(0);
                    cursor += 1;
                    let key_str = if cursor + key_len as usize <= second_half.len() {
                        _to_str(second_half, cursor, cursor + key_len as usize)
                    } else { break };
                    cursor += key_len as usize;

                    let _ = map.contains_key(&CustomType0(key_str.to_string()));
                }
                5 => {
                    let mut iter = map.iter_mut();
                    while let Some((k, v)) = iter.next() {
                        println!("Mut: {:?}", *v);
                    }
                }
                6 => {
                    map.clear();
                }
                7 => {
                    let cloned_map = map.clone();
                    println!("Clone len: {}", cloned_map.len());
                }
                8 => {
                    let key_len = second_half.get(cursor).map(|b| *b % 17).unwrap_or(0);
                    cursor += 1;
                    let key_str = if cursor + key_len as usize <= second_half.len() {
                        _to_str(second_half, cursor, cursor + key_len as usize)
                    } else { break };
                    cursor += key_len as usize;

                    let val = map.get_or_insert(CustomType0(key_str.to_string()), || {
                        let val_len = second_half.get(cursor).map(|b| *b %17).unwrap_or(0);
                        cursor +=1;
                        let val_str = if cursor + val_len as usize <= second_half.len() {
                            _to_str(second_half, cursor, cursor + val_len as usize)
                        } else { "" };
                        cursor += val_len as usize;
                        CustomType1(val_str.to_string())
                    });
                    println!("GetOrInsert: {:?}", val);
                }
                9 => {
                    let key = _to_usize(second_half, cursor);
                    cursor += 8;
                    let _ = map.get_mut(&CustomType0(key.to_string()));
                }
                10 => {
                    let mut vec = ordnung::compact::Vec::new();
                    for (k, v) in map.iter() {
                        vec.push((k.0.clone(), v.0.clone()));
                    }
                }
                _ => {}
            }
        }

        let final_len = map.len();
        println!("Final Length: {}", final_len);
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