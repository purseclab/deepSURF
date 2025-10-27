#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

fn make_buf32(src: &[u8]) -> [u8; 32] {
    let mut b = [0u8; 32];
    let mut i = 0usize;
    while i < 32 {
        b[i] = _to_u8(src, i);
        i += 1;
    }
    b
}

fn make_smallvec_u8(first: &[u8], second: &[u8]) -> SmallVec<[u8; 32]> {
    let sel = _to_u8(first, 0);
    match sel % 7 {
        0 => SmallVec::<[u8; 32]>::new(),
        1 => SmallVec::<[u8; 32]>::with_capacity(_to_usize(first, 32)),
        2 => {
            let buf = make_buf32(first);
            let take = (_to_u8(first, 1) as usize).min(32);
            SmallVec::<[u8; 32]>::from_slice(&buf[..take])
        }
        3 => {
            let mut v = Vec::<u8>::new();
            let count = (_to_u8(second, 2) as usize) % 65;
            let mut i = 0usize;
            while i < count {
                v.push(_to_u8(second, (3 + i) % (second.len().saturating_sub(1))));
                i += 1;
            }
            SmallVec::<[u8; 32]>::from_vec(v)
        }
        4 => {
            let buf = make_buf32(second);
            SmallVec::<[u8; 32]>::from_buf(buf)
        }
        5 => {
            let buf = make_buf32(first);
            let len = _to_usize(first, 40);
            SmallVec::<[u8; 32]>::from_buf_and_len(buf, len)
        }
        _ => {
            let elem = _to_u8(first, 2);
            let n = _to_usize(first, 48);
            SmallVec::<[u8; 32]>::from_elem(elem, n)
        }
    }
}

fn build_string_from(second: &[u8], base: usize) -> String {
    let slen = if second.len() == 0 { 0 } else { (_to_u8(second, base % second.len()) as usize) % second.len() };
    let end = if slen + 1 > second.len() { second.len() } else { slen + 1 };
    let s = _to_str(second, 0, end);
    String::from_str(s).unwrap_or_default()
}

