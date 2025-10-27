#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;    

use simple_slab::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

struct CustomType0(String);
impl Default for CustomType0 {
    fn default() -> Self {
        CustomType0(String::from(""))
    }
}
impl std::fmt::Debug for CustomType0 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 234 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let t_0 = _to_u8(GLOBAL_DATA, 0);
        let t_1 = _to_u16(GLOBAL_DATA, 1);
        let t_3 = _to_u64(GLOBAL_DATA, 7);
        let t_5 = _to_usize(GLOBAL_DATA, 31);
        let t_6 = _to_i8(GLOBAL_DATA, 39);
        let t_8 = _to_i32(GLOBAL_DATA, 42);
        let t_15 = _to_bool(GLOBAL_DATA, 94);
        let t_16 = _to_str(GLOBAL_DATA, 95, 105);

        let mut slab = match t_0 % 2 {
            0 => Slab::<CustomType0>::new(),
            _ => Slab::with_capacity(t_5 % 65)
        };

        let ops_count = t_0 as usize % 5 + 1;
        for i in 0..ops_count {
            let op_selector = _to_u8(global_data.second_half, i * 3) % 4;
            match op_selector {
                0 => {
                    let elem = CustomType0(format!("{}{}", t_16, t_8));
                    slab.insert(elem);
                    let _ = slab.len();
                }
                1 => {
                    let _ = slab.remove(t_5.wrapping_add(i) as usize);
                }
                2 => {
                    let mut iter = slab.iter();
                    while let Some(item) = iter.next() {
                        println!("{:?}", *item);
                    }
                }
                3 => {
                    let val = CustomType0(t_16.to_string() + &t_3.to_string());
                    slab.insert(val);
                    let _ = slab.iter_mut().next().map(|x| x.0.push_str("FUZZ"));
                }
                _ => {
                    let _ = slab.insert(CustomType0(String::new()));
                }
            }

            if t_15 {
                let _ = slab.insert(CustomType0(t_16.replace('\0', "")));
                let _ = slab.len().wrapping_add(t_6 as usize);
            }
        }

        let drain_count = _to_u8(global_data.second_half, 150) % 3;
        for _ in 0..drain_count {
            let idx = _to_usize(global_data.second_half, 200) % (slab.len() + 1);
            let _ = slab.remove(idx);
        }

        let final_insert = CustomType0(format!("FINAL-{}-{}", t_1, t_8));
        slab.insert(final_insert);
        let _ = slab.len().wrapping_sub(_to_usize(global_data.second_half, 210));
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