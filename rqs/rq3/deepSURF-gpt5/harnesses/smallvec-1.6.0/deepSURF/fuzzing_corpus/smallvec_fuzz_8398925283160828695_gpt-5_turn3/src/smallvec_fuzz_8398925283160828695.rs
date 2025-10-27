#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};
use std::borrow::{Borrow, BorrowMut};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct CustomType1(String);

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 320 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let first = global_data.first_half;
        let second = global_data.second_half;

        let vec_len_a = (_to_u8(first, 0) % 65) as usize;
        let mut base_vec_u8 = Vec::with_capacity(vec_len_a);
        for i in 0..vec_len_a {
            let b = _to_u8(second, i % second.len());
            base_vec_u8.push(b);
        }

        let mut v: SmallVec<[u8; 32]> = match _to_u8(first, 1) % 6 {
            0 => SmallVec::<[u8; 32]>::new(),
            1 => SmallVec::<[u8; 32]>::with_capacity(_to_usize(first, 2)),
            2 => SmallVec::<[u8; 32]>::from_slice(&second[0..(vec_len_a.min(second.len()))]),
            3 => SmallVec::<[u8; 32]>::from_vec(base_vec_u8.clone()),
            4 => SmallVec::<[u8; 32]>::from_elem(_to_u8(first, 10), _to_usize(first, 11)),
            _ => SmallVec::<[u8; 32]>::from_buf([_to_u8(first, 12); 32]),
        };

        let _ = v.capacity();
        let _ = v.len();
        let _ = v.is_empty();

        for _ in 0..(_to_u8(first, 13) % 8) {
            v.push(_to_u8(first, 14));
        }

        v.reserve(_to_usize(first, 15));
        let _ = v.try_reserve(_to_usize(first, 23));
        v.reserve_exact(_to_usize(first, 31));
        let _ = v.try_reserve_exact(_to_usize(first, 39));
        v.grow(_to_usize(first, 47));
        let _ = v.try_grow(_to_usize(first, 55));

        if v.len() > 0 {
            let s = v.as_slice();
            let r = &s[0];
            println!("{:?}", *r);
            let ms = v.as_mut_slice();
            let r2 = &mut ms[0];
            println!("{:?}", *r2);
            let r3 = &v[0];
            println!("{:?}", *r3);
        }

        let insert_idx = _to_usize(first, 63);
        v.insert(insert_idx, _to_u8(first, 64));
        let many_len = (_to_u8(first, 65) % 65) as usize;
        let mut many = Vec::with_capacity(many_len);
        for i in 0..many_len {
            many.push(_to_u8(first, 66 + (i % 8)));
        }
        v.insert_many(_to_usize(first, 74), many);

        v.extend_from_slice(&second[0..(vec_len_a.min(second.len()))]);

        let op_count = (_to_u8(first, 82) % 10) as usize;
        for i in 0..op_count {
            match (_to_u8(first, 83 + i) % 7) {
                0 => {
                    if !v.is_empty() {
                        let _ = v.pop();
                    } else {
                        v.push(_to_u8(first, 91 + i));
                    }
                }
                1 => {
                    if v.len() > 0 {
                        let _ = v.swap_remove(_to_usize(first, 99));
                    }
                }
                2 => {
                    v.truncate(_to_usize(first, 107));
                }
                3 => {
                    let _ = v.cmp(&v.clone());
                }
                4 => {
                    let _ = v.partial_cmp(&v.clone());
                }
                5 => {
                    let range_choice = _to_u8(first, 115 + i);
                    let mut d = if range_choice % 3 == 0 {
                        v.drain(..)
                    } else if range_choice % 3 == 1 {
                        v.drain(_to_usize(first, 123).._to_usize(first, 131))
                    } else {
                        v.drain(_to_usize(first, 139)..)
                    };
                    let _ = d.next();
                    let _ = d.next_back();
                }
                _ => {
                    v.retain(|e| {
                        if _to_u8(first, 147) % 2 == 0 {
                            panic!("INTENTIONAL PANIC!");
                        }
                        *e % 2 == (_to_u8(first, 148) % 2)
                    });
                }
            }
        }

        let remove_idx_a = _to_usize(first, 149);
        let _ = v.remove(remove_idx_a);

        v.dedup();
        v.dedup_by(|a, b| {
            if _to_u8(first, 150) % 3 == 0 {
                panic!("INTENTIONAL PANIC!");
            }
            *a == *b
        });
        v.dedup_by_key(|x| {
            if _to_u8(first, 151) % 5 == 0 {
                panic!("INTENTIONAL PANIC!");
            }
            *x % 4
        });

        if v.len() > 0 {
            let s2: &[u8] = v.borrow();
            let rr = _unwrap_option(s2.get(0).cloned());
            println!("{:?}", rr);
            let sm2: &mut [u8] = v.borrow_mut();
            if !sm2.is_empty() {
                sm2[0] = sm2[0].wrapping_add(_to_u8(first, 152));
            }
            let _dm = v.deref_mut();
            let _dr = v.deref();
        }

        let mut it = v.clone().into_iter();
        let _ = it.next();
        let _ = it.next_back();
        let rem = it.as_slice();
        if let Some(rf) = rem.get(0) {
            println!("{:?}", *rf);
        }

        let mut other = SmallVec::<[u8; 32]>::from_vec(base_vec_u8.clone());
        v.append(&mut other);

        let _ = v.into_vec();

        let ct_vec_len = (_to_u8(first, 153) % 65) as usize;
        let mut base_vec_ct = Vec::with_capacity(ct_vec_len);
        for i in 0..ct_vec_len {
            let start = (_to_u8(second, i % second.len()) as usize) % second.len();
            let extra = (_to_u8(second, (i + 7) % second.len()) as usize) % (second.len() - start);
            let s = _to_str(second, start, start + extra);
            base_vec_ct.push(CustomType1(String::from(s)));
        }

        let mut v_ct: SmallVec<[CustomType1; 16]> = match _to_u8(first, 154) % 4 {
            0 => SmallVec::<[CustomType1; 16]>::new(),
            1 => SmallVec::<[CustomType1; 16]>::with_capacity(_to_usize(first, 155)),
            2 => SmallVec::<[CustomType1; 16]>::from_vec(base_vec_ct.clone()),
            _ => {
                let slice_len = ct_vec_len.min(base_vec_ct.len());
                SmallVec::<[CustomType1; 16]>::from_vec(base_vec_ct[0..slice_len].to_vec())
            }
        };

        if let Some(s0) = base_vec_ct.get(0).cloned() {
            v_ct.push(s0);
        }
        let _ = v_ct.capacity();
        let _ = v_ct.len();

        let remove_idx_b = _to_usize(first, 156);
        let _ = v_ct.remove(remove_idx_b);

        if v_ct.len() > 0 {
            let s_ct = v_ct.as_slice();
            let rr_ct = &s_ct[0];
            println!("{:?}", rr_ct);
        }

        let _ = v_ct.try_reserve(_to_usize(first, 157));
        v_ct.reserve(_to_usize(first, 158));
        v_ct.shrink_to_fit();
        v_ct.clear();
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