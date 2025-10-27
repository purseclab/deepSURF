#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use simple_slab::*;
use global_data::*;
use std::ops::{Deref, DerefMut, Index};

#[derive(Debug)]
struct CustomType0(String);

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 100 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let constructor_choice = _to_u8(GLOBAL_DATA, 0) % 3;
        let mut slab = match constructor_choice {
            0 => Slab::new(),
            1 => Slab::with_capacity(_to_usize(GLOBAL_DATA, 1)),
            _ => {
                let capacity = _to_usize(GLOBAL_DATA, 2);
                let mut temp = Slab::new();
                for _ in 0..capacity {
                    temp.insert(CustomType0(String::new()));
                }
                temp
            }
        };

        let mut base = match constructor_choice {
            0 => 1,
            1 => 9,
            _ => 16
        };
        let num_ops = _to_u8(GLOBAL_DATA, base) % 65;
        base += 1;

        for _ in 0..num_ops {
            if base + 3 > GLOBAL_DATA.len() { break; }
            let op = _to_u8(GLOBAL_DATA, base) % 9;
            base += 1;

            match op {
                0 => {
                    let len_offset = _to_u8(GLOBAL_DATA, base) % 17;
                    base += 1;
                    let end = base + len_offset as usize;
                    if end > GLOBAL_DATA.len() { break; }
                    let s = String::from(_to_str(GLOBAL_DATA, base, end));
                    let custom = CustomType0(s);
                    slab.insert(custom);
                    let _ = slab.len();
                    base = end;
                }
                1 => {
                    let idx = _to_usize(GLOBAL_DATA, base);
                    base += 8;
                    println!("{:?}", slab.index(idx));
                    let _ = slab.remove(idx);
                }
                2 => {
                    let mut iter = slab.iter();
                    while let Some(elem) = iter.next() {
                        println!("{:?}", elem.deref());
                    }
                }
                3 => {
                    let mut iter = slab.iter_mut();
                    while let Some(mut elem) = iter.next() {
                        println!("{:?}", elem.deref_mut());
                    }
                }
                4 => {
                    for mut elem in slab.iter_mut() {
                        *elem.deref_mut() = CustomType0(String::from("modified"));
                    }
                }
                5 => {
                    let new_cap = _to_usize(GLOBAL_DATA, base);
                    base += 8;
                    let mut new_slab = Slab::with_capacity(new_cap);
                    std::mem::swap(&mut slab, &mut new_slab);
                }
                6 => {
                    let idx = _to_usize(GLOBAL_DATA, base);
                    base += 8;
                    println!("Index {}: {:?}", idx, slab.index(idx));
                }
                7 => {
                    let idx = _to_usize(GLOBAL_DATA, base);
                    base += 8;
                    for (i, elem) in slab.iter_mut().enumerate() {
                        if i == idx {
                            *elem = CustomType0(String::from("altered"));
                            break;
                        }
                    }
                }
                _ => {
                    let capacity = slab.len();
                    let mut temp = Slab::with_capacity(capacity * 2);
                    for elem in slab.iter() {
                        temp.insert(CustomType0(elem.0.clone()));
                    }
                    slab = temp;
                }
            }
        }

        let _drain = slab.into_iter();
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