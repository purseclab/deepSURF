#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 128 { return; }
        set_global_data(data);
        let global = get_global_data();
        let g0 = global.first_half;
        let g1 = global.second_half;

        type SV = SmallVec<[u8; 32]>;

        let ctor_sel = _to_u8(g0, 0) % 6;
        let mut sv: SV = match ctor_sel {
            0 => SV::new(),
            1 => {
                let cap = _to_usize(g0, 1);
                SV::with_capacity(cap)
            }
            2 => {
                let len = (_to_u8(g0, 9) % 65) as usize;
                let slice_len = std::cmp::min(len, g1.len());
                SV::from_slice(&g1[..slice_len])
            }
            3 => {
                let len = (_to_u8(g0, 17) % 65) as usize;
                let vec_len = std::cmp::min(len, g1.len());
                SV::from_vec(g1[..vec_len].to_vec())
            }
            4 => {
                let elem = _to_u8(g0, 25);
                let n = (_to_u8(g0, 26) % 65) as usize;
                SV::from_elem(elem, n)
            }
            _ => {
                let len = (_to_u8(g0, 33) % 65) as usize;
                let iter_len = std::cmp::min(len, g1.len());
                SV::from_iter(g1[..iter_len].iter().cloned())
            }
        };

        let ops = (_to_u8(g0, 41) % 20) as usize;
        for i in 0..ops {
            let op = _to_u8(g0, 42 + i) % 11;
            match op {
                0 => {
                    let val = _to_u8(g1, i % g1.len());
                    sv.push(val);
                }
                1 => {
                    let _ = sv.pop();
                }
                2 => {
                    let idx = _to_usize(g1, i % (g1.len().saturating_sub(8)));
                    let val = _to_u8(g0, (50 + i) % g0.len());
                    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                        sv.insert(idx, val);
                    }));
                }
                3 => {
                    let idx = _to_usize(g0, (60 + i) % (g0.len().saturating_sub(8)));
                    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                        sv.remove(idx);
                    }));
                }
                4 => {
                    let idx = _to_usize(g1, i % (g1.len().saturating_sub(8)));
                    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                        sv.swap_remove(idx);
                    }));
                }
                5 => {
                    let len = _to_usize(g0, (70 + i) % (g0.len().saturating_sub(8)));
                    sv.truncate(len);
                }
                6 => {
                    let new_len = _to_usize(g0, (80 + i) % (g0.len().saturating_sub(8)));
                    sv.resize(new_len, _to_u8(g1, i % g1.len()));
                }
                7 => {
                    sv.retain(|x| *x % 2 == 0);
                }
                8 => {
                    let add_len = (_to_u8(g0, (90 + i) % g0.len()) % 65) as usize;
                    let slice_end = std::cmp::min(add_len, g1.len());
                    sv.extend_from_slice(&g1[..slice_end]);
                }
                9 => sv.clear(),
                _ => {
                    let _borrowed = sv.as_slice();
                    println!("{:?}", _borrowed.deref());
                }
            }
        }

        let idx_final = _to_usize(g0, 120);
        let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let removed = sv.remove(idx_final);
            println!("{:?}", removed);
        }));

        let _cap = sv.capacity();
        let _len = sv.len();
        let _is_empty = sv.is_empty();
        let _ = u8::from_str("255");
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