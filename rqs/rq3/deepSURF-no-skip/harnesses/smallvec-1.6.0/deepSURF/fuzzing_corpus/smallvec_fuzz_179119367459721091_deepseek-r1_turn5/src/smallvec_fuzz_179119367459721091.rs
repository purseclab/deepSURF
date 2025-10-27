#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

#[derive(Debug)]
struct CustomType1(usize);

impl core::clone::Clone for CustomType1 {
    fn clone(&self) -> Self {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 35);
        let custom_impl_inst_num = self.0;
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0 {
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector {
            1 => global_data.first_half,
            _ => global_data.second_half,
        };
        let t_11 = _to_usize(GLOBAL_DATA, 43);
        CustomType1(t_11)
    }
}

impl core::marker::Copy for CustomType1 {}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 2048 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        
        let mut construct_selector = _to_u8(GLOBAL_DATA, 0) % 4;
        let mut sv = match construct_selector {
            0 => {
                let cap = _to_usize(GLOBAL_DATA, 1) % 128;
                SmallVec::<[CustomType1; 128]>::with_capacity(cap)
            }
            1 => {
                let len = _to_usize(GLOBAL_DATA, 10) % 65;
                let elem = CustomType1(_to_usize(GLOBAL_DATA, 20));
                SmallVec::from_elem(elem, len)
            }
            2 => {
                let len = _to_usize(GLOBAL_DATA, 30) % 65;
                let mut vec = Vec::with_capacity(len);
                for i in 0..len {
                    vec.push(CustomType1(_to_usize(GLOBAL_DATA, 40 + i * 4)));
                }
                SmallVec::from_vec(vec)
            }
            _ => {
                let buf = [CustomType1(0); 128];
                let len = _to_usize(GLOBAL_DATA, 100) % 65;
                SmallVec::from_buf_and_len(buf, len)
            }
        };

        let ops = _to_usize(GLOBAL_DATA, 200) % 8 + 3;
        for op in 0..ops {
            let op_selector = _to_u8(GLOBAL_DATA, 300 + op as usize) % 8;
            match op_selector {
                0 => {
                    let idx = _to_usize(GLOBAL_DATA, 400 + op as usize * 4);
                    let elem = CustomType1(_to_usize(GLOBAL_DATA, 500 + op as usize * 4));
                    sv.insert(idx, elem);
                }
                1 => {
                    sv.truncate(_to_usize(GLOBAL_DATA, 600 + op as usize * 4) % 65);
                }
                2 => {
                    let cap = _to_usize(GLOBAL_DATA, 700 + op as usize * 4);
                    sv.reserve(cap);
                }
                3 => {
                    let idx = _to_usize(GLOBAL_DATA, 800 + op as usize * 4);
                    if !sv.is_empty() {
                        sv.remove(idx % sv.len());
                    }
                }
                4 => {
                    let val = CustomType1(_to_usize(GLOBAL_DATA, 900 + op as usize * 4));
                    sv.push(val);
                }
                5 => {
                    let val = CustomType1(_to_usize(GLOBAL_DATA, 1000 + op as usize * 4));
                    sv.extend_from_slice(&[val]);
                }
                6 => {
                    let other_len = _to_usize(GLOBAL_DATA, 1100 + op as usize * 4) % 65;
                    let mut other = SmallVec::<[CustomType1; 128]>::new();
                    for i in 0..other_len {
                        other.push(CustomType1(_to_usize(GLOBAL_DATA, 1200 + i * 4)));
                    }
                    sv.append(&mut other);
                }
                _ => {
                    let slice = &[
                        CustomType1(_to_usize(GLOBAL_DATA, 1300)),
                        CustomType1(_to_usize(GLOBAL_DATA, 1304)),
                    ];
                    sv.extend_from_slice(slice);
                }
            }
        }

        let mut vec_data = Vec::new();
        for i in 0..32 {
            vec_data.push(CustomType1(_to_usize(GLOBAL_DATA, 1400 + i * 4)));
        }
        let extend_len = _to_usize(GLOBAL_DATA, 1600) % 65;
        let slice = &vec_data[..vec_data.len().min(extend_len)];
        sv.extend_from_slice(slice);

        if let Some(e) = sv.pop() {
            println!("{:?}", sv);
        }

        let _ = sv.capacity();
        sv.shrink_to_fit();
        sv.drain(.._to_usize(GLOBAL_DATA, 1700) % (sv.len() + 1));
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