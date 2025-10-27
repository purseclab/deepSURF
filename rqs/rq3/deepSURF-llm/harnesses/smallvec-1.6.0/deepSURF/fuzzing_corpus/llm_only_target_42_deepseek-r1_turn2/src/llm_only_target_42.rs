#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;    

use smallvec::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

#[derive(Debug)]
struct CustomElement(u32);
impl PartialEq for CustomElement {
    fn eq(&self, other: &Self) -> bool {
        let CustomElement(a) = self;
        let CustomElement(b) = other;
        (a ^ b) % _to_u8(get_global_data().first_half, 105) as u32 == 0
    }
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 105 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let t_0 = _to_u8(GLOBAL_DATA, 0);
        let t_5 = _to_usize(GLOBAL_DATA, 31);
        let t_2 = _to_u32(GLOBAL_DATA, 3);
        let t_15 = _to_bool(GLOBAL_DATA, 94);
        let t_10 = _to_i128(GLOBAL_DATA, 54);
        let t_8 = _to_i32(GLOBAL_DATA, 42);
        let t_12 = _to_f32(GLOBAL_DATA, 78);
        let t_9 = _to_i64(GLOBAL_DATA, 46);
        let t_4 = _to_u128(GLOBAL_DATA, 15);
        let t_3 = _to_u64(GLOBAL_DATA, 7);
        let t_6 = _to_i8(GLOBAL_DATA, 39);
        let t_7 = _to_i16(GLOBAL_DATA, 40);
        let t_1 = _to_u16(GLOBAL_DATA, 1);

        let mut vec1 = match t_15 {
            true => SmallVec::<[u32; 32]>::new(),
            false => SmallVec::with_capacity(t_5 % 128)
        };

        let ops_count = (t_0 as usize) % 12;
        for i in 0..ops_count {
            match GLOBAL_DATA[i % GLOBAL_DATA.len()] % 7 {
                0 => vec1.push(_to_u32(GLOBAL_DATA, i * 4)),
                1 => vec1.insert(t_5.wrapping_add(i), t_2),
                2 => { vec1.pop(); }
                3 => vec1.truncate(t_5 % 64),
                4 => if !vec1.is_empty() {
                    let _ = vec1.drain((t_5 % vec1.len())..);
                },
                5 => { vec1.swap_remove(t_5 % (vec1.len() + 1)); },
                6 => {
                    let choice = t_10 % 2 == 0;
                    vec1.retain(move |x: &mut u32| {
                        let val = *x;
                        if choice { val % 2 == 0 } else { val.is_power_of_two() }
                    });
                },
                _ => ()
            }
        }

        let mut custom_vec = SmallVec::<[CustomElement; 64]>::new();
        custom_vec.push(CustomElement(t_2));
        for _ in 0.._to_u8(GLOBAL_DATA, 99) {
            custom_vec.insert(t_5 % (custom_vec.len() + 1), CustomElement(t_8 as u32));
        }
        custom_vec.dedup();
        println!("{:?}", custom_vec.as_slice());

        let slice_len = _to_usize(GLOBAL_DATA, 100) % 65;
        let mut heap_vec = SmallVec::<[u64; 128]>::new();
        for idx in 0..slice_len {
            heap_vec.push(_to_u64(GLOBAL_DATA, 101 + idx * 8));
        }
        heap_vec.insert(t_5 % (heap_vec.len() + 1), t_3);
        let _drained = heap_vec.drain(..);
        
        vec1.insert(t_5 % (vec1.len() + 1), t_12.to_bits());
        println!("{:?}", &vec1[..]);
        
        let mut decomposed = SmallVec::<[String; 16]>::new();
        decomposed.insert(0, _to_str(GLOBAL_DATA, 95, 105).to_string());
        decomposed.insert(0, format!("{:?}", t_4));
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