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
        if data.len() < 300 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let op_count = _to_usize(GLOBAL_DATA, 0) % 7;
        let mut sv_vec: Vec<SmallVec<[String; 16]>> = Vec::new();

        for i in 0..op_count {
            let constructor_type = _to_u8(GLOBAL_DATA, i*4) % 5;
            let mem_offset = i*16;

            match constructor_type {
                0 => {
                    sv_vec.push(SmallVec::new());
                },
                1 => {
                    let capacity = _to_usize(GLOBAL_DATA, mem_offset);
                    sv_vec.push(SmallVec::with_capacity(capacity));
                },
                2 => {
                    let elem_count = _to_usize(GLOBAL_DATA, mem_offset) % 65;
                    let str_part = _to_str(GLOBAL_DATA, mem_offset+8, mem_offset+16);
                    sv_vec.push(SmallVec::from_elem(str_part.to_string(), elem_count));
                },
                3 => {
                    let s1 = _to_str(GLOBAL_DATA, mem_offset, mem_offset+8);
                    let s2 = _to_str(GLOBAL_DATA, mem_offset+8, mem_offset+16);
                    let arr = [s1.to_string(), s2.to_string()];
                    sv_vec.push(SmallVec::from_iter(arr));
                },
                4 => {
                    let v: Vec<String> = (0..3).map(|j| {
                        let start = mem_offset + j*5;
                        _to_str(GLOBAL_DATA, start, start+5).to_string()
                    }).collect();
                    sv_vec.push(SmallVec::from_vec(v));
                },
                _ => {}
            }
        }

        let mut merged: SmallVec<[String; 16]> = SmallVec::new();
        for sv in &mut sv_vec {
            merged.append(sv);
            let cap_info = _to_usize(GLOBAL_DATA, sv.as_ptr() as usize % GLOBAL_DATA.len());
            sv.reserve(cap_info);
            println!("Capacity after reserve: {:?}", sv.capacity());

            let idx = _to_usize(GLOBAL_DATA, sv.len() % GLOBAL_DATA.len());
            if let Some(deref_val) = sv.get(idx) {
                println!("Dereferenced: {:?}", *deref_val);
            }

            if sv.len() % 3 == 0 {
                let mut drainer = sv.drain(1..sv.len().saturating_sub(1));
                while let Some(item) = drainer.next() {
                    println!("Draining: {}", item);
                }
            }
        }

        let final_reserve = _to_usize(GLOBAL_DATA, 250);
        merged.reserve(final_reserve);
        match merged.as_mut_slice().get_mut(0) {
            Some(val) => *val = _to_str(GLOBAL_DATA, 260, 270).to_string(),
            None => {}
        }
        let _ = merged.into_vec();
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