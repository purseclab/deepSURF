#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::SmallVec;
use global_data::*;
use std::ops::{Deref, DerefMut};

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 1024 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let mut op_index = 0;

        // Dynamically choose constructor
        let constructor_choice = _to_u8(GLOBAL_DATA, op_index) % 3;
        op_index += 1;

        let mut sv = match constructor_choice {
            0 => {
                let capacity = _to_usize(GLOBAL_DATA, op_index);
                op_index += 8;
                SmallVec::<[u8; 16]>::with_capacity(capacity)
            },
            1 => {
                let slice_start = _to_usize(GLOBAL_DATA, op_index);
                op_index += 8;
                let slice_end = slice_start + (_to_usize(GLOBAL_DATA, op_index) % 16); // limit slice size modulo 65 (direction 7c)
                let slice = &GLOBAL_DATA[slice_start..slice_end];
                SmallVec::from_slice(slice)
            },
            2 => SmallVec::<[u8; 16]>::new(),
            _ => unreachable!(),
        };

        // Execute a number of operations
        let num_ops = _to_usize(GLOBAL_DATA, op_index) % 10;
        op_index += 8;

        for _ in 0..num_ops {
            if op_index + 1 > GLOBAL_DATA.len() { break; }
            let op = _to_u8(GLOBAL_DATA, op_index) % 6;
            op_index += 1;

            match op {
                0 => { // push
                    let val = _to_u8(GLOBAL_DATA, op_index);
                    op_index += 1;
                    sv.push(val);
                    println!("Pushed: {}", val);
                },
                1 => { // pop
                    if let Some(val) = sv.pop() {
                        println!("Popped: {}", val);
                    }
                },
                2 => { // insert
                    let idx = _to_usize(GLOBAL_DATA, op_index);
                    op_index += 8;
                    let val = _to_u8(GLOBAL_DATA, op_index);
                    op_index += 1;
                    sv.insert(idx, val);
                },
                3 => { // try_reserve (target URAPI)
                    let additional = _to_usize(GLOBAL_DATA, op_index);
                    op_index += 8;
                    let _ = sv.try_reserve(additional);
                    println!("After reserve: {:?}", sv.as_slice());
                },
                4 => { // truncate
                    let len = _to_usize(GLOBAL_DATA, op_index) % (sv.len() + 1);
                    op_index += 8;
                    sv.truncate(len);
                },
                5 => { // drain
                    let start = _to_usize(GLOBAL_DATA, op_index) % (sv.len() + 1);
                    op_index += 8;
                    let end = _to_usize(GLOBAL_DATA, op_index) % (sv.len() + 1);
                    op_index += 8;
                    if start <= end {
                        let _ = sv.drain(start..end);
                    }
                },
                _ => unreachable!(),
            }
        }

        // Perform the target URAPI call
        let additional = _to_usize(GLOBAL_DATA, op_index);
        let _ = sv.try_reserve(additional);
        println!("Final: {:?}", sv.as_slice());
    });
}

// ... existing conversion functions ...

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