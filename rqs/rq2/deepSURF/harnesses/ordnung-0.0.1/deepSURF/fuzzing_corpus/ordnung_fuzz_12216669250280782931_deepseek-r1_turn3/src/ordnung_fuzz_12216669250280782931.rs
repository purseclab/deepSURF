#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use ordnung::*;
use global_data::*;
use std::ops::{Deref, DerefMut};

#[derive(Hash, Eq, PartialEq)]
struct CustomType0(String);
struct CustomType4 {
    data: String,
    current_offset: usize,
}
struct CustomType1(String);
struct CustomType2(String);

impl Iterator for CustomType4 {
    type Item = (CustomType0, CustomType1);

    fn next(&mut self) -> Option<Self::Item> {
        let global_data = get_global_data();
        let data = global_data.first_half;

        if self.current_offset + 2 > data.len() {
            return None;
        }

        let k_len = _to_u8(data, self.current_offset) % 17;
        self.current_offset += 1;
        let k = CustomType0(_to_str(data, self.current_offset, self.current_offset + k_len as usize).to_string());
        self.current_offset += k_len as usize;

        if self.current_offset >= data.len() {
            return None;
        }

        let v_len = _to_u8(data, self.current_offset) % 17;
        self.current_offset += 1;
        let v = CustomType1(_to_str(data, self.current_offset, self.current_offset + v_len as usize).to_string());
        self.current_offset += v_len as usize;

        Some((k, v))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let global_data = get_global_data();
        let data = global_data.first_half;
        let custom_impl_num = _to_usize(data, 0);
        let custom_impl_inst_num = self.data.len();
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0 {
            panic!("INTENTIONAL PANIC!");
        }
        let data = match selector {
            1 => global_data.first_half,
            _ => global_data.second_half,
        };
        (data.len(), Some(data.len()))
    }
}

impl IntoIterator for CustomType2 {
    type Item = (CustomType0, CustomType1);
    type IntoIter = CustomType4;

    fn into_iter(self) -> Self::IntoIter {
        let global_data = get_global_data();
        let data = global_data.first_half;
        CustomType4 {
            data: self.0,
            current_offset: 0,
        }
    }
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 2048 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let mut old_map = {
            let len = _to_u8(GLOBAL_DATA, 0) % 17;
            let s = _to_str(GLOBAL_DATA, 1, 1 + len as usize);
            Map::from_iter(CustomType2(s.to_string()))
        };

        let capacity = _to_usize(GLOBAL_DATA, 64);
        let mut cap_map = Map::with_capacity(capacity);
        let mut new_map = Map::new();

        let ops_cnt = _to_usize(GLOBAL_DATA, 72) % 256;
        let mut offset = 80;

        for _ in 0..ops_cnt {
            if offset + 2 > GLOBAL_DATA.len() { break; }
            let op = _to_u8(GLOBAL_DATA, offset) % 8;
            offset += 1;
            let target = _to_u8(GLOBAL_DATA, offset) % 3;
            offset += 1;

            match (op, target) {
                (0, 0) | (0, 1) | (0, 2) => {
                    if offset + 2 > GLOBAL_DATA.len() { continue; }
                    let k_len = _to_u8(GLOBAL_DATA, offset) % 17;
                    offset += 1;
                    let k = CustomType0(_to_str(GLOBAL_DATA, offset, offset + k_len as usize).to_string());
                    offset += k_len as usize;

                    let v_len = _to_u8(GLOBAL_DATA, offset) % 17;
                    offset += 1;
                    let v = CustomType1(_to_str(GLOBAL_DATA, offset, offset + v_len as usize).to_string());
                    offset += v_len as usize;

                    match target {
                        0 => old_map.insert(k, v),
                        1 => cap_map.insert(k, v),
                        _ => new_map.insert(k, v),
                    };
                },
                (1, _) => {
                    let map = match target {
                        0 => &mut old_map,
                        1 => &mut cap_map,
                        _ => &mut new_map,
                    };
                    if offset + 1 > GLOBAL_DATA.len() { continue; }
                    let k_len = _to_u8(GLOBAL_DATA, offset) % 17;
                    offset +=1;
                    if offset + k_len as usize > GLOBAL_DATA.len() { continue; }
                    let k = CustomType0(_to_str(GLOBAL_DATA, offset, offset + k_len as usize).to_string());
                    offset += k_len as usize;
                    map.remove(&k);
                },
                (2, _) => {
                    let map = match target {
                        0 => &old_map,
                        1 => &cap_map,
                        _ => &new_map,
                    };
                    let _ = map.is_empty();
                },
                (3, _) => {
                    let map = match target {
                        0 => &mut old_map,
                        1 => &mut cap_map,
                        _ => &mut new_map,
                    };
                    for (k, v) in map.iter_mut() {
                        if offset + 1 > GLOBAL_DATA.len() { break; }
                        let mod_len = _to_u8(GLOBAL_DATA, offset) % 17;
                        offset +=1;
                        if offset + mod_len as usize > GLOBAL_DATA.len() { break; }
                        *v = CustomType1(_to_str(GLOBAL_DATA, offset, offset + mod_len as usize).to_string());
                        offset += mod_len as usize;
                        break;
                    }
                },
                (4, _) => {
                    let map = match target {
                        0 => &old_map,
                        1 => &cap_map,
                        _ => &new_map,
                    };
                    let _len = map.len();
                    for (k, v) in map.iter() {
                        println!("{:?} {:?}", k.0, v.0);
                    }
                },
                (5, _) => {
                    let map = match target {
                        0 => &mut old_map,
                        1 => &mut cap_map,
                        _ => &mut new_map,
                    };
                    map.clear();
                },
                (6, _) => {
                    let map = match target {
                        0 => &old_map,
                        1 => &cap_map,
                        _ => &new_map,
                    };
                    let other = map.clone();
                },
                (7, _) => {
                    let map = match target {
                        0 => &old_map,
                        1 => &cap_map,
                        _ => &new_map,
                    };
                    let _ = map.get(&CustomType0(String::new()));
                },
                _ => {}
            }
        }

        let _ = old_map.is_empty();
        let _ = cap_map.is_empty();
        let _ = new_map.is_empty();
        let _v: Vec<_> = new_map.iter().collect();
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