#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;

fn _custom_fn0(x: &mut String) -> bool {
    let global_data = get_global_data();
    let GLOBAL_DATA = global_data.first_half;
    let t_9 = _to_u8(GLOBAL_DATA, 49);
    if t_9 % 2 == 0 {
        panic!("INTENTIONAL PANIC!");
    }
    let t_10 = _to_bool(GLOBAL_DATA, 50);
    t_10
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 512 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let (first_half, second_half) = (global_data.first_half, global_data.second_half);

        let mut sv = match _to_u8(first_half, 0) % 5 {
            0 => SmallVec::<[String; 16]>::new(),
            1 => {
                let cap = _to_usize(first_half, 1) % 65;
                SmallVec::with_capacity(cap)
            },
            2 => {
                let elem_len = _to_usize(first_half, 1) % 65;
                let elem = _to_str(second_half, 0, 16).to_string();
                SmallVec::from_elem(elem, elem_len)
            },
            3 => {
                let vec_len = _to_usize(second_half, 0) % 65;
                let mut vec = Vec::with_capacity(vec_len);
                for i in 0..vec_len {
                    vec.push(_to_str(second_half, i*8, i*8+8).to_string());
                }
                SmallVec::from_vec(vec)
            },
            4 => {
                let slice_len = _to_usize(second_half, 0) % 65;
                let items: Vec<_> = (0..slice_len).map(|i| 
                    _to_str(second_half, i*4, i*4+4).to_string()
                ).collect();
                SmallVec::from_vec(items)
            },
            _ => unreachable!()
        };

        let op_count = _to_usize(first_half, 8) % 8;
        for op_idx in 0..op_count {
            match _to_u8(second_half, op_idx) % 7 {
                0 => sv.push(_to_str(first_half, op_idx*3, op_idx*3+3).to_string()),
                1 => sv.insert(_to_usize(second_half, op_idx*8) % sv.len().saturating_add(1), 
                              _to_str(first_half, op_idx*4, op_idx*4+4).to_string()),
                2 => {
                    let _ = sv.pop();
                }
                3 => sv.truncate(_to_usize(second_half, op_idx*8)),
                4 => {
                    let _ = sv.as_slice();
                    println!("{:?}", sv);
                }
                5 => sv.retain(|x| _custom_fn0(x)),
                6 => {
                    let new_cap = _to_usize(second_half, op_idx*8);
                    sv.grow(new_cap);
                },
                _ => {}
            }
        }

        let _drained: Vec<_> = sv.drain().collect();

        sv.shrink_to_fit();
        let _ = sv.into_iter().next();
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