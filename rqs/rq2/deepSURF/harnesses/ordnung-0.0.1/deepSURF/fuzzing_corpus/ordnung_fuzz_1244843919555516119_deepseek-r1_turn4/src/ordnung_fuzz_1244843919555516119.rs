#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use ordnung::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
struct CustomType0(String);
#[derive(Debug, Clone)]
struct CustomType1(String);
#[derive(Debug)]
struct CustomType2(String);
#[derive(Debug)]
struct CustomType3(String);
#[derive(Debug)]
struct CustomType4(String);
#[derive(Debug)]
struct CustomType5(String);

impl core::iter::Iterator for CustomType4 {
    type Item = (CustomType0, CustomType1);
    
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
        let key_len = _to_u8(GLOBAL_DATA, 32) % 17;
        let key_part = _to_str(GLOBAL_DATA, 33, 33 + key_len as usize);
        let key = CustomType0(key_part.to_string());
        let val_start = 33 + key_len as usize;
        let val_len = _to_u8(GLOBAL_DATA, val_start) % 17;
        let val_part = _to_str(GLOBAL_DATA, val_start + 1, val_start + 1 + val_len as usize);
        let value = CustomType1(val_part.to_string());
        Some((key, value))
    }
}

impl core::iter::IntoIterator for CustomType2 {
    type Item = (CustomType0, CustomType1);
    type IntoIter = CustomType4;
    
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
        let mut t_9 = _to_u8(GLOBAL_DATA,57) %17;
        let t_10 = _to_str(GLOBAL_DATA,58,58 +t_9 as usize);
        let t_11=String::from(t_10);
        CustomType4(t_11)
    }
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 500 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let second_half = global_data.second_half;

        let constructor_choice = _to_u8(GLOBAL_DATA,0) %3;
        let mut map = match constructor_choice {
            0 => ordnung::Map::<CustomType0, CustomType1>::new(),
            1 => {
                let capacity = _to_usize(GLOBAL_DATA,1) %65;
                ordnung::Map::with_capacity(capacity)
            }
            2 => {
                let len_iter = _to_u8(GLOBAL_DATA,2) %17;
                let slice = _to_str(GLOBAL_DATA,3, 3 + len_iter as usize);
                let ct2 = CustomType2(slice.to_string());
                ordnung::Map::from_iter(ct2)
            }
            _ => unreachable!()
        };

        for i in 0..(_to_usize(second_half,0) %20) {
            match _to_u8(second_half, i as usize +1) %14 {
                0 => {
                    let klen = _to_u8(GLOBAL_DATA,10 +i*3) %17;
                    let kval = _to_str(GLOBAL_DATA,11 +i*3, 11+i*3 +klen as usize);
                    let vlen = _to_u8(second_half,10 +i*3) %17;
                    let vval = _to_str(second_half,11 +i*3,11+i*3 +vlen as usize);
                    map.insert(CustomType0(kval.to_string()), CustomType1(vval.to_string()));
                }
                1 => {
                    let klen = _to_u8(GLOBAL_DATA,70 +i*2) %17;
                    let kval = _to_str(GLOBAL_DATA,71 +i*2,71 +i*2 +klen as usize);
                    let key = CustomType0(kval.to_string());
                    let _ = map.get(&key);
                }
                2 => {
                    let klen = _to_u8(second_half,50 +i*2) %17;
                    let kval = _to_str(second_half,51 +i*2,51 +i*2 +klen as usize);
                    let key = CustomType0(kval.to_string());
                    let _ = map.get_mut(&key);
                }
                3 => {
                    let klen = _to_u8(GLOBAL_DATA,100 +i) %17;
                    let kval = _to_str(GLOBAL_DATA,101 +i,101 +i +klen as usize);
                    let key = CustomType0(kval.to_string());
                    let _ = map.contains_key(&key);
                }
                4 => { map.clear(); }
                5 => { 
                    let _ = map.len();
                    let _ = map.is_empty();
                }
                6 => {
                    let mut cloned = map.clone();
                    let klen = _to_u8(second_half,100 +i) %17;
                    let kval = _to_str(second_half,101 +i,101 +i +klen as usize);
                    let key = CustomType0(kval.to_string());
                    cloned.get(&key);
                }
                7 => {
                    for (k, v) in map.iter() {
                        println!("{:?} {:?}", *k, *v);
                    }
                }
                8 => {
                    for (_, v) in map.iter_mut() {
                        v.0.push('!');
                    }
                }
                9 => {
                    let klen = _to_u8(GLOBAL_DATA,200 +i) %17;
                    let kval = _to_str(GLOBAL_DATA,201 +i,201 +i +klen as usize);
                    map.remove(&CustomType0(kval.to_string()));
                }
                _ => continue
            }
        }

        let target_key_len = _to_u8(GLOBAL_DATA,150) %17;
        let target_key = _to_str(GLOBAL_DATA,151, 151 + target_key_len as usize);
        let key = CustomType0(target_key.to_string());
        let _ = map.get(&key);
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