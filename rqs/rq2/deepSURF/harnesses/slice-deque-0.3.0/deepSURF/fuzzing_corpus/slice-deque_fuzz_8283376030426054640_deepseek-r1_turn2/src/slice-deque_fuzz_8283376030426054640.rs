#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use slice_deque::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut};

#[derive(Debug)]
struct CustomType0(String);

impl std::clone::Clone for CustomType0 {
    fn clone(&self) -> Self {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 1);
        let custom_impl_inst_num = self.0.len();
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0 {
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector {
            1 => global_data.first_half,
            _ => global_data.second_half,
        };
        let mut t_2 = _to_u8(GLOBAL_DATA, 9) % 17;
        let t_3 = _to_str(GLOBAL_DATA, 10, 10 + t_2 as usize);
        let t_4 = String::from(t_3);
        CustomType0(t_4)
    }
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 4500 {
            return;
        }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let mut index = 2;
        let base_cap = _to_u8(GLOBAL_DATA, 0) % 65;
        let mut t_1 = Vec::with_capacity(base_cap as usize);

        for _ in 0..42 {
            let str_len = _to_u8(GLOBAL_DATA, index) % 17;
            index += 1;
            let s = _to_str(GLOBAL_DATA, index, index + str_len as usize);
            t_1.push(CustomType0(s.to_string()));
            index += str_len as usize;
        }

        let constructor_selector = _to_u8(GLOBAL_DATA, 1) % 3;
        let mut t_135 = match constructor_selector {
            0 => SliceDeque::from(&mut t_1[..]),
            1 => {
                let cap = _to_usize(GLOBAL_DATA, 700) % 65;
                let mut d = SliceDeque::with_capacity(cap);
                for elem in t_1 {
                    d.push_back(elem);
                }
                d
            }
            _ => SliceDeque::from_iter(t_1.into_iter()),
        };

        let op_count = _to_u8(GLOBAL_DATA, 299) % 15;
        let mut op_index = 300;
        for _ in 0..op_count {
            let op = _to_u8(GLOBAL_DATA, op_index) % 9;
            op_index += 1;

            match op {
                0 => {
                    let str_len = _to_u8(GLOBAL_DATA, op_index) % 17;
                    op_index += 1;
                    let s = _to_str(GLOBAL_DATA, op_index, op_index + str_len as usize);
                    t_135.push_front(CustomType0(s.to_string()));
                    op_index += str_len as usize;
                }
                1 => {
                    let trunc_len = _to_usize(GLOBAL_DATA, op_index);
                    t_135.truncate(trunc_len);
                    op_index += 8;
                }
                2 => {
                    t_135.pop_back();
                }
                3 => {
                    let idx = _to_usize(GLOBAL_DATA, op_index);
                    let _ = t_135.swap_remove_back(idx);
                    op_index += 8;
                }
                4 => {
                    let idx = _to_usize(GLOBAL_DATA, op_index);
                    if let Some(elem) = t_135.get_mut(idx) {
                        *elem = CustomType0("mutated".into());
                    }
                    op_index += 8;
                }
                5 => {
                    let slice = t_135.as_slice();
                    let _ = slice.first().map(|x| println!("{:?}", x));
                }
                6 => {
                    let _ = t_135.as_mut_slice();
                }
                7 => {
                    let mut d2 = SliceDeque::new();
                    d2.push_back(CustomType0("dummy".into()));
                    t_135.append(&mut d2);
                }
                8 => {
                    t_135.splice(0..0, Vec::new().into_iter());
                }
                _ => (),
            }
        }

        let mut t_136 = slice_deque::SliceDeque::into_iter(t_135);
        t_136.next_back();
        let _ = t_136.as_slice();
        for _ in 0..2 {
            t_136.next();
        }
        t_136.next_back();
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