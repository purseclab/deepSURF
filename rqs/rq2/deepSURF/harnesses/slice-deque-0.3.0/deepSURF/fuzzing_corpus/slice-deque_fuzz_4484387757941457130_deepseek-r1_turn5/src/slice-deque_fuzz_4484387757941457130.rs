#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use slice_deque::*;
use global_data::*;

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 1 {return;}
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let mut idx = 0;

        let op_count = if GLOBAL_DATA.len() > 0 { (_to_u8(GLOBAL_DATA, idx) % 65) as usize } else { 0 };
        idx += 1;

        for _ in 0..op_count {
            if idx >= GLOBAL_DATA.len() { break; }
            let op = _to_u8(GLOBAL_DATA, idx) % 7;
            idx += 1;

            match op {
                0 => {
                    let deque = SliceDeque::<u8>::new();
                    println!("New deque: {:?}", deque.as_slice());
                    let len = _to_usize(GLOBAL_DATA, idx);
                    idx += 8;
                    let mut cloned = deque.clone();
                    cloned.shrink_to_fit();
                },
                1 => {
                    let capacity = _to_usize(GLOBAL_DATA, idx) % 65;
                    idx += 8;
                    let mut deque = SliceDeque::<u8>::with_capacity(capacity);
                    let slice = deque.as_mut_slice();
                    println!("With capacity: {:?}", slice);
                },
                2 => {
                    let len = _to_usize(GLOBAL_DATA, idx);
                    idx += 8;
                    if idx + len > GLOBAL_DATA.len() { continue; }
                    let data = &GLOBAL_DATA[idx..idx+len];
                    idx += len;
                    let mut deque = SliceDeque::from(data);
                    let _ = deque.drain(0..deque.len());
                },
                3 => {
                    let mut deque = SliceDeque::default();
                    let cnt = _to_u8(GLOBAL_DATA, idx) % 65;
                    idx += 1;
                    for _ in 0..cnt {
                        if idx >= GLOBAL_DATA.len() { break; }
                        deque.push_back(_to_u8(GLOBAL_DATA, idx));
                        idx += 1;
                    }
                    let _res = deque.try_reserve(_to_usize(GLOBAL_DATA, idx));
                    idx += 8;
                },
                4 => {
                    let size = _to_usize(GLOBAL_DATA, idx);
                    idx += 8;
                    let mut deque: SliceDeque<_> = (0..size).map(|_| _to_u8(GLOBAL_DATA, idx)).collect();
                    let mut iter = deque.into_iter();
                    while let Some(v) = iter.next() {
                        println!("Drained: {}", v);
                    }
                },
                5 => {
                    let mut deque = SliceDeque::new();
                    let val = _to_u8(GLOBAL_DATA, idx);
                    idx += 1;
                    deque.push_front(val);
                    let _ = deque.splice(0..1, std::iter::empty::<u8>());
                    println!("After splice: {:?}", deque.as_slice());
                },
                6 => {
                    let mut deque = SliceDeque::default();
                    let value = _to_u8(GLOBAL_DATA, idx);
                    idx += 1;
                    deque.push_front(value);
                    {
                        let _drained = deque.drain(0..1);
                    }
                    let _formatted = format!("{:?}", deque);
                },
                _ => unreachable!()
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