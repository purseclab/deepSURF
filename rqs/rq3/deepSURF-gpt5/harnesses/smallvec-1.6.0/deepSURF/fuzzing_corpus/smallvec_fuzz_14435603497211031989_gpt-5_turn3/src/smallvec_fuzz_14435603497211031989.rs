#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};
use std::borrow::BorrowMut;

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 800 { return; }
        set_global_data(data);
        let halves = get_global_data();
        let first = halves.first_half;
        let second = halves.second_half;

        let len_n = (_to_u8(first, 9) % 65) as usize;
        let mut vec_u8 = std::vec::Vec::with_capacity(64);
        for i in 0..len_n {
            let b = _to_u8(first, 10 + i);
            vec_u8.push(b);
        }

        let mut base_arr: [u8; 32] = [0; 32];
        for i in 0..32 {
            base_arr[i] = _to_u8(first, 50 + i);
        }

        let ctor_sel = _to_u8(first, 0) % 6;
        let mut v: SmallVec<[u8; 32]> = match ctor_sel {
            0 => SmallVec::<[u8; 32]>::new(),
            1 => SmallVec::<[u8; 32]>::with_capacity(_to_usize(first, 2)),
            2 => SmallVec::<[u8; 32]>::from_slice(&vec_u8),
            3 => SmallVec::<[u8; 32]>::from_vec(vec_u8.clone()),
            4 => SmallVec::<[u8; 32]>::from_elem(_to_u8(first, 4), (_to_u8(first, 5) % 65) as usize),
            _ => SmallVec::<[u8; 32]>::from_buf_and_len(base_arr, ((_to_u8(first, 6) % 33) as usize)),
        };

        let _ = v.capacity();
        let _ = v.is_empty();
        let _ = v.len();

        v.reserve(_to_usize(first, 60));
        let pv = _to_u8(first, 61);
        v.push(pv);
        if len_n > 0 {
            let half = len_n / 2;
            let slice_half = &vec_u8[..half];
            v.extend_from_slice(slice_half);
        }

        let idx_ins = _to_usize(first, 62);
        v.insert(idx_ins, _to_u8(first, 63));
        let idx_rem = _to_usize(first, 64);
        let _ = if !v.is_empty() { Some(v.remove(idx_rem)) } else { None };
        v.truncate((_to_u8(first, 65) % 65) as usize);

        {
            let s = v.as_slice();
            println!("{:?}", &*s);
        }
        {
            let s = v.as_ref();
            println!("{:?}", &*s);
        }

        {
            let s: &mut [u8] = v.borrow_mut();
            println!("{:?}", &*s);
            if !s.is_empty() {
                s[0] = s[0].wrapping_add(1);
            }
        }

        {
            let sm2 = v.as_mut();
            println!("{:?}", &*sm2);
            if !sm2.is_empty() {
                sm2[sm2.len() - 1] = sm2[sm2.len() - 1].wrapping_sub(1);
            }
        }

        {
            let dm = Deref::deref(&v);
            println!("{:?}", &*dm);
            let dmm = DerefMut::deref_mut(&mut v);
            if !dmm.is_empty() {
                dmm[0] = dmm[0].wrapping_add(2);
            }
        }

        let ops = (_to_u8(second, 0) % 20) as usize;
        for i in 0..ops {
            let tag = _to_u8(second, 1 + i);
            match tag % 10 {
                0 => {
                    v.reserve(_to_usize(second, 40 + i));
                }
                1 => {
                    v.try_reserve(_to_usize(second, 60 + i)).ok();
                }
                2 => {
                    v.resize((_to_u8(second, 80 + i) % 65) as usize, _to_u8(second, 100 + i));
                }
                3 => {
                    v.retain(|e| {
                        let t = _to_u8(second, 120 + i);
                        if t % 7 == 0 { return false; }
                        *e % 2 == (t % 2)
                    });
                }
                4 => {
                    v.dedup();
                }
                5 => {
                    v.dedup_by(|a, b| {
                        let t = _to_u8(second, 140 + i);
                        if t % 5 == 0 {
                            return true;
                        }
                        (*a % 3) == (*b % 3)
                    });
                }
                6 => {
                    let end = _to_usize(second, 160 + i);
                    let mut d = v.drain(0..end);
                    let _ = d.next();
                    let _ = d.next_back();
                }
                7 => {
                    let idx = _to_usize(second, 180 + i);
                    if !v.is_empty() {
                        let _ = v[idx % v.len()];
                    }
                }
                8 => {
                    let idx = _to_usize(second, 200 + i);
                    if !v.is_empty() {
                        let val = _to_u8(second, 220 + i);
                        let slot = idx % v.len();
                        v[slot] = val;
                    }
                }
                _ => {
                    v.truncate((_to_u8(second, 240 + i) % 65) as usize);
                }
            }
        }

        let vc = v.clone();
        let _ = v.eq(&vc);
        let _ = v.partial_cmp(&vc);
        let _ = v.cmp(&vc);

        {
            let mut it = v.clone().into_iter();
            let _ = it.next();
            let _ = it.next_back();
            let _ = it.as_slice();
            let _ = it.as_mut_slice();
        }

        let s1 = v.as_slice();
        let mut v2 = ToSmallVec::<[u8; 32]>::to_smallvec(s1);
        v.append(&mut v2);

        {
            let bm: &mut [u8] = v.borrow_mut();
            println!("{:?}", &*bm);
        }

        if !v.is_empty() {
            let _ = v.pop();
        }

        v.shrink_to_fit();
        let _ = v.into_boxed_slice();
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