#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use ordnung::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

fn main (){
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 400 {return;}
        set_global_data(data);
        let global_data = get_global_data();
        let first_half = global_data.first_half;
        let second_half = global_data.second_half;

        let constructor_select = _to_u8(first_half, 50) % 3;
        let vec = match constructor_select {
            0 => {
                let len = _to_usize(first_half, 51) % 65;
                ordnung::compact::Vec::with_capacity(len)
            },
            1 => {
                let mut std_vec = Vec::new();
                std_vec.push(_to_i32(first_half, 52));
                std_vec.push(_to_i32(first_half, 56));
                ordnung::compact::Vec::from(std_vec)
            },
            _ => ordnung::compact::Vec::new(),
        };

        let selector = _to_u8(second_half, 0) % 2;
        let mut target_vec: ordnung::compact::Vec<i32> = match selector {
            0 => {
                let len = _to_usize(second_half, 2) % 65;
                ordnung::compact::Vec::with_capacity(len)
            },
            _ => ordnung::compact::Vec::from_iter(vec![_to_i32(second_half, 4), _to_i32(second_half, 8)]),
        };

        let mut map = Map::new();
        for i in 0..3 {
            let k_start = 5 + i*10;
            let k_len = _to_u8(second_half, k_start) as usize % 16;
            let key = _to_str(second_half, k_start+1, k_start+1+k_len);
            let v_start = k_start+1+k_len;
            let value = _to_i32(second_half, v_start);
            map.insert(key.to_string(), value);
        }

        if _to_bool(second_half, 50) {
            target_vec.push(map.len() as i32);
        }

        let mut iter = map.iter();
        while let Some((k, v)) = iter.next() {
            if _to_bool(second_half, 70) {
                println!("Map entry: {}={}", k, v);
            }
        }

        target_vec.push(_to_i32(second_half, 74));
        target_vec.push(_to_i32(second_half, 78));

        let slice = target_vec.deref();
        let ptr_value = slice.as_ptr();
        println!("Slice pointer: {:?}", ptr_value);

        if _to_bool(second_half, 100) && !target_vec.is_empty() {
            let index = _to_usize(second_half, 101) % target_vec.len();
            let mut cloned = target_vec.clone();
            cloned.remove(index);
            let _ = cloned.deref_mut();
        }

        let mut secondary = ordnung::compact::Vec::new();
        for idx in 150..160 {
            if idx < second_half.len() {
                secondary.push(_to_i32(second_half, idx));
            }
        }

        if _to_bool(second_half, 160) {
            let capacity = map.len() % 65;
            let mut clone_map = Map::with_capacity(capacity);
            for (k, v) in map.iter() {
                clone_map.insert(k.clone(), *v);
            }
            println!("Map clone len: {}", clone_map.len());
        }

        let comparison = vec.deref() == secondary.deref();
        println!("Vec equality: {}", comparison);

        if _to_bool(second_half, 200) {
            let mut extended = vec.clone();
            extended.push(_to_i32(second_half, 201));
            let _ = extended.deref_mut();
        }

        if !map.is_empty() {
            let key_idx = _to_usize(second_half, 210) % map.len();
            if let Some(k) = map.iter().nth(key_idx).map(|(k, _)| k) {
                println!("Random key: {:?}", k);
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