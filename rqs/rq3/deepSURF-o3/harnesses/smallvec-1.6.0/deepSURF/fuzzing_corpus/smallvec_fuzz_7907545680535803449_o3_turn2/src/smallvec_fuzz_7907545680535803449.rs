#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::ops::{Deref, DerefMut, Index, IndexMut};

fn build_string(offset: usize) -> String {
    let gd = get_global_data().first_half;
    let len = _to_u8(gd, offset) % 17;
    let s = _to_str(gd, offset + 1, offset + 1 + len as usize);
    String::from(s)
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 1400 { return; }
        set_global_data(data);
        let gd = get_global_data().first_half;

        let mut raw_vec: Vec<String> = Vec::with_capacity(65);
        let string_count = (_to_u8(gd, 0) % 65) as usize;
        for i in 0..string_count {
            let offset = 1 + i * 20;
            let s = build_string(offset);
            raw_vec.push(s);
        }

        let ctor_sel = _to_u8(gd, 350);
        let mut sv: SmallVec<[String; 32]> = match ctor_sel % 5 {
            0 => SmallVec::new(),
            1 => {
                let cap = _to_usize(gd, 351) % 65;
                SmallVec::with_capacity(cap)
            }
            2 => SmallVec::from(&raw_vec[..]),
            3 => SmallVec::from(raw_vec.clone()),
            _ => SmallVec::from_iter(raw_vec.clone().into_iter()),
        };

        if ctor_sel % 2 == 0 {
            sv.push(build_string(380));
        }

        let reserve_amt = _to_usize(gd, 400);
        if ctor_sel % 3 == 0 {
            let _ = sv.try_reserve(reserve_amt);
        } else {
            sv.reserve(reserve_amt);
        }

        if !sv.is_empty() && _to_u8(gd, 410) % 2 == 0 {
            sv.pop();
        }

        if sv.len() < 65 {
            let idx = if sv.len() == 0 { 0 } else { _to_usize(gd, 420) % sv.len() };
            sv.insert(idx, build_string(430));
        }

        if sv.len() > 0 {
            let idx = _to_usize(gd, 450) % sv.len();
            sv.remove(idx);
        }

        let len = sv.len();
        let start = _to_usize(gd, 480) % (len + 1);
        let end = _to_usize(gd, 488) % (len + 1);
        let (lo, hi) = if start <= end { (start, end) } else { (end, start) };
        let range_sel = _to_u8(gd, 470);

        let mut drained = match range_sel % 5 {
            0 => sv.drain(lo..hi),
            1 => sv.drain(lo..=hi),
            2 => sv.drain(lo..),
            3 => sv.drain(..hi),
            _ => sv.drain(..),
        };

        let mut collected: SmallVec<[String; 32]> = SmallVec::new();
        drained.for_each(|item| collected.push(item));

        collected.dedup();
        collected.truncate((_to_u8(gd, 520) % 33) as usize);

        sv.shrink_to_fit();

        let slice_ref = sv.as_slice();
        if !slice_ref.is_empty() {
            let first = &slice_ref[0];
            println!("{:?}", first.deref());
        }

        let _ = sv.partial_cmp(&collected);
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