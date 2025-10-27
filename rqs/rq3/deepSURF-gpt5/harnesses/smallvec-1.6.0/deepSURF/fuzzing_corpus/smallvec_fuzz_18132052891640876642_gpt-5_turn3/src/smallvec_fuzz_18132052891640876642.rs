#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};
use std::borrow::{Borrow, BorrowMut};

fn pick_index(data: &[u8], seed: usize, width: usize) -> usize {
    if data.len() <= width { 0 } else { seed % (data.len() - width) }
}
fn build_array<const N: usize>(src: &[u8], offset: usize) -> [u8; N] {
    let mut a = [0u8; N];
    if src.is_empty() { return a; }
    for i in 0..N {
        a[i] = _to_u8(src, (offset + i) % src.len());
    }
    a
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 420 { return; }
        set_global_data(data);
        let g = get_global_data();
        let fh = g.first_half;
        let sh = g.second_half;

        let i0 = pick_index(fh, _to_usize(fh, 0), 1);
        let i1 = pick_index(fh, _to_usize(fh, 2), 8);
        let i2 = pick_index(fh, _to_usize(fh, 4), 8);
        let i3 = pick_index(fh, _to_usize(fh, 6), 8);
        let i4 = pick_index(fh, _to_usize(fh, 8), 8);
        let i5 = pick_index(fh, _to_usize(fh, 10), 8);
        let i6 = pick_index(fh, _to_usize(fh, 12), 8);
        let i7 = pick_index(fh, _to_usize(fh, 14), 1);
        let i8 = pick_index(sh, _to_usize(sh, 1), 8);
        let i9 = pick_index(sh, _to_usize(sh, 3), 8);
        let i10 = pick_index(sh, _to_usize(sh, 5), 8);
        let i11 = pick_index(sh, _to_usize(sh, 7), 8);
        let i12 = pick_index(sh, _to_usize(sh, 9), 1);
        let i13 = pick_index(sh, _to_usize(sh, 11), 8);
        let i14 = pick_index(fh, _to_usize(fh, 16), 8);
        let i15 = pick_index(fh, _to_usize(fh, 18), 8);
        let i16 = pick_index(sh, _to_usize(sh, 13), 8);
        let i17 = pick_index(sh, _to_usize(sh, 15), 8);

        let mut base_vec_len = (_to_u8(fh, i0) % 65) as usize;
        if base_vec_len == 0 { base_vec_len = 1; }
        let mut base_vec = Vec::with_capacity(base_vec_len);
        for k in 0..base_vec_len {
            base_vec.push(_to_u8(fh, (i0 + k) % fh.len()));
        }
        let base_slice = &base_vec[..];

        let arr32_a = build_array::<32>(fh, i7 as usize);
        let arr32_b = build_array::<32>(sh, i12 as usize);
        let arr16 = build_array::<16>(fh, i0 as usize);

        let ctor_sel = _to_u8(fh, i0) % 8;
        let mut sv: SmallVec<[u8; 32]> = match ctor_sel {
            0 => SmallVec::<[u8; 32]>::new(),
            1 => SmallVec::<[u8; 32]>::with_capacity(_to_usize(fh, i1)),
            2 => SmallVec::<[u8; 32]>::from_elem(_to_u8(fh, i0), _to_usize(fh, i2)),
            3 => SmallVec::<[u8; 32]>::from_vec(base_vec.clone()),
            4 => SmallVec::<[u8; 32]>::from_slice(base_slice),
            5 => SmallVec::<[u8; 32]>::from_buf(arr32_a),
            6 => SmallVec::<[u8; 32]>::from_buf_and_len(arr32_b, _to_usize(fh, i3)),
            _ => base_slice.to_smallvec(),
        };

        let ctor_sel2 = _to_u8(sh, i12) % 6;
        let mut sv2: SmallVec<[u8; 16]> = match ctor_sel2 {
            0 => SmallVec::<[u8; 16]>::new(),
            1 => SmallVec::<[u8; 16]>::with_capacity(_to_usize(sh, i8)),
            2 => SmallVec::<[u8; 16]>::from_elem(_to_u8(sh, i12), _to_usize(sh, i9)),
            3 => SmallVec::<[u8; 16]>::from_vec(base_vec.clone()),
            4 => SmallVec::<[u8; 16]>::from_slice(base_slice),
            _ => SmallVec::<[u8; 16]>::from_buf(arr16),
        };

        let _ = sv.try_reserve_exact(_to_usize(fh, i4));
        let _ = sv.try_reserve(_to_usize(fh, i5));
        sv.reserve_exact(_to_usize(fh, i6));
        sv.reserve(_to_usize(sh, i8));

        let ops = (_to_u8(sh, i12) % 20) as usize + 1;
        for step in 0..ops {
            let sel = _to_u8(sh, (i12 + step) % sh.len()) % 16;
            match sel {
                0 => {
                    let idx = _to_usize(fh, i14);
                    let val = _to_u8(fh, i7);
                    sv.insert(idx, val);
                }
                1 => {
                    let add = _to_usize(sh, i9);
                    let _ = sv.try_reserve_exact(add);
                }
                2 => {
                    let add = _to_usize(sh, i10);
                    sv.reserve_exact(add);
                }
                3 => {
                    let add = _to_usize(sh, i11);
                    let _ = sv.try_reserve(add);
                }
                4 => {
                    let new_cap = _to_usize(fh, i1);
                    sv.grow(new_cap);
                }
                5 => {
                    let new_cap = _to_usize(fh, i2);
                    let _ = sv.try_grow(new_cap);
                }
                6 => {
                    let val = _to_u8(fh, i0);
                    sv.push(val);
                }
                7 => {
                    let idx = _to_usize(sh, i13);
                    let _ = sv.swap_remove(idx);
                }
                8 => {
                    let end = _to_usize(fh, i15);
                    let mut d = sv.drain(0..end);
                    let _ = d.next();
                    let _ = d.next_back();
                }
                9 => {
                    let sl = sv.as_slice();
                    println!("{:?}", sl);
                }
                10 => {
                    let msl = sv2.as_mut_slice();
                    println!("{:?}", msl);
                }
                11 => {
                    let _ = sv.len();
                    let _ = sv.capacity();
                    let _ = sv.is_empty();
                }
                12 => {
                    if sv.len() > 0 {
                        let idx = _to_usize(fh, i3);
                        let r = &sv[idx];
                        println!("{}", *r as u8);
                    }
                }
                13 => {
                    let len = _to_usize(sh, i16);
                    sv.truncate(len);
                }
                14 => {
                    sv.shrink_to_fit();
                }
                _ => {
                    let slice_len = (_to_u8(fh, i7) % 65) as usize;
                    let mut tmp = Vec::with_capacity(slice_len);
                    for t in 0..slice_len {
                        tmp.push(_to_u8(sh, (i12 + t) % sh.len()));
                    }
                    sv.extend_from_slice(&tmp);
                }
            }
        }

        let eq = sv == sv2;
        println!("{}", eq);
        let _ = sv.as_slice().cmp(sv2.as_slice());
        let _ = _unwrap_option(sv.as_slice().partial_cmp(sv2.as_slice()));

        let v = sv.clone().into_vec();
        println!("{}", v.len());
        let mut sv3 = SmallVec::<[u8; 32]>::from_vec(v);

        let bx = sv2.clone().into_boxed_slice();
        println!("{}", bx.len());

        let _ = sv2.clone().into_inner();

        let mut donor = sv2.clone();
        sv3.append(&mut donor);

        let resize_len = _to_usize(fh, i6);
        let mut count = 0u8;
        sv3.resize_with(resize_len, || {
            count = count.wrapping_add(1);
            if _to_u8(fh, i0) % 2 == 0 { panic!("INTENTIONAL PANIC!"); }
            count
        });

        let _ = sv3.as_ref();
        let _ = sv3.as_mut();
        let _ = sv3.deref();
        let _ = sv3.deref_mut();

        let k = _to_usize(sh, i17);
        sv3.insert_many(k, (0..(_to_u8(sh, i12) as usize)).map(|x| x as u8));
        let _ = sv3.remove(_to_usize(fh, i5));

        let cap_after = sv3.capacity();
        println!("{}", cap_after);

        let _: &[u8] = sv3.borrow();
        let _: &mut [u8] = sv3.borrow_mut();

        let _ = sv3.as_ptr();
        let _ = sv3.as_mut_ptr();

        let mut it = sv3.clone().into_iter();
        let _ = it.as_slice();
        let _ = it.as_mut_slice();
        let _ = it.next();
        let _ = it.next_back();

        let arr_for_extend = build_array::<32>(fh, i0 as usize);
        sv3.extend_from_slice(&arr_for_extend);
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