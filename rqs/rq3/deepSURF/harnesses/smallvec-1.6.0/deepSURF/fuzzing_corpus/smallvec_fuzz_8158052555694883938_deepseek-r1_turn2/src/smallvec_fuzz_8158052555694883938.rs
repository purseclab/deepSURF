#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::ops::{Deref, DerefMut, Index, IndexMut, RangeBounds};
use std::str::FromStr;

#[derive(Debug)]
struct CustomType3(String);

impl Clone for CustomType3 {
    fn clone(&self) -> Self {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 19);
        let custom_impl_inst_num = self.0.len();
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0 {
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector {
            1 => global_data.first_half,
            _ => global_data.second_half,
        };
        let mut t_6 = _to_u8(GLOBAL_DATA, 27) % 17;
        let t_7 = _to_str(GLOBAL_DATA, 28, 28 + t_6 as usize);
        let t_8 = String::from(t_7);
        CustomType3(t_8)
    }
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 2048 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let op_count = _to_usize(GLOBAL_DATA, 0) % 8 + 3;
        let mut sv_collection: Vec<SmallVec<[CustomType3; 32]>> = Vec::new();

        for i in 0..op_count {
            let op_selector = _to_u8(GLOBAL_DATA, i * 4) % 6;
            match op_selector {
                0 => {
                    let elem_count = _to_usize(GLOBAL_DATA, i * 4 + 1) % 33;
                    let mut sv = SmallVec::new();
                    for _ in 0..elem_count {
                        let len = _to_u8(GLOBAL_DATA, i * 4 + 2) % 128;
                        let s = _to_str(GLOBAL_DATA, i * 4 + 3, i * 4 + 3 + len as usize);
                        sv.push(CustomType3(s.to_string()));
                    }
                    sv_collection.push(sv);
                },
                1 => {
                    let cap = _to_usize(GLOBAL_DATA, i * 4 + 1);
                    let sv = SmallVec::with_capacity(cap);
                    sv_collection.push(sv);
                },
                2 => {
                    let slice_len = _to_usize(GLOBAL_DATA, i * 4 + 1) % 33;
                    let mut elems = Vec::new();
                    for j in 0..slice_len {
                        let len = _to_u8(GLOBAL_DATA, i * 4 + j + 2) % 128;
                        let s = _to_str(GLOBAL_DATA, i * 4 + j + 3, i * 4 + j + 3 + len as usize);
                        elems.push(CustomType3(s.to_string()));
                    }
                    let sv = SmallVec::from_vec(elems);
                    sv_collection.push(sv);
                },
                3 => {
                    if let Some(prev_sv) = sv_collection.last_mut() {
                        let len = _to_u8(GLOBAL_DATA, i * 4 + 1) % 128;
                        let s = _to_str(GLOBAL_DATA, i * 4 + 2, i * 4 + 2 + len as usize);
                        prev_sv.push(CustomType3(s.to_string()));
                    }
                },
                4 => {
                    if let Some(prev_sv) = sv_collection.last_mut() {
                        let _ = prev_sv.pop();
                    }
                },
                5 => {
                    if let Some(prev_sv) = sv_collection.last() {
                        let index = _to_usize(GLOBAL_DATA, i * 4 + 1);
                        if let Some(elem) = prev_sv.get(index) {
                            println!("{:?}", elem);
                        }
                    }
                },
                _ => (),
            }
        }

        if let Some(mut sv) = sv_collection.pop() {
            let start_idx = op_count * 4;
            let start = _to_usize(GLOBAL_DATA, start_idx);
            let end = _to_usize(GLOBAL_DATA, start_idx + 1);
            let range = start..end;

            if let Some(elem) = sv.as_slice().get(0) {
                println!("{:?}", elem);
            }
            if let Some(elem) = sv.as_mut_slice().get_mut(0) {
                *elem = CustomType3(String::new());
            }

            let mut drain = sv.drain(range);
            let _ = drain.next();
            println!("Drain remaining: {}", drain.size_hint().0);
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