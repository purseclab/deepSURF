#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::cmp::Ordering;

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 256 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let mut sv = match _to_u8(GLOBAL_DATA, 0) % 3 {
            0 => {
                let buf_size = 64;
                let mut buf = [0u8; 64];
                let buf_data = _to_slice(GLOBAL_DATA, 1, buf_size);
                buf.copy_from_slice(buf_data);
                SmallVec::from_buf_and_len(buf, _to_usize(GLOBAL_DATA, 65) % buf_size)
            }
            1 => {
                let capacity = _to_usize(GLOBAL_DATA, 1) % 128;
                SmallVec::<[u8; 64]>::with_capacity(capacity)
            }
            2 => {
                let elem = _to_u8(GLOBAL_DATA, 1);
                let count = _to_usize(GLOBAL_DATA, 2) % 65;
                SmallVec::<[u8; 64]>::from_elem(elem, count)
            }
            _ => unreachable!(),
        };

        for _ in 0..3 {
            match _to_u8(GLOBAL_DATA, 20) % 5 {
                0 => {
                    sv.push(_to_u8(GLOBAL_DATA, 30));
                }
                1 => {
                    if !sv.is_empty() {
                        sv.pop();
                    }
                }
                2 => {
                    let idx = _to_usize(GLOBAL_DATA, 40) % (sv.len() + 1);
                    sv.insert(idx, _to_u8(GLOBAL_DATA, 48));
                }
                3 => {
                    let new_len = _to_usize(GLOBAL_DATA, 56) % (sv.len() + 1);
                    sv.truncate(new_len);
                }
                4 => {
                    let additional = _to_usize(GLOBAL_DATA, 64);
                    let _ = sv.try_reserve(additional);
                }
                _ => (),
            }
        }

        let _ = sv.try_grow(_to_usize(GLOBAL_DATA, 72));

        if let Some(first) = sv.first() {
            println!("{:?}", first);
        }

        let mut sv2 = match _to_u8(GLOBAL_DATA, 80) % 2 {
            0 => SmallVec::from_slice(sv.as_slice()),
            1 => {
                let mut v = Vec::with_capacity(sv.len());
                v.extend(sv.drain(..));
                SmallVec::from_vec(v)
            }
            _ => unreachable!(),
        };

        if _to_u8(GLOBAL_DATA, 90) % 2 == 0 {
            sv2.extend_from_slice(sv.as_slice());
        }

        match sv.partial_cmp(&sv2) {
            Some(Ordering::Less) => (),
            Some(Ordering::Greater) => (),
            Some(Ordering::Equal) => (),
            None => (),
        }

        let _ = sv.capacity();
        let _ = sv2.into_vec();
    });
}

fn _to_slice(data: &[u8], start: usize, end: usize) -> &[u8] {
    &data[start..end]
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