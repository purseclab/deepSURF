#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::ops::{Deref, DerefMut, Index, IndexMut};
use std::str::FromStr;

#[derive(Debug)]
struct CustomType1(String);

impl core::clone::Clone for CustomType1 {
    fn clone(&self) -> Self {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 42);
        let custom_impl_inst_num = self.0.len();
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0 {
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector {
            1 => global_data.first_half,
            _ => global_data.second_half,
        };
        let mut t_10 = _to_u8(GLOBAL_DATA, 50) % 17;
        let t_11 = _to_str(GLOBAL_DATA, 51, 51 + t_10 as usize);
        let t_12 = String::from(t_11);
        let t_13 = CustomType1(t_12);
        t_13
    }
}

fn make_custom(slice: &[u8], idx: usize) -> CustomType1 {
    let len = _to_u8(slice, idx) % 17;
    let s = _to_str(slice, idx + 1, idx + 1 + len as usize);
    CustomType1(String::from(s))
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 200 {
            return;
        }
        set_global_data(data);
        let global_data = get_global_data();
        let first = global_data.first_half;
        let second = global_data.second_half;

        let base_elem = make_custom(first, 5);
        let alt_elem = make_custom(second, 23);

        let buf: [CustomType1; 16] = std::array::from_fn(|_| base_elem.clone());
        let mut len_for_buf = (_to_u8(first, 40) % 16) as usize;
        if len_for_buf == 0 {
            len_for_buf = 1;
        }

        let selector = _to_u8(first, 60) % 7;
        let mut sv: SmallVec<[CustomType1; 16]> = match selector {
            0 => SmallVec::new(),
            1 => {
                let cap = _to_usize(first, 61) % 65;
                SmallVec::with_capacity(cap)
            }
            2 => {
                let n = _to_usize(first, 67) % 65;
                SmallVec::from_elem(base_elem.clone(), n)
            }
            3 => {
                let vec_len = (_to_u8(first, 73) % 65) as usize;
                let mut v = Vec::new();
                for i in 0..vec_len {
                    v.push(make_custom(second, 80 + (i as usize) % 20));
                }
                SmallVec::from_vec(v)
            }
            4 => {
                let slice_len = (_to_u8(first, 90) % 16) as usize;
                let slice = &buf[0..slice_len];
                SmallVec::from(slice)
            }
            5 => SmallVec::from_buf_and_len(buf.clone(), len_for_buf),
            _ => {
                let iter_len = (_to_u8(first, 100) % 16) as usize;
                let iter = (0..iter_len).map(|i| make_custom(first, 104 + (i as usize) % 20));
                SmallVec::from_iter(iter)
            }
        };

        let push_elem = make_custom(first, 130);
        sv.push(push_elem.clone());

        let reserve_amt = _to_usize(first, 140);
        sv.reserve(reserve_amt);

        if !sv.is_empty() {
            let idx = _to_usize(first, 148) % sv.len();
            let item_ref = &sv[idx];
            println!("{:?}", item_ref);
        }

        let new_len = _to_usize(first, 152);
        sv.resize(new_len, alt_elem.clone());

        let cap = sv.capacity();
        println!("{:?}", cap);

        let slice_ref = sv.as_slice();
        println!("{:?}", slice_ref.len());

        if !sv.is_empty() {
            let rem_idx = _to_usize(first, 160) % sv.len();
            sv.remove(rem_idx);
        }

        sv.shrink_to_fit();

        if let Some(popped) = sv.pop() {
            println!("{:?}", popped);
        }

        let vec_copy = sv.clone().into_vec();
        let _sv_again: SmallVec<[CustomType1; 16]> = SmallVec::from_vec(vec_copy);
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