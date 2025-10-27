#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use arc_swap::{ArcSwap, ArcSwapOption, Cache, cache::Access};
use global_data::*;
use std::sync::atomic::Ordering;
use std::sync::Arc;

#[derive(Debug)]
struct CustomType2(String);
#[derive(Debug)]
struct CustomType1(String);
#[derive(Debug)]
struct CustomType0(String);

impl std::ops::Deref for CustomType1 {
    type Target = CustomType2;
    
    fn deref(&self) -> &Self::Target {
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
        let t_0 = _to_u8(GLOBAL_DATA, 8) % 17;
        let t_1 = _to_str(GLOBAL_DATA, 9, 9 + t_0 as usize);
        let t_2 = String::from(t_1);
        let t_3 = Box::leak(Box::new(CustomType2(t_2)));
        t_3
    }
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 512 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        
        let op_count = _to_u8(GLOBAL_DATA, 0) % 16;
        for i in 0..op_count {
            let selector = _to_u8(GLOBAL_DATA, 1 + i as usize) % 7;
            match selector {
                0 => {
                    let idx = 10 * i as usize + 2;
                    let len = _to_u8(GLOBAL_DATA, idx) % 17;
                    let s = _to_str(GLOBAL_DATA, idx + 1, idx + 1 + len as usize);
                    let ct = CustomType0(s.to_string());
                    let arc_swap = ArcSwap::from_pointee(ct);
                    let mut cache = Cache::new(&arc_swap);
                    let guard = cache.load();
                    println!("{:?}", *guard);
                }
                1 => {
                    let idx = 10 * i as usize + 2;
                    let len = _to_u8(GLOBAL_DATA, idx) % 17;
                    let s = _to_str(GLOBAL_DATA, idx + 1, idx + 1 + len as usize);
                    let ct = CustomType1(s.to_string());
                    let arc_swap = ArcSwap::from_pointee(ct);
                    let mut cache = Cache::new(&arc_swap);
                    let guard = cache.load();
                    let mut other_cache = Cache::new(&arc_swap);
                    let mut map_cache = other_cache.map(|x| x);
                    let mapped = map_cache.load();
                    println!("{:?}{:?}", *guard, *mapped);
                }
                2 => {
                    let arc = Arc::new(CustomType0("default".to_string()));
                    let arc_swap = ArcSwap::new(arc);
                    arc_swap.store(Arc::new(CustomType0("new".to_string())));
                    let guard = arc_swap.load();
                    println!("{:?}", *guard);
                }
                3 => {
                    let arc_swap = ArcSwap::from_pointee(CustomType0("old".to_string()));
                    let new_arc = Arc::new(CustomType0("new".to_string()));
                    let guard = arc_swap.swap(new_arc);
                    println!("{:?}", *guard);
                }
                4 => {
                    let arc_swap = ArcSwapOption::<CustomType0>::empty();
                    let new_arc = Arc::new(CustomType0("filled".to_string()));
                    arc_swap.store(Some(new_arc));
                    let guard = arc_swap.load_full();
                    println!("{:?}", *_unwrap_option(guard));
                }
                5 => {
                    let arc_swap = ArcSwap::from_pointee(CustomType0("base".to_string()));
                    let new_arc = Arc::new(CustomType0("current".to_string()));
                    let guard = arc_swap.compare_and_swap(arc_swap.load(), new_arc);
                    println!("{:?}", *guard);
                }
                6 => {
                    let idx = 10 * i as usize + 2;
                    let len = _to_u8(GLOBAL_DATA, idx) % 17;
                    let s = _to_str(GLOBAL_DATA, idx + 1, idx + 1 + len as usize);
                    let ct = CustomType0(s.to_string());
                    let arc = Arc::new(ct);
                    let arc_swap = ArcSwap::new(arc);
                    let mut cache = Cache::new(&arc_swap);
                    let guard = cache.load();
                    let mut other_cache = Cache::new(&arc_swap);
                    let mut mapped = other_cache.map(|x| x);
                    mapped.load();
                    println!("{:?}", *guard);
                }
                _ => {}
            }
        }

        let mut t_5 = _to_u8(GLOBAL_DATA, 25) % 17;
        let t_6 = _to_str(GLOBAL_DATA, 26, 26 + t_5 as usize);
        let t_7 = String::from(t_6);
        let t_8 = CustomType1(t_7);
        let arc_swap = ArcSwap::from_pointee(t_8);
        let mut t_9 = Cache::new(&arc_swap);
        let mut t_10 = &mut t_9;
        t_10.load();
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