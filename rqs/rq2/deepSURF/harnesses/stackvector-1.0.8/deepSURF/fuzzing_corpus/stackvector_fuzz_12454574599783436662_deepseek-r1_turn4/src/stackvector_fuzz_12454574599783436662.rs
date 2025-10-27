#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use stackvector::*;
use global_data::*;
use std::ops::{Deref, DerefMut, Index, IndexMut};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
struct CustomType1(String);

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 4096 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let g = global_data.first_half;

        let op_count = _to_u8(g, 0) % 5;
        let mut offset = 1;

        for _ in 0..op_count {
            let op_selector = _to_u8(g, offset) % 4;
            offset += 1;

            match op_selector {
                0 => {
                    let constructor = _to_u8(g, offset) % 3;
                    offset += 1;

                    let mut elements = Vec::new();
                    let elem_count = _to_u8(g, offset) % 65;
                    offset += 1;

                    for _ in 0..elem_count {
                        let s_len = _to_u8(g, offset) % 20;
                        offset += 1;
                        let end = offset + s_len as usize;
                        if end > g.len() { break; }
                        let s = _to_str(g, offset, end);
                        elements.push(CustomType1(s.to_string()));
                        offset = end;
                    }

                    let mut sv = match constructor {
                        0 => stackvector::StackVec::<[CustomType1; 128]>::from(elements),
                        1 => stackvector::StackVec::from_iter(elements.iter().cloned()),
                        _ => {
                            let mut sv = stackvector::StackVec::<[CustomType1; 128]>::new();
                            for e in elements { sv.push(e); }
                            sv
                        }
                    };

                    for i in 0.._to_usize(g, offset) % 64 {
                        if let Some(e) = sv.get(i) {
                            println!("Elem {}: {:?}", i, *e);
                        }
                        if let Some(e) = sv.get_mut(i) {
                            e.0.push_str("_mutated");
                        }
                    }
                    offset += 8;

                    let drain = sv.drain();
                    for e in drain { println!("Drained: {:?}", e); }

                    let cmp_sv = stackvector::StackVec::<[CustomType1; 128]>::new();
                    println!("Cmp: {:?}", sv.partial_cmp(&cmp_sv));
                },
                1 => {
                    let elem = CustomType1("FUZZ".into());
                    let count = _to_usize(g, offset);
                    offset += 8;
                    let sv = stackvector::StackVec::<[CustomType1; 128]>::from_elem(elem, count);
                    println!("FromElem len: {:?}", sv.len());
                },
                2 => {
                    let buffer: [CustomType1; 128] = std::array::from_fn(|_| CustomType1(String::new()));
                    let sv = stackvector::StackVec::<[CustomType1; 128]>::from_buf(buffer);
                    sv.into_vec();
                },
                3 => {
                    let mut sv = stackvector::StackVec::<[CustomType1; 128]>::new();
                    let trunc_len = _to_usize(g, offset);
                    offset += 8;
                    sv.truncate(trunc_len);

                    let insert_pos = _to_usize(g, offset);
                    offset += 8;
                    if let Some(_) = sv.get(insert_pos) {
                        sv.insert(insert_pos, CustomType1("inserted".into()));
                    }
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