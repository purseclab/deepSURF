#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

#[derive(Debug)]
struct CustomType1(String);

impl Clone for CustomType1 {
    fn clone(&self) -> Self {
        let global_data = get_global_data();
        let selector_seed = _to_usize(global_data.first_half, 9) + self.0.len();
        match selector_seed % 3 {
            0 => panic!("INTENTIONAL PANIC!"),
            1 => {
                let len = _to_u8(global_data.first_half, 17) % 17;
                let s = _to_str(global_data.first_half, 18, 18 + len as usize);
                CustomType1(s.to_string())
            }
            _ => {
                let len = _to_u8(global_data.second_half, 21) % 17;
                let s = _to_str(global_data.second_half, 22, 22 + len as usize);
                CustomType1(s.to_string())
            }
        }
    }
}

fn gen_custom_type1(slice: &[u8], offset: usize) -> CustomType1 {
    let len = _to_u8(slice, offset) % 17;
    let s = _to_str(slice, offset + 1, offset + 1 + len as usize);
    CustomType1(s.to_string())
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 256 {
            return;
        }
        set_global_data(data);
        let global = get_global_data();
        let first = global.first_half;
        let second = global.second_half;

        let init_elem = gen_custom_type1(first, 3);
        let n_init = _to_usize(first, 20) % 65;
        let mut sv = SmallVec::<[CustomType1; 16]>::from_elem(init_elem.clone(), n_init);

        let ops = (_to_u8(first, 27) % 20) as usize;
        for i in 0..ops {
            let selector = _to_u8(data, i) % 8;
            match selector {
                0 => {
                    let elem = if i % 2 == 0 {
                        gen_custom_type1(first, (30 + i) % (first.len() - 18))
                    } else {
                        gen_custom_type1(second, (40 + i) % (second.len() - 18))
                    };
                    sv.push(elem);
                }
                1 => {
                    sv.pop();
                }
                2 => {
                    if !sv.is_empty() {
                        let idx_src = i % (data.len() - 8);
                        let idx = _to_usize(data, idx_src) % sv.len();
                        sv.remove(idx);
                    }
                }
                3 => {
                    sv.clear();
                }
                4 => {
                    if sv.len() < 65 {
                        let elem = gen_custom_type1(first, (50 + i) % (first.len() - 18));
                        let pos = _to_usize(first, 60) % (sv.len() + 1);
                        sv.insert(pos, elem);
                    }
                }
                5 => {
                    let new_len = _to_usize(first, 70) % 65;
                    let filler = gen_custom_type1(second, (80 + i) % (second.len() - 18));
                    sv.resize(new_len, filler);
                }
                6 => {
                    let mut other =
                        SmallVec::<[CustomType1; 16]>::with_capacity(_to_usize(second, 90));
                    other.extend(sv.iter().cloned());
                    sv.append(&mut other);
                }
                7 => {
                    let start = 0;
                    let end = std::cmp::min(sv.len(), 2);
                    let mut drain_iter = sv.drain(start..end);
                    let _ = drain_iter.next();
                    let _ = drain_iter.next_back();
                }
                _ => {}
            }
        }

        let elem2 = gen_custom_type1(second, 100);
        let n2 = _to_usize(second, 110) % 65;
        let sv2 = SmallVec::<[CustomType1; 16]>::from_elem(elem2.clone(), n2);

        let slice_ref = sv.as_slice();
        if !slice_ref.is_empty() {
            println!("{:?}", slice_ref[0]);
        }

        let _vec_out = sv.clone().into_vec();
        let _box_out = sv2.clone().into_boxed_slice();

        let fixed_array = [elem2.clone(), elem2.clone(), elem2.clone(), elem2];
        let vec_from_array: Vec<CustomType1> = fixed_array.iter().cloned().collect();
        let _sv3 = SmallVec::<[CustomType1; 8]>::from_vec(vec_from_array);
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