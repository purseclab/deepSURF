#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use stackvector::*;
use global_data::*;
use std::ops::{Deref, DerefMut, Index, IndexMut};

struct CustomType3(String);
struct CustomType4(String);
struct CustomType2(String);
struct CustomType1(String);

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
        (t_6, t_8)
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
        let mut t_10 = (_to_u8(GLOBAL_DATA, 73) as usize) % 17;
        let t_11 = _to_str(GLOBAL_DATA, 74, 74 + t_10);
        let t_12 = String::from(t_11);
        Some(CustomType1(t_12))
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
        let mut t_15 = (_to_u8(GLOBAL_DATA, 98) as usize) % 17;
        let t_16 = _to_str(GLOBAL_DATA, 99, 99 + t_15);
        CustomType3(String::from(t_16))
    }
}

impl std::cmp::PartialEq for CustomType4 {
    fn eq(&self, _: &Self) -> bool {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 132);
        let custom_impl_inst_num = self.0.len();
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0 {
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector {
            1 => global_data.first_half,
            _ => global_data.second_half,
        };
        _to_bool(GLOBAL_DATA, 140)
    }
}

fn _custom_fn0(_: &mut CustomType1) -> CustomType4 {
    let global_data = get_global_data();
    let GLOBAL_DATA = global_data.first_half;
    let t_26 = _to_u8(GLOBAL_DATA, 141);
    if t_26 % 2 == 0 {
        panic!("INTENTIONAL PANIC!");
    }
    let mut t_27 = (_to_u8(GLOBAL_DATA, 142) as usize) % 17;
    let t_28 = _to_str(GLOBAL_DATA, 143, 143 + t_27);
    CustomType4(String::from(t_28))
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 768 {return;}
        set_global_data(data);
        let global_data = get_global_data();
        let first_half = global_data.first_half;
        
        let ctor_selector = _to_u8(first_half, 0) % 3;
        let mut stack_vec = match ctor_selector {
            0 => {
                let t_20_str = _to_str(first_half, 1, 50);
                let t_22 = CustomType2(t_20_str.to_string());
                StackVec::<[CustomType1; 64]>::from_iter(t_22)
            },
            1 => {
                let elem_count = _to_usize(first_half, 1) % 65;
                let mut items = Vec::new();
                for i in 0..elem_count {
                    let offset = 2 + i * 3;
                    let len = (_to_u8(first_half, offset) as usize) % 17;
                    let s = _to_str(first_half, offset + 1, offset + 1 + len);
                    items.push(CustomType1(s.to_string()));
                }
                let mut svec = StackVec::<[CustomType1; 64]>::new();
                for item in items {
                    svec.push(item);
                }
                svec
            },
            _ => StackVec::<[CustomType1; 64]>::new(),
        };

        let ops = _to_usize(first_half, 200) % 10 + 5;
        for i in 0..ops {
            let op_byte = _to_u8(first_half, 210 + i as usize);
            match op_byte % 7 {
                0 => {
                    let idx = _to_usize(first_half, 220 + i * 4);
                    let elem_len = (_to_u8(first_half, 224 + i * 4) as usize) % 17;
                    let s = _to_str(first_half, 225 + i * 4, 225 + i * 4 + elem_len);
                    stack_vec.insert(idx, CustomType1(s.to_string()));
                },
                1 => {
                    let idx = _to_usize(first_half, 260 + i * 4);
                    if let Some(_) = stack_vec.get(idx) {
                        println!("{:?}", stack_vec[idx].0);
                    }
                },
                2 => {
                    let _ = stack_vec.pop();
                },
                3 => {
                    let len = _to_usize(first_half, 300 + i * 4);
                    stack_vec.truncate(len);
                },
                4 => {
                    stack_vec.dedup_by_key(_custom_fn0);
                },
                5 => {
                    let idx = _to_usize(first_half, 350 + i * 4);
                    if let Some(slice) = stack_vec.get(..idx) {
                        println!("Slice len: {}", slice.len());
                    }
                },
                _ => {
                    let drain = stack_vec.drain();
                    for elem in drain {
                        println!("Drained: {}", elem.0);
                    }
                },
            };
        }

        let key_fn = _custom_fn0;
        stack_vec.dedup_by_key(key_fn);
        println!("Final len: {}", stack_vec.len());
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