#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct CustomType3(String);

impl core::clone::Clone for CustomType3 {
    fn clone(&self) -> Self {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 19);
        let custom_impl_inst_num = self.0.len();
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0 {
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector {
            1 => global_data.first_half,
            _ => global_data.second_half,
        };
        let mut t_6 = _to_u8(GLOBAL_DATA, 27) % 17;
        let t_7 = _to_str(GLOBAL_DATA, 28, 28 + t_6 as usize);
        let t_8 = String::from(t_7);
        CustomType3(t_8)
    }
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 4096 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        
        let mut ops = _to_usize(GLOBAL_DATA, 0) % 16;
        let mut idx = 10;
        
        let mut vec_base = Vec::new();
        for _ in 0..32 {
            let elem_len = _to_u8(GLOBAL_DATA, idx) % 32;
            let elem_data = _to_str(GLOBAL_DATA, idx + 1, idx + 1 + elem_len as usize);
            vec_base.push(CustomType3(elem_data.to_string()));
            idx += 1 + elem_len as usize;
        }
        
        let constructor_sel = _to_u8(GLOBAL_DATA, 500) % 5;
        let mut sv = match constructor_sel {
            0 => SmallVec::from_iter(vec_base.iter().cloned()),
            1 => SmallVec::from_vec(vec_base.clone()),
            2 => {
                let elem = _to_str(GLOBAL_DATA, 600, 620).to_string();
                SmallVec::from_elem(CustomType3(elem), 8)
            }
            3 => SmallVec::with_capacity(_to_usize(GLOBAL_DATA, 620) % 64),
            _ => {
                let mut tmp = SmallVec::new();
                tmp.extend(vec_base.iter().cloned().cycle().take(12));
                tmp
            }
        };
        
        while ops > 0 {
            let opcode = _to_u8(GLOBAL_DATA, idx) % 10;
            idx += 1;
            
            match opcode {
                0 => {
                    sv.push(CustomType3(String::from("fuzz")));
                    println!("{:#?}", sv.as_slice());
                    let _ = sv.pop();
                }
                1 => {
                    let idx_remove = _to_usize(GLOBAL_DATA, idx);
                    sv.truncate(idx_remove);
                    let new_cap = _to_usize(GLOBAL_DATA, idx+4) % 128;
                    sv.reserve(new_cap);
                    idx += 8;
                }
                2 => {
                    let elem = CustomType3(_to_str(GLOBAL_DATA, idx, idx + 32).to_string());
                    sv.insert(sv.len() / 2, elem);
                    idx += 32;
                }
                3 => {
                    let other = SmallVec::from_iter(vec_base[..8].iter().cloned());
                    println!("{:?}", sv.partial_cmp(&other));
                    let _ = sv.eq(&other);
                }
                4 => {
                    let mut drain = sv.drain(3.._to_usize(GLOBAL_DATA, idx));
                    idx += 4;
                    while let Some(item) = drain.next() {
                        println!("{:?}", item);
                    }
                }
                5 => {
                    let slice_size = _to_usize(GLOBAL_DATA, idx) % 65;
                    sv.extend(vec_base[..slice_size].iter().cloned());
                    println!("Capacity: {}", sv.capacity());
                    idx += 4;
                }
                6 => {
                    let mut drain = sv.drain(..);
                    drain.next_back();
                    for item in drain.by_ref() {
                        println!("{:?}", item);
                    }
                }
                7 => {
                    sv.retain(|x: &mut CustomType3| x.0.len() > 2);
                    sv.shrink_to_fit();
                }
                8 => {
                    let start = _to_usize(GLOBAL_DATA, idx) % (sv.len() + 1);
                    let end = start + _to_usize(GLOBAL_DATA, idx+4) % (sv.len() - start + 1);
                    let _ = sv.get(start..end);
                }
                9 => {
                    let index = _to_usize(GLOBAL_DATA, idx) % (sv.len() + 1);
                    if let Some(elem) = sv.get_mut(index) {
                        *elem = CustomType3(String::from("modified"));
                    }
                }
                _ => ()
            }
            ops -= 1;
        }
        
        let range_sel = _to_usize(GLOBAL_DATA, idx);
        idx += 4;
        {
            let mut drain = sv.drain(range_sel..);
            let mut iter = drain.by_ref();
            iter.next_back();
        }
        
        let mut cmp_sv = SmallVec::<[CustomType3; 32]>::new();
        cmp_sv.extend(vec_base.iter().cycle().take(24).cloned());
        println!("{:?}", sv.cmp(&cmp_sv));
        let _ = sv.as_ptr();
        let _slice = sv.as_mut_slice();
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