#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::SmallVec;
use global_data::*;
use std::ops::{Deref, DerefMut};
use std::borrow::BorrowMut;

#[derive(Debug, Clone)]
struct CustomType1(String);

type SmallVecType = SmallVec<[CustomType1; 32]>;

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 8192 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let gdata = global_data.first_half;

        let constructor_selector = _to_u8(gdata, 0) % 5;
        let op_count = _to_u8(gdata, 1) % 10;

        let mut sv = match constructor_selector {
            0 => SmallVecType::new(),
            1 => SmallVecType::with_capacity(_to_usize(gdata, 2) % 65),
            2 => {
                let mut vec = Vec::new();
                for i in 0.._to_usize(gdata, 4) % 65 {
                    let offset = 100 + i * 20;
                    let s = _to_str(gdata, offset, offset + 16);
                    vec.push(CustomType1(s.to_string()));
                }
                SmallVecType::from_vec(vec)
            }
            3 => {
                let mut tmp = SmallVecType::new();
                tmp.extend((0.._to_usize(gdata, 8) % 32).map(|i| {
                    let offset = 200 + i * 18;
                    CustomType1(_to_str(gdata, offset, offset + 16).to_string())
                }));
                tmp
            }
            4 => {
                let elem = CustomType1(_to_str(gdata, 300, 316).to_string());
                SmallVecType::from_elem(elem, _to_usize(gdata, 317) % 65)
            }
            _ => unreachable!()
        };

        for op_index in 0..op_count {
            let op_base = 500 + (op_index as usize) * 30;
            match _to_u8(gdata, op_base) % 12 {
                0 => sv.push(CustomType1(_to_str(gdata, op_base+1, op_base+17).to_string())),
                1 => { sv.pop(); },
                2 => sv.truncate(_to_usize(gdata, op_base+1) % (sv.len() + 1)),
                3 => {
                    if !sv.is_empty() {
                        sv.swap_remove(_to_usize(gdata, op_base+1) % sv.len());
                    }
                }
                4 => {
                    sv.reserve(_to_usize(gdata, op_base+1));
                }
                5 => {
                    sv.drain();
                }
                6 => {
                    let value = CustomType1(_to_str(gdata, op_base+2, op_base+18).to_string());
                    sv.push(value);
                }
                7 => {
                    let idx = _to_usize(gdata, op_base+1) % (sv.len() + 1);
                    let elem = CustomType1(_to_str(gdata, op_base+9, op_base+25).to_string());
                    sv.insert(idx, elem);
                }
                8 => {
                    let mut slice: &mut [CustomType1] = sv.borrow_mut();
                    if !slice.is_empty() {
                        let idx = _to_usize(gdata, op_base+1) % slice.len();
                        slice[idx] = CustomType1("MODIFIED".into());
                        println!("{:?}", &slice[idx]);
                    }
                }
                9 => {
                    if !sv.is_empty() {
                        let idx = _to_usize(gdata, op_base+1) % sv.len();
                        let _ = sv.remove(idx);
                    }
                }
                10 => {
                    let slice = sv.as_slice();
                    if !slice.is_empty() {
                        println!("{:?}", &slice[0]);
                    }
                }
                11 => {
                    sv.reserve(_to_usize(gdata, op_base+1));
                }
                _ => unreachable!()
            };
        }

        let _: &mut [_] = sv.borrow_mut();
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