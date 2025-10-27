#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use endian_trait::*;
use global_data::*;
use std::char;
use std::str::FromStr;

struct CustomType0(String);

impl Endian for CustomType0 {
    fn to_le(self) -> Self {
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
        let t_2 = _to_u8(GLOBAL_DATA, 9) % 17;
        let t_3 = _to_str(GLOBAL_DATA, 10, 10 + t_2 as usize);
        CustomType0(t_3.to_string())
    }

    fn from_be(self) -> Self {
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
        let t_6 = _to_u8(GLOBAL_DATA, 34) % 17;
        let t_7 = _to_str(GLOBAL_DATA, 35, 35 + t_6 as usize);
        CustomType0(t_7.to_string())
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
        let t_10 = _to_u8(GLOBAL_DATA, 59) % 17;
        let t_11 = _to_str(GLOBAL_DATA, 60, 60 + t_10 as usize);
        CustomType0(t_11.to_string())
    }

    fn from_le(self) -> Self {
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
        let t_14 = _to_u8(GLOBAL_DATA, 84) % 17;
        let t_15 = _to_str(GLOBAL_DATA, 85, 85 + t_14 as usize);
        CustomType0(t_15.to_string())
    }
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 4096 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let mut data_index = 0;
        let num_custom = _to_usize(GLOBAL_DATA, 0) % 65;
        data_index += 1;
        let mut custom_vec = Vec::with_capacity(num_custom);
        for _ in 0..num_custom {
            if data_index >= GLOBAL_DATA.len() { break; }
            let str_len = _to_u8(GLOBAL_DATA, data_index) as usize % 17;
            data_index += 1;
            let start = data_index;
            let end = start + str_len;
            if end > GLOBAL_DATA.len() { break; }
            let s = _to_str(GLOBAL_DATA, start, end);
            custom_vec.push(CustomType0(s.to_string()));
            data_index = end;
        }

        let num_u16 = _to_usize(GLOBAL_DATA, data_index) % 65;
        data_index += 1;
        let mut u16_vec = Vec::with_capacity(num_u16);
        for _ in 0..num_u16 {
            if data_index + 2 > GLOBAL_DATA.len() { break; }
            u16_vec.push(_to_u16(GLOBAL_DATA, data_index));
            data_index += 2;
        }

        let num_chars = _to_usize(GLOBAL_DATA, data_index) % 65;
        data_index += 1;
        let mut char_vec = Vec::with_capacity(num_chars);
        for _ in 0..num_chars {
            if data_index + 4 > GLOBAL_DATA.len() { break; }
            let val = _to_u32(GLOBAL_DATA, data_index);
            char_vec.push(char::from_u32(val).unwrap_or('\0'));
            data_index += 4;
        }

        let num_ops = _to_usize(GLOBAL_DATA, data_index) % 100;
        data_index += 1;

        for _ in 0..num_ops {
            if data_index >= GLOBAL_DATA.len() { break; }
            let op_code = _to_u8(GLOBAL_DATA, data_index);
            data_index += 1;
            let ds = op_code % 3;
            let method = (op_code >> 2) % 4;

            match ds {
                0 => if !custom_vec.is_empty() {
                    let slice = &mut custom_vec[..];
                    match method {
                        0 => slice.to_be(),
                        1 => slice.to_le(),
                        2 => slice.from_be(),
                        3 => slice.from_le(),
                        _ => slice,
                    };
                    let _ = &slice[0].0;
                },
                1 => if !u16_vec.is_empty() {
                    let slice = &mut u16_vec[..];
                    match method {
                        0 => slice.to_be(),
                        1 => slice.to_le(),
                        2 => slice.from_be(),
                        3 => slice.from_le(),
                        _ => slice,
                    };
                    let _ = slice[0];
                },
                2 => if !char_vec.is_empty() {
                    let slice = &mut char_vec[..];
                    match method {
                        0 => slice.to_be(),
                        1 => slice.to_le(),
                        2 => slice.from_be(),
                        3 => slice.from_le(),
                        _ => slice,
                    };
                    let _ = slice[0];
                },
                _ => (),
            }
        }

        if !custom_vec.is_empty() {
            let slice = &mut custom_vec[..];
            slice.to_be();
            let _ = &slice[0].0;
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