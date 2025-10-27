#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use simple_slab::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

#[derive(Debug)]
struct CustomType0(String);

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 128 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let SH = global_data.second_half;

        let cap = _to_usize(GLOBAL_DATA, 0);
        let mut slab = if _to_bool(GLOBAL_DATA, 8) {
            simple_slab::Slab::<CustomType0>::new()
        } else {
            simple_slab::Slab::<CustomType0>::with_capacity(cap)
        };

        let s_all = _to_str(SH, 0, SH.len());
        let base = CustomType0(s_all.to_string());

        let ins_count = (_to_u8(GLOBAL_DATA, 9) as usize) % 20 + 1;
        for _ in 0..ins_count {
            slab.insert(CustomType0(base.0.clone()));
        }

        {
            let mut it = slab.iter();
            if let Some(r) = it.next() {
                println!("{:?}", &*r);
            }
        }

        let idx1 = _to_usize(GLOBAL_DATA, 16);
        if slab.len() > 0 {
            let r = &slab[idx1];
            println!("{:?}", &*r);
        }

        let off1 = _to_usize(GLOBAL_DATA, 24);
        if slab.len() > 0 {
            let removed = slab.remove(off1);
            println!("{:?}", removed);
        }

        {
            let mut mit = slab.iter_mut();
            if let Some(mr) = mit.next() {
                println!("{:?}", &*mr);
            }
        }

        {
            let mut iit = (&mut slab).into_iter();
            if let Some(mr2) = iit.next() {
                println!("{:?}", &*mr2);
            }
        }

        for op in 0..((_to_u8(GLOBAL_DATA, 10) as usize) % 25) {
            match (_to_u8(GLOBAL_DATA, 11 + (op % 10)) % 5) {
                0 => {
                    slab.insert(CustomType0(format!("{}-{}", base.0, op)));
                }
                1 => {
                    let i = _to_usize(GLOBAL_DATA, 32);
                    if slab.len() > 0 {
                        let r = &slab[i];
                        println!("{:?}", &*r);
                    }
                }
                2 => {
                    let mut it2 = slab.iter();
                    if let Some(r2) = it2.next() {
                        println!("{:?}", &*r2);
                    }
                }
                3 => {
                    let mut mit2 = slab.iter_mut();
                    if let Some(mr3) = mit2.next() {
                        println!("{:?}", &*mr3);
                    }
                }
                _ => {
                    let i2 = _to_usize(GLOBAL_DATA, 40);
                    if slab.len() > 0 {
                        let removed2 = slab.remove(i2);
                        println!("{:?}", removed2);
                    }
                }
            }
        }

        {
            let mut it3 = (&slab).into_iter();
            if let Some(r3) = it3.next() {
                println!("{:?}", &*r3);
            }
        }

        let off_final = _to_usize(GLOBAL_DATA, 48);
        if slab.len() > 0 {
            let removed3 = slab.remove(off_final);
            println!("{:?}", removed3);
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