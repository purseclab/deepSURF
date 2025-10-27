#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::SmallVec;
use global_data::*;

struct CustomType1(String);

fn main (){
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 256 {return;}
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let mut offset = 0;

        let op_constructor = _to_u8(GLOBAL_DATA, offset) % 6;
        offset += 1;

        let mut sv = match op_constructor {
            0 => SmallVec::<[u8; 64]>::new(),
            1 => {
                let cap = _to_usize(GLOBAL_DATA, offset);
                offset += 8;
                SmallVec::with_capacity(cap)
            }
            2 => {
                let elem = _to_u8(GLOBAL_DATA, offset);
                offset += 1;
                let count = _to_usize(GLOBAL_DATA, offset) % 65;
                offset += 8;
                SmallVec::from_elem(elem, count)
            }
            3 => {
                let len = _to_usize(GLOBAL_DATA, offset) % 65;
                offset += 8;
                let end = offset + len;
                if end > GLOBAL_DATA.len() { return; }
                SmallVec::from_slice(&GLOBAL_DATA[offset..end])
            }
            4 => {
                let len = _to_usize(GLOBAL_DATA, offset) % 65;
                offset += 8;
                let mut v = Vec::with_capacity(len);
                for i in 0..len {
                    v.push(_to_u8(GLOBAL_DATA, offset + i % 65));
                }
                SmallVec::from_vec(v)
            }
            _ => SmallVec::from_buf([0u8; 64])
        };

        while offset < GLOBAL_DATA.len() {
            let op = _to_u8(GLOBAL_DATA, offset) % 12;
            offset += 1;
            match op {
                0 => sv.push(_to_u8(GLOBAL_DATA, offset)),
                1 => { sv.pop(); },
                2 => sv.insert(
                    _to_usize(GLOBAL_DATA, offset),
                    _to_u8(GLOBAL_DATA, offset + 8)
                ),
                3 => { let _ = sv.remove(_to_usize(GLOBAL_DATA, offset)); },
                4 => sv.truncate(_to_usize(GLOBAL_DATA, offset)),
                5 => sv.reserve(_to_usize(GLOBAL_DATA, offset)),
                6 => sv.shrink_to_fit(),
                7 => sv.reserve_exact(_to_usize(GLOBAL_DATA, offset)),
                8 => sv.append(&mut SmallVec::from_buf([0u8; 64])),
                9 => {
                    let len = _to_usize(GLOBAL_DATA, offset) % 65;
                    let end = offset + len;
                    if end > GLOBAL_DATA.len() { break; }
                    sv.extend_from_slice(&GLOBAL_DATA[offset..end]);
                },
                10 => sv.dedup(),
                11 => sv.clear(),
                _ => (),
            };

            let _ = sv.as_slice();
            let _ = sv.as_mut_slice();
            let _ = sv.len();
            let _ = sv.capacity();
            let _ = sv.is_empty();

            offset = offset.saturating_add(8);
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