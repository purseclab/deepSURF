#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

#[derive(Debug)]
struct CustomType1(String);
#[derive(Debug)]
struct CustomType2(String);

impl core::cmp::PartialEq for CustomType2 {
    fn eq(&self, other: &Self) -> bool {
        let global_data = get_global_data();
        let custom_impl_num = _to_usize(global_data.first_half, 34);
        let custom_impl_inst_num = self.0.len();
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0 {
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector {
            1 => global_data.first_half,
            _ => global_data.second_half,
        };
        _to_bool(GLOBAL_DATA, 42)
    }
}

fn _custom_fn0(_: &mut CustomType1) -> CustomType2 {
    let global_data = get_global_data();
    let GLOBAL_DATA = global_data.first_half;
    if _to_u8(GLOBAL_DATA, 43) % 2 == 0 {
        panic!("INTENTIONAL PANIC!");
    }
    CustomType2(String::from(_to_str(GLOBAL_DATA, 45, 45 + _to_u8(GLOBAL_DATA, 44) as usize)))
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 1024 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let constructor = _to_u8(GLOBAL_DATA, 0) % 4;
        let mut sv = match constructor {
            0 => {
                let elem_size = _to_usize(GLOBAL_DATA, 1) % 65;
                let slice_data = &GLOBAL_DATA[10..150];
                let items: Vec<CustomType1> = slice_data.chunks(8)
                    .take(elem_size)
                    .map(|c| CustomType1(String::from_utf8_lossy(c).into()))
                    .collect();
                SmallVec::from_vec(items)
            }
            1 => {
                let count = _to_usize(GLOBAL_DATA, 1) % 65;
                let mut sv = SmallVec::<[CustomType1; 32]>::new();
                for _ in 0..count {
                    let elem = CustomType1(String::from(_to_str(GLOBAL_DATA, 2, 25)));
                    sv.push(elem);
                }
                sv
            }
            2 => {
                let mut v = SmallVec::<[CustomType1; 32]>::new();
                for i in 0..(_to_usize(GLOBAL_DATA, 1) % 65) {
                    v.push(CustomType1(String::from(_to_str(GLOBAL_DATA, 10+i*8, 18+i*8))));
                }
                v
            }
            _ => SmallVec::<[CustomType1; 32]>::with_capacity(_to_usize(GLOBAL_DATA, 1)),
        };

        for op in 0..(_to_u8(GLOBAL_DATA, 50) % 8) {
            match op % 7 {
                0 => sv.push(CustomType1(String::new())),
                1 => { let _ = sv.pop(); }
                2 => sv.insert(_to_usize(GLOBAL_DATA, 60 + op as usize) % (sv.len()+1), CustomType1(String::new())),
                3 => sv.truncate(_to_usize(GLOBAL_DATA, 70 + op as usize) % (sv.len()+1)),
                4 => sv.extend([CustomType1(String::new()), CustomType1(String::new())]),
                5 => sv.append(&mut SmallVec::<[CustomType1; 32]>::from_vec(vec![CustomType1(String::new())])),
                _ => println!("Deref slice: {:?}", sv.as_slice()),
            };
        }

        let _ = sv.dedup_by_key(_custom_fn0);

        for _ in 0..(_to_u8(GLOBAL_DATA, 100) % 5) {
            match _to_usize(GLOBAL_DATA, 150) % 3 {
                0 => { let _ = sv.as_mut_slice(); }
                1 => sv.shrink_to_fit(),
                _ => sv.reserve(_to_usize(GLOBAL_DATA, 200)),
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