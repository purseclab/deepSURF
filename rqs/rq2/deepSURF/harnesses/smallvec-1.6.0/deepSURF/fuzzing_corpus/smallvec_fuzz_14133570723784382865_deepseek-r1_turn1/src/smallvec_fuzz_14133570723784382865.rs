#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::cmp::Ordering;

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 4096 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let mut offset = 0;

        let mut vecs = Vec::new();

        let create_method = _to_u8(GLOBAL_DATA, offset) % 3;
        offset += 1;

        match create_method {
            0 => {
                let slice_len = _to_usize(GLOBAL_DATA, offset) % 65;
                offset += 8;
                let slice = &GLOBAL_DATA[offset..(offset + slice_len).min(GLOBAL_DATA.len())];
                offset += slice_len;
                vecs.push(SmallVec::<[u8; 256]>::from_slice(slice));
            }
            1 => {
                let capacity = _to_usize(GLOBAL_DATA, offset) % 65;
                offset += 8;
                let mut sv = SmallVec::<[u8; 256]>::with_capacity(capacity);
                let elem_count = _to_usize(GLOBAL_DATA, offset) % 65;
                offset += 8;
                for _ in 0..elem_count {
                    if offset >= GLOBAL_DATA.len() { break; }
                    sv.push(GLOBAL_DATA[offset]);
                    offset += 1;
                }
                vecs.push(sv);
            }
            2 => {
                let elem = if offset < GLOBAL_DATA.len() { GLOBAL_DATA[offset] } else { 0 };
                offset += 1;
                let count = _to_usize(GLOBAL_DATA, offset) % 65;
                offset += 8;
                vecs.push(SmallVec::<[u8; 256]>::from_elem(elem, count));
            }
            _ => {}
        }

        let ops = _to_u8(GLOBAL_DATA, offset) % 65;
        offset += 1;

        for _ in 0..ops {
            if offset >= GLOBAL_DATA.len() { break; }
            let op = GLOBAL_DATA[offset] % 5;
            offset += 1;

            match op {
                0 => {
                    let cloned = vecs.last().unwrap().clone();
                    vecs.push(cloned);
                }
                1 => {
                    let idx = _to_usize(GLOBAL_DATA, offset);
                    offset += 8;
                    if let Some(mut sv) = vecs.pop() {
                        let elem = if offset < GLOBAL_DATA.len() { GLOBAL_DATA[offset] } else { 0 };
                        offset += 1;
                        sv.insert(idx, elem);
                        vecs.push(sv);
                    }
                }
                2 => {
                    let idx = _to_usize(GLOBAL_DATA, offset);
                    offset += 8;
                    if let Some(mut sv) = vecs.get_mut(0) {
                        let sl = sv.as_mut_slice();
                        println!("{:?}", sl.get(idx));
                    }
                }
                3 => {
                    if vecs.len() > 1 {
                        let ord = vecs[0].cmp(&vecs[1]);
                        println!("{:?}", ord);
                    }
                }
                4 => {
                    if let Some(sv) = vecs.last() {
                        let mut new_sv = sv.clone();
                        new_sv.extend_from_slice(&GLOBAL_DATA[offset..(offset + 8).min(GLOBAL_DATA.len())]);
                        vecs.push(new_sv);
                        offset += 8;
                    }
                }
                _ => {}
            }
        }

        while let Some(mut sv) = vecs.pop() {
            let _ = sv.clone();
            let _ = sv.as_ptr();
            if !sv.is_empty() {
                let idx = _to_usize(GLOBAL_DATA, offset);
                offset += 8;
                println!("{:?}", sv.as_slice()[idx]);
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