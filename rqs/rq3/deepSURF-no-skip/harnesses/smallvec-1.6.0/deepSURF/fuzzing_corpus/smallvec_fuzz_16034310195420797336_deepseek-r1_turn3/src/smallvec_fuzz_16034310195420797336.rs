#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::ops::{Deref, DerefMut};

struct CustomType1(String);

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 1024 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let op_count = _to_u8(GLOBAL_DATA, 0) % 16;
        let mut offset = 1;

        for _ in 0..op_count {
            let op_selector = _to_u8(GLOBAL_DATA, offset) % 7;
            offset += 1;

            match op_selector {
                0 => {
                    let capacity = _to_usize(GLOBAL_DATA, offset);
                    let mut sv = SmallVec::<[u8; 128]>::with_capacity(capacity);
                    let elements = _to_u8(GLOBAL_DATA, offset + 8) % 65;
                    for i in 0..elements {
                        sv.push(_to_u8(GLOBAL_DATA, offset + 9 + i as usize));
                    }
                    let ptr = sv.as_mut_ptr();
                    let _ = _to_u8(GLOBAL_DATA, ptr as usize % GLOBAL_DATA.len());
                },
                1 => {
                    let len = _to_usize(GLOBAL_DATA, offset) % 65;
                    let mut buf = [0u8; 128];
                    for i in 0..len {
                        buf[i] = _to_u8(GLOBAL_DATA, offset + 8 + i);
                    }
                    let sv = SmallVec::from_buf(buf);
                    let mut sv_ref = &sv;
                    let _len = sv_ref.len();
                },
                2 => {
                    let slice_data = _to_str(GLOBAL_DATA, offset, offset + 64);
                    let mut sv = SmallVec::<[u8; 128]>::from_slice(slice_data.as_bytes());
                    sv.insert(0, _to_u8(GLOBAL_DATA, offset));
                    sv.remove(sv.len() - 1);
                },
                3 => {
                    let mut sv = SmallVec::<[u8; 128]>::new();
                    let count = _to_u8(GLOBAL_DATA, offset) % 64;
                    for _ in 0..count {
                        sv.push(_to_u8(GLOBAL_DATA, offset));
                        offset += 1;
                    }
                    let _ = sv.pop();
                    sv.shrink_to_fit();
                },
                4 => {
                    let mut sv1 = SmallVec::<[u8; 128]>::new();
                    let mut sv2 = SmallVec::<[u8; 128]>::with_capacity(32);
                    sv1.extend_from_slice(&GLOBAL_DATA[offset..offset+32]);
                    sv2.append(&mut sv1);
                    sv2.clear();
                },
                5 => {
                    let capacity = _to_usize(GLOBAL_DATA, offset);
                    let mut sv = SmallVec::<[u8; 128]>::with_capacity(capacity);
                    let new_len = _to_usize(GLOBAL_DATA, offset + 8) % 65;
                    for i in 0..new_len {
                        let data_index = (offset + 8 + i) % GLOBAL_DATA.len();
                        sv.push(_to_u8(GLOBAL_DATA, data_index));
                    }
                    let slice = sv.as_mut_slice();
                    let _ = &mut slice[..];
                },
                6 => {
                    let mut sv = SmallVec::<[u8; 128]>::new();
                    let index = _to_usize(GLOBAL_DATA, offset) % 64;
                    let value = _to_u8(GLOBAL_DATA, offset + 8);
                    sv.insert(index, value);
                    println!("{:?}", sv[index]);
                },
                _ => (),
            }
            offset = offset.saturating_add(64);
        }

        let t_0 = _to_u8(GLOBAL_DATA, offset);
        if t_0 % 2 == 0 {
            panic!("INTENTIONAL PANIC!");
        }
        let t_6 = _to_usize(GLOBAL_DATA, offset + 1);
        let buf = [_to_u8(GLOBAL_DATA, offset + 9); 128];
        let mut t_7 = SmallVec::from_buf_and_len(buf, t_6);
        let _ptr = t_7.as_mut_ptr();
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