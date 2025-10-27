#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use slice_deque::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

#[derive(Debug, Default, PartialEq)]
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
        let t_5 = CustomType0(t_4);
        t_5
    }
}

fn _custom_fn0(str0: &mut CustomType0) -> bool {
    let global_data = get_global_data();
    let GLOBAL_DATA = global_data.first_half;
    let custom_impl_num = _to_usize(GLOBAL_DATA, 570);
    let custom_impl_inst_num = str0.0.len();
    let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0 {
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector {
            1 => global_data.first_half,
            _ => global_data.second_half,
        };
        let t_137 = _to_bool(GLOBAL_DATA, 578);
        t_137
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 1158 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let constructor_selector = _to_u8(GLOBAL_DATA, 0) % 5;
        let mut t_135 = match constructor_selector {
            0 => {
                let cap = _to_usize(GLOBAL_DATA, 1) % 65;
                SliceDeque::with_capacity(cap)
            },
            1 => {
                let elem = CustomType0("base".to_string());
                let count = _to_usize(GLOBAL_DATA, 1) % 65;
                let v = vec![elem; count];
                SliceDeque::from_iter(v)
            },
            2 => {
                let mut v = Vec::new();
                for i in 0..(_to_usize(GLOBAL_DATA, 1) % 65) {
                    let len = _to_u8(GLOBAL_DATA, 2 + i * 2) % 17;
                    let s = _to_str(GLOBAL_DATA, 3 + i * 2, 3 + i * 2 + len as usize);
                    v.push(CustomType0(s.to_string()));
                }
                SliceDeque::from_iter(v)
            },
            3 => {
                let len = _to_usize(GLOBAL_DATA, 1) % 65;
                let mut buf = vec![CustomType0(String::new()); len];
                SliceDeque::from(&mut buf[..])
            },
            _ => SliceDeque::new(),
        };

        let ops_count = _to_u8(GLOBAL_DATA, 100) % 10;
        for op_idx in 0..ops_count {
            let op_byte = _to_u8(GLOBAL_DATA, 101 + op_idx as usize);
            match op_byte % 7 {
                0 => {
                    let len = _to_u8(GLOBAL_DATA, 200 + op_idx as usize) % 17;
                    let s = _to_str(GLOBAL_DATA, 300 + op_idx as usize * 2, 300 + op_idx as usize * 2 + len as usize);
                    t_135.push_back(CustomType0(s.to_string()));
                },
                1 => {
                    let _ = t_135.pop_front();
                },
                2 => {
                    let len = _to_usize(GLOBAL_DATA, 400 + op_idx as usize);
                    t_135.truncate(len);
                },
                3 => {
                    let _ = t_135.front_mut().map(|x| x.0.push('!'));
                },
                4 => {
                    let range = 0..t_135.len() / 2;
                    let replace = vec![CustomType0("splice".to_string()); _to_usize(GLOBAL_DATA, 500 + op_idx as usize) % 5];
                    let _ = t_135.splice(range, replace.into_iter());
                },
                5 => {
                    let new_len = _to_usize(GLOBAL_DATA, 600 + op_idx as usize);
                    t_135.resize_default(new_len);
                },
                _ => {
                    let mut filter = t_135.drain_filter(|x| x.0.len() % 2 == 0);
                    let _ = filter.next();
                },
            }

            let _ = t_135.as_slice();
            let _ = t_135.as_mut_slice();
            let _ = t_135.front().map(|x| println!("{:?}", x));
        }

        {
            let mut filter = t_135.drain_filter(_custom_fn0);
            for _ in 0..(_to_usize(GLOBAL_DATA, 900) % 5) {
                filter.next();
            }
        }

        let fmt_data = &mut CustomType0(String::new());
        let _ = t_135 == &[fmt_data.clone()];
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