#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::ops::{IndexMut, DerefMut};

#[derive(Debug, Clone, Copy)]
struct CustomType1(usize);

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 1500 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let mut vec_pool: Vec<SmallVec<[CustomType1; 16]>> = Vec::new();
        let mut base = 0;

        for _ in 0..3 {
            let constructor_selector = _to_u8(GLOBAL_DATA, base) % 4;
            base += 1;

            let sv = match constructor_selector {
                0 => {
                    let elem = CustomType1(_to_usize(GLOBAL_DATA, base));
                    let count = _to_usize(GLOBAL_DATA, base + 8);
                    base += 16;
                    SmallVec::<[CustomType1; 16]>::from_elem(elem, count)
                },
                1 => {
                    let cap = _to_usize(GLOBAL_DATA, base);
                    base += 8;
                    SmallVec::<[CustomType1; 16]>::with_capacity(cap)
                },
                2 => {
                    let slice_len = _to_usize(GLOBAL_DATA, base) % 65;
                    let mut temp = vec![];
                    for i in 0..slice_len {
                        temp.push(CustomType1(_to_usize(GLOBAL_DATA, base + 1 + i * 8)));
                    }
                    base += 1 + slice_len * 8;
                    SmallVec::<[CustomType1; 16]>::from_slice(&temp)
                },
                _ => SmallVec::<[CustomType1; 16]>::new(),
            };
            vec_pool.push(sv);
        }

        for i in 0..vec_pool.len() {
            let op_count = _to_usize(GLOBAL_DATA, base) % 5;
            base += 8;

            for _ in 0..op_count {
                let op_type = _to_u8(GLOBAL_DATA, base) % 6;
                base += 1;

                match op_type {
                    0 => {
                        vec_pool[i].push(CustomType1(_to_usize(GLOBAL_DATA, base)));
                        base += 8;
                    },
                    1 => { vec_pool[i].pop(); },
                    2 => {
                        let idx = _to_usize(GLOBAL_DATA, base);
                        let elem = CustomType1(_to_usize(GLOBAL_DATA, base + 8));
                        base += 16;
                        vec_pool[i].insert(idx, elem);
                    },
                    3 => {
                        let start = _to_usize(GLOBAL_DATA, base);
                        let end = _to_usize(GLOBAL_DATA, base + 8);
                        base += 16;
                        vec_pool[i].drain(start..end);
                    },
                    4 => {
                        let elem = CustomType1(_to_usize(GLOBAL_DATA, base));
                        base += 8;
                        vec_pool[i].extend_from_slice(&[elem]);
                    },
                    _ => {
                        let idx = _to_usize(GLOBAL_DATA, base);
                        let _ = vec_pool[i].index_mut(idx);
                        base += 8;
                    }
                }
            }
        }

        let final_selector = _to_u8(GLOBAL_DATA, base) % vec_pool.len() as u8;
        let target_vec = &mut vec_pool[final_selector as usize];
        let index = _to_usize(GLOBAL_DATA, base + 1);
        let elem = target_vec.index_mut(index);
        *elem = CustomType1(_to_usize(GLOBAL_DATA, base + 9));
        println!("{:?}", target_vec.as_mut_slice());
    });
}

// ... (conversion functions and utility functions omitted as per direction II.2)

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