#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::ops::{IndexMut, DerefMut};

fn main (){
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 512 {return;}
        set_global_data(data);
        let global_data = get_global_data();
        let g = &global_data.first_half;
        
        let constructor_sel = _to_u8(g, 0) % 4;
        let mut sv = match constructor_sel {
            0 => SmallVec::<[usize; 16]>::new(),
            1 => SmallVec::with_capacity(_to_usize(g, 1)),
            2 => {
                let count = _to_usize(g, 2) % 65;
                let mut vec = Vec::with_capacity(count);
                for i in 0..count {
                    vec.push(_to_usize(g, 4 + i*4));
                }
                SmallVec::from_vec(vec)
            },
            _ => {
                let slice: &[usize] = &[_to_usize(g, 100), _to_usize(g, 104)];
                SmallVec::from_slice(slice)
            }
        };

        let op_count = _to_usize(g, 200) % 16;
        for op_idx in 0..op_count {
            let byte = _to_u8(g, 201 + op_idx);
            match byte % 7 {
                0 => sv.push(_to_usize(g, 300 + op_idx*4)),
                1 => {sv.pop();},
                2 => sv.truncate(_to_usize(g, 400 + op_idx*4)),
                3 => {
                    let idx = _to_usize(g, 500 + op_idx*4) % (sv.len() + 1);
                    sv.insert(idx, _to_usize(g, 600 + op_idx*4));
                },
                4 => {
                    let cap = _to_usize(g, 700 + op_idx*4);
                    sv.reserve(cap);
                },
                5 => {
                    let slice = &[_to_usize(g, 800), _to_usize(g, 804)];
                    sv.extend_from_slice(slice);
                },
                _ => sv.shrink_to_fit(),
            };
        }

        let closure_sel = _to_u8(g, 900) % 3;
        sv.dedup_by_key(|x| match closure_sel {
            0 => *x,
            1 => *x % 2,
            _ => x.wrapping_mul(2),
        });

        println!("{:?}", sv.as_slice());
        let _ = sv.as_mut_slice().first_mut().map(|x| *x += 1);
    });
}

// ... (remaining type converter functions omitted as per directions) ...

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