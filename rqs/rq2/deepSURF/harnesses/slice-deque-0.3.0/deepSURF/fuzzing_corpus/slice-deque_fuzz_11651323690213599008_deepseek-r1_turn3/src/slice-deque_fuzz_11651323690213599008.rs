#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use slice_deque::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

#[derive(Debug)]
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
        CustomType0(t_4)
    }
}

impl std::cmp::PartialEq for CustomType0 {
    fn eq(&self, _: &Self) -> bool {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 571);
        let custom_impl_inst_num = self.0.len();
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0 {
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector {
            1 => global_data.first_half,
            _ => global_data.second_half,
        };
        _to_bool(GLOBAL_DATA, 579)
    }
}

impl std::cmp::PartialOrd for CustomType0 {
    fn partial_cmp(&self, _: &Self) -> Option<std::cmp::Ordering> {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 580);
        let custom_impl_inst_num = self.0.len();
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0 {
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector {
            1 => global_data.first_half,
            _ => global_data.second_half,
        };
        let t_140 = _to_usize(GLOBAL_DATA, 588);
        let t_141 = match t_140 % 3 {
            0 => std::cmp::Ordering::Less,
            1 => std::cmp::Ordering::Greater,
            2 => std::cmp::Ordering::Equal,
            _ => unreachable!(),
        };
        Some(t_141)
    }
}

fn build_deque(gdata: &[u8]) -> SliceDeque<CustomType0> {
    let mut pos = 0;
    let constructor = _to_u8(gdata, pos) % 5;
    pos += 1;

    match constructor {
        0 => {
            let cap = _to_usize(gdata, pos) % 65;
            pos += 8;
            SliceDeque::with_capacity(cap)
        }
        1 => SliceDeque::new(),
        2 => {
            let elem_count = _to_usize(gdata, pos) % 65;
            pos += 8;
            from_elem(CustomType0("".to_string()), elem_count)
        }
        3 => {
            let mut vec = Vec::new();
            let elems = _to_usize(gdata, pos) % 65;
            pos += 8;
            for _ in 0..elems {
                let len = _to_u8(gdata, pos) % 17;
                pos += 1;
                let s = _to_str(gdata, pos, pos + len as usize);
                pos += len as usize;
                vec.push(CustomType0(s.to_string()));
            }
            SliceDeque::from(&vec[..])
        }
        4 => {
            let mut dq = SliceDeque::new();
            let elems = _to_usize(gdata, pos) % 65;
            pos += 8;
            for _ in 0..elems {
                let len = _to_u8(gdata, pos) % 17;
                pos += 1;
                let s = _to_str(gdata, pos, pos + len as usize);
                pos += len as usize;
                dq.push_back(CustomType0(s.to_string()));
            }
            dq
        }
        _ => unreachable!()
    }
}

fn mutate_deque(dq: &mut SliceDeque<CustomType0>, gdata: &[u8]) {
    let mut pos = 1024;
    let ops = _to_u8(gdata, pos) % 12;
    pos += 1;

    for _ in 0..ops {
        let op = _to_u8(gdata, pos) % 12;
        pos += 1;

        match op {
            0 => {
                let len = _to_u8(gdata, pos + 1) % 17;
                let s = _to_str(gdata, pos + 2, pos + 2 + len as usize);
                dq.push_front(CustomType0(s.to_string()));
                pos += 2 + len as usize;
            }
            1 => {
                let len = _to_u8(gdata, pos + 1) % 17;
                let s = _to_str(gdata, pos + 2, pos + 2 + len as usize);
                dq.push_back(CustomType0(s.to_string()));
                pos += 2 + len as usize;
            }
            2 => { dq.pop_front(); }
            3 => { dq.pop_back(); }
            4 => {
                let idx = _to_usize(gdata, pos + 1) % (dq.len() + 1);
                dq.drain(idx..dq.len());
                pos += 8;
            }
            5 => println!("Deref front: {:?}", dq.front()),
            6 => {
                if let Some(front) = dq.front_mut() {
                    *front = CustomType0("mutated".to_string());
                }
            }
            7 => {
                let slices = dq.as_slices();
                println!("Slices: {:?} {:?}", slices.0, slices.1);
            }
            8 => {
                let other_slice = &[CustomType0("extend".to_string())];
                dq.extend_from_slice(other_slice);
            }
            9 => {
                let idx = _to_usize(gdata, pos + 1) % (dq.len() + 1);
                if !dq.is_empty() {
                    println!("Elem at {}: {:?}", idx, dq.index(idx % dq.len()));
                }
            }
            10 => {
                if let Some(back) = dq.back_mut() {
                    *back = CustomType0("changed".to_string());
                }
            }
            11 => {
                let mut iter = dq.iter();
                println!("First element via iter: {:?}", iter.next());
            }
            _ => unreachable!()
        }
    }
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 4096 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let (g1, g2) = (global_data.first_half, global_data.second_half);

        let mut dq1 = build_deque(g1);
        let mut dq2 = build_deque(g2);

        mutate_deque(&mut dq1, g1);
        mutate_deque(&mut dq2, g2);

        for _ in 0..(_to_u8(g1, 2048) % 4) {
            let mut combined = SliceDeque::new();
            combined.append(&mut dq1);
            combined.append(&mut dq2);
            let _ = combined.as_mut_slice();
        }

        let dq_slice = dq2.as_slice();
        let _ = dq1.as_mut_slices();
        let result = dq1.partial_cmp(&dq_slice);
        println!("Partial cmp result: {:?}", result);
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