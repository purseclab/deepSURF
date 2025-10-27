#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use bumpalo::*;
use core::alloc::Layout;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 1024 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        
        let constructor_choice = _to_u8(GLOBAL_DATA, 0) % 4;
        let capacity = _to_usize(GLOBAL_DATA, 1);
        let mut bump = match constructor_choice {
            0 => Bump::new(),
            1 => _unwrap_result(Bump::try_new()),
            2 => Bump::with_capacity(capacity),
            _ => _unwrap_result(Bump::try_with_capacity(capacity)),
        };

        let num_ops = _to_u8(GLOBAL_DATA, 9) % 16;
        let mut offset = 10;

        for _ in 0..num_ops {
            if offset + 1 > GLOBAL_DATA.len() { break; }
            let op_type = _to_u8(GLOBAL_DATA, offset) % 6;
            offset += 1;

            match op_type {
                0 => {
                    if offset + 4 > GLOBAL_DATA.len() { break; }
                    let val = _to_u32(GLOBAL_DATA, offset);
                    offset += 4;
                    let x = bump.alloc(val);
                    println!("{:?}", x);
                }
                1 => {
                    if offset + 1 > GLOBAL_DATA.len() { break; }
                    let len = (_to_u8(GLOBAL_DATA, offset) % 65) as usize;
                    offset += 1;
                    let end = offset + len;
                    if end > GLOBAL_DATA.len() { break; }
                    let s = _to_str(GLOBAL_DATA, offset, end);
                    let x = bump.alloc_str(s);
                    println!("{}", x);
                    offset = end;
                }
                2 => {
                    if offset + 16 > GLOBAL_DATA.len() { break; }
                    let size = _to_usize(GLOBAL_DATA, offset);
                    offset += 8;
                    let align = _to_usize(GLOBAL_DATA, offset);
                    offset += 8;
                    let layout = Layout::from_size_align(size, align).unwrap_or_else(|_| Layout::new::<u8>());
                    let ptr = _unwrap_result(bump.try_alloc_layout(layout));
                    println!("{:?}", ptr);
                }
                3 => {
                    let mut chunks = bump.iter_allocated_chunks();
                    while let Some(chunk) = chunks.next() {
                        println!("Chunk: {:?}", chunk.as_ptr());
                    }
                }
                4 => {
                    if offset + 1 > GLOBAL_DATA.len() { break; }
                    let len = (_to_u8(GLOBAL_DATA, offset) % 65) as usize;
                    offset += 1;
                    let slice = bump.alloc_slice_fill_default::<u8>(len);
                    println!("{:?}", slice);
                }
                _ => {
                    if offset + 4 > GLOBAL_DATA.len() { break; }
                    let val = _to_u32(GLOBAL_DATA, offset);
                    offset += 4;
                    let x = _unwrap_result(bump.try_alloc(val));
                    println!("{:?}", x);
                }
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