#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

struct CustomRange {
    a: usize,
    b: usize,
    flip: u8,
}

impl core::ops::RangeBounds<usize> for CustomRange {
    fn start_bound(&self) -> core::ops::Bound<&usize> {
        let global_data = get_global_data();
        let GLOBAL_DATA = if self.flip % 2 == 0 { global_data.first_half } else { global_data.second_half };
        let t = _to_u8(GLOBAL_DATA, 0);
        if t % 3 == 0 {
            panic!("INTENTIONAL PANIC!");
        }
        core::ops::Bound::Included(&self.a)
    }
    fn end_bound(&self) -> core::ops::Bound<&usize> {
        let global_data = get_global_data();
        let GLOBAL_DATA = if self.flip % 2 == 0 { global_data.second_half } else { global_data.first_half };
        let t = _to_u8(GLOBAL_DATA, 1);
        if t % 5 == 0 {
            panic!("INTENTIONAL PANIC!");
        }
        core::ops::Bound::Excluded(&self.b)
    }
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 400 { return; }
        set_global_data(data);
        let global = get_global_data();
        let FH = global.first_half;
        let SH = global.second_half;

        let base_len = (_to_u8(FH, 2) as usize) % 65;
        let mut base_vec = std::vec::Vec::with_capacity(64);
        for i in 0..base_len {
            base_vec.push(_to_u8(FH, 3 + i));
        }

        let choice = _to_u8(FH, 70) % 6;
        let mut v: SmallVec<[u8; 32]> = match choice {
            0 => {
                let mut tmp = SmallVec::<[u8; 32]>::new();
                for &b in &base_vec { tmp.push(b); }
                tmp
            }
            1 => {
                let cap = _to_usize(FH, 24);
                let mut tmp = SmallVec::<[u8; 32]>::with_capacity(cap);
                tmp.extend_from_slice(&base_vec);
                tmp
            }
            2 => SmallVec::<[u8; 32]>::from_slice(&base_vec),
            3 => {
                let elem = _to_u8(FH, 90);
                let n = (_to_u8(FH, 91) as usize) % 65;
                SmallVec::<[u8; 32]>::from_elem(elem, n)
            }
            4 => SmallVec::<[u8; 32]>::from_vec(base_vec.clone()),
            _ => base_vec.as_slice().to_smallvec(),
        };

        let _ = v.capacity();
        let _ = v.len();
        let _ = v.is_empty();

        let _ = v.try_grow(_to_usize(FH, 40));
        v.reserve(_to_usize(SH, 24));
        let _ = v.try_reserve(_to_usize(FH, 56));
        v.reserve_exact(_to_usize(SH, 72));
        let _ = v.try_reserve_exact(_to_usize(FH, 88));

        {
            let s = v.as_slice();
            if !s.is_empty() {
                println!("{}", s[0]);
            }
        }
        {
            let m = v.as_mut_slice();
            if !m.is_empty() {
                m[0] = m[0].wrapping_add(_to_u8(SH, 40));
                println!("{}", m[0]);
            }
        }

        v.insert(_to_usize(FH, 56), _to_u8(SH, 56));
        let slice_for_insert = &base_vec[0..base_vec.len().min(1)];
        v.insert_from_slice(_to_usize(SH, 72), slice_for_insert);

        v.dedup();
        v.dedup_by(|a, b| {
            let flag = _to_u8(FH, 104);
            if flag % 7 == 0 {
                panic!("INTENTIONAL PANIC!");
            }
            (*a & 1) == (*b & 1)
        });
        v.dedup_by_key(|x| {
            let r = _to_u8(SH, 105);
            if r % 11 == 0 {
                panic!("INTENTIONAL PANIC!");
            }
            *x >> 3
        });
        v.retain(|e| {
            let z = _to_u8(FH, 106);
            if z % 13 == 0 {
                panic!("INTENTIONAL PANIC!");
            }
            ((*e as u16 + z as u16) % 2) == 0
        });
        v.resize_with(_to_usize(SH, 88), || _to_u8(FH, 108));
        v.resize(_to_usize(FH, 110), _to_u8(SH, 109));

        let r0 = CustomRange {
            a: _to_usize(FH, 112),
            b: _to_usize(SH, 112),
            flip: _to_u8(FH, 120),
        };
        {
            let mut d0 = v.drain(r0);
            let step0 = _to_u8(SH, 120) % 4;
            match step0 {
                0 => { let _ = d0.next(); }
                1 => { let _ = d0.next_back(); }
                2 => {
                    let times = (_to_u8(FH, 121) % 8) as usize;
                    for _ in 0..times {
                        let _ = d0.next();
                    }
                }
                _ => { d0.count(); }
            }
        }

        v.push(_to_u8(FH, 90));
        let _ = v.pop();
        let _ = v.swap_remove(_to_usize(SH, 136));
        let _ = v.remove(_to_usize(FH, 128));
        v.truncate(_to_usize(FH, 130));
        v.shrink_to_fit();

        let mut other = SmallVec::<[u8; 32]>::from_slice(&base_vec);
        v.append(&mut other);

        let _ = v.partial_cmp(&SmallVec::<[u8; 32]>::from_slice(&base_vec));
        let _ = v.cmp(&SmallVec::<[u8; 32]>::from_slice(&base_vec));

        let i0 = _to_usize(FH, 131);
        let _ = v.as_ref();
        let _ = v.as_mut();
        let _ = v.deref();
        let _ = v.deref_mut();
        let _ptr = v.as_ptr();
        let _mptr = v.as_mut_ptr();

        let op_count = (_to_u8(FH, 129) % 10) as usize;
        for k in 0..op_count {
            match (_to_u8(FH, 3 + (k % base_len).min(base_len.saturating_sub(1))) % 8) {
                0 => v.push(_to_u8(SH, 56)),
                1 => { let _ = v.pop(); }
                2 => v.insert(_to_usize(FH, 132), _to_u8(SH, 40)),
                3 => {
                    let r = CustomRange { a: _to_usize(FH, 133), b: _to_usize(SH, 112), flip: _to_u8(FH, 120) };
                    let mut dr = v.drain(r);
                    if k % 2 == 0 { let _ = dr.next(); } else { dr.count(); }
                }
                4 => v.extend_from_slice(&base_vec),
                5 => v.clear(),
                6 => v.reserve(_to_usize(SH, 24)),
                _ => v.truncate(_to_usize(FH, 134)),
            }
        }

        if v.len() > 0 {
            println!("{}", v[i0]);
        }

        let vec_after = v.clone().into_vec();
        let mut v2 = SmallVec::<[u8; 32]>::from_vec(vec_after);
        let r_final = CustomRange { a: _to_usize(FH, 112), b: _to_usize(SH, 136), flip: _to_u8(FH, 120) };
        let mut df = v2.drain(r_final);
        let _ = df.size_hint();
        let _ = df.next_back();
        df.count();
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