#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use slice_deque::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

#[derive(Debug)]
struct CustomType0(String);
#[derive(Debug)]
struct CustomType1(String);

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

impl std::cmp::PartialEq for CustomType1 {
    fn eq(&self, other: &Self) -> bool {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 570);
        let custom_impl_inst_num = self.0.len();
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
}

fn _custom_fn0(str0: &mut CustomType0) -> CustomType1 {
    let global_data = get_global_data();
    let GLOBAL_DATA = global_data.first_half;
    let custom_impl_num = _to_usize(GLOBAL_DATA, 579);
    let custom_impl_inst_num = str0.0.len();
    let selector = (custom_impl_num + custom_impl_inst_num) % 3;
    if selector == 0 {
        panic!("INTENTIONAL PANIC!");
    }
    let GLOBAL_DATA = match selector {
        1 => global_data.first_half,
        _ => global_data.second_half,
    };
    let mut t_138 = _to_u8(GLOBAL_DATA, 587) % 17;
    let t_139 = _to_str(GLOBAL_DATA, 588, 588 + t_138 as usize);
    let t_140 = String::from(t_139);
    CustomType1(t_140)
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 3000 {
            return;
        }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let mut op_index = 0;

        let constructor_selector = _to_u8(GLOBAL_DATA, op_index) % 3;
        op_index += 1;

        let mut deque = match constructor_selector {
            0 => SliceDeque::new(),
            1 => {
                let cap = _to_usize(GLOBAL_DATA, op_index) % 65;
                op_index += 8;
                SliceDeque::with_capacity(cap)
            }
            2 => {
                let elem_len = _to_u8(GLOBAL_DATA, op_index) as usize;
                op_index += 1;
                let elem_str = _to_str(GLOBAL_DATA, op_index, op_index + elem_len);
                op_index += elem_len;
                let count = _to_usize(GLOBAL_DATA, op_index) % 65;
                op_index += 8;
                from_elem(CustomType0(elem_str.to_string()), count)
            }
            _ => SliceDeque::new(),
        };

        let operation_count = _to_u8(GLOBAL_DATA, op_index) % 10;
        op_index += 1;

        for _ in 0..operation_count {
            let op_type = _to_u8(GLOBAL_DATA, op_index) % 6;
            op_index += 1;

            match op_type {
                0 => {
                    if GLOBAL_DATA.len() > op_index + 1 {
                        let len = _to_u8(GLOBAL_DATA, op_index) as usize;
                        let s = _to_str(GLOBAL_DATA, op_index + 1, op_index + 1 + len);
                        deque.push_back(CustomType0(s.to_string()));
                        op_index += 1 + len;
                    }
                }
                1 => {
                    if GLOBAL_DATA.len() > op_index + 1 {
                        let len = _to_u8(GLOBAL_DATA, op_index) as usize;
                        let s = _to_str(GLOBAL_DATA, op_index + 1, op_index + 1 + len);
                        deque.push_front(CustomType0(s.to_string()));
                        op_index += 1 + len;
                    }
                }
                2 => {
                    let trunc_len = _to_usize(GLOBAL_DATA, op_index);
                    op_index += 8;
                    deque.truncate(trunc_len);
                }
                3 => {
                    let drain_start = _to_usize(GLOBAL_DATA, op_index);
                    op_index += 8;
                    let drain_end = _to_usize(GLOBAL_DATA, op_index);
                    op_index += 8;
                    deque.drain(drain_start..drain_end);
                }
                4 => {
                    let idx = _to_usize(GLOBAL_DATA, op_index);
                    op_index += 8;
                    if GLOBAL_DATA.len() > op_index + 1 {
                        let len = _to_u8(GLOBAL_DATA, op_index) as usize;
                        let s = _to_str(GLOBAL_DATA, op_index + 1, op_index + 1 + len);
                        deque.insert(idx, CustomType0(s.to_string()));
                        op_index += 1 + len;
                    }
                }
                5 => {
                    deque.retain(|x| x.0.len() % 2 == 0);
                }
                _ => (),
            }
        }

        if let Some(front) = deque.front() {
            println!("{:?}", front);
        }
        if let Some(back) = deque.back_mut() {
            back.0.push('!');
        }

        deque.dedup_by_key(_custom_fn0);

        let _ = deque.as_slices();
        let _ = deque.as_mut_slice();
        let _ = deque.len();
        let _ = deque.capacity();

        if !deque.is_empty() {
            let idx = _to_usize(GLOBAL_DATA, op_index) % deque.len();
            let _ = deque.remove(idx);
        }

        let _ = SliceDeque::from_iter(vec![CustomType0("test".into())]);
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