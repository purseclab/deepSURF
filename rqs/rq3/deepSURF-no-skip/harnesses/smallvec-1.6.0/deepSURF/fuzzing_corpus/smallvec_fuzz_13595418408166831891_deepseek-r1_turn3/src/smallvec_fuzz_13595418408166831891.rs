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
        CustomType1(t_12)
    }
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 950 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let mut constructor_selector = _to_u8(GLOBAL_DATA, 0) % 4;
        let mut smallvec = match constructor_selector {
            0 => SmallVec::<[CustomType1; 32]>::new(),
            1 => SmallVec::with_capacity(_to_usize(GLOBAL_DATA, 1)),
            2 => SmallVec::from_elem(CustomType1(_to_usize(GLOBAL_DATA, 2)), _to_usize(GLOBAL_DATA, 3) % 65),
            3 => {
                let mut v = Vec::new();
                for i in 0.._to_usize(GLOBAL_DATA, 4) % 65 {
                    v.push(CustomType1(_to_usize(GLOBAL_DATA, 5 + i)));
                }
                SmallVec::from_vec(v)
            }
            _ => unreachable!()
        };

        let num_ops = _to_usize(GLOBAL_DATA, 50) % 10;
        for i in 0..num_ops {
            let op_type = _to_u8(GLOBAL_DATA, 60 + i) % 8;
            match op_type {
                0 => smallvec.push(CustomType1(_to_usize(GLOBAL_DATA, 70 + i))),
                1 => { let _ = smallvec.pop(); }
                2 => smallvec.reserve(_to_usize(GLOBAL_DATA, 80 + i)),
                3 => smallvec.truncate(_to_usize(GLOBAL_DATA, 90 + i) % 65),
                4 => smallvec.insert(_to_usize(GLOBAL_DATA, 100 + i) % 65, CustomType1(_to_usize(GLOBAL_DATA, 200 + i))),
                5 => { let _ = smallvec.as_slice(); }
                6 => { let _ = smallvec.as_mut_slice(); }
                7 => { if !smallvec.is_empty() { println!("{:?}", smallvec[0]); } }
                _ => {}
            }
        }

        let mut insert_slice = Vec::new();
        let slice_len = _to_usize(GLOBAL_DATA, 300) % 65;
        for i in 0..slice_len {
            insert_slice.push(CustomType1(_to_usize(GLOBAL_DATA, 310 + i * 2)));
        }

        let insert_pos = _to_usize(GLOBAL_DATA, 400);
        smallvec.insert_from_slice(insert_pos, &insert_slice);

        let post_ops = _to_usize(GLOBAL_DATA, 500) % 6;
        let mut i = 0;
        while i < post_ops {
            let op_type = _to_u8(GLOBAL_DATA, 510 + i) % 4;
            match op_type {
                0 => smallvec.dedup(),
                1 => smallvec.shrink_to_fit(),
                2 => { let _ = smallvec.capacity(); },
                3 => {
                    let _ = smallvec.into_vec();
                    break;
                },
                _ => {}
            }
            i += 1;
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