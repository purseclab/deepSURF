#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use stackvector::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

#[derive(Clone, Debug)]
struct CustomType1(String);
struct CustomType3(String);
struct CustomType2(String);

impl std::iter::Iterator for CustomType3 {
    type Item = CustomType1;
    
    fn size_hint(&self) -> (usize, Option<usize>) {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 41);
        let custom_impl_inst_num = self.0.len();
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0 {
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector {
            1 => global_data.first_half,
            _ => global_data.second_half,
        };
        let t_6 = _to_usize(GLOBAL_DATA, 49);
        let t_7 = _to_usize(GLOBAL_DATA, 57);
        let t_8 = Some(t_7);
        let t_9 = (t_6, t_8);
        return t_9;
    }
    
    fn next(&mut self) -> Option<Self::Item> {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 65);
        let custom_impl_inst_num = self.0.len();
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0 {
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector {
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
        if selector == 0 {
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector {
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

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 2048 { return; }
        set_global_data(data);
        let global_data = get_global_data();

        let op_count = _to_usize(global_data.first_half, 0) % 65;
        let mut vecs = Vec::new();

        for i in 0..op_count {
            let selector = _to_u8(global_data.first_half, 1 + i) % 4;
            let vec = match selector {
                0 => {
                    let s = _to_str(global_data.first_half, 10 + i * 8, 18 + i * 8);
                    let custom2 = CustomType2(s.to_string());
                    StackVec::<[CustomType1; 64]>::from_iter(custom2)
                }
                1 => {
                    let elem_count = _to_usize(global_data.first_half, 2 + i * 4) % 65;
                    let elem_slice: Vec<CustomType1> = (0..elem_count)
                        .map(|j| {
                            let start = 3 + j * 2;
                            let len = _to_u8(global_data.first_half, start) % 17;
                            let s = _to_str(global_data.first_half, start + 1, start + 1 + len as usize);
                            CustomType1(s.to_string())
                        })
                        .collect();
                    StackVec::from_vec(elem_slice)
                }
                2 => {
                    let len = _to_u8(global_data.first_half, 5 + i * 3) % 17;
                    let s = _to_str(global_data.first_half, 6 + i * 3, 6 + i * 3 + len as usize);
                    let elem = CustomType1(s.to_string());
                    StackVec::from_elem(elem, 64)
                }
                _ => StackVec::<[CustomType1; 64]>::new(),
            };
            vecs.push(vec);
        }

        let mut new_vecs = Vec::new();
        for vec in &mut vecs {
            let ops = _to_usize(global_data.second_half, 0) % 65;
            for j in 0..ops {
                let op = _to_u8(global_data.second_half, 1 + j) % 6;
                match op {
                    0 => {
                        let len = _to_u8(global_data.second_half, 2 + j * 4) % 17;
                        let s = _to_str(global_data.second_half, 3 + j * 4, 3 + j * 4 + len as usize);
                        vec.push(CustomType1(s.to_string()));
                    }
                    1 => {
                        let _ = vec.pop();
                    }
                    2 => {
                        let idx = _to_usize(global_data.second_half, 2 + j * 4);
                        let len = _to_u8(global_data.second_half, 3 + j * 4) % 17;
                        let s = _to_str(global_data.second_half, 4 + j * 4, 4 + j * 4 + len as usize);
                        vec.insert(idx, CustomType1(s.to_string()));
                    }
                    3 => {
                        let mut drain = vec.drain();
                        while let Some(_) = drain.next() {}
                    }
                    4 => {
                        let cloned = vec.clone();
                        new_vecs.push(cloned);
                    }
                    5 => {
                        let slice = vec.as_slice();
                        println!("{:?}", slice);
                    }
                    _ => {}
                }
            }
        }
        vecs.extend(new_vecs);
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