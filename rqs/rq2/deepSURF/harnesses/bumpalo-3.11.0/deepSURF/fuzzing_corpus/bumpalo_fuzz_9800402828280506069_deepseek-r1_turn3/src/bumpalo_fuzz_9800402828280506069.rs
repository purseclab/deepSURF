#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use bumpalo::*;
use core::alloc::Layout;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

#[derive(Debug)]
struct CustomType0(String);

fn _custom_fn0(id: usize) -> CustomType0 {
    let global_data = get_global_data();
    let GLOBAL_DATA = global_data.first_half;
    let t_5 = _to_u8(GLOBAL_DATA, 16);
    if t_5 % 2 == 0 {
        panic!("INTENTIONAL PANIC!");
    }
    let mut t_6 = _to_u8(GLOBAL_DATA, 17) % 17;
    let t_7 = _to_str(GLOBAL_DATA, 18, 18 + t_6 as usize);
    CustomType0(String::from(t_7))
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 128 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let mut bump = match _to_u8(GLOBAL_DATA, 0) % 4 {
            0 => _unwrap_result(Bump::try_with_capacity(_to_usize(GLOBAL_DATA, 8))),
            1 => Bump::with_capacity(_to_usize(GLOBAL_DATA, 16)),
            2 => Bump::new(),
            _ => Bump::default(),
        };

        let ops_count = _to_u8(GLOBAL_DATA, 24) % 8;
        for i in 0..ops_count {
            let op_selector = _to_u8(GLOBAL_DATA, 25 + i as usize) % 6;
            match op_selector {
                0 => {
                    let slice = bump.alloc_slice_fill_copy(_to_usize(GLOBAL_DATA, 32) % 65, 42u8);
                    println!("{:?}", &*slice);
                }
                1 => {
                    let s = _to_str(GLOBAL_DATA, 40, 56);
                    let allocated = bump.alloc_str(s);
                    println!("{}", allocated);
                }
                2 => {
                    let layout = Layout::from_size_align(_to_usize(GLOBAL_DATA, 64), 8).unwrap();
                    let ptr = _unwrap_result(bump.try_alloc_layout(layout));
                    println!("{:p}", ptr.as_ptr());
                }
                3 => {
                    let mut chunks = bump.iter_allocated_chunks();
                    while let Some(chunk) = chunks.next() {
                        println!("Chunk: {:?}", chunk.as_ptr());
                    }
                }
                4 => {
                    let slice = bump.alloc_slice_fill_with(_to_usize(GLOBAL_DATA, 72) % 65, |idx| idx % 256);
                    println!("{:?}", &*slice);
                }
                _ => {
                    let val = bump.alloc_slice_fill_with(_to_usize(GLOBAL_DATA, 80) % 65, _custom_fn0);
                    println!("{:?}", &*val);
                }
            }
        }

        bump.alloc_slice_fill_with(_to_usize(GLOBAL_DATA, 96), |i| i as u8);
        println!("Allocated bytes: {}", bump.allocated_bytes());
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