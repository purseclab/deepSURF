#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::ops::{Deref, DerefMut, Index, IndexMut};
use std::io::Write;

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 2000 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let mut vec_len = _to_usize(GLOBAL_DATA, 0) % 65;
        let mut items = Vec::new();
        let mut data_idx = 1;

        for _ in 0..vec_len {
            if data_idx >= GLOBAL_DATA.len() { break; }
            items.push(_to_u8(GLOBAL_DATA, data_idx));
            data_idx += 1;
        }

        let constructor_choice = _to_u8(GLOBAL_DATA, data_idx) % 5;
        data_idx += 1;

        let mut sv: SmallVec<[u8; 32]> = match constructor_choice {
            0 => SmallVec::new(),
            1 => SmallVec::with_capacity(_to_usize(GLOBAL_DATA, data_idx)),
            2 => SmallVec::from_slice(&items),
            3 => SmallVec::from_vec(items.clone()),
            4 => SmallVec::from_elem(_to_u8(GLOBAL_DATA, data_idx), _to_usize(GLOBAL_DATA, data_idx + 1)),
            _ => unreachable!(),
        };

        let op_count = _to_u8(GLOBAL_DATA, data_idx) % 16;
        data_idx += 1;

        for _ in 0..op_count {
            let op = _to_u8(GLOBAL_DATA, data_idx) % 10;
            data_idx += 1;

            match op {
                0 => sv.push(_to_u8(GLOBAL_DATA, data_idx)),
                1 => {
                    let idx = _to_usize(GLOBAL_DATA, data_idx);
                    data_idx += std::mem::size_of::<usize>();
                    sv.insert(idx, _to_u8(GLOBAL_DATA, data_idx));
                }
                2 => {
                    let idx = _to_usize(GLOBAL_DATA, data_idx);
                    data_idx += std::mem::size_of::<usize>();
                    let _ = sv.remove(idx);
                }
                3 => {
                    let len = _to_usize(GLOBAL_DATA, data_idx);
                    data_idx += std::mem::size_of::<usize>();
                    sv.truncate(len);
                }
                4 => {
                    let capacity = _to_usize(GLOBAL_DATA, data_idx);
                    data_idx += std::mem::size_of::<usize>();
                    sv.reserve(capacity);
                }
                5 => {
                    let _ = sv.pop();
                }
                6 => {
                    let _ = sv.as_slice();
                }
                7 => sv.dedup(),
                8 => {
                    let other = SmallVec::from_slice(&items);
                    let _ = sv.partial_cmp(&other);
                }
                9 => {
                    sv.extend_from_slice(&items);
                }
                _ => {}
            }

            data_idx += match op {
                0 | 4 | 5 | 6 | 7 | 8 | 9 => 0,
                1 | 3 => 1,
                _ => std::mem::size_of::<usize>(),
            };

            if !sv.is_empty() {
                println!("{:?}", sv[0]);
                let slice = sv.as_slice();
                println!("{:?}", slice.len());
            }
        }

        sv.dedup();

        let ops_after = _to_u8(GLOBAL_DATA, data_idx) % 6;
        data_idx += 1;

        match ops_after {
            0 => sv.extend_from_slice(&items),
            1 => sv.shrink_to_fit(),
            2 => {
                let idx_range = _to_usize(GLOBAL_DATA, data_idx)..;
                println!("{:?}", &sv[idx_range]);
            }
            3 => {
                if let Some(e) = sv.get_mut(0) {
                    *e = e.wrapping_add(1);
                }
            }
            4 => {
                let _ = sv.clone();
            }
            5 => {
                let mut buf = [0u8; 128];
                let len = std::cmp::min(sv.len(), buf.len());
                buf[..len].copy_from_slice(&sv[..len]);
                let _ = sv.as_mut_slice().write_all(&buf);
            }
            _ => {}
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