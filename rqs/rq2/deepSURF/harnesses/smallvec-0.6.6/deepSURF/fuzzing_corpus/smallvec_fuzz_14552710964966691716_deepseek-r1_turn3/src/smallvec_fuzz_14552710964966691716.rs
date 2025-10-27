#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 256 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        
        let mut sv_list = vec![];
        let op_count = _to_usize(global_data.first_half, 0) % 8 + 2;

        for i in 0..op_count {
            let constructor_sel = _to_u8(global_data.second_half, i * 3) % 4;
            match constructor_sel {
                0 => {
                    let cap = _to_usize(global_data.first_half, 5 + i * 8);
                    let sv = SmallVec::<[u8; 64]>::with_capacity(cap);
                    sv_list.push(sv);
                },
                1 => {
                    let elem = _to_u8(global_data.second_half, 10 + i * 4);
                    let len = _to_usize(global_data.second_half, 14 + i * 4) % 65;
                    let sv = SmallVec::from_elem(elem, len);
                    println!("Created from_elem: {:?}", sv.as_slice());
                    sv_list.push(sv);
                },
                2 => {
                    let slice_len = _to_usize(global_data.first_half, 20 + i * 8) % global_data.second_half.len();
                    let sv = SmallVec::from_slice(&global_data.second_half[..slice_len]);
                    println!("From slice: {:?}", sv);
                    sv_list.push(sv);
                },
                3 => {
                    let sv: SmallVec<[u8; 64]> = SmallVec::new();
                    sv_list.push(sv);
                },
                _ => {}
            }
        }

        for sv in &mut sv_list {
            let ops = _to_u8(global_data.first_half, 50) % 6;
            match ops {
                0 => {
                    let val = _to_u8(global_data.second_half, 100);
                    sv.push(val);
                    let _ = sv.pop();
                },
                1 => {
                    let idx = _to_usize(global_data.first_half, 60);
                    print!("Index {}: {:?}", idx, sv.get(idx));
                },
                2 => {
                    sv.truncate(_to_usize(global_data.first_half, 70));
                },
                3 => {
                    let other_slice = &global_data.second_half[_to_usize(global_data.first_half, 80) % global_data.second_half.len()..];
                    sv.extend_from_slice(other_slice);
                },
                4 => {
                    let cap = sv.capacity();
                    sv.reserve(_to_usize(global_data.first_half, 90));
                },
                5 => {
                    let _ = sv.drain();
                },
                _ => {}
            }
        }

        let cap1 = _to_usize(global_data.first_half, 150);
        let target_sv = SmallVec::<[u8; 64]>::with_capacity(cap1);
        let mut sv2 = SmallVec::<[u8; 64]>::from_slice(&global_data.second_half[..16]);
        sv2.insert_from_slice(_to_usize(global_data.first_half, 160), &global_data.second_half[16..32]);

        println!("Target SV len: {}", target_sv.len());
        print!("SV2 last: {:?}", sv2.last().map(|x| *x));
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