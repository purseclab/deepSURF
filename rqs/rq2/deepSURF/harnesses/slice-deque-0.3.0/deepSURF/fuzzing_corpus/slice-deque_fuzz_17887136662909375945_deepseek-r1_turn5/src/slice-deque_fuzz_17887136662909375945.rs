#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use slice_deque::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut};

#[derive(Debug, PartialEq, PartialOrd)]
struct CustomType0(String);

impl std::clone::Clone for CustomType0 {
    fn clone(&self) -> Self {
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
        let mut t_2 = _to_u8(GLOBAL_DATA, 9) % 17;
        let t_3 = _to_str(GLOBAL_DATA, 10, 10 + t_2 as usize);
        let t_4 = String::from(t_3);
        let t_5 = CustomType0(t_4);
        t_5
    }
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 2850 {return;}
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let num_ops = _to_u8(GLOBAL_DATA, 0) as usize % 5;
        
        let create_selector = _to_u8(GLOBAL_DATA, 1) % 4;
        let mut sdq = match create_selector {
            0 => SliceDeque::new(),
            1 => SliceDeque::with_capacity(_to_usize(GLOBAL_DATA, 2) % 100),
            2 => {
                let len = _to_usize(GLOBAL_DATA, 100) % 65;
                from_elem(CustomType0(String::new()), len)
            }
            _ => {
                let items: Vec<_> = (0.._to_usize(GLOBAL_DATA, 200) % 65)
                    .map(|i| {
                        let start = 300 + i * 15;
                        let len = _to_u8(GLOBAL_DATA, start) % 15;
                        CustomType0(String::from(_to_str(GLOBAL_DATA, start + 1, start + 1 + len as usize)))
                    })
                    .collect();
                SliceDeque::from_iter(items)
            }
        };

        for i in 0..num_ops {
            let op_byte = _to_u8(global_data.second_half, i * 3);
            match op_byte % 8 {
                0 => sdq.push_front(CustomType0(String::from("fuzz"))),
                1 => {sdq.pop_front();},
                2 => {
                    let idx = _to_usize(global_data.second_half, i * 3 + 1) % (sdq.len() + 1);
                    sdq.insert(idx, CustomType0(String::from("insert")));
                }
                3 => sdq.truncate(_to_usize(global_data.second_half, i * 3 + 2) % (sdq.len() + 1)),
                4 => {
                    let mut sdq2 = SliceDeque::new();
                    sdq2.push_back(CustomType0(String::from("append")));
                    sdq.append(&mut sdq2);
                }
                5 => {
                    let _ = sdq.front().map(|x| println!("{:?}", x.0));
                }
                6 => {
                    let _ = sdq.back_mut().map(|x| x.0.push_str("mut"));
                }
                _ => { sdq.drain_filter(|x| x.0.len() % 2 == 0); },
            };
        }

        let mut sdq2 = SliceDeque::from_iter(vec![
            CustomType0(String::from("vec1")),
            CustomType0(String::from("vec2")),
            CustomType0(String::from("vec3"))
        ]);
        sdq.splice(1..3, sdq2.drain(0..2));

        let t_134 = &mut sdq[..];
        let t_135 = SliceDeque::from(t_134);
        let t_136 = t_135.into_iter();
        let count = t_136.count();

        {
            let mut drain_iter = sdq.drain(1..3);
            while let Some(elem) = drain_iter.next() {
                println!("Drained: {}", elem.0);
            }
        }

        let cloned = sdq.clone();
        let mut combined = SliceDeque::with_capacity(sdq.len() + cloned.len());
        combined.extend(sdq.iter().chain(cloned.iter()).cloned());
        combined.truncate(_to_usize(global_data.second_half, 100) % 65);

        let _ = combined.partial_cmp(&sdq);
        let _fmt = format!("{:?}", combined.as_slice());
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