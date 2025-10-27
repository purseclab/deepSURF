#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::ops::Deref;
use std::str::FromStr;

#[derive(Debug, PartialEq, PartialOrd)]
struct CustomType1(String);

impl core::clone::Clone for CustomType1 {
    fn clone(&self) -> Self {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 19);
        let custom_impl_inst_num = self.0.len();
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0 {
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector {
            1 => global_data.first_half,
            _ => global_data.second_half,
        };
        let mut t_6 = _to_u8(GLOBAL_DATA, 27) % 17;
        let t_7 = _to_str(GLOBAL_DATA, 28, 28 + t_6 as usize);
        let t_8 = String::from(t_7);
        let t_9 = CustomType1(t_8);
        t_9
    }
}

type SmallArr32 = [CustomType1; 32];
type SmallArr4 = [CustomType1; 4];

fn build_elem(data: &[u8], idx: usize) -> CustomType1 {
    let len_byte = _to_u8(data, idx % data.len()) % 17;
    let start = (idx + 1) % (data.len().saturating_sub(len_byte as usize + 1));
    let end = start + len_byte as usize;
    let s = _to_str(data, start, end);
    CustomType1(String::from(s))
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 1500 {
            return;
        }
        set_global_data(data);
        let global_data = get_global_data();
        let first = global_data.first_half;
        let second = global_data.second_half;

        let init_len = (_to_u8(first, 0) % 65) as usize;
        let mut tmp_vec = Vec::with_capacity(init_len);
        for i in 0..init_len {
            tmp_vec.push(build_elem(first, 10 + i * 20));
        }

        let selector = _to_u8(first, 1) % 5;
        let mut sv1: SmallVec<SmallArr32> = match selector {
            0 => SmallVec::from_vec(tmp_vec.clone()),
            1 => SmallVec::from_iter(tmp_vec.clone().into_iter()),
            2 => {
                if tmp_vec.is_empty() {
                    SmallVec::new()
                } else {
                    SmallVec::from_elem(tmp_vec[0].clone(), tmp_vec.len())
                }
            }
            3 => SmallVec::from_iter(tmp_vec.clone().into_iter()),
            _ => {
                let mut v = SmallVec::with_capacity(tmp_vec.len() + 1);
                v.extend(tmp_vec.clone());
                v
            }
        };

        let init_len2 = (_to_u8(second, 0) % 33) as usize;
        let mut tmp_vec2 = Vec::with_capacity(init_len2);
        for i in 0..init_len2 {
            tmp_vec2.push(build_elem(second, 200 + i * 15));
        }

        let selector2 = _to_u8(second, 2) % 5;
        let mut sv2: SmallVec<SmallArr4> = match selector2 {
            0 => SmallVec::from_vec(tmp_vec2.clone()),
            1 => SmallVec::from_iter(tmp_vec2.clone().into_iter()),
            2 => {
                if tmp_vec2.is_empty() {
                    SmallVec::new()
                } else {
                    SmallVec::from_elem(tmp_vec2[0].clone(), tmp_vec2.len())
                }
            }
            3 => SmallVec::from_iter(tmp_vec2.clone().into_iter()),
            _ => {
                let mut v = SmallVec::with_capacity(tmp_vec2.len() + 1);
                v.extend(tmp_vec2.clone());
                v
            }
        };

        let op_cnt = _to_u8(second, 3);
        for i in 0..op_cnt {
            let op_byte = _to_u8(second, 4 + i as usize);
            let idx_base = 500 + (i as usize) * 8;
            let idx_read = idx_base % (second.len().saturating_sub(9));
            let num = _to_usize(second, idx_read);
            match op_byte % 10 {
                0 => sv1.insert(num, build_elem(first, idx_base)),
                1 => {
                    sv1.truncate(num);
                }
                2 => {
                    if !sv1.is_empty() {
                        sv1.remove(num);
                    }
                }
                3 => sv1.resize(num, build_elem(first, idx_base + 1)),
                4 => {
                    sv1.swap_remove(num);
                }
                5 => sv1.reserve(num),
                6 => sv1.append(&mut sv2),
                7 => sv2.append(&mut sv1),
                8 => sv1.extend(sv2.clone().into_iter()),
                _ => {
                    let tmp: SmallVec<SmallArr32> = SmallVec::from_iter(sv2.clone().into_iter());
                    let _ = sv1.partial_cmp(&tmp);
                }
            }
        }

        println!("{:?}", sv1.as_slice().deref());
        println!("{:?}", sv2.as_slice().deref());

        let _cap1 = sv1.capacity();
        let _len1 = sv1.len();
        let _ = sv1.is_empty();

        let _drain = sv1.drain(0..(_to_usize(first, 100) % (sv1.len() + 1)));
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