#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use cbox::*;
use global_data::*;
use std::ops::{Deref, DerefMut};

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 1024 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let first_half = global_data.first_half;
        let second_half = global_data.second_half;

        let num_ops = _to_usize(first_half, 0) % 32;
        let mut operations = vec![];

        for i in 0..num_ops {
            let op_selector = _to_u8(second_half, i as usize) % 7;
            operations.push(op_selector);
        }

        let mut cbox_store = vec![];

        for (idx, op) in operations.iter().enumerate() {
            match op {
                0 => {
                    let ptr_val = _to_usize(first_half, idx * 8 % first_half.len());
                    let cb = CBox::<str>::new(ptr_val as *mut i8);
                    cbox_store.push(cb);
                }
                1 => {
                    let start = idx * 2 % first_half.len();
                    let len = _to_u8(first_half, start) % 65;
                    let str_slice = _to_str(first_half, start + 1, start + 1 + (len as usize));
                    let cb = CBox::from(str_slice);
                    cbox_store.push(cb);
                }
                2 => {
                    if let Some(cb) = cbox_store.last() {
                        println!("{:?}", cb.deref());
                    }
                }
                3 => {
                    if cbox_store.len() > 1 {
                        let cb1 = &cbox_store[0];
                        let cb2 = &cbox_store[1];
                        cb1.deref().eq(cb2.deref());
                    }
                }
                4 => { 
                    if let Some(cb) = cbox_store.last_mut() {
                        let dummy_ptr = _to_usize(second_half, idx * 4) as *mut i8;
                        let other = CBox::<str>::new(dummy_ptr);
                        cb.deref().eq(other.deref());
                    }
                }
                5 => {
                    if let Some(cb) = cbox_store.last() {
                        let cloned = cb.clone();
                        cloned.deref().eq(cb.deref());
                    }
                }
                _ => {
                    if cbox_store.len() > 1 {
                        cbox_store[0].deref().eq(cbox_store[1].deref());
                    }
                }
            }
        }

        let t_pointer = _to_usize(first_half, 512) as *mut i8;
        let critical_ct = CBox::<str>::new(t_pointer);
        let panic_str = _to_str(second_half, 0, 16);
        critical_ct.deref().eq(panic_str);
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