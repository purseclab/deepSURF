#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::ops::{Deref, DerefMut, Index, IndexMut};

#[derive(Debug, PartialEq, PartialOrd)]
struct CustomType1(usize);
struct CustomType3(String);

impl core::clone::Clone for CustomType1 {
    fn clone(&self) -> Self {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 10);
        let custom_impl_inst_num = self.0;
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0{
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector{
                    1 => global_data.first_half,
                    _ => global_data.second_half,
        };
        let t_4 = _to_usize(GLOBAL_DATA, 18);
        CustomType1(t_4)
    }
}

fn main(){
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 2048 {return;}
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        
        let mut constructor_bytes = 0;
        let mut offset = 0;
        
        let constructor_selector = _to_u8(GLOBAL_DATA, offset) % 4;
        offset += 1;
        
        let mut sv = match constructor_selector {
            0 => SmallVec::<[CustomType1; 32]>::new(),
            1 => {
                let capacity = _to_usize(GLOBAL_DATA, offset);
                offset += 8;
                constructor_bytes += 8;
                SmallVec::with_capacity(capacity)
            }
            2 => {
                let mut elements = Vec::new();
                let elem_count = _to_usize(GLOBAL_DATA, offset) % 65;
                offset += 8;
                for _ in 0..elem_count {
                    elements.push(CustomType1(_to_usize(GLOBAL_DATA, offset)));
                    offset += 8;
                }
                SmallVec::from_vec(elements)
            }
            _ => {
                let elem = CustomType1(_to_usize(GLOBAL_DATA, offset));
                offset += 8;
                let count = _to_usize(GLOBAL_DATA, offset) % 65;
                offset += 8;
                constructor_bytes += 16;
                SmallVec::from_elem(elem, count)
            }
        };

        let op_count = _to_usize(GLOBAL_DATA, offset) % 10;
        offset += 8;
        for i in 0..op_count {
            let op_select = _to_u8(GLOBAL_DATA, offset + i) % 6;
            match op_select {
                0 => {
                    let elem = CustomType1(_to_usize(GLOBAL_DATA, offset + 1 + i * 8));
                    sv.push(elem);
                }
                1 => {
                    sv.pop();
                }
                2 => {
                    let idx = _to_usize(GLOBAL_DATA, offset + 1 + i * 16);
                    let elem = CustomType1(_to_usize(GLOBAL_DATA, offset + 9 + i * 16));
                    sv.insert(idx, elem);
                }
                3 => {
                    let len = _to_usize(GLOBAL_DATA, offset + 1 + i * 8);
                    sv.truncate(len);
                }
                4 => {
                    let start = _to_usize(GLOBAL_DATA, offset + 1 + i * 16);
                    let end = _to_usize(GLOBAL_DATA, offset + 9 + i * 16);
                    sv.drain(start..end);
                }
                5 => {
                    let new_cap = _to_usize(GLOBAL_DATA, offset + 1 + i * 8);
                    sv.reserve(new_cap);
                }
                _ => {}
            }
        }

        let index = _to_usize(GLOBAL_DATA, 1300);
        let _ = &sv[index];
        println!("{:?}", sv.as_slice());

        let cmp_sv = SmallVec::from_elem(CustomType1(0), _to_usize(GLOBAL_DATA, 1400) % 65);
        let _ = sv.partial_cmp(&cmp_sv);

        let mut extend_sv = SmallVec::<[CustomType1; 32]>::new();
        let extend_count = _to_usize(GLOBAL_DATA, 1500) % 65;
        for _ in 0..extend_count {
            extend_sv.push(CustomType1(_to_usize(GLOBAL_DATA, 1508)));
        }
        sv.append(&mut extend_sv);

        let retain_val = _to_usize(GLOBAL_DATA, 1600) % 2 == 0;
        sv.retain(|x| x.0 % 2 == retain_val as usize);

        let shrink_index = _to_usize(GLOBAL_DATA, 1700);
        sv.shrink_to_fit();
        if !sv.is_empty() {
            sv.swap_remove(shrink_index % sv.len());
        }

        let clone_sv = sv.clone();
        let eq_result = sv == clone_sv;
        println!("Equality check: {}", eq_result);
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