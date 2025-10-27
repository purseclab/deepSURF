#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};
use std::borrow::Borrow;

#[derive(Debug, Copy)]
struct CustomType1(usize);

impl core::clone::Clone for CustomType1 {
    fn clone(&self) -> Self {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 555);
        let custom_impl_inst_num = self.0;
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0 {
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector {
            1 => global_data.first_half,
            _ => global_data.second_half,
        };
        let t_136 = _to_usize(GLOBAL_DATA, 563);
        let t_137 = CustomType1(t_136);
        t_137
    }
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 1200 { return; }
        set_global_data(data);
        let gd = get_global_data();
        let fh = gd.first_half;
        let sh = gd.second_half;

        let mut arr32_a = [0u8; 32];
        for i in 0..32 {
            arr32_a[i] = _to_u8(fh, 100 + i);
        }
        let mut arr32_b = [0u8; 32];
        for i in 0..32 {
            arr32_b[i] = _to_u8(sh, 100 + i);
        }

        let mut v_src_a = std::vec::Vec::with_capacity(65);
        let len_a = (_to_u8(fh, 72) % 65) as usize;
        for i in 0..len_a {
            v_src_a.push(_to_u8(fh, 73 + i));
        }
        let mut v_src_b = std::vec::Vec::with_capacity(65);
        let len_b = (_to_u8(sh, 72) % 65) as usize;
        for i in 0..len_b {
            v_src_b.push(_to_u8(sh, 73 + i));
        }

        let sel_a = _to_u8(fh, 0);
        let mut sv_a: SmallVec<[u8; 32]> = match sel_a % 8 {
            0 => SmallVec::<[u8; 32]>::new(),
            1 => SmallVec::<[u8; 32]>::with_capacity(_to_usize(fh, 8)),
            2 => SmallVec::<[u8; 32]>::from_elem(_to_u8(fh, 16), _to_usize(fh, 24)),
            3 => SmallVec::<[u8; 32]>::from_slice(&v_src_a[..]),
            4 => smallvec::ToSmallVec::<[u8; 32]>::to_smallvec(&v_src_a[..]),
            5 => SmallVec::<[u8; 32]>::from_vec(v_src_a),
            6 => SmallVec::<[u8; 32]>::from(arr32_a),
            _ => SmallVec::<[u8; 32]>::from_buf_and_len(arr32_a, _to_usize(fh, 64)),
        };

        let sel_b = _to_u8(sh, 0);
        let mut sv_b: SmallVec<[u8; 32]> = match sel_b % 8 {
            0 => SmallVec::<[u8; 32]>::new(),
            1 => SmallVec::<[u8; 32]>::with_capacity(_to_usize(sh, 8)),
            2 => SmallVec::<[u8; 32]>::from_elem(_to_u8(sh, 16), _to_usize(sh, 24)),
            3 => SmallVec::<[u8; 32]>::from_slice(&v_src_b[..]),
            4 => smallvec::ToSmallVec::<[u8; 32]>::to_smallvec(&v_src_b[..]),
            5 => SmallVec::<[u8; 32]>::from_vec(v_src_b),
            6 => SmallVec::<[u8; 32]>::from(arr32_b),
            _ => SmallVec::<[u8; 32]>::from_buf_and_len(arr32_b, _to_usize(sh, 64)),
        };

        let s1 = sv_a.as_slice();
        let s2 = sv_b.as_slice();
        println!("{:?} {:?}", s1, s2);
        let _ = sv_a.partial_cmp(&sv_b);
        let _ = sv_a.cmp(&sv_b);
        let _ = sv_a.eq(&sv_b);

        let cap_a = sv_a.capacity();
        let len0 = sv_a.len();
        println!("{} {}", cap_a, len0);

        let _brw: &[u8] = sv_a.borrow();
        println!("{:?}", _brw);

        let _asref = sv_b.as_ref();
        println!("{:?}", _asref);

        let idx_slice = sv_a.index(0.._to_usize(fh, 540));
        println!("{:?}", idx_slice);

        let mut v64 = SmallVec::<[u8; 64]>::with_capacity(_to_usize(sh, 200));
        for i in 0..((_to_u8(sh, 210) % 65) as usize) {
            v64.push(_to_u8(sh, 211 + i));
        }

        let ops = (_to_u8(fh, 132) % 20) as usize;
        for i in 0..ops {
            match _to_u8(sh, 133 + i) % 12 {
                0 => { sv_a.push(_to_u8(fh, 140 + i)); }
                1 => { let _ = sv_a.pop(); }
                2 => { sv_a.insert(_to_usize(fh, 200 + i), _to_u8(sh, 160 + i)); }
                3 => { let _ = sv_a.remove(_to_usize(sh, 220 + i)); }
                4 => { sv_a.truncate(_to_usize(fh, 280 + i)); }
                5 => { sv_a.reserve(_to_usize(sh, 340 + i)); }
                6 => { let _ = sv_a.try_reserve(_to_usize(fh, 400 + i)); }
                7 => {
                    sv_a.resize_with(_to_usize(sh, 460 + i), || {
                        let x = _to_u8(fh, 520);
                        if x % 2 == 0 { panic!("INTENTIONAL PANIC!"); }
                        _to_u8(sh, 521)
                    });
                }
                8 => {
                    let as_slice = sv_b.as_slice();
                    println!("{:?}", as_slice);
                    sv_a.extend_from_slice(as_slice);
                }
                9 => { sv_a.dedup(); }
                10 => {
                    sv_a.dedup_by_key(|v| {
                        *v
                    });
                }
                _ => {
                    sv_a.retain(|x| {
                        let b = _to_bool(fh, 530);
                        if b { true } else { *x = x.wrapping_add(1); false }
                    });
                }
            }
            let _ = sv_a.partial_cmp(&sv_b);
            let _ = sv_a.cmp(&sv_b);
            let _ = sv_a.eq(&sv_b);
            let s = sv_a.as_slice();
            println!("{:?}", s);
        }

        let mut d = sv_a.drain(0.._to_usize(fh, 548));
        if let Some(v) = d.next() {
            println!("{}", v);
        }
        if let Some(v) = d.next_back() {
            println!("{}", v);
        }
        drop(d);

        let mut sv_a_clone = sv_a.clone();
        let sv_b_clone = sv_b.clone();

        let ams = sv_a_clone.as_mut_slice();
        println!("{:?}", ams.deref());

        v64.append(&mut sv_a_clone);

        let _ = sv_b_clone.partial_cmp(&sv_b);

        let mut ct = SmallVec::<[CustomType1; 16]>::new();
        for i in 0..((_to_u8(fh, 560) % 16) as usize) {
            ct.push(CustomType1(_to_usize(fh, 561 + i)));
        }
        let _ct_clone = ct.clone();
        ct.resize_with(_to_usize(sh, 580), || CustomType1(_to_usize(sh, 588)));
        let cts = ct.as_slice();
        println!("{:?}", cts);

        let _ = sv_b_clone.into_vec();
        let _ = sv_b.into_boxed_slice();

        let _ = sv_a.partial_cmp(&sv_a);
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