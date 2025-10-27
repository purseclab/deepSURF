#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::ops::{DerefMut, IndexMut};

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 1024 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let g = global_data.first_half;

        let mut sv = match _to_u8(g, 0) % 5 {
            0 => SmallVec::<[u8; 256]>::new(),
            1 => SmallVec::with_capacity(_to_usize(g, 1)),
            2 => SmallVec::from_slice(&g[_to_usize(g, 3) % g.len()..][.._to_usize(g, 5) % 65]),
            3 => SmallVec::from_elem(_to_u8(g, 7), _to_usize(g, 8) % 65),
            _ => {
                let buf: [u8; 256] = g[..256].try_into().unwrap();
                SmallVec::from_buf_and_len(buf, _to_usize(g, 256))
            }
        };

        for i in 0.._to_u8(g, 300) % 65 {
            match _to_u8(g, 301 + i as usize) % 9 {
                0 => sv.push(_to_u8(g, 400 + i as usize)),
                1 => { sv.pop(); }
                2 => sv.truncate(_to_usize(g, 500 + i as usize)),
                3 => sv.extend_from_slice(&g[_to_usize(g, 550) % g.len()..][.._to_usize(g, 600) % 65]),
                4 => sv.reserve(_to_usize(g, 650)),
                5 => sv.insert(_to_usize(g, 700), _to_u8(g, 701)),
                6 => { sv.swap_remove(_to_usize(g, 702)); }
                7 => { 
                    let s = sv.as_mut_slice();
                    if !s.is_empty() {
                        println!("{:?}", s.as_ptr());
                        s[_to_usize(g, 703) % s.len()] = _to_u8(g, 704);
                    }
                }
                _ => {
                    let d = sv.deref_mut();
                    if !d.is_empty() {
                        let idx = _to_usize(g, 705) % d.len();
                        d[idx] = _to_u8(g, 706);
                        println!("{}", d[idx]);
                    }
                }
            }
        }

        let _ = sv.deref_mut();
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