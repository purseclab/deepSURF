#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use qwutils::*;
use qwutils::arc_slice::ArcSlice;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

#[derive(Debug)]
struct CustomType0(usize);

impl std::clone::Clone for CustomType0 {
    fn clone(&self) -> Self {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 1);
        let custom_impl_inst_num = self.0;
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0 {
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector {
            1 => global_data.first_half,
            _ => global_data.second_half,
        };
        let t_2 = _to_usize(GLOBAL_DATA, 9);
        CustomType0(t_2)
    }
}

impl std::marker::Copy for CustomType0 {}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 4096 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let ops_count = _to_usize(GLOBAL_DATA, 0) % 8;
        let mut vecs: Vec<Vec<CustomType0>> = Vec::with_capacity(4);
        let mut arc_slices: Vec<ArcSlice<CustomType0>> = Vec::with_capacity(4);

        for i in 0..ops_count {
            let op_selector = _to_u8(GLOBAL_DATA, i * 32) % 6;
            match op_selector {
                0 => {
                    let mut v = Vec::with_capacity(32);
                    for j in 0..(_to_usize(GLOBAL_DATA, i * 64) % 32) {
                        v.push(CustomType0(_to_usize(GLOBAL_DATA, j * 8)));
                    }
                    vecs.push(v);
                }
                1 => {
                    let mut aslice = ArcSlice::new();
                    for j in 0..(_to_usize(GLOBAL_DATA, i * 64) % 32) {
                        aslice.push(CustomType0(_to_usize(GLOBAL_DATA, j * 8)));
                    }
                    arc_slices.push(aslice);
                }
                2 => {
                    if let Some(vec) = vecs.last_mut() {
                        let idx = _to_usize(GLOBAL_DATA, i * 32);
                        let slice = &arc_slices[_to_usize(GLOBAL_DATA, i * 64) % arc_slices.len()];
                        vec.insert_slice_copy(idx, &slice[..]);
                    }
                }
                3 => {
                    if let Some(aslice) = arc_slices.last_mut() {
                        let idx = _to_usize(GLOBAL_DATA, i * 32);
                        let vec = &vecs[_to_usize(GLOBAL_DATA, i * 64) % vecs.len()];
                        aslice.insert_slice(idx, &vec[..]);
                    }
                }
                4 => {
                    if let Some(vec) = vecs.last_mut() {
                        let other = &arc_slices[_to_usize(GLOBAL_DATA, i * 64) % arc_slices.len()];
                        vec.extend_from_slice_copy(&other[..]);
                    }
                }
                5 => {
                    let vecs_len = vecs.len();
                    let target_vec = &mut vecs[_to_usize(GLOBAL_DATA, i * 32) % vecs_len];
                    let insert_idx = _to_usize(GLOBAL_DATA, i * 64);
                    let arc_slices_len = arc_slices.len();
                    let src_slice = &arc_slices[_to_usize(GLOBAL_DATA, i * 128) % arc_slices_len];
                    target_vec.insert_slice_copy(insert_idx, &src_slice[..]);
                }
                _ => (),
            }
        }

        if let Some(main_vec) = vecs.first_mut() {
            let t_69 = _to_usize(GLOBAL_DATA, 273);
            let arc_slices_len = arc_slices.len();
            let src = &arc_slices[_to_usize(GLOBAL_DATA, 281) % arc_slices_len];
            main_vec.insert_slice_copy(t_69, &src[..]);
            println!("{:?}", main_vec);
        }

        for aslice in &arc_slices {
            let extracted = aslice.extract();
            println!("Extracted: {:?}", extracted);
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