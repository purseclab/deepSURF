#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

fn main (){
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 256 {return;}
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let num_ops = _to_u8(GLOBAL_DATA, 0) % 8 + 1;
        let mut offset = 1;
        let mut buffers = Vec::new();

        for _ in 0..num_ops {
            let constructor = _to_u8(GLOBAL_DATA, offset) % 5;
            offset += 1;

            match constructor {
                0 => {
                    let cap = _to_usize(GLOBAL_DATA, offset);
                    offset += 8;
                    let mut sv = SmallVec::with_capacity(cap);
                    let pushes = _to_u8(GLOBAL_DATA, offset) % 32;
                    offset += 1;
                    for _ in 0..pushes {
                        let s_len = _to_usize(GLOBAL_DATA, offset) % 16;
                        offset += s_len;
                        let s = _to_str(GLOBAL_DATA, offset - s_len, offset);
                        sv.push(s.to_string());
                    }
                    sv.truncate(_to_usize(GLOBAL_DATA, offset));
                    offset += 8;
                    buffers.push(sv);
                }
                1 => {
                    let count = _to_usize(GLOBAL_DATA, offset) % 65;
                    offset += 8;
                    let mut vec = Vec::with_capacity(count);
                    for _ in 0..count {
                        let s_len = _to_usize(GLOBAL_DATA, offset) % 16;
                        offset += s_len;
                        let s = _to_str(GLOBAL_DATA, offset - s_len, offset);
                        vec.push(s.to_string());
                    }
                    let mut sv = SmallVec::from_vec(vec);
                    sv.shrink_to_fit();
                    buffers.push(sv);
                }
                2 => {
                    let len = _to_usize(GLOBAL_DATA, offset) % 64;
                    offset += 8;
                    let arr: [String; 64] = core::array::from_fn(|i| {
                        let base = offset + i * 10;
                        let s = _to_str(GLOBAL_DATA, base, base + 10);
                        s.to_string()
                    });
                    let sv = SmallVec::from_buf_and_len(arr, len);
                    offset += 640;
                    buffers.push(sv);
                }
                3 => {
                    let slice_len = _to_usize(GLOBAL_DATA, offset) % 32;
                    offset += 8;
                    let mut items = Vec::new();
                    for _ in 0..slice_len {
                        let s_len = _to_usize(GLOBAL_DATA, offset) % 16;
                        offset += s_len;
                        let s = _to_str(GLOBAL_DATA, offset - s_len, offset);
                        items.push(s.to_string());
                    }
                    let sv = SmallVec::from_vec(items);
                    buffers.push(sv);
                }
                4 => {
                    let cap = _to_usize(GLOBAL_DATA, offset);
                    offset += 8;
                    let elem = String::from("filler");
                    let sv = SmallVec::from_elem(elem, cap);
                    sv.as_ptr();
                    buffers.push(sv);
                }
                _ => unreachable!()
            }
        }

        for sv in &mut buffers {
            let ptr = sv.as_ptr();
            println!("{:?}", ptr);
            if sv.len() > 0 {
                sv.get(_to_usize(GLOBAL_DATA, offset) % sv.len()).map(|x| println!("{}", x));
                sv.as_mut_ptr();
            }
        }

        for mut sv in buffers {
            if !sv.is_empty() {
                sv.insert(0, String::from("inserted"));
                sv.remove(sv.len() - 1);
                sv.as_slice().iter().for_each(|x| println!("{}", x));
            }
            let _ = sv.capacity();
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