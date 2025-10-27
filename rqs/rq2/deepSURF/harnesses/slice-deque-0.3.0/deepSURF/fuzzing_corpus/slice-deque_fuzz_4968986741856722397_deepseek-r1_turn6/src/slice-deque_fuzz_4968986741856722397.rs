#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use slice_deque::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

#[derive(Debug)]
struct CustomType1(String);
#[derive(Debug)]
struct CustomType2(String);
#[derive(Debug, Clone, PartialEq)]
struct CustomType0(String);

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
        (t_0, t_2)
    }
    
    fn next(&mut self) -> Option<Self::Item> {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 24);
        let custom_impl_inst_num = self.0.len();
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        let GLOBAL_DATA = match selector {
            1 => global_data.first_half,
            _ => global_data.second_half,
        };
        let mut t_4 = _to_u8(GLOBAL_DATA, 32) % 17;
        let t_5 = _to_str(GLOBAL_DATA, 33, 33 + t_4 as usize);
        let t_6 = String::from(t_5);
        let t_7 = CustomType0(t_6);
        Some(t_7)
    }
}

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
        CustomType2(t_11)
    }
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 2048 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        
        let op_count = _to_usize(GLOBAL_DATA, 0) % 8;
        let mut deques = Vec::with_capacity(4);
        
        let mut base_deque = match _to_u8(GLOBAL_DATA, 8) % 4 {
            0 => SliceDeque::new(),
            1 => SliceDeque::with_capacity(_to_usize(GLOBAL_DATA, 16) % 65),
            2 => from_elem(CustomType0(String::new()), _to_usize(GLOBAL_DATA, 24) % 65),
            _ => {
                let s = _to_str(GLOBAL_DATA, 32, 32 + (_to_u8(GLOBAL_DATA, 40) % 17) as usize);
                SliceDeque::from_iter(CustomType1(s.to_string()))
            }
        };
        
        for i in 0..op_count {
            let op_byte = _to_u8(GLOBAL_DATA, 48 + i as usize);
            match op_byte % 7 {
                0 => {
                    let val = CustomType0(_to_str(GLOBAL_DATA, 64 + i*16, 64 + i*16 + 8).to_string());
                    base_deque.push_back(val);
                }
                1 => {
                    base_deque.pop_front();
                }
                2 => {
                    let len = _to_usize(GLOBAL_DATA, 128 + i*8) % base_deque.len().max(1);
                    base_deque.truncate(len);
                }
                3 => {
                    let slice = base_deque.as_slice();
                    let mid = slice.len() / 2;
                    let (left, right) = slice.split_at(mid);
                    println!("Split slices: {:?} | {:?}", left, right);
                }
                4 => {
                    let capacity = _to_usize(GLOBAL_DATA, 192 + i*8) % 65;
                    let _ = base_deque.try_reserve(capacity);
                }
                5 => {
                    let mut cloned = base_deque.clone();
                    cloned.append(&mut base_deque);
                    deques.push(cloned);
                }
                _ => {
                    let elem = CustomType0(_to_str(GLOBAL_DATA, 256 + i*32, 256 + i*32 + 16).to_string());
                    base_deque.push_front(elem);
                }
            }
        }
        
        let vec_data: Vec<CustomType0> = (0.._to_usize(GLOBAL_DATA, 384) % 65)
            .map(|i| {
                let start = 512 + i * 16;
                let len = _to_u8(GLOBAL_DATA, start) % 17;
                CustomType0(_to_str(GLOBAL_DATA, start + 1, start + 1 + len as usize).to_string())
            })
            .collect();
        
        let arr: [_; 12] = vec![CustomType0("test".to_string()); 12].try_into().unwrap();
        let slice_deque_from_slice = SliceDeque::from(vec_data.as_slice());
        
        let comparisons = [
            &arr as &[_],
            vec_data.as_slice(),
            slice_deque_from_slice.as_slice()
        ];
        
        for comp in &comparisons {
            let eq_result = base_deque.as_slice() == *comp;
            println!("Comparison result: {}", eq_result);
            
            if let Some(front) = base_deque.front() {
                println!("Front element: {:?}", *front);
            }
            if let Some(back) = base_deque.back_mut() {
                *back = CustomType0("modified".to_string());
            }
        }
        
        {
            let mut drain_filter = base_deque.drain_filter(|x| x.0.len() % 2 == 0);
            while let Some(item) = drain_filter.next() {
                println!("Drained item: {:?}", item);
            }
        }
        
        let mut mirror = SliceDeque::from_iter(base_deque.into_iter());
        mirror.extend_from_slice(&vec_data);
        println!("Mirror len: {}, capacity: {}", mirror.len(), mirror.capacity());
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