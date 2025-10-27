#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 512 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let second_half = global_data.second_half;

        let constructor_selector = _to_u8(GLOBAL_DATA, 0) % 4;
        let mut sv = match constructor_selector {
            0 => SmallVec::<[u8;32]>::new(),
            1 => {
                let cap = _to_usize(GLOBAL_DATA, 1);
                SmallVec::with_capacity(cap)
            },
            2 => {
                let len = (_to_usize(GLOBAL_DATA, 1) % 65).min(GLOBAL_DATA.len().saturating_sub(8)/8);
                let vec = (0..len).map(|i| _to_u8(GLOBAL_DATA, 8 + i)).collect();
                SmallVec::from_vec(vec)
            },
            3 => {
                let len = 32.min(GLOBAL_DATA.len().saturating_sub(1));
                SmallVec::from_slice(&GLOBAL_DATA[1..=len])
            },
            _ => unreachable!()
        };

        let op_count = _to_usize(second_half, 0) % 8;
        let mut offset = 8;
        for _ in 0..op_count {
            if offset >= second_half.len() { break; }
            match _to_u8(second_half, offset) % 7 {
                0 => {
                    offset += 1;
                    if offset < second_half.len() {
                        let value = _to_u8(second_half, offset);
                        sv.push(value);
                        offset += 1;
                    }
                },
                1 => {
                    offset += 1;
                    if offset + 8 < second_half.len() {
                        let idx = _to_usize(second_half, offset);
                        offset += 8;
                        let val = _to_u8(second_half, offset);
                        sv.insert(idx, val);
                        offset += 1;
                    }
                },
                2 => {
                    offset += 1;
                    if offset + 8 < second_half.len() {
                        sv.truncate(_to_usize(second_half, offset));
                        offset += 8;
                    }
                },
                3 => {
                    offset += 1;
                    println!("{:?}", sv.as_slice());
                },
                4 => {
                    offset += 1;
                    if offset + 8 < second_half.len() {
                        sv.swap_remove(_to_usize(second_half, offset));
                        offset += 8;
                    }
                },
                5 => {
                    offset += 1;
                    let _ = sv.pop();
                },
                6 => {
                    offset += 1;
                    if offset + 8 < second_half.len() {
                        sv.remove(_to_usize(second_half, offset));
                        offset += 8;
                    }
                },
                _ => ()
            }
        }

        if !sv.is_empty() {
            let idx = _to_usize(second_half, second_half.len().saturating_sub(8)) % sv.len();
            sv.remove(idx);
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