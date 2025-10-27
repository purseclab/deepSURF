#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use bumpalo::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};
use core::alloc::Layout;

#[derive(Debug)]
struct CustomType0(String);

impl core::clone::Clone for CustomType0 {
    fn clone(&self) -> Self {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 16);
        let custom_impl_inst_num = self.0.len();
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0 {
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector {
            1 => global_data.first_half,
            _ => global_data.second_half,
        };
        let mut t_5 = _to_u8(GLOBAL_DATA, 24) % 17;
        let t_6 = _to_str(GLOBAL_DATA, 25, 25 + t_5 as usize);
        let t_7 = String::from(t_6);
        let t_8 = CustomType0(t_7);
        return t_8;
    }
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 1024 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let mut idx = 0;

        let constructor_sel = _to_u8(GLOBAL_DATA, idx) % 4;
        idx += 1;
        let mut bump = match constructor_sel {
            0 => Bump::new(),
            1 => {
                let cap = _to_usize(GLOBAL_DATA, idx);
                idx += 8;
                Bump::with_capacity(cap)
            },
            2 => _unwrap_result(Bump::try_new()),
            3 => {
                let cap = _to_usize(GLOBAL_DATA, idx);
                idx += 8;
                _unwrap_result(Bump::try_with_capacity(cap))
            },
            _ => unreachable!(),
        };

        let n_ops = _to_u8(GLOBAL_DATA, idx) % 5 + 1;
        idx += 1;

        for _ in 0..n_ops {
            if idx >= GLOBAL_DATA.len() { break; }
            let op_sel = _to_u8(GLOBAL_DATA, idx) % 6;
            idx += 1;
            match op_sel {
                0 => {
                    let len = _to_usize(GLOBAL_DATA, idx) % 65;
                    idx += 8;
                    let str_len = _to_u8(GLOBAL_DATA, idx) % 16;
                    idx += 1;
                    let start = idx;
                    let end = start + str_len as usize;
                    if end > GLOBAL_DATA.len() { continue; }
                    let s = _to_str(GLOBAL_DATA, start, end);
                    idx = end;
                    let elem = CustomType0(s.to_string());
                    let slice = bump.alloc_slice_fill_clone(len, &elem);
                    println!("Slice: {:?} ({})", slice, elem.0);
                },
                1 => {
                    let str_len = _to_u8(GLOBAL_DATA, idx) % 16;
                    idx += 1;
                    let start = idx;
                    let end = start + str_len as usize;
                    if end > GLOBAL_DATA.len() { continue; }
                    let s = _to_str(GLOBAL_DATA, start, end);
                    idx = end;
                    let _ = bump.alloc_str(s);
                    println!("String: {}", s);
                },
                2 => {
                    let len = _to_u8(GLOBAL_DATA, idx) % 65;
                    idx += 1;
                    let start = idx;
                    let mut end = start + len as usize;
                    if end > GLOBAL_DATA.len() { end = GLOBAL_DATA.len(); }
                    let src = &GLOBAL_DATA[start..end];
                    idx = end;
                    let slice = bump.alloc_slice_clone(src);
                    println!("Cloned slice: {:?}", slice);
                },
                3 => {
                    let size = _to_usize(GLOBAL_DATA, idx);
                    idx += 8;
                    let align = 1 << (_to_u8(GLOBAL_DATA, idx) % 4);
                    idx += 1;
                    if let Ok(layout) = Layout::from_size_align(size, align) {
                        let _ = bump.try_alloc_layout(layout).unwrap();
                    }
                },
                4 => {
                    let _ = bump.allocated_bytes();
                    let mut chunks = bump.iter_allocated_chunks();
                    while let Some(chunk) = chunks.next() {
                        println!("Chunk: {:?}", chunk);
                    }
                },
                5 => {
                    let len = _to_usize(GLOBAL_DATA, idx) % 65;
                    idx += 8;
                    let value = _to_u8(GLOBAL_DATA, idx);
                    idx += 1;
                    let slice = bump.alloc_slice_fill_copy(len as usize, value);
                    println!("Filled copy: {:?}", slice);
                },
                _ => (),
            }
        }

        let len = _to_usize(GLOBAL_DATA, idx) % 65;
        idx += 8;
        let str_len = _to_u8(GLOBAL_DATA, idx) % 16;
        idx += 1;
        let start = idx;
        let end = start + str_len as usize;
        if end > GLOBAL_DATA.len() { return; }
        let s = _to_str(GLOBAL_DATA, start, end);
        let elem = CustomType0(s.to_string());
        bump.alloc_slice_fill_clone(len, &elem);
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