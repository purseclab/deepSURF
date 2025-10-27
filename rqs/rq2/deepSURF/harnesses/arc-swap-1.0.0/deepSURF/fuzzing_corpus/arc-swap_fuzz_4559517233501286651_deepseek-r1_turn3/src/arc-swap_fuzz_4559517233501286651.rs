#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use std::sync::Arc;
use arc_swap::ArcSwap;
use global_data::*;
use std::ops::Deref;

#[derive(Debug)]
struct CustomType1(String);
#[derive(Debug)]
struct CustomType0(String);

impl std::convert::Into<CustomType1> for CustomType0 {
    fn into(self) -> CustomType1 {
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
        let t_3 = CustomType1(t_2);
        t_3
    }
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 512 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let mut offset = 0;
        let num_instances = _to_u8(GLOBAL_DATA, offset) % 5;
        offset += 1;

        for _ in 0..num_instances {
            let ctor_selector = _to_u8(GLOBAL_DATA, offset) % 3;
            offset += 1;

            match ctor_selector {
                0 => {
                    let t4 = _to_u8(GLOBAL_DATA, offset) % 17;
                    offset += 1;
                    let end = offset + t4 as usize;
                    let t5 = _to_str(GLOBAL_DATA, offset, end);
                    offset = end;
                    let val = CustomType0(t5.to_string());
                    let arc = ArcSwap::from_pointee(val);
                    println!("Created via from_pointee: {:?}", arc);
                    
                    let new_val = CustomType0(_to_str(GLOBAL_DATA, offset, offset + 5).to_string());
                    arc.store(Arc::new(new_val));
                    println!("After store: {:?}", arc);
                }
                1 => {
                    let atomic = ArcSwap::new(Arc::new(CustomType0("default".into())));
                    println!("Created via new: {:?}", atomic);
                    
                    let guard = atomic.load_full();
                    println!("Guard content: {:?}", *guard);
                }
                2 => {
                    let empty = ArcSwap::from_pointee(CustomType0("".into()));
                    println!("Created empty: {:?}", empty);
                    
                    let mut t4 = _to_u8(GLOBAL_DATA, offset) % 17;
                    offset += 1;
                    let s = _to_str(GLOBAL_DATA, offset, offset + t4 as usize);
                    offset += t4 as usize;
                    empty.store(Arc::new(CustomType0(s.to_string())));
                    println!("After store on empty: {:?}", empty);
                }
                _ => unreachable!(),
            }
        }

        let dynamic_arc = ArcSwap::from_pointee(CustomType0("init".into()));
        for _ in 0..3 {
            let swap_val = CustomType0(format!("swap{}", _to_u8(GLOBAL_DATA, offset)));
            offset += 1;
            let old = dynamic_arc.swap(Arc::new(swap_val));
            println!("After swap: {:?} -> {:?}", dynamic_arc, old);
        }

        let compare_val = Arc::new(CustomType0("compare".into()));
        let guard = dynamic_arc.load();
        let cas_result = dynamic_arc.compare_and_swap(&*guard, compare_val);
        println!("CAS result: {:?}", cas_result);
        println!("Final state: {:?}", dynamic_arc);
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