fn make_smallvec_string(first: &[u8], second: &[u8]) -> SmallVec<[String; 8]> {
    let sel = _to_u8(second, 0);
    match sel % 5 {
        0 => SmallVec::<[String; 8]>::new(),
        1 => SmallVec::<[String; 8]>::with_capacity(_to_usize(first, 8)),
        2 => {
            let count = (_to_u8(second, 1) as usize) % 65;
            let mut v = Vec::<String>::new();
            let mut i = 0usize;
            while i < count {
                v.push(build_string_from(second, 2 + i));
                i += 1;
            }
            SmallVec::<[String; 8]>::from_vec(v)
        }
        3 => {
            let mut v = Vec::<String>::new();
            let count = (_to_u8(first, 3) as usize) % 65;
            let mut i = 0usize;
            while i < count {
                v.push(build_string_from(second, 4 + i));
                i += 1;
            }
            SmallVec::<[String; 8]>::from_vec(v)
        }
        _ => {
            let elem = build_string_from(second, 5);
            SmallVec::<[String; 8]>::from_elem(elem, _to_usize(first, 16))
        }
    }
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 128 { return; }
        set_global_data(data);
        let gd = get_global_data();
        let first = gd.first_half;
        let second = gd.second_half;

        let mut sv_u8 = make_smallvec_u8(first, second);
        let mut sv_str = make_smallvec_string(first, second);

        let mut pre_other = SmallVec::<[u8; 32]>::from_elem(_to_u8(first, 4), _to_usize(first, 24));
        sv_u8.append(&mut pre_other);

        let _ = sv_u8.capacity();
        let _ = sv_u8.len();
        let _ = sv_u8.is_empty();

        let s_ref = sv_u8.as_slice();
        if !s_ref.is_empty() {
            println!("{:?}", s_ref[0]);
        }
        let ms_ref = sv_u8.as_mut_slice();
        if !ms_ref.is_empty() {
            ms_ref[0] = ms_ref[0].wrapping_add(1);
        }
        let _ = sv_u8.deref();
        let _ = sv_u8.deref_mut();

        let mut ops = (_to_u8(first, 56) as usize) % 20;
        while ops > 0 {
            match _to_u8(first, ops % first.len()) % 16 {
                0 => {
                    sv_u8.push(_to_u8(second, ops % second.len()));
                }
                1 => {
                    sv_u8.insert(_to_usize(first, 32), _to_u8(first, ops % first.len()));
                }
                2 => {
                    let _ = sv_u8.pop();
                }
                3 => {
                    let _ = sv_u8.remove(_to_usize(second, 24));
                }
                4 => {
                    sv_u8.truncate(_to_usize(first, 40));
                }
                5 => {
                    sv_u8.reserve(_to_usize(second, 8));
                }
                6 => {
                    let _ = sv_u8.try_reserve_exact(_to_usize(first, 16));
                }
                7 => {
                    sv_u8.shrink_to_fit();
                }
                8 => {
                    let _ = sv_u8.swap_remove(_to_usize(second, 16));
                }
                9 => {
                    let mut it = sv_u8.clone().into_iter();
                    let _ = it.next();
                    let _ = it.next_back();
                }
                10 => {
                    let range_end = _to_usize(first, 0);
                    let mut d = sv_u8.drain(0..range_end);
                    let _ = d.next();
                    let _ = d.next_back();
                }
                11 => {
                    let _ = sv_u8.as_ptr();
                    let _ = sv_u8.as_mut_ptr();
                }
                12 => {
                    let other_len = (_to_u8(second, 4) as usize) % 65;
                    let mut other_vec = Vec::<u8>::new();
                    let mut i = 0usize;
                    while i < other_len {
                        other_vec.push(_to_u8(second, (5 + i) % (second.len().saturating_sub(1))));
                        i += 1;
                    }
                    let mut other = SmallVec::<[u8; 32]>::from_vec(other_vec);
                    sv_u8.append(&mut other);
                }
                13 => {
                    let _ = sv_u8.cmp(&sv_u8.clone());
                    let _ = sv_u8.partial_cmp(&sv_u8.clone());
                }
                14 => {
                    if !sv_u8.is_empty() {
                        println!("{:?}", sv_u8[_to_usize(first, 8) % sv_u8.len()]);
                    }
                }
                _ => {
                    sv_u8.clear();
                }
            }
            ops -= 1;
        }

        let _ = sv_str.capacity();
        let _ = sv_str.len();

        let s = sv_str.as_slice();
        if !s.is_empty() {
            println!("{:?}", &s[0]);
        }

        let mut more_ops = (_to_u8(second, 63) as usize) % 18;
        while more_ops > 0 {
            match _to_u8(second, more_ops % second.len()) % 14 {
                0 => {
                    sv_str.push(build_string_from(second, more_ops));
                }
                1 => {
                    sv_str.insert(_to_usize(first, 56), build_string_from(second, more_ops + 7));
                }
                2 => {
                    let keep = _to_u8(first, 7);
                    sv_str.retain(move |_| {
                        if keep % 2 == 0 { panic!("INTENTIONAL PANIC!"); }
                        true
                    });
                }
                3 => {
                    sv_str.dedup();
                }
                4 => {
                    let mut flag = _to_u8(second, 11);
                    sv_str.dedup_by(move |_, _| {
                        flag = flag.wrapping_add(1);
                        flag % 3 == 0
                    });
                }
                5 => {
                    let mut flip = _to_u8(first, 21);
                    sv_str.dedup_by_key(move |_| {
                        flip = flip.wrapping_add(5);
                        flip
                    });
                }
                6 => {
                    sv_str.resize_with(_to_usize(second, 32), || build_string_from(second, 13));
                }
                7 => {
                    sv_str.truncate(_to_usize(first, 24));
                }
                8 => {
                    sv_str.reserve_exact(_to_usize(second, 40));
                }
                9 => {
                    let mut it = sv_str.clone().into_iter();
                    let _ = it.next();
                    let _ = it.next_back();
                }
                10 => {
                    let range_end = _to_usize(second, 48);
                    let mut d = sv_str.drain(0..range_end);
                    let _ = d.next();
                    let _ = d.next_back();
                }
                11 => {
                    let _ = sv_str.cmp(&sv_str.clone());
                    let _ = sv_str.partial_cmp(&sv_str.clone());
                }
                12 => {
                    if !sv_str.is_empty() {
                        println!("{:?}", sv_str[_to_usize(first, 12) % sv_str.len()]);
                    }
                }
                _ => {
                    sv_str.clear();
                }
            }
            more_ops -= 1;
        }

        sv_u8.clear();
        sv_str.clear();

        let _ = sv_u8.into_vec();
        let _ = sv_str.into_boxed_slice();
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