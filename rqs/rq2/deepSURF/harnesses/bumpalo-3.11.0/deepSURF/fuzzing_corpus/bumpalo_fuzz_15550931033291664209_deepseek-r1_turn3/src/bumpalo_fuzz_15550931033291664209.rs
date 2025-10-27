#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use bumpalo::*;
use global_data::*;
use std::alloc::Layout;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

struct CustomType0(String);

fn _custom_fn0() -> CustomType0 {
    let global_data = get_global_data();
    let GLOBAL_DATA = global_data.first_half;
    let t_4 = _to_u8(GLOBAL_DATA, 8);
    if t_4 % 2 == 0 {
        panic!("INTENTIONAL PANIC!");
    }
    let mut t_5 = _to_u8(GLOBAL_DATA, 9) % 17;
    let t_6 = _to_str(GLOBAL_DATA, 10, 10 + t_5 as usize);
    let t_7 = String::from(t_6);
    let t_8 = CustomType0(t_7);
    return t_8;
}

struct CustomType1(Vec<u8>);

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 512 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let constructor_selector = _to_u8(GLOBAL_DATA, 0) % 4;
        let mut bump = match constructor_selector {
            0 => Bump::new(),
            1 => _unwrap_result(Bump::try_new()),
            2 => Bump::with_capacity(_to_usize(GLOBAL_DATA, 1) % 1024),
            3 => _unwrap_result(Bump::try_with_capacity(_to_usize(GLOBAL_DATA, 2) % 2048)),
            _ => unreachable!(),
        };

        let op_count = _to_u8(GLOBAL_DATA, 3) % 8;
        for i in 0..op_count {
            let op_code = _to_u8(GLOBAL_DATA, 4 + i as usize) % 9;
            match op_code {
                0 => {
                    bump.alloc_with(|| _custom_fn0());
                }
                1 => {
                    let start = _to_u16(GLOBAL_DATA, 5 + i as usize) as usize;
                    let end = start + (_to_u8(GLOBAL_DATA, 7 + i as usize) % 65) as usize;
                    let s = _to_str(GLOBAL_DATA, start, end);
                    let allocated = bump.alloc_str(s);
                    println!("{}", allocated);
                }
                2 => {
                    let layout_size = _to_usize(GLOBAL_DATA, 10 + i as usize);
                    let layout_align = _to_usize(GLOBAL_DATA, 18 + i as usize);
                    let layout = _unwrap_result(Layout::from_size_align(layout_size, layout_align));
                    let ptr = _unwrap_result(bump.try_alloc_layout(layout));
                    println!("{:?}", ptr);
                }
                3 => {
                    let vec_len = (_to_u8(GLOBAL_DATA, 26 + i as usize) % 65) as usize;
                    let val = _to_u8(GLOBAL_DATA, 27 + i as usize);
                    let slice = bump.alloc_slice_fill_copy(vec_len, val);
                    println!("{:?}", slice);
                }
                4 => {
                    let chunk_iter = bump.iter_allocated_chunks();
                    for chunk in chunk_iter {
                        println!("{:?}", chunk);
                    }
                }
                5 => {
                    let layout = Layout::new::<u128>();
                    let ptr = bump.alloc_layout(layout);
                    println!("{:?}", ptr);
                }
                6 => {
                    let iter = bump.iter_allocated_chunks();
                    let count = iter.count();
                    if count > 0 && _to_u8(GLOBAL_DATA, 35 + i as usize) % 5 == 0 {
                        panic!("CHUNK ITER PANIC");
                    }
                }
                7 => {
                    bump.alloc_try_with(|| -> Result<CustomType1, ()> {
                        let len = _to_u8(GLOBAL_DATA, 40 + i as usize) % 65;
                        let mut v = Vec::with_capacity(len as usize);
                        v.extend_from_slice(&GLOBAL_DATA[50..50 + len as usize]);
                        Ok(CustomType1(v))
                    }).unwrap();
                }
                8 => {
                    let val = _to_u8(GLOBAL_DATA, 100 + i as usize);
                    let data = bump.alloc(val);
                    if *data == val % 2 {
                        panic!("ODD ALLOC PANIC");
                    }
                }
                _ => unreachable!()
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