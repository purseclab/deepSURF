#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::SmallVec;
use global_data::*;
use std::ops::{Deref, DerefMut};

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 1024 { return; }
        set_global_data(data);
        let global_data = get_global_data().first_half;

        let op_count = _to_usize(global_data, 0) % 32;
        let mut vec_storage = Vec::new();
        let mut smallvecs = Vec::new();

        for i in 0..op_count {
            let switch = _to_u8(global_data, i);
            match switch % 7 {
                0 => {
                    let cap = _to_usize(global_data, i * 4);
                    smallvecs.push(SmallVec::<[u8; 32]>::with_capacity(cap));
                }
                1 => {
                    let slice_start = _to_usize(global_data, i * 8);
                    let slice_end = _to_usize(global_data, i * 8 + 4);
                    let slice = &global_data[slice_start..slice_end];
                    vec_storage.extend_from_slice(slice);
                    smallvecs.push(SmallVec::from_slice(&vec_storage));
                }
                2 => {
                    let array = [_to_u8(global_data, i); 32];
                    let len = _to_usize(global_data, i * 8);
                    smallvecs.push(SmallVec::from_buf_and_len(array, len));
                }
                3 => {
                    let elem = _to_u8(global_data, i);
                    let count = _to_usize(global_data, i * 2);
                    let sv = SmallVec::<[u8; 32]>::from_elem(elem, count);
                    println!("{:?}", sv.deref());
                    smallvecs.push(sv);
                }
                4 => {
                    if let Some(mut sv) = smallvecs.pop() {
                        let index = _to_usize(global_data, i * 3);
                        sv.insert(index, _to_u8(global_data, i));
                        sv.retain(|x| *x % 2 == 0);
                        smallvecs.push(sv);
                    }
                }
                5 => {
                    if let Some(sv) = smallvecs.last_mut() {
                        sv.extend_from_slice(&global_data[64..128]);
                        let _ = sv.as_mut_slice().get_mut(0).map(|v| *v += 1);
                    }
                }
                6 => {
                    smallvecs.iter_mut().for_each(|sv| {
                        sv.drain(..).for_each(|x| vec_storage.push(x));
                        sv.shrink_to_fit();
                    });
                }
                _ => (),
            }
        }

        smallvecs.iter().for_each(|sv| {
            let _ = sv.capacity();
            let _ = sv.as_ptr();
            sv.into_iter().for_each(|x| println!("{:?}", x));
        });

        if let Some(mut sv) = smallvecs.into_iter().next() {
            let drainer = sv.drain(.._to_usize(global_data, 256));
            drainer.enumerate().for_each(|(i, x)| {
                if i % 3 == 0 {
                    println!("Drained: {}", x);
                }
            });
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