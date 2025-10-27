#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use bumpalo::*;
use global_data::*;
use core::alloc::Layout;

#[derive(Debug)]
struct CustomType2(String);
#[derive(Debug)]
struct CustomType0(String);
#[derive(Debug)]
struct CustomType1(String);

impl core::iter::IntoIterator for CustomType0 {
    type Item = CustomType1;
    type IntoIter = CustomType2;
    
    fn into_iter(self) -> Self::IntoIter {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 49);
        let selector = (custom_impl_num + self.0.len()) % 3;
        if selector == 0 { panic!("INTENTIONAL PANIC!"); }
        let GLOBAL_DATA = match selector {
            1 => global_data.first_half,
            _ => global_data.second_half,
        };
        CustomType2(String::from(_to_str(GLOBAL_DATA, 57, 57 + (_to_u8(GLOBAL_DATA, 57) % 17) as usize)))
    }
}

impl core::iter::Iterator for CustomType2 {
    type Item = CustomType1;
    
    fn next(&mut self) -> Option<Self::Item> {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 24);
        let selector = (custom_impl_num + self.0.len()) % 3;
        if selector == 0 { panic!("INTENTIONAL PANIC!"); }
        let GLOBAL_DATA = match selector {
            1 => global_data.first_half,
            _ => global_data.second_half,
        };
        Some(CustomType1(String::from(_to_str(GLOBAL_DATA, 33, 33 + (_to_u8(GLOBAL_DATA,32) % 17) as usize))))
    }
}

impl core::iter::ExactSizeIterator for CustomType2 {
    fn len(&self) -> usize {
        let global_data = get_global_data();
        _to_usize(global_data.first_half, 8)
    }
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 1024 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let mut bump = match _to_u8(GLOBAL_DATA, 0) % 4 {
            0 => Bump::new(),
            1 => Bump::with_capacity(_to_usize(GLOBAL_DATA, 1)),
            2 => _unwrap_result(Bump::try_new()),
            3 => _unwrap_result(Bump::try_with_capacity(_to_usize(GLOBAL_DATA, 1))),
            _ => unreachable!(),
        };

        let mut offset = 16;
        for _ in 0..(_to_u8(GLOBAL_DATA, 8) % 10) {
            match _to_u8(GLOBAL_DATA, offset) % 6 {
                0 => {
                    let s = String::from(_to_str(GLOBAL_DATA, offset + 1, offset + 1 + (_to_u8(GLOBAL_DATA, offset + 1) % 32) as usize));
                    let _ = bump.alloc(s);
                }
                1 => {
                    let layout = Layout::new::<[u8; 256]>();
                    let _ = bump.alloc_layout(layout);
                }
                2 => {
                    let ptr = bump.alloc_str(_to_str(GLOBAL_DATA, offset + 1, offset + 33));
                    println!("{:?}", ptr);
                }
                3 => {
                    let _slice = bump.alloc_slice_fill_copy(64, _to_u8(GLOBAL_DATA, offset + 1));
                }
                4 => {
                    for chunk in bump.iter_allocated_chunks() {
                        println!("{:?}", chunk);
                    }
                }
                5 => {
                    let iter = CustomType0(String::from(_to_str(GLOBAL_DATA, offset + 1, offset + 33)));
                    let slice = bump.alloc_slice_fill_iter(iter);
                    println!("{:?}", slice);
                }
                _ => bump.reset()
            }
            offset = (offset + 64) % 256;
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