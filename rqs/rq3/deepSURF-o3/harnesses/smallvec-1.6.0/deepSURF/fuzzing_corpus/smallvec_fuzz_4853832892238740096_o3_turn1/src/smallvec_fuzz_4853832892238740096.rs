#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 200 {
            return;
        }
        set_global_data(data);
        let global_data = get_global_data();
        let G = global_data.first_half;
        let mut idx: usize = 0;

        let constructor_tag = _to_u8(G, idx);
        idx += 1;

        let mut buf = [0u8; 32];
        for i in 0..32 {
            buf[i] = _to_u8(G, idx + i);
        }
        idx += 32;

        if idx + 8 > G.len() {
            return;
        }
        let len_param = _to_usize(G, idx);
        idx += 8;

        let mut sv: SmallVec<[u8; 32]> = match constructor_tag % 5 {
            0 => SmallVec::from_buf_and_len(buf, len_param),
            1 => SmallVec::from_buf(buf),
            2 => {
                let mut v = Vec::new();
                for i in 0..16 {
                    v.push(_to_u8(G, idx + i));
                }
                idx += 16;
                SmallVec::from_vec(v)
            }
            3 => SmallVec::with_capacity(len_param % 65),
            _ => SmallVec::new(),
        };

        if idx >= G.len() {
            return;
        }
        let ops_count = (_to_u8(G, idx) % 20) as usize;
        idx += 1;

        for _ in 0..ops_count {
            if idx >= G.len() {
                break;
            }
            let op_tag = _to_u8(G, idx);
            idx += 1;
            match op_tag % 10 {
                0 => {
                    if idx >= G.len() {
                        break;
                    }
                    let val = _to_u8(G, idx);
                    idx += 1;
                    sv.push(val);
                }
                1 => {
                    sv.pop();
                }
                2 => {
                    if idx + 8 >= G.len() {
                        break;
                    }
                    let pos = _to_usize(G, idx);
                    idx += 8;
                    if idx >= G.len() {
                        break;
                    }
                    let val = _to_u8(G, idx);
                    idx += 1;
                    let insert_pos = if sv.is_empty() { 0 } else { pos % sv.len() };
                    sv.insert(insert_pos, val);
                }
                3 => {
                    if !sv.is_empty() {
                        if idx + 8 >= G.len() {
                            break;
                        }
                        let pos = _to_usize(G, idx);
                        idx += 8;
                        sv.remove(pos % sv.len());
                    }
                }
                4 => {
                    if idx + 8 >= G.len() {
                        break;
                    }
                    let new_len = _to_usize(G, idx) % 65;
                    idx += 8;
                    sv.truncate(new_len);
                }
                5 => {
                    if idx >= G.len() {
                        break;
                    }
                    let slice_len = (_to_u8(G, idx) % 16) as usize;
                    idx += 1;
                    if idx + slice_len > G.len() {
                        break;
                    }
                    let slice = &G[idx..idx + slice_len];
                    idx += slice_len;
                    sv.extend_from_slice(slice);
                }
                6 => {
                    let slice_ref = sv.as_slice();
                    println!("{:?}", slice_ref);
                }
                7 => {
                    if let Some(v) = sv.pop() {
                        sv.push(v);
                    }
                }
                8 => {
                    if idx + 8 >= G.len() {
                        break;
                    }
                    let add = _to_usize(G, idx);
                    idx += 8;
                    sv.reserve(add);
                }
                _ => {
                    sv.clear();
                }
            }
        }

        let mut clone_vec = sv.clone();
        let slice_after = clone_vec.as_slice();
        println!("{:?}", slice_after);

        let mut vec2: SmallVec<[u8; 32]> = SmallVec::from_slice(slice_after);
        vec2.append(&mut clone_vec);
        let _ = vec2.partial_cmp(&clone_vec);
        let _ = vec2.cmp(&clone_vec);

        let mut iter = vec2.clone().into_iter();
        while let Some(item) = iter.next() {
            if item % 2 == 0 {
                vec2.push(item);
            }
        }

        if vec2.len() > 1 {
            let mid = vec2.len() / 2;
            let mut d = vec2.drain(0..mid);
            while let Some(v) = d.next() {
                println!("{:?}", v);
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