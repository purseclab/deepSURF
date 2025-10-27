#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;

type SmallVec8 = SmallVec<[u8; 8]>;

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 130 {
            return;
        }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let len_vec = (_to_u8(GLOBAL_DATA, 0) % 65) as usize;
        let mut vec_data = Vec::with_capacity(len_vec);
        for i in 0..len_vec {
            vec_data.push(_to_u8(GLOBAL_DATA, 1 + i));
        }

        let ctor_choice = _to_u8(GLOBAL_DATA, 100);
        let mut small = match ctor_choice % 4 {
            0 => SmallVec8::from_vec(vec_data.clone()),
            1 => SmallVec8::from_slice(&vec_data),
            2 => SmallVec8::from_iter(vec_data.clone()),
            _ => SmallVec8::from_elem(_to_u8(GLOBAL_DATA, 2), len_vec),
        };

        let mut other = SmallVec8::with_capacity(len_vec);
        other.extend_from_slice(&vec_data);
        small.append(&mut other);

        let ops = _to_u8(GLOBAL_DATA, 101) % 20;
        for idx in 0..ops {
            let b = _to_u8(GLOBAL_DATA, 102 + idx as usize);
            match b % 10 {
                0 => small.push(b),
                1 => {
                    small.pop();
                }
                2 => {
                    let pos = (b as usize) % (small.len() + 1);
                    small.insert(pos, b);
                }
                3 => {
                    if !small.is_empty() {
                        let pos = (b as usize) % small.len();
                        small.remove(pos);
                    }
                }
                4 => small.truncate((b as usize) % 65),
                5 => small.reserve(b as usize),
                6 => small.shrink_to_fit(),
                7 => {
                    let cap = small.capacity();
                    println!("{}", cap);
                }
                8 => {
                    let slice_ref = small.as_slice();
                    println!("{:?}", slice_ref);
                }
                _ => small.clear(),
            }
        }

        if let Some(first) = small.as_mut_slice().get_mut(0) {
            *first = first.wrapping_add(1);
        }

        let mut compare_small = SmallVec8::from_iter(vec_data);
        let _ = small.cmp(&compare_small);
        let _ = small.partial_cmp(&compare_small);

        let vec_again = small.into_vec();
        let _ = SmallVec8::from_vec(vec_again);
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