#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;

fn _custom_fn0() -> String {
    let global_data = get_global_data();
    let GLOBAL_DATA = global_data.first_half;
    let t_10 = _to_u8(GLOBAL_DATA, 42);
    if t_10 % 2 == 0 {
        panic!("INTENTIONAL PANIC!");
    }
    let mut t_11 = _to_u8(GLOBAL_DATA, 43) % 17;
    let t_12 = _to_str(GLOBAL_DATA, 44, 44 + t_11 as usize);
    String::from(t_12)
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 256 {
            return;
        }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let mut sv = match _to_u8(GLOBAL_DATA, 0) % 3 {
            0 => SmallVec::<[String; 64]>::with_capacity(_to_usize(GLOBAL_DATA, 1)),
            1 => SmallVec::from_elem(String::new(), _to_usize(GLOBAL_DATA, 1) % 65),
            _ => SmallVec::from(vec![_to_str(GLOBAL_DATA, 1, 65).to_string()])
        };

        let operations = _to_u8(GLOBAL_DATA, 66) % 64;
        let mut offset = 67;

        for _ in 0..operations {
            if offset + 1 > GLOBAL_DATA.len() {
                break;
            }
            match _to_u8(GLOBAL_DATA, offset) % 7 {
                0 => {
                    let len = _to_usize(GLOBAL_DATA, offset + 1);
                    offset += 9;
                    sv.resize_with(len, || _custom_fn0());
                    println!("{:?}", sv.as_slice());
                }
                1 => {
                    let elem = _to_str(GLOBAL_DATA, offset + 1, offset + 65).to_string();
                    offset += 65;
                    sv.push(elem);
                }
                2 => {
                    sv.pop();
                }
                3 => {
                    let idx = _to_usize(GLOBAL_DATA, offset + 1);
                    offset += 9;
                    let elem = _to_str(GLOBAL_DATA, offset, offset + 32).to_string();
                    offset += 32;
                    sv.insert(idx, elem);
                }
                4 => {
                    sv.truncate(_to_usize(GLOBAL_DATA, offset + 1));
                    offset += 9;
                }
                5 => {
                    let idx = _to_usize(GLOBAL_DATA, offset + 1);
                    offset += 9;
                    if !sv.is_empty() {
                        let _ = sv.remove(idx % sv.len());
                    }
                }
                6 => {
                    sv.push(_to_str(GLOBAL_DATA, offset + 1, offset + 17).to_string());
                    offset += 17;
                }
                _ => unreachable!(),
            }
            offset += 1;
        }

        let new_len = _to_usize(GLOBAL_DATA, offset);
        sv.resize_with(new_len, || _custom_fn0());
        let _ = sv.capacity();
        let _ = sv.as_ptr();
        let _ = sv.as_mut_slice().len();
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