#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

struct PanicString(String);

impl core::clone::Clone for PanicString {
    fn clone(&self) -> Self {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 42);
        let custom_impl_inst_num = self.0.len();
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0 {
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector {
            1 => global_data.first_half,
            _ => global_data.second_half,
        };
        let mut t_10 = _to_u8(GLOBAL_DATA, 50) % 17;
        let t_11 = _to_str(GLOBAL_DATA, 51, 51 + t_10 as usize);
        PanicString(String::from(t_11))
    }
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 256 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let num_ops = _to_u8(GLOBAL_DATA, 0) % 8;
        let mut offset = 1;

        let constructor_selector = _to_u8(GLOBAL_DATA, offset) % 4;
        offset += 1;

        let mut sv = match constructor_selector {
            0 => smallvec::SmallVec::<[PanicString; 16]>::new(),
            1 => smallvec::SmallVec::with_capacity(_to_usize(GLOBAL_DATA, offset)),
            2 => {
                let elem = create_panic_string(GLOBAL_DATA, offset);
                smallvec::SmallVec::from_elem(elem, _to_usize(GLOBAL_DATA, offset + 4) % 65)
            }
            _ => smallvec::SmallVec::from_vec(vec![
                create_panic_string(GLOBAL_DATA, offset),
                create_panic_string(GLOBAL_DATA, offset + 8)
            ])
        };

        for _ in 0..num_ops {
            let op = _to_u8(GLOBAL_DATA, offset) % 6;
            offset = (offset + 1) % 128;

            match op {
                0 => {
                    let new_len = _to_usize(GLOBAL_DATA, offset);
                    let elem = create_panic_string(GLOBAL_DATA, offset + 4);
                    sv.resize(new_len, elem);
                }
                1 => sv.push(create_panic_string(GLOBAL_DATA, offset)),
                2 => { sv.pop(); }
                3 => {
                    let cap = sv.capacity();
                    println!("Capacity: {}", cap);
                }
                4 => {
                    let idx = _to_usize(GLOBAL_DATA, offset) % (sv.len() + 1);
                    let elem = create_panic_string(GLOBAL_DATA, offset + 4);
                    sv.insert(idx, elem);
                }
                5 => {
                    if !sv.is_empty() {
                        let idx = _to_usize(GLOBAL_DATA, offset) % sv.len();
                        sv.remove(idx);
                    }
                }
                _ => sv.truncate(_to_usize(GLOBAL_DATA, offset)),
            }

            if let Some(e) = sv.last() {
                println!("Last element: {:?}", e.0);
            }
        }

        sv.resize(
            _to_usize(GLOBAL_DATA, offset),
            create_panic_string(GLOBAL_DATA, offset + 4)
        );

        let slice = sv.as_slice();
        println!("Final vector: {:?}", slice.iter().map(|x| &x.0).collect::<Vec<_>>());

        let _ = sv.into_vec();
    });
}

fn create_panic_string(data: &[u8], index: usize) -> PanicString {
    let mut len = _to_u8(data, index) % 17;
    let s = _to_str(data, index + 1, index + 1 + len as usize);
    PanicString(String::from(s))
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