#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::SmallVec;
use global_data::*;

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 1024 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let first_half = global_data.first_half;

        let mut sv = match _to_u8(first_half, 0) % 4 {
            0 => SmallVec::<[u8; 16]>::new(),
            1 => SmallVec::with_capacity(_to_usize(first_half, 1) % 65),
            2 => SmallVec::from_elem(_to_u8(first_half, 2), _to_usize(first_half, 3) % 65),
            3 => {
                let slice_start = _to_usize(first_half, 4) % first_half.len();
                let slice_end = _to_usize(first_half, 8) % first_half.len();
                let slice = if slice_start <= slice_end {
                    &first_half[slice_start..slice_end]
                } else {
                    &first_half[slice_end..slice_start]
                };
                SmallVec::from_slice(slice)
            }
            _ => unreachable!(),
        };

        let second_half = global_data.second_half;
        let mut data_index = 0;
        let op_count = _to_u8(second_half, data_index) % 65;
        data_index += 1;

        for _ in 0..op_count {
            if data_index >= second_half.len() { break; }
            match _to_u8(second_half, data_index) % 7 {
                0 => {
                    data_index += 1;
                    if data_index >= second_half.len() { break; }
                    sv.push(_to_u8(second_half, data_index));
                    data_index += 1;
                }
                1 => { sv.pop(); data_index += 1; }
                2 => {
                    data_index += 1;
                    if data_index + std::mem::size_of::<usize>() >= second_half.len() { break; }
                    let idx = _to_usize(second_half, data_index);
                    data_index += std::mem::size_of::<usize>();
                    let val = _to_u8(second_half, data_index);
                    data_index += 1;
                    sv.insert(idx, val);
                }
                3 => {
                    data_index += 1;
                    let len = _to_usize(second_half, data_index);
                    data_index += std::mem::size_of::<usize>();
                    sv.truncate(len);
                }
                4 => {
                    data_index += 1;
                    let _ = sv.as_slice();
                    println!("{:?}", sv.as_slice());
                }
                5 => {
                    data_index += 1;
                    let mut drain = sv.drain();
                    while let Some(elem) = drain.next() {
                        println!("{}", elem);
                    }
                }
                6 => {
                    data_index += 1;
                    if data_index + std::mem::size_of::<usize>() * 2 >= second_half.len() { break; }
                    let start = _to_usize(second_half, data_index);
                    data_index += std::mem::size_of::<usize>();
                    let end = _to_usize(second_half, data_index);
                    data_index += std::mem::size_of::<usize>();
                    sv.extend_from_slice(&second_half[start.min(end)..end.max(start)]);
                }
                _ => unreachable!(),
            }
        }

        sv.into_vec();
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