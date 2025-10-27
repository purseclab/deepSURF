#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::SmallVec;
use global_data::*;

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 2048 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let (first_half, second_half) = (global_data.first_half, global_data.second_half);

        let constructor_selector = _to_u8(first_half, 0) % 5;
        let mut sv = match constructor_selector {
            0 => SmallVec::<[u8; 64]>::new(),
            1 => SmallVec::with_capacity(_to_usize(first_half, 1)),
            2 => {
                let start = _to_usize(first_half, 1);
                let end = _to_usize(first_half, 9);
                if end <= second_half.len() && start <= end {
                    SmallVec::from_slice(&second_half[start..end])
                } else {
                    return;
                }
            },
            3 => SmallVec::from_vec(second_half.iter().take(_to_usize(first_half, 1) % 65).cloned().collect()),
            4 => SmallVec::from_elem(_to_u8(first_half, 10), _to_usize(first_half, 11) % 65),
            _ => unreachable!(),
        };

        let op_count = _to_u8(first_half, 17) % 16;
        for i in 0..op_count {
            let op_byte = _to_u8(first_half, 18 + i as usize) % 10;
            match op_byte {
                0 => sv.push(_to_u8(first_half, 34 + i as usize)),
                1 => { sv.pop(); },
                2 => sv.insert(_to_usize(first_half, 50 + i as usize), _to_u8(first_half, 66 + i as usize)),
                3 => sv.extend_from_slice(&second_half[_to_usize(first_half, 82 + i as usize)..][.._to_usize(first_half, 98 + i as usize) % 65]),
                4 => sv.truncate(_to_usize(first_half, 114 + i as usize)),
                5 => println!("{:?}", sv.as_slice()),
                6 => if let Some(elem) = sv.as_mut_slice().first_mut() { *elem = _to_u8(first_half, 130 + i as usize); },
                7 => sv.extend(second_half.iter().take(_to_usize(first_half, 146 + i as usize)).cloned()),
                8 => {
                    let other = SmallVec::from_slice(&second_half[_to_usize(first_half, 162 + i as usize) % 65..]);
                    let _ = sv.partial_cmp(&other);
                },
                9 => sv.dedup(),
                _ => unreachable!(),
            }
        }
        
        let ext_start = _to_usize(first_half, 200);
        let ext_len = _to_usize(first_half, 208) % 65;
        let ext_data = second_half.get(ext_start..).and_then(|d| d.get(..ext_len)).unwrap_or(&[]);
        sv.extend(ext_data.iter().cloned());
        let _ = sv.cmp(&SmallVec::from_slice(ext_data));
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