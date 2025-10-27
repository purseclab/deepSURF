#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

type Arr = [u8; 32];

fn predicate0(_: &mut u8) -> bool {
    let global_data = get_global_data();
    let GLOBAL_DATA = global_data.first_half;
    let t_0 = _to_u8(GLOBAL_DATA, 40 % GLOBAL_DATA.len());
    if t_0 % 3 == 0 {
        panic!("INTENTIONAL PANIC!");
    }
    _to_bool(GLOBAL_DATA, 41 % GLOBAL_DATA.len())
}

fn predicate1(item: &mut u8) -> bool {
    let global_data = get_global_data();
    let GLOBAL_DATA = global_data.second_half;
    let idx = (*item as usize) % GLOBAL_DATA.len();
    let byte = _to_u8(GLOBAL_DATA, idx);
    byte % 2 == 0
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 128 {
            return;
        }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let GLOBAL_DATA2 = global_data.second_half;

        let choice = _to_u8(GLOBAL_DATA, 0 % GLOBAL_DATA.len());
        let cap = _to_usize(GLOBAL_DATA, 1 % (GLOBAL_DATA.len() - 8));
        let elem = _to_u8(GLOBAL_DATA, 9 % GLOBAL_DATA.len());

        let n_raw = _to_u8(GLOBAL_DATA, 10 % GLOBAL_DATA.len());
        let n = (n_raw % 65) as usize;

        let slice_start = 11usize;
        let slice_end = slice_start + n.min(GLOBAL_DATA.len() - slice_start);
        let slice = &GLOBAL_DATA[slice_start..slice_end];

        let mut vec_a: SmallVec<Arr> = match choice % 4 {
            0 => SmallVec::new(),
            1 => SmallVec::with_capacity(cap),
            2 => SmallVec::from_slice(slice),
            _ => SmallVec::from_elem(elem, n),
        };

        let buf: [u8; 32] = [0; 32];
        let mut vec_b: SmallVec<Arr> = SmallVec::from_buf_and_len(buf, 0);

        let op_total = (_to_u8(GLOBAL_DATA, 60 % GLOBAL_DATA.len()) % 8) as usize;
        for i in 0..op_total {
            let op_idx = i % GLOBAL_DATA2.len();
            let op_byte = _to_u8(GLOBAL_DATA2, op_idx);
            match op_byte % 8 {
                0 => {
                    let val = _to_u8(GLOBAL_DATA2, (op_idx + 1) % GLOBAL_DATA2.len());
                    vec_a.push(val);
                }
                1 => {
                    vec_a.pop();
                }
                2 => {
                    let idx = _to_usize(GLOBAL_DATA2, (op_idx + 2) % (GLOBAL_DATA2.len() - 8));
                    vec_a.swap_remove(idx);
                }
                3 => {
                    let idx = _to_usize(GLOBAL_DATA2, (op_idx + 3) % (GLOBAL_DATA2.len() - 8));
                    let val = _to_u8(GLOBAL_DATA2, (op_idx + 4) % GLOBAL_DATA2.len());
                    if idx < 65 {
                        vec_a.insert(idx, val);
                    }
                }
                4 => {
                    vec_a.extend_from_slice(slice);
                }
                5 => {
                    let new_len = _to_usize(GLOBAL_DATA2, (op_idx + 5) % (GLOBAL_DATA2.len() - 8)) % 65;
                    vec_a.truncate(new_len);
                }
                6 => {
                    let addl = _to_usize(GLOBAL_DATA2, (op_idx + 6) % (GLOBAL_DATA2.len() - 8));
                    vec_a.reserve(addl);
                }
                _ => {
                    vec_a.clear();
                }
            }
        }

        let mut vec_ref = &mut vec_a;
        if choice % 2 == 0 {
            vec_ref.retain(predicate0);
        } else {
            vec_ref.retain(predicate1);
        }

        let _len = vec_ref.len();
        let _capacity = vec_ref.capacity();

        let slice_ref = vec_ref.as_slice();
        if !slice_ref.is_empty() {
            println!("{:?}", slice_ref[0]);
        }

        vec_b.extend_from_slice(slice_ref);
        let _ = (*vec_ref).partial_cmp(&vec_b);

        let _ = vec_a.clone().into_vec();
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