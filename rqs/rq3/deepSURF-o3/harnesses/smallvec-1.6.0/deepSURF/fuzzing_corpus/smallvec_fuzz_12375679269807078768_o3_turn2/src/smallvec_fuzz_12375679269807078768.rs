#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::hint::black_box;
use std::ops::RangeBounds;

fn next_byte(slice: &[u8], cursor: &mut usize) -> u8 {
    let b = _to_u8(slice, *cursor % slice.len());
    *cursor += 1;
    b
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 140 {
            return;
        }
        set_global_data(data);
        let global_data = get_global_data();
        let fh = global_data.first_half;
        let mut cursor = 0usize;

        let constructor_selector = next_byte(fh, &mut cursor);
        let mut small_vec: SmallVec<[u8; 64]> = match constructor_selector % 5 {
            0 => SmallVec::new(),
            1 => {
                let len = (next_byte(fh, &mut cursor) as usize % 65) as usize;
                let mut v = Vec::new();
                for _ in 0..len {
                    v.push(next_byte(fh, &mut cursor));
                }
                SmallVec::from_vec(v)
            }
            2 => {
                let len = (next_byte(fh, &mut cursor) as usize % fh.len()).min(64);
                SmallVec::from_slice(&fh[..len])
            }
            3 => {
                let mut buf = [0u8; 64];
                for i in 0..64 {
                    buf[i] = next_byte(fh, &mut cursor);
                }
                let len = (next_byte(fh, &mut cursor) as usize) % 64;
                SmallVec::from_buf_and_len(buf, len)
            }
            _ => {
                let cap = (next_byte(fh, &mut cursor) as usize % 65) + 1;
                SmallVec::with_capacity(cap)
            }
        };

        let ops = (next_byte(fh, &mut cursor) % 20) + 1;
        for _ in 0..ops {
            match next_byte(fh, &mut cursor) % 17 {
                0 => small_vec.push(next_byte(fh, &mut cursor)),
                1 => {
                    small_vec.pop();
                }
                2 => {
                    if !small_vec.is_empty() {
                        let idx = _to_usize(fh, cursor % fh.len());
                        let _ = small_vec.remove(idx % small_vec.len());
                    }
                }
                3 => small_vec.reserve(_to_usize(fh, cursor % fh.len())),
                4 => {
                    let _ = small_vec.try_reserve(_to_usize(fh, cursor % fh.len()));
                }
                5 => small_vec.truncate(_to_usize(fh, cursor % fh.len())),
                6 => {
                    if !small_vec.is_empty() {
                        let idx = _to_usize(fh, cursor % fh.len()) % small_vec.len();
                        small_vec.insert(idx, next_byte(fh, &mut cursor));
                    }
                }
                7 => {
                    if !small_vec.is_empty() {
                        let idx = _to_usize(fh, cursor % fh.len()) % small_vec.len();
                        let _ = small_vec.swap_remove(idx);
                    }
                }
                8 => small_vec.clear(),
                9 => {
                    let slice_len = (next_byte(fh, &mut cursor) as usize % fh.len()).min(32);
                    small_vec.extend_from_slice(&fh[..slice_len]);
                }
                10 => small_vec.dedup(),
                11 => {
                    let new_len = (next_byte(fh, &mut cursor) as usize) % 65;
                    small_vec.resize(new_len, next_byte(fh, &mut cursor));
                }
                12 => {
                    small_vec.retain(|e| {
                        if *e == next_byte(fh, &mut cursor) {
                            panic!("INTENTIONAL PANIC!");
                        }
                        *e % 2 == 0
                    });
                }
                13 => {
                    let slice = small_vec.as_slice();
                    if let Some(first) = slice.first() {
                        let val = *first;
                        black_box(val);
                    }
                }
                14 => {
                    let mut clone_sv = small_vec.clone();
                    let _ = small_vec.partial_cmp(&clone_sv);
                    let _ = small_vec.cmp(&clone_sv);
                    if !clone_sv.is_empty() {
                        let range_end = clone_sv.len() / 2;
                        let mut drain_it = clone_sv.drain(0..range_end);
                        let _ = drain_it.next();
                        let _ = drain_it.next_back();
                    }
                }
                15 => {
                    let _boxed: Box<[u8]> = small_vec.clone().into_boxed_slice();
                    black_box(_boxed.len());
                }
                _ => small_vec.shrink_to_fit(),
            }
        }

        let _ = small_vec.len();
        small_vec.shrink_to_fit();
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