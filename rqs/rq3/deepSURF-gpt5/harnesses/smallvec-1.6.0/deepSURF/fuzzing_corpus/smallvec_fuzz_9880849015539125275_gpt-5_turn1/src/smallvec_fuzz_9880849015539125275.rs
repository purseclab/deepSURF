#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 200 { return; }
        set_global_data(data);
        let gd = get_global_data();
        let first = gd.first_half;
        let second = gd.second_half;

        let _ = u8::from_str("0");

        let mut buf32_a = [0u8; 32];
        let mut buf32_b = [0u8; 32];
        for i in 0..32 {
            buf32_a[i] = first[i % first.len()];
            buf32_b[i] = first[(i + 32) % first.len()];
        }

        let vec_len = (_to_u8(first, 5) % 65) as usize;
        let mut v_init = Vec::with_capacity(vec_len);
        for i in 0..vec_len {
            v_init.push(second[i % second.len()]);
        }

        let ctor_sel = _to_u8(first, 1);
        let mut sv_u8: SmallVec<[u8; 32]> = match ctor_sel % 7 {
            0 => SmallVec::new(),
            1 => SmallVec::with_capacity(_to_usize(first, 2)),
            2 => SmallVec::from_slice(first),
            3 => SmallVec::from_vec(v_init),
            4 => SmallVec::from_buf(buf32_a),
            5 => SmallVec::from_buf_and_len(buf32_b, _to_usize(first, 26)),
            _ => SmallVec::from_elem(_to_u8(second, 0), (_to_usize(second, 2) % 65)),
        };

        let ops = 1 + (_to_u8(first, 60) % 4) as usize;
        for i in 0..ops {
            match _to_u8(first, 61 + i) % 10 {
                0 => {
                    sv_u8.push(_to_u8(second, 6));
                }
                1 => {
                    sv_u8.reserve(_to_usize(first, 34));
                    let _ = sv_u8.try_reserve_exact(_to_usize(second, 10));
                }
                2 => {
                    sv_u8.insert(_to_usize(first, 30), _to_u8(second, 8));
                }
                3 => {
                    let s_mut = sv_u8.deref_mut();
                    println!("{:?}", &s_mut);
                    if !s_mut.is_empty() {
                        let r0 = &mut s_mut[0];
                        println!("{}", *r0);
                        *r0 = r0.wrapping_add(1);
                    }
                }
                4 => {
                    let s_ref = sv_u8.deref();
                    println!("{:?}", &s_ref);
                    let sl = sv_u8.as_slice();
                    println!("{:?}", &sl);
                    let slm = sv_u8.as_mut_slice();
                    if !slm.is_empty() {
                        slm[slm.len() - 1] = slm[slm.len() - 1].wrapping_add(1);
                    }
                }
                5 => {
                    sv_u8.retain(|e| {
                        if *e == _to_u8(first, 40) {
                            return false;
                        }
                        true
                    });
                    sv_u8.dedup();
                }
                6 => {
                    sv_u8.dedup_by(|a, b| {
                        if _to_bool(first, 50) {
                            panic!("INTENTIONAL PANIC!");
                        }
                        *a == *b
                    });
                }
                7 => {
                    let _ = sv_u8.len();
                    let _ = sv_u8.capacity();
                    let _ = sv_u8.is_empty();
                }
                8 => {
                    let drain_len = _to_usize(first, 44);
                    let mut d = sv_u8.drain(0..drain_len);
                    let _ = d.next();
                    let _ = d.next_back();
                }
                _ => {
                    sv_u8.truncate(_to_usize(first, 46));
                    let _ = sv_u8.pop();
                }
            }
        }

        let mut extra = [0u8; 16];
        for i in 0..16 {
            extra[i] = second[i % second.len()];
        }
        sv_u8.extend_from_slice(&extra);

        let idx = _to_usize(second, 16);
        if sv_u8.len() > 0 {
            let _ = sv_u8[idx];
            let r = &sv_u8[0];
            println!("{}", *r);
        }

        let s_mut_again = sv_u8.deref_mut();
        println!("{:?}", &s_mut_again);

        if sv_u8.len() > 0 {
            let _ = sv_u8.swap_remove(_to_usize(second, 14));
        }
        sv_u8.insert_from_slice(_to_usize(first, 12), &extra);

        sv_u8.resize_with(_to_usize(second, 12), || _to_u8(first, 42));

        let mut sv_u8_b = SmallVec::<[u8; 32]>::from_elem(_to_u8(first, 20), (_to_usize(second, 22) % 65));
        sv_u8.append(&mut sv_u8_b);

        let sv_clone = sv_u8.clone();
        let _ = sv_u8.partial_cmp(&sv_clone);
        let _ = sv_u8.cmp(&sv_clone);
        let _ = sv_u8.eq(&sv_clone);

        let mut it = sv_u8.clone().into_iter();
        let _ = it.as_slice();
        let m = it.as_mut_slice();
        if !m.is_empty() {
            m[0] = m[0].wrapping_add(1);
        }
        let _ = it.next();
        let _ = it.next_back();

        let vec_out = sv_u8.clone().into_vec();
        let _sv_back = SmallVec::<[u8; 32]>::from_vec(vec_out);

        let mut buf_i16 = [0i16; 36];
        for i in 0..36 {
            let base = (i * 2) % (first.len().saturating_sub(2).max(2));
            buf_i16[i] = _to_i16(first, base);
        }
        let mut sv_i16 = match _to_u8(second, 24) % 4 {
            0 => SmallVec::<[i16; 36]>::from_buf(buf_i16),
            1 => SmallVec::<[i16; 36]>::from_buf_and_len(buf_i16, _to_usize(second, 26)),
            2 => SmallVec::<[i16; 36]>::from_slice(&buf_i16),
            _ => {
                let mut vi = Vec::with_capacity((_to_u8(second, 28) % 65) as usize);
                let count = vi.capacity();
                for i in 0..count {
                    let base = (i * 2) % (second.len().saturating_sub(2).max(2));
                    vi.push(_to_i16(second, base));
                }
                SmallVec::<[i16; 36]>::from_vec(vi)
            }
        };

        let _ = sv_i16.len();
        let _ = sv_i16.capacity();
        let _ = sv_i16.is_empty();

        let sref_i16 = sv_i16.deref();
        println!("{:?}", &sref_i16);

        let smut_i16 = sv_i16.deref_mut();
        println!("{:?}", &smut_i16);

        sv_i16.truncate(_to_usize(first, 70));
        sv_i16.clear();

        let s_mut_final = sv_i16.deref_mut();
        println!("{:?}", &s_mut_final);
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