#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use simple_slab::*;
use global_data::*;
use std::ops::{Index, IndexMut};

#[derive(Debug)]
struct CustomType0(String);

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 2048 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let mut slab = match _to_u8(GLOBAL_DATA, 0) % 3 {
            0 => simple_slab::Slab::<CustomType0>::new(),
            1 => simple_slab::Slab::with_capacity(_to_usize(GLOBAL_DATA, 8)),
            _ => {
                let mut s = simple_slab::Slab::new();
                s.insert(CustomType0(String::new()));
                s
            }
        };

        let ops = _to_usize(GLOBAL_DATA, 16) % 65;
        for i in 0..ops {
            match _to_u8(GLOBAL_DATA, 24 + i) % 8 {
                0 => {
                    let idx1 = _to_usize(GLOBAL_DATA, 32 + i*16);
                    let idx2 = _to_usize(GLOBAL_DATA, 40 + i*16);
                    if idx1 < GLOBAL_DATA.len() && idx2 < GLOBAL_DATA.len() && idx1 < idx2 {
                        let s = _to_str(GLOBAL_DATA, idx1, idx2);
                        slab.insert(CustomType0(s.to_string()));
                    }
                }
                1 => {
                    let _ = slab.remove(_to_usize(GLOBAL_DATA, 32 + i*8));
                }
                2 => {
                    let mut iter = slab.iter();
                    for _ in 0.._to_usize(GLOBAL_DATA, 32 + i*8) % 10 {
                        if let Some(x) = iter.next() { println!("{:?}", x); }
                    }
                }
                3 => {
                    let mut iter = slab.iter_mut();
                    for _ in 0.._to_usize(GLOBAL_DATA, 32 + i*8) % 10 {
                        if let Some(x) = iter.next() { *x = CustomType0("modified".into()); }
                    }
                }
                4 => {
                    let idx = _to_usize(GLOBAL_DATA, 32 + i*8);
                    let _ = slab.index(idx);
                }
                5 => {
                    let _: Vec<_> = slab.into_iter().take(_to_usize(GLOBAL_DATA, 32 + i*8) % 10).collect();
                    slab = match _to_u8(GLOBAL_DATA, global_data.second_half[i % global_data.second_half.len()].into()) % 2 {
                        0 => simple_slab::Slab::new(),
                        _ => simple_slab::Slab::with_capacity(_to_usize(GLOBAL_DATA, global_data.second_half[i % global_data.second_half.len()].into())),
                    };
                }
                6 => {
                    drop(slab);
                    slab = match _to_u8(GLOBAL_DATA, global_data.second_half[i % global_data.second_half.len()].into()) % 2 {
                        0 => simple_slab::Slab::new(),
                        _ => simple_slab::Slab::with_capacity(_to_usize(GLOBAL_DATA, global_data.second_half[i % global_data.second_half.len()].into())),
                    };
                }
                7 => {
                    let _ = _to_usize(GLOBAL_DATA, 32 + i*8);
                    slab.insert(CustomType0(String::from("fuzz")));
                }
                _ => unreachable!()
            }
        }

        let final_index = _to_usize(GLOBAL_DATA, GLOBAL_DATA.len().saturating_sub(8));
        let _ = slab.index(final_index);
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