#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use slice_deque::*;
use global_data::*;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

struct CustomType1(String);
#[derive(Clone, Hash)]
struct CustomType0(String);
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
        if data.len() < 1024 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let constructor_selector = _to_u8(GLOBAL_DATA, 0) % 5;
        let mut deque = match constructor_selector {
            0 => SliceDeque::new(),
            1 => {
                let capacity = _to_usize(GLOBAL_DATA, 1) % 65;
                SliceDeque::with_capacity(capacity)
            }
            2 => {
                let elem_count = _to_usize(GLOBAL_DATA, 10) % 65;
                let mut vec = Vec::with_capacity(elem_count);
                for i in 0..elem_count {
                    let offset = 18 + i * 20;
                    let str_len = _to_u8(GLOBAL_DATA, offset) % 17;
                    let str_data = _to_str(GLOBAL_DATA, offset + 1, offset + 1 + str_len as usize);
                    vec.push(CustomType0(str_data.to_string()));
                }
                SliceDeque::from(vec.as_slice())
            }
            3 => {
                let str_seed = _to_str(GLOBAL_DATA, 50, 50 + (_to_u8(GLOBAL_DATA, 49) % 17) as usize);
                SliceDeque::from_iter(CustomType1(str_seed.to_string()))
            }
            _ => {
                let elem = CustomType0(_to_str(GLOBAL_DATA, 70, 70 + (_to_u8(GLOBAL_DATA, 69) % 17) as usize).to_string());
                let count = _to_usize(GLOBAL_DATA, 100) % 65;
                from_elem(elem, count)
            }
        };

        let op_count = _to_usize(GLOBAL_DATA, 150) % 20;
        let mut data_offset = 160;
        
        for _ in 0..op_count {
            if data_offset + 8 > GLOBAL_DATA.len() { break; }
            
            match _to_u8(GLOBAL_DATA, data_offset) % 9 {
                0 => {
                    let str_len = _to_u8(GLOBAL_DATA, data_offset + 1) % 17;
                    data_offset += 2;
                    let elem_str = _to_str(GLOBAL_DATA, data_offset, data_offset + str_len as usize);
                    deque.push_back(CustomType0(elem_str.to_string()));
                    data_offset += str_len as usize;
                }
                1 => {
                    if let Some(e) = deque.pop_front() {
                        println!("Pop front: {:?}", e.0);
                    }
                    data_offset += 1;
                }
                2 => {
                    let (f, b) = deque.as_slices();
                    println!("Front slice: {} elements", f.len());
                    println!("Back slice: {} elements", b.len());
                    data_offset += 1;
                }
                3 => {
                    let new_len = _to_usize(GLOBAL_DATA, data_offset + 1);
                    deque.truncate(new_len);
                    data_offset += 9;
                }
                4 => {
                    let mut hasher = DefaultHasher::new();
                    deque.hash(&mut hasher);
                    println!("Hash: {:x}", hasher.finish());
                    data_offset += 1;
                }
                5 => {
                    let range = _to_usize(GLOBAL_DATA, data_offset + 1).._to_usize(GLOBAL_DATA, data_offset + 9);
                    let mut drain = deque.drain(range);
                    while let Some(e) = drain.next() {
                        println!("Drained: {}", e.0);
                    }
                    data_offset += 17;
                }
                6 => {
                    let target = _to_usize(GLOBAL_DATA, data_offset + 1) % deque.len();
                    println!("Index {}: {:?}", target, deque[target].0);
                    data_offset += 9;
                }
                7 => {
                    let new_cap = _to_usize(GLOBAL_DATA, data_offset + 1) % 128;
                    deque.reserve(new_cap);
                    data_offset += 9;
                }
                _ => {
                    if let Some(e) = deque.back_mut() {
                        e.0.push_str("!");
                        println!("Modified back: {}", e.0);
                    }
                    data_offset += 1;
                }
            }
        }

        let mut final_hasher = DefaultHasher::new();
        deque.hash(&mut final_hasher);
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