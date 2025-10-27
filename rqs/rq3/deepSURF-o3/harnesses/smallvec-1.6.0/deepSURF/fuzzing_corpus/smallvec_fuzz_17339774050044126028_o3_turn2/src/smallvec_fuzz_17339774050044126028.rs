#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

fn build_initial_smallvec(choice: u8, g: &[u8]) -> SmallVec<[u8; 32]> {
    match choice % 7 {
        0 => SmallVec::<[u8; 32]>::new(),
        1 => {
            let cap = _to_usize(g, 1);
            SmallVec::<[u8; 32]>::with_capacity(cap)
        }
        2 => {
            let mut buf = [0u8; 32];
            for i in 0..32 {
                buf[i] = _to_u8(g, i);
            }
            SmallVec::from_buf(buf)
        }
        3 => {
            let mut buf = [0u8; 32];
            for i in 0..32 {
                buf[i] = _to_u8(g, i);
            }
            let len = _to_usize(g, 33);
            SmallVec::from_buf_and_len(buf, len)
        }
        4 => {
            let slice_len = _to_u8(g, 65) as usize;
            let slice = &g[..slice_len.min(g.len())];
            SmallVec::from_slice(slice)
        }
        5 => {
            let vec_len = (_to_u8(g, 66) as usize) % 65;
            let mut v = Vec::with_capacity(vec_len);
            for i in 0..vec_len {
                v.push(_to_u8(g, 67 + i));
            }
            SmallVec::from_vec(v)
        }
        _ => {
            let elem = _to_u8(g, 99);
            let n = _to_usize(g, 100) % 65;
            SmallVec::from_elem(elem, n)
        }
    }
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 300 {
            return;
        }
        set_global_data(data);
        let global_data = get_global_data();
        let G = global_data.first_half;

        let mut sv = build_initial_smallvec(_to_u8(G, 0), G);
        sv.push(_to_u8(G, 132));

        let ops = (_to_u8(G, 108) % 15) as usize;
        for i in 0..ops {
            let base = 110 + i * 10;
            if base + 9 >= G.len() {
                break;
            }
            match _to_u8(G, base) % 10 {
                0 => {
                    let val = _to_u8(G, base + 1);
                    sv.push(val);
                }
                1 => {
                    sv.pop();
                }
                2 => {
                    let len = _to_usize(G, base + 2);
                    sv.truncate(len);
                }
                3 => {
                    let additional = _to_usize(G, base + 3);
                    sv.reserve(additional);
                }
                4 => {
                    let idx = _to_usize(G, base + 4);
                    let val = _to_u8(G, base + 5);
                    sv.insert(idx, val);
                }
                5 => {
                    let idx = _to_usize(G, base + 6);
                    if !sv.is_empty() {
                        sv.remove(idx);
                    }
                }
                6 => {
                    let slice_len = (_to_u8(G, base + 7) as usize) % 65;
                    if base + 8 + slice_len < G.len() {
                        let slice = &G[base + 8..base + 8 + slice_len];
                        sv.extend_from_slice(slice);
                    }
                }
                7 => {
                    let range_end = _to_usize(G, base + 8);
                    let drained_items: SmallVec<[u8; 32]> = sv.drain(0..range_end).collect();
                    sv.extend_from_slice(&drained_items);
                }
                8 => {
                    let slice = sv.as_slice();
                    if let Some(r) = slice.first() {
                        println!("{}", *r);
                    }
                }
                _ => {}
            }
        }

        let cloned = sv.clone();
        let _ = sv.cmp(&cloned);
        let _ = sv.partial_cmp(&cloned);

        if let Some(first) = sv.as_slice().first() {
            println!("{}", *first);
        }

        sv.push(_to_u8(G, 140));
        sv.shrink_to_fit();
        println!("{}", sv.capacity());
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