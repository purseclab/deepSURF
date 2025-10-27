#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::SmallVec;
use global_data::*;

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 1024 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let num_ops = _to_u8(GLOBAL_DATA, 0) % 65;
        let mut data_idx = 1;
        let mut sv = SmallVec::<[String; 16]>::new();

        for _ in 0..num_ops {
            if data_idx + 1 > GLOBAL_DATA.len() { break; }
            let op_byte = _to_u8(GLOBAL_DATA, data_idx);
            data_idx += 1;

            match op_byte % 12 {
                0 => sv = SmallVec::new(),
                1 => {
                    let capacity = _to_usize(GLOBAL_DATA, data_idx);
                    data_idx += 8;
                    sv = SmallVec::with_capacity(capacity);
                }
                2 => {
                    let elem_count = _to_u8(GLOBAL_DATA, data_idx) % 65;
                    data_idx += 1;
                    let mut elems = Vec::new();
                    for _ in 0..elem_count {
                        if data_idx + 10 > GLOBAL_DATA.len() { break; }
                        elems.push(String::from(_to_str(GLOBAL_DATA, data_idx, data_idx + 10)));
                        data_idx += 10;
                    }
                    sv = SmallVec::from_vec(elems);
                }
                3 => {
                    let elem_count = _to_u8(GLOBAL_DATA, data_idx) % 65;
                    data_idx += 1;
                    let mut elems = vec![String::new(); elem_count as usize];
                    for e in &mut elems {
                        if data_idx + 10 > GLOBAL_DATA.len() { break; }
                        *e = String::from(_to_str(GLOBAL_DATA, data_idx, data_idx + 10));
                        data_idx += 10;
                    }
                    sv = SmallVec::from_vec(elems);
                }
                4 => {
                    if data_idx + 10 > GLOBAL_DATA.len() { continue; }
                    let s = String::from(_to_str(GLOBAL_DATA, data_idx, data_idx + 10));
                    data_idx += 10;
                    sv.push(s);
                }
                5 => {
                    let idx = _to_usize(GLOBAL_DATA, data_idx);
                    data_idx += 8;
                    if data_idx + 10 > GLOBAL_DATA.len() { continue; }
                    let s = String::from(_to_str(GLOBAL_DATA, data_idx, data_idx + 10));
                    data_idx += 10;
                    sv.insert(idx, s);
                }
                6 => {
                    let idx = _to_usize(GLOBAL_DATA, data_idx);
                    data_idx += 8;
                    sv.remove(idx);
                }
                7 => {
                    let len = _to_usize(GLOBAL_DATA, data_idx);
                    data_idx += 8;
                    sv.truncate(len);
                }
                8 => {
                    let start = _to_usize(GLOBAL_DATA, data_idx);
                    data_idx += 8;
                    let end = _to_usize(GLOBAL_DATA, data_idx);
                    data_idx += 8;
                    sv.drain(start..end);
                }
                9 => {
                    let _ = sv.pop();
                }
                10 => {
                    let new_len = _to_usize(GLOBAL_DATA, data_idx);
                    data_idx += 8;
                    sv.resize_with(new_len, || String::new());
                }
                11 => {
                    let slice = sv.as_slice();
                    println!("{:?}", slice);
                }
                _ => {}
            }
        }

        let remove_idx = _to_usize(GLOBAL_DATA, data_idx);
        sv.remove(remove_idx);
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