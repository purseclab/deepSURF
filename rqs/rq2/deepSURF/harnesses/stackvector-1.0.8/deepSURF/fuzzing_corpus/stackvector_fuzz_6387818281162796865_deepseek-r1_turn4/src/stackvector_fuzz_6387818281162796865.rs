#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use stackvector::*;
use global_data::*;
use std::ops::{Index, IndexMut};

#[derive(Debug)]
struct CustomType1(String);
struct CustomType2(String);
struct CustomType3(String);

impl std::iter::IntoIterator for CustomType2 {
    type Item = CustomType1;
    type IntoIter = CustomType3;
    
    fn into_iter(self) -> Self::IntoIter {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let selector = _to_u8(GLOBAL_DATA, 0) % 3;
        if selector == 0 { panic!("INTENTIONAL PANIC!"); }
        let slice_size = _to_usize(GLOBAL_DATA, 1) % 65;
        let generated = String::from(_to_str(GLOBAL_DATA, 5, 5 + slice_size));
        CustomType3(generated)
    }
}

impl std::iter::Iterator for CustomType3 {
    type Item = CustomType1;
    
    fn next(&mut self) -> Option<Self::Item> {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let next_len = _to_usize(GLOBAL_DATA, 25) % 20;
        Some(CustomType1(String::from(_to_str(GLOBAL_DATA, 30, 30 + next_len))))
    }
}

impl std::clone::Clone for CustomType1 {
    fn clone(&self) -> Self {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.second_half;
        let cloned_slice = _to_str(GLOBAL_DATA, 50, 54);
        CustomType1(cloned_slice.to_string())
    }
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 2048 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let constructor_sel = _to_u8(GLOBAL_DATA, 0) % 3;
        let mut sv = match constructor_sel {
            0 => {
                let elem_count = _to_usize(GLOBAL_DATA, 1) % 65;
                let elem_str = _to_str(GLOBAL_DATA, 8, 24);
                StackVec::<[CustomType1; 64]>::from_elem(CustomType1(elem_str.to_string()), elem_count)
            },
            1 => {
                let slice_len = _to_usize(GLOBAL_DATA, 32) % 20;
                let items: Vec<_> = (0..slice_len).map(|i| {
                    let offset = 40 + i * 10;
                    CustomType1(String::from(_to_str(GLOBAL_DATA, offset, offset + 10)))
                }).collect();
                StackVec::from_vec(items)
            },
            _ => StackVec::<[CustomType1; 64]>::from_iter(CustomType2(String::new()))
        };

        let op_count = _to_u8(GLOBAL_DATA, 200) % 10;
        for i in 0..op_count {
            let op_byte = _to_u8(GLOBAL_DATA, 201 + i as usize) % 6;
            match op_byte {
                0 => {
                    let v = _to_usize(GLOBAL_DATA, 220 + (i as usize)*8);
                    sv.push(CustomType1(String::from(_to_str(GLOBAL_DATA, v, v + 8))));
                    println!("Pushed {:?}", sv.as_slice());
                },
                1 => {
                    let idx = _to_usize(GLOBAL_DATA, 240 + (i as usize)*8);
                    if !sv.is_empty() { 
                        let _ = sv.swap_remove(idx % sv.len());
                        println!("Swap removed at {}", idx);
                    }
                },
                2 => {
                    sv.clear();
                    println!("Cleared: {:?}", sv.as_slice());
                },
                3 => {
                    let pos = _to_usize(GLOBAL_DATA, 260 + (i as usize)*8);
                    let val = CustomType1(String::from(_to_str(GLOBAL_DATA, 280, 300)));
                    if pos <= sv.len() { 
                        sv.insert(pos, val); 
                        println!("Inserted at {}", pos);
                    }
                },
                4 => {
                    let new_len = _to_usize(GLOBAL_DATA, 300 + (i as usize)*8);
                    let val = CustomType1(String::from(_to_str(GLOBAL_DATA, 350, 370)));
                    sv.resize(new_len, val.clone());
                    println!("Resized to {} with {:?}", new_len, val);
                },
                _ => {
                    let _drained: Vec<_> = sv.drain().collect();
                    println!("Drained {} elements", _drained.len());
                }
            };
        }

        let resize_len = _to_usize(GLOBAL_DATA, 400);
        let resize_val = CustomType1(String::from(_to_str(GLOBAL_DATA, 408, 428)));
        sv.resize(resize_len, resize_val);
        println!("Final vector length: {}", sv.len());
        
        if let Ok(inner) = sv.into_inner() {
            println!("Converted to inner array: {:?}", inner);
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