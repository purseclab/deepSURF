#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

#[derive(Debug, Clone)]
struct CustomType1(String);
#[derive(Debug)]
struct CustomType2(String);
#[derive(Debug)]
struct CustomType3(String);

impl std::iter::Iterator for CustomType3 {
    type Item = CustomType1;

    fn size_hint(&self) -> (usize, Option<usize>) {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 57);
        let custom_impl_inst_num = self.0.len();
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0 {
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector {
            1 => global_data.first_half,
            _ => global_data.second_half,
        };
        let t_10 = _to_usize(GLOBAL_DATA, 65);
        let t_11 = _to_usize(GLOBAL_DATA, 73);
        (t_10, Some(t_11))
    }

    fn next(&mut self) -> Option<Self::Item> {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 81);
        let custom_impl_inst_num = self.0.len();
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0 {
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector {
            1 => global_data.first_half,
            _ => global_data.second_half,
        };
        let t_14 = _to_u8(GLOBAL_DATA, 89) % 17;
        let t_15 = _to_str(GLOBAL_DATA, 90, 90 + t_14 as usize);
        Some(CustomType1(String::from(t_15)))
    }
}

impl std::iter::IntoIterator for CustomType2 {
    type Item = CustomType1;
    type IntoIter = CustomType3;

    fn into_iter(self) -> Self::IntoIter {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 106);
        let custom_impl_inst_num = self.0.len();
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0 {
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector {
            1 => global_data.first_half,
            _ => global_data.second_half,
        };
        let t_19 = _to_u8(GLOBAL_DATA, 114) % 17;
        let t_20 = _to_str(GLOBAL_DATA, 115, 115 + t_19 as usize);
        CustomType3(String::from(t_20))
    }
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 1024 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let creation_selector = _to_u8(GLOBAL_DATA, 0) % 4;
        let mut sv: SmallVec<[CustomType1; 16]> = match creation_selector {
            0 => SmallVec::new(),
            1 => SmallVec::with_capacity(_to_usize(GLOBAL_DATA, 1) % 65),
            2 => {
                let elem = CustomType1(String::from(_to_str(GLOBAL_DATA, 2, 10)));
                SmallVec::from_elem(elem, _to_usize(GLOBAL_DATA, 11) % 65)
            }
            _ => SmallVec::from_vec(vec![
                CustomType1(String::from(_to_str(GLOBAL_DATA, 20, 25))),
                CustomType1(String::from(_to_str(GLOBAL_DATA, 26, 30)))
            ]),
        };

        let op_count = _to_usize(GLOBAL_DATA, 50) % 16;
        for i in 0..op_count {
            let op_byte = _to_u8(GLOBAL_DATA, 60 + i);
            match op_byte % 7 {
                0 => {
                    let str_len = _to_u8(GLOBAL_DATA, 70 + i) % 17;
                    let s = _to_str(GLOBAL_DATA, 80 + i, 80 + i + str_len as usize);
                    sv.push(CustomType1(String::from(s)));
                }
                1 => {
                    sv.pop();
                }
                2 => {
                    sv.truncate(_to_usize(GLOBAL_DATA, 100 + i) % 65);
                }
                3 => {
                    if !sv.is_empty() {
                        println!("Removed: {:?}", sv.remove(_to_usize(GLOBAL_DATA, 120 + i) % sv.len()));
                    }
                }
                4 => {
                    let insert_index = _to_usize(GLOBAL_DATA, 130 + i) % (sv.len() + 1);
                    let t_23 = _to_u8(GLOBAL_DATA, 140 + i) % 17;
                    let t_24 = _to_str(GLOBAL_DATA, 141 + i, 141 + i + t_23 as usize);
                    sv.insert(insert_index, CustomType1(String::from(t_24)));
                }
                5 => {
                    let mut drain = sv.drain();
                    while let Some(item) = drain.next() {
                        println!("Drained: {:?}", item);
                    }
                }
                6 => {
                    println!("Vector contents: {:?}", sv.as_slice());
                }
                _ => unreachable!(),
            }
        }

        let insert_index = _to_usize(GLOBAL_DATA, 200);
        let t_23 = _to_u8(GLOBAL_DATA, 210) % 17;
        let t_24 = _to_str(GLOBAL_DATA, 211, 211 + t_23 as usize);
        let t_26 = CustomType2(String::from(t_24));
        sv.insert_many(insert_index, t_26);
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