#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use stackvector::*;
use global_data::*;
use std::ops::Deref;

fn main (){
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 1024 {return;}
        set_global_data(data);
        let global_data = get_global_data();
        let g_data = global_data.first_half;

        let operations = _to_u8(g_data, 0) % 8;

        for i in 0..operations {
            let selector = _to_usize(g_data, i as usize * 4) % 5;
            match selector {
                0 => {
                    let mut vec = StackVec::<[u8; 256]>::new();
                    let pushes = _to_usize(g_data, i as usize * 4 + 1) % 65;
                    for j in 0..pushes {
                        vec.push(_to_u8(g_data, i as usize *4 +2 + j));
                    }
                    let _ = vec.deref();
                    let _ = vec.pop();
                }
                1 => {
                    let start = _to_usize(g_data, i as usize *4 +1);
                    let len = _to_usize(g_data, i as usize *4 +2) % 65;
                    let slice = &g_data[start..start+len];
                    let vec = StackVec::<[u8; 256]>::from_slice(slice);
                    let idx = _to_usize(g_data, i as usize *4 +3);
                    let _ = vec.get(idx);
                    println!("{:?}", vec.deref());
                }
                2 => {
                    let elem = _to_u8(g_data, i as usize *4 +1);
                    let count = _to_usize(g_data, i as usize *4 +2) % 65;
                    let mut vec = StackVec::<[u8; 256]>::from_elem(elem, count);
                    let _ = vec.as_slice();
                    let truncate = _to_usize(g_data, i as usize *4 +3);
                    vec.deref();
                    vec.truncate(truncate);
                }
                3 => {
                    let len = _to_usize(g_data, i as usize *4 +1) % 65;
                    let start = _to_usize(g_data, i as usize *4 +2);
                    let iter_data = &g_data[start..start+len];
                    let vec = StackVec::<[u8; 256]>::from_iter(iter_data.iter().cloned());
                    let remove_idx = _to_usize(g_data, i as usize *4 +3);
                    let _ = vec.get(remove_idx);
                    vec.deref();
                }
                4 => {
                    let mut vec1 = StackVec::<[u8; 256]>::new();
                    let elements = _to_usize(g_data, i as usize *4 +1) % 65;
                    for j in 0..elements {
                        vec1.push(_to_u8(g_data, i as usize *4 +2 + j));
                    }
                    let vec2 = StackVec::<[u8; 256]>::from_slice(vec1.deref());
                    let _ = vec1.eq(&vec2);
                    vec1.deref();
                }
                _ => {}
            }
        }

        let mut final_vec = StackVec::<[u8; 256]>::new();
        let count = _to_usize(g_data, 0) % 65;
        for i in 0..count {
            final_vec.push(_to_u8(g_data, i));
        }
        final_vec.deref();
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