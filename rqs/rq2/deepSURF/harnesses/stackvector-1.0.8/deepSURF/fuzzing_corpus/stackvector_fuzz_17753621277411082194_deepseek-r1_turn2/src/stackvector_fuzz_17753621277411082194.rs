#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use stackvector::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};
use std::io::Write;

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 512 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let g1 = global_data.first_half;
        let g2 = global_data.second_half;

        let op_count = _to_usize(g1, 0) % 8;
        let mut buffers = vec![];

        for i in 0..op_count {
            let selector = _to_u8(g2, i as usize) % 6;
            match selector {
                0 => {
                    let buf = [0u8; 64];
                    let len = _to_usize(g1, 8);
                    let mut vec = StackVec::from_buf_and_len(buf, len);
                    let push_count = _to_usize(g2, 16) % 65;
                    for j in 0..push_count {
                        vec.push(_to_u8(g1, 24 + j));
                    }
                    buffers.push(vec);
                }
                1 => {
                    let elem = _to_u8(g1, 128);
                    let count = _to_usize(g1, 129);
                    buffers.push(StackVec::from_elem(elem, count));
                }
                2 => {
                    let start = _to_usize(g1, 200) % g2.len();
                    let end = start + (_to_usize(g1, 208) % 65);
                    let slice = if end <= g2.len() { &g2[start..end] } else { &[] };
                    buffers.push(StackVec::from_slice(slice));
                }
                3 => {
                    let count = _to_usize(g1, 256) % 65;
                    let items: Vec<_> = g2.iter().take(count).cloned().collect();
                    buffers.push(StackVec::from_iter(items));
                }
                4 => {
                    let mut vec: StackVec<[u8; 64]> = StackVec::new();
                    let trunc_pos = _to_usize(g1, 300) % 65;
                    vec.truncate(trunc_pos);
                    buffers.push(vec);
                }
                5 => {
                    let mut vec = StackVec::from_slice(&[]);
                    let write_len = _to_usize(g1, 320) % g2.len();
                    let write_data = &g2[..write_len];
                    let _ = vec.write_all(write_data);
                    buffers.push(vec);
                }
                _ => {}
            }
        }

        let write_idx = _to_usize(g1, 400) % buffers.len();
        let data_start = _to_usize(g1, 408) % g1.len();
        let data_end = data_start + (_to_usize(g1, 416) % 65);
        let write_slice = if data_end <= g1.len() { &g1[data_start..data_end] } else { &[] };

        if let Some(vec) = buffers.get_mut(write_idx) {
            _unwrap_result(vec.write_all(write_slice));
            let drain = vec.drain();
            for elem in drain {
                println!("{:?}", elem);
            }
            let cmp_value = _to_u8(g1, 424);
            let cmp_vec = StackVec::from_elem(cmp_value, vec.len());
            let _ = vec.deref().partial_cmp(&cmp_vec);
            vec.clear();
        }

        for mut vec in buffers {
            let idx = _to_usize(g2, 500) % (vec.len() + 1);
            if vec.len() > 0 {
                println!("{:?}", &vec[idx]);
                vec.swap_remove(idx % vec.len());
            }
            let _ = vec.into_vec();
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