#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 72 {
            return;
        }
        set_global_data(data);
        let global_data = get_global_data();
        let fh = global_data.first_half;
        let sh = global_data.second_half;

        let selector = _to_u8(fh, 0) % 4;
        let capacity = _to_usize(fh, 1);

        let mut sv: SmallVec<[u8; 32]> = match selector {
            0 => SmallVec::with_capacity(capacity),
            1 => SmallVec::new(),
            2 => {
                let len_vec = (_to_u8(fh, 5) % 65) as usize;
                let mut v = Vec::new();
                for i in 0..len_vec {
                    v.push(_to_u8(sh, i % sh.len()));
                }
                SmallVec::from_vec(v)
            }
            _ => {
                let len_elem = (_to_u8(fh, 7) % 65) as usize;
                SmallVec::from_elem(_to_u8(fh, 6), len_elem)
            }
        };

        let ops = _to_u8(fh, 8) % 20;
        let idx_val = _to_usize(fh, 20);
        let additional = _to_usize(fh, 28);

        for i in 0..ops {
            let op = _to_u8(sh, i as usize % sh.len()) % 12;
            match op {
                0 => sv.push(_to_u8(fh, 9 + i as usize)),
                1 => {
                    sv.pop();
                }
                2 => {
                    sv.reserve(additional);
                }
                3 => {
                    sv.resize((_to_u8(fh, 10 + i as usize) % 65) as usize, _to_u8(sh, i as usize % sh.len()));
                }
                4 => {
                    let _ = sv.try_reserve_exact(additional);
                }
                5 => {
                    if sv.len() > 0 {
                        let _ = sv.remove(idx_val);
                    }
                }
                6 => sv.insert(idx_val, _to_u8(sh, (i + 1) as usize % sh.len())),
                7 => {
                    let mut other = SmallVec::<[u8; 32]>::with_capacity(capacity / 2 + 1);
                    other.push(_to_u8(fh, 11));
                    sv.append(&mut other);
                }
                8 => sv.clear(),
                9 => sv.truncate((_to_u8(sh, i as usize % sh.len()) % 65) as usize),
                10 => {
                    let slice_ref = sv.as_slice();
                    if !slice_ref.is_empty() {
                        let _ = slice_ref[0];
                    }
                }
                _ => {
                    let _cmp_clone = sv.clone();
                    let _ = sv.cmp(&_cmp_clone);
                }
            }
        }

        let mut sv2 = SmallVec::<[u8; 32]>::with_capacity(capacity / 2 + 1);
        let _ = sv.partial_cmp(&sv2);
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