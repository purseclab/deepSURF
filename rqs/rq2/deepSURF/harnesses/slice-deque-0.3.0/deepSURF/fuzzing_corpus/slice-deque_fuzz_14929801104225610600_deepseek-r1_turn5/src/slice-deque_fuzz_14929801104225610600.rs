#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use slice_deque::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut};

#[derive(PartialEq, PartialOrd)]
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
        return t_5;
    }
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 2280 {return;}
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let mut deq = match _to_u8(GLOBAL_DATA, 0) % 4 {
            0 => SliceDeque::new(),
            1 => SliceDeque::with_capacity(_to_usize(GLOBAL_DATA, 1) % 65),
            2 => {
                let elem_count = _to_usize(GLOBAL_DATA, 2) % 32;
                let mut v = Vec::new();
                for i in 0..elem_count {
                    let offset = 3 + i*18;
                    let str_len = _to_u8(GLOBAL_DATA, offset) % 17;
                    let s = _to_str(GLOBAL_DATA, offset+1, offset+1+str_len as usize);
                    v.push(CustomType0(s.to_string()));
                }
                SliceDeque::from(&v[..])
            }
            _ => slice_deque::from_elem(
                CustomType0(_to_str(GLOBAL_DATA, 500, 527).to_string()),
                _to_usize(GLOBAL_DATA, 528) % 65
            )
        };

        let ops_count = (_to_u8(GLOBAL_DATA, 529) as usize) % 8;
        for op_idx in 0..ops_count {
            match _to_u8(GLOBAL_DATA, 530 + op_idx) % 7 {
                0 => {
                    let push_count = _to_usize(GLOBAL_DATA, 540 + op_idx*4) % 8;
                    for _ in 0..push_count {
                        let offset = 550 + op_idx*20;
                        let s = _to_str(GLOBAL_DATA, offset, offset + 16);
                        deq.push_back(CustomType0(s.to_string()));
                    }
                }
                1 => {
                    let trunc_len = _to_usize(GLOBAL_DATA, 700 + op_idx) % 65;
                    deq.truncate(trunc_len);
                }
                2 => {
                    let drain_start = _to_usize(GLOBAL_DATA, 710 + op_idx) % deq.len().max(1);
                    let drain_end = drain_start + _to_usize(GLOBAL_DATA, 715 + op_idx) % (deq.len().max(1) - drain_start);
                    let _ = deq.drain(drain_start..drain_end);
                }
                3 => {
                    let _ = deq.append(&mut slice_deque::from_elem(
                        CustomType0(_to_str(GLOBAL_DATA, 720, 735).to_string()),
                        _to_usize(GLOBAL_DATA, 736) % 8
                    ));
                }
                4 => {
                    let cap = _to_usize(GLOBAL_DATA, 740 + op_idx) % 128;
                    let _ = deq.try_reserve(cap);
                }
                5 => {
                    if let Some(front) = deq.front_mut() {
                        *front = CustomType0(_to_str(GLOBAL_DATA, 750 + op_idx*2, 760 + op_idx*2).to_string());
                        println!("{:?}", front.0);
                    }
                }
                _ => {
                    let back_idx = deq.len().checked_sub(1).unwrap_or(0);
                    if let Some(back) = deq.get_mut(back_idx) {
                        *back = CustomType0(_to_str(GLOBAL_DATA, 800 + op_idx*3, 803 + op_idx*3).to_string());
                    }
                }
            };

            let curr_len = deq.len();
            let _ = deq.partial_cmp(&deq.as_slice());
            println!("Operation {} length: {}", op_idx, curr_len);
        }

        let final_slices = deq.as_slices();
        println!("Final slices: {:?} {:?}", final_slices.0.len(), final_slices.1.len());
        let total_len = deq.len();
        println!("Final length: {}", total_len);
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