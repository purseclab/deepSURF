#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::ops::{Deref, DerefMut};

#[derive(Debug, PartialEq)]
struct CustomType1(usize);

impl core::marker::Copy for CustomType1 {}

impl core::clone::Clone for CustomType1 {
    fn clone(&self) -> Self {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 43);
        let custom_impl_inst_num = self.0;
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0 {
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector {
            1 => global_data.first_half,
            _ => global_data.second_half,
        };
        let t_12 = _to_usize(GLOBAL_DATA, 51);
        let t_13 = CustomType1(t_12);
        t_13
    }
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 1024 {
            return;
        }
        set_global_data(data);
        let global_data = get_global_data();
        let FIRST = global_data.first_half;
        let SECOND = global_data.second_half;

        let selector = _to_u8(FIRST, 0) % 4;
        let mut sv1: SmallVec<[CustomType1; 32]> = match selector {
            0 => SmallVec::new(),
            1 => {
                let cap = _to_usize(FIRST, 1) % 64;
                SmallVec::with_capacity(cap)
            }
            2 => {
                let mut buf = [CustomType1(0); 32];
                for i in 0..32 {
                    buf[i] = CustomType1(_to_usize(FIRST, 9 + i));
                }
                let len = _to_u8(FIRST, 41) as usize % 32;
                SmallVec::from_buf_and_len(buf, len)
            }
            _ => {
                let slice_len = (_to_u8(FIRST, 42) % 32) as usize;
                let mut temp_vec = Vec::with_capacity(slice_len);
                for idx in 0..slice_len {
                    temp_vec.push(CustomType1(_to_usize(FIRST, 43 + idx)));
                }
                SmallVec::from_slice(&temp_vec[..])
            }
        };

        let push_count = _to_u8(SECOND, 0) % 10;
        for i in 0..push_count {
            let val = CustomType1(_to_usize(SECOND, 1 + i as usize));
            sv1.push(val);
        }

        let additional = _to_usize(SECOND, 20) % 128;
        sv1.reserve(additional);

        let idx_raw = _to_usize(SECOND, 28);
        let insert_index = if sv1.is_empty() { 0 } else { idx_raw % sv1.len() };

        let slice_len = (_to_u8(SECOND, 36) % 65) as usize;
        let mut temp_slice_vec = Vec::with_capacity(slice_len);
        for j in 0..slice_len {
            temp_slice_vec.push(CustomType1(_to_usize(SECOND, 37 + j)));
        }
        let slice_to_insert = &temp_slice_vec[..];

        sv1.insert_from_slice(insert_index, slice_to_insert);

        sv1.dedup();

        let mut toggle = true;
        sv1.retain(|item| {
            toggle = !toggle;
            if toggle && (*item).0 % 2 == 0 {
                return false;
            }
            true
        });

        let new_len = _to_u8(SECOND, 100) as usize % 40;
        let default_val = CustomType1(_to_usize(SECOND, 101));
        sv1.resize(new_len, default_val);

        if !sv1.is_empty() {
            let ref_first: &CustomType1 = &sv1[0];
            println!("First element {:?}", *ref_first);
        }

        let mut sv2 = SmallVec::<[CustomType1; 32]>::from_vec(sv1.clone().into_vec());
        sv1.append(&mut sv2);

        let trunc_len = _to_u8(SECOND, 200) as usize % 40;
        sv1.truncate(trunc_len);

        let _ = sv1.capacity();
        let _ = sv1.len();
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