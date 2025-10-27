#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::ops::{Deref, DerefMut, Index, IndexMut};

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 1024 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let g = global_data.first_half;

        let mut vecs = Vec::new();
        let mut op_index = 0;

        for _ in 0..3 {
            let constructor = _to_usize(g, op_index) % 6;
            op_index += 2;

            match constructor {
                0 => vecs.push(SmallVec::<[u8; 64]>::new()),
                1 => vecs.push(SmallVec::with_capacity(_to_usize(g, op_index))),
                2 => {
                    let slice_start = _to_usize(g, op_index);
                    let slice_len = _to_usize(g, op_index+1) % 65;
                    if slice_start + slice_len <= g.len() {
                        vecs.push(SmallVec::from_slice(&g[slice_start..slice_start+slice_len]));
                    }
                }
                3 => {
                    let elem = _to_u8(g, op_index);
                    let count = _to_usize(g, op_index+1) % 65;
                    vecs.push(SmallVec::from_elem(elem, count));
                }
                4 => {
                    let cap = _to_usize(g, op_index);
                    let mut sv = SmallVec::<[u8; 64]>::with_capacity(cap);
                    for i in 0.._to_usize(g, op_index+1) % 65 {
                        sv.push(_to_u8(g, op_index+i));
                    }
                    vecs.push(sv);
                }
                _ => vecs.push(SmallVec::from_vec((0.._to_usize(g, op_index) % 65).map(|i| _to_u8(g, i)).collect())),
            }
        }

        for i in 0..vecs.len() {
            vecs[i].push(_to_u8(g, op_index));
            op_index += 1;

            if !vecs[i].is_empty() {
                vecs[i].truncate(_to_usize(g, op_index));
                let current_len = vecs[i].len();
                let insert_pos = _to_usize(g, op_index+1) % (current_len + 1);
                let elem = _to_u8(g, op_index+2);
                vecs[i].insert(insert_pos, elem);
                println!("{:?}", vecs[i].as_slice());
            }
        }

        let mut operations = vec![];
        for _ in 0..5 {
            if op_index + 3 >= g.len() { break; }
            operations.push((
                _to_usize(g, op_index) % vecs.len(),
                _to_usize(g, op_index+1),
                _to_usize(g, op_index+2),
                _to_usize(g, op_index+3)
            ));
            op_index += 4;
        }

        for op in operations {
            let vec_idx = op.0;
            if vec_idx >= vecs.len() { continue; }

            let other_slice = if op.1 % 7 == 2 {
                let other_idx = op.2 % vecs.len();
                vecs.get(other_idx).map(|v| v.as_slice().to_vec())
            } else {
                None
            };

            let sv = &mut vecs[vec_idx];
            match op.1 % 7 {
                0 => sv.extend_from_slice(&g[op.2 % g.len()..(op.2 + op.3) % g.len()]),
                1 => {
                    let new_cap = op.2;
                    let _ = sv.try_reserve(new_cap);
                }
                2 => {
                    if let Some(other) = &other_slice {
                        println!("Comparison: {:?}", sv.as_slice().cmp(other));
                    }
                }
                3 => {
                    let drain_range = op.2 % (sv.len() + 1)..op.3 % (sv.len() + 1);
                    let _ = sv.drain(drain_range);
                }
                4 => {
                    let new_len = op.2 % 65;
                    let value = _to_u8(g, op.3);
                    sv.resize(new_len, value);
                }
                5 => {
                    let retain_value = _to_u8(g, op.2);
                    sv.retain(|x| *x != retain_value);
                }
                _ => {
                    let cloned = sv.clone();
                    vecs.push(cloned);
                }
            }
        }

        if let Some(sv) = vecs.pop() {
            let cap = sv.capacity();
            sv.into_inner();
            let mut new_sv = SmallVec::<[u8; 64]>::with_capacity(_to_usize(g, op_index));
            new_sv.extend((0.._to_usize(g, op_index+1) % 65).map(|i| _to_u8(g, i)));
            if !new_sv.is_empty() {
                new_sv.drain(.._to_usize(g, op_index+2) % new_sv.len());
                println!("{:?}", new_sv.as_mut_slice());
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