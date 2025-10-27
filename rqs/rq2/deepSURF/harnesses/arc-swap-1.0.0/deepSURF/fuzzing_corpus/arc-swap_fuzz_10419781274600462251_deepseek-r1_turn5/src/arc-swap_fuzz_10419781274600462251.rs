#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use arc_swap::{ArcSwap, cache::Cache, access::Access};
use global_data::*;
use std::sync::Arc;

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 1024 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let data = global_data.first_half;

        let mut offset = 0;
        let num_ops = _to_usize(data, offset) % 20;
        offset += 2;

        let val1 = make_arc_str(&data, &mut offset);
        let mut arc_swap1 = ArcSwap::new(val1);
        let val2 = make_arc_str(&data, &mut offset);
        let arc_swap2 = ArcSwap::new(val2);
        let arc_swap3 = ArcSwap::from_pointee(String::new());

        for _ in 0..num_ops {
            let op = _to_u8(data, offset) % 8;
            offset += 1;
            
            match op {
                0 => {
                    let new_val = make_arc_str(&data, &mut offset);
                    arc_swap1.store(new_val);
                },
                1 => {
                    let new_val = make_arc_str(&data, &mut offset);
                    let _ = arc_swap1.swap(new_val);
                },
                2 => {
                    let guard = arc_swap1.load();
                    let new_val = make_arc_str(&data, &mut offset);
                    let _ = arc_swap1.compare_and_swap(&*guard, new_val);
                },
                3 => {
                    let proj = arc_swap1.map(|s: &Arc<String>| &**s);
                    let guard = proj.load();
                    println!("{:?}", *guard);
                },
                4 => {
                    let mut cache = Cache::new(&arc_swap2);
                    let val = cache.load();
                    let _ = arc_swap3.swap(val.clone());
                },
                5 => {
                    let val = arc_swap2.load_full();
                    println!("{}", val);
                },
                6 => {
                    let val = arc_swap1.load_full();
                    arc_swap3.store(val);
                },
                7 => {
                    let guard = arc_swap2.load();
                    arc_swap1.store(arc_swap::Guard::into_inner(guard));
                },
                _ => (),
            }
        }

        let guard = arc_swap1.load();
        let _ = arc_swap::Guard::into_inner(guard);
        let _ = arc_swap2.into_inner();
    });
}

fn make_arc_str(data: &[u8], offset: &mut usize) -> Arc<String> {
    let len = _to_u8(data, *offset) % 23;
    *offset += 1;
    let s = _to_str(data, *offset, *offset + len as usize);
    *offset += len as usize;

    let global_data = get_global_data();
    let first_half = global_data.first_half;

    let custom_impl_num = _to_usize(first_half, 0);
    let custom_impl_inst_num = s.len();
    let selector = (custom_impl_num + custom_impl_inst_num) % 3;
    if selector == 0 {
        panic!("INTENTIAL PANIC!");
    }

    let selected_data = match selector {
        1 => global_data.first_half,
        _ => global_data.second_half,
    };

    let t_0 = _to_u8(selected_data, 8) % 17;
    let t_1 = _to_str(selected_data, 9, 9 + t_0 as usize);
    Arc::new(String::from(t_1))
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