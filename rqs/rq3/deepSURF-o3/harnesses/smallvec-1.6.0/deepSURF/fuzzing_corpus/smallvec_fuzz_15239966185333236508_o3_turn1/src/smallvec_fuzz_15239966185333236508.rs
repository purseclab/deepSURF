#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::ops::{Deref, DerefMut, Index, IndexMut};
use std::str::FromStr;

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 128 {
            return;
        }
        set_global_data(data);
        let global = get_global_data();
        let first = global.first_half;
        let second = global.second_half;

        let ctor_sel = _to_u8(first, 0) % 6;
        let mut sv: SmallVec<[u8; 32]> = match ctor_sel {
            0 => SmallVec::<[u8; 32]>::new(),
            1 => {
                let cap = _to_usize(first, 1);
                SmallVec::<[u8; 32]>::with_capacity(cap)
            }
            2 => {
                let mut buf = [0u8; 32];
                for i in 0..32 {
                    buf[i] = _to_u8(second, i);
                }
                let len = _to_usize(first, 5) % 32;
                SmallVec::from_buf_and_len(buf, len)
            }
            3 => {
                let len = (_to_u8(first, 9) % 32) as usize;
                let slice_start = 10;
                let slice_end = slice_start + len;
                let slice = &second[slice_start..slice_end];
                SmallVec::from_slice(slice)
            }
            4 => {
                let len = (_to_u8(first, 42) % 32) as usize;
                let mut v = Vec::new();
                for i in 0..len {
                    v.push(_to_u8(second, 50 + i));
                }
                SmallVec::from_vec(v)
            }
            _ => {
                let elem = _to_u8(first, 60);
                let count = (_to_usize(first, 61) % 32) as usize;
                SmallVec::from_elem(elem, count)
            }
        };

        let loop_ops = (_to_u8(first, 62) % 10 + 1) as usize;
        for i in 0..loop_ops {
            let op = _to_u8(first, 63 + i) % 10;
            match op {
                0 => {
                    let val = _to_u8(second, (i * 3) % second.len());
                    sv.push(val);
                }
                1 => {
                    let _ = sv.pop();
                }
                2 => {
                    if !sv.is_empty() {
                        let idx = _to_usize(first, 70 + i) % sv.len();
                        sv.remove(idx);
                    }
                }
                3 => {
                    let add = _to_u8(second, (i * 5) % second.len());
                    let idx = _to_usize(first, 80 + i) % (sv.len() + 1);
                    sv.insert(idx, add);
                }
                4 => {
                    let amt = _to_usize(first, 90 + i);
                    sv.reserve(amt);
                }
                5 => {
                    let len = _to_usize(first, 100 + i) % 65;
                    let val = _to_u8(second, (i * 7) % second.len());
                    sv.resize(len, val);
                }
                6 => {
                    let len = _to_usize(first, 110 + i) % 65;
                    sv.truncate(len);
                }
                7 => {
                    let _ = sv.capacity();
                }
                8 => {
                    let _ = sv.is_empty();
                }
                _ => {}
            }
        }

        let mut comparator = |a: &mut u8, b: &mut u8| -> bool {
            let flip = _to_bool(first, 120);
            if flip {
                *a == *b
            } else {
                (*a % 2) == (*b % 2)
            }
        };
        sv.dedup_by(&mut comparator);

        let slice_ref = sv.as_slice();
        if !slice_ref.is_empty() {
            let first_elem = slice_ref[0];
            println!("{}", first_elem);
        }

        let mut_slice_ref = sv.as_mut_slice();
        if !mut_slice_ref.is_empty() {
            mut_slice_ref[0] = mut_slice_ref[0].wrapping_add(1);
        }

        let _final_len = sv.len();
        let _ = sv.deref();
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