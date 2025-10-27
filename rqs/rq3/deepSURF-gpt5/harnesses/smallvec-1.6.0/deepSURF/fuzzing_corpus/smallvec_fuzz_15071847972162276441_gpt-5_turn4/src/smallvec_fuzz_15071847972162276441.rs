#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};
use std::borrow::{Borrow, BorrowMut};

fn make_buf32() -> [u8; 32] {
    let gd = get_global_data();
    let fh = gd.first_half;
    let mut buf = [0u8; 32];
    let mut i = 0usize;
    while i < 32 {
        buf[i] = _to_u8(fh, 90 + i);
        i += 1;
    }
    buf
}

fn make_vec_from_second(len_hint_idx: usize) -> Vec<u8> {
    let gd = get_global_data();
    let sh = gd.second_half;
    let n = (_to_u8(sh, len_hint_idx) % 65) as usize;
    let mut v = Vec::with_capacity(n);
    let mut i = 0usize;
    while i < n && i < sh.len() {
        v.push(sh[i]);
        i += 1;
    }
    v
}

fn make_smallvec32() -> SmallVec<[u8; 32]> {
    let gd = get_global_data();
    let fh = gd.first_half;
    let sh = gd.second_half;
    let mode = _to_u8(fh, 80) % 9;
    let buf = make_buf32();
    if mode == 0 {
        SmallVec::<[u8; 32]>::new()
    } else if mode == 1 {
        let cap = _to_usize(fh, 0);
        SmallVec::<[u8; 32]>::with_capacity(cap)
    } else if mode == 2 {
        SmallVec::<[u8; 32]>::from_buf(buf)
    } else if mode == 3 {
        let len = _to_usize(fh, 35);
        SmallVec::<[u8; 32]>::from_buf_and_len(buf, len)
    } else if mode == 4 {
        let start = (_to_u8(sh, 1) as usize) % sh.len();
        let count = (_to_u8(sh, 2) % 65) as usize;
        let end = if start.saturating_add(count) > sh.len() { sh.len() } else { start + count };
        SmallVec::<[u8; 32]>::from_slice(&sh[start..end])
    } else if mode == 5 {
        let v = make_vec_from_second(3);
        SmallVec::<[u8; 32]>::from_vec(v)
    } else if mode == 6 {
        let elem = _to_u8(fh, 10);
        let n = _to_u8(fh, 11) as usize % 65;
        SmallVec::<[u8; 32]>::from_elem(elem, n)
    } else if mode == 7 {
        let count = (_to_u8(fh, 12) % 65) as usize;
        let it = (0..count).map(|i| (i as u8).wrapping_add(_to_u8(fh, 13)));
        SmallVec::<[u8; 32]>::from_iter(it)
    } else {
        let slice = &sh[0..(if sh.len() < 32 { sh.len() } else { 32 })];
        slice.to_smallvec()
    }
}

