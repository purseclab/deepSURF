#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use stackvector::*;
use global_data::*;
use std::ops::{DerefMut, Index};

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 256 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        
        let operation_count = _to_usize(global_data.first_half, 0) % 8;
        let mut offset = 8;

        for _ in 0..operation_count {
            let op_selector = _to_u8(global_data.first_half, offset) % 6;
            offset += 1;

            match op_selector {
                0 => {
                    let elem = _to_u8(global_data.first_half, offset);
                    let mut vec = StackVec::<[u8; 32]>::new();
                    vec.push(elem);
                    let _ = vec.pop();
                    println!("{:?}", vec.as_slice());
                }
                1 => {
                    let len = _to_usize(global_data.first_half, offset);
                    let elem = _to_u8(global_data.first_half, offset + 8);
                    let mut vec = StackVec::<[u8; 32]>::from_elem(elem, len % 65);
                    vec.insert(0, elem);
                    println!("Inserted: {}", vec[0]);
                }
                2 => {
                    let slice_data = &global_data.second_half[..32];
                    let mut vec = StackVec::<[u8; 32]>::from_slice(slice_data);
                    let mut drain = vec.drain();
                    let _ = drain.next_back();
                }
                3 => {
                    let mut vec = StackVec::<[u8; 32]>::new();
                    for i in 0..5 {
                        vec.push(_to_u8(global_data.second_half, i));
                    }
                    vec.truncate(_to_usize(global_data.second_half, 5) % 6);
                    println!("Truncated: {:?}", vec.as_slice());
                }
                4 => {
                    let mut vec1 = StackVec::<[u8; 32]>::new();
                    let mut vec2 = StackVec::<[u8; 32]>::new();
                    vec1.push(_to_u8(global_data.first_half, offset));
                    vec2.push(_to_u8(global_data.first_half, offset + 1));
                    let _ = vec1.partial_cmp(&vec2);
                }
                5 => {
                    let buf = [0u8; 32];
                    let len = _to_usize(global_data.second_half, offset) % 32;
                    let mut vec = StackVec::from_buf_and_len(buf, len);
                    let mut drained = vec.drain();
                    let _val = drained.next_back();
                    println!("Drained: {:?}", _val);
                }
                _ => panic!("INTENTIONAL PANIC!")
            };
            offset = offset.wrapping_add(16) % 128;
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