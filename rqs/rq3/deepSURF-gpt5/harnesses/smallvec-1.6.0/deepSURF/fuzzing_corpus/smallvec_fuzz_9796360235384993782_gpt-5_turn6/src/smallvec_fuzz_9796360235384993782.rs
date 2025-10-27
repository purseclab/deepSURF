#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

#[derive(Debug, Clone, PartialEq, Eq)]
struct CustomType1(String);

fn mk_ct1(slice: &[u8], off: usize) -> CustomType1 {
    let l = slice.len();
    let idx = if l == 0 { 0 } else { _to_u8(slice, off % l) as usize % l };
    let len = if l == 0 { 0 } else { (_to_u8(slice, idx) % 17) as usize };
    let start = idx;
    let end = if start + len <= l { start + len } else { l };
    let s = _to_str(slice, start, end);
    CustomType1(String::from(s))
}

fn build_vec(slice: &[u8], off: usize, max: usize) -> Vec<CustomType1> {
    let count = if slice.is_empty() { 0 } else { (_to_u8(slice, off % slice.len()) % 65) as usize };
    let mut v = Vec::with_capacity(count);
    for i in 0..count.min(max) {
        v.push(mk_ct1(slice, off + i));
    }
    v
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 240 { return; }
        set_global_data(data);
        let gd = get_global_data();
        let fh = gd.first_half;
        let sh = gd.second_half;

        let base_a = mk_ct1(fh, 1);
        let base_b = mk_ct1(sh, 3);
        let mut vec_a = build_vec(fh, 40, 65);
        if vec_a.is_empty() {
            vec_a.push(base_a.clone());
        }
        let slice_a = &vec_a[..];

        type Arr = [CustomType1; 32];

        let ctor_sel = _to_u8(fh, 8);
        let mut sv: SmallVec<Arr> = match ctor_sel % 5 {
            0 => SmallVec::<Arr>::new(),
            1 => SmallVec::<Arr>::with_capacity(_to_usize(fh, 16)),
            2 => SmallVec::<Arr>::from_vec(vec_a.clone()),
            3 => SmallVec::<Arr>::from(slice_a),
            _ => {
                let buf: Arr = std::array::from_fn(|_| base_a.clone());
                SmallVec::<Arr>::from_buf_and_len(buf, _to_usize(fh, 24))
            }
        };

        let mut other: SmallVec<Arr> = {
            let sel = _to_u8(sh, 4);
            match sel % 3 {
                0 => SmallVec::<Arr>::from_vec(vec_a.clone()),
                1 => SmallVec::<Arr>::with_capacity(_to_usize(sh, 8)),
                _ => SmallVec::<Arr>::from(slice_a),
            }
        };

        sv.reserve(_to_usize(fh, 32));
        let _ = sv.try_reserve_exact(_to_usize(fh, 40));
        sv.reserve_exact(_to_usize(fh, 48));

        if !sv.is_empty() {
            let sref = sv.as_slice();
            if !sref.is_empty() {
                let first = &sref[0];
                println!("{:?}", first);
            }
        }

        let op_count = (if sh.is_empty() { 1 } else { (_to_u8(sh, 12) % 12) + 1 }) as usize;
        for i in 0..op_count {
            let sel = if sh.is_empty() { 0 } else { _to_u8(sh, (16 + i) % sh.len()) % 12 };
            match sel {
                0 => {
                    let elem = mk_ct1(sh, 20 + i);
                    sv.push(elem);
                }
                1 => {
                    let idx = _to_usize(fh, 56);
                    let elem = mk_ct1(fh, 60 + i);
                    sv.insert(idx, elem);
                }
                2 => {
                    let mut ins = build_vec(sh, 64 + i, 8);
                    if ins.is_empty() {
                        ins.push(base_b.clone());
                    }
                    let idx = _to_usize(sh, 24);
                    sv.insert_many(idx, ins.into_iter());
                }
                3 => {
                    let idx = _to_usize(fh, 64);
                    let _ = sv.remove(idx);
                }
                4 => {
                    let idx = _to_usize(sh, 32);
                    let _ = sv.swap_remove(idx);
                }
                5 => {
                    sv.truncate(_to_usize(fh, 72));
                }
                6 => {
                    let slice_b_vec = build_vec(fh, 80 + i, 10);
                    let slice_b = &slice_b_vec[..];
                    sv.extend(slice_b.iter().cloned());
                }
                7 => {
                    sv.retain(|x| {
                        let b = if fh.is_empty() { 0 } else { _to_u8(fh, (88 + i) % fh.len()) };
                        if b % 5 == 0 {
                            panic!("INTENTIONAL PANIC!");
                        }
                        !x.0.is_empty()
                    });
                }
                8 => {
                    sv.dedup_by(|a, b| {
                        let bt = if sh.is_empty() { 0 } else { _to_u8(sh, (96 + i) % sh.len()) };
                        if bt % 7 == 0 {
                            panic!("INTENTIONAL PANIC!");
                        }
                        a.0 == b.0
                    });
                }
                9 => {
                    let start = _to_usize(fh, 80);
                    let end = _to_usize(sh, 40);
                    let mut d = sv.drain(start..end);
                    let _ = d.next();
                    let _ = d.next_back();
                }
                10 => {
                    let idx = _to_usize(sh, 48);
                    let elem = mk_ct1(sh, 100 + i);
                    sv.insert(idx, elem);
                }
                _ => {
                    let _ = sv.try_reserve(_to_usize(fh, 88));
                }
            }

            if !sv.is_empty() {
                let sref = sv.as_slice();
                if !sref.is_empty() {
                    let first = &sref[0];
                    println!("{:?}", first);
                }
                let sref_mut = sv.as_mut_slice();
                if !sref_mut.is_empty() {
                    sref_mut[0] = mk_ct1(fh, 120 + i);
                }
            }
        }

        let mut clone_for_cmp = sv.clone();
        let _ = sv.eq(&clone_for_cmp);

        other.append(&mut clone_for_cmp);

        let sref = sv.as_slice();
        let mut small_from_slice: SmallVec<Arr> = SmallVec::<Arr>::new();
        small_from_slice.extend(sref.iter().cloned());
        let mut combined = small_from_slice.clone();
        combined.append(&mut other);

        if !combined.is_empty() {
            let r = &combined[0];
            println!("{:?}", r);
            let mut cloned = combined.clone().into_iter();
            let _ = cloned.next();
            let _ = cloned.next_back();
        }

        let idx_final = _to_usize(sh, 56);
        let elem_final = mk_ct1(fh, 140);
        sv.insert(idx_final, elem_final);
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