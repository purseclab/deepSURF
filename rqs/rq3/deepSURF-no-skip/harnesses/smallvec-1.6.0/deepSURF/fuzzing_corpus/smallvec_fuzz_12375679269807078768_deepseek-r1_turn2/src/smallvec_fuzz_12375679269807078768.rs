#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::{SmallVec, ToSmallVec};
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

struct CustomType1(String);

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 4096 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let mut smallvecs = Vec::new();
        let constructor_selector = _to_u8(GLOBAL_DATA, 0) % 4;

        match constructor_selector {
            0 => {
                let cap = _to_usize(GLOBAL_DATA, 1) % 65;
                let mut sv = SmallVec::<[String; 32]>::with_capacity(cap);
                for i in 0.._to_u8(GLOBAL_DATA, 2) % 65 {
                    let s = _to_str(GLOBAL_DATA, i as usize * 8, (i as usize + 1) * 8);
                    sv.push(s.to_string());
                }
                smallvecs.push(sv);
            }
            1 => {
                let elem = _to_str(GLOBAL_DATA, 1, 9).to_string();
                let count = _to_usize(GLOBAL_DATA, 9) % 65;
                let sv = SmallVec::from_elem(elem, count);
                smallvecs.push(sv);
            }
            2 => {
                let mut sv = SmallVec::new();
                let count = _to_u8(GLOBAL_DATA, 1) % 8;
                for i in 0..count {
                    let start = _to_usize(GLOBAL_DATA, i as usize * 8) % (GLOBAL_DATA.len().saturating_sub(8));
                    let s = _to_str(GLOBAL_DATA, start, start + 8);
                    sv.push(s.to_string());
                }
                smallvecs.push(sv);
            }
            3 => {
                let count = _to_u8(GLOBAL_DATA, 1) % 4;
                let elements = (0..count).map(|i| (i as u8 + b'a') as char).map(|c| c.to_string());
                let sv = SmallVec::from_iter(elements);
                smallvecs.push(sv);
            }
            _ => return,
        }

        for mut sv in &mut smallvecs {
            let ops = _to_usize(GLOBAL_DATA, 100) % 8;
            for _ in 0..ops {
                let op = _to_u8(GLOBAL_DATA, 200) % 6;
                match op {
                    0 => {
                        sv.shrink_to_fit();
                        let _ = sv.as_slice();
                        sv.push(String::from("fuzz"));
                    }
                    1 => {
                        let idx = _to_usize(GLOBAL_DATA, 300);
                        sv.insert(idx % (sv.len() + 1), String::from("insert"));
                    }
                    2 => {
                        let idx = _to_usize(GLOBAL_DATA, 400);
                        if !sv.is_empty() {
                            sv.remove(idx % sv.len());
                        }
                    }
                    3 => {
                        let items = vec![String::from("x"), String::from("y")];
                        sv.extend(items);
                    }
                    4 => {
                        let idx = _to_usize(GLOBAL_DATA, 500);
                        let s = _to_str(GLOBAL_DATA, idx, idx + 8);
                        *sv.index_mut(0) = s.to_string();
                    }
                    5 => {
                        let cap = _to_usize(GLOBAL_DATA, 600);
                        sv.reserve(cap);
                    }
                    _ => {}
                }
            }
        }

        for mut sv in smallvecs {
            sv.shrink_to_fit();
            let _ = sv.as_ptr();
            println!("{:?}", sv.capacity());
            let mut iter = sv.into_iter();
            while let Some(item) = iter.next() {
                println!("{:?}", item);
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