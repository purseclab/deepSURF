#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::str::FromStr;

#[derive(Debug, Clone)]
struct CustomType1(String);
#[derive(Debug)]
struct CustomType2(String);

impl core::cmp::PartialEq for CustomType2 {
    fn eq(&self, _: &Self) -> bool {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 34);
        let custom_impl_inst_num = self.0.len();
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0 {
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector {
            1 => global_data.first_half,
            _ => global_data.second_half,
        };
        _to_bool(GLOBAL_DATA, 42)
    }
}

fn _custom_fn0(_: &mut CustomType1) -> CustomType2 {
    let global_data = get_global_data();
    let GLOBAL_DATA = global_data.first_half;
    let t_10 = _to_u8(GLOBAL_DATA, 43);
    if t_10 % 2 == 0 {
        panic!("INTENTIONAL PANIC!");
    }
    let t_11 = _to_u8(GLOBAL_DATA, 44) % 17;
    let t_12 = _to_str(GLOBAL_DATA, 45, 45 + t_11 as usize);
    CustomType2(String::from(t_12))
}

type Buf = [CustomType1; 64];

fn build_item(offset: usize) -> CustomType1 {
    let global_data = get_global_data();
    let GLOBAL_DATA = global_data.first_half;
    let len = _to_u8(GLOBAL_DATA, offset) % 17;
    let s = _to_str(GLOBAL_DATA, offset + 1, offset + 1 + len as usize);
    CustomType1(String::from(s))
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 200 {
            return;
        }
        set_global_data(data);
        let global_data = get_global_data();
        let first = global_data.first_half;

        let selector = _to_u8(first, 0);
        let cap = (_to_usize(first, 2) % 65).max(1);
        let elem = build_item(6);
        let n_items = (_to_u8(first, 3) % 17 + 1) as usize;

        let mut temp_vec = Vec::new();
        for j in 0..n_items {
            let offset = 10 + (j * 3) % (first.len() - 20);
            temp_vec.push(build_item(offset));
        }

        let mut sv: SmallVec<Buf> = match selector % 5 {
            0 => SmallVec::new(),
            1 => SmallVec::with_capacity(cap),
            2 => SmallVec::from_elem(elem.clone(), n_items),
            3 => SmallVec::from_iter(temp_vec.clone()),
            _ => SmallVec::from_vec(temp_vec),
        };

        let op_count = (_to_u8(first, 4) % 20) as usize;
        for i in 0..op_count {
            let op_sel = _to_u8(first, 50 + (i % 50)) % 8;
            match op_sel {
                0 => {
                    let item = build_item((i * 5 + 20) % (first.len() - 20));
                    sv.push(item);
                }
                1 => {
                    let _ = sv.pop();
                }
                2 => {
                    if !sv.is_empty() {
                        let idx = _to_usize(first, 60 + i % 10) % sv.len();
                        let _ = sv.remove(idx);
                    }
                }
                3 => {
                    let item = build_item((i * 7 + 25) % (first.len() - 25));
                    let idx = if sv.is_empty() {
                        0
                    } else {
                        _to_usize(first, 70 + i % 10) % sv.len()
                    };
                    sv.insert(idx, item);
                }
                4 => {
                    let len = _to_usize(first, 80 + i % 10) % 65;
                    sv.truncate(len);
                }
                5 => {
                    let additional = _to_usize(first, 90 + i % 10);
                    sv.reserve(additional);
                }
                6 => {
                    let slice = sv.as_slice();
                    println!("{:?}", slice.len());
                }
                7 => {
                    let mut f = |x: &mut CustomType1| _custom_fn0(x);
                    sv.dedup_by_key(&mut f);
                }
                _ => {}
            }
        }

        let mut key_fn = _custom_fn0;
        sv.dedup_by_key(&mut key_fn);

        println!("{:?}", sv.capacity());
        if !sv.is_empty() {
            println!("{:?}", &sv[0].0.len());
        }
        let _clone = sv.clone();
        let _vec = sv.into_vec();
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