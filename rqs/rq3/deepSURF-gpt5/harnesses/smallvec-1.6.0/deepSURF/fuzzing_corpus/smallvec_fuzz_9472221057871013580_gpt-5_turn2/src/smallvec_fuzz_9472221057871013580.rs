#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 500 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let FIRST = global_data.first_half;
        let SECOND = global_data.second_half;

        let cap0 = _to_usize(FIRST, 0);
        let mut sv_u8_a: SmallVec<[u8; 36]> = SmallVec::with_capacity(cap0);
        let mut buf36 = [0u8; 36];
        for i in 0..36 { buf36[i] = _to_u8(FIRST, 10 + i); }
        let len0 = _to_usize(FIRST, 60);
        let mut sv_u8_b = SmallVec::<[u8; 36]>::from_buf_and_len(buf36, len0);

        let mut arr36b = [0u8; 36];
        for i in 0..36 { arr36b[i] = _to_u8(SECOND, 10 + i); }
        let sv_u8_c = SmallVec::<[u8; 36]>::from_buf(arr36b);

        let slice_len = (_to_u8(FIRST, 96) % 65) as usize;
        let mut sl_vec: Vec<u8> = Vec::new();
        for i in 0..slice_len {
            sl_vec.push(_to_u8(FIRST, 97 + i));
        }
        let mut sv_u8_d = SmallVec::<[u8; 36]>::from_vec(sl_vec);

        let n_i16 = (_to_u8(SECOND, 0) % 65) as usize;
        let mut v_i16: Vec<i16> = Vec::with_capacity(n_i16);
        let mut idx = 1usize;
        for _ in 0..n_i16 {
            v_i16.push(_to_i16(SECOND, idx));
            idx += 2;
        }
        let mut sv_i16 = SmallVec::<[i16; 16]>::from_vec(v_i16);

        let _ = sv_u8_a.capacity();
        let _ = sv_u8_b.capacity();
        println!("{:?}", sv_u8_c.as_slice());
        println!("{:?}", sv_u8_d.as_slice());
        println!("{:?}", sv_i16.as_slice());

        let add0 = _to_usize(FIRST, 140);
        sv_u8_a.reserve(add0);
        let add1 = _to_usize(FIRST, 148);
        let _ = sv_u8_b.try_reserve(add1);
        let add2 = _to_usize(FIRST, 156);
        sv_u8_d.reserve_exact(add2);

        sv_u8_a.push(_to_u8(FIRST, 164));
        sv_u8_a.push(_to_u8(FIRST, 165));
        let _ = sv_u8_a.pop();

        let idx_ins = _to_usize(SECOND, 64);
        sv_u8_b.insert(idx_ins, _to_u8(SECOND, 72));

        let idx_rm = _to_usize(SECOND, 80);
        let _ = sv_u8_b.remove(idx_rm);
        let idx_sw = _to_usize(SECOND, 88);
        let _ = sv_u8_b.swap_remove(idx_sw);

        let tlen0 = _to_usize(FIRST, 172);
        sv_u8_b.truncate(tlen0);

        let mut other = SmallVec::<[u8; 36]>::from_slice(sv_u8_c.as_slice());
        sv_u8_a.append(&mut other);

        let r0 = _to_usize(SECOND, 96);
        let r1 = _to_usize(SECOND, 104);
        let range_start = core::cmp::min(r0, r1);
        let range_end = core::cmp::max(r0, r1);
        {
            let mut d = sv_u8_a.drain(range_start..range_end);
            let _ = d.next();
            let _ = d.next_back();
        }

        let tlen1 = _to_usize(SECOND, 112);
        sv_u8_a.truncate(tlen1);

        let mut rng_len = _to_usize(SECOND, 120);
        sv_i16.resize_with(rng_len, || {
            let b = _to_i16(FIRST, 180);
            if b % 3 == 0 { panic!("INTENTIONAL PANIC!"); }
            b
        });

        let b1: &[u8] = std::borrow::Borrow::borrow(&sv_u8_a);
        println!("{:?}", &*b1);
        let b2: &mut [u8] = std::borrow::BorrowMut::borrow_mut(&mut sv_u8_a);
        println!("{:?}", &*b2);

        let _ = sv_u8_d.try_grow(_to_usize(FIRST, 236));
        sv_u8_d.grow(_to_usize(SECOND, 168));
        sv_u8_d.shrink_to_fit();
        let l0 = sv_u8_d.len();
        let e0 = sv_u8_d.is_empty();
        println!("{:?}", l0);
        println!("{:?}", e0);

        let mut kops = (_to_u8(FIRST, 188) % 20) as usize + 1;
        let mut pos = 0usize;
        while kops > 0 {
            let code = _to_u8(SECOND, 128 + (pos % 32));
            match code % 10 {
                0 => {
                    let x = _to_usize(FIRST, 196);
                    sv_u8_d.truncate(x);
                }
                1 => {
                    let idx0 = _to_usize(SECOND, 136);
                    sv_u8_d.insert(idx0, _to_u8(SECOND, 137));
                }
                2 => {
                    let _ = sv_u8_d.pop();
                    let y = _to_usize(SECOND, 138);
                    sv_u8_d.truncate(y);
                }
                3 => {
                    let z = _to_usize(FIRST, 204);
                    sv_u8_a.reserve(z);
                    println!("{:?}", &*sv_u8_a.as_slice());
                }
                4 => {
                    sv_i16.retain(|v| {
                        if *v == _to_i16(FIRST, 212) { panic!("INTENTIONAL PANIC!"); }
                        *v % 2 == 0
                    });
                }
                5 => {
                    sv_i16.dedup_by(|a, b| {
                        let t = _to_bool(SECOND, 144);
                        if t { panic!("INTENTIONAL PANIC!"); }
                        *a == *b
                    });
                }
                6 => {
                    sv_i16.dedup_by_key(|x| {
                        let key = *x as i32;
                        key
                    });
                }
                7 => {
                    let idx1 = _to_usize(FIRST, 220);
                    let _ = sv_i16.as_mut_slice();
                    let _ = sv_i16.get(idx1);
                }
                8 => {
                    let extra = _to_usize(SECOND, 152);
                    let val = _to_i16(SECOND, 160);
                    sv_i16.resize(extra, val);
                }
                _ => {
                    let s = sv_i16.as_slice();
                    println!("{:?}", &*s);
                }
            }
            pos += 1;
            kops -= 1;
        }

        println!("{:?}", &*sv_u8_b.as_slice());
        println!("{:?}", &*sv_u8_d.as_slice());
        println!("{:?}", &*sv_i16.as_slice());

        let mut it = sv_u8_d.clone().into_iter();
        let _ = it.next();
        let _ = it.next_back();
        println!("{:?}", it.as_slice());
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