#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use slice_deque::*;
use global_data::*;
use std::ops::{Deref, DerefMut};

#[derive(Clone)]
struct CustomType2(String);
struct CustomType3;
struct CustomType0(String);
struct CustomType4(String);

impl std::ops::RangeBounds<CustomType2> for CustomType3 {
    fn end_bound(&self) -> std::collections::Bound<&CustomType2> {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let selector = _to_usize(GLOBAL_DATA, 42) % 3;
        if selector == 0 {
            panic!("INTENTIONAL PANIC!");
        }
        std::collections::Bound::Unbounded
    }

    fn start_bound(&self) -> std::collections::Bound<&CustomType2> {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.second_half;
        let selector = _to_usize(GLOBAL_DATA, 512) % 3;
        if selector == 0 {
            panic!("INTENTIONAL PANIC!");
        }
        std::collections::Bound::Unbounded
    }
}

impl std::iter::Iterator for CustomType4 {
    type Item = CustomType2;
    fn next(&mut self) -> Option<Self::Item> {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let str_len = _to_u8(GLOBAL_DATA, 768) % 17;
        Some(CustomType2(_to_str(GLOBAL_DATA, 896, 896 + str_len as usize).to_string()))
    }
}

impl std::iter::IntoIterator for CustomType0 {
    type Item = CustomType2;
    type IntoIter = CustomType4;
    fn into_iter(self) -> Self::IntoIter {
        CustomType4(self.0)
    }
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 4096 { return; }
        set_global_data(data);
        let global_data = get_global_data();

        let mut deques = Vec::new();
        for _ in 0..3 {
            let constructor_sel = _to_u8(global_data.first_half, 0) % 3;
            let deq = match constructor_sel {
                0 => SliceDeque::new(),
                1 => {
                    let cap = _to_usize(global_data.second_half, 8);
                    SliceDeque::with_capacity(cap)
                }
                _ => {
                    let vec_len = _to_u8(global_data.first_half, 16) % 65;
                    let mut v = Vec::new();
                    for i in 0..vec_len {
                        let s = _to_str(global_data.second_half, i as usize * 32, (i as usize + 1) * 32);
                        v.push(CustomType2(s.to_string()));
                    }
                    SliceDeque::from(&v[..])
                }
            };
            deques.push(deq);
        }

        for d in &mut deques {
            for _ in 0..5 {
                let op = _to_u8(global_data.first_half, 128) % 8;
                match op {
                    0 => {
                        let idx = _to_usize(global_data.second_half, 256);
                        if let Some(x) = d.get(idx) {
                            println!("{:?}", x.0);
                        }
                    }
                    1 => {
                        let str_len = _to_u8(global_data.second_half, 384) % 17;
                        let s = _to_str(global_data.first_half, 512, 512 + str_len as usize);
                        d.push_back(CustomType2(s.to_string()));
                    }
                    2 => {
                        let idx = _to_usize(global_data.first_half, 640);
                        d.remove(idx);
                    }
                    3 => {
                        let range_start = _to_usize(global_data.second_half, 768);
                        let range_end = _to_usize(global_data.second_half, 1024);
                        let mut drained = d.drain(range_start..range_end);
                        while let Some(x) = drained.next() {
                            println!("Drained: {}", x.0);
                        }
                    }
                    4 => {
                        let r_sel = _to_u8(global_data.first_half, 896) % 2;
                        let range = if r_sel == 0 {
                            0..d.len()/2
                        } else {
                            d.len()/3..2*d.len()/3
                        };
                        let iter = CustomType0(_to_str(global_data.second_half, 1152, 1280).to_string());
                        let mut splice = d.splice(range, iter);
                        splice.by_ref().count();
                    }
                    5 => {
                        let len = _to_usize(global_data.first_half, 1536);
                        d.truncate(len);
                    }
                    6 => {
                        let filter_byte = _to_u8(global_data.second_half, 1792);
                        let mut drain_filter = d.drain_filter(|x| x.0.len() % 2 == filter_byte as usize % 2);
                        drain_filter.next();
                    }
                    7 => {
                        let ext_len = _to_u8(global_data.first_half, 2048) % 65;
                        let mut ext_vec = Vec::new();
                        for i in 0..ext_len {
                            let s = _to_str(global_data.second_half, 2304 + i as usize * 16, 2304 + (i as usize + 1) * 16);
                            ext_vec.push(CustomType2(s.to_string()));
                        }
                        d.extend_from_slice(&ext_vec);
                    }
                    _ => unreachable!()
                };
            }
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