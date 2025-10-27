#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

struct CustomType1(String);

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 1024 {return;}
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        
        let op_count = _to_u8(GLOBAL_DATA, 0) % 10;
        let mut offset = 1;

        for _ in 0..op_count {
            if offset >= GLOBAL_DATA.len() { break; }
            let op = _to_u8(GLOBAL_DATA, offset) % 6;
            offset += 1;

            match op {
                0 => {
                    let cap = _to_usize(GLOBAL_DATA, offset);
                    offset += 8;
                    let mut sv = SmallVec::<[String; 64]>::with_capacity(cap);
                    let pushes = _to_u8(GLOBAL_DATA, offset) % 65;
                    offset += 1;
                    for _ in 0..pushes {
                        if offset >= GLOBAL_DATA.len() { break; }
                        let l = _to_u8(GLOBAL_DATA, offset) % 65;
                        offset += 1;
                        sv.push(_to_str(GLOBAL_DATA, offset, offset + l as usize).to_string());
                        offset += l as usize;
                    }
                    let s = sv.as_slice();
                    println!("{:?}", s);
                    sv.insert(_to_usize(GLOBAL_DATA, offset) % (sv.len() + 1), "".to_string());
                }
                1 => {
                    let count = _to_u8(GLOBAL_DATA, offset) % 65;
                    offset += 1;
                    let mut items = vec![];
                    for _ in 0..count {
                        if offset >= GLOBAL_DATA.len() { break; }
                        let l = _to_u8(GLOBAL_DATA, offset) % 65;
                        offset += 1;
                        items.push(_to_str(GLOBAL_DATA, offset, offset + l as usize).to_string());
                        offset += l as usize;
                    }
                    let mut sv = SmallVec::<[String; 64]>::from_vec(items);
                    let s = sv.as_mut_slice();
                    s[_to_usize(GLOBAL_DATA, offset) % s.len()] = "X".to_string();
                }
                2 => {
                    let mut sv = SmallVec::<[String; 64]>::new();
                    let inserts = _to_u8(GLOBAL_DATA, offset) % 65;
                    offset += 1;
                    for _ in 0..inserts {
                        if offset >= GLOBAL_DATA.len() { break; }
                        let l = _to_u8(GLOBAL_DATA, offset) % 65;
                        offset += 1;
                        sv.insert(sv.len(), _to_str(GLOBAL_DATA, offset, offset + l as usize).to_string());
                        offset += l as usize;
                    }
                    let _ = sv.drain(.._to_usize(GLOBAL_DATA, offset) % (sv.len() + 1));
                    println!("{:?}", sv.as_slice());
                }
                3 => {
                    let vec_size = _to_u8(GLOBAL_DATA, offset) % 65;
                    offset += 1;
                    let mut source = vec![];
                    for _ in 0..vec_size {
                        if offset >= GLOBAL_DATA.len() { break; }
                        let l = _to_u8(GLOBAL_DATA, offset) % 65;
                        offset += 1;
                        source.push(_to_str(GLOBAL_DATA, offset, offset + l as usize).to_string());
                        offset += l as usize;
                    }
                    let sv = SmallVec::<[String; 64]>::from_iter(source.into_iter());
                    let _ = sv.as_ptr();
                    let _ = sv.into_iter().next();
                }
                4 => {
                    let slice_len = _to_usize(GLOBAL_DATA, offset) % 65;
                    offset += 8;
                    let mut sv = SmallVec::<[String; 64]>::from_vec(vec!["a".repeat(slice_len)]);
                    sv.extend(vec!["b".repeat(_to_usize(GLOBAL_DATA, offset) % 65)]);
                    let _ = sv.as_slice();
                }
                5 => {
                    let mut sv1 = SmallVec::<[String; 64]>::new();
                    let mut sv2 = SmallVec::<[String; 64]>::new();
                    let elems = _to_u8(GLOBAL_DATA, offset) % 65;
                    offset += 1;
                    for _ in 0..elems {
                        if offset >= GLOBAL_DATA.len() { break; }
                        let l = _to_u8(GLOBAL_DATA, offset) % 65;
                        offset += 1;
                        let s = _to_str(GLOBAL_DATA, offset, offset + l as usize).to_string();
                        sv1.push(s.clone());
                        sv2.push(s);
                        offset += l as usize;
                    }
                    let _ = sv1.cmp(&sv2);
                    sv1.append(&mut sv2);
                    let _ = sv1.partial_cmp(&sv1);
                }
                _ => (),
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