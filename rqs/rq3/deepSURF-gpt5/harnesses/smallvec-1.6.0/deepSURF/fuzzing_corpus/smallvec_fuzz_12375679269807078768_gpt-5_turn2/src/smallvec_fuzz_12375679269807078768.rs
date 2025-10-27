#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 256 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let first = global_data.first_half;
        let second = global_data.second_half;

        let mut arr16 = [0u8; 16];
        for i in 0..16 {
            arr16[i] = _to_u8(first, 16 + i);
        }

        let l1 = std::cmp::min(second.len(), (_to_u8(second, 1) as usize) % 65);
        let s1 = &second[0..l1];
        let mut vec1 = Vec::with_capacity(l1);
        for i in 0..l1 {
            vec1.push(_to_u8(second, i));
        }

        let choice = _to_u8(first, 0) % 5;
        let mut sv: SmallVec<[u8; 16]> = match choice {
            0 => SmallVec::new(),
            1 => SmallVec::with_capacity(_to_usize(first, 8)),
            2 => SmallVec::from_slice(s1),
            3 => SmallVec::from_vec(vec1),
            4 => SmallVec::from_buf_and_len(arr16, _to_usize(first, 26)),
            _ => SmallVec::new(),
        };

        sv.push(_to_u8(first, 40));
        sv.reserve(_to_usize(first, 64));
        sv.shrink_to_fit();

        let s = sv.as_slice();
        println!("{:?}", s);
        if !s.is_empty() {
            let r = &s[0];
            println!("{}", *r);
        }

        if sv.len() > 0 {
            sv[0] = _to_u8(second, 3);
            println!("{}", sv[0]);
        }

        sv.insert(_to_usize(first, 48), _to_u8(second, 4));

        let l2 = std::cmp::min(second.len(), (_to_u8(second, 2) as usize) % 65);
        let s2 = &second[0..l2];
        sv.extend_from_slice(s2);

        sv.retain(|v| {
            if _to_u8(first, 100) % 19 == 0 {
                panic!("INTENTIONAL PANIC!");
            }
            *v = v.wrapping_add(1);
            (*v % 2) == (_to_u8(first, 104) % 2)
        });

        sv.dedup();
        sv.dedup_by(|a, b| {
            if _to_u8(first, 105) % 3 == 0 {
                *a = a.wrapping_add(1);
                *b = b.wrapping_sub(1);
            }
            _to_bool(first, 106)
        });
        sv.dedup_by_key(|v| {
            if _to_u8(first, 107) % 5 == 0 {
                *v = v.wrapping_add(2);
            }
            _to_u8(second, 5)
        });

        sv.resize_with(_to_usize(second, 12), || {
            if _to_u8(first, 108) % 7 == 0 {
                panic!("INTENTIONAL PANIC!");
            }
            _to_u8(first, 109)
        });

        {
            let mut dr = sv.drain(0.._to_usize(second, 20));
            let _ = dr.next();
            let _ = dr.next_back();
        }

        sv.shrink_to_fit();

        let max_ops = std::cmp::min(((_to_u8(first, 120) as usize) % 20) + 1, second.len().saturating_sub(31));
        for i in 0..max_ops {
            let code = _to_u8(second, 30 + i) % 12;
            match code {
                0 => { sv.push(_to_u8(first, 121)); }
                1 => { let _ = sv.pop(); }
                2 => { sv.truncate(_to_usize(second, 32)); }
                3 => { let _ = sv.swap_remove(_to_usize(second, 34)); }
                4 => { let _ = sv.remove(_to_usize(second, 36)); }
                5 => { sv.reserve_exact(_to_usize(second, 38)); }
                6 => { let _ = sv.try_reserve(_to_usize(second, 40)); }
                7 => { let _ = sv.try_reserve_exact(_to_usize(second, 42)); }
                8 => { sv.grow(_to_usize(second, 44)); }
                9 => { let _ = sv.insert_from_slice(_to_usize(second, 46), &s2[..std::cmp::min(s2.len(), 3)]); }
                10 => {
                    let k = (_to_u8(first, 122) as usize) % 10;
                    let it = (0..k).map(|x| x as u8);
                    sv.insert_many(_to_usize(second, 48), it);
                }
                11 => { sv.shrink_to_fit(); }
                _ => {}
            }
        }

        let l3 = std::cmp::min(first.len(), (_to_u8(first, 123) as usize) % 65);
        let s3 = &first[0..l3];
        let mut sv2: SmallVec<[u8; 16]> = s3.to_smallvec();
        sv.append(&mut sv2);
        sv.shrink_to_fit();

        let cl = sv.clone();
        let ord = cl.cmp(&sv);
        println!("{:?}", ord);
        let _ = cl.partial_cmp(&sv);
        println!("{}", cl.eq(&sv));

        let bx = sv.clone().into_boxed_slice();
        println!("{}", bx.len());

        let vec_out = sv.clone().into_vec();
        println!("{}", vec_out.len());

        let sl = sv.as_ref();
        println!("{}", sl.len());
        if !sl.is_empty() {
            let rr = &sl[sl.len() - 1];
            println!("{}", *rr);
        }

        {
            let slm = sv.as_mut();
            if !slm.is_empty() {
                slm[0] = slm[0].wrapping_add(_to_u8(first, 124));
            }
        }

        let _ = sv.try_grow(_to_usize(second, 60));
        let cap = sv.capacity();
        println!("{}", cap);

        let len = sv.len();
        println!("{}", len);

        let is_empty = sv.is_empty();
        println!("{}", is_empty);

        sv.clear();
        sv.shrink_to_fit();
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