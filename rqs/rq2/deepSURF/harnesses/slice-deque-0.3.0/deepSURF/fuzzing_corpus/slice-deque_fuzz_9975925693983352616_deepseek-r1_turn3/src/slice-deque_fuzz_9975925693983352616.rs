#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use slice_deque::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut};

struct CustomType0(String);

impl Clone for CustomType0 {
    fn clone(&self) -> Self {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 1);
        let selector = (custom_impl_num + self.0.len()) % 3;
        if selector == 0 {
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector {
            1 => global_data.first_half,
            _ => global_data.second_half,
        };
        let str_len = _to_u8(GLOBAL_DATA, 9) % 17;
        let t_str = _to_str(GLOBAL_DATA, 10, 10 + str_len as usize);
        CustomType0(t_str.to_string())
    }
}

fn _custom_fn0(str0: &mut CustomType0) -> bool {
    let global_data = get_global_data();
    let GLOBAL_DATA = global_data.first_half;
    let custom_impl_num = _to_usize(GLOBAL_DATA, 570);
    let selector = (custom_impl_num + str0.0.len()) % 3;
    if selector == 0 {
        panic!("INTENTIONAL PANIC!");
    }
    let GLOBAL_DATA = match selector {
        1 => global_data.first_half,
        _ => global_data.second_half,
    };
    _to_bool(GLOBAL_DATA, 578)
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 2300 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let mut data_offset = 0;

        let constructor_sel = _to_u8(GLOBAL_DATA, data_offset) % 3;
        data_offset += 1;
        
        let mut t_135 = match constructor_sel {
            0 => SliceDeque::new(),
            1 => {
                let capacity = _to_usize(GLOBAL_DATA, data_offset) % 65;
                data_offset += 8;
                SliceDeque::with_capacity(capacity)
            },
            _ => {
                let elem_count = _to_u8(GLOBAL_DATA, data_offset) % 65;
                data_offset += 1;
                let mut items = Vec::with_capacity(elem_count as usize);
                for _ in 0..elem_count {
                    let str_len = _to_u8(GLOBAL_DATA, data_offset) % 17;
                    data_offset += 1;
                    let start = data_offset % GLOBAL_DATA.len();
                    let end = (start + str_len as usize).min(GLOBAL_DATA.len());
                    let s = String::from(_to_str(GLOBAL_DATA, start, end));
                    items.push(CustomType0(s));
                    data_offset += str_len as usize;
                }
                SliceDeque::from_iter(items)
            }
        };

        let num_ops = _to_u8(GLOBAL_DATA, data_offset) % 20;
        data_offset += 1;
        
        for _ in 0..num_ops {
            let op_sel = _to_u8(GLOBAL_DATA, data_offset) % 6;
            data_offset += 1;
            
            match op_sel {
                0 => {
                    if let Some(front) = t_135.front() {
                        println!("{:?}", front.0);
                    }
                },
                1 => {
                    if let Some(back) = t_135.back_mut() {
                        back.0.push('X');
                    }
                },
                2 => {
                    let str_len = _to_u8(GLOBAL_DATA, data_offset) % 17;
                    data_offset += 1;
                    let start = data_offset % GLOBAL_DATA.len();
                    let end = (start + str_len as usize).min(GLOBAL_DATA.len());
                    let elem = CustomType0(String::from(_to_str(GLOBAL_DATA, start, end)));
                    t_135.push_front(elem);
                    data_offset += str_len as usize;
                },
                3 => {
                    let new_len = _to_usize(GLOBAL_DATA, data_offset) % 65;
                    data_offset += 8;
                    t_135.truncate(new_len);
                },
                4 => {
                    let splice_pos = _to_usize(GLOBAL_DATA, data_offset);
                    data_offset += 8;
                    let insert_elem = CustomType0(String::from("splice"));
                    t_135.splice(splice_pos..splice_pos, vec![insert_elem]);
                },
                _ => {
                    if t_135.len() > 2 {
                        let split_pos = _to_usize(GLOBAL_DATA, data_offset) % t_135.len();
                        data_offset += 8;
                        let _split = t_135.split_off(split_pos);
                    }
                }
            }
        }

        let mut t_136 = &mut t_135;
        let pred_sel = _to_u8(GLOBAL_DATA, data_offset) % 2;
        data_offset += 1;
        
        let t_139 = match pred_sel {
            0 => SliceDeque::drain_filter(t_136, _custom_fn0 as fn(&mut CustomType0) -> bool),
            _ => SliceDeque::drain_filter(t_136, (|x: &mut CustomType0| x.0.len() % 2 == 0) as fn(&mut CustomType0) -> bool)
        };
        
        let mut drained_count = 0;
        for item in t_139 {
            drained_count += 1;
            println!("Drained: {}", item.0);
        }

        if let Some(first) = t_135.front() {
            println!("Final front: {:?}", first.0);
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