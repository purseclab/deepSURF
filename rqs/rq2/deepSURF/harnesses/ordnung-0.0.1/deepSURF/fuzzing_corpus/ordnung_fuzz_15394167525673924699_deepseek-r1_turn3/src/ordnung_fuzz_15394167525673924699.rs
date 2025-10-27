#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use ordnung::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

#[derive(Eq, Hash, PartialEq)]
struct CustomType0(String);
#[derive(PartialEq)]
struct CustomType1(String);
struct CustomType2(String);
struct CustomType5(String);
struct CustomType3(String);
struct CustomType4(Vec<(CustomType0, CustomType1)>);

impl core::iter::IntoIterator for CustomType2 {
    type Item = (CustomType0, CustomType1);
    type IntoIter = std::vec::IntoIter<(CustomType0, CustomType1)>;
    
    fn into_iter(self) -> Self::IntoIter {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let mut items = Vec::new();
        let mut offset = 0;

        while offset + 4 < GLOBAL_DATA.len() {
            let k_len = (_to_u8(GLOBAL_DATA, offset) % 65) as usize;
            offset += 1;
            if offset + k_len > GLOBAL_DATA.len() { break; }
            let k = CustomType0(String::from(_to_str(GLOBAL_DATA, offset, offset + k_len)));
            offset += k_len;

            let v_len = (_to_u8(GLOBAL_DATA, offset) % 65) as usize;
            offset += 1;
            if offset + v_len > GLOBAL_DATA.len() { break; }
            let v = CustomType1(String::from(_to_str(GLOBAL_DATA, offset, offset + v_len)));
            offset += v_len;

            items.push((k, v));
        }
        
        items.into_iter()
    }
}

fn main (){
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 2048 {return;}
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let mut offset = 0;

        let mut map1 = ordnung::Map::<CustomType0, CustomType1>::new();
        let cap = _to_usize(GLOBAL_DATA, offset);
        offset += 8;
        let mut map2 = ordnung::Map::<CustomType0, CustomType1>::with_capacity(cap);
        let t_18 = CustomType2(String::from(""));
        let map3 = ordnung::Map::from_iter(t_18);

        let op_count = _to_u8(GLOBAL_DATA, offset) % 65;
        offset += 1;

        for _ in 0..op_count {
            if offset + 1 > GLOBAL_DATA.len() { break; }
            let op_byte = _to_u8(GLOBAL_DATA, offset) % 7;
            offset += 1;

            match op_byte {
                0 => {
                    let target = if _to_u8(GLOBAL_DATA, offset) % 2 == 0 { &mut map1 } else { &mut map2 };
                    offset += 1;
                    let k_len = _to_u8(GLOBAL_DATA, offset) % 65;
                    offset += 1;
                    let k = CustomType0(String::from(_to_str(GLOBAL_DATA, offset, offset + k_len as usize)));
                    offset += k_len as usize;
                    let v_len = _to_u8(GLOBAL_DATA, offset) % 65;
                    offset += 1;
                    let v = CustomType1(String::from(_to_str(GLOBAL_DATA, offset, offset + v_len as usize)));
                    offset += v_len as usize;
                    target.insert(k, v);
                }
                1 => {
                    let target = if _to_u8(GLOBAL_DATA, offset) % 2 == 0 { &mut map1 } else { &mut map2 };
                    offset += 1;
                    let k_len = _to_u8(GLOBAL_DATA, offset) % 65;
                    offset += 1;
                    let k_str = _to_str(GLOBAL_DATA, offset, offset + k_len as usize);
                    offset += k_len as usize;
                    let k = CustomType0(String::from(k_str));
                    target.remove(&k);
                }
                2 => {
                    let target = if _to_u8(GLOBAL_DATA, offset) % 2 == 0 { &mut map1 } else { &mut map2 };
                    offset += 1;
                    target.clear();
                }
                3 => {
                    let target = if _to_u8(GLOBAL_DATA, offset) % 2 == 0 { &map1 } else { &map2 };
                    offset += 1;
                    target.len();
                }
                4 => {
                    let mut target = if _to_u8(GLOBAL_DATA, offset) % 2 == 0 { map1.iter_mut() } else { map2.iter_mut() };
                    offset += 1;
                    for (k, v) in target.by_ref() {
                        let _ = *v;
                    }
                }
                5 => {
                    let target = if _to_u8(GLOBAL_DATA, offset) % 2 == 0 { &map1 } else { &map2 };
                    offset += 1;
                    let k_len = _to_u8(GLOBAL_DATA, offset) % 65;
                    offset += 1;
                    let k_str = _to_str(GLOBAL_DATA, offset, offset + k_len as usize);
                    offset += k_len as usize;
                    let k = CustomType0(String::from(k_str));
                    let _ = target.get(&k);
                }
                _ => {
                    let _ = map1.eq(&map2);
                    let _ = map2.eq(&map3);
                    let _ = map1.eq(&map3);
                }
            }
        }

        let _ = map1.eq(&map2);
        let _ = map2.eq(&map3);
        let _ = map1.eq(&map3);
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