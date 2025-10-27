#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use arc_swap::*;
use arc_swap::access::{Access, Map};
use global_data::*;
use std::ops::Deref;
use std::sync::Arc;

#[derive(Debug, Clone)]
struct Config {
    part1: String,
    part2: String,
}

struct CustomType1(String);
struct CustomType2(Arc<CustomType3>);
#[derive(Debug)]
struct CustomType3(String);

impl Access<CustomType3> for CustomType1 {
    type Guard = CustomType2;
    
    fn load(&self) -> Self::Guard {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 25);
        let custom_impl_inst_num = self.0.len();
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0 {
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector {
            1 => global_data.first_half,
            _ => global_data.second_half,
        };
        let mut len = _to_u8(GLOBAL_DATA, 33) % 64;
        let s = _to_str(GLOBAL_DATA, 34, 34 + len as usize);
        CustomType2(Arc::new(CustomType3(s.to_string())))
    }
}

impl Deref for CustomType2 {
    type Target = CustomType3;
    
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 300 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        
        let primary_data = global_data.first_half;
        let secondary_data = global_data.second_half;

        let ct1_len = _to_u8(primary_data, 50) % 64;
        let ct1_str = _to_str(primary_data, 51, 51 + ct1_len as usize);
        let ct1 = CustomType1(ct1_str.to_string());
        
        let cfg_p1_len = _to_u8(secondary_data, 0) % 64;
        let cfg_p1 = _to_str(secondary_data, 1, 1 + cfg_p1_len as usize);
        let cfg_p2_len = _to_u8(secondary_data, 100) % 64;
        let cfg_p2 = _to_str(secondary_data, 101, 101 + cfg_p2_len as usize);
        let config_arc = ArcSwap::from_pointee(Config { part1: cfg_p1.to_string(), part2: cfg_p2.to_string() });

        let ops_count = _to_u8(secondary_data, 200) % 5;
        for op_idx in 0..ops_count {
            match op_idx % 4 {
                0 => {
                    let new_val = Config {
                        part1: format!("{}_{}", cfg_p1, op_idx),
                        part2: format!("{}_{}", cfg_p2, op_idx),
                    };
                    config_arc.store(Arc::new(new_val));
                }
                1 => {
                    config_arc.rcu(|old| {
                        let mut new = (**old).clone();
                        new.part1.push_str("-modified");
                        Arc::new(new)
                    });
                }
                2 => {
                    let curr = config_arc.load_full();
                    let new_val = Config {
                        part1: curr.part1.clone(),
                        part2: format!("{}_v2", curr.part2),
                    };
                    config_arc.compare_and_swap(&curr, Arc::new(new_val));
                }
                _ => {
                    config_arc.swap(Arc::new(Config {
                        part1: "reset".into(),
                        part2: "reset".into(),
                    }));
                }
            }
        }

        let map_access = Map::new(&config_arc, |cfg: &Config| &cfg.part1);
        let guard = map_access.load();
        println!("{:?}", *guard);
        
        let t14 = arc_swap::access::DynAccess::<CustomType3>::load(&ct1);
        println!("{:?}", *t14.deref());

        let mut cache = cache::Cache::new(&config_arc);
        let cached = cache.load();
        println!("Cached: {}", cached.part2);
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