#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::ops::{Deref, DerefMut};

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 512 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let op_count = _to_usize(global_data.first_half, 0) % 10;
        let mut vecs = Vec::new();
        let mut data_idx = 8;

        for _ in 0..op_count {
            if data_idx + 1 > global_data.first_half.len() { break; }
            let op = _to_u8(global_data.first_half, data_idx) % 7;
            data_idx += 1;

            match op {
                0 => {
                    let sv = SmallVec::<[u8; 64]>::new();
                    println!("Created: {:?}", sv);
                    vecs.push(sv);
                },
                1 => {
                    let cap = _to_usize(global_data.first_half, data_idx);
                    data_idx = (data_idx + 8) % global_data.first_half.len();
                    let sv = SmallVec::<[u8;64]>::with_capacity(cap);
                    vecs.push(sv);
                },
                2 => {
                    let slice_len = _to_usize(global_data.second_half, data_idx) % 65;
                    let start = _to_usize(global_data.second_half, data_idx + 8) % global_data.second_half.len();
                    let end = (start + slice_len).min(global_data.second_half.len());
                    let sv = SmallVec::from_slice(&global_data.second_half[start..end]);
                    vecs.push(sv);
                },
                3 => {
                    if let Some(mut sv) = vecs.pop() {
                        let count = _to_usize(global_data.first_half, data_idx) % 65;
                        data_idx = (data_idx + 8) % global_data.first_half.len();
                        for _ in 0..count {
                            let val = *vecs.get(_to_usize(global_data.first_half, data_idx) % vecs.len())
                                .and_then(|v| v.first()).unwrap_or(&0);
                            sv.push(val);
                            data_idx = (data_idx + 1) % global_data.first_half.len();
                        }
                        vecs.push(sv);
                    }
                },
                4 => {
                    if let Some(mut sv) = vecs.pop() {
                        let _ = sv.pop();
                        vecs.push(sv);
                    }
                },
                5 => {
                    let idx = _to_usize(global_data.first_half, data_idx) % (vecs.len() + 1);
                    data_idx = (data_idx + 8) % global_data.first_half.len();
                    if let Some(sv) = vecs.get_mut(idx) {
                        *sv = sv.drain().collect();
                    }
                },
                6 => {
                    let a = _to_usize(global_data.first_half, data_idx) % vecs.len();
                    let b = _to_usize(global_data.first_half, data_idx + 8) % vecs.len();
                    data_idx = (data_idx + 16) % global_data.first_half.len();
                    if let (Some(va), Some(vb)) = (vecs.get(a), vecs.get(b)) {
                        let _ = va.partial_cmp(vb);
                    }
                },
                _ => {}
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