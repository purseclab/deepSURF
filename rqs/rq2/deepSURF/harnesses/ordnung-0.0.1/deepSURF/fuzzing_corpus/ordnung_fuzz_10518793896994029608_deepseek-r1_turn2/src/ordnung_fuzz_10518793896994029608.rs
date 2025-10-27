#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use ordnung::*;
use global_data::*;
use std::ops::{Deref, DerefMut};

#[derive(Debug)]
struct CustomType2(String);
#[derive(Debug)]
struct CustomType1(String);
#[derive(Debug)]
struct CustomType0(String);

impl core::iter::Iterator for CustomType2 {
    type Item = CustomType1;
    
    fn next(&mut self) -> Option<Self::Item> {
        let global_data = get_global_data();
        let data = global_data.first_half;
        let selector = (_to_usize(data, 0) + self.0.len()) % 3;
        if selector == 0 { panic!("INTENTIONAL PANIC!"); }
        let data = match selector { 1 => data, _ => global_data.second_half };
        let len = _to_u8(data, 8) % 17;
        Some(CustomType1(String::from(_to_str(data, 9, 9 + len as usize))))
    }
}

impl core::iter::IntoIterator for CustomType0 {
    type Item = CustomType1;
    type IntoIter = CustomType2;
    
    fn into_iter(self) -> Self::IntoIter {
        let global_data = get_global_data();
        let data = global_data.first_half;
        let selector = (_to_usize(data, 25) + self.0.len()) % 3;
        if selector == 0 { panic!("INTENTIONAL PANIC!"); }
        let data = match selector { 1 => data, _ => global_data.second_half };
        let len = _to_u8(data, 33) % 17;
        CustomType2(String::from(_to_str(data, 34, 34 + len as usize)))
    }
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 2048 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let gdata = global_data.first_half;

        let mut vec = match _to_u8(gdata, 0) % 4 {
            0 => ordnung::compact::Vec::new(),
            1 => ordnung::compact::Vec::with_capacity(_to_usize(gdata, 1)),
            2 => ordnung::compact::Vec::from_iter(CustomType0(String::from(_to_str(gdata, 2, 70)))),
            _ => ordnung::compact::Vec::from_iter((0.._to_u8(gdata, 100)).map(|i| CustomType1(format!("item_{}", i)))),
        };

        let mut map = match _to_u8(gdata, 101) % 2 {
            0 => ordnung::Map::<String, String>::new(),
            _ => ordnung::Map::with_capacity(_to_usize(gdata, 102)),
        };

        for i in 0..(_to_u8(gdata, 150) % 65) {
            match _to_u8(gdata, 151 + i as usize) % 10 {
                0 => vec.push(CustomType1(String::from(_to_str(gdata, 200 + i as usize * 10, 210 + i as usize * 10)))),
                1 => { vec.pop(); }
                2 => { 
                    map.insert(
                        String::from(_to_str(gdata, 500 + i as usize * 20, 510 + i as usize * 20)),
                        String::from(_to_str(gdata, 510 + i as usize * 20, 520 + i as usize * 20))
                    ); 
                }
                3 => { vec.clear(); }
                4 => if !vec.is_empty() { let _ = vec.remove(_to_usize(gdata, 900 + i as usize * 10) % vec.len()); },
                5 => {
                    let key = _to_str(gdata, 1000 + i as usize * 5, 1005 + i as usize * 5);
                    if let Some(v) = map.get(key) {
                        println!("{:?}", v);
                    }
                }
                6 => {
                    let key = _to_str(gdata, 1050 + i as usize * 5, 1055 + i as usize * 5);
                    map.remove(key);
                }
                7 => {
                    let slice = vec.deref();
                    println!("{:?}", slice);
                }
                8 => {
                    let key = _to_str(gdata, 1100 + i as usize * 5, 1105 + i as usize * 5);
                    if map.contains_key(key) {
                        println!("Key exists");
                    }
                }
                _ => {
                    if let Some((k, v)) = map.iter_mut().next() {
                        *v = String::from(_to_str(gdata, 1150 + i as usize * 5, 1155 + i as usize * 5));
                        println!("Mutated: {} -> {}", k, v);
                    }
                }
            }
            if i % 3 == 0 {
                println!("Capacity: {}", vec.capacity());
                let mut iter = map.iter();
                if let Some((k, v)) = iter.next() {
                    println!("First entry: {}: {}", k, v);
                }
            }
        }

        let _ = vec.as_ptr();
        if let Some((k, v)) = map.iter_mut().next() {
            println!("Final entry: {} -> {}", k, v);
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