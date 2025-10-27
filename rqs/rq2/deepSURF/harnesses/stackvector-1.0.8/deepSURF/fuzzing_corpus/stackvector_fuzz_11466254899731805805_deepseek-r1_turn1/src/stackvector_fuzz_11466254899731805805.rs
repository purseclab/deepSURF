#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use stackvector::*;
use global_data::*;
use std::ops::{Deref, DerefMut, Index, IndexMut};

#[derive(Debug, Clone, Copy)]
struct CustomType1(usize);

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 4096 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let g = global_data.first_half;

        let constructor_selector = _to_u8(g, 0) % 4;
        let mut vec = match constructor_selector {
            0 => StackVec::<[CustomType1; 32]>::new(),
            1 => {
                let len = _to_usize(g, 1);
                StackVec::from_buf_and_len([CustomType1(0); 32], len)
            }
            2 => {
                let elem = CustomType1(_to_usize(g, 2));
                let n = _to_usize(g, 10);
                StackVec::from_elem(elem, n)
            }
            3 => {
                let src_len = _to_usize(g, 20) % 65;
                let mut src = Vec::with_capacity(src_len);
                for i in 0..src_len {
                    src.push(CustomType1(_to_usize(g, 30 + i*2)));
                }
                StackVec::from_slice(&src)
            }
            _ => unreachable!(),
        };

        let ops_count = _to_usize(g, 100) % 20;
        for i in 0..ops_count {
            let op_selector = _to_usize(g, 200 + i*4) % 9;
            match op_selector {
                0 => vec.push(CustomType1(_to_usize(g, 300 + i*4))),
                1 => { vec.pop(); }
                2 => vec.extend_from_slice(&[CustomType1(_to_usize(g, 400 + i*4))]),
                3 => vec.truncate(_to_usize(g, 500 + i*4)),
                4 => vec.insert(_to_usize(g, 600 + i*4), CustomType1(_to_usize(g, 604 + i*4))),
                5 => { let _ = vec.swap_remove(_to_usize(g, 700 + i*4)); }
                6 => {
                    let mut drain = vec.drain();
                    while let Some(item) = drain.next() {
                        println!("Drained: {:?}", item);
                    }
                }
                7 => {
                    if !vec.is_empty() {
                        let idx = _to_usize(g, 800 + i*4) % vec.len();
                        println!("Index {}: {:?}", idx, vec[idx]);
                    }
                }
                8 => {
                    let slice = vec.as_slice();
                    println!("Slice len: {}", slice.len());
                }
                _ => unreachable!(),
            };
        }

        let slice_len = _to_usize(g, 1000) % 65;
        let mut extend_slice = Vec::with_capacity(slice_len);
        for i in 0..slice_len {
            extend_slice.push(CustomType1(_to_usize(g, 1001 + i*2)));
        }
        vec.extend_from_slice(&extend_slice);

        let post_ops = _to_usize(g, 2000) % 5;
        for i in 0..post_ops {
            match _to_usize(g, 2100 + i*4) % 3 {
                0 => vec.push(CustomType1(_to_usize(g, 2200 + i*4))),
                1 => { vec.truncate(_to_usize(g, 2300 + i*4)); }
                2 => println!("Final capacity: {}", vec.capacity()),
                _ => unreachable!(),
            }
        }

        if !vec.is_empty() {
            println!("First element: {:?}", vec[0]);
            println!("Last element: {:?}", vec[vec.len()-1]);
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