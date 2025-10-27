#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use arenavec::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

fn main (){
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 420 {return;}
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        
        let mut op_count = _to_u8(GLOBAL_DATA, 64) % 8;
        let mut vec_storage = Vec::new();
        
        let backing_selector = _to_u8(GLOBAL_DATA, 72) % 2;
        let backing = match backing_selector {
            0 => arenavec::common::ArenaBacking::MemoryMap,
            _ => arenavec::common::ArenaBacking::SystemAllocation
        };
        let cap = _to_usize(GLOBAL_DATA, 80);
        let arena = arenavec::rc::Arena::init_capacity(backing, cap);
        let arena = _unwrap_result(arena);
        let inner_ref = arena.inner();

        for i in 0..op_count {
            let selector = _to_u8(GLOBAL_DATA, 65 + i as usize) % 4;
            match selector {
                0 => {
                    let handle_idx = 80 + i as usize * 40;
                    let mut t_22 = _to_u8(GLOBAL_DATA, handle_idx) % 17;
                    let _ = _to_str(GLOBAL_DATA, handle_idx + 1, handle_idx + 1 + t_22 as usize);
                    
                    let cap_selector = _to_u8(GLOBAL_DATA, handle_idx + 18) % 2;
                    let sv = match cap_selector {
                        0 => arenavec::common::SliceVec::new(inner_ref.clone()),
                        _ => {
                            let cap = _to_usize(GLOBAL_DATA, handle_idx + 20);
                            arenavec::common::SliceVec::with_capacity(inner_ref.clone(), cap)
                        }
                    };
                    vec_storage.push(sv);
                },
                1 => {
                    if let Some(sv) = vec_storage.last_mut() {
                        let elem_idx = 200 + i as usize * 15;
                        let elem = _to_u8(GLOBAL_DATA, elem_idx) % 128;
                        sv.push((elem as char).to_string());
                    }
                },
                2 => {
                    if let Some(sv) = vec_storage.last_mut() {
                        sv.pop();
                    }
                },
                3 => {
                    if vec_storage.len() > 1 {
                        let split_idx = vec_storage.len() - 1;
                        let mut sv1 = vec_storage.remove(split_idx);
                        if let Some(sv2) = vec_storage.last_mut() {
                            sv2.append(&mut sv1);
                        }
                    }
                },
                _ => unreachable!()
            }
        }
        
        for sv in &vec_storage {
            let mut iter = sv.iter();
            while let Some(_) = iter.next() {}
        }
        
        for sv in &mut vec_storage {
            let mut iter_mut = sv.iter_mut();
            while let Some(_) = iter_mut.next() {}
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