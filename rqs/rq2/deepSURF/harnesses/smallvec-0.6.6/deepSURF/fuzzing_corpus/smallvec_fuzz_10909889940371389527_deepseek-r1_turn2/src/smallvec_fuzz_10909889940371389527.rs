#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 2048 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let mut offset = 0;

        let constructor_selector = _to_u8(global_data.first_half, 0) % 5;
        let mut sv = match constructor_selector {
            0 => SmallVec::<[String; 16]>::new(),
            1 => {
                let cap = _to_usize(global_data.first_half, 1);
                SmallVec::with_capacity(cap)
            },
            2 => {
                let slice_len = _to_usize(global_data.first_half, 1) % 65;
                let mut elems = Vec::with_capacity(slice_len);
                for i in 0..slice_len {
                    let len = _to_u8(global_data.first_half, 2 + i) as usize;
                    let s = _to_str(global_data.first_half, 10 + i*10, 10 + i*10 + len).to_string();
                    elems.push(s);
                }
                SmallVec::from_vec(elems)
            },
            3 => {
                let count = _to_usize(global_data.first_half, 1) % 65;
                let elem = _to_str(global_data.first_half, 5, 15).to_string();
                SmallVec::from_elem(elem, count)
            },
            _ => SmallVec::from_vec(vec![
                _to_str(global_data.first_half, 20, 25).to_string(),
                _to_str(global_data.first_half, 25, 30).to_string()
            ]),
        };

        let num_ops = _to_u8(global_data.first_half, 100) % 20;
        offset += 100;

        for _ in 0..num_ops {
            if offset + 4 >= global_data.first_half.len() { break; }

            let op = _to_u8(global_data.first_half, offset) % 7;
            offset += 1;

            match op {
                0 => sv.push({
                    let len = _to_u8(global_data.first_half, offset);
                    offset += 1;
                    _to_str(global_data.first_half, offset, offset + len as usize).to_string()
                }),
                1 => {
                    let idx = _to_usize(global_data.first_half, offset);
                    offset += std::mem::size_of::<usize>();
                    sv.insert(idx, {
                        let len = _to_u8(global_data.first_half, offset);
                        offset += 1;
                        _to_str(global_data.first_half, offset, offset + len as usize).to_string()
                    });
                },
                2 => { let _ = sv.pop(); },
                3 => sv.dedup(),
                4 => {
                    let new_len = _to_usize(global_data.first_half, offset);
                    offset += std::mem::size_of::<usize>();
                    sv.truncate(new_len);
                },
                5 => {
                    let drained: Vec<_> = sv.drain().collect();
                    println!("Drained {} items", drained.len());
                },
                6 => {
                    let slice = sv.as_mut_slice();
                    if !slice.is_empty() {
                        let idx = _to_usize(global_data.first_half, offset) % slice.len();
                        println!("Indexing mut: {:?}", &mut slice[idx]);
                    }
                },
                _ => (),
            }
        }

        if let Some(elem) = sv.get(_to_usize(global_data.second_half, 0) % sv.len().max(1)) {
            println!("Elem: {:?}", *elem);
        }
        if !sv.is_empty() {
            println!("Last: {:?}", sv.last().unwrap());
        }
        println!("Capacity: {}", sv.capacity());
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