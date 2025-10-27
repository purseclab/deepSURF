#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 128 {
            return;
        }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let ctor_choice = _to_u8(GLOBAL_DATA, 0);
        let mut small_vec: SmallVec<[u8; 32]> = match ctor_choice % 6 {
            0 => SmallVec::new(),
            1 => {
                let cap = _to_usize(GLOBAL_DATA, 1) % 65;
                SmallVec::with_capacity(cap)
            }
            2 => {
                let slice_len = _to_u8(GLOBAL_DATA, 9) as usize;
                let slice_start = 10usize;
                let slice_end = (slice_start + slice_len).min(GLOBAL_DATA.len());
                SmallVec::from_slice(&GLOBAL_DATA[slice_start..slice_end])
            }
            3 => {
                let vec_len = _to_u8(GLOBAL_DATA, 20) as usize % 65;
                let mut v = Vec::with_capacity(vec_len);
                for i in 0..vec_len {
                    v.push(_to_u8(GLOBAL_DATA, 21 + i));
                }
                SmallVec::from_vec(v)
            }
            4 => {
                let elem = _to_u8(GLOBAL_DATA, 40);
                let repeat = _to_usize(GLOBAL_DATA, 41) % 65;
                SmallVec::from_elem(elem, repeat)
            }
            _ => {
                let mut buf = [0u8; 32];
                for i in 0..32 {
                    buf[i] = _to_u8(GLOBAL_DATA, 49 + i);
                }
                let len = _to_usize(GLOBAL_DATA, 81).min(32);
                SmallVec::from_buf_and_len(buf, len)
            }
        };

        let _ = small_vec.len();
        let _ = small_vec.is_empty();
        let _ = small_vec.capacity();

        let _slice_ref: &[u8] = small_vec.deref();
        let _slice_mut: &mut [u8] = small_vec.deref_mut();

        if let Ok(_parsed) = u8::from_str("42") {
            let _ = _parsed;
        }

        let op_total = (_to_u8(GLOBAL_DATA, 90) % 20) as usize;
        let mut cursor = 91usize;
        for _ in 0..op_total {
            if cursor >= GLOBAL_DATA.len() {
                break;
            }
            let op = _to_u8(GLOBAL_DATA, cursor);
            cursor += 1;
            match op % 12 {
                0 => {
                    let value = _to_u8(GLOBAL_DATA, cursor);
                    cursor += 1;
                    small_vec.push(value);
                }
                1 => {
                    small_vec.pop();
                }
                2 => {
                    if cursor + 8 <= GLOBAL_DATA.len() {
                        let index = _to_usize(GLOBAL_DATA, cursor);
                        cursor += 8;
                        small_vec.remove(index);
                    }
                }
                3 => {
                    if cursor + 9 <= GLOBAL_DATA.len() {
                        let index = _to_usize(GLOBAL_DATA, cursor);
                        cursor += 8;
                        let val = _to_u8(GLOBAL_DATA, cursor);
                        cursor += 1;
                        small_vec.insert(index, val);
                    }
                }
                4 => {
                    if cursor + 8 <= GLOBAL_DATA.len() {
                        let index = _to_usize(GLOBAL_DATA, cursor);
                        cursor += 8;
                        small_vec.swap_remove(index);
                    }
                }
                5 => {
                    let additional = _to_usize(GLOBAL_DATA, cursor);
                    cursor += 8;
                    small_vec.reserve(additional);
                }
                6 => {
                    let slice = small_vec.as_slice();
                    println!("{:?}", slice.len());
                }
                7 => {
                    let _ = small_vec.as_ptr();
                }
                8 => {
                    small_vec.clear();
                }
                9 => {
                    let truncate_len = _to_usize(GLOBAL_DATA, cursor);
                    cursor += 8;
                    small_vec.truncate(truncate_len);
                }
                10 => {
                    let _ = small_vec.clone();
                }
                _ => {
                    let _ = small_vec.drain(0..0);
                }
            }
        }

        let ptr = small_vec.as_mut_ptr();
        println!("{:?}", ptr);

        let _ = <SmallVec<[u8; 32]> as Index<usize>>::index(&small_vec, 0);
        let _ = <SmallVec<[u8; 32]> as IndexMut<usize>>::index_mut(&mut small_vec, 0);

        small_vec.shrink_to_fit();
        let _vec = small_vec.into_vec();
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