#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::ops::{Deref, DerefMut, Index, IndexMut};

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 256 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let (fh, sh) = (global_data.first_half, global_data.second_half);

        let ctor_sel = _to_u8(fh, 0) % 6;
        let mut sv = match ctor_sel {
            0 => SmallVec::<[u8; 64]>::new(),
            1 => {
                let cap = _to_usize(fh, 1);
                SmallVec::with_capacity(cap)
            }
            2 => {
                let elem = _to_u8(fh, 1);
                let n = _to_usize(fh, 2) % 65;
                SmallVec::from_elem(elem, n)
            }
            3 => {
                let len = _to_usize(fh, 1) % 65;
                let vec = sh[..len.min(sh.len())].to_vec();
                SmallVec::from_vec(vec)
            }
            4 => {
                let len = _to_usize(fh, 1);
                let slice = &sh[..len.min(sh.len())];
                SmallVec::from_slice(slice)
            }
            _ => {
                let mut buf = [0; 64];
                for i in 0..64 {
                    buf[i] = _to_u8(sh, i);
                }
                SmallVec::from_buf(buf)
            }
        };

        let ops = _to_u8(fh, 9) % 65;
        let mut pos = 10;
        for _ in 0..ops {
            if pos >= fh.len() { break; }
            match _to_u8(fh, pos) % 7 {
                0 => sv.push(_to_u8(sh, pos)),
                1 => { sv.pop(); }
                2 => sv.truncate(_to_usize(fh, pos + 1)),
                3 => {
                    let idx = _to_usize(fh, pos + 1);
                    let val = _to_u8(fh, pos + 9);
                    sv.insert(idx, val);
                }
                4 => {
                    let sl = sv.as_slice();
                    if !sl.is_empty() {
                        let idx = _to_usize(fh, pos + 1) % sl.len();
                        println!("{}", sl[idx]);
                    }
                }
                5 => {
                    let mut drain = sv.drain();
                    while let Some(e) = drain.next() { println!("{}", e); }
                }
                _ => {
                    let cap = sv.capacity();
                    sv.reserve(cap);
                }
            }
            pos += _to_u8(fh, pos + 1) as usize % 16 + 1;
        }

        let result = sv.into_inner();
        if let Ok(arr) = result { println!("{:?}", arr); }
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