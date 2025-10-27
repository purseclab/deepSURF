#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::cmp::Ordering;

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 1024 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        
        let mut base_offset = 0;
        let num_vectors = _to_usize(GLOBAL_DATA, base_offset) % 4;
        base_offset += 8;

        let mut vecs: Vec<SmallVec<[usize; 32]>> = Vec::new();
        for _ in 0..num_vectors {
            let len = _to_usize(GLOBAL_DATA, base_offset) % 65;
            base_offset += 8;
            
            let mut v = Vec::new();
            for i in 0..len {
                v.push(_to_usize(GLOBAL_DATA, base_offset + i*8));
            }
            base_offset += len * 8;
            
            let constructor_choice = _to_u8(GLOBAL_DATA, base_offset) % 4;
            base_offset += 1;
            
            let sv = match constructor_choice {
                0 => SmallVec::<[usize; 32]>::from_slice(&v),
                1 => SmallVec::<[usize; 32]>::from_vec(v),
                2 => SmallVec::<[usize; 32]>::from_iter(v.into_iter()),
                3 => SmallVec::<[usize; 32]>::with_capacity(_to_usize(GLOBAL_DATA, base_offset)),
                _ => unreachable!()
            };
            vecs.push(sv);
        }

        let op_count = _to_usize(GLOBAL_DATA, base_offset) % 8;
        base_offset += 8;
        
        for i in 0..op_count {
            match _to_u8(GLOBAL_DATA, base_offset + i) % 10 {
                0 => {
                    let sv = &mut vecs[i % num_vectors];
                    let cap = _to_usize(GLOBAL_DATA, base_offset);
                    sv.reserve(cap);
                }
                1 => {
                    let sv = &mut vecs[i % num_vectors];
                    let elem = _to_usize(GLOBAL_DATA, base_offset);
                    sv.push(elem);
                }
                2 => {
                    let sv = &mut vecs[i % num_vectors];
                    if !sv.is_empty() {
                        let _ = sv.pop();
                    }
                }
                3 => {
                    let sv1 = &vecs[i % num_vectors];
                    let sv2 = &vecs[(i+1) % num_vectors];
                    let _ = sv1.partial_cmp(sv2);
                }
                4 => {
                    let sv = &mut vecs[i % num_vectors];
                    let index = _to_usize(GLOBAL_DATA, base_offset) % (sv.len() + 1);
                    let val = _to_usize(GLOBAL_DATA, base_offset + 8);
                    sv.insert(index, val);
                }
                5 => {
                    let sv = &mut vecs[i % num_vectors];
                    let new_len = _to_usize(GLOBAL_DATA, base_offset) % 65;
                    sv.truncate(new_len);
                }
                6 => {
                    let sv = &vecs[i % num_vectors];
                    let _: SmallVec<[usize; 32]> = sv.as_slice().to_smallvec();
                }
                7 => {
                    let sv = &mut vecs[i % num_vectors];
                    let index = _to_usize(GLOBAL_DATA, base_offset) % (sv.len() + 1);
                    let _ = sv.remove(index);
                }
                8 => {
                    let sv = &vecs[i % num_vectors];
                    let elem = _to_usize(GLOBAL_DATA, base_offset);
                    let _ = sv.clone().into_iter().find(|&x| x == elem);
                }
                9 => {
                    let sv = &mut vecs[i % num_vectors];
                    let elem = _to_usize(GLOBAL_DATA, base_offset);
                    sv.retain(|x| *x != elem);
                }
                _ => {}
            }
            base_offset += 16;
        }

        for sv in &vecs {
            let slice = sv.as_slice();
            let _: SmallVec<[usize; 32]> = slice.to_smallvec();
            println!("{:?}", slice);
        }

        {
            let drain_target = &mut vecs[0];
            let drain_range = _to_usize(GLOBAL_DATA, base_offset) % (drain_target.len() + 1);
            let _drain_iter = drain_target.drain(0..drain_range);
        }

        let capacity = _to_usize(GLOBAL_DATA, base_offset + 8);
        let mut dynamic_sv = SmallVec::<[usize; 32]>::with_capacity(capacity);
        dynamic_sv.extend_from_slice(&vecs[0].as_slice());
        let _: SmallVec<[usize; 32]> = dynamic_sv.as_slice().to_smallvec();

        let merged = vecs.into_iter().flat_map(|sv| sv.into_iter()).collect::<Vec<_>>();
        let _: SmallVec<[usize; 32]> = merged.as_slice().to_smallvec();
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