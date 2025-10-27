#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};
use std::borrow::{Borrow, BorrowMut};

fn build_smallvec_u8(first: &[u8], second: &[u8], arr16: [u8; 16], vec_u8: &Vec<u8>) -> SmallVec<[u8; 16]> {
    let choice = _to_u8(first, 20);
    match choice % 7 {
        0 => SmallVec::<[u8; 16]>::new(),
        1 => SmallVec::<[u8; 16]>::with_capacity(_to_usize(first, 21)),
        2 => SmallVec::<[u8; 16]>::from_buf(arr16),
        3 => SmallVec::<[u8; 16]>::from_buf_and_len(arr16, _to_usize(first, 29)),
        4 => SmallVec::<[u8; 16]>::from_vec(vec_u8.clone()),
        5 => SmallVec::<[u8; 16]>::from_slice(&vec_u8[..]),
        _ => {
            let tmp: SmallVec<[u8; 32]> = (&vec_u8[..]).to_smallvec();
            SmallVec::<[u8; 16]>::from_vec(tmp.into_vec())
        }
    }
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 420 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let first = global_data.first_half;
        let second = global_data.second_half;

        let mut arr16 = [0u8; 16];
        let mut i = 0usize;
        while i < 16 {
            arr16[i] = _to_u8(first, i);
            i += 1;
        }

        let len_v = (_to_u8(second, 1) as usize) % 65;
        let mut vec_u8 = Vec::with_capacity(len_v);
        let mut j = 0usize;
        while j < len_v {
            vec_u8.push(_to_u8(second, 2 + j));
            j += 1;
        }

        let mut sv = build_smallvec_u8(first, second, arr16, &vec_u8);

        let push_count = (_to_u8(first, 46) as usize) % 32;
        let mut k = 0usize;
        while k < push_count {
            sv.push(_to_u8(second, 40 + k));
            k += 1;
        }

        let s1 = sv.as_slice();
        if !s1.is_empty() {
            println!("{}", s1[0]);
        }

        sv.reserve(_to_usize(first, 60));
        let _ = sv.try_reserve(_to_usize(first, 68));
        sv.reserve_exact(_to_usize(first, 76));
        let _ = sv.try_reserve_exact(_to_usize(first, 84));
        sv.grow(_to_usize(first, 92));
        let _cap = sv.capacity();
        println!("{}", _cap);

        if sv.len() > 0 {
            let r = &sv[0];
            println!("{}", *r);
        }

        {
            let m = sv.as_mut_slice();
            if !m.is_empty() {
                m[0] = m[0].wrapping_add(1);
                println!("{}", m[0]);
            }
        }

        {
            let b: &[u8] = sv.borrow();
            if !b.is_empty() {
                println!("{}", b[0]);
            }
            let bm: &mut [u8] = sv.borrow_mut();
            if !bm.is_empty() {
                bm[0] = bm[0].wrapping_add(1);
                println!("{}", bm[0]);
            }
        }

        let dref = sv.deref();
        println!("{}", dref.len());
        let drefm = sv.deref_mut();
        if !drefm.is_empty() {
            drefm[0] = drefm[0].wrapping_add(1);
        }

        let idx1 = _to_usize(first, 100);
        let ret1 = sv.swap_remove(idx1);
        println!("{}", ret1);

        sv.insert(_to_usize(second, 12), _to_u8(second, 13));
        let rem = sv.remove(_to_usize(first, 108));
        println!("{}", rem);

        sv.insert_many(_to_usize(second, 20), vec_u8.clone());

        sv.resize_with(_to_usize(first, 116), || {
            let gd = get_global_data();
            let b = _to_u8(gd.first_half, 0);
            if b % 2 == 0 {
                panic!("INTENTIONAL PANIC!");
            }
            _to_u8(gd.second_half, 1)
        });

        sv.retain(|v| {
            let gd = get_global_data();
            let t = _to_u8(gd.first_half, 2);
            if t % 3 == 0 {
                panic!("INTENTIONAL PANIC!");
            }
            (*v % 2) == (t % 2)
        });

        sv.dedup();

        sv.dedup_by(|a, b| {
            let gd = get_global_data();
            let x = _to_u8(gd.first_half, 3);
            if x % 5 == 0 {
                panic!("INTENTIONAL PANIC!");
            }
            *a == *b
        });

        sv.truncate(_to_usize(second, 28));

        let mut dr = sv.drain(0.._to_usize(second, 36));
        if let Some(xn) = dr.next() {
            println!("{}", xn);
        }
        if let Some(xb) = dr.next_back() {
            println!("{}", xb);
        }
        drop(dr);

        let mut sv2 = SmallVec::<[u8; 24]>::from_elem(_to_u8(second, 3), _to_usize(first, 124));
        sv.append(&mut sv2);

        let v_mid = sv.clone().into_vec();
        let mut sv3 = SmallVec::<[u8; 16]>::from_vec(v_mid);
        let idx2 = _to_usize(second, 44);
        let r2 = sv3.swap_remove(idx2);
        println!("{}", r2);

        let _ = sv.partial_cmp(&sv3);
        let _ = sv.cmp(&sv3);

        if sv.len() > 0 {
            sv[0] = sv[0].wrapping_add(1);
            println!("{}", sv[0]);
        }

        let mut it = sv3.clone().into_iter();
        let sl = it.as_slice();
        println!("{}", sl.len());
        let slm = it.as_mut_slice();
        if !slm.is_empty() {
            slm[0] = slm[0].wrapping_add(1);
            println!("{}", slm[0]);
        }
        if let Some(n1) = it.next() {
            println!("{}", n1);
        }
        if let Some(n2) = it.next_back() {
            println!("{}", n2);
        }

        let ops = (_to_u8(first, 150) as usize) % 10;
        let mut opi = 0usize;
        while opi < ops {
            match _to_u8(second, 150 + opi) % 8 {
                0 => sv.push(_to_u8(first, 152 + opi)),
                1 => {
                    let _ = sv.pop();
                }
                2 => sv.reserve(_to_usize(first, 156)),
                3 => sv.insert(_to_usize(second, 160), _to_u8(second, 161)),
                4 => {
                    let _ = sv.swap_remove(_to_usize(first, 164));
                }
                5 => {
                    let _ = sv.remove(_to_usize(second, 168));
                }
                6 => sv.truncate(_to_usize(first, 172)),
                _ => sv.clear(),
            }
            opi += 1;
        }
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