#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use stack_dst::*;
use global_data::*;
use std::any::Any;
use std::fmt::Debug;
use std::ops::{Deref, DerefMut};

#[derive(Debug)]
struct CustomType0(String);

#[derive(Clone, Copy)]
struct CustomType1(usize);

impl std::convert::AsRef<dyn Any> for CustomType1 {
    fn as_ref(&self) -> &dyn Any {
        let global_data = get_global_data();
        let selector = (self.0 + _to_usize(global_data.first_half, 0)) % 3;
        if selector == 0 {
            panic!("INTENTIONAL PANIC!");
        }
        let data_part = match selector {
            1 => global_data.first_half,
            _ => global_data.second_half,
        };
        &self.0
    }
}

impl std::default::Default for CustomType1 {
    fn default() -> Self {
        let global_data = get_global_data();
        if _to_u8(global_data.first_half, 66) % 2 == 0 {
            panic!("INTENTIONAL PANIC!");
        }
        CustomType1(_to_usize(global_data.first_half, 67))
    }
}

impl std::clone::Clone for CustomType0 {
    fn clone(&self) -> Self {
        let global_data = get_global_data();
        let selector = (self.0.len() + _to_usize(global_data.first_half, 76)) % 3;
        if selector == 0 {
            panic!("INTENTIONAL PANIC!");
        }
        let data_part = match selector {
            1 => global_data.first_half,
            _ => global_data.second_half,
        };
        CustomType0(self.0.clone())
    }
}

impl std::convert::AsMut<dyn Any> for CustomType1 {
    fn as_mut(&mut self) -> &mut dyn Any {
        let global_data = get_global_data();
        let selector = (self.0 + _to_usize(global_data.first_half, 41)) % 3;
        if selector == 0 {
            panic!("INTENTIONAL PANIC!");
        }
        let data_part = match selector {
            1 => global_data.first_half,
            _ => global_data.second_half,
        };
        &mut self.0
    }
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 4096 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        
        let mut stack_str = stack_dst::StackA::<str, [usize; 9]>::new();
        let mut stack_slice = stack_dst::StackA::<[CustomType0], [usize; 9]>::new();
        let mut value_store = Vec::new();
        
        let num_operations = _to_u8(GLOBAL_DATA, 0) % 8 + 8;
        let mut data_cursor = 1;

        for _ in 0..num_operations {
            if data_cursor >= GLOBAL_DATA.len() { break; }
            let op_select = GLOBAL_DATA[data_cursor] % 7;
            data_cursor += 1;

            match op_select {
                0 => {
                    if let Some(top) = stack_slice.top() {
                        println!("{:?}", top);
                    }
                    let _ = stack_slice.push_cloned(&value_store[..]);
                },
                1 => {
                    let val = _to_u64(GLOBAL_DATA, data_cursor);
                    data_cursor += 8;
                    let mut value_store_local = Vec::new();
                    for _ in 0..(_to_u8(GLOBAL_DATA, data_cursor) % 65) {
                        data_cursor += 1;
                        let s_len = _to_u8(GLOBAL_DATA, data_cursor) % 17;
                        data_cursor += 1;
                        let s = _to_str(GLOBAL_DATA, data_cursor, data_cursor + s_len as usize);
                        value_store_local.push(CustomType0(s.to_string()));
                        data_cursor += s_len as usize;
                    }
                    let _ = stack_slice.push_cloned(&value_store_local[..]);
                },
                2 => {
                    let start = _to_usize(GLOBAL_DATA, data_cursor);
                    data_cursor += 8;
                    let end = _to_usize(GLOBAL_DATA, data_cursor);
                    data_cursor += 8;
                    let s = _to_str(GLOBAL_DATA, start, end);
                    let _ = stack_str.push_str(s);
                },
                3 => {
                    if let Some(top) = stack_slice.top_mut() {
                        let new_len = _to_u8(GLOBAL_DATA, data_cursor) % 17;
                        data_cursor += 1;
                        let new_str = _to_str(GLOBAL_DATA, data_cursor, data_cursor + new_len as usize);
                        data_cursor += new_len as usize;
                        if !top.is_empty() {
                            top[0] = CustomType0(new_str.to_string());
                        }
                    }
                },
                4 => {
                    let vec_size = _to_u8(GLOBAL_DATA, data_cursor) % 65;
                    data_cursor += 1;
                    let mut temp_vec = Vec::with_capacity(vec_size as usize);
                    for _ in 0..vec_size {
                        let s_len = _to_u8(GLOBAL_DATA, data_cursor) % 17;
                        data_cursor += 1;
                        let slice = _to_str(GLOBAL_DATA, data_cursor, data_cursor + s_len as usize);
                        data_cursor += s_len as usize;
                        temp_vec.push(CustomType0(slice.to_string()));
                    }
                    let _ = stack_slice.push_cloned(&temp_vec);
                },
                5 => {
                    if let Some(top_ref) = stack_slice.top() {
                        println!("{:?}", top_ref);
                    }
                    stack_slice.pop();
                    if let Some(top_ref) = stack_str.top() {
                        println!("{:?}", top_ref);
                    }
                    stack_str.pop();
                },
                6 => {
                    let mut waker_store = Vec::new();
                    for _ in 0..(_to_u8(GLOBAL_DATA, data_cursor) % 65) {
                        data_cursor += 1;
                        let val = _to_u64(GLOBAL_DATA, data_cursor);
                        data_cursor += 8;
                        if let Ok(value) = ValueA::<dyn Debug, [usize;9]>::new_stable(val, |x| x as &dyn Debug) {
                            println!("{:?}", value.deref());
                            waker_store.push(value);
                        }
                    }
                },
                _ => ()
            }

            if _to_u8(GLOBAL_DATA, data_cursor) % 3 == 0 {
                let _ = stack_slice.push_cloned(&value_store[..]);
                data_cursor += 1;
            }
        }

        let final_str_len = _to_u8(GLOBAL_DATA, data_cursor) % 65;
        data_cursor += 1;
        if data_cursor + final_str_len as usize <= GLOBAL_DATA.len() {
            let s = _to_str(GLOBAL_DATA, data_cursor, data_cursor + final_str_len as usize);
            let _ = stack_str.push_str(s);
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