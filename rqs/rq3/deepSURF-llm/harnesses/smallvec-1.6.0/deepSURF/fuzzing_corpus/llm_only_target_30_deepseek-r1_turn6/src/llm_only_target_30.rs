#![forbid(unsafe_code)]

#[macro_use]
extern crate afl;    

use smallvec::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 190 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let t_0 = _to_u8(GLOBAL_DATA, 0);
        let t_5 = _to_usize(GLOBAL_DATA, 31);
        let t_6 = _to_i8(GLOBAL_DATA, 39);
        let t_7 = _to_i16(GLOBAL_DATA, 40);
        let t_8 = _to_i32(GLOBAL_DATA, 42);
        let t_9 = _to_i64(GLOBAL_DATA, 46);
        let t_15 = _to_bool(GLOBAL_DATA, 94);

        let mut sv = SmallVec::<[u8; 32]>::from_vec(vec![t_0; t_5 % 65]);
        let elem_count = t_5 % 65;
        for i in 0..elem_count {
            sv.push((t_0.wrapping_add(i as u8)) as u8);
        }

        let mut sv2 = SmallVec::<[u8; 16]>::with_capacity(t_5 % 128);
        sv2.extend_from_slice(&sv);

        if !sv.is_empty() {
            sv.insert((t_6 as usize) % sv.len(), t_0);
            sv.swap_remove(t_7 as usize % sv.len());
        }

        sv.dedup_by_key(|x| {
            if t_15 {
                panic!("dedup panic triggered");
            }
            x.wrapping_div(t_0.max(1))
        });

        let drain_range = (t_8 as usize) % (sv.len() + 1);
        let _: Vec<_> = sv.drain(0..drain_range).collect();

        if t_15 {
            sv2.truncate(t_5 % (sv2.len() + 1));
        }

        if !sv2.is_empty() {
            println!("{:?}", sv2.as_ptr());
            sv2.remove(t_9 as usize % sv2.len());
            sv2.shrink_to_fit();
        }

        let mut sv3 = SmallVec::from_buf([t_0; 32]);
        sv3.extend(sv2.drain(..));
        sv3.append(&mut SmallVec::<[u8; 8]>::from_slice(&[t_0; 8]));

        sv3.retain(|x| {
            if _to_bool(GLOBAL_DATA, 93) {
                panic!("retain panic");
            }
            *x != 0
        });

        let _ = sv3.as_mut_slice();
        println!("{:?}", sv3.index(t_5 % sv3.len()));
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