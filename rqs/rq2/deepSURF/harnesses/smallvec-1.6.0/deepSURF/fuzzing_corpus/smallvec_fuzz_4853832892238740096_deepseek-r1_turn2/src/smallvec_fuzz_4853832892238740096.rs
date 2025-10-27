#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::SmallVec;
use global_data::*;
use std::cmp::Ordering;

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 4096 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let op_count = _to_usize(GLOBAL_DATA, 0) % 16;
        let mut vecs = vec![];

        for i in 0..op_count {
            let op_type = _to_u8(GLOBAL_DATA, i * 3) % 7;
            let offset = i * 8;

            match op_type {
                0 => {
                    let capacity = _to_usize(GLOBAL_DATA, offset);
                    let sv = SmallVec::<[u8; 64]>::with_capacity(capacity);
                    println!("Created capacity: {:?}", sv.capacity());
                    vecs.push(sv);
                }
                1 => {
                    let elem = _to_u8(GLOBAL_DATA, offset);
                    let count = _to_usize(GLOBAL_DATA, offset + 1) % 65;
                    let sv = SmallVec::from_elem(elem, count);
                    println!("From_elem len: {}", sv.len());
                    vecs.push(sv);
                }
                2 => {
                    let slice_start = _to_usize(GLOBAL_DATA, offset) % (GLOBAL_DATA.len() - 64);
                    let len = _to_usize(GLOBAL_DATA, offset + 4) % 65;
                    let slice = &GLOBAL_DATA[slice_start..slice_start + len];
                    let sv = SmallVec::from_slice(slice);
                    println!("From_slice: {:?}", sv);
                    vecs.push(sv);
                }
                3 => {
                    let mut arr = [0u8; 64];
                    let buf_start = _to_usize(GLOBAL_DATA, offset) % (GLOBAL_DATA.len() - 64);
                    arr.copy_from_slice(&GLOBAL_DATA[buf_start..buf_start + 64]);
                    let len = _to_usize(GLOBAL_DATA, offset + 64);
                    let sv = SmallVec::from_buf_and_len(arr, len);
                    println!("From_buf_and_len: {:?}", sv.as_ptr());
                    vecs.push(sv);
                }
                4 => {
                    if let Some(mut sv) = vecs.pop() {
                        let new_len = _to_usize(GLOBAL_DATA, offset);
                        sv.resize(new_len, _to_u8(GLOBAL_DATA, offset + 1));
                        let cap = sv.capacity();
                        println!("Resized to: {} (cap {})", sv.len(), cap);
                    }
                }
                5 => {
                    if let Some(sv) = vecs.last() {
                        let idx = _to_usize(GLOBAL_DATA, offset) % (sv.len() + 1);
                        let value = _to_u8(GLOBAL_DATA, offset + 1);
                        let mut sv_clone = sv.clone();
                        sv_clone.insert(idx, value);
                        println!("Inserted at {}: {:?}", idx, sv_clone.as_slice());
                    }
                }
                6 => {
                    if vecs.len() >= 2 {
                        let (a, b) = (vecs.pop().unwrap(), vecs.pop().unwrap());
                        let ord = a.cmp(&b);
                        println!("Compare result: {:?}", ord);
                        vecs.extend([a, b]);
                    }
                }
                _ => {}
            }

            if !vecs.is_empty() {
                let last_idx = vecs.len() - 1;
                let sv = &mut vecs[last_idx];
                let drain_start = _to_usize(GLOBAL_DATA, offset) % (sv.len() + 1);
                let drain_end = _to_usize(GLOBAL_DATA, offset + 1) % (sv.len() + 1);
                let drain_range = drain_start.min(drain_end)..drain_end.max(drain_start);
                sv.drain(drain_range.clone());
                println!("Drained range: {:?}", drain_range);
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