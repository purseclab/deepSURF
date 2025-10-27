#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use ordnung::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

#[derive(Debug)]
struct CustomType3(String);
#[derive(Debug)]
struct CustomType1(String);
#[derive(Debug)]
struct CustomType2(String);
#[derive(Debug, Hash, Eq, PartialEq)]
struct CustomType0(String);
#[derive(Debug)]
struct CustomType5(String);
#[derive(Debug)]
struct CustomType4(String);

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
        let mut t_9 = _to_u8(GLOBAL_DATA, 57) % 17;
        let t_10 = _to_str(GLOBAL_DATA, 58, 58 + t_9 as usize);
        let t_11 = String::from(t_10);
        let t_12 = CustomType4(t_11);
        return t_12;
    }
}

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
        let k_start = 332;
        let k_len = _to_u8(GLOBAL_DATA, k_start) % 17;
        let k = _to_str(GLOBAL_DATA, k_start + 1, k_start + 1 + k_len as usize);
        let v_start = k_start + 1 + k_len as usize;
        let v_len = _to_u8(GLOBAL_DATA, v_start) % 17;
        let v = _to_str(GLOBAL_DATA, v_start + 1, v_start + 1 + v_len as usize);
        Some((CustomType0(k.to_string()), CustomType1(v.to_string())))
    }
}

fn main (){
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 4096 {return;}
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let constructor_selector = _to_u8(GLOBAL_DATA, 0) % 3;
        let mut map = match constructor_selector {
            0 => Map::new(),
            1 => Map::with_capacity(_to_usize(GLOBAL_DATA,1)),
            2 => {
                let mut t_13 = _to_u8(GLOBAL_DATA,1) % 17;
                let t_14 = _to_str(GLOBAL_DATA,2,2+t_13 as usize);
                let t_15 = CustomType2(t_14.to_string());
                Map::from_iter(t_15)
            },
            _ => unreachable!()
        };

        let insert_ops = _to_usize(GLOBAL_DATA,100) % 10;
        for idx in 0..insert_ops {
            let k_start = 200 + idx * 20;
            let k_len = _to_u8(GLOBAL_DATA, k_start) % 17;
            let k = _to_str(GLOBAL_DATA, k_start + 1, k_start + 1 + k_len as usize);
            let v_start = k_start + 1 + k_len as usize;
            let v_len = _to_u8(GLOBAL_DATA, v_start) % 17;
            let v = _to_str(GLOBAL_DATA, v_start + 1, v_start + 1 + v_len as usize);
            map.insert(CustomType0(k.to_string()), CustomType1(v.to_string()));
        }

        let mut entries = vec![];
        let iter_ops = _to_usize(GLOBAL_DATA, 800) % 10;
        for idx in 0..iter_ops {
            let k_start = 900 + idx * 20;
            let k_len = _to_u8(GLOBAL_DATA, k_start) % 17;
            let k = _to_str(GLOBAL_DATA, k_start + 1, k_start + 1 + k_len as usize);
            let v_start = k_start + 1 + k_len as usize;
            let v_len = _to_u8(GLOBAL_DATA, v_start) % 17;
            let v = _to_str(GLOBAL_DATA, v_start + 1, v_start + 1 + v_len as usize);
            entries.push((CustomType0(k.to_string()), CustomType1(v.to_string())));
        }

        let _temp_map: Map<CustomType0, CustomType1> = Map::from_iter(entries.into_iter());

        let lookup_ops = _to_usize(GLOBAL_DATA,300) % 10;
        for idx in 0..lookup_ops {
            let k_start = 400 + idx * 20;
            let k_len = _to_u8(GLOBAL_DATA, k_start) % 17;
            let k = _to_str(GLOBAL_DATA, k_start + 1, k_start + 1 + k_len as usize);
            if let Some(val) = map.get(&CustomType0(k.to_string())) {
                println!("{val:?}");
            }
        }

        let mut iter_mut = map.iter_mut();
        while let Some((_, v)) = iter_mut.next() {
            v.0.push_str(&_to_str(GLOBAL_DATA,500,504));
        }

        let _len = map.len();
        let _empty = map.is_empty();

        let remove_ops = _to_usize(GLOBAL_DATA,600) % 10;
        for idx in 0..remove_ops {
            let k_start = 700 + idx * 20;
            let k_len = _to_u8(GLOBAL_DATA, k_start) % 17;
            let k = _to_str(GLOBAL_DATA, k_start + 1, k_start + 1 + k_len as usize);
            map.remove(&CustomType0(k.to_string()));
        }

        let _ = map.contains_key(&CustomType0(String::new()));
        map.clear();
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