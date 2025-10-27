#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

fn main (){
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 512 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let mut offset = 0;

        let constructor_choice = _to_u8(GLOBAL_DATA, offset);
        offset = offset.wrapping_add(1);
        
        let mut sv = match constructor_choice % 4 {
            0 => SmallVec::<[String; 32]>::new(),
            1 => {
                let capacity = _to_usize(GLOBAL_DATA, offset);
                offset = offset.wrapping_add(8);
                SmallVec::with_capacity(capacity)
            },
            2 => {
                let slices = _to_u8(GLOBAL_DATA, offset) % 8 + 1;
                offset = offset.wrapping_add(1);
                let mut items = vec![];
                for _ in 0..slices {
                    let len = _to_u8(GLOBAL_DATA, offset) as usize;
                    offset = offset.wrapping_add(1);
                    items.push(String::from(_to_str(GLOBAL_DATA, offset, offset + len)));
                    offset = offset.wrapping_add(len);
                }
                SmallVec::from_vec(items)
            },
            _ => {
                let elem_count = _to_usize(GLOBAL_DATA, offset);
                offset = offset.wrapping_add(8);
                SmallVec::from_elem(String::from("X"), elem_count)
            }
        };

        let op_count = _to_u8(GLOBAL_DATA, offset) % 16;
        offset = offset.wrapping_add(1);

        for _ in 0..op_count {
            let op_type = _to_u8(GLOBAL_DATA, offset);
            offset = offset.wrapping_add(1);

            match op_type % 11 {
                0 => {
                    let trunc_len = _to_usize(GLOBAL_DATA, offset) % (sv.len() + 1);
                    offset = offset.wrapping_add(8);
                    sv.truncate(trunc_len);
                },
                1 => {
                    let additional = _to_usize(GLOBAL_DATA, offset);
                    offset = offset.wrapping_add(8);
                    let _ = sv.try_reserve_exact(additional);
                },
                2 => {
                    if !sv.is_empty() {
                        let idx = _to_usize(GLOBAL_DATA, offset) % sv.len();
                        offset = offset.wrapping_add(8);
                        let _ = sv.swap_remove(idx);
                    }
                },
                3 => {
                    let drain_start = _to_usize(GLOBAL_DATA, offset) % (sv.len() + 1);
                    offset = offset.wrapping_add(8);
                    let drain_end = _to_usize(GLOBAL_DATA, offset) % (sv.len() + 1);
                    offset = offset.wrapping_add(8);
                    let _ = sv.drain(drain_start..drain_end);
                },
                4 => {
                    let new_len = _to_usize(GLOBAL_DATA, offset);
                    offset = offset.wrapping_add(8);
                    sv.resize_with(new_len, || String::new());
                },
                5 => {
                    let mut sv2 = SmallVec::<[String; 32]>::new();
                    let to_extract = _to_usize(GLOBAL_DATA, offset) % 10;
                    offset = offset.wrapping_add(8);
                    for _ in 0..to_extract {
                        let len = _to_u8(GLOBAL_DATA, offset) as usize;
                        offset = offset.wrapping_add(1);
                        sv2.push(String::from(_to_str(GLOBAL_DATA, offset, offset + len)));
                        offset = offset.wrapping_add(len);
                    }
                    sv.append(&mut sv2);
                },
                6 => {
                    let inserted = _to_usize(GLOBAL_DATA, offset) % (sv.len() + 1);
                    offset = offset.wrapping_add(8);
                    let len = _to_u8(GLOBAL_DATA, offset) as usize;
                    offset = offset.wrapping_add(1);
                    sv.insert(inserted, String::from(_to_str(GLOBAL_DATA, offset, offset + len)));
                    offset = offset.wrapping_add(len);
                },
                7 => {
                    let idx = _to_usize(GLOBAL_DATA, offset) % (sv.len() + 1);
                    offset = offset.wrapping_add(8);
                    let elem = String::from("fuzz");
                    sv.insert(idx, elem);
                },
                8 => {
                    let elem = String::from("temp");
                    sv.push(elem);
                },
                9 => {
                    let retain_prob = _to_u8(GLOBAL_DATA, offset) % 100;
                    offset = offset.wrapping_add(1);
                    sv.retain(|_| retain_prob < 50);
                },
                _ => {
                    let capacity = sv.capacity();
                    let insert_pos = _to_usize(GLOBAL_DATA, offset) % (sv.len() + 1);
                    offset = offset.wrapping_add(8);
                    let elem = String::from("insert_many");
                    sv.insert_many(insert_pos, [elem].iter().cloned());
                }
            }

            let slice = sv.as_slice();
            println!("Slice len: {}", slice.len());
            let mslice = sv.as_mut_slice();
            println!("Mut slice len: {}", mslice.len());

            if let Some(e) = sv.get(sv.len().wrapping_sub(1)) {
                println!("Last element: {:?}", e);
            }
            if let Some(e) = sv.pop() {
                sv.push(e);
            }
            let cap = sv.capacity();
            let _ = sv.try_reserve(cap.wrapping_add(1));
        }

        let comparison = SmallVec::from_elem(String::from("A"), sv.len());
        let _ = sv.partial_cmp(&comparison);
        let _ = sv.cmp(&comparison);
        let _ = sv.eq(&comparison);
        let _ = sv.as_ptr();
        let _ = sv.as_mut_ptr();
        let _ = SmallVec::<[String; 32]>::from_vec(vec![String::from("fromvec")]);
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