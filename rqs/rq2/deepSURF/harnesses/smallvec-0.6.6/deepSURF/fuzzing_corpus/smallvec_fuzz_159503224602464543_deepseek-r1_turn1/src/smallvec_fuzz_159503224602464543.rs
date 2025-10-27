#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::SmallVec;
use global_data::*;

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 512 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let (first_half, second_half) = (global_data.first_half, global_data.second_half);
        let mut vec_pool: Vec<SmallVec<[u8; 16]>> = Vec::new();
        let mut current_vec = 0;
        
        let num_ops = _to_usize(first_half, 0) % 64;
        let mut data_index = 8;
        
        for _ in 0..num_ops {
            if data_index >= first_half.len() { break; }
            
            let op_select = _to_u8(first_half, data_index) % 11;
            data_index += 1;
            
            match op_select {
                0 => {
                    let cap = _to_usize(first_half, data_index) % 65;
                    data_index += 8;
                    vec_pool.push(SmallVec::with_capacity(cap));
                },
                1 => {
                    let slice_len = _to_usize(first_half, data_index) % 65;
                    data_index += 8;
                    let start = _to_usize(first_half, data_index) % (first_half.len() - slice_len).max(1);
                    data_index += 8;
                    vec_pool.push(SmallVec::from_slice(&first_half[start..start+slice_len]));
                },
                2 => {
                    let elem = _to_u8(first_half, data_index);
                    data_index += 1;
                    let count = _to_usize(first_half, data_index) % 65;
                    data_index += 8;
                    vec_pool.push(SmallVec::from_elem(elem, count));
                },
                3 => {
                    if let Some(v) = vec_pool.get_mut(current_vec) {
                        let additional = _to_usize(first_half, data_index);
                        data_index += 8;
                        v.reserve(additional);
                    }
                },
                4 => {
                    if let Some(v) = vec_pool.get_mut(current_vec) {
                        let elem = _to_u8(first_half, data_index);
                        data_index += 1;
                        v.push(elem);
                    }
                },
                5 => {
                    if let Some(v) = vec_pool.get_mut(current_vec) {
                        if let Some(e) = v.pop() {
                            println!("{:?}", e);
                        }
                    }
                },
                6 => {
                    if let Some(v) = vec_pool.get_mut(current_vec) {
                        let index = _to_usize(first_half, data_index);
                        data_index += 8;
                        if !v.is_empty() {
                            let _ = v.swap_remove(index % v.len());
                        }
                    }
                },
                7 => {
                    if let Some(v) = vec_pool.get_mut(current_vec) {
                        let capacity = _to_usize(first_half, data_index);
                        data_index += 8;
                        v.grow(capacity);
                    }
                },
                8 => {
                    if let Some(v) = vec_pool.get(current_vec) {
                        let other = SmallVec::from_slice(&second_half[..16]);
                        let _ = v.partial_cmp(&other);
                    }
                },
                9 => {
                    if let Some(v) = vec_pool.get_mut(current_vec) {
                        let len = _to_usize(first_half, data_index);
                        data_index += 8;
                        v.truncate(len);
                    }
                },
                10 => {
                    if let Some(v) = vec_pool.get_mut(current_vec) {
                        let _drained: Vec<_> = v.drain().collect();
                    }
                },
                _ => {}
            }
            
            if let Some(v) = vec_pool.get(current_vec) {
                println!("{:?}", v.as_slice());
                current_vec = (current_vec + 1) % vec_pool.len().max(1);
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