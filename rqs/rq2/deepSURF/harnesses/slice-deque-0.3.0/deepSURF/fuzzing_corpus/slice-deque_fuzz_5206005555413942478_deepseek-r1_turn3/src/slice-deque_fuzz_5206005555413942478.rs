#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use slice_deque::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

#[derive(PartialEq, PartialOrd)]
struct CustomType0(String);

impl std::clone::Clone for CustomType0 {
    fn clone(&self) -> Self {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 1);
        let custom_impl_inst_num = self.0.len();
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0 {
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector {
            1 => global_data.first_half,
            _ => global_data.second_half,
        };
        let mut t_2 = _to_u8(GLOBAL_DATA, 9) % 17;
        let t_3 = _to_str(GLOBAL_DATA, 10, 10 + t_2 as usize);
        let t_4 = String::from(t_3);
        CustomType0(t_4)
    }
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 4096 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let mut offset = 0;
        
        let constructor_sel = _to_u8(global_data.first_half, offset) % 4;
        offset += 1;
        
        let mut main_deque = match constructor_sel {
            0 => SliceDeque::new(),
            1 => SliceDeque::with_capacity(_to_usize(global_data.first_half, offset) % 128),
            2 => {
                let len = _to_usize(global_data.first_half, offset) % 65;
                offset += 8;
                let mut v = Vec::with_capacity(len);
                for _ in 0..len {
                    let s_len = _to_u8(global_data.first_half, offset) % 32;
                    offset += 1;
                    let s = _to_str(global_data.first_half, offset, offset + s_len as usize);
                    v.push(CustomType0(s.to_string()));
                    offset += s_len as usize;
                }
                SliceDeque::from(v.as_slice())
            }
            _ => {
                let elem = CustomType0("base".to_string());
                let count = _to_usize(global_data.first_half, offset) % 65;
                offset += 8;
                slice_deque::from_elem(elem, count)
            }
        };
        
        let ops_count = _to_u8(global_data.first_half, offset) % 16;
        offset += 1;
        
        for _ in 0..ops_count {
            let op = _to_u8(global_data.first_half, offset) % 7;
            offset += 1;
            
            match op {
                0 => {
                    let s_len = _to_u8(global_data.first_half, offset) % 32;
                    offset += 1;
                    let s = _to_str(global_data.first_half, offset, offset + s_len as usize);
                    main_deque.push_back(CustomType0(s.to_string()));
                    offset += s_len as usize;
                }
                1 => {
                    if let Some(front) = main_deque.front_mut() {
                        *front = CustomType0("modified".to_string());
                        println!("{:?}", front.0);
                    }
                }
                2 => {
                    let len = _to_usize(global_data.first_half, offset);
                    offset += 8;
                    main_deque.truncate(len);
                }
                3 => {
                    let mut temp = SliceDeque::new();
                    let count = _to_u8(global_data.first_half, offset) % 16;
                    offset += 1;
                    for _ in 0..count {
                        let s_len = _to_u8(global_data.first_half, offset) % 32;
                        offset += 1;
                        let s = _to_str(global_data.first_half, offset, offset + s_len as usize);
                        temp.push_front(CustomType0(s.to_string()));
                        offset += s_len as usize;
                    }
                    main_deque.append(&mut temp);
                }
                4 => {
                    let drain_range = 0.._to_usize(global_data.first_half, offset) % (main_deque.len() + 1);
                    offset += 8;
                    let mut d = main_deque.drain(drain_range);
                    while let Some(e) = d.next() {
                        println!("Drained: {}", e.0);
                    }
                }
                5 => {
                    let old_len = main_deque.len();
                    main_deque.retain(|x| x.0.len() % 2 == 0);
                    println!("Retained {} -> {}", old_len, main_deque.len());
                }
                _ => {
                    let mut other = slice_deque::from_elem(CustomType0("x".to_string()), _to_usize(global_data.first_half, offset) % 65);
                    offset += 8;
                    main_deque.extend_from_slice(other.as_slice());
                }
            }
        }
        
        let mut secondary_deque = SliceDeque::new();
        let sec_count = _to_u8(global_data.first_half, offset) % 32;
        offset += 1;
        for _ in 0..sec_count {
            let s_len = _to_u8(global_data.first_half, offset) % 32;
            offset += 1;
            let s = _to_str(global_data.first_half, offset, offset + s_len as usize);
            secondary_deque.push_front(CustomType0(s.to_string()));
            offset += s_len as usize;
        }
        
        if let Some(ord) = main_deque.partial_cmp(&secondary_deque) {
            println!("Comparison: {:?}", ord);
        }
        
        main_deque.append(&mut secondary_deque);
        
        let third_deque = SliceDeque::from_iter(main_deque.drain_filter(|x| x.0.contains('a')));
        println!("Drained {} elements", third_deque.len());
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