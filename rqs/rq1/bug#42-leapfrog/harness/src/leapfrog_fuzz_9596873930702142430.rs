#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use leapfrog::*;
use leapfrog::hashmap::Entry;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

#[derive(Debug)]
struct CustomType5(String);
struct CustomType1(String);
struct CustomType4(String);
struct CustomType3(String);
struct CustomType2(usize);

impl leapfrog::Value for CustomType2 {
    
    fn redirect() -> Self {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let t_2 = _to_u8(GLOBAL_DATA, 16);
        if t_2 % 2 == 0{
            panic!("INTENTIONAL PANIC!");
        }
        let t_3 = _to_usize(GLOBAL_DATA, 17);
        let t_4 = CustomType2(t_3);
        return t_4;
    }
    
    fn is_null(&self) -> bool {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 25);
        let custom_impl_inst_num = self.0;
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0{
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector{
                    1 => global_data.first_half,
                    _ => global_data.second_half,
        };
        let t_5 = _to_bool(GLOBAL_DATA, 33);
        return t_5;
    }
    
    fn is_redirect(&self) -> bool {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 34);
        let custom_impl_inst_num = self.0;
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0{
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector{
                    1 => global_data.first_half,
                    _ => global_data.second_half,
        };
        let t_6 = _to_bool(GLOBAL_DATA, 42);
        return t_6;
    }
    
    fn null() -> Self {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let t_7 = _to_u8(GLOBAL_DATA, 43);
        if t_7 % 2 == 0{
            panic!("INTENTIONAL PANIC!");
        }
        let t_8 = _to_usize(GLOBAL_DATA, 44);
        let t_9 = CustomType2(t_8);
        return t_9;
    }
}

impl std::clone::Clone for CustomType5 {
    
    fn clone(&self) -> Self {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 119);
        let custom_impl_inst_num = self.0.len();
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0{
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector{
                    1 => global_data.first_half,
                    _ => global_data.second_half,
        };
        let mut t_23 = _to_u8(GLOBAL_DATA, 127) % 17;
        let t_24 = _to_str(GLOBAL_DATA, 128, 128 + t_23 as usize);
        let t_25 = String::from(t_24);
        let t_26 = CustomType5(t_25);
        return t_26;
    }
}
impl std::clone::Clone for CustomType2 {
    
    fn clone(&self) -> Self {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 52);
        let custom_impl_inst_num = self.0;
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0{
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector{
                    1 => global_data.first_half,
                    _ => global_data.second_half,
        };
        let t_10 = _to_usize(GLOBAL_DATA, 60);
        let t_11 = CustomType2(t_10);
        return t_11;
    }
}

impl std::cmp::Eq for CustomType5 {}

impl std::cmp::PartialEq for CustomType5 {
    
    fn eq(&self, _: &Self) -> bool {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 152);
        let custom_impl_inst_num = self.0.len();
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0{
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector{
                    1 => global_data.first_half,
                    _ => global_data.second_half,
        };
        let t_27 = _to_bool(GLOBAL_DATA, 160);
        return t_27;
    }
}

impl std::hash::Hash for CustomType5 {
    
    fn hash<H: std::hash::Hasher>(&self, _: &mut H) {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 144);
        let custom_impl_inst_num = self.0.len();
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0{
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector{
                    1 => global_data.first_half,
                    _ => global_data.second_half,
        };
        return ;
    }
}

impl std::default::Default for CustomType3 {
    
    fn default() -> Self {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let t_14 = _to_u8(GLOBAL_DATA, 76);
        if t_14 % 2 == 0{
            panic!("INTENTIONAL PANIC!");
        }
        let mut t_15 = _to_u8(GLOBAL_DATA, 77) % 17;
        let t_16 = _to_str(GLOBAL_DATA, 78, 78 + t_15 as usize);
        let t_17 = String::from(t_16);
        let t_18 = CustomType3(t_17);
        return t_18;
    }
}

impl std::fmt::Debug for CustomType2 {
    
    fn fmt(&self, _: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 68);
        let custom_impl_inst_num = self.0;
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0{
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector{
                    1 => global_data.first_half,
                    _ => global_data.second_half,
        };
        let t_12 = ();
        let t_13 = Ok(t_12);
        return t_13;
    }
}

impl std::marker::Copy for CustomType2 {}

fn main(){
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 1024 {return;}
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        
        let capacity = _to_usize(GLOBAL_DATA, 161);
        let constructor_choice = _to_u8(GLOBAL_DATA, 165) % 2;

        let mut map = match constructor_choice {
            0 => leapfrog::hashmap::HashMap::with_capacity(capacity),
            1 => leapfrog::hashmap::HashMap::with_capacity_and_hasher(
                capacity,
                std::hash::BuildHasherDefault::<std::collections::hash_map::DefaultHasher>::default()
            ),
            _ => unreachable!()
        };

        let operations = _to_usize(GLOBAL_DATA, 200) % 10 + 5;
        for i in 0..operations {
            let op_select = _to_u8(GLOBAL_DATA, 201 + i) % 6;
            let key_len = _to_u8(GLOBAL_DATA, 210 + i) % 17;
            let key_str = _to_str(GLOBAL_DATA, 220 + i * 20, 220 + i * 20 + key_len as usize);
            let key = CustomType5(String::from(key_str));
            let value = CustomType2(_to_usize(GLOBAL_DATA, 300 + i * 8));

            match op_select {
                0 => {
                    map.insert(key, value);
                }
                1 => {
                    let exists = map.contains_key(&key);
                    println!("Contains key: {:?} {}", key, exists);
                }
                2 => {
                    if let Some(v) = map.get(&key) {
                        println!("Fetched value: {:?}", *v);
                    }
                }
                3 => {
                    let entry = map.entry(key);
                    match entry {
                        Entry::Occupied(mut oe) => {
                            println!("Occupied entry: {:?}", oe.key());
                            oe.insert(value);
                        }
                        Entry::Vacant(ve) => {
                            ve.insert_entry(value);
                        }
                    }
                }
                4 => {
                    if let Some(mut v) = map.get_mut(&key) {
                        *v = value;
                        println!("Updated value: {:?}", v);
                    }
                }
                5 => {
                    let deleted = map.remove(&key);
                    println!("Removed value: {:?}", deleted);
                }
                _ => {}
            }
        }

        let removal_key_len = _to_u8(GLOBAL_DATA, 900) % 17;
        let removal_key_str = _to_str(GLOBAL_DATA, 910, 910 + removal_key_len as usize);
        let removal_key = CustomType5(String::from(removal_key_str));
        let entry = map.entry(removal_key);
        let inserted_entry = entry.insert_entry(CustomType2(_to_usize(GLOBAL_DATA, 950)));
        inserted_entry.remove();

        let check_key = _to_u8(GLOBAL_DATA, 960) % 17;
        let check_key_str = _to_str(GLOBAL_DATA, 970, 970 + check_key as usize);
        let check_entry = map.entry(CustomType5(String::from(check_key_str)));
        println!("Checked entry: {:?}", check_entry.key());

        if let Some(kv) = map.get_key_value(&CustomType5(String::default())) {
            println!("Key-value pair: {:?} {:?}", kv.0, kv.1);
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