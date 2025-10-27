#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};
use std::borrow::{Borrow, BorrowMut};

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 320 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let tag = _to_u8(GLOBAL_DATA, 0);
        let l1 = (_to_u8(GLOBAL_DATA, 1) % 65) as usize;
        let s1 = 10usize;
        let e1 = s1 + l1;
        let slice1 = &GLOBAL_DATA[s1..e1];

        let l2 = (_to_u8(GLOBAL_DATA, 2) % 65) as usize;
        let s2 = 20usize;
        let e2 = s2 + l2;
        let slice2 = &GLOBAL_DATA[s2..e2];
        let vec1: Vec<u8> = slice2.to_vec();

        let arr16: [u8; 16] = [
            GLOBAL_DATA[40], GLOBAL_DATA[41], GLOBAL_DATA[42], GLOBAL_DATA[43],
            GLOBAL_DATA[44], GLOBAL_DATA[45], GLOBAL_DATA[46], GLOBAL_DATA[47],
            GLOBAL_DATA[48], GLOBAL_DATA[49], GLOBAL_DATA[50], GLOBAL_DATA[51],
            GLOBAL_DATA[52], GLOBAL_DATA[53], GLOBAL_DATA[54], GLOBAL_DATA[55],
        ];
        let arr16b: [u8; 16] = [
            GLOBAL_DATA[56], GLOBAL_DATA[57], GLOBAL_DATA[58], GLOBAL_DATA[59],
            GLOBAL_DATA[60], GLOBAL_DATA[61], GLOBAL_DATA[62], GLOBAL_DATA[63],
            GLOBAL_DATA[64], GLOBAL_DATA[65], GLOBAL_DATA[66], GLOBAL_DATA[67],
            GLOBAL_DATA[68], GLOBAL_DATA[69], GLOBAL_DATA[70], GLOBAL_DATA[71],
        ];
        let len_fb = _to_usize(GLOBAL_DATA, 70);
        let cap = _to_usize(GLOBAL_DATA, 78);

        let mut sv: SmallVec<[u8; 16]> = match tag % 8 {
            0 => SmallVec::<[u8; 16]>::new(),
            1 => SmallVec::<[u8; 16]>::with_capacity(cap),
            2 => SmallVec::<[u8; 16]>::from_slice(slice1),
            3 => SmallVec::<[u8; 16]>::from_vec(vec1.clone()),
            4 => SmallVec::<[u8; 16]>::from_iter(vec1.clone()),
            5 => SmallVec::<[u8; 16]>::from_buf(arr16),
            6 => SmallVec::<[u8; 16]>::from_buf_and_len(arr16b, len_fb),
            _ => SmallVec::<[u8; 16]>::from_slice(slice2),
        };

        println!("{}", sv.capacity());
        println!("{}", sv.len());
        let sref = sv.as_slice();
        println!("{:?}", sref);

        let mut sv2: SmallVec<[u8; 32]> = SmallVec::from_slice(slice1);
        sv2.extend_from_slice(slice2);
        let _eq = sv.eq(&sv2);
        println!("{}", _eq);

        for b in slice2 {
            sv.push(*b);
        }
        let as_mut = sv.as_mut_slice();
        if !as_mut.is_empty() {
            println!("{}", as_mut[0]);
            as_mut[0] = as_mut[0].wrapping_add(1);
        }

        let grow_to = _to_usize(GLOBAL_DATA, 120);
        let _ = sv.try_grow(grow_to);

        let mut it = sv.clone().into_iter();
        let remaining = it.as_slice();
        println!("{:?}", remaining);
        let _ = it.next();
        let _ = it.next_back();

        let op_count = (_to_u8(GLOBAL_DATA, 4) % 16) as usize;
        for i in 0..op_count {
            match GLOBAL_DATA[5 + i] % 13 {
                0 => {
                    let _ = sv.try_reserve(_to_usize(GLOBAL_DATA, 112));
                }
                1 => {
                    let _ = sv.try_reserve_exact(_to_usize(GLOBAL_DATA, 128));
                }
                2 => {
                    sv.extend_from_slice(slice1);
                }
                3 => {
                    sv.insert(_to_usize(GLOBAL_DATA, 104), GLOBAL_DATA[95]);
                }
                4 => {
                    let _ = sv.remove(_to_usize(GLOBAL_DATA, 112));
                }
                5 => {
                    let _ = sv.swap_remove(_to_usize(GLOBAL_DATA, 128));
                }
                6 => {
                    sv.truncate(_to_usize(GLOBAL_DATA, 136));
                }
                7 => {
                    sv.insert_from_slice(_to_usize(GLOBAL_DATA, 144), slice2);
                }
                8 => {
                    let mut f = || -> u8 {
                        _to_u8(GLOBAL_DATA, 80)
                    };
                    sv.resize_with(_to_usize(GLOBAL_DATA, 70), &mut f);
                }
                9 => {
                    let mut pred = |x: &mut u8| -> bool {
                        let gd = get_global_data();
                        if gd.first_half[0] % 2 == 0 {
                            panic!("INTENTIONAL PANIC!");
                        }
                        *x % 2 == 0
                    };
                    sv.retain(&mut pred);
                }
                10 => {
                    let mut same = |a: &mut u8, b: &mut u8| -> bool {
                        let gd = get_global_data();
                        if gd.first_half[1] % 3 == 0 {
                            panic!("INTENTIONAL PANIC!");
                        }
                        *a == *b
                    };
                    sv.dedup_by(&mut same);
                }
                11 => {
                    let end = _to_usize(GLOBAL_DATA, 152);
                    {
                        let mut d = sv.drain(0..end);
                        let _ = d.next();
                        let _ = d.next_back();
                    }
                }
                _ => {
                    let _ = sv.try_grow(_to_usize(GLOBAL_DATA, 96));
                }
            }
        }

        println!("{}", sv.capacity());
        println!("{}", sv.len());
        let sref2 = sv.as_slice().to_vec();
        println!("{:?}", sref2);

        let _ = sv.pop();
        let _ = sv.try_reserve(_to_usize(GLOBAL_DATA, 100));
        let _ = sv.try_reserve_exact(_to_usize(GLOBAL_DATA, 108));

        {
            let m = sv.as_mut_slice();
            if !m.is_empty() {
                println!("{}", m[0]);
            }
        }

        let _ = sv.as_ref();
        let _: &[u8] = sv.borrow();
        let _ = sv.as_mut();
        let _: &mut [u8] = sv.borrow_mut();

        let _p = sv.as_ptr();

        let _ = sv.eq(&sv2);

        let _ = sv.clone().into_vec();
        let _ = sv.clone().into_boxed_slice();

        if sref2.len() > 0 {
            println!("{}", sref2[0]);
        }
        let _ = sv.len();
        let _ = sv.capacity();
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