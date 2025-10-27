#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use qwutils::*;
use qwutils::arc_slice::ArcSlice;
use global_data::*;
use std::ops::{Deref, DerefMut, Index, IndexMut};

#[derive(Debug)]
struct CustomType0(String);

fn _custom_fn0() -> CustomType0 {
    let global_data = get_global_data();
    let GLOBAL_DATA = global_data.first_half;
    let t_0 = _to_u8(GLOBAL_DATA, 0);
    if t_0 % 2 == 0 {
        panic!("INTENTIONAL PANIC!");
    }
    let t_1 = _to_u8(GLOBAL_DATA, 1) % 17;
    let t_2 = _to_str(GLOBAL_DATA, 2, 2 + t_1 as usize);
    let t_3 = String::from(t_2);
    CustomType0(t_3)
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 200 {
            return;
        }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let op_count = (_to_u8(GLOBAL_DATA, 0) % 8) as usize;
        let mut index = 1;
        let mut arc_slice: Option<ArcSlice<u8>> = None;

        for _ in 0..op_count {
            let op_type = _to_u8(GLOBAL_DATA, index) % 4;
            index += 1;

            match op_type {
                0 => {
                    let closure = || _custom_fn0();
                    let result: Option<CustomType0> = qwutils::if_type::if_type(closure);
                    println!("{:?}", result);
                }
                1 => {
                    let elem_count = _to_usize(GLOBAL_DATA, index) % 65;
                    index += 8;
                    let data_start = index;
                    let data_end = data_start + elem_count;
                    let vec = GLOBAL_DATA[data_start..data_end].to_vec();
                    let slice = ArcSlice::from(vec);
                    arc_slice = Some(slice.clone());
                    println!("{:?}", slice.as_ref());
                    let result: Option<ArcSlice<u8>> = qwutils::if_type::if_type(|| slice);
                    println!("{:?}", result);
                    index = data_end;
                }
                2 => {
                    if let Some(mut s) = arc_slice.take() {
                        let pos = _to_usize(GLOBAL_DATA, index);
                        index += 8;
                        let len = _to_usize(GLOBAL_DATA, index) % 65;
                        index += 8;
                        let data_start = index;
                        let data_end = data_start + len;
                        let data = &GLOBAL_DATA[data_start..data_end];
                        ArcSlice::insert_slice(&mut s, pos, data);
                        arc_slice = Some(s);
                        println!("{:?}", arc_slice.as_ref().unwrap().deref());
                        index = data_end;
                    }
                }
                3 => {
                    let mut vec = Vec::new();
                    let pos = _to_usize(GLOBAL_DATA, index);
                    index += 8;
                    let len = _to_usize(GLOBAL_DATA, index) % 65;
                    index += 8;
                    let data_start = index;
                    let data_end = data_start + len;
                    let data = &GLOBAL_DATA[data_start..data_end];
                    VecExt::insert_slice_clone(&mut vec, pos, data);
                    println!("{:?}", vec.as_slice());
                    index = data_end;
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