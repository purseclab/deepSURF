#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::ops::{Deref, DerefMut, Index, IndexMut};

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 90 {
            return;
        }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let constructor_selector = _to_u8(GLOBAL_DATA, 0);
        let mut base_array = [0u8; 16];
        for i in 0..16 {
            base_array[i] = _to_u8(GLOBAL_DATA, 1 + i);
        }
        let capacity_val = _to_usize(GLOBAL_DATA, 20);
        let vec_len = (_to_u8(GLOBAL_DATA, 28) % 65) as usize;
        let mut vec_from_data = Vec::with_capacity(vec_len);
        for i in 0..vec_len {
            vec_from_data.push(_to_u8(GLOBAL_DATA, 29 + i));
        }

        let mut sv: SmallVec<[u8; 16]> = match constructor_selector % 5 {
            0 => SmallVec::new(),
            1 => SmallVec::with_capacity(capacity_val),
            2 => SmallVec::from_buf(base_array),
            3 => SmallVec::from_vec(vec_from_data),
            _ => SmallVec::from_buf_and_len(base_array, (capacity_val % 16) + 1),
        };

        let operations_to_do = (_to_u8(GLOBAL_DATA, 60) % 16) as usize;
        for (idx, byte) in global_data.second_half.iter().take(operations_to_do).enumerate() {
            match byte % 10 {
                0 => sv.push(_to_u8(GLOBAL_DATA, (61 + idx) % GLOBAL_DATA.len())),
                1 => {
                    let _ = sv.pop();
                }
                2 => {
                    let index_val = _to_usize(GLOBAL_DATA, (62 + idx) % (GLOBAL_DATA.len() - 8));
                    sv.insert(index_val % (sv.len() + 1), *byte);
                }
                3 => {
                    if !sv.is_empty() {
                        sv.remove((*byte as usize) % sv.len());
                    }
                }
                4 => {
                    if !sv.is_empty() {
                        sv.swap_remove((*byte as usize) % sv.len());
                    }
                }
                5 => {
                    sv.truncate((*byte as usize) % 65);
                }
                6 => {
                    let slice_start = (*byte as usize) % GLOBAL_DATA.len();
                    let slice_end = slice_start + 4;
                    if slice_end <= GLOBAL_DATA.len() {
                        let slice = &GLOBAL_DATA[slice_start..slice_end];
                        sv.extend_from_slice(slice);
                    }
                }
                7 => {
                    sv.retain(|v| *v % 2 == 0);
                }
                8 => {
                    sv.dedup();
                }
                _ => {
                    let _ = sv.try_reserve_exact(*byte as usize);
                }
            }
        }

        println!("{:?}", sv.as_slice().deref());

        let boxed = sv.into_boxed_slice();
        println!("{:?}", &boxed[..]);

        let vec_again = boxed.into_vec();
        let mut sv_again = SmallVec::<[u8; 16]>::from_vec(vec_again);
        if sv_again.len() > 1 {
            let first_ref = &sv_again[0];
            println!("{}", *first_ref);
        }

        sv_again.clear();
        let _ = sv_again.into_boxed_slice();
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