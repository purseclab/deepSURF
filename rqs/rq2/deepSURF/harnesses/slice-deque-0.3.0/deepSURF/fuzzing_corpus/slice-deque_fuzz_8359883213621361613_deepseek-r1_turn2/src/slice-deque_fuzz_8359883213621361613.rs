#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use slice_deque::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

#[derive(Debug, PartialEq)]
struct CustomType0(String);

impl std::clone::Clone for CustomType0 {
    
    fn clone(&self) -> Self {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 1);
        let custom_impl_inst_num = self.0.len();
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0{
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector{
                    1 => global_data.first_half,
                    _ => global_data.second_half,
        };
        let mut t_2 = _to_u8(GLOBAL_DATA, 9) % 17;
        let t_3 = _to_str(GLOBAL_DATA, 10, 10 + t_2 as usize);
        let t_4 = String::from(t_3);
        let t_5 = CustomType0(t_4);
        return t_5;
    }
}

fn main (){
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 4096 {return;}
        set_global_data(data);
        let global_data = get_global_data();
        
        let mut to_create = _to_usize(global_data.first_half, 0) % 4;
        let mut deques = Vec::new();
        
        let mut data_offset = 8;
        let elem_count = _to_usize(global_data.first_half, 0) % 65;
        let mut t_1 = Vec::with_capacity(elem_count);
        let data_slice = global_data.first_half;
        for _ in 0..elem_count {
            if data_offset + 1 >= data_slice.len() { break; }
            let len = _to_u8(data_slice, data_offset) % 17;
            data_offset += 1;
            let end = data_offset + len as usize;
            let s = _to_str(data_slice, data_offset, end);
            t_1.push(CustomType0(String::from(s)));
            data_offset = end;
        }
        let trunc_len = _to_usize(global_data.second_half, 0) % (elem_count + 1);
        t_1.truncate(trunc_len);
        let t_134 = &t_1[..];
        
        let mut sdq = slice_deque::SliceDeque::from(t_134);
        deques.push(sdq);
        
        let elem_count2 = _to_usize(global_data.second_half, 8) % 65;
        let len = _to_u8(global_data.second_half, 16) % 17;
        let s = _to_str(global_data.second_half, 17, 17 + len as usize);
        let filler = CustomType0(String::from(s));
        let sdq2 = slice_deque::from_elem(filler, elem_count2);
        deques.push(sdq2);
        
        let op_count = _to_usize(global_data.first_half, 8) % 16;
        for _ in 0..op_count {
            match _to_u8(global_data.second_half, 64) % 7 {
                0 => {
                    let len = _to_usize(global_data.second_half, 128);
                    deques[0].truncate(len);
                }
                1 => {
                    let elem = deques[0].pop_front();
                    println!("{:?}", elem.as_ref());
                }
                2 => {
                    let len = _to_usize(global_data.second_half, 256);
                    deques[1].resize(len, CustomType0(String::new()));
                }
                3 => {
                    let slice = deques[0].as_slice();
                    let mut new_sdq = slice_deque::SliceDeque::from(slice);
                    deques.push(new_sdq);
                }
                4 => {
                    let mut new_sdq = slice_deque::SliceDeque::with_capacity(_to_usize(global_data.second_half, 384) % 65);
                    new_sdq.extend(deques[1].as_slice().iter().cloned());
                    deques.push(new_sdq);
                }
                5 => {
                    println!("{:?}", deques[0].front());
                    println!("{:?}", deques[0].back_mut());
                }
                _ => {
                    let mut sdq_new = slice_deque::SliceDeque::new();
                    sdq_new.extend_from_slice(deques[0].as_slice());
                    deques.push(sdq_new);
                }
            }
        }
        
        let iter_deque: SliceDeque<_> = t_1.clone().into_iter().collect();
        println!("Eq check: {}", deques[0] == iter_deque);
        let array_eq_check = deques[1].as_slice() == [CustomType0(String::new())].as_slice();
        println!("Array eq: {}", array_eq_check);
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