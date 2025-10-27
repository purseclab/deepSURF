#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use qwutils::*;
use qwutils::arc_slice::ArcSlice;
use global_data::*;
use std::ops::{Deref, DerefMut};

#[derive(Debug)]
struct CustomType0(String);

impl std::clone::Clone for CustomType0 {
    fn clone(&self) -> Self {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 1);
        let custom_impl_inst_num = self.0.len();
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0 {
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector {
            1 => global_data.first_half,
            _ => global_data.second_half,
        };
        let mut t_2 = _to_u8(GLOBAL_DATA, 9) % 17;
        let t_3 = _to_str(GLOBAL_DATA, 10, 10 + t_2 as usize);
        let t_4 = String::from(t_3);
        CustomType0(t_4)
    }
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 8192 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let first_half = global_data.first_half;
        let second_half = global_data.second_half;

        let mut base_vec = Vec::with_capacity((_to_u8(first_half, 0) % 65) as usize);
        let mut arc_slice = ArcSlice::new();
        let operations = _to_u8(second_half, 0) % 65;

        for op_idx in 0..operations {
            let selector = _to_u8(second_half, 1 + op_idx as usize) % 6;
            match selector {
                0 => {
                    let count = _to_u8(first_half, 5 + op_idx as usize) % 65;
                    for i in 0..count {
                        let offset = 100 + (op_idx * 20 + i * 2) as usize;
                        let len = _to_u8(first_half, offset) % 17;
                        let s = _to_str(first_half, offset + 1, offset + 1 + len as usize);
                        base_vec.push(CustomType0(s.to_string()));
                    }
                },
                1 => {
                    if !base_vec.is_empty() {
                        let idx = _to_usize(second_half, 50 + op_idx as usize * 8);
                        let slice_len = _to_u8(second_half, 50 + op_idx as usize * 8 + 4) % 65;
                        let end = std::cmp::min(base_vec.len(), slice_len as usize);
                        let temp_slice = base_vec[..end].to_vec();
                        base_vec.insert_slice_clone(idx, &temp_slice);
                    }
                },
                2 => {
                    let mut new_arc = ArcSlice::from(base_vec.clone());
                    let arc_idx = _to_usize(second_half, 200 + op_idx as usize * 8);
                    let arc_src_len = _to_u8(second_half, 200 + op_idx as usize * 8 + 4) % 65;
                    new_arc.insert_slice(
                        arc_idx,
                        &base_vec[..std::cmp::min(base_vec.len(), arc_src_len as usize)]
                    );
                    arc_slice = new_arc;
                },
                3 => {
                    let ext_start = _to_u8(second_half, 300 + op_idx as usize) % 65;
                    let ext_end = _to_u8(second_half, 300 + op_idx as usize + 1) % 65;
                    base_vec.extend(arc_slice[ext_start as usize..ext_end as usize].iter().cloned());
                },
                4 => {
                    let truncate_len = _to_u8(second_half, 400 + op_idx as usize) % 65;
                    base_vec.truncate(truncate_len as usize);
                },
                5 => {
                    let split_at = _to_usize(second_half, 500 + op_idx as usize * 8);
                    if split_at < base_vec.len() {
                        let _ = base_vec.split_off(split_at);
                    }
                },
                _ => {}
            }

            if !base_vec.is_empty() {
                let deref_idx = _to_usize(second_half, 600 + op_idx as usize) % base_vec.len();
                println!("{:?}", base_vec[deref_idx]);
            }
        }

        let final_idx = _to_usize(second_half, 1000);
        let final_slice = &arc_slice[..(_to_u8(second_half, 1004) % 65) as usize];
        base_vec.insert_slice_clone(final_idx, final_slice);
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