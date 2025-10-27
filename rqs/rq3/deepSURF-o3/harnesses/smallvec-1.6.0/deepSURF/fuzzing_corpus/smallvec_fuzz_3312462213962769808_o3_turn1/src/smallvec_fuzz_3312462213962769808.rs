#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::cmp::Ordering;

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 800 {
            return;
        }
        set_global_data(data);
        let global_data = get_global_data();
        let first = global_data.first_half;
        let second = global_data.second_half;

        // Create a Vec<u32> with up to 65 elements.
        let mut base_vec = Vec::with_capacity(65);
        for i in 0..65 {
            let val = _to_u32(first, i * 4);
            base_vec.push(val);
        }
        let slice_all = &base_vec[..];

        // Dynamically pick a constructor for the first SmallVec.
        let selector = _to_u8(first, 1) % 3;
        let sv1: SmallVec<[u32; 16]> = match selector {
            0 => SmallVec::from_slice(slice_all),
            1 => SmallVec::from_vec(base_vec.clone()),
            _ => SmallVec::from_iter(slice_all.iter().copied()),
        };

        // Second SmallVec using with_capacity.
        let cap = _to_usize(first, 32);
        let mut sv2: SmallVec<[u32; 16]> = SmallVec::with_capacity(cap);

        // Outer loop controlling operation count.
        let op_count = (_to_u8(second, 2) % 20) as usize;
        let mut data_idx = 40usize;

        for _ in 0..op_count {
            if data_idx + 16 >= data.len() {
                break;
            }
            let opcode = _to_u8(data, data_idx);
            data_idx += 1;
            match opcode % 10 {
                0 => {
                    let val = _to_u32(data, data_idx);
                    data_idx += 4;
                    sv2.push(val);
                }
                1 => {
                    sv2.pop();
                }
                2 => {
                    sv2.extend_from_slice(slice_all);
                }
                3 => {
                    let _ = sv2.truncate(_to_usize(data, data_idx));
                    data_idx += 8;
                }
                4 => {
                    let idx = _to_usize(data, data_idx);
                    data_idx += 8;
                    sv2.remove(idx);
                }
                5 => {
                    let idx = _to_usize(data, data_idx);
                    data_idx += 8;
                    sv2.swap_remove(idx);
                }
                6 => {
                    let idx = _to_usize(data, data_idx);
                    data_idx += 8;
                    let val = _to_u32(data, data_idx);
                    data_idx += 4;
                    sv2.insert(idx, val);
                }
                7 => {
                    let ordering: Ordering = sv1.cmp(&sv2);
                    println!("{:?}", ordering);
                }
                8 => {
                    let _res = sv2.try_reserve_exact(_to_usize(data, data_idx));
                    data_idx += 8;
                }
                _ => {
                    let range_end = _to_usize(data, data_idx);
                    data_idx += 8;
                    let _ = sv2.drain(0..range_end);
                }
            }
        }

        // Final calls to target URAPI and related APIs.
        let after_slice = sv2.as_slice();
        let _sv3 = SmallVec::<[u32; 16]>::from_slice(after_slice);
        println!("{:?}", after_slice.len());
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