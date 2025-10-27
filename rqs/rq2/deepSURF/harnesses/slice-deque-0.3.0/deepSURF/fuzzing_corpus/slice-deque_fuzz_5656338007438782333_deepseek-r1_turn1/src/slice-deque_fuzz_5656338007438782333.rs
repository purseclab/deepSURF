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
struct CustomType2(String);
#[derive(Debug)]
struct CustomType1(String);

impl std::iter::Iterator for CustomType2 {
    type Item = CustomType0;
    
    fn size_hint(&self) -> (usize, Option<usize>) {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 570);
        let custom_impl_inst_num = self.0.len();
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0{
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector{
                    1 => global_data.first_half,
                    _ => global_data.second_half,
        };
        let t_137 = _to_usize(GLOBAL_DATA, 578);
        let t_138 = _to_usize(GLOBAL_DATA, 586);
        let t_139 = Some(t_138);
        let t_140 = (t_137, t_139);
        return t_140;
    }
    
    fn next(&mut self) -> Option<Self::Item> {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 594);
        let custom_impl_inst_num = self.0.len();
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0{
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector{
                    1 => global_data.first_half,
                    _ => global_data.second_half,
        };
        let mut t_141 = _to_u8(GLOBAL_DATA, 602) % 17;
        let t_142 = _to_str(GLOBAL_DATA, 603, 603 + t_141 as usize);
        let t_143 = String::from(t_142);
        let t_144 = CustomType0(t_143);
        let t_145 = Some(t_144);
        return t_145;
    }
}

impl std::clone::Clone for CustomType0 {
    
    fn clone(&self) -> Self {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 1);
        let custom_impl_inst_num = self.0.len();
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0{
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector{
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

impl std::iter::IntoIterator for CustomType1 {
    type Item = CustomType0;
    type IntoIter = CustomType2;
    
    fn into_iter(self) -> Self::IntoIter {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 619);
        let custom_impl_inst_num = self.0.len();
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0{
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector{
                    1 => global_data.first_half,
                    _ => global_data.second_half,
        };
        let mut t_146 = _to_u8(GLOBAL_DATA, 627) % 17;
        let t_147 = _to_str(GLOBAL_DATA, 628, 628 + t_146 as usize);
        let t_148 = String::from(t_147);
        let t_149 = CustomType2(t_148);
        return t_149;
    }
}

fn main (){
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 5000 {return;}
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let mut index = 0;

        let constructor_selector = _to_u8(GLOBAL_DATA, index) % 3;
        index += 1;
        let mut deque = match constructor_selector {
            0 => {
                let count = _to_u8(GLOBAL_DATA, index) % 65;
                index += 1;
                let mut vec = Vec::with_capacity(count as usize);
                for _ in 0..count {
                    let str_len = _to_u8(GLOBAL_DATA, index) % 17;
                    index += 1;
                    let str = _to_str(GLOBAL_DATA, index, index + str_len as usize);
                    index += str_len as usize;
                    vec.push(CustomType0(str.to_string()));
                }
                SliceDeque::from(&vec[..])
            }
            1 => {
                let capacity = _to_usize(GLOBAL_DATA, index);
                index += std::mem::size_of::<usize>();
                SliceDeque::with_capacity(capacity)
            }
            _ => SliceDeque::new()
        };

        let ops_before = _to_u8(GLOBAL_DATA, index) % 5;
        index += 1;
        for _ in 0..ops_before {
            let op = _to_u8(GLOBAL_DATA, index) % 5;
            index += 1;
            match op {
                0 => {
                    let str_len = _to_u8(GLOBAL_DATA, index) % 17;
                    index += 1;
                    let str = _to_str(GLOBAL_DATA, index, index + str_len as usize);
                    index += str_len as usize;
                    deque.push_front(CustomType0(str.to_string()));
                }
                1 => {
                    let _ = deque.pop_back();
                }
                2 => {
                    let len = _to_usize(GLOBAL_DATA, index);
                    index += std::mem::size_of::<usize>();
                    deque.truncate(len);
                }
                3 => {
                    let drain_len = _to_usize(GLOBAL_DATA, index) % (deque.len() + 1);
                    index += std::mem::size_of::<usize>();
                    if drain_len > 0 {
                        deque.drain(0..drain_len);
                    }
                }
                _ => println!("{:?}", deque.as_slice()),
            }
        }

        let extend_selector = _to_u8(GLOBAL_DATA, index) % 2;
        index += 1;
        match extend_selector {
            0 => {
                let str_len = _to_u8(GLOBAL_DATA, index) % 17;
                index += 1;
                let str = _to_str(GLOBAL_DATA, index, index + str_len as usize);
                index += str_len as usize;
                deque.extend(CustomType1(str.to_string()));
            }
            _ => {
                let vec_len = _to_u8(GLOBAL_DATA, index) % 65;
                index += 1;
                let mut vec = Vec::new();
                for _ in 0..vec_len {
                    let str_len = _to_u8(GLOBAL_DATA, index) % 17;
                    index += 1;
                    let str = _to_str(GLOBAL_DATA, index, index + str_len as usize);
                    index += str_len as usize;
                    vec.push(CustomType0(str.to_string()));
                }
                deque.extend(vec);
            }
        }

        let ops_after = _to_u8(GLOBAL_DATA, index) % 5;
        index += 1;
        for _ in 0..ops_after {
            let op = _to_u8(GLOBAL_DATA, index) % 5;
            index += 1;
            match op {
                0 => {
                    let str_len = _to_u8(GLOBAL_DATA, index) % 17;
                    index += 1;
                    let str = _to_str(GLOBAL_DATA, index, index + str_len as usize);
                    index += str_len as usize;
                    deque.push_back(CustomType0(str.to_string()));
                }
                1 => {
                    let _ = deque.pop_front();
                }
                2 => {
                    let slice = deque.as_slice();
                    println!("Slice len: {}", slice.len());
                }
                3 => {
                    let additional = _to_usize(GLOBAL_DATA, index);
                    index += std::mem::size_of::<usize>();
                    deque.reserve(additional);
                }
                _ => println!("Length: {}, Capacity: {}", deque.len(), deque.capacity()),
            }
        }

        println!("Front: {:?}, Back: {:?}", deque.front(), deque.back());
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