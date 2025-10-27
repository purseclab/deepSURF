#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::SmallVec;
use global_data::*;
use std::fmt::Debug;

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 1024 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let g = global_data.first_half;
        let mut offset = 0;

        let mut sv = match _to_u8(g, offset) % 4 {
            0 => SmallVec::<[u64; 32]>::new(),
            1 => SmallVec::with_capacity(_to_usize(g, offset + 1)),
            2 => {
                let elem_count = _to_usize(g, offset + 1) % 65;
                let mut items = Vec::with_capacity(elem_count);
                for i in 0..elem_count {
                    items.push(_to_u64(g, offset + 1 + i * 8));
                }
                SmallVec::from_slice(&items)
            },
            3 => SmallVec::from_elem(_to_u64(g, offset + 1), _to_usize(g, offset + 9)),
            _ => unreachable!()
        };

        offset += 65;
        let op_count = _to_u8(g, offset) % 20;

        for _ in 0..op_count {
            match _to_u8(g, offset) % 9 {
                0 => sv.push(_to_u64(g, offset + 1)),
                1 => { sv.pop(); },
                2 => sv.insert(_to_usize(g, offset + 1), _to_u64(g, offset + 9)),
                3 => sv.truncate(_to_usize(g, offset + 1)),
                4 => sv.extend_from_slice(&[_to_u64(g, offset + 1), _to_u64(g, offset + 9)]),
                5 => {
                    let cloned = sv.clone();
                    println!("Cloned len:{:?} cap:{:?}", cloned.len(), cloned.capacity());
                    cloned.as_slice().iter().for_each(|x| { println!("{:?}", *x); });
                },
                6 => sv.dedup(),
                7 => {
                    let drained = sv.drain();
                    drained.for_each(|x| { println!("Drained: {:?}", x); });
                },
                8 => sv.shrink_to_fit(),
                _ => unreachable!()
            };
            offset += 17;

            if !sv.is_empty() {
                let idx = _to_usize(g, offset) % sv.len();
                println!("Index {} value: {:?}", idx, sv[idx]);
                let slice = sv.as_slice();
                println!("First elem: {:?}", *slice.first().unwrap());
            }
            
            let _ = sv.partial_cmp(&SmallVec::new());
            println!("Vec state: {:?}", sv.as_slice());
        }

        let mut sv2 = sv.clone();
        sv2.extend_from_slice(&[_to_u64(g, offset), _to_u64(g, offset + 8)]);
        sv2.as_mut_slice().reverse();
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