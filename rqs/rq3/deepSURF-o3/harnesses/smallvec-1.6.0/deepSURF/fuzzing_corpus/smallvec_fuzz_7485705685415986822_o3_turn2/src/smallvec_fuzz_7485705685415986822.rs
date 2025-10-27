#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::ops::{Deref, DerefMut, Index, IndexMut};

#[derive(Debug, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct CustomType1(usize);

impl Clone for CustomType1 {
    fn clone(&self) -> Self {
        let g = get_global_data();
        let v = _to_u8(g.second_half, (self.0 % 128) as usize);
        if v % 3 == 0 {
            panic!("INTENTIONAL PANIC!");
        }
        CustomType1(self.0 ^ v as usize)
    }
}

fn build_smallvec(data: &[u8], mut cur: usize) -> (SmallVec<[CustomType1; 32]>, usize) {
    let choice = _to_u8(data, cur);
    cur += 1;
    let sv = match choice % 7 {
        0 => SmallVec::<[CustomType1; 32]>::new(),
        1 => {
            let cap = _to_usize(data, cur);
            cur += 8;
            SmallVec::<[CustomType1; 32]>::with_capacity(cap)
        }
        2 => {
            let val = _to_u8(data, cur) as usize;
            cur += 1;
            let n = (_to_u8(data, cur) % 65) as usize;
            cur += 1;
            SmallVec::from_elem(CustomType1(val), n)
        }
        3 => {
            let n = (_to_u8(data, cur) % 65) as usize;
            cur += 1;
            let mut tmp = Vec::with_capacity(n);
            for i in 0..n {
                let idx = (cur + i) % data.len();
                tmp.push(CustomType1(_to_u8(data, idx) as usize));
            }
            SmallVec::from_slice(&tmp)
        }
        4 => {
            let n = (_to_u8(data, cur) % 65) as usize;
            cur += 1;
            let it = (0..n).map(|off| {
                let idx = (cur + off) % data.len();
                CustomType1(_to_u8(data, idx) as usize)
            });
            SmallVec::from_iter(it)
        }
        5 => {
            let n = (_to_u8(data, cur) % 65) as usize;
            cur += 1;
            let mut vec = Vec::with_capacity(n);
            for i in 0..n {
                let idx = (cur + i) % data.len();
                vec.push(CustomType1(_to_u8(data, idx) as usize));
            }
            SmallVec::from_vec(vec)
        }
        _ => {
            let val = _to_u8(data, cur) as usize;
            cur += 1;
            let n = (_to_u8(data, cur) % 10 + 1) as usize;
            cur += 1;
            smallvec![CustomType1(val); n]
        }
    };
    (sv, cur)
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 1024 { return; }
        set_global_data(data);
        let mut cursor = 0;

        let (mut vec_a, nxt) = build_smallvec(data, cursor); cursor = nxt;
        let (mut vec_b, nxt) = build_smallvec(data, cursor); cursor = nxt;

        let ops = (_to_u8(data, cursor) % 25) as usize; cursor += 1;

        for i in 0..ops {
            let op = _to_u8(data, (cursor + i) % data.len());
            match op % 12 {
                0 => vec_a.push(CustomType1(op as usize)),
                1 => { vec_a.pop(); },
                2 => {
                    let idx = _to_usize(data, (cursor + i) % (data.len() - 8));
                    vec_a.insert(idx, CustomType1(idx));
                }
                3 => if !vec_a.is_empty() {
                        let idx = _to_usize(data, (cursor + i) % (data.len() - 8)) % vec_a.len();
                        vec_a.remove(idx);
                    },
                4 => if !vec_a.is_empty() {
                        let idx = _to_u8(data, (cursor + i) % data.len()) as usize % vec_a.len();
                        vec_a.swap_remove(idx);
                    },
                5 => vec_a.truncate(_to_u8(data, (cursor + i) % data.len()) as usize),
                6 => vec_a.dedup(),
                7 => vec_a.retain(|it| it.0 % 2 == 0),
                8 => {
                    let extra = [CustomType1(op as usize); 4];
                    vec_a.extend_from_slice(&extra);
                }
                9 => vec_a.extend_from_slice(vec_b.as_slice()),
                10 => { let mut d = vec_a.drain(..); let _ = d.next(); }
                _ => vec_a.resize(op as usize, CustomType1(op as usize)),
            }
        }

        let slice_ref = vec_a.as_slice();
        if !slice_ref.is_empty() {
            let _ = slice_ref[0];
        }

        let _ = vec_a.cmp(&vec_b);

        vec_b.push(CustomType1(vec_a.len()));
        vec_b.dedup();
        let _ = vec_b.pop();

        vec_a.cmp(&vec_b);
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