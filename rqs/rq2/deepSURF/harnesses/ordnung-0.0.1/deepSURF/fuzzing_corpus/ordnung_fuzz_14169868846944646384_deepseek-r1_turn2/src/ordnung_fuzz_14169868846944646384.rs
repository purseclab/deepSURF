#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use ordnung::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut};

#[derive(Debug)]
struct CustomType1(String);
struct CustomType0(String);
struct CustomType2(String);

impl core::iter::IntoIterator for CustomType0 {
    type Item = CustomType1;
    type IntoIter = CustomType2;
    
    fn into_iter(self) -> Self::IntoIter {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 25);
        let custom_impl_inst_num = self.0.len();
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0 {
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector {
            1 => global_data.first_half,
            _ => global_data.second_half,
        };
        let mut t_5 = _to_u8(GLOBAL_DATA, 33) % 17;
        let t_6 = _to_str(GLOBAL_DATA, 34, 34 + t_5 as usize);
        let t_7 = String::from(t_6);
        let t_8 = CustomType2(t_7);
        t_8
    }
}

impl core::iter::Iterator for CustomType2 {
    type Item = CustomType1;
    
    fn next(&mut self) -> Option<Self::Item> {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 0);
        let custom_impl_inst_num = self.0.len();
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0 {
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector {
            1 => global_data.first_half,
            _ => global_data.second_half,
        };
        let mut t_0 = _to_u8(GLOBAL_DATA, 8) % 17;
        let t_1 = _to_str(GLOBAL_DATA, 9, 9 + t_0 as usize);
        let t_2 = String::from(t_1);
        let t_3 = CustomType1(t_2);
        Some(t_3)
    }
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 500 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let mut ops = _to_usize(GLOBAL_DATA, 0) % 16;
        let mut map = ordnung::Map::new();
        let mut vecs = Vec::new();

        for _ in 0..(_to_usize(GLOBAL_DATA, 8) % 7) {
            let construct_sel = _to_u8(GLOBAL_DATA, 16) % 5;
            let vec = match construct_sel {
                0 => compact::Vec::new(),
                1 => compact::Vec::with_capacity(_to_usize(GLOBAL_DATA, 24) % 65),
                2 => compact::Vec::from_iter(std::iter::repeat_with(|| {
                    let t = _to_str(GLOBAL_DATA, 32 + _to_u8(GLOBAL_DATA, 40) as usize, 48);
                    CustomType1(t.to_string())
                }).take(_to_usize(GLOBAL_DATA, 50) % 65)),
                3 => {
                    let raw = _to_u8(GLOBAL_DATA, 60);
                    let mut v = compact::Vec::new();
                    for _ in 0..(raw % 16) {
                        let elem = _to_str(GLOBAL_DATA, 80 + _to_usize(GLOBAL_DATA, 90) % 32, 112);
                        v.push(CustomType1(elem.to_string()));
                    }
                    v
                }
                _ => compact::Vec::from_iter(CustomType0(_to_str(GLOBAL_DATA, 120, 128).to_string()))
            };
            vecs.push(vec);
        }

        while ops > 0 {
            let op = _to_u8(GLOBAL_DATA, 200 + ops as usize) % 12;
            match op {
                0 => {
                    let key_part = _to_str(GLOBAL_DATA, 300 + ops * 10, 340);
                    map.insert(key_part.to_string(), ops);
                }
                1 => {
                    if let Some(vec) = vecs.get_mut(0) {
                        let elem = _to_str(GLOBAL_DATA, 400 + ops * 5, 440);
                        vec.push(CustomType1(elem.to_string()));
                    }
                }
                2 => {
                    if let Some(vec) = vecs.get_mut(1) {
                        let _ = vec.pop();
                    }
                }
                3 => {
                    let slice = match vecs.get_mut(0) {
                        Some(v) => v.deref_mut(),
                        None => continue,
                    };
                    println!("Deref slice len: {}", slice.len());
                }
                4 => {
                    for entry in map.iter_mut() {
                        *entry.1 += 1;
                        println!("Entry value: {}", entry.1);
                    }
                }
                5 => {
                    if let Some(vec) = vecs.get_mut(1) {
                        println!("Capacity: {}", vec.capacity());
                    }
                }
                6 => {
                    let key = _to_str(GLOBAL_DATA, 500, 520);
                    let _ = map.contains_key(key);
                }
                7 => {
                    let key = _to_str(GLOBAL_DATA, 550, 570);
                    let _ = map.get_mut(key);
                }
                8 => {
                    if let Some(vec) = vecs.get_mut(2) {
                        vec.clear();
                    }
                }
                9 => {
                    map.clear();
                }
                10 => {
                    let index = _to_usize(GLOBAL_DATA, 600) % vecs.len();
                    if let Some(vec) = vecs.get_mut(index) {
                        let _ = vec.len();
                    }
                }
                _ => {}
            }
            ops -= 1;
        }

        for vec in vecs.iter_mut() {
            let slice = vec.deref_mut();
            if !slice.is_empty() {
                println!("Final elem: {:?}", &slice[0]);
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