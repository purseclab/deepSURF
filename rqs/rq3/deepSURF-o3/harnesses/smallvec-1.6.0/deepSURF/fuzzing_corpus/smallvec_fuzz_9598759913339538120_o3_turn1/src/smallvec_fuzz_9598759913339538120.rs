#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 120 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let constructor_choice = _to_u8(GLOBAL_DATA, 0) % 7;

        let mut buf = [0u8; 32];
        for i in 0..32 {
            buf[i] = _to_u8(GLOBAL_DATA, 1 + i);
        }

        let slice_len_raw = _to_u8(GLOBAL_DATA, 33) as usize;
        let slice_available = GLOBAL_DATA.len() - 34;
        let slice_len = if slice_available == 0 { 0 } else { slice_len_raw % slice_available.min(65) };
        let slice_start = 34;
        let slice_end = slice_start + slice_len;
        let slice = &GLOBAL_DATA[slice_start..slice_end];

        let elem_for_from_elem = _to_u8(GLOBAL_DATA, 34);
        let n_for_from_elem = (_to_u8(GLOBAL_DATA, 35) as usize % 65) + 1;

        let cap = _to_usize(GLOBAL_DATA, 36);
        let additional_reserve = _to_usize(GLOBAL_DATA, 44);

        let vec_len = (GLOBAL_DATA.len() - 40).min(65);
        let vec_for_from_vec: Vec<u8> = GLOBAL_DATA[40..40 + vec_len].to_vec();

        let len_for_buf_and_len = (_to_u8(GLOBAL_DATA, 60) as usize) % 32;

        let mut small_vec: SmallVec<[u8; 32]> = match constructor_choice {
            0 => SmallVec::<[u8; 32]>::new(),
            1 => SmallVec::<[u8; 32]>::with_capacity(cap),
            2 => SmallVec::<[u8; 32]>::from_slice(slice),
            3 => SmallVec::<[u8; 32]>::from_vec(vec_for_from_vec.clone()),
            4 => SmallVec::<[u8; 32]>::from_elem(elem_for_from_elem, n_for_from_elem),
            5 => SmallVec::<[u8; 32]>::from_buf(buf),
            _ => SmallVec::<[u8; 32]>::from_buf_and_len(buf, len_for_buf_and_len),
        };

        small_vec.reserve(additional_reserve);

        let pre_slice = small_vec.as_slice();
        if !pre_slice.is_empty() {
            println!("{:?}", pre_slice[0]);
        }

        let new_cap = _to_usize(GLOBAL_DATA, 52);
        let _ = small_vec.try_grow(new_cap);

        if !small_vec.is_empty() {
            let remove_idx = (elem_for_from_elem as usize) % small_vec.len();
            let _ = small_vec.remove(remove_idx);
        }

        let truncate_len = (_to_usize(GLOBAL_DATA, 58) % 65) as usize;
        small_vec.truncate(truncate_len);

        small_vec.push(_to_u8(GLOBAL_DATA, 59));

        let cloned_vec = small_vec.clone();
        let _ = SmallVec::<[u8; 32]>::cmp(&small_vec, &cloned_vec);

        let _ = small_vec.try_grow(_to_usize(GLOBAL_DATA, 60));

        let after_slice = small_vec.as_slice();
        if !after_slice.is_empty() {
            println!("{:?}", after_slice[after_slice.len() - 1]);
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