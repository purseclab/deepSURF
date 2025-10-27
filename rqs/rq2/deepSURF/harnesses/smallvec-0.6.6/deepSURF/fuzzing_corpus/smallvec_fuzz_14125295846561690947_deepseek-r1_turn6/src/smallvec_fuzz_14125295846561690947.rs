#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::ops::{Deref, DerefMut, Index, IndexMut};

#[derive(Debug, PartialEq, PartialOrd)]
struct CustomType1(String);

impl Clone for CustomType1 {
    fn clone(&self) -> Self {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 41);
        let selector = (custom_impl_num + self.0.len()) % 3;
        if selector == 0 {
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector {
            1 => global_data.first_half,
            _ => global_data.second_half,
        };
        let len = _to_u8(GLOBAL_DATA, 49) % 17;
        let s = _to_str(GLOBAL_DATA, 50, 50 + len as usize);
        CustomType1(String::from(s))
    }
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 900 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let elem = {
            let len = _to_u8(GLOBAL_DATA, 66) % 17;
            let s = _to_str(GLOBAL_DATA, 67, 67 + len as usize);
            CustomType1(String::from(s))
        };

        let capacity = _to_usize(GLOBAL_DATA, 150) % 65;
        let mut sv1 = SmallVec::<[CustomType1; 16]>::with_capacity(capacity);
        let push_idx = _to_usize(GLOBAL_DATA, 158);
        sv1.insert(push_idx, elem.clone());

        let slice_len = _to_usize(GLOBAL_DATA, 166) % 65;
        let mut elements = (0..slice_len).map(|i| {
            let base = 174 + i * 18;
            let len = _to_u8(GLOBAL_DATA, base) % 17;
            let s = _to_str(GLOBAL_DATA, base + 1, base + 1 + len as usize);
            CustomType1(String::from(s))
        }).collect::<Vec<_>>();
        let mut sv2 = SmallVec::<[CustomType1; 16]>::from_vec(elements.clone());
        let mut sv3 = SmallVec::<[CustomType1; 16]>::from_vec(elements);
        let mut sv4 = SmallVec::<[CustomType1; 16]>::from_elem(elem.clone(), _to_usize(GLOBAL_DATA, 500) % 65);

        let num_ops = _to_u8(GLOBAL_DATA, 600) % 5;
        let mut offset = 601;
        for _ in 0..num_ops {
            let op_selector = _to_u8(GLOBAL_DATA, offset);
            offset = offset.wrapping_add(1);
            match op_selector % 4 {
                0 => {
                    sv2.truncate(_to_usize(GLOBAL_DATA, offset));
                    offset += 8;
                },
                1 => {
                    let start = _to_usize(GLOBAL_DATA, offset);
                    let end = _to_usize(GLOBAL_DATA, offset + 8);
                    let _ = sv3.drain();
                    offset += 16;
                },
                2 => {
                    let idx = _to_usize(GLOBAL_DATA, offset);
                    let elem_len = _to_u8(GLOBAL_DATA, offset + 8) % 17;
                    let s = _to_str(GLOBAL_DATA, offset + 9, offset + 9 + elem_len as usize);
                    sv1.push(CustomType1(s.to_string()));
                    offset += 9 + elem_len as usize;
                },
                3 => {
                    let new_len = _to_usize(GLOBAL_DATA, offset);
                    sv4.resize(new_len, elem.clone());
                    offset += 8;
                },
                _ => (),
            };
        }

        let cmp1 = sv1.partial_cmp(&sv2);
        let cmp2 = sv3.len().cmp(&sv4.len());
        
        let drain_start = _to_usize(GLOBAL_DATA, 700);
        let drain_end = _to_usize(GLOBAL_DATA, 708);
        let _ = sv2.drain();
        
        if !sv4.is_empty() {
            println!("{:?}", &sv4[0]);
            let idx = _to_usize(GLOBAL_DATA, 508) % sv4.len();
            sv4[idx] = CustomType1(String::new());
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