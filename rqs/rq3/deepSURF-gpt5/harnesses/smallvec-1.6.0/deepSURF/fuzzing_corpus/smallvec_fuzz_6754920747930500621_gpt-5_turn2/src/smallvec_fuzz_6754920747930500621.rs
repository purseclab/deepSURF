#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 512 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let FIRST = global_data.first_half;
        let SECOND = global_data.second_half;

        let len_count = _to_u8(FIRST, 0) % 65;
        let mut base_vec = std::vec::Vec::with_capacity(64);
        let mut i = 0usize;
        while i < len_count as usize {
            base_vec.push(_to_u8(SECOND, 1 + i));
            i += 1;
        }

        let selector = _to_u8(FIRST, 10) % 6;
        let mut sv: SmallVec<[u8; 16]> = match selector {
            0 => SmallVec::<[u8; 16]>::new(),
            1 => SmallVec::<[u8; 16]>::with_capacity(_to_usize(FIRST, 12)),
            2 => SmallVec::<[u8; 16]>::from_vec(base_vec.clone()),
            3 => SmallVec::<[u8; 16]>::from_slice(&base_vec[..]),
            4 => (&base_vec[..]).to_smallvec(),
            _ => SmallVec::<[u8; 16]>::from_elem(_to_u8(FIRST, 20), _to_usize(FIRST, 22)),
        };

        sv.reserve(_to_usize(FIRST, 30));
        let _ = sv.try_reserve_exact(_to_usize(FIRST, 38));

        let push_count = _to_u8(FIRST, 46) % 20;
        let mut j = 0usize;
        while j < push_count as usize {
            sv.push(_to_u8(SECOND, 2 + j));
            j += 1;
        }

        let ext_len = _to_u8(FIRST, 55) % 65;
        let ext_end = std::cmp::min(ext_len as usize, base_vec.len());
        let ext_slice = &base_vec[..ext_end];
        sv.extend_from_slice(ext_slice);
        sv.insert_from_slice(_to_usize(FIRST, 60), ext_slice);

        sv.retain(|x| {
            let gd = get_global_data();
            let v = _to_u8(gd.first_half, 80);
            *x = x.wrapping_add(v);
            _to_bool(gd.first_half, 81)
        });

        sv.dedup_by(|a, b| {
            let gd = get_global_data();
            let sel = _to_u8(gd.first_half, 71);
            if sel % 13 == 0 { panic!("INTENTIONAL PANIC!"); }
            *a == *b
        });

        let sl = sv.as_slice();
        let view = if sl.len() >= 8 { &sl[0..8] } else { sl };
        println!("{:?}", view);

        let mut rounds = (_to_u8(FIRST, 88) % 8) as usize;
        while rounds > 0 {
            let op = _to_u8(FIRST, 89 + rounds) % 8;
            match op {
                0 => {
                    sv.push(_to_u8(SECOND, 16 + rounds));
                }
                1 => {
                    let _ = sv.pop();
                    sv.reserve(_to_usize(FIRST, 96));
                }
                2 => {
                    let mut tmp_sv = SmallVec::<[u8; 16]>::from_slice(ext_slice);
                    sv.append(&mut tmp_sv);
                }
                3 => {
                    let l = sv.len();
                    if l > 0 {
                        let idx = _to_usize(FIRST, 102);
                        let _ = sv.remove(idx % l);
                    }
                }
                4 => {
                    let start = _to_usize(FIRST, 104);
                    let end = _to_usize(FIRST, 106);
                    let mut dr = sv.drain(start..end);
                    let mut steps = _to_u8(FIRST, 108) % 5;
                    while steps > 0 {
                        let _ = dr.next();
                        steps -= 1;
                    }
                }
                5 => {
                    sv.truncate(_to_usize(FIRST, 110));
                }
                6 => {
                    sv.resize_with(_to_usize(FIRST, 118), || {
                        let gd = get_global_data();
                        let v = _to_u8(gd.second_half, 10);
                        if v % 17 == 0 { panic!("INTENTIONAL PANIC!"); }
                        v
                    });
                }
                _ => {
                    let _ = sv.is_empty();
                    let _ = sv.capacity();
                }
            }

            let sel2 = _to_u8(FIRST, 90) % 7;
            let i1 = _to_usize(FIRST, 92);
            let i2 = _to_usize(SECOND, 100);
            match sel2 {
                0 => {
                    let m = sv.index_mut(i1);
                    *m = m.wrapping_add(1);
                    println!("{}", *m);
                }
                1 => {
                    let m = sv.index_mut(i1..i2);
                    if !m.is_empty() { m[0] = m[0].wrapping_add(1); }
                    let k = std::cmp::min(m.len(), 8);
                    println!("{:?}", &m[0..k]);
                }
                2 => {
                    let m = sv.index_mut(i1..=i2);
                    if !m.is_empty() { m[0] = m[0].wrapping_add(1); }
                    let k = std::cmp::min(m.len(), 8);
                    println!("{:?}", &m[0..k]);
                }
                3 => {
                    let m = sv.index_mut(i1..);
                    if !m.is_empty() { m[0] = m[0].wrapping_add(1); }
                    let k = std::cmp::min(m.len(), 8);
                    println!("{:?}", &m[0..k]);
                }
                4 => {
                    let m = sv.index_mut(..i2);
                    if !m.is_empty() { m[0] = m[0].wrapping_add(1); }
                    let k = std::cmp::min(m.len(), 8);
                    println!("{:?}", &m[0..k]);
                }
                5 => {
                    let m = sv.index_mut(..);
                    if !m.is_empty() { m[0] = m[0].wrapping_add(1); }
                    let k = std::cmp::min(m.len(), 8);
                    println!("{:?}", &m[0..k]);
                }
                _ => {
                    let m = sv.index_mut(..=i2);
                    if !m.is_empty() { m[0] = m[0].wrapping_add(1); }
                    let k = std::cmp::min(m.len(), 8);
                    println!("{:?}", &m[0..k]);
                }
            }

            rounds -= 1;
        }

        let sv2 = SmallVec::<[u8; 16]>::from_slice(&base_vec[..ext_end]);
        let _ = sv.partial_cmp(&sv2);
        let _ = sv.cmp(&sv2);
        let _ = sv == sv2;

        let _ = sv.as_ptr();
        let _ = sv.as_mut_ptr();

        let mut other = SmallVec::<[u8; 16]>::from_slice(&base_vec[..ext_end]);
        sv.append(&mut other);
        let _ = sv.clone().into_vec();
        let _ = sv.clone().into_boxed_slice();
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