#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::ops::{Deref, DerefMut, Index, IndexMut};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct MyElem(u32);

fn take_index(len: usize, cursor: &mut usize, step: usize) -> usize {
    let idx = *cursor % len.saturating_sub(step);
    *cursor += step;
    idx
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 256 {
            return;
        }
        set_global_data(data);
        let global_data = get_global_data();
        let bytes = global_data.first_half;
        let len_bytes = bytes.len();

        let mut cursor = 0usize;

        let ctor_selector = _to_u8(bytes, take_index(len_bytes, &mut cursor, 1));
        let cap_raw = _to_usize(bytes, take_index(len_bytes, &mut cursor, 8));
        let elem_seed = _to_u32(bytes, take_index(len_bytes, &mut cursor, 4));
        let elem_template = MyElem(elem_seed);
        let n_elems = (_to_u8(bytes, take_index(len_bytes, &mut cursor, 1)) % 65) as usize;

        let mut tmp_vec: Vec<MyElem> = Vec::with_capacity(n_elems);
        for i in 0..n_elems {
            let seed = _to_u32(bytes, take_index(len_bytes, &mut cursor, 4)).wrapping_add(i as u32);
            tmp_vec.push(MyElem(seed));
        }

        let mut sv: SmallVec<[MyElem; 16]> = match ctor_selector % 4 {
            0 => SmallVec::new(),
            1 => SmallVec::with_capacity(cap_raw),
            2 => SmallVec::from_elem(elem_template, n_elems),
            _ => SmallVec::from_slice(&tmp_vec),
        };

        let op_count = (_to_u8(bytes, take_index(len_bytes, &mut cursor, 1)) % 20) as usize;
        for _ in 0..op_count {
            let op_code = _to_u8(bytes, take_index(len_bytes, &mut cursor, 1));
            match op_code % 10 {
                0 => {
                    let val = MyElem(_to_u32(bytes, take_index(len_bytes, &mut cursor, 4)));
                    sv.push(val);
                }
                1 => {
                    sv.pop();
                }
                2 => {
                    let additional = _to_usize(bytes, take_index(len_bytes, &mut cursor, 8));
                    sv.reserve(additional);
                }
                3 => {
                    let idx = _to_usize(bytes, take_index(len_bytes, &mut cursor, 8));
                    let elem = MyElem(_to_u32(bytes, take_index(len_bytes, &mut cursor, 4)));
                    sv.insert(idx, elem);
                }
                4 => {
                    if !sv.is_empty() {
                        let idx = _to_usize(bytes, take_index(len_bytes, &mut cursor, 8));
                        sv.remove(idx);
                    }
                }
                5 => {
                    let idx = _to_usize(bytes, take_index(len_bytes, &mut cursor, 8));
                    sv.truncate(idx);
                }
                6 => {
                    let idx = _to_usize(bytes, take_index(len_bytes, &mut cursor, 8));
                    sv.swap_remove(idx);
                }
                7 => {
                    let mut dr = sv.drain(..);
                    let item_opt = dr.next();
                    if let Some(ref it) = item_opt {
                        println!("{:?}", *it);
                    }
                }
                8 => {
                    sv.clear();
                }
                _ => {
                    sv.shrink_to_fit();
                }
            }
        }

        let slice_ref = sv.as_slice();
        if !slice_ref.is_empty() {
            let first_ref = &slice_ref[0];
            println!("{:?}", *first_ref);
        }

        let cmp_vec = SmallVec::<[MyElem; 16]>::from_vec(tmp_vec.clone());
        let _ = sv.partial_cmp(&cmp_vec);
        let _ = sv.cmp(&cmp_vec);

        let mut clone_vec = sv.clone();
        clone_vec.append(&mut cmp_vec.clone());

        let _ = sv.into_vec();
        let _ = clone_vec.into_boxed_slice();
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