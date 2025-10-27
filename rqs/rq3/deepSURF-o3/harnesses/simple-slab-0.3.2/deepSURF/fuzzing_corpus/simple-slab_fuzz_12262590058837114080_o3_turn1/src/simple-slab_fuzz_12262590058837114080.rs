#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use simple_slab::*;
use global_data::*;
use std::ops::{Deref, DerefMut, Index, IndexMut};
use std::str::FromStr;

#[derive(Clone, Debug)]
struct CustomType0(String);

fn build_string(src: &[u8], start: usize, len: usize) -> String {
    let end = std::cmp::min(start + len, src.len());
    _to_str(src, start, end).to_string()
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 64 {
            return;
        }
        set_global_data(data);
        let global = get_global_data();
        let GLOBAL_DATA = global.first_half;
        let capacity = _to_usize(GLOBAL_DATA, 0);
        let ctrl = _to_u8(GLOBAL_DATA, 8);
        let mut slab = if ctrl % 2 == 0 {
            Slab::<CustomType0>::new()
        } else {
            Slab::<CustomType0>::with_capacity(capacity)
        };

        let mut cursor = 16;
        while cursor + 16 < GLOBAL_DATA.len() {
            let op = _to_u8(GLOBAL_DATA, cursor) % 8;
            cursor += 1;
            match op {
                0 => {
                    let str_len = (_to_u8(GLOBAL_DATA, cursor) as usize % 10) + 1;
                    let s = build_string(GLOBAL_DATA, cursor + 1, str_len);
                    cursor += 1 + str_len;
                    slab.insert(CustomType0(s));
                }
                1 => {
                    if slab.len() > 0 {
                        let idx = _to_usize(GLOBAL_DATA, cursor) % slab.len();
                        cursor += 8;
                        let removed = slab.remove(idx);
                        println!("{:?}", removed);
                    } else {
                        cursor += 8;
                    }
                }
                2 => {
                    let mut it = slab.iter();
                    while let Some(item_ref) = it.next() {
                        println!("{:?}", *item_ref);
                    }
                }
                3 => {
                    let mut it_mut = slab.iter_mut();
                    while let Some(item_mut) = it_mut.next() {
                        let ch = _to_char(GLOBAL_DATA, cursor);
                        cursor += 4;
                        item_mut.0.push(ch);
                    }
                }
                4 => {
                    if slab.len() > 0 {
                        let idx = _to_usize(GLOBAL_DATA, cursor) % slab.len();
                        cursor += 8;
                        let r = &slab[idx];
                        println!("{:?}", *r);
                    } else {
                        cursor += 8;
                    }
                }
                5 => {
                    let l = slab.len();
                    println!("{:?}", l);
                }
                6 => {
                    let mut it_owned = (&slab).into_iter();
                    while let Some(r) = it_owned.next() {
                        println!("{:?}", *r);
                    }
                }
                _ => {}
            }
        }

        std::mem::drop(slab);

        let capacity2 = _to_usize(global.second_half, 0);
        let mut slab2 = Slab::<CustomType0>::with_capacity(capacity2);
        if global.second_half.len() > 8 {
            let s = build_string(global.second_half, 8, 12);
            slab2.insert(CustomType0(s));
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