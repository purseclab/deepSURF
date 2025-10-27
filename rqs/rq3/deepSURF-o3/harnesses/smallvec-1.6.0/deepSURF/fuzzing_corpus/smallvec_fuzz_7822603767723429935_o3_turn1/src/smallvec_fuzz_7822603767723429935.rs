#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::ops::{Deref, DerefMut, Index, IndexMut};

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 300 {
            return;
        }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        type SV32 = SmallVec<[i32; 32]>;

        let choice = _to_u8(GLOBAL_DATA, 0);
        let mut sv: SV32 = match choice % 8 {
            0 => SV32::new(),
            1 => {
                let cap = (_to_usize(GLOBAL_DATA, 1) % 65).max(1);
                SV32::with_capacity(cap)
            }
            2 => {
                let elem = _to_i32(GLOBAL_DATA, 5);
                let n = (_to_usize(GLOBAL_DATA, 9) % 65).max(1);
                SV32::from_elem(elem, n)
            }
            3 => {
                let n = (_to_u8(GLOBAL_DATA, 13) % 65) as usize;
                let mut v = Vec::with_capacity(n);
                for i in 0..n {
                    v.push(_to_i32(GLOBAL_DATA, 14 + 4 * i));
                }
                SV32::from_vec(v)
            }
            4 => {
                let n = (_to_u8(GLOBAL_DATA, 100) % 65) as usize;
                let mut tmp = Vec::with_capacity(n);
                for i in 0..n {
                    tmp.push(_to_i32(GLOBAL_DATA, 101 + 4 * i));
                }
                SV32::from_slice(&tmp)
            }
            5 => {
                let n = (_to_u8(GLOBAL_DATA, 140) % 65) as usize;
                let iter = (0..n).map(|i| _to_i32(GLOBAL_DATA, 141 + 4 * i));
                SV32::from_iter(iter)
            }
            6 => {
                let val = _to_i32(GLOBAL_DATA, 200);
                let buf = [val; 32];
                SV32::from_buf(buf)
            }
            _ => {
                let mut buf = [0i32; 32];
                for i in 0..32 {
                    buf[i] = _to_i32(GLOBAL_DATA, 50 + 4 * i);
                }
                let len = (_to_usize(GLOBAL_DATA, 300) % 33).max(1);
                SV32::from_buf_and_len(buf, len)
            }
        };

        sv.reserve(_to_usize(GLOBAL_DATA, 60));

        if sv.len() > 0 {
            let idx = _to_usize(GLOBAL_DATA, 64);
            let _ = sv.remove(idx);
        }

        let slice_mut = sv.as_mut();
        if !slice_mut.is_empty() {
            slice_mut[0] = slice_mut[0].wrapping_add(1);
            println!("{:?}", slice_mut.len());
        }

        sv.push(_to_i32(GLOBAL_DATA, 68));
        sv.truncate(_to_usize(GLOBAL_DATA, 72));

        let sv_clone = sv.clone();
        let _ = sv.cmp(&sv_clone);
        let _ = sv.partial_cmp(&sv_clone);

        let _drain = sv.drain(0..(_to_usize(GLOBAL_DATA, 76) % (sv.len() + 1)));
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