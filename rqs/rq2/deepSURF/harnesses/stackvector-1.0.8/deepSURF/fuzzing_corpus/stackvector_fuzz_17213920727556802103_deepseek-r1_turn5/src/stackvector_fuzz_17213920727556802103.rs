#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use std::hash::Hash;
use stackvector::*;
use global_data::*;

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 256 {
            return;
        }
        set_global_data(data);
        let global_data = get_global_data();
        let first_half = global_data.first_half;
        let second_half = global_data.second_half;
        let op_count = _to_usize(first_half, 0) % 5;
        let mut op_idx = 1;

        for _ in 0..op_count {
            let op = _to_u8(first_half, op_idx) % 5;
            op_idx += 1;

            match op {
                0 => {
                    let start = _to_usize(first_half, op_idx);
                    let len = _to_usize(first_half, op_idx + 1) % 65;
                    op_idx += 2;
                    let end = start + len;
                    if end <= first_half.len() {
                        let slice = &first_half[start..end];
                        let mut sv = StackVec::<[u8; 128]>::from_slice(slice);
                        println!("{:?}", sv);
                        let _ = sv.as_slice();
                        sv.as_mut_slice();
                        sv.swap_remove(_to_usize(second_half, 0));
                        sv.dedup_by(|a, b| a == b);
                    }
                }
                1 => {
                    let elem = _to_u8(first_half, op_idx);
                    let count = _to_usize(first_half, op_idx + 1) % 65;
                    op_idx += 2;
                    let sv = StackVec::from_elem(elem, count);
                    let mut other = StackVec::<[u8; 64]>::new();
                    other.extend_from_slice(&second_half[.._to_usize(second_half, 0) % 65]);
                    println!("{:?}", sv.partial_cmp(&other));
                    sv.hash(&mut std::collections::hash_map::DefaultHasher::new());
                }
                2 => {
                    let mut sv = StackVec::<[u8; 128]>::new();
                    let pushes = _to_usize(first_half, op_idx) % 65;
                    op_idx += 1;
                    for i in 0..pushes {
                        sv.push(_to_u8(second_half, i));
                    }
                    let idx = _to_usize(second_half, pushes);
                    sv.insert(idx, _to_u8(second_half, pushes + 1));
                    sv.truncate(_to_usize(second_half, pushes + 2));
                    println!("{:?}", sv);
                    sv.dedup();
                    sv.resize(_to_usize(second_half, 3) % 65, _to_u8(second_half, 4));
                    let _drain = sv.drain();
                }
                3 => {
                    let len = _to_usize(first_half, op_idx) % 65;
                    op_idx += 1;
                    let arr: [u8; 64] = std::array::from_fn(|i| _to_u8(second_half, i));
                    let mut sv = StackVec::from_buf_and_len(arr, len);
                    if let Some(val) = sv.pop() {
                        println!("{}", val);
                    }
                    sv.push(_to_u8(second_half, 64));
                    sv.insert_from_slice(_to_usize(second_half, 65), &second_half[66..70]);
                    let _inner: [u8; 64] = sv.into_inner().unwrap_or([0; 64]);
                }
                4 => {
                    let vec: Vec<u8> = (0..16).map(|i| _to_u8(second_half, i)).collect();
                    let sv = StackVec::<[u8; 64]>::from_vec(vec);
                    let mut iter = sv.clone().into_iter();
                    for _ in 0.._to_usize(second_half, 16) {
                        iter.next_back();
                    }
                    let mut cloned = sv.clone();
                    cloned.retain(|x| *x % 2 == 0u8);
                }
                _ => (),
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