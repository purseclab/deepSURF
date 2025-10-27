#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};
use std::cmp::PartialOrd;

#[derive(Debug, Copy, PartialEq, PartialOrd)]
struct CustomType1(usize);

impl Clone for CustomType1 {
    fn clone(&self) -> Self {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 555);
        let custom_impl_inst_num = self.0;
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0 {
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector {
            1 => global_data.first_half,
            _ => global_data.second_half,
        };
        let t_136 = _to_usize(GLOBAL_DATA, 563);
        CustomType1(t_136)
    }
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 4562 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let num_ops = _to_u8(GLOBAL_DATA, 0) % 5 + 3;
        let mut vec_pool = Vec::with_capacity(5);

        for i in 0..num_ops {
            let selector = _to_u8(GLOBAL_DATA, i as usize * 3) % 6;
            let elem_count = _to_usize(GLOBAL_DATA, i as usize * 5 + 1) % 65;
            let mut base_offset = i as usize * 50;

            let mut sv = match selector {
                0 => {
                    let mut v = Vec::with_capacity(elem_count);
                    for j in 0..elem_count {
                        let val = _to_usize(GLOBAL_DATA, base_offset + j);
                        v.push(CustomType1(val));
                    }
                    SmallVec::<[CustomType1; 32]>::from_vec(v)
                }
                1 => {
                    let elements: Vec<CustomType1> = (0..elem_count)
                        .map(|j| CustomType1(_to_usize(GLOBAL_DATA, base_offset + j * 2)))
                        .collect();
                    SmallVec::from_slice(&elements)
                }
                2 => {
                    let mut sv = SmallVec::<[CustomType1; 32]>::with_capacity(elem_count);
                    for j in 0..elem_count {
                        sv.push(CustomType1(_to_usize(GLOBAL_DATA, base_offset + j * 3)));
                    }
                    sv
                }
                3 => {
                    let elements: Vec<CustomType1> = (0..elem_count)
                        .map(|j| CustomType1(_to_usize(GLOBAL_DATA, base_offset + j * 4)))
                        .collect();
                    SmallVec::from_iter(elements.into_iter())
                }
                _ => {
                    let mut sv = SmallVec::<[CustomType1; 32]>::new();
                    let cap = _to_usize(GLOBAL_DATA, base_offset);
                    sv.reserve(cap);
                    sv
                }
            };

            let trunc_len = _to_usize(GLOBAL_DATA, base_offset + 10) % 65;
            let reserve_amt = _to_usize(GLOBAL_DATA, base_offset + 15);
            let insert_idx = _to_usize(GLOBAL_DATA, base_offset + 20) % (sv.len() + 1);
            let insert_val = CustomType1(_to_usize(GLOBAL_DATA, base_offset + 25));

            sv.truncate(trunc_len);
            println!("Truncated: {:?}", sv.as_slice());
            sv.reserve(reserve_amt);
            let _ = sv.pop();
            sv.insert(insert_idx, insert_val);

            if sv.len() > 5 {
                sv.swap_remove(3);
            }

            sv.drain(2..4);

            vec_pool.push(sv);
        }

        for i in 0..vec_pool.len() / 2 {
            let (first, second) = vec_pool.split_at_mut(i + 1);
            let a = &first[i];
            let b = &mut second[0];
            
            let cmp_result = a.partial_cmp(b);
            println!("Comparison: {:?}", cmp_result);

            let slice_a = a.as_slice();
            let slice_b = b.as_mut_slice();
            println!("Slices: {:?} vs {:?}", slice_a, slice_b);

            let mut cloned = a.clone();
            let extend_elements = &b.as_slice()[..2];
            cloned.extend_from_slice(extend_elements);
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