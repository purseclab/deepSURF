#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use ordnung::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 500 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let mut map = Map::new();
        let mut key_offset = 0;
        let mut val_offset = 250;
        for _ in 0..(_to_usize(GLOBAL_DATA, 0) % 5) {
            let key_part = _to_str(GLOBAL_DATA, key_offset, key_offset + 8);
            let val_part = _to_u8(global_data.second_half, val_offset);
            map.insert(key_part.to_string(), val_part);
            key_offset += 8;
            val_offset += 1;
        }

        let vec_type = _to_u8(GLOBAL_DATA, 50) % 4;
        let mut vec = match vec_type {
            0 => {
                let cap = _to_usize(GLOBAL_DATA, 51) % 65;
                compact::Vec::with_capacity(cap)
            },
            1 => {
                let t_9 = _to_u8(GLOBAL_DATA, 51) % 17;
                let t_10 = _to_str(GLOBAL_DATA, 52, 52 + t_9 as usize);
                compact::Vec::from_iter(t_10.bytes())
            },
            2 => {
                let mut std_vec = std::vec::Vec::new();
                for i in 0..(_to_usize(GLOBAL_DATA, 51) % 65) {
                    std_vec.push(_to_u8(GLOBAL_DATA, 52 + i));
                }
                compact::Vec::from(std_vec)
            },
            3 => compact::Vec::new(),
            _ => unreachable!()
        };

        let ops = _to_usize(global_data.second_half, 0) % 15 + 5;
        let mut offset = 100;
        for _ in 0..ops {
            let op = _to_u8(global_data.second_half, offset) % 7;
            offset += 1;

            match op {
                0 => {
                    let idx = _to_usize(global_data.second_half, offset);
                    let _ = vec.remove(idx % 65);
                    offset += 8;
                },
                1 => { let _ = vec.pop(); },
                2 => {
                    let elem = _to_u8(global_data.second_half, offset);
                    offset += 1;
                    let cap = vec.capacity();
                    vec.push(elem);
                    println!("Capacity after push: {}", cap);
                },
                3 => {
                    println!("Current vec len: {}", vec.len());
                    vec.clear();
                },
                4 => {
                    let new_cap = _to_usize(global_data.second_half, offset);
                    let mut temp = compact::Vec::with_capacity(new_cap % 65);
                    temp.push(_to_u8(global_data.second_half, offset + 8));
                    vec = temp;
                    offset += 9;
                },
                5 => {
                    let key_part = _to_str(global_data.second_half, offset, offset + 8);
                    let val_part = _to_u8(global_data.second_half, offset + 8);
                    let v = map.get(&*key_part.to_string()).map(|x| *x);
                    if let Some(val) = v {
                        vec.push(val);
                    }
                    offset += 9;
                },
                6 => {
                    let mut iter = map.iter();
                    while let Some((k, v)) = iter.next() {
                        println!("Map entry: {} -> {}", k, v);
                        vec.push(*v);
                    }
                },
                _ => {}
            }

            for elem in vec.iter() {
                println!("Element debug: {:?}", elem);
            }
            
            if !vec.is_empty() {
                let _val = &vec[vec.len() % vec.len()];
                println!("Accessed elem: {}", _val);
            }
        }

        let mut cloned = vec.clone();
        while let Some(e) = cloned.pop() {
            println!("Cloned pop: {}", e);
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