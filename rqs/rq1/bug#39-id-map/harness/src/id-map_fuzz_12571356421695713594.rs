#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use id_map::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

#[derive(PartialEq)]
struct CustomType0(String);

impl std::clone::Clone for CustomType0 {
    fn clone(&self) -> Self {
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
        let mut t_0 = _to_u8(GLOBAL_DATA, 8) % 17;
        let t_1 = _to_str(GLOBAL_DATA, 9, 9 + t_0 as usize);
        let t_2 = String::from(t_1);
        let t_3 = CustomType0(t_2);
        return t_3;
    }
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 500 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let mut offset = 0;
        
        let constructor_selector = _to_u8(global_data.first_half, offset);
        offset += 1;
        let map_capacity = _to_usize(global_data.first_half, offset);
        offset += std::mem::size_of::<usize>();
        
        let mut primary_map = match constructor_selector % 2 {
            0 => IdMap::<CustomType0>::new(),
            _ => IdMap::with_capacity(map_capacity % 65),
        };
        
        let insert_rounds = _to_u8(global_data.first_half, offset);
        offset += 1;
        
        for _ in 0..insert_rounds {
            let value_len = _to_u8(global_data.first_half, offset) % 65;
            offset += 1;
            let value_str = _to_str(global_data.first_half, offset, offset + value_len as usize);
            offset += value_len as usize;
            primary_map.insert(CustomType0(value_str.to_string()));
        }
        
        let mut secondary_map = IdMap::new();
        let operation_count = _to_u8(global_data.first_half, offset);
        offset += 1;
        
        for _ in 0..operation_count {
            let op_selector = _to_u8(global_data.first_half, offset);
            offset += 1;
            
            match op_selector % 7 {
                0 => {
                    let target_id = _to_usize(global_data.first_half, offset);
                    offset += std::mem::size_of::<usize>();
                    if let Some(val) = primary_map.get(target_id) {
                        println!("{:?}", val.0);
                    }
                },
                1 => {
                    let remove_id = _to_usize(global_data.first_half, offset);
                    offset += std::mem::size_of::<usize>();
                    primary_map.remove(remove_id);
                },
                2 => {
                    let mut iter = primary_map.values_mut();
                    while let Some(val) = iter.next() {
                        println!("{}", val.0);
                    }
                },
                3 => {
                    let mut set = id_set::IdSet::new_filled(_to_usize(global_data.first_half, offset));
                    offset += std::mem::size_of::<usize>();
                    primary_map.remove_set(&set);
                },
                4 => {
                    let key = _to_usize(global_data.first_half, offset);
                    offset += std::mem::size_of::<usize>();
                    primary_map.get_or_insert(key, CustomType0(String::new()));
                },
                5 => {
                    primary_map.retain(|id, _| id % 2 == 0);
                },
                _ => {
                    secondary_map.clone_from(&primary_map);
                }
            }
        }

        let dynamic_map = IdMap::from_iter(secondary_map.iter().map(|(id, val)| (id, val.clone())));
        let comparison = &primary_map == &dynamic_map;
        println!("Comparison: {}", comparison);
        
        let mut into_iter = primary_map.into_iter();
        while let Some((_, val)) = into_iter.next() {
            println!("{}", val.0);
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