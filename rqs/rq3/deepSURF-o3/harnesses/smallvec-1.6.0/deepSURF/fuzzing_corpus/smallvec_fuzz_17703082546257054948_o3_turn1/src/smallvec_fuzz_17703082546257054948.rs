#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::cmp::Ordering;

fn build_array32(data: &[u8], start: usize) -> [u8; 32] {
    let mut buf = [0u8; 32];
    for i in 0..32 {
        buf[i] = _to_u8(data, start + i);
    }
    buf
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 260 {
            return;
        }
        set_global_data(data);
        let global_data = get_global_data();
        let first = global_data.first_half;

        let buf = build_array32(first, 30);
        let len_from = (_to_u8(first, 62) as usize) % 33;

        let mut vec_a: SmallVec<[u8; 32]> = match _to_u8(first, 0) % 6 {
            0 => SmallVec::new(),
            1 => SmallVec::with_capacity((_to_usize(first, 1) % 65) + 1),
            2 => SmallVec::from_buf(buf),
            3 => SmallVec::from_buf_and_len(buf, len_from),
            4 => {
                let mut v = Vec::new();
                let vec_len = (_to_u8(first, 10) as usize) % 65;
                for i in 0..vec_len {
                    v.push(_to_u8(first, 11 + i));
                }
                SmallVec::from_vec(v)
            }
            _ => {
                let slice_len = (_to_u8(first, 20) as usize) % 65;
                let slice_start = 21;
                let slice_end = slice_start + slice_len;
                let slice = &first[slice_start..slice_end];
                SmallVec::from_slice(slice)
            }
        };

        let op_total = (_to_u8(first, 100) % 10) + 1;
        for i in 0..op_total {
            match _to_u8(first, 110 + i as usize) % 10 {
                0 => vec_a.push(_to_u8(first, 120 + i as usize)),
                1 => {
                    if !vec_a.is_empty() {
                        vec_a.pop();
                    }
                }
                2 => vec_a.reserve(_to_usize(first, 130 + i as usize)),
                3 => vec_a.shrink_to_fit(),
                4 => {
                    let tgt = (_to_usize(first, 140 + i as usize)) % 65;
                    vec_a.truncate(tgt);
                }
                5 => {
                    let extra_slice_len = (_to_u8(first, 150 + i as usize) as usize) % 65;
                    let slice_start = 160 + i as usize;
                    let slice_end = slice_start + extra_slice_len;
                    if slice_end <= first.len() {
                        let slice = &first[slice_start..slice_end];
                        vec_a.extend_from_slice(slice);
                    }
                }
                6 => {
                    let cap_before = (&vec_a).capacity();
                    let len_before = (&vec_a).len();
                    println!("cap_before {} len_before {}", cap_before, len_before);
                }
                7 => {
                    let slice_ref = (&vec_a).as_slice();
                    println!("{:?}", slice_ref);
                }
                8 => {
                    let vec_clone = vec_a.clone();
                    let cmp_res: Ordering = vec_a.cmp(&vec_clone);
                    println!("{:?}", cmp_res);
                }
                _ => {
                    if vec_a.len() >= 2 {
                        vec_a.swap_remove(0);
                    }
                }
            }
        }

        let final_capacity = (&vec_a).capacity();
        println!("final_capacity {}", final_capacity);

        let _ = vec_a.into_vec();
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