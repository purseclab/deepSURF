#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use ordnung::*;
use global_data::*;
use std::ops::Deref;

struct CustomType0(String);
#[derive(Debug)]
struct CustomType1(String);
struct CustomType2(String);

impl Iterator for CustomType2 {
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
        let t_3 = CustomType1(t_1.to_string());
        Some(t_3)
    }
}

impl IntoIterator for CustomType0 {
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
        CustomType2(t_6.to_string())
    }
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 1024 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let ops = _to_u8(global_data.first_half, 0) % 8;

        for i in 0..ops {
            let selector = _to_u8(global_data.second_half, i as usize) % 6;
            match selector {
                0 => {
                    let cap = _to_usize(global_data.first_half, 10);
                    let vec = ordnung::compact::Vec::<String>::with_capacity(cap);
                    println!("{:?}", vec.deref());
                }
                1 => {
                    let mut map = ordnung::Map::new();
                    let key_len = _to_u8(global_data.second_half, 20) % 65;
                    let key = _to_str(global_data.second_half, 21, 21 + key_len as usize);
                    let val = _to_usize(global_data.first_half, 30);
                    map.insert(key.to_string(), val);
                    println!("{:?}", map.get(key));
                }
                2 => {
                    let mut vec = ordnung::compact::Vec::new();
                    let elem_len = _to_u8(global_data.first_half, 40) % 17;
                    let elem = _to_str(global_data.first_half, 41, 41 + elem_len as usize);
                    vec.push(elem.to_string());
                    let popped = vec.pop();
                    println!("{:?}", popped);
                }
                3 => {
                    let mut t_9 = _to_u8(global_data.first_half, 50) % 17;
                    let t_10 = _to_str(global_data.first_half, 51, 51 + t_9 as usize);
                    let t_12 = CustomType0(t_10.to_string());
                    let vec = ordnung::compact::Vec::from_iter(t_12);
                    println!("{:?}", vec.deref());
                }
                4 => {
                    let vec_src = vec![_to_usize(global_data.second_half, 60)];
                    let vec = ordnung::compact::Vec::from(vec_src);
                    let cloned = vec.clone();
                    println!("{}", vec.eq(&cloned));
                }
                5 => {
                    let mut map = ordnung::Map::<String, usize>::with_capacity(_to_usize(global_data.first_half, 70));
                    let key_len = _to_u8(global_data.second_half, 80) % 65;
                    let key = _to_str(global_data.second_half, 81, 81 + key_len as usize);
                    map.insert(key.to_string(), _to_usize(global_data.first_half, 90));
                    let mut iter = map.iter_mut();
                    let val = iter.next();
                    println!("{:?}", val);
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