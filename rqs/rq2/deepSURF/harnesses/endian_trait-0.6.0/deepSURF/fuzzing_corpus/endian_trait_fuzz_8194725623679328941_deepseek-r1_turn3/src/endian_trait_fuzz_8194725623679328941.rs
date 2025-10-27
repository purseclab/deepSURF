#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use endian_trait::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

struct CustomType0(String);

impl Endian for CustomType0 {
    fn from_be(self) -> Self {
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
        let len = _to_u8(GLOBAL_DATA, 9) % 17;
        let s = _to_str(GLOBAL_DATA, 10, 10 + len as usize);
        CustomType0(s.to_string())
    }

    fn from_le(self) -> Self {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 26);
        let custom_impl_inst_num = self.0.len();
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0 {
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector {
            1 => global_data.first_half,
            _ => global_data.second_half,
        };
        let len = _to_u8(GLOBAL_DATA, 34) % 17;
        let s = _to_str(GLOBAL_DATA, 35, 35 + len as usize);
        CustomType0(s.to_string())
    }

    fn to_be(self) -> Self {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 51);
        let custom_impl_inst_num = self.0.len();
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0 {
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector {
            1 => global_data.first_half,
            _ => global_data.second_half,
        };
        let len = _to_u8(GLOBAL_DATA, 59) % 17;
        let s = _to_str(GLOBAL_DATA, 60, 60 + len as usize);
        CustomType0(s.to_string())
    }

    fn to_le(self) -> Self {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 76);
        let custom_impl_inst_num = self.0.len();
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0 {
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector {
            1 => global_data.first_half,
            _ => global_data.second_half,
        };
        let len = _to_u8(GLOBAL_DATA, 84) % 17;
        let s = _to_str(GLOBAL_DATA, 85, 85 + len as usize);
        CustomType0(s.to_string())
    }
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 4000 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let mut vec_custom = Vec::new();
        let custom_count = (_to_u8(GLOBAL_DATA, 0) % 65) as usize;
        let mut data_offset = 1;
        for _ in 0..custom_count {
            if data_offset + 17 >= GLOBAL_DATA.len() { break; }
            let len = _to_u8(GLOBAL_DATA, data_offset) % 17;
            let s = _to_str(GLOBAL_DATA, data_offset + 1, data_offset + 1 + len as usize);
            vec_custom.push(CustomType0(s.to_string()));
            data_offset += 17;
        }

        let mut vec_u16 = Vec::new();
        let u16_count = (_to_u8(GLOBAL_DATA, 1000) % 65) as usize;
        for i in 0..u16_count {
            let offset = 1001 + i * 2;
            if offset + 1 >= GLOBAL_DATA.len() { break; }
            vec_u16.push(_to_u16(GLOBAL_DATA, offset));
        }

        let mut vec_char = Vec::new();
        let char_count = (_to_u8(GLOBAL_DATA, 2000) % 65) as usize;
        for i in 0..char_count {
            let offset = 2001 + i * 4;
            if offset + 3 >= GLOBAL_DATA.len() { break; }
            vec_char.push(_to_char(GLOBAL_DATA, offset));
        }

        let ops_count = _to_u8(GLOBAL_DATA, 3000) % 10;
        for op_idx in 0..ops_count {
            let selector = _to_u8(GLOBAL_DATA, 3001 + op_idx as usize) % 8;
            match selector {
                0 => {
                    let slice = &mut vec_custom[..];
                    slice.to_le();
                    println!("Custom slice: {:?}", slice.iter().map(|x| &x.0).collect::<Vec<_>>());
                }
                1 => {
                    let slice = &mut vec_u16[..];
                    slice.from_be().to_le();
                    for elem in slice.iter_mut() {
                        *elem = elem.to_le();
                        println!("u16: {}", elem);
                    }
                }
                2 => {
                    let slice = &mut vec_char[..];
                    slice.to_le().from_le();
                    for elem in slice {
                        *elem = elem.from_be();
                        println!("char: {}", elem);
                    }
                }
                3 => {
                    let slice = &mut vec_custom[..];
                    slice.from_le().to_be();
                    slice.to_le();
                }
                4 => {
                    let slice = &mut vec_u16[..];
                    slice.to_be();
                    for elem in slice {
                        *elem = elem.from_le();
                    }
                }
                5 => {
                    let slice = &mut vec_char[..];
                    slice.from_be();
                    println!("Chars: {:?}", slice);
                }
                6 => {
                    if let Some(elem) = vec_custom.get_mut(0) {
                        let v = std::mem::replace(elem, CustomType0(String::new()));
                        *elem = v.to_le();
                    }
                }
                7 => {
                    let mut s = vec_u16.as_mut_slice();
                    s.to_le();
                    s.from_be();
                }
                _ => {}
            }
        }

        let mut mixed = vec_custom.into_iter().chain(vec_u16.into_iter().map(|x| CustomType0(x.to_string()))).collect::<Vec<_>>();
        if let Some(slice) = mixed.get_mut(..) {
            let _ = slice.to_le();
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