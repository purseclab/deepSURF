#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use stackvector::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

#[derive(Debug)]
struct CustomType3(String);
#[derive(Debug, Clone)]
struct CustomType1(String);
#[derive(Debug)]
struct CustomType2(String);

impl std::iter::Iterator for CustomType3 {
    type Item = CustomType1;
    
    fn size_hint(&self) -> (usize, Option<usize>) {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 41);
        let custom_impl_inst_num = self.0.len();
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0{
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector{
            1 => global_data.first_half,
            _ => global_data.second_half,
        };
        let t_6 = _to_usize(GLOBAL_DATA, 49);
        let t_7 = _to_usize(GLOBAL_DATA, 57);
        let t_8 = Some(t_7);
        return (t_6, t_8);
    }
    
    fn next(&mut self) -> Option<Self::Item> {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 65);
        let custom_impl_inst_num = self.0.len();
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0{
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector{
            1 => global_data.first_half,
            _ => global_data.second_half,
        };
        let mut t_10 = _to_u8(GLOBAL_DATA, 73) % 17;
        let t_11 = _to_str(GLOBAL_DATA, 74, 74 + t_10 as usize);
        let t_12 = String::from(t_11);
        let t_13 = CustomType1(t_12);
        let t_14 = Some(t_13);
        return t_14;
    }
}

impl std::iter::IntoIterator for CustomType2 {
    type Item = CustomType1;
    type IntoIter = CustomType3;
    
    fn into_iter(self) -> Self::IntoIter {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 90);
        let custom_impl_inst_num = self.0.len();
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0{
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector{
            1 => global_data.first_half,
            _ => global_data.second_half,
        };
        let mut t_15 = _to_u8(GLOBAL_DATA, 98) % 17;
        let t_16 = _to_str(GLOBAL_DATA, 99, 99 + t_15 as usize);
        let t_17 = String::from(t_16);
        let t_18 = CustomType3(t_17);
        return t_18;
    }
}

fn main (){
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 1024 {return;}
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        
        let constructor_selector = _to_u8(GLOBAL_DATA, 0) % 4;
        let mut stack_vec = match constructor_selector {
            0 => StackVec::<[CustomType1; 16]>::new(),
            1 => {
                let count = _to_usize(GLOBAL_DATA, 1) % 16;
                let mut offset = 2;
                let mut elements = Vec::new();
                for _ in 0..count {
                    let len = _to_u8(GLOBAL_DATA, offset) % 17;
                    let s = _to_str(GLOBAL_DATA, offset+1, offset+1 + len as usize);
                    elements.push(CustomType1(s.to_string()));
                    offset += 1 + len as usize;
                }
                StackVec::from_vec(elements)
            },
            2 => {
                let len = _to_u8(GLOBAL_DATA, 1) % 17;
                let s = _to_str(GLOBAL_DATA, 2, 2 + len as usize);
                let count = _to_usize(GLOBAL_DATA, 2 + len as usize) % 16;
                let mut stack_vec = StackVec::<[CustomType1; 16]>::new();
                for _ in 0..count {
                    stack_vec.push(CustomType1(s.to_string()));
                }
                stack_vec
            },
            _ => {
                let len = _to_u8(GLOBAL_DATA, 1) % 17;
                let s = _to_str(GLOBAL_DATA, 2, 2 + len as usize);
                StackVec::from_iter(CustomType2(s.to_string()))
            },
        };

        let num_ops = _to_usize(GLOBAL_DATA, 100) % 65;
        let mut op_offset = 101;
        
        for _ in 0..num_ops {
            if op_offset +8 >= GLOBAL_DATA.len() { break; }
            let op_selector = _to_u8(GLOBAL_DATA, op_offset) % 8;
            op_offset +=1;
            
            match op_selector {
                0 => {
                    let len = _to_u8(GLOBAL_DATA, op_offset) % 17;
                    let s = _to_str(GLOBAL_DATA, op_offset+1, op_offset+1 + len as usize);
                    stack_vec.push(CustomType1(s.to_string()));
                    op_offset +=1 + len as usize;
                },
                1 => { let _ = stack_vec.pop(); },
                2 => {
                    let idx = _to_usize(GLOBAL_DATA, op_offset);
                    let len = _to_u8(GLOBAL_DATA, op_offset+8) % 17;
                    let s = _to_str(GLOBAL_DATA, op_offset+9, op_offset+9 + len as usize);
                    stack_vec.insert(idx, CustomType1(s.to_string()));
                    op_offset +=9 + len as usize;
                },
                3 => stack_vec.clear(),
                4 => { let _ = stack_vec.as_slice(); },
                5 => { let _ = stack_vec.as_mut_slice(); },
                6 => {
                    let idx = _to_usize(GLOBAL_DATA, op_offset);
                    let _ = stack_vec.swap_remove(idx);
                    op_offset +=8;
                },
                7 => {
                    let len = _to_usize(GLOBAL_DATA, op_offset);
                    stack_vec.truncate(len);
                    op_offset +=8;
                },
                _ => unreachable!(),
            }
            
            println!("{:?}", stack_vec.as_slice());
            println!("{:?}", stack_vec.as_mut_slice());
        }
    });
}

// Type converter functions omitted as per directions

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