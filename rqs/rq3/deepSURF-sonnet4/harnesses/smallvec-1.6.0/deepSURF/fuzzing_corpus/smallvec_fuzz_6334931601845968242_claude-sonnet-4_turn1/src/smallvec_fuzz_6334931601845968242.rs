#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

struct CustomType0(String);
struct CustomType1(String);

fn main (){
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 280 {return;}
        set_global_data(data);
        let global_data = get_global_data();
        
        let operations_count = _to_u8(global_data.first_half, 0) % 20 + 5;
        
        let mut t_2 = _to_u8(global_data.first_half, 1) % 17;
        let t_3 = _to_str(global_data.first_half, 2, 2 + t_2 as usize);
        let t_4 = String::from(t_3);
        let t_5 = CustomType0(t_4.clone());
        
        let constructor_choice = _to_u8(global_data.first_half, 20) % 4;
        let mut smallvec1 = match constructor_choice {
            0 => {
                let capacity = _to_usize(global_data.first_half, 21);
                smallvec::SmallVec::<[String; 15]>::with_capacity(capacity)
            },
            1 => {
                let vec_size = _to_u8(global_data.first_half, 21) % 12;
                let mut vec = Vec::new();
                for i in 0..vec_size {
                    let s_len = _to_u8(global_data.first_half, 29 + i as usize) % 8;
                    let s = _to_str(global_data.first_half, 41 + i as usize * 8, 41 + i as usize * 8 + s_len as usize);
                    vec.push(String::from(s));
                }
                smallvec::SmallVec::<[String; 15]>::from_vec(vec)
            },
            2 => {
                let elem_len = _to_u8(global_data.first_half, 21) % 8;
                let elem_str = _to_str(global_data.first_half, 22, 22 + elem_len as usize);
                let elem = String::from(elem_str);
                let count = _to_usize(global_data.first_half, 30);
                smallvec::SmallVec::<[String; 15]>::from_elem(elem, count)
            },
            _ => smallvec::SmallVec::<[String; 15]>::new()
        };
        
        let mut smallvec2 = smallvec::SmallVec::<[String; 12]>::new();
        
        for op_idx in 0..operations_count {
            let base_offset = 50 + op_idx as usize * 12;
            let operation = _to_u8(global_data.first_half, base_offset) % 15;
            
            match operation {
                0 => {
                    let index = _to_usize(global_data.first_half, base_offset + 1);
                    if smallvec1.len() > 0 {
                        let removed = smallvec1.swap_remove(index);
                        println!("{:?}", removed);
                    }
                },
                1 => {
                    let str_len = _to_u8(global_data.first_half, base_offset + 1) % 10;
                    let s = _to_str(global_data.first_half, base_offset + 2, base_offset + 2 + str_len as usize);
                    smallvec1.push(String::from(s));
                },
                2 => {
                    if let Some(popped) = smallvec1.pop() {
                        println!("{:?}", popped);
                    }
                },
                3 => {
                    let index = _to_usize(global_data.first_half, base_offset + 1);
                    let str_len = _to_u8(global_data.first_half, base_offset + 9) % 8;
                    let s = _to_str(global_data.first_half, base_offset + 10, base_offset + 10 + str_len as usize);
                    smallvec1.insert(index, String::from(s));
                },
                4 => {
                    let len = _to_usize(global_data.first_half, base_offset + 1);
                    smallvec1.truncate(len);
                },
                5 => {
                    let capacity = _to_usize(global_data.first_half, base_offset + 1);
                    smallvec1.reserve(capacity);
                },
                6 => {
                    smallvec1.clear();
                },
                7 => {
                    let slice_ref = smallvec1.as_slice();
                    if slice_ref.len() > 0 {
                        println!("{:?}", &slice_ref[0]);
                    }
                },
                8 => {
                    let index = _to_usize(global_data.first_half, base_offset + 1);
                    if smallvec1.len() > 0 {
                        let removed = smallvec1.remove(index);
                        println!("{:?}", removed);
                    }
                },
                9 => {
                    smallvec1.append(&mut smallvec2);
                },
                10 => {
                    let clone_result = smallvec1.clone();
                    println!("{:?}", clone_result.len());
                },
                11 => {
                    let range_start = _to_usize(global_data.first_half, base_offset + 1);
                    let range_end = _to_usize(global_data.first_half, base_offset + 9);
                    let mut drain = smallvec1.drain(range_start..range_end);
                    if let Some(item) = drain.next() {
                        println!("{:?}", item);
                    }
                },
                12 => {
                    smallvec1.shrink_to_fit();
                },
                13 => {
                    let into_vec = smallvec1.clone().into_vec();
                    println!("{:?}", into_vec.len());
                },
                _ => {
                    let capacity_info = smallvec1.capacity();
                    println!("{:?}", capacity_info);
                }
            }
        }
        
        if smallvec1.len() > 0 {
            let final_index = _to_usize(global_data.second_half, 0);
            let final_removed = smallvec1.swap_remove(final_index);
            println!("{:?}", final_removed);
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