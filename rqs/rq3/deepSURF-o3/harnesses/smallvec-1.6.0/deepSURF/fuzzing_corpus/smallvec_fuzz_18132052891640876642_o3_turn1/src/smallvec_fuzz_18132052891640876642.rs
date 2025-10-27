#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::ops::{Deref, DerefMut, Index, IndexMut};
use std::str::FromStr;

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 300 {
            return;
        }
        set_global_data(data);
        let global_data = get_global_data();
        let g1 = global_data.first_half;
        let g2 = global_data.second_half;

        let choice = _to_u8(g1, 0) % 5;
        let mut sv: SmallVec<[u8; 32]> = match choice {
            0 => SmallVec::new(),
            1 => {
                let cap = _to_usize(g1, 1);
                SmallVec::with_capacity(cap)
            }
            2 => {
                let len = (_to_u8(g1, 9) % 65) as usize;
                let mut v = Vec::new();
                for i in 0..len {
                    v.push(_to_u8(g1, 10 + i));
                }
                SmallVec::from_vec(v)
            }
            3 => {
                let len = (_to_u8(g1, 75) % 65) as usize;
                let slice = &g1[76..76 + len];
                SmallVec::from_slice(slice)
            }
            _ => {
                let elem = _to_u8(g1, 70);
                let n = _to_usize(g1, 60) % 65;
                SmallVec::from_elem(elem, n)
            }
        };

        let ops = _to_u8(g1, 150) % 25;
        for i in 0..ops {
            let op = _to_u8(g1, (151 + i as usize) % g1.len()) % 12;
            match op {
                0 => sv.push(_to_u8(g2, i as usize % g2.len())),
                1 => {
                    sv.pop();
                }
                2 => {
                    if !sv.is_empty() {
                        let idx = (_to_u8(g1, (160 + i as usize) % g1.len()) as usize) % sv.len();
                        sv.remove(idx);
                    }
                }
                3 => {
                    let add = _to_usize(g1, 170 % (g1.len() - 8));
                    let _ = sv.try_reserve(add);
                }
                4 => {
                    let new_len = (_to_u8(g2, i as usize % g2.len()) % 65) as usize;
                    sv.truncate(new_len.min(sv.len()));
                }
                5 => {
                    let len = (_to_u8(g2, i as usize % g2.len()) % 65) as usize;
                    if len <= g2.len() {
                        sv.extend_from_slice(&g2[..len]);
                    }
                }
                6 => {
                    if !sv.is_empty() {
                        let idx = (_to_u8(g1, (180 + i as usize) % g1.len()) as usize) % sv.len();
                        let val = _to_u8(g2, (i as usize + 3) % g2.len());
                        sv.insert(idx, val);
                    }
                }
                7 => sv.clear(),
                8 => {
                    let slice_ref = sv.as_slice();
                    if !slice_ref.is_empty() {
                        println!("{:?}", slice_ref[0]);
                    }
                }
                9 => println!("{:?}", sv.len()),
                10 => println!("{:?}", sv.capacity()),
                _ => sv.shrink_to_fit(),
            }
        }

        let additional = _to_usize(g1, 200);
        let _ = sv.try_reserve_exact(additional);

        let slice_after = sv.as_slice();
        println!("{:?}", slice_after.len());
        if !slice_after.is_empty() {
            println!("{:?}", slice_after[0]);
        }

        let cloned = sv.clone();
        println!("{:?}", cloned.as_slice());
        let vec_out = cloned.into_vec();
        println!("{:?}", vec_out.len());
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