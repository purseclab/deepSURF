#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use slice_deque::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut};

#[derive(Debug)]
struct CustomType0(String);

impl Clone for CustomType0 {
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

fn _custom_fn0(str0: &CustomType0) -> bool {
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
    _to_bool(GLOBAL_DATA, 578)
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 5000 {
            return;
        }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let mut t_1 = Vec::new();
        let elem_count = _to_usize(GLOBAL_DATA, 0) % 65;
        for i in 0..elem_count {
            let offset = 1 + i * 3;
            let str_len = _to_u8(GLOBAL_DATA, offset) % 17;
            let start = offset + 1;
            let end = start + str_len as usize;
            let s = _to_str(GLOBAL_DATA, start, end);
            t_1.push(CustomType0(s.to_string()));
        }

        let constructor_sel = _to_u8(GLOBAL_DATA, 200) % 3;
        let mut deque = match constructor_sel {
            0 => slice_deque::SliceDeque::from(&mut t_1[..]),
            1 => slice_deque::SliceDeque::from_iter(t_1.into_iter()),
            2 => {
                let cap = _to_usize(GLOBAL_DATA, 201) % 65;
                let mut sd = slice_deque::SliceDeque::with_capacity(cap);
                sd.extend(t_1);
                sd
            }
            _ => unreachable!(),
        };

        let ops_before = _to_usize(GLOBAL_DATA, 202) % 20;
        for i in 0..ops_before {
            let op_sel = _to_u8(GLOBAL_DATA, 203 + i as usize) % 6;
            match op_sel {
                0 => {
                    let elem = CustomType0(format!("op{}_front", i));
                    deque.push_front(elem);
                }
                1 => {
                    let elem = CustomType0(format!("op{}_back", i));
                    deque.push_back(elem);
                }
                2 => {
                    if let Some(e) = deque.pop_front() {
                        println!("{:?}", e);
                    }
                }
                3 => {
                    deque.truncate(_to_usize(GLOBAL_DATA, 300 + i as usize) % (deque.len() + 1));
                }
                4 => {
                    let drain_range = 0.._to_usize(GLOBAL_DATA, 400 + i as usize) % (deque.len() + 1);
                    let _ = deque.drain(drain_range);
                }
                5 => {
                    if let Some(e) = deque.back_mut() {
                        *e = CustomType0("modified".into());
                    }
                }
                _ => {}
            }
        }

        if let Some(front) = deque.front() {
            println!("Front: {:?}", front);
        }
        if let Some(back) = deque.back() {
            println!("Back: {:?}", back);
        }

        deque.retain(_custom_fn0);

        let ops_after = _to_usize(GLOBAL_DATA, 500) % 20;
        for i in 0..ops_after {
            let op_sel = _to_u8(GLOBAL_DATA, 501 + i as usize) % 5;
            match op_sel {
                0 => {
                    let (s1, s2) = deque.as_slices();
                    println!("Slices: {} + {}", s1.len(), s2.len());
                }
                1 => {
                    let mut splicer = deque.splice(0..0, vec![
                        CustomType0("splice1".into()),
                        CustomType0("splice2".into())
                    ]);
                    let _ = splicer.next();
                }
                2 => {
                    let mut filter = deque.drain_filter(|x| x.0.len() % 2 == 0);
                    let _ = filter.next();
                }
                3 => {
                    let _ = deque.as_mut_slices();
                }
                4 => {
                    let mut iter = deque.into_iter();
                    let _ = iter.next();
                    deque = iter.collect();
                }
                _ => {}
            }
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