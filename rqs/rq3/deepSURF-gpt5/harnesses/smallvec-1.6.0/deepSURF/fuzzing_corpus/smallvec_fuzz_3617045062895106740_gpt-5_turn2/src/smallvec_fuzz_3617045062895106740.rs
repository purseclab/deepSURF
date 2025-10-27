#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

type A32 = [u8; 32];

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 520 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let first = global_data.first_half;
        let second = global_data.second_half;

        let cap = _to_usize(first, 0);
        let elem = _to_u8(first, 8);
        let count = _to_usize(first, 9);
        let len_buf = _to_usize(first, 60);
        let idx_a = _to_usize(first, 68);
        let idx_b = _to_usize(first, 76);
        let idx_c = _to_usize(first, 84);
        let idx_d = _to_usize(first, 92);
        let idx_e = _to_usize(first, 100);
        let idx_f = _to_usize(first, 108);
        let idx_g = _to_usize(first, 116);
        let idx_h = _to_usize(first, 124);
        let idx_i = _to_usize(first, 132);
        let idx_j = _to_usize(first, 140);

        let mut buf_arr = [0u8; 32];
        let mut i = 0usize;
        while i < 32 {
            buf_arr[i] = _to_u8(first, 20 + i);
            i += 1;
        }

        let tag = _to_u8(second, 0) % 7;
        let mut v_from_vec: Vec<u8> = {
            let mut n = (_to_u8(second, 1) % 65) as usize;
            let avail = second.len().saturating_sub(2);
            n = n.min(avail);
            second[2..2 + n].to_vec()
        };
        let slice_for_from_slice: &[u8] = {
            let sl = (_to_u8(second, 60) % 65) as usize;
            let start = 61usize;
            let end = start + sl.min(second.len().saturating_sub(start));
            &second[start..end]
        };

        let mut sv: SmallVec<A32> = match tag {
            0 => SmallVec::<A32>::new(),
            1 => SmallVec::<A32>::with_capacity(cap),
            2 => SmallVec::<A32>::from_elem(elem, count),
            3 => SmallVec::<A32>::from_buf(buf_arr),
            4 => SmallVec::<A32>::from_buf_and_len(buf_arr, len_buf),
            5 => SmallVec::<A32>::from_vec(v_from_vec.clone()),
            _ => SmallVec::<A32>::from_slice(slice_for_from_slice),
        };

        let r0 = sv.as_mut_slice();
        if r0.len() > 1 {
            r0[1] = r0[1].wrapping_sub(_to_u8(second, 10));
            println!("{:?}", r0[1]);
        }

        let ops = (_to_u8(second, 3) % 12) as usize;
        let mut k = 0usize;
        while k < ops {
            let code = _to_u8(second, 4 + k) % 12;
            match code {
                0 => {
                    sv.push(_to_u8(second, 30 + k));
                }
                1 => {
                    sv.insert(idx_a, _to_u8(second, 40 + k));
                }
                2 => {
                    let _ = sv.remove(idx_b);
                }
                3 => {
                    sv.truncate(idx_c);
                }
                4 => {
                    sv.reserve(idx_d);
                }
                5 => {
                    let sln = (_to_u8(second, 50 + k) % 65) as usize;
                    let st = 52usize;
                    let en = st + sln.min(second.len().saturating_sub(st));
                    let sl = &second[st..en];
                    sv.extend_from_slice(sl);
                }
                6 => {
                    let mut other = SmallVec::<A32>::from_elem(_to_u8(second, 70 + k), idx_e);
                    sv.append(&mut other);
                }
                7 => {
                    let _ = sv.pop();
                }
                8 => {
                    let _ = sv.swap_remove(idx_f);
                }
                9 => {
                    let ln = (_to_u8(second, 90 + k) % 65) as usize;
                    let st = 92usize;
                    let en = st + ln.min(second.len().saturating_sub(st));
                    let iter = second[st..en].iter().cloned();
                    sv.extend(iter);
                }
                10 => {
                    let ln = (_to_u8(second, 120 + k) % 65) as usize;
                    let st = 122usize;
                    let en = st + ln.min(second.len().saturating_sub(st));
                    let sl = &second[st..en];
                    sv.insert_from_slice(idx_g, sl);
                }
                _ => {
                    let mut f = |a: &mut u8, b: &mut u8| -> bool {
                        let p = _to_u8(first, 52);
                        if p % 2 == 0 {
                            panic!("INTENTIONAL PANIC!");
                        }
                        *a == *b
                    };
                    sv.dedup_by(&mut f);
                }
            }
            let s = sv.as_mut_slice();
            if !s.is_empty() {
                s[0] = s[0].wrapping_add(_to_u8(second, 200));
                println!("{:?}", s[0]);
            }
            k += 1;
        }

        let b1 = sv.as_slice();
        if let Some(x) = b1.get(0) {
            println!("{:?}", *x);
        }
        let b2 = sv.as_ref();
        if let Some(x) = b2.get(0) {
            println!("{:?}", *x);
        }
        if sv.len() > 0 {
            let x = &sv[0];
            println!("{:?}", *x);
        }

        let _ = sv.try_reserve(idx_h);
        let _ = sv.try_reserve_exact(idx_i);
        sv.reserve_exact(idx_j);

        {
            let dr_end = _to_usize(second, 20);
            let mut d = sv.drain(0..dr_end);
            let _ = d.next();
            let _ = d.next_back();
        }

        let _len = sv.len();
        let _cap = sv.capacity();
        let _empty = sv.is_empty();

        let s_mut = sv.as_mut_slice();
        if !s_mut.is_empty() {
            s_mut[0] = s_mut[0].wrapping_add(1);
            println!("{:?}", s_mut[0]);
        }

        sv.shrink_to_fit();

        let cl = sv.clone();
        let _ = sv.partial_cmp(&cl);
        let _ = sv.cmp(&cl);
        let _ = sv.eq(&cl);

        let mut it = cl.into_iter();
        let it_s = it.as_slice();
        if let Some(x) = it_s.get(0) {
            println!("{:?}", *x);
        }
        let it_sm = it.as_mut_slice();
        if !it_sm.is_empty() {
            println!("{:?}", it_sm[0]);
        }
        let _ = it.next();
        let _ = it.next_back();

        let bx = sv.clone().into_boxed_slice();
        if !bx.is_empty() {
            println!("{:?}", bx[0]);
        }

        let mut v2 = sv.clone().into_vec();
        if !v2.is_empty() {
            println!("{:?}", v2[0]);
        }
        let mut sv2 = SmallVec::<A32>::from_vec(v2);
        let s2 = sv2.as_mut_slice();
        if !s2.is_empty() {
            println!("{:?}", s2[0]);
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