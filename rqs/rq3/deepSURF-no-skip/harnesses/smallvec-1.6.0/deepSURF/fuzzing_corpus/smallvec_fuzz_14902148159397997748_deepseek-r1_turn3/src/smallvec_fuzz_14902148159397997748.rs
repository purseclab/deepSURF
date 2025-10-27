#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::ops::Deref;

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 512 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let first_half = global_data.first_half;

        let mut offset = 0;
        let op_count = _to_u8(first_half, offset) % 8;
        offset += 1;

        let mut vec1: SmallVec<[u8; 32]> = SmallVec::new();
        let mut vec2 = SmallVec::with_capacity(_to_usize(first_half, offset));
        offset += 8;

        for _ in 0..op_count {
            match _to_u8(first_half, offset) % 10 {
                0 => {
                    let elements = (0.._to_usize(first_half, offset + 1) % 65)
                        .map(|i| _to_u8(first_half, offset + 9 + i))
                        .collect::<Vec<_>>();
                    vec1 = SmallVec::from_slice(&elements);
                    offset += 9 + elements.len();
                }
                1 => {
                    vec1.push(_to_u8(first_half, offset + 1));
                    offset += 2;
                }
                2 => {
                    vec1.insert(_to_usize(first_half, offset + 1) % (vec1.len() + 1), _to_u8(first_half, offset + 9));
                    offset += 10;
                }
                3 => {
                    let _ = vec1.pop();
                }
                4 => {
                    let mut temp_vec = SmallVec::<[u8; 32]>::from_elem(_to_u8(first_half, offset + 1), _to_usize(first_half, offset + 2) % 65);
                    vec1.append(&mut temp_vec);
                    offset += 10;
                }
                5 => {
                    vec1.truncate(_to_usize(first_half, offset + 1) % (vec1.len() + 1));
                    offset += 9;
                }
                6 => {
                    let ref_vec = &vec1;
                    let _slice: &[u8] = ref_vec.deref();
                    println!("{:?}", _slice);
                    if !_slice.is_empty() {
                        let idx = _to_usize(first_half, offset + 1) % _slice.len();
                        println!("{}", _slice[idx]);
                    }
                    offset += 9;
                }
                7 => {
                    let drained: Vec<_> = vec1.drain(..).collect();
                    vec2 = SmallVec::from_vec(drained);
                }
                8 => {
                    let cap = _to_usize(first_half, offset + 1);
                    vec1.reserve(cap);
                    offset += 9;
                }
                9 => {
                    let new_len = _to_usize(first_half, offset + 1);
                    vec1.resize(new_len, _to_u8(first_half, offset + 9));
                    offset += 17;
                }
                _ => unreachable!()
            }

            vec2 = SmallVec::from_elem(_to_u8(first_half, offset), _to_usize(first_half, offset + 1) % 65);
            offset += 9;

            let cmp_result = vec1.partial_cmp(&vec2);
            if let Some(order) = cmp_result {
                match order {
                    std::cmp::Ordering::Less => vec1.extend(vec2.drain(..)),
                    std::cmp::Ordering::Greater => vec2.extend(vec1.drain(..)),
                    _ => {}
                }
            }

            let mut combined_iter = vec1.iter().chain(vec2.iter());
            let _ = combined_iter.next();
            let _ = combined_iter.next_back();

            if vec1.len() > 32 {
                panic!("INTENTIONAL PANIC!");
            }

            let mut sorted = vec1.clone();
            sorted.sort();
            let _ = sorted.as_slice();

            let mut merged = SmallVec::<[u8; 64]>::new();
            merged.extend_from_slice(vec1.as_slice());
            merged.extend_from_slice(vec2.as_slice());
            merged.dedup();
            let _ = merged.capacity();
        }

        let combined = [vec1.deref(), vec2.deref()].concat();
        let _ = SmallVec::<[u8; 64]>::from_slice(&combined);
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