#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::SmallVec;
use global_data::*;
use std::fmt::Debug;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct CustomType1(String);

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 4096 { return; }
        set_global_data(data);
        let global_data = get_global_data();

        let mut offset = 0;
        let op_count = _to_usize(global_data.first_half, offset) % 8 + 1;
        offset += std::mem::size_of::<usize>();

        let constructor_select = _to_u8(global_data.first_half, offset) % 3;
        offset += 1;
        let mut sv1 = match constructor_select {
            0 => SmallVec::<[CustomType1; 32]>::new(),
            1 => {
                let capacity = _to_usize(global_data.first_half, offset);
                offset += std::mem::size_of::<usize>();
                SmallVec::with_capacity(capacity)
            },
            _ => {
                let elem_count = _to_u8(global_data.first_half, offset) % 65;
                offset += 1;
                let mut elements = Vec::new();
                for _ in 0..elem_count {
                    let len = _to_u8(global_data.first_half, offset) as usize;
                    offset += 1;
                    let s = _to_str(global_data.first_half, offset, offset + len);
                    offset += len;
                    elements.push(CustomType1(s.to_string()));
                }
                SmallVec::from_vec(elements)
            }
        };

        for _ in 0..op_count {
            match _to_u8(global_data.first_half, offset) % 7 {
                0 => {
                    offset += 1;
                    let len = _to_u8(global_data.first_half, offset) as usize;
                    offset += 1;
                    let s = _to_str(global_data.first_half, offset, offset + len);
                    offset += len;
                    sv1.push(CustomType1(s.to_string()));
                },
                1 => { let _ = sv1.pop(); },
                2 => {
                    let idx = _to_usize(global_data.first_half, offset);
                    offset += std::mem::size_of::<usize>();
                    if idx <= sv1.len() {
                        let len = _to_u8(global_data.first_half, offset) as usize;
                        offset += 1;
                        let s = _to_str(global_data.first_half, offset, offset + len);
                        offset += len;
                        sv1.insert(idx, CustomType1(s.to_string()));
                    }
                },
                3 => {
                    let new_len = _to_usize(global_data.first_half, offset);
                    offset += std::mem::size_of::<usize>();
                    sv1.truncate(new_len);
                },
                4 => {
                    let start = _to_usize(global_data.first_half, offset);
                    offset += std::mem::size_of::<usize>();
                    let end = _to_usize(global_data.first_half, offset);
                    offset += std::mem::size_of::<usize>();
                    if start <= end && end <= sv1.len() {
                        let _ = sv1.drain(start..end);
                    }
                },
                5 => {
                    offset += 1;
                    if let Some(e) = sv1.last() {
                        println!("Last element: {:?}", e);
                    }
                    let capacity = _to_usize(global_data.first_half, offset);
                    offset += std::mem::size_of::<usize>();
                    sv1.reserve(capacity);
                },
                _ => {
                    let mut sv2 = SmallVec::<[CustomType1; 32]>::new();
                    let elem_count = _to_u8(global_data.first_half, offset) % 65;
                    offset += 1;
                    for _ in 0..elem_count {
                        let len = _to_u8(global_data.first_half, offset) as usize;
                        offset += 1;
                        let s = _to_str(global_data.first_half, offset, offset + len);
                        offset += len;
                        sv2.push(CustomType1(s.to_string()));
                    }
                    sv1.append(&mut sv2);
                }
            }
        }

        let mut sv2 = SmallVec::<[CustomType1; 32]>::new();
        let elem_count = _to_u8(global_data.second_half, 0) % 65;
        for i in 0..elem_count {
            let len = _to_u8(global_data.second_half, 1 + i as usize) as usize;
            let start = 2 + i as usize;
            let s = _to_str(global_data.second_half, start, start + len);
            sv2.push(CustomType1(s.to_string()));
        }

        sv1.append(&mut sv2);
        let ordering = sv1.cmp(&sv2);
        println!("Final comparison: {:?}", ordering);
        println!("sv1: {:?}", sv1);
        println!("sv2: {:?}", sv2);
    });
}

// Type conversion functions remain unchanged as per directions

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