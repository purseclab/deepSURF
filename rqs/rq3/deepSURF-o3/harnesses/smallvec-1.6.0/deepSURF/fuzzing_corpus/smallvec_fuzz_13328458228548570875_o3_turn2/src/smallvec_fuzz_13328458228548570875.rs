#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::hash::{Hash, Hasher};

#[derive(Debug)]
struct CustomType1(usize);

struct CustomType2(String);

impl Hash for CustomType1 {
    fn hash<H: Hasher>(&self, _state: &mut H) {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 0);
        let custom_impl_inst_num = self.0;
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0 {
            panic!("INTENTIONAL PANIC!");
        }
        let _GLOBAL_DATA = match selector {
            1 => global_data.first_half,
            _ => global_data.second_half,
        };
    }
}

impl Hasher for CustomType2 {
    fn write(&mut self, _bytes: &[u8]) {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 17);
        let custom_impl_inst_num = self.0.len();
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0 {
            panic!("INTENTIONAL PANIC!");
        }
        let _GLOBAL_DATA = match selector {
            1 => global_data.first_half,
            _ => global_data.second_half,
        };
    }

    fn finish(&self) -> u64 {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 25);
        let custom_impl_inst_num = self.0.len();
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0 {
            panic!("INTENTIONAL PANIC!");
        }
        let _GLOBAL_DATA = match selector {
            1 => global_data.first_half,
            _ => global_data.second_half,
        };
        _to_u64(GLOBAL_DATA, 33)
    }
}

impl Clone for CustomType1 {
    fn clone(&self) -> Self {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 42);
        let custom_impl_inst_num = self.0;
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0 {
            panic!("INTENTIONAL PANIC!");
        }
        let _GLOBAL_DATA = match selector {
            1 => global_data.first_half,
            _ => global_data.second_half,
        };
        let t_5 = _to_usize(GLOBAL_DATA, 50);
        CustomType1(t_5)
    }
}

impl core::marker::Copy for CustomType1 {}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 900 {
            return;
        }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let mut vals: Vec<CustomType1> = Vec::with_capacity(65);
        let vec_len = (_to_u8(GLOBAL_DATA, 1) % 65) as usize;
        for i in 0..vec_len {
            let idx = 2 + i * 8;
            if idx + 8 <= GLOBAL_DATA.len() {
                let v = _to_usize(GLOBAL_DATA, idx);
                vals.push(CustomType1(v));
            }
        }

        let selector = _to_u8(GLOBAL_DATA, 5) % 5;
        let mut small: SmallVec<[CustomType1; 32]> = match selector {
            0 => SmallVec::new(),
            1 => {
                let cap = _to_usize(GLOBAL_DATA, 6) % 33;
                SmallVec::with_capacity(cap)
            }
            2 => {
                let elem = CustomType1(_to_usize(GLOBAL_DATA, 14));
                let n = _to_usize(GLOBAL_DATA, 22) % 33;
                SmallVec::from_elem(elem, n)
            }
            3 => SmallVec::from_slice(&vals),
            _ => SmallVec::from_vec(vals.clone()),
        };

        let ops = _to_u8(GLOBAL_DATA, 30) % 20;
        for i in 0..ops {
            let op = _to_u8(GLOBAL_DATA, 31 + i as usize) % 10;
            match op {
                0 => {
                    let v = CustomType1(_to_usize(GLOBAL_DATA, 40 + i as usize * 8));
                    small.push(v);
                }
                1 => {
                    small.pop();
                }
                2 => {
                    if !small.is_empty() {
                        let idx = _to_usize(GLOBAL_DATA, 48 + i as usize * 8) % small.len();
                        let v = CustomType1(_to_usize(GLOBAL_DATA, 56 + i as usize * 8));
                        small.insert(idx, v);
                    }
                }
                3 => {
                    if !small.is_empty() {
                        let idx = _to_usize(GLOBAL_DATA, 64 + i as usize * 8) % small.len();
                        small.remove(idx);
                    }
                }
                4 => {
                    let new_len = _to_usize(GLOBAL_DATA, 72 + i as usize * 8) % 33;
                    small.truncate(new_len);
                }
                5 => {
                    let _ = small.len();
                }
                6 => {
                    let _ = small.is_empty();
                }
                7 => {
                    let _slice = small.as_slice();
                }
                8 => {
                    small.clear();
                }
                _ => {}
            }
        }

        let text_len = _to_u8(GLOBAL_DATA, 400) % 17;
        let txt = _to_str(GLOBAL_DATA, 401, 401 + text_len as usize);
        let mut hasher = CustomType2(String::from(txt));

        small.hash(&mut hasher);

        if !small.is_empty() {
            let end = _to_usize(GLOBAL_DATA, 500) % small.len();
            let mut dr = small.drain(0..end);
            if let Some(item) = dr.next() {
                println!("{:?}", item);
            }
        }

        small.hash(&mut hasher);

        let _ = small.capacity();
        let slice = small.as_slice();
        if !slice.is_empty() {
            println!("{:?}", slice[0]);
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