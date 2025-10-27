#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use slice_deque::*;
use global_data::*;
use std::ops::{Deref, DerefMut};

#[derive(Debug)]
struct CustomType0(String);

impl std::clone::Clone for CustomType0 {
    fn clone(&self) -> Self {
        let global_data = get_global_data();
        let custom_impl_num = _to_usize(global_data.first_half, 1);
        let custom_impl_inst_num = self.0.len();
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0 {
            panic!("INTENTIONAL PANIC!");
        }
        let source_data = match selector {
            1 => global_data.first_half,
            _ => global_data.second_half,
        };
        let str_len = _to_u8(source_data, 9) % 17;
        let t_str = _to_str(source_data, 10, 10 + str_len as usize);
        CustomType0(String::from(t_str))
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

        let mut deque = match constructor_sel {
            0 => {
                let elem_count = (_to_u8(global_data.first_half, offset) % 65) as usize;
                offset += 1;
                let mut vec = Vec::with_capacity(elem_count);
                for _ in 0..elem_count {
                    let str_len = _to_u8(global_data.first_half, offset) % 17;
                    offset += 1;
                    let str = _to_str(global_data.first_half, offset, offset + str_len as usize);
                    offset += str_len as usize;
                    vec.push(CustomType0(String::from(str)));
                }
                SliceDeque::from(vec.as_mut_slice())
            }
            1 => {
                let capacity = _to_u8(global_data.first_half, offset) % 65;
                offset += 1;
                SliceDeque::with_capacity(capacity as usize)
            }
            2 => {
                let elem = {
                    let str_len = _to_u8(global_data.first_half, offset) % 17;
                    offset += 1;
                    let str = _to_str(global_data.first_half, offset, offset + str_len as usize);
                    offset += str_len as usize;
                    CustomType0(String::from(str))
                };
                let count = _to_u8(global_data.first_half, offset) % 65;
                offset += 1;
                slice_deque::from_elem(elem, count as usize)
            }
            _ => SliceDeque::new()
        };

        let op_count = _to_u8(global_data.first_half, offset) % 32;
        offset += 1;

        for _ in 0..op_count {
            let op = _to_u8(global_data.first_half, offset) % 7;
            offset += 1;

            match op {
                0 => {
                    let str_len = _to_u8(global_data.first_half, offset) % 17;
                    offset += 1;
                    let str = _to_str(global_data.first_half, offset, offset + str_len as usize);
                    offset += str_len as usize;
                    deque.push_back(CustomType0(String::from(str)));
                }
                1 => {
                    let str_len = _to_u8(global_data.first_half, offset) % 17;
                    offset += 1;
                    let str = _to_str(global_data.first_half, offset, offset + str_len as usize);
                    offset += str_len as usize;
                    deque.push_front(CustomType0(String::from(str)));
                }
                2 => {
                    let len = _to_usize(global_data.first_half, offset);
                    offset += std::mem::size_of::<usize>();
                    deque.truncate(len);
                }
                3 => {
                    if let Some(item) = deque.pop_front() {
                        println!("{:?}", item);
                    }
                }
                4 => {
                    if let Some(slice) = deque.as_slice().get(0) {
                        println!("{:?}", slice);
                    }
                }
                5 => {
                    let idx = _to_usize(global_data.first_half, offset);
                    offset += std::mem::size_of::<usize>();
                    if idx < deque.len() {
                        deque.remove(idx);
                    }
                }
                6 => {
                    if let Some(mut_slice) = deque.as_mut_slice().get_mut(0) {
                        *mut_slice = CustomType0(String::new());
                    }
                }
                _ => {}
            }
            let _ = deque.front().map(|f| println!("{:?}", f));
        }

        let _ = deque.front();
        let _ = deque.back();
        let _ = deque.len();
        let _ = deque.as_slices();
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