#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 1024 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let num_ops = _to_usize(GLOBAL_DATA, 0) % 64;
        let mut vecs = smallvec::SmallVec::<[SmallVec<[u8; 128]>; 16]>::new();

        for i in 0..num_ops {
            let op_selector = _to_u8(GLOBAL_DATA, i * 2 + 1) % 11;
            let idx = _to_usize(GLOBAL_DATA, i * 4 + 8) % vecs.len().max(1);
            
            match op_selector {
                0 => {
                    let param = _to_usize(GLOBAL_DATA, i * 8 + 16);
                    let mut sv = SmallVec::new();
                    sv.reserve_exact(param);
                    vecs.push(sv);
                }
                1 => {
                    let capacity = _to_usize(GLOBAL_DATA, i * 8 + 24);
                    let mut sv = SmallVec::<[u8; 128]>::with_capacity(capacity);
                    sv.resize(_to_usize(GLOBAL_DATA, i * 8 + 32) % 128, 0);
                    vecs.push(sv);
                }
                2 => {
                    let slice_start = _to_usize(GLOBAL_DATA, i * 8 + 16) % 512;
                    let slice_len = _to_usize(GLOBAL_DATA, i * 8 + 24) % 128;
                    let slice = &GLOBAL_DATA[slice_start..slice_start + slice_len];
                    let sv = SmallVec::<[u8; 128]>::from_slice(slice);
                    vecs.push(sv);
                }
                3 => {
                    let elem = _to_u8(GLOBAL_DATA, i * 8 + 16);
                    let count = _to_usize(GLOBAL_DATA, i * 8 + 24) % 128;
                    let sv = SmallVec::from_elem(elem, count);
                    vecs.push(sv);
                }
                4 => if !vecs.is_empty() {
                    let val = _to_u8(GLOBAL_DATA, i * 8 + 16);
                    vecs[idx].push(val);
                }
                5 => if !vecs.is_empty() {
                    vecs[idx].pop();
                }
                6 => if !vecs.is_empty() {
                    let pos = _to_usize(GLOBAL_DATA, i * 8 + 16) % (vecs[idx].len() + 1);
                    let val = _to_u8(GLOBAL_DATA, i * 8 + 24);
                    vecs[idx].insert(pos, val);
                }
                7 => if !vecs.is_empty() {
                    let amount = _to_usize(GLOBAL_DATA, i * 8 + 16);
                    vecs[idx].reserve_exact(amount);
                }
                8 => if !vecs.is_empty() {
                    let new_len = _to_usize(GLOBAL_DATA, i * 8 + 16) % 256;
                    vecs[idx].truncate(new_len);
                }
                9 => if !vecs.is_empty() {
                    let drain_range = 0.._to_usize(GLOBAL_DATA, i * 8 + 16) % vecs[idx].len();
                    let _ = vecs[idx].drain(drain_range);
                }
                10 => if !vecs.is_empty() {
                    let other_idx = _to_usize(GLOBAL_DATA, i * 8 + 16) % vecs.len();
                    if idx != other_idx {
                        let a = idx.min(other_idx);
                        let b = idx.max(other_idx);
                        let (left, right) = vecs.split_at_mut(b);
                        if let (Some(vec_a), Some(vec_b)) = (left.get_mut(a), right.get_mut(0)) {
                            vec_a.append(vec_b);
                        }
                    }
                }
                _ => {}
            }

            if !vecs.is_empty() {
                let vec_len = vecs.len();
                let idx = _to_usize(GLOBAL_DATA, i * 8 + 32) % vec_len;
                let _ = vecs[idx].as_mut_slice().first_mut().map(|v| *v = v.wrapping_add(1));
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