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
        let SECOND = global_data.second_half;

        let mode = _to_u8(GLOBAL_DATA, 0) % 6;
        let mut buf = [0u8; 32];
        for i in 0..32 {
            buf[i] = _to_u8(SECOND, i % SECOND.len());
        }

        let mut sv: SmallVec<[u8; 32]> = match mode {
            0 => SmallVec::new(),
            1 => {
                let cap = _to_usize(GLOBAL_DATA, 2) % 65;
                SmallVec::with_capacity(cap)
            }
            2 => SmallVec::from_buf(buf),
            3 => {
                let len = _to_usize(GLOBAL_DATA, 4) % 32;
                SmallVec::from_buf_and_len(buf, len)
            }
            4 => {
                let slice_len =
                    (_to_u8(GLOBAL_DATA, 6) % (SECOND.len().min(32) as u8)) as usize;
                let slice = &SECOND[..slice_len];
                SmallVec::from_slice(slice)
            }
            _ => {
                let n = _to_usize(GLOBAL_DATA, 8) % 65;
                let elem = _to_u8(GLOBAL_DATA, 9);
                SmallVec::from_elem(elem, n)
            }
        };

        let operations = _to_u8(GLOBAL_DATA, 10) % 10;
        for i in 0..operations {
            let op_code = _to_u8(GLOBAL_DATA, 11 + i as usize);
            match op_code % 8 {
                0 => {
                    let val = _to_u8(GLOBAL_DATA, (12 + i as usize) % GLOBAL_DATA.len());
                    sv.push(val);
                }
                1 => {
                    sv.pop();
                }
                2 => {
                    let len = sv.len();
                    if len > 0 {
                        let idx = _to_usize(GLOBAL_DATA, 14) % len;
                        sv.remove(idx);
                    }
                }
                3 => {
                    let add = _to_usize(GLOBAL_DATA, 16) % 65;
                    sv.reserve(add);
                }
                4 => {
                    sv.clear();
                }
                5 => {
                    let new_len = _to_usize(GLOBAL_DATA, 18) % 65;
                    let fill = _to_u8(GLOBAL_DATA, 19);
                    sv.resize(new_len, fill);
                }
                6 => {
                    let len = sv.len();
                    let new_len = if len == 0 {
                        0
                    } else {
                        _to_usize(GLOBAL_DATA, 20) % len
                    };
                    sv.truncate(new_len);
                }
                _ => {
                    sv.dedup();
                }
            }
        }

        let cloned1 = (&sv).clone();
        let slice_ref = cloned1.as_slice();
        if let Some(first) = slice_ref.get(0) {
            println!("{}", first);
        }

        let _equal = sv == cloned1;
        if let Some(ord) = sv.partial_cmp(&cloned1) {
            println!("{:?}", ord);
        }

        let _capacity = sv.capacity();

        let vec_out = cloned1.into_vec();
        let slice2 = vec_out.as_slice();
        let smallvec2 = SmallVec::<[u8; 32]>::from_slice(slice2);
        let _cloned2 = (&smallvec2).clone();
        let deref_sample = *smallvec2.index(0);
        println!("{}", deref_sample);
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