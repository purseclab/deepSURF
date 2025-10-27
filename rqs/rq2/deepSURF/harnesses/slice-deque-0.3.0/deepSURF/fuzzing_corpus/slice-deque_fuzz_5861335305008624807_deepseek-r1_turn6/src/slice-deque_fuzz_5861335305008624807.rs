#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use slice_deque::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

#[derive(Debug, PartialEq, Clone)]
struct CustomType0(String);
struct CustomType1(String);
struct CustomType2(String);

impl std::iter::IntoIterator for CustomType1 {
    type Item = CustomType0;
    type IntoIter = CustomType2;
    
    fn into_iter(self) -> Self::IntoIter {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 49);
        let custom_impl_inst_num = self.0.len();
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0 {
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector {
            1 => global_data.first_half,
            _ => global_data.second_half,
        };
        let mut t_9 = _to_u8(GLOBAL_DATA, 57) % 17;
        let t_10 = _to_str(GLOBAL_DATA, 58, 58 + t_9 as usize);
        let t_11 = String::from(t_10);
        let t_12 = CustomType2(t_11);
        return t_12;
    }
}

impl std::iter::Iterator for CustomType2 {
    type Item = CustomType0;
    
    fn size_hint(&self) -> (usize, Option<usize>) {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 0);
        let custom_impl_inst_num = self.0.len();
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0 {
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector {
            1 => global_data.first_half,
            _ => global_data.second_half,
        };
        let t_0 = _to_usize(GLOBAL_DATA, 8);
        let t_1 = _to_usize(GLOBAL_DATA, 16);
        let t_2 = Some(t_1);
        let t_3 = (t_0, t_2);
        return t_3;
    }
    
    fn next(&mut self) -> Option<Self::Item> {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 24);
        let custom_impl_inst_num = self.0.len();
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0 {
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector {
            1 => global_data.first_half,
            _ => global_data.second_half,
        };
        let mut t_4 = _to_u8(GLOBAL_DATA, 32) % 17;
        let t_5 = _to_str(GLOBAL_DATA, 33, 33 + t_4 as usize);
        let t_6 = String::from(t_5);
        let t_7 = CustomType0(t_6);
        let t_8 = Some(t_7);
        return t_8;
    }
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 2200 {return;}
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let op_count = _to_usize(GLOBAL_DATA, 0) % 7;
        let mut deques = Vec::with_capacity(3);
        
        let constructor_idx = _to_u8(GLOBAL_DATA, 8) % 3;
        match constructor_idx {
            0 => deques.push(SliceDeque::new()),
            1 => deques.push(SliceDeque::with_capacity(_to_usize(GLOBAL_DATA, 16) % 65)),
            _ => {
                let ct1 = CustomType1(String::from(_to_str(GLOBAL_DATA, 24, 24 + (_to_u8(GLOBAL_DATA, 32) % 17) as usize)));
                deques.push(SliceDeque::from_iter(ct1));
            }
        }

        for op_idx in 0..op_count {
            let op_selector = _to_u8(GLOBAL_DATA, 40 + op_idx as usize) % 6;
            match op_selector {
                0 => {
                    let value = CustomType0(String::from(_to_str(GLOBAL_DATA, 48 + op_idx * 24, 48 + op_idx * 24 + (_to_u8(GLOBAL_DATA, 49 + op_idx * 24) % 17) as usize)));
                    deques[0].push_back(value);
                }
                1 => {
                    let _ = deques[0].pop_front().map(|x| println!("{:?}", x.0));
                }
                2 => {
                    deques[0].truncate(_to_usize(GLOBAL_DATA, 160 + op_idx * 8) % 65);
                }
                3 => {
                    let ct1 = CustomType1(String::from(_to_str(GLOBAL_DATA, 200 + op_idx * 32, 200 + op_idx * 32 + (_to_u8(GLOBAL_DATA, 201 + op_idx * 32) % 17) as usize)));
                    deques[0].extend(ct1);
                }
                4 => {
                    let drain_range = _to_usize(GLOBAL_DATA, 300 + op_idx * 16) % (deques[0].len() + 1);
                    let _d = deques[0].drain(drain_range..);
                }
                5 => {
                    deques.push(slice_deque::from_elem(CustomType0(String::new()), _to_usize(GLOBAL_DATA, 400 + op_idx * 8) % 65));
                }
                _ => {}
            }
        }

        let deque_len = deques.len();
        let target_deque = if _to_u8(GLOBAL_DATA, 600) % 2 == 0 {
            &mut deques[0]
        } else {
            let last_idx = deque_len.checked_sub(1).unwrap_or(0);
            &mut deques[last_idx]
        };

        let mut comparison_vec = Vec::new();
        let vec_len = _to_usize(GLOBAL_DATA, 608) % 65;
        for i in 0..vec_len {
            let s_len = _to_u8(GLOBAL_DATA, 616 + i * 24) % 17;
            let s = _to_str(GLOBAL_DATA, 617 + i * 24, 617 + i * 24 + s_len as usize);
            comparison_vec.push(CustomType0(String::from(s)));
        }
        
        let _ = target_deque.front().map(|x| println!("{:?}", x.0));
        let _ = target_deque.back_mut().map(|x| *x = CustomType0(String::new()));
        
        let comparison_array: [_; 30] = comparison_vec.clone().try_into().unwrap();
        let _ = (&*target_deque).eq(&comparison_array);
        let _ = deques[0].as_mut_slices().0.get_mut(0).map(|v| *v = CustomType0(String::new()));
        let _ = deques.last_mut().map(|d| d.splice(0..0, comparison_vec));
        if let Some(s) = deques[0].as_slices().1.first() {
            println!("{:?}", s.0);
        }
        for item in &deques[0] {
            println!("{:?}", item.0);
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