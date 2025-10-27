#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use slice_deque::*;
use global_data::*;

#[derive(Debug, Clone)]
struct CustomType0(String);

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 4096 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let mut offset = 0;
        let constructor_selector = _to_u8(GLOBAL_DATA, offset);
        offset += 1;

        let mut deque = match constructor_selector % 5 {
            0 => SliceDeque::new(),
            1 => {
                let cap = _to_usize(GLOBAL_DATA, offset);
                offset += 8;
                SliceDeque::with_capacity(cap)
            }
            2 => {
                let elem_count = _to_u8(GLOBAL_DATA, offset) % 65;
                offset += 1;
                let mut items = Vec::new();
                for _ in 0..elem_count {
                    let len = _to_u8(GLOBAL_DATA, offset) % 17;
                    offset += 1;
                    let s = _to_str(GLOBAL_DATA, offset, offset + len as usize);
                    offset += len as usize;
                    items.push(CustomType0(s.to_string()));
                }
                SliceDeque::from(items.as_slice())
            }
            3 => {
                let elem_len = _to_usize(GLOBAL_DATA, offset);
                offset += 8;
                let len = _to_u8(GLOBAL_DATA, offset) % 17;
                offset += 1;
                let s = _to_str(GLOBAL_DATA, offset, offset + len as usize);
                offset += len as usize;
                slice_deque::from_elem(CustomType0(s.to_string()), elem_len)
            }
            _ => {
                let elem_count = _to_u8(GLOBAL_DATA, offset) % 65;
                offset += 1;
                let mut items = Vec::new();
                for _ in 0..elem_count {
                    let len = _to_u8(GLOBAL_DATA, offset) % 17;
                    offset += 1;
                    let s = _to_str(GLOBAL_DATA, offset, offset + len as usize);
                    offset += len as usize;
                    items.push(CustomType0(s.to_string()));
                }
                SliceDeque::from_iter(items.into_iter())
            }
        };

        let ops_count = _to_usize(GLOBAL_DATA, offset) % 32;
        offset += 8;

        for _ in 0..ops_count {
            let op = _to_u8(GLOBAL_DATA, offset) % 7;
            offset += 1;
            match op {
                0 => {
                    let len = _to_u8(GLOBAL_DATA, offset) % 17;
                    offset += 1;
                    let s = _to_str(GLOBAL_DATA, offset, offset + len as usize);
                    offset += len as usize;
                    deque.push_back(CustomType0(s.to_string()));
                }
                1 => {
                    let len = _to_u8(GLOBAL_DATA, offset) % 17;
                    offset += 1;
                    let s = _to_str(GLOBAL_DATA, offset, offset + len as usize);
                    offset += len as usize;
                    deque.push_front(CustomType0(s.to_string()));
                }
                2 => { deque.pop_back(); }
                3 => { deque.pop_front(); }
                4 => { 
                    let target = _to_usize(GLOBAL_DATA, offset);
                    offset += 8;
                    deque.truncate(target);
                }
                5 => {
                    let start = _to_usize(GLOBAL_DATA, offset);
                    offset += 8;
                    let end = _to_usize(GLOBAL_DATA, offset);
                    offset += 8;
                    let _ = deque.drain(start..end);
                }
                _ => {
                    let idx = _to_usize(GLOBAL_DATA, offset) % deque.len();
                    offset += 8;
                    println!("{:?}", &deque[idx]);
                }
            }
        }

        let reserve_amount = _to_usize(GLOBAL_DATA, offset);
        offset += 8;
        deque.reserve_exact(reserve_amount);

        let post_ops_count = _to_usize(GLOBAL_DATA, offset) % 32;
        offset += 8;

        for _ in 0..post_ops_count {
            let op = _to_u8(GLOBAL_DATA, offset) % 7;
            offset += 1;
            match op {
                0 => println!("Front: {:?}", deque.front()),
                1 => println!("Back: {:?}", deque.back()),
                2 => {
                    if let Some(e) = deque.front_mut() {
                        e.0.push('!');
                    }
                }
                3 => {
                    if let Some(e) = deque.back_mut() {
                        e.0.push_str("?");
                    }
                }
                4 => {
                    let (a, b) = deque.as_slices();
                    println!("{:?} | {:?}", a, b);
                }
                5 => {
                    let mut iter = deque.iter();
                    let _ = iter.next().map(|e| println!("First: {:?}", e));
                }
                _ => {
                    deque.shrink_to_fit();
                }
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