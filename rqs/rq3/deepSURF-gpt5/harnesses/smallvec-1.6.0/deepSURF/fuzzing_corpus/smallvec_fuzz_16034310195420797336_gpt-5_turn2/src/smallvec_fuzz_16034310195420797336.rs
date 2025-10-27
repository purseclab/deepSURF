#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 260 { return; }
        set_global_data(data);
        let gd = get_global_data();
        let GLOBAL_DATA = gd.first_half;
        let GLOBAL_DATA2 = gd.second_half;

        let mut arr36 = [0u8; 36];
        for i in 0..36 {
            arr36[i] = _to_u8(GLOBAL_DATA, i);
        }
        let mut arr64 = [0u8; 64];
        for i in 0..64 {
            arr64[i] = _to_u8(GLOBAL_DATA2, i);
        }
        let mut vec_init: Vec<u8> = Vec::new();
        let vec_len = (_to_u8(GLOBAL_DATA2, 64) % 65) as usize;
        for i in 0..vec_len {
            vec_init.push(_to_u8(GLOBAL_DATA2, 65 + i));
        }

        let ctor_choice = _to_u8(GLOBAL_DATA, 60);
        let mut sv: smallvec::SmallVec<[u8; 36]> = match ctor_choice % 5 {
            0 => smallvec::SmallVec::<[u8; 36]>::new(),
            1 => smallvec::SmallVec::<[u8; 36]>::with_capacity(_to_usize(GLOBAL_DATA, 61)),
            2 => smallvec::SmallVec::<[u8; 36]>::from_buf(arr36),
            3 => smallvec::SmallVec::<[u8; 36]>::from_buf_and_len(arr36, _to_usize(GLOBAL_DATA, 69)),
            _ => smallvec::SmallVec::<[u8; 36]>::from_vec(vec_init),
        };

        let mut sv2 = smallvec::SmallVec::<[u8; 36]>::from_slice(&arr64[..]);

        let _p0 = (&mut sv).as_mut_ptr();
        let _cap0 = sv.capacity();
        let _len0 = sv.len();
        println!("{}", _cap0);
        println!("{}", _len0);

        sv.push(_to_u8(GLOBAL_DATA, 70));
        sv.insert(_to_usize(GLOBAL_DATA, 71), _to_u8(GLOBAL_DATA2, 0));
        sv.extend_from_slice(&arr36[..]);
        let _ = sv.try_reserve(_to_usize(GLOBAL_DATA, 72));
        println!("{:?}", sv.as_slice());
        println!("{:?}", sv.as_mut_slice());
        let _p1 = (&mut sv).as_mut_ptr();

        let ord = smallvec::SmallVec::cmp(&sv, &sv2);
        println!("{:?}", ord);
        let _eq = smallvec::SmallVec::partial_cmp(&sv, &sv2);
        if let Some(o) = _eq { println!("{:?}", o); }

        sv.append(&mut sv2);
        let _opt = sv.pop();
        let _p2 = (&mut sv).as_mut_ptr();

        let ops = (1 + (_to_u8(GLOBAL_DATA, 73) % 10)) as usize;
        for i in 0..ops {
            let sel = _to_u8(GLOBAL_DATA2, i);
            match sel % 12 {
                0 => {
                    sv.push(_to_u8(GLOBAL_DATA, 74 + (i % 20)));
                    println!("{:?}", sv.as_slice());
                }
                1 => {
                    sv.insert(_to_usize(GLOBAL_DATA, 90), _to_u8(GLOBAL_DATA2, 10 + i));
                    println!("{:?}", sv.as_mut_slice());
                }
                2 => {
                    let _ = sv.remove(_to_usize(GLOBAL_DATA, 91));
                    println!("{}", sv.len());
                }
                3 => {
                    let _ = sv.swap_remove(_to_usize(GLOBAL_DATA2, 20));
                    println!("{}", sv.is_empty());
                }
                4 => {
                    sv.reserve(_to_usize(GLOBAL_DATA, 92));
                    println!("{}", sv.capacity());
                }
                5 => {
                    let _ = sv.try_reserve_exact(_to_usize(GLOBAL_DATA2, 21));
                    println!("{}", sv.capacity());
                }
                6 => {
                    sv.truncate(_to_usize(GLOBAL_DATA, 93));
                    println!("{}", sv.len());
                }
                7 => {
                    sv.resize_with(_to_usize(GLOBAL_DATA2, 22), || _to_u8(GLOBAL_DATA, 94));
                    println!("{:?}", sv.as_slice());
                }
                8 => {
                    sv.retain(|e| {
                        let b = _to_bool(GLOBAL_DATA, 95);
                        if b { *e % 2 == 0 } else { *e % 2 == 1 }
                    });
                    println!("{:?}", sv.as_slice());
                }
                9 => {
                    sv.dedup();
                    println!("{}", sv.len());
                }
                10 => {
                    {
                        let mut d = sv.drain(0.._to_usize(GLOBAL_DATA2, 23));
                        let _ = d.next();
                        let _ = d.next_back();
                    }
                    println!("{}", sv.len());
                }
                _ => {
                    let idx = _to_usize(GLOBAL_DATA, 96);
                    let r = sv.index(idx);
                    println!("{}", *r);
                }
            }
            let _p_it = (&mut sv).as_mut_ptr();
            println!("{:?}", sv.as_slice());
        }

        let idx_a = _to_usize(GLOBAL_DATA, 97);
        let r_a = sv.index(idx_a);
        println!("{}", *r_a);
        let idx_b = _to_usize(GLOBAL_DATA2, 24);
        let r_b = sv.index(idx_b);
        println!("{}", *r_b);

        let mut it = sv.clone().into_iter();
        let _n1 = it.next();
        let _n2 = it.next_back();
        if let Some(x) = _n1 { println!("{}", x); }
        if let Some(x) = _n2 { println!("{}", x); }

        let slice_ref = sv.as_ref();
        println!("{:?}", slice_ref);
        let slice_mut = sv.as_mut();
        println!("{:?}", slice_mut);

        let _p3 = (&mut sv).as_mut_ptr();
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