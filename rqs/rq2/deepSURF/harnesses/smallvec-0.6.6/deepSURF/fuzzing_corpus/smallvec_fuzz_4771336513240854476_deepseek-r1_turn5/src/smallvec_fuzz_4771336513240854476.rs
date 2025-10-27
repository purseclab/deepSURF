#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::SmallVec;
use global_data::*;
use std::cmp::Ordering;

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 2048 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let constructor_selector = _to_u8(GLOBAL_DATA, 0) % 4;
        let mut sv: SmallVec<[String; 16]> = match constructor_selector {
            0 => SmallVec::new(),
            1 => {
                let cap = _to_usize(GLOBAL_DATA, 1) % 65;
                SmallVec::with_capacity(cap)
            },
            2 => {
                let slice_len = _to_u8(GLOBAL_DATA, 1) % 65;
                let mut temp = (0..slice_len).map(|i| {
                    let offset = 2 + i as usize * 3;
                    let len = _to_u8(GLOBAL_DATA, offset) as usize % 10;
                    _to_str(GLOBAL_DATA, offset + 1, offset + 1 + len).to_string()
                }).collect();
                SmallVec::from_vec(temp)
            },
            3 => {
                let elem_count = _to_usize(GLOBAL_DATA, 1) % 65;
                let elem = _to_str(GLOBAL_DATA, 2, 2 + _to_u8(GLOBAL_DATA, 2) as usize % 10).to_string();
                SmallVec::from_elem(elem, elem_count)
            },
            _ => unreachable!()
        };

        let ops_cnt = _to_u8(GLOBAL_DATA, 100) % 16;
        for i in 0..ops_cnt {
            let op = _to_u8(GLOBAL_DATA, 101 + i as usize) % 7;
            match op {
                0 => {
                    let s = _to_str(GLOBAL_DATA, 120 + i as usize * 5, 125 + i as usize * 5).to_string();
                    sv.push(s);
                },
                1 => {
                    sv.pop();
                },
                2 => {
                    let idx = _to_usize(GLOBAL_DATA, 200 + i as usize) % (sv.len() + 1);
                    let s = _to_str(GLOBAL_DATA, 210 + i as usize * 3, 213 + i as usize * 3).to_string();
                    sv.insert(idx, s);
                },
                3 => sv.truncate(_to_usize(GLOBAL_DATA, 300 + i as usize) % (sv.len() + 1)),
                4 => sv.push(_to_str(GLOBAL_DATA, 400 + i as usize, 405 + i as usize).to_string()),
                5 => {
                    let cap = _to_usize(GLOBAL_DATA, 500 + i as usize);
                    sv.reserve(cap);
                },
                _ => {
                    let mut sv2 = SmallVec::new();
                    let cnt = _to_u8(GLOBAL_DATA, 600 + i as usize) % 8;
                    for j in 0..cnt {
                        let s = _to_str(GLOBAL_DATA, 610 + j as usize * 4, 614 + j as usize * 4).to_string();
                        sv2.push(s);
                    }
                    let _ = sv.partial_cmp(&sv2);
                    let _ = sv.as_slice();
                    sv.extend(sv2.drain());
                }
            }
        }

        {
            let mut drain = sv.drain();
            drain.next();
            drain.next_back();
        }

        println!("{:?}", sv.as_slice());
        println!("{:?}", sv.capacity());
        println!("{}", sv.is_empty());
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