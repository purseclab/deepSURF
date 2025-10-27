#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::SmallVec;
use global_data::*;

#[derive(Debug)]
struct CustomType1(String);

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 3000 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let num_ops = _to_u8(GLOBAL_DATA, 0) as usize;
        let mut data_offset = 1;

        for _ in 0..num_ops {
            if data_offset + 2 >= GLOBAL_DATA.len() { break; }

            let op_selector = _to_u8(GLOBAL_DATA, data_offset) % 5;
            data_offset += 1;

            match op_selector {
                0 => {
                    let capacity = _to_usize(GLOBAL_DATA, data_offset);
                    let mut sv = SmallVec::<[CustomType1; 32]>::with_capacity(capacity);
                    data_offset += 2;

                    let elem_count = _to_u8(GLOBAL_DATA, data_offset) as usize % 15;
                    data_offset += 1;
                    
                    for _ in 0..elem_count {
                        if data_offset + 2 > GLOBAL_DATA.len() { break; }
                        let len = _to_u8(GLOBAL_DATA, data_offset) % 17;
                        let s = _to_str(GLOBAL_DATA, data_offset + 1, data_offset + 1 + len as usize);
                        sv.push(CustomType1(s.to_string()));
                        data_offset += 1 + len as usize;
                    }

                    if !sv.is_empty() {
                        let _ = sv.pop();
                    }
                }
                1 => {
                    let slice_len = _to_u8(GLOBAL_DATA, data_offset) as usize;
                    data_offset += 1;
                    let mut temp = Vec::new();
                    
                    for _ in 0..slice_len {
                        if data_offset + 2 > GLOBAL_DATA.len() { break; }
                        let len = _to_u8(GLOBAL_DATA, data_offset) % 17;
                        let s = _to_str(GLOBAL_DATA, data_offset + 1, data_offset + 1 + len as usize);
                        temp.push(CustomType1(s.to_string()));
                        data_offset += 1 + len as usize;
                    }
                    
                    let sv = SmallVec::<[CustomType1; 32]>::from_vec(temp);
                    if let Some(item) = sv.get(0) {
                        println!("{:?}", *item);
                    }
                }
                2 => {
                    let elem_count = _to_u8(GLOBAL_DATA, data_offset) as usize;
                    data_offset += 1;
                    let mut sv = SmallVec::<[CustomType1; 32]>::new();

                    for _ in 0..elem_count {
                        if data_offset + 2 > GLOBAL_DATA.len() { break; }
                        let len = _to_u8(GLOBAL_DATA, data_offset) % 17;
                        let s = _to_str(GLOBAL_DATA, data_offset + 1, data_offset + 1 + len as usize);
                        sv.push(CustomType1(s.to_string()));
                        data_offset += 1 + len as usize;
                    }

                    sv.shrink_to_fit();
                    let _cap = sv.capacity();
                }
                3 => {
                    let init_cap = _to_usize(GLOBAL_DATA, data_offset);
                    data_offset += 2;
                    let mut sv = SmallVec::<[CustomType1; 32]>::with_capacity(init_cap);
                    
                    let insert_idx = _to_usize(GLOBAL_DATA, data_offset);
                    data_offset += 2;
                    let len = _to_u8(GLOBAL_DATA, data_offset) % 17;
                    let s = _to_str(GLOBAL_DATA, data_offset + 1, data_offset + 1 + len as usize);
                    sv.insert(insert_idx, CustomType1(s.to_string()));
                    data_offset += 1 + len as usize;
                }
                4 => {
                    let mut t3 = Vec::new();
                    for _ in 0..32 {
                        if data_offset + 2 > GLOBAL_DATA.len() { break; }
                        let len = _to_u8(GLOBAL_DATA, data_offset) % 17;
                        let s = _to_str(GLOBAL_DATA, data_offset + 1, data_offset + 1 + len as usize);
                        t3.push(CustomType1(s.to_string()));
                        data_offset += 1 + len as usize;
                    }
                    
                    if !t3.is_empty() {
                        let trunc_idx = _to_usize(GLOBAL_DATA, data_offset);
                        data_offset += 2;
                        t3.truncate(trunc_idx);
                    }
                    
                    let sv = SmallVec::<[CustomType1; 32]>::from_vec(t3);
                    let full_slice = sv.as_slice();
                    if !full_slice.is_empty() {
                        println!("{:?}", &*full_slice);
                    }
                }
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