#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};
use std::borrow::{Borrow, BorrowMut};

fn take_u8(h: &[u8], idx: &mut usize) -> u8 {
    let l = h.len();
    let pos = if l > 0 { *idx % l } else { 0 };
    *idx = idx.wrapping_add(1);
    _to_u8(h, pos)
}
fn take_usize(h: &[u8], idx: &mut usize) -> usize {
    let l = h.len();
    let pos = if l > 8 { *idx % (l - 8) } else { 0 };
    *idx = idx.wrapping_add(8);
    _to_usize(h, pos)
}
fn take_slice<'a>(h: &'a [u8], idx: &mut usize) -> &'a [u8] {
    let l = h.len();
    if l == 0 {
        return &h[0..0];
    }
    let start = if l > 1 { *idx % (l - 1) } else { 0 };
    *idx = idx.wrapping_add(1);
    let mut len = (take_u8(h, idx) as usize) % 65;
    if start + len > l {
        len = l - start;
    }
    &h[start..start + len]
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 512 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let FH = global_data.first_half;
        let SH = global_data.second_half;
        let mut ifh = 0usize;
        let mut ish = 0usize;

        let selector = take_u8(FH, &mut ifh);
        let mut sv: SmallVec<[u8; 32]> = match selector % 4 {
            0 => SmallVec::<[u8; 32]>::new(),
            1 => {
                let cap = take_usize(FH, &mut ifh);
                SmallVec::<[u8; 32]>::with_capacity(cap)
            }
            2 => {
                let sl = take_slice(SH, &mut ish);
                SmallVec::<[u8; 32]>::from_slice(sl)
            }
            _ => {
                let mut buf = [0u8; 32];
                for i in 0..32 {
                    buf[i] = take_u8(FH, &mut ifh);
                }
                let ln = take_usize(FH, &mut ifh);
                SmallVec::<[u8; 32]>::from_buf_and_len(buf, ln)
            }
        };

        let sl2 = take_slice(SH, &mut ish);
        let mut other: SmallVec<[u8; 64]> = sl2.to_smallvec();
        sv.append(&mut other);

        let add0 = take_usize(FH, &mut ifh);
        let _ = sv.try_reserve(add0);

        let ops = (take_u8(FH, &mut ifh) % 25) as usize;
        for _ in 0..ops {
            let tag = take_u8(SH, &mut ish) as usize;
            match tag % 21 {
                0 => {
                    let v = take_u8(FH, &mut ifh);
                    sv.push(v);
                }
                1 => {
                    let idx = take_usize(FH, &mut ifh);
                    let v = take_u8(SH, &mut ish);
                    sv.insert(idx, v);
                }
                2 => {
                    let sl = take_slice(SH, &mut ish);
                    sv.extend_from_slice(sl);
                }
                3 => {
                    let add = take_usize(FH, &mut ifh);
                    let _ = sv.try_reserve(add);
                }
                4 => {
                    let add = take_usize(SH, &mut ish);
                    let _ = sv.try_reserve_exact(add);
                }
                5 => {
                    let add = take_usize(FH, &mut ifh);
                    sv.reserve(add);
                }
                6 => {
                    let idx = take_usize(SH, &mut ish);
                    let _ = sv.remove(idx);
                }
                7 => {
                    let idx = take_usize(FH, &mut ifh);
                    let _ = sv.swap_remove(idx);
                }
                8 => {
                    let new_len = take_usize(SH, &mut ish);
                    sv.truncate(new_len);
                }
                9 => {
                    let r = sv.as_slice();
                    println!("{:?}", r);
                    if !r.is_empty() {
                        println!("{:?}", r[0]);
                    }
                }
                10 => {
                    let mr = sv.as_mut_slice();
                    if !mr.is_empty() {
                        let inc = take_u8(FH, &mut ifh);
                        mr[0] = mr[0].wrapping_add(inc);
                        println!("{:?}", &mr[0]);
                    }
                }
                11 => {
                    let ib = if FH.len() > 0 { ifh % FH.len() } else { 0 };
                    let panic_toggle = _to_bool(FH, ib);
                    ifh = ifh.wrapping_add(1);
                    sv.retain(|x: &mut u8| {
                        if panic_toggle {
                            panic!("INTENTIONAL PANIC!");
                        }
                        (*x & 1) == 0
                    });
                }
                12 => {
                    sv.dedup_by(|a: &mut u8, b: &mut u8| {
                        let ib = if FH.len() > 0 { ifh % FH.len() } else { 0 };
                        let p = _to_bool(FH, ib);
                        ifh = ifh.wrapping_add(1);
                        if p {
                            panic!("INTENTIONAL PANIC!");
                        }
                        *a == *b
                    });
                }
                13 => {
                    sv.dedup_by_key(|x: &mut u8| {
                        let k = *x / 2;
                        k
                    });
                }
                14 => {
                    let end = take_usize(SH, &mut ish);
                    let mut d = sv.drain(0..end);
                    let _ = d.next();
                    let _ = d.next_back();
                }
                15 => {
                    let l = sv.len();
                    let c = sv.capacity();
                    println!("{:?}", (l, c));
                    let cmp_vec = SmallVec::<[u8; 32]>::from_slice(take_slice(FH, &mut ifh));
                    let ord = SmallVec::cmp(&sv, &cmp_vec);
                    println!("{:?}", ord);
                    let pord = SmallVec::partial_cmp(&sv, &cmp_vec);
                    let _ = _unwrap_option(pord);
                }
                16 => {
                    let new_len = take_usize(FH, &mut ifh);
                    let val = take_u8(SH, &mut ish);
                    sv.resize(new_len, val);
                }
                17 => {
                    let new_len = take_usize(SH, &mut ish);
                    let mut toggle = take_u8(FH, &mut ifh);
                    sv.resize_with(new_len, move || {
                        toggle = toggle.wrapping_add(1);
                        if (toggle & 1) == 0 {
                            0u8
                        } else {
                            1u8
                        }
                    });
                }
                18 => {
                    let idx = take_usize(FH, &mut ifh);
                    let sl = take_slice(SH, &mut ish);
                    sv.insert_from_slice(idx, sl);
                }
                19 => {
                    let idx = take_usize(FH, &mut ifh);
                    let k = (take_u8(SH, &mut ish) as usize) % 65;
                    let mut v = Vec::with_capacity(k);
                    for _ in 0..k {
                        v.push(take_u8(FH, &mut ifh));
                    }
                    sv.insert_many(idx, v);
                }
                _ => {
                    sv.clear();
                }
            }

            let _ = sv.try_grow(take_usize(FH, &mut ifh));
            sv.grow(take_usize(SH, &mut ish));

            let b: &[u8] = sv.borrow();
            println!("{:?}", b);
            let bm: &mut [u8] = sv.borrow_mut();
            if !bm.is_empty() {
                println!("{:?}", bm[0]);
            }

            let sref = sv.deref();
            println!("{:?}", sref);
            if sv.len() > 0 {
                let _ = &sv[0..1];
                let _ = &mut sv[0..1];
                sv[0] = sv[0].wrapping_add(1);
            }

            let _ = sv.pop();
            let _ = sv.as_ptr();
        }

        let add1 = take_usize(FH, &mut ifh);
        let _ = sv.try_reserve(add1);
        let add2 = take_usize(SH, &mut ish);
        let _ = _unwrap_result(sv.try_reserve(add2));
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