fn run_ops_on_smallvec(sv: &mut SmallVec<[u8; 32]>, other: &mut SmallVec<[u8; 32]>) {
    let gd = get_global_data();
    let fh = gd.first_half;
    let sh = gd.second_half;
    let mut i = 0usize;
    let ops = (_to_u8(fh, 14) % 20) as usize + 1;
    while i < ops {
        let op = _to_u8(fh, 100 + (i % 20)) % 16;
        if op == 0 {
            let mut it = (&*sv).into_iter();
            if let Some(r) = it.next() {
                println!("{:?}", *r);
            }
            if let Some(r) = it.next_back() {
                println!("{:?}", *r);
            }
        } else if op == 1 {
            let v = _to_u8(sh, (i % (sh.len() as usize)) as usize);
            sv.push(v);
        } else if op == 2 {
            let idx = _to_usize(fh, 15);
            sv.insert(idx, _to_u8(fh, 23));
        } else if op == 3 {
            let idx = _to_usize(fh, 16);
            let _ = sv.pop();
            let _ = sv.remove(idx);
        } else if op == 4 {
            let new_len = _to_usize(sh, 8);
            let val = _to_u8(sh, 9);
            sv.resize(new_len, val);
        } else if op == 5 {
            let add = _to_usize(fh, 17);
            sv.reserve(add);
            let _ = sv.try_reserve(add);
        } else if op == 6 {
            let add = _to_usize(sh, 10);
            sv.reserve_exact(add);
            let _ = sv.try_reserve_exact(add);
        } else if op == 7 {
            let start = (_to_u8(sh, 11) as usize) % sh.len();
            let count = (_to_u8(sh, 12) % 65) as usize;
            let end = if start.saturating_add(count) > sh.len() { sh.len() } else { start + count };
            sv.extend_from_slice(&sh[start..end]);
        } else if op == 8 {
            let _ = sv.try_grow(_to_usize(fh, 18));
            sv.grow(_to_usize(fh, 19));
        } else if op == 9 {
            let _ = sv.len();
            let _ = sv.capacity();
            let _ = sv.is_empty();
            let a = sv.as_slice();
            if !a.is_empty() {
                println!("{:?}", a[0]);
            }
            let b = sv.as_mut_slice();
            if !b.is_empty() {
                b[0] = b[0].wrapping_add(_to_u8(fh, 20));
            }
        } else if op == 10 {
            sv.dedup();
        } else if op == 11 {
            let mut toggler = _to_u8(fh, 21);
            sv.retain(|x| {
                toggler = toggler.wrapping_add(*x);
                if toggler % 7 == 0 {
                    panic!("INTENTIONAL PANIC!");
                }
                (toggler & 1) == 0
            });
        } else if op == 12 {
            let mut gate = _to_u8(fh, 22);
            sv.dedup_by(|a, b| {
                gate = gate.wrapping_add(a.wrapping_add(*b));
                if gate % 5 == 0 {
                    panic!("INTENTIONAL PANIC!");
                }
                *a == *b
            });
        } else if op == 13 {
            sv.dedup_by_key(|x| {
                let k = x.wrapping_mul(3);
                k
            });
        } else if op == 14 {
            let idx = _to_usize(sh, 13);
            let count = _to_usize(sh, 14);
            let mut d = sv.drain(idx..count);
            let _ = d.next();
            let _ = d.next_back();
        } else {
            sv.append(other);
        }
        i += 1;
    }
    let _ = (&*sv).partial_cmp(&*other);
    let _ = (&*sv).cmp(&*other);
    let _ = (&*sv).eq(&*other);
    if sv.len() > 0 {
        let _ = sv.index(0);
        let _ = { sv[0] = sv[0].wrapping_add(1); };
    }
    let r: &[u8] = (&*sv).borrow();
    if !r.is_empty() {
        println!("{:?}", r[0]);
    }
    let m: &mut [u8] = sv.borrow_mut();
    if !m.is_empty() {
        m[0] = m[0].wrapping_add(2);
    }
    let d = sv.deref();
    if !d.is_empty() {
        println!("{:?}", d[0]);
    }
    let dm = sv.deref_mut();
    if !dm.is_empty() {
        dm[0] = dm[0].wrapping_add(3);
    }
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 260 { return; }
        set_global_data(data);
        let mut sv = make_smallvec32();
        let mut other = make_smallvec32();
        run_ops_on_smallvec(&mut sv, &mut other);
        let mut iter = (&sv).into_iter();
        if let Some(r) = iter.next() {
            println!("{:?}", *r);
        }
        if let Some(r) = iter.next_back() {
            println!("{:?}", *r);
        }
        let mut owned_iter = sv.clone().into_iter();
        let s = owned_iter.as_slice();
        if !s.is_empty() {
            println!("{:?}", s[0]);
        }
        let m = owned_iter.as_mut_slice();
        if !m.is_empty() {
            m[0] = m[0].wrapping_add(4);
        }
        let _ = owned_iter.next();
        let _ = owned_iter.next_back();
        let v = sv.clone().into_vec();
        let _sv2 = SmallVec::<[u8; 32]>::from_vec(v);
        let _ = sv.into_boxed_slice();
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