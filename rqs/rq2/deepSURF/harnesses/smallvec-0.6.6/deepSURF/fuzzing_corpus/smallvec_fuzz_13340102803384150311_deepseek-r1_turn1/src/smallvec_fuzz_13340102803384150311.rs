#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 256 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        
        let mut vecs: Vec<SmallVec<[u8; 32]>> = Vec::new();
        let constructor_count = (_to_u8(global_data.first_half, 0) % 4) as usize + 1;
        let mut offset = 1;

        for _ in 0..constructor_count {
            let selector = _to_u8(global_data.first_half, offset) % 5;
            offset += 1;

            let sv = match selector {
                0 => SmallVec::new(),
                1 => {
                    let capacity = _to_usize(global_data.first_half, offset);
                    offset += 8;
                    SmallVec::with_capacity(capacity)
                },
                2 => {
                    let elem = _to_u8(global_data.first_half, offset);
                    let len = _to_usize(global_data.first_half, offset + 1) % 65;
                    offset += 9;
                    SmallVec::from_elem(elem, len)
                },
                3 => {
                    let len = _to_usize(global_data.first_half, offset) % 65;
                    let data_start = offset + 8;
                    let data_end = data_start + len as usize;
                    if data_end > global_data.first_half.len() { continue; }
                    let slice = &global_data.first_half[data_start..data_end];
                    SmallVec::from_slice(slice)
                },
                4 => {
                    let len = _to_usize(global_data.first_half, offset) % 65;
                    let data_start = offset + 8;
                    let data_end = data_start + len as usize;
                    if data_end > global_data.first_half.len() { continue; }
                    SmallVec::from_vec(global_data.first_half[data_start..data_end].to_vec())
                },
                _ => SmallVec::new()
            };
            vecs.push(sv);
        }

        let op_count = _to_usize(global_data.second_half, 0) % 16;
        let mut offset = 8;
        
        for i in 0..vecs.len() {
            for _ in 0..op_count {
                if offset >= global_data.second_half.len() { break; }
                let op = _to_u8(global_data.second_half, offset) % 11;
                offset += 1;

                match op {
                    0 => {
                        let elem = _to_u8(global_data.second_half, offset);
                        vecs[i].push(elem);
                        offset += 1;
                    },
                    1 => { vecs[i].pop(); },
                    2 => { vecs[i].truncate(_to_usize(global_data.second_half, offset)); offset += 8; },
                    3 => { vecs[i].insert(_to_usize(global_data.second_half, offset), _to_u8(global_data.second_half, offset + 8)); offset += 9; },
                    4 => { let _ = vecs[i].drain(); },
                    5 => { println!("{:?}", vecs[i].as_slice()); },
                    6 => { let _ = vecs[i].capacity(); },
                    7 => { vecs[i].shrink_to_fit(); },
                    8 => { let _ = vecs[i].dedup(); },
                    9 => { let _ = vecs[i].extend_from_slice(&global_data.second_half[offset..(_to_usize(global_data.second_half, offset) % 64 + offset)]); offset += 8; },
                    10 => { 
                        let mut slice = vecs[i].as_mut_slice();
                        if !slice.is_empty() {
                            slice[0] = _to_u8(global_data.second_half, offset);
                        }
                        offset += 1;
                    },
                    _ => {}
                }
            }
            vecs[i].is_empty();
        }

        let comp_vec = SmallVec::from_slice(&global_data.second_half[(_to_usize(global_data.second_half, 0) % 64)..]);
        for sv in &vecs {
            let _ = sv.partial_cmp(&comp_vec);
            let _ = sv.eq(&comp_vec);
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