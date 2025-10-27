#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::SmallVec;
use global_data::*;

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 512 { return; }
        set_global_data(data);
        let global_data = get_global_data().first_half;
        let mut data_index = 0;

        let constructor = _to_u8(global_data, data_index) % 4;
        data_index += 1;

        let mut sv = match constructor {
            0 => SmallVec::<[u8; 32]>::new(),
            1 => {
                let cap = _to_usize(global_data, data_index);
                data_index += 8;
                SmallVec::with_capacity(cap)
            },
            2 => {
                let elem = _to_u8(global_data, data_index);
                data_index += 1;
                let len = _to_usize(global_data, data_index) % 65;
                data_index += 8;
                SmallVec::from_elem(elem, len)
            },
            3 => {
                let slice_len = _to_usize(global_data, data_index) % 65;
                data_index += 8;
                let start = data_index;
                let end = start + slice_len;
                let slice = &global_data[start..end];
                data_index += slice_len;
                SmallVec::from_slice(slice)
            },
            _ => unreachable!(),
        };

        let num_ops = _to_u8(global_data, data_index) % 8;
        data_index += 1;

        for _ in 0..num_ops {
            let op = _to_u8(global_data, data_index) % 7;
            data_index += 1;

            match op {
                0 => {
                    let additional = _to_usize(global_data, data_index);
                    data_index += 8;
                    sv.reserve_exact(additional);
                },
                1 => {
                    let value = _to_u8(global_data, data_index);
                    data_index += 1;
                    sv.push(value);
                },
                2 => {
                    let popped = sv.pop();
                    println!("Popped: {:?}", popped);
                },
                3 => {
                    if !sv.is_empty() {
                        let idx = _to_usize(global_data, data_index) % sv.len();
                        data_index += 8;
                        println!("Index {}: {:?}", idx, sv[idx]);
                    }
                },
                4 => {
                    let cap_before = sv.capacity();
                    sv.shrink_to_fit();
                    println!("Capacity: {} -> {}", cap_before, sv.capacity());
                },
                5 => {
                    let len = _to_usize(global_data, data_index);
                    data_index += 8;
                    sv.truncate(len);
                },
                6 => {
                    let new_len = sv.len().wrapping_add(_to_usize(global_data, data_index));
                    data_index += 8;
                    sv.resize_with(new_len, || _to_u8(global_data, data_index));
                    data_index += 1;
                },
                _ => unreachable!(),
            }
        }

        let final_additional = _to_usize(global_data, data_index);
        sv.reserve_exact(final_additional);
        println!("Final slice: {:?}", sv.as_slice());
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