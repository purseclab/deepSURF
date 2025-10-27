#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::ops::{Deref, DerefMut, Index, IndexMut};

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 130 {
            return;
        }
        set_global_data(data);
        let global_data = get_global_data();
        let first = global_data.first_half;
        let second = global_data.second_half;

        let ctor_byte = _to_u8(first, 0);
        let mut sv: SmallVec<[u8; 32]> = match ctor_byte % 6 {
            0 => SmallVec::<[u8; 32]>::new(),
            1 => {
                let cap = _to_usize(first, 1) % 65;
                SmallVec::<[u8; 32]>::with_capacity(cap)
            }
            2 => {
                let elem = _to_u8(first, 3);
                let len = _to_usize(first, 4) % 65;
                SmallVec::from_elem(elem, len)
            }
            3 => {
                let slice_len = (_to_u8(first, 6) % 16) as usize;
                let slice = &second[..slice_len];
                SmallVec::from_slice(slice)
            }
            4 => {
                let vec_len = (_to_u8(first, 8) % 20) as usize;
                let mut v = Vec::with_capacity(vec_len);
                for i in 0..vec_len {
                    v.push(_to_u8(second, i));
                }
                SmallVec::from_vec(v)
            }
            _ => {
                let buf_len = (_to_u8(first, 10) % 32) as usize;
                let mut buf = [0u8; 32];
                for i in 0..buf_len {
                    buf[i] = _to_u8(second, i + 32);
                }
                SmallVec::from_buf_and_len(buf, buf_len)
            }
        };

        let op_count = _to_u8(first, 12) % 20;
        for i in 0..op_count {
            let op = _to_u8(second, i as usize);
            match op % 10 {
                0 => {
                    let val = _to_u8(second, i as usize + 1);
                    sv.push(val);
                }
                1 => {
                    sv.pop();
                }
                2 => {
                    let idx = _to_usize(first, 14 + i as usize);
                    let val = _to_u8(second, i as usize + 2);
                    if sv.len() >= idx {
                        sv.insert(idx, val);
                    }
                }
                3 => {
                    let new_len = _to_usize(first, 16 + i as usize);
                    sv.truncate(new_len);
                }
                4 => {
                    if !sv.is_empty() {
                        let idx = _to_usize(first, 18 + i as usize);
                        if idx < sv.len() {
                            sv.remove(idx);
                        }
                    }
                }
                5 => {
                    let additional = _to_usize(first, 20 + i as usize);
                    sv.reserve(additional);
                }
                6 => {
                    let slice_len = (_to_u8(second, i as usize + 3) % 16) as usize;
                    if slice_len <= second.len() {
                        let slice = &second[..slice_len];
                        sv.extend_from_slice(slice);
                    }
                }
                7 => {
                    let keep_even = _to_bool(first, 22 + i as usize);
                    sv.retain(|x| if keep_even { *x % 2 == 0 } else { *x % 2 == 1 });
                }
                8 => {
                    sv.clear();
                }
                _ => {
                    let _ = sv.partial_cmp(&sv.clone());
                }
            }
        }

        let slice_ref = sv.as_slice();
        println!("{:?}", slice_ref.deref());

        let vec1 = sv.into_vec();
        println!("{:?}", vec1.last());

        let sv2 = SmallVec::<[u8; 32]>::from_vec(vec1);
        let vec2 = sv2.into_vec();
        println!("{:?}", vec2.first());
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