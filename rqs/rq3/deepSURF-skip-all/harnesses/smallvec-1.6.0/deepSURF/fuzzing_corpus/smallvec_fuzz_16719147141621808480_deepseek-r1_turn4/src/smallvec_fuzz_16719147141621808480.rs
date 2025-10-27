#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::ops::{Deref, DerefMut};

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 512 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let g = global_data.first_half;

        let ctor_choice = _to_u8(g, 0) % 6;
        let mut sv = match ctor_choice {
            0 => SmallVec::<[u8; 64]>::new(),
            1 => SmallVec::with_capacity(_to_usize(g, 1)),
            2 => SmallVec::from_vec(vec![_to_u8(g, 2); _to_usize(g, 3) % 65]),
            3 => {
                let mut buf = [0u8; 64];
                for i in 0..64 {
                    buf[i] = _to_u8(g, 4 + i);
                }
                SmallVec::from_buf_and_len(buf, _to_usize(g, 68) % 65)
            },
            4 => SmallVec::from_slice(&g[100..100 + _to_usize(g, 132) % 65]),
            5 => SmallVec::from_elem(_to_u8(g, 197), _to_usize(g, 198) % 65),
            _ => unreachable!(),
        };

        let ops = _to_usize(g, 200) % 15;
        for i in 0..ops {
            let op = _to_u8(g, 201 + i) % 9;
            match op {
                0 => sv.reserve_exact(_to_usize(g, 216 + i)),
                1 => sv.extend_from_slice(&g[300..300 + _to_usize(g, 231 + i) % 65]),
                2 => {
                    if !sv.is_empty() {
                        sv.remove(_to_usize(g, 246 + i) % sv.len());
                    }
                },
                3 => {
                    let mut other = SmallVec::<[u8; 64]>::from_slice(sv.as_slice());
                    sv.append(&mut other);
                },
                4 => {
                    let idx = _to_usize(g, 261 + i) % (sv.len() + 1);
                    sv.insert(idx, _to_u8(g, 276 + i));
                },
                5 => {
                    let cmp_result = sv.partial_cmp(&SmallVec::from_slice(sv.as_slice()));
                    println!("{:?}", cmp_result);
                },
                6 => sv.shrink_to_fit(),
                7 => {
                    let drained: Vec<_> = sv.drain(..).collect();
                    println!("Drained {} items", drained.len());
                },
                8 => {
                    let cloned = sv.clone();
                    sv.extend(cloned);
                },
                _ => (),
            }
        }

        {
            let drain_start = _to_usize(g, 400) % (sv.len() + 1);
            let drain_end = _to_usize(g, 401) % (sv.len() + 1);
            let drain_range = drain_start.min(drain_end)..drain_start.max(drain_end);
            let _ = sv.drain(drain_range);
        }

        if _to_usize(g, 450) % 2 == 0 {
            sv.clear();
            sv.extend((0.._to_usize(g, 451) % 65).map(|i| _to_u8(g, 452 + i)));
        }

        if !sv.is_empty() {
            let idx = _to_usize(g, 500) % sv.len();
            *sv.get_mut(idx).unwrap() = _to_u8(g, 501);
            println!("Element {}: {:?}", idx, sv.as_ptr());
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