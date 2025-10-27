#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

#[derive(Clone, Debug, PartialEq)]
struct CustomType1(String);

fn _custom_fn0() -> CustomType1 {
    let global_data = get_global_data();
    let GLOBAL_DATA = global_data.first_half;
    let t_10 = _to_u8(GLOBAL_DATA, 42);
    if t_10 % 2 == 0 {
        panic!("INTENTIONAL PANIC!");
    }
    let t_11 = _to_u8(GLOBAL_DATA, 43) % 17;
    let t_12 = _to_str(GLOBAL_DATA, 44, 44 + t_11 as usize);
    let t_13 = String::from(t_12);
    CustomType1(t_13)
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 300 {
            return;
        }
        set_global_data(data);
        let global_data = get_global_data();
        let GD0 = global_data.first_half;
        let GD1 = global_data.second_half;

        let ctor_choice = _to_u8(GD0, 0) % 4;
        let mut sv_u8: SmallVec<[u8; 32]> = match ctor_choice {
            0 => SmallVec::new(),
            1 => {
                let cap = _to_usize(GD0, 1);
                SmallVec::with_capacity(cap)
            }
            2 => {
                let len_raw = (_to_u8(GD0, 9) % 20) as usize;
                let vec_data: Vec<u8> = GD0[10..10 + len_raw].to_vec();
                SmallVec::from_vec(vec_data)
            }
            _ => {
                let len_raw = (_to_u8(GD0, 40) % 32) as usize;
                let mut buf = [0u8; 32];
                for i in 0..len_raw {
                    buf[i] = _to_u8(GD0, 41 + i);
                }
                SmallVec::from_buf_and_len(buf, len_raw)
            }
        };

        let base_elem = _custom_fn0();
        let copies = (_to_u8(GD0, 90) % 17) as usize;
        let mut sv_custom: SmallVec<[CustomType1; 16]> = SmallVec::from_elem(base_elem.clone(), copies);

        sv_u8.push(_to_u8(GD0, 110));
        let ext_len = (_to_u8(GD0, 111) % 5) as usize;
        sv_u8.extend_from_slice(&GD0[112..112 + ext_len]);

        let new_len_custom = _to_usize(GD0, 118);
        sv_custom.resize_with(new_len_custom, _custom_fn0);

        let op_count = (_to_u8(GD1, 0) % 10) as usize;
        for i in 0..op_count {
            let op_code = _to_u8(GD1, 1 + i) % 6;
            match op_code {
                0 => {
                    let idx = _to_usize(GD1, 20 + i);
                    sv_custom.insert(idx, _custom_fn0());
                }
                1 => {
                    if !sv_custom.is_empty() {
                        let idx = _to_usize(GD1, 40 + i) % sv_custom.len();
                        sv_custom.remove(idx);
                    }
                }
                2 => sv_custom.dedup(),
                3 => {
                    let nl = _to_usize(GD1, 60 + i);
                    sv_custom.resize_with(nl, _custom_fn0);
                }
                4 => sv_custom.retain(|_| _to_bool(GD1, 80 + i)),
                _ => sv_custom.clear(),
            }
        }

        let mut inc: u8 = 0;
        let mut gen = || {
            inc = inc.wrapping_add(1);
            inc
        };
        let new_len_u8 = _to_usize(GD1, 100);
        sv_u8.resize_with(new_len_u8, &mut gen);

        if !sv_custom.is_empty() {
            println!("{:?}", sv_custom[0]);
        }
        println!("{}", sv_u8.capacity());
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