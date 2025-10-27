#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::ops::{Deref, DerefMut, Index, IndexMut};

fn _custom_fn0(_: &mut u8) -> bool {
    let global_data = get_global_data();
    let GLOBAL_DATA = global_data.first_half;
    let t_9 = _to_u8(GLOBAL_DATA, 34);
    if t_9 % 2 == 0 {
        panic!("INTENTIONAL PANIC!");
    }
    _to_bool(GLOBAL_DATA, 35)
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 256 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let mut sv = match _to_u8(GLOBAL_DATA, 0) % 4 {
            0 => {
                let cap = _to_usize(GLOBAL_DATA, 1);
                SmallVec::<[u8; 64]>::with_capacity(cap)
            }
            1 => {
                let start = _to_usize(GLOBAL_DATA, 5);
                let len = _to_usize(GLOBAL_DATA, 13) % 65;
                let end = start.saturating_add(len);
                let slice = if end > GLOBAL_DATA.len() { &[] } else { &GLOBAL_DATA[start..end] };
                SmallVec::from_slice(slice)
            }
            2 => {
                let mut buf = [0u8; 64];
                let start = _to_usize(GLOBAL_DATA, 15);
                let count = std::cmp::min(64, GLOBAL_DATA.len().saturating_sub(start));
                buf[..count].copy_from_slice(&GLOBAL_DATA[start..start.saturating_add(count)]);
                SmallVec::from_buf_and_len(buf, _to_usize(GLOBAL_DATA, 79))
            }
            _ => {
                let elem = _to_u8(GLOBAL_DATA, 120);
                SmallVec::<[u8; 64]>::from_elem(elem, _to_usize(GLOBAL_DATA, 121))
            }
        };

        let op_count = _to_u8(GLOBAL_DATA, 83) % 8;
        for i in 0..op_count {
            match _to_u8(GLOBAL_DATA, 84 + i as usize) % 9 {
                0 => sv.push(_to_u8(GLOBAL_DATA, 92 + i as usize)),
                1 => sv.insert(_to_usize(GLOBAL_DATA, 100 + i as usize), _to_u8(GLOBAL_DATA, 108 + i as usize)),
                2 => { let _ = sv.pop(); }
                3 => sv.truncate(_to_usize(GLOBAL_DATA, 116 + i as usize)),
                4 => {
                    let start = _to_usize(GLOBAL_DATA, 124 + i as usize);
                    let end = _to_usize(GLOBAL_DATA, 132 + i as usize);
                    let _ = sv.drain(start..end);
                }
                5 => sv.as_mut_slice().reverse(),
                6 => sv.extend_from_slice(&GLOBAL_DATA[140..160]),
                7 => sv.shrink_to_fit(),
                8 => {
                    let new_cap = _to_usize(GLOBAL_DATA, 160 + i as usize);
                    let _ = sv.try_reserve(new_cap);
                }
                _ => ()
            }
        }

        sv.retain(_custom_fn0);

        for _ in 0..(_to_usize(GLOBAL_DATA, 170) % 4) {
            match _to_u8(GLOBAL_DATA, 178) % 5 {
                0 => {
                    let mut tmp: SmallVec<[u8; 64]> = SmallVec::from_slice(sv.as_slice());
                    sv.append(&mut tmp);
                }
                1 => {
                    let slice = sv.as_mut_slice();
                    if !slice.is_empty() {
                        *slice.last_mut().unwrap() = _to_u8(GLOBAL_DATA, 182);
                    }
                }
                2 => {
                    let other = SmallVec::from_slice(&GLOBAL_DATA[190..210]);
                    let _ = sv.cmp(&other);
                }
                3 => {
                    let index = _to_usize(GLOBAL_DATA, 210);
                    if index < sv.len() {
                        let _ = sv.swap_remove(index);
                    }
                }
                4 => {
                    let mut iter = sv.into_iter();
                    let _ = iter.next_back();
                    sv = iter.collect();
                }
                _ => ()
            }
        }

        let _ = sv.partial_cmp(&SmallVec::new());
        println!("{:?}", sv.as_slice());
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