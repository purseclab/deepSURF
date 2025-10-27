#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;    
use smallvec::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};
fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 105 {
            set_global_data(data);
            return;
        }
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let t_0 = _to_u8(GLOBAL_DATA, 0);
        let t_1 = _to_u16(GLOBAL_DATA, 1);
        let t_2 = _to_u32(GLOBAL_DATA, 3);
        let t_3 = _to_u64(GLOBAL_DATA, 7);
        let t_4 = _to_u128(GLOBAL_DATA, 15);
        let t_5 = _to_usize(GLOBAL_DATA, 31);
        let t_6 = _to_i8(GLOBAL_DATA, 39);
        let t_7 = _to_i16(GLOBAL_DATA, 40);
        let t_8 = _to_i32(GLOBAL_DATA, 42);
        let t_9 = _to_i64(GLOBAL_DATA, 46);
        let t_10 = _to_i128(GLOBAL_DATA, 54);
        let t_11 = _to_isize(GLOBAL_DATA, 70);
        let t_12 = _to_f32(GLOBAL_DATA, 78);
        let t_13 = _to_f64(GLOBAL_DATA, 82);
        let t_14 = _to_char(GLOBAL_DATA, 90);
        let t_15 = _to_bool(GLOBAL_DATA, 94);
        let t_16 = _to_str(GLOBAL_DATA, 95, 105);
        let t_17 = String::from(t_16);

        let mut sv = if t_15 {
            SmallVec::<[u32; 16]>::new()
        } else {
            SmallVec::from_buf([t_2; 16])
        };

        for _ in 0..(t_5 % 65) {
            sv.push(t_2);
        }

        let drained: Vec<_> = sv.drain(t_5 % sv.len()..).collect();
        let mut heap_vec = drained.into_iter().cycle().take(t_5 % 65).collect();
        let mut sv_from_vec = SmallVec::<[u32; 16]>::from_vec(heap_vec);

        sv_from_vec.insert(t_5 % (sv_from_vec.len() + 1), t_2);
        sv_from_vec.swap_remove(t_5 % sv_from_vec.len());

        let buf = [t_2; 32];
        let target_sv = SmallVec::<[u32; 32]>::from_buf_and_len(buf, t_5 % 33);
        println!("{:?}", target_sv.as_slice());

        let mut combined = SmallVec::<[u32; 64]>::new();
        combined.append(&mut sv);
        combined.append(&mut sv_from_vec);

        combined.truncate(t_5 % (combined.len() + 1));
        combined.retain(|x| *x == t_2);

        let popped = combined.pop();
        println!("{:?}", popped.as_ref());

        let _ = combined.into_boxed_slice();
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