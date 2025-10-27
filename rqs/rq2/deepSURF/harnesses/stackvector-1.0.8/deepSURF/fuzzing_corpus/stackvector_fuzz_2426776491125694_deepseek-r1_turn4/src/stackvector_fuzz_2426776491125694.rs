#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use stackvector::*;
use global_data::*;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 1024 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let fh = global_data.first_half;
        let sh = global_data.second_half;

        let constructors = [_to_usize(fh, 0) % 3, _to_usize(fh, 8) % 3, _to_usize(fh, 16) % 3];

        let mut vec0 = {
            let i = 0;
            let base = i * 24;
            match constructors[i] {
                0 => {
                    let buf = [_to_u8(fh, base); 128];
                    let len = _to_usize(fh, base + 1) % 65;
                    StackVec::from_buf_and_len(buf, len)
                }
                1 => {
                    let slice_len = _to_usize(fh, base) % 65;
                    let slice_data = &sh[_to_usize(fh, base+8)..][..slice_len];
                    StackVec::from_slice(slice_data)
                }
                _ => StackVec::<[u8; 128]>::new()
            }
        };

        let mut vec1 = {
            let i = 1;
            let base = i * 24;
            match constructors[i] {
                0 => {
                    let buf = [_to_u8(fh, base); 128];
                    let len = _to_usize(fh, base + 1) % 65;
                    StackVec::from_buf_and_len(buf, len)
                }
                1 => {
                    let slice_len = _to_usize(fh, base) % 65;
                    let slice_data = &sh[_to_usize(fh, base+8)..][..slice_len];
                    StackVec::from_slice(slice_data)
                }
                _ => StackVec::<[u8; 128]>::new()
            }
        };

        let mut vec2 = {
            let i = 2;
            let base = i * 24;
            match constructors[i] {
                0 => {
                    let buf = [_to_u8(fh, base); 128];
                    let len = _to_usize(fh, base + 1) % 65;
                    StackVec::from_buf_and_len(buf, len)
                }
                1 => {
                    let slice_len = _to_usize(fh, base) % 65;
                    let slice_data = &sh[_to_usize(fh, base+8)..][..slice_len];
                    StackVec::from_slice(slice_data)
                }
                _ => StackVec::<[u8; 128]>::new()
            }
        };

        let mut hasher = DefaultHasher::new();
        let op_count = _to_usize(sh, 0) % 65;
        let mut data_idx = 8;

        for _ in 0..op_count {
            if data_idx >= sh.len() { break; }
            let op = _to_u8(sh, data_idx) % 10;
            data_idx += 1;

            match op {
                0 => vec0.push(_to_u8(sh, data_idx)),
                1 => { let _ = vec1.pop(); },
                2 => vec2.insert(_to_usize(sh, data_idx), _to_u8(sh, data_idx + 8)),
                3 => vec0.extend_from_slice(&sh[data_idx..][.._to_usize(sh, data_idx + 1) % 65]),
                4 => vec1.truncate(_to_usize(sh, data_idx)),
                5 => {
                    vec0.hash(&mut hasher);
                    let _ = hasher.finish();
                }
                6 => {
                    let drained: Vec<_> = vec2.drain().collect();
                    println!("Drained {} elements", drained.len());
                }
                7 => vec1.insert_from_slice(_to_usize(sh, data_idx), &vec0.as_slice()[.._to_usize(sh, data_idx+8) % 65]),
                8 => vec2.retain(|x| *x != _to_u8(sh, data_idx)),
                _ => {
                    match vec0.into_inner() {
                        Ok(inner) => vec0 = StackVec::from_buf(inner),
                        Err(e) => vec0 = e,
                    }
                }
            };
            data_idx = data_idx.wrapping_add(16);
        }

        let slice = &vec0[_to_usize(sh, data_idx)..][.._to_usize(sh, data_idx+8) % 65];
        println!("Slice: {:?}", slice);
        vec1.extend_from_slice(slice);

        vec2.hash(&mut hasher);
        println!("Final hash: {:?}", hasher.finish());

        let cmp_result = vec0.partial_cmp(&vec1);
        println!("Comparison: {:?}", cmp_result);
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