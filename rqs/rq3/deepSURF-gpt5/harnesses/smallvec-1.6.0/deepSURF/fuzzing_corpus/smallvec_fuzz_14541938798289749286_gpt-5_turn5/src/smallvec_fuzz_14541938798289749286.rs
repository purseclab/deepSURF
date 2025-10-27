#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};
use std::borrow::{Borrow, BorrowMut};

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 200 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let GLOBAL_DATA2 = global_data.second_half;

        let mut arr_u8_16 = [0u8; 16];
        for i in 0..16 {
            arr_u8_16[i] = _to_u8(GLOBAL_DATA, i);
        }
        let mut arr_i32_12 = [0i32; 12];
        for j in 0..12 {
            arr_i32_12[j] = _to_i32(GLOBAL_DATA2, j * 4);
        }

        let choice = _to_u8(GLOBAL_DATA, 32);
        let slice_len_a = (_to_u8(GLOBAL_DATA, 33) as usize) % 16;
        let mut v_u8_a: SmallVec<[u8; 16]> = match choice % 4 {
            0 => SmallVec::<[u8; 16]>::from_buf(arr_u8_16),
            1 => SmallVec::<[u8; 16]>::from_slice(&arr_u8_16[..slice_len_a]),
            2 => {
                let mut v = SmallVec::<[u8; 16]>::new();
                let c = (_to_u8(GLOBAL_DATA, 34) as usize) % 16;
                for k in 0..c {
                    v.push(arr_u8_16[k]);
                }
                v
            }
            _ => {
                let mut vbase = Vec::new();
                let vlen = (_to_u8(GLOBAL_DATA, 35) as usize) % 16;
                for k in 0..vlen {
                    vbase.push(arr_u8_16[k]);
                }
                SmallVec::<[u8; 16]>::from_vec(vbase)
            }
        };

        let elem_i32 = _to_i32(GLOBAL_DATA2, 60);
        let n_i32 = _to_usize(GLOBAL_DATA2, 64) % 65;
        let mut v_i32_b: SmallVec<[i32; 12]> = SmallVec::from_elem(elem_i32, n_i32);

        let cap_c = _to_usize(GLOBAL_DATA, 40);
        let mut v_u8_c: SmallVec<[u8; 16]> = SmallVec::with_capacity(cap_c);
        let ext_len_c = (_to_u8(GLOBAL_DATA, 41) as usize) % 16;
        v_u8_c.extend_from_slice(&arr_u8_16[..ext_len_c]);

        let slice_len_d = (_to_u8(GLOBAL_DATA, 42) as usize) % 16;
        let slice_d = &arr_u8_16[..slice_len_d];
        let mut v_u8_d: SmallVec<[u8; 16]> = slice_d.to_smallvec();

        let _ = (&v_u8_a).is_empty();
        let _ = (&v_u8_c).is_empty();
        let _ = (&v_u8_d).is_empty();
        let _ = (&v_i32_b).is_empty();

        let s_ref = v_u8_a.as_slice();
        if s_ref.len() > 0 {
            println!("{}", s_ref[0]);
        }
        let s_mut = v_u8_a.as_mut_slice();
        if s_mut.len() > 0 {
            s_mut[0] = s_mut[0].wrapping_add(1);
            println!("{}", s_mut[0]);
        }

        let u_sz1 = _to_usize(GLOBAL_DATA, 48);
        let u_sz2 = _to_usize(GLOBAL_DATA, 56);
        let u_sz3 = _to_usize(GLOBAL_DATA2, 24);
        let u_sz4 = _to_usize(GLOBAL_DATA2, 32);

        let ops = (_to_u8(GLOBAL_DATA, 52) % 20) as usize + 1;
        for i in 0..ops {
            let which = _to_u8(GLOBAL_DATA, (53 + i) % 80) % 3;
            let op = _to_u8(GLOBAL_DATA2, (10 + i) % 80) % 12;
            if which == 0 {
                let v = &mut v_u8_a;
                match op {
                    0 => {
                        let idx = u_sz1;
                        let val = _to_u8(GLOBAL_DATA2, 8);
                        v.insert(idx, val);
                    }
                    1 => {
                        let idx = u_sz2;
                        let _ = v.remove(idx);
                    }
                    2 => {
                        let val = _to_u8(GLOBAL_DATA2, 12);
                        v.push(val);
                    }
                    3 => {
                        let _ = v.pop();
                    }
                    4 => {
                        let add = u_sz3;
                        v.reserve(add);
                    }
                    5 => {
                        let nlen = u_sz4;
                        let val = _to_u8(GLOBAL_DATA2, 16);
                        v.resize(nlen, val);
                    }
                    6 => {
                        let nlen2 = _to_usize(GLOBAL_DATA2, 40);
                        let mut t = _to_u8(GLOBAL_DATA, 44);
                        v.resize_with(nlen2, || {
                            t = t.wrapping_add(1);
                            if t % 7 == 0 { panic!("INTENTIONAL PANIC!"); }
                            _to_u8(GLOBAL_DATA, (t as usize) % 60)
                        });
                    }
                    7 => {
                        let idxm = _to_usize(GLOBAL_DATA2, 48);
                        let _ = v.swap_remove(idxm);
                    }
                    8 => {
                        let idxi = _to_usize(GLOBAL_DATA2, 56);
                        let k = (_to_u8(GLOBAL_DATA, 45) as usize) % 16;
                        v.insert_from_slice(idxi, &arr_u8_16[..k]);
                    }
                    9 => {
                        let dend = _to_usize(GLOBAL_DATA2, 64);
                        let mut dr = v.drain(0..dend);
                        let _ = dr.next();
                        let _ = dr.next_back();
                    }
                    10 => {
                        let _ = v.capacity();
                        let _ = v.len();
                    }
                    _ => {
                        v.clear();
                    }
                }
                let _ = (&*v).is_empty();
            } else if which == 1 {
                let v = &mut v_u8_c;
                match op {
                    0 => {
                        let other = &mut v_u8_d;
                        v.append(other);
                    }
                    1 => {
                        let add = _to_usize(GLOBAL_DATA2, 72);
                        v.reserve_exact(add);
                    }
                    2 => {
                        let tr = _to_usize(GLOBAL_DATA, 64);
                        v.truncate(tr);
                    }
                    3 => {
                        let _ = v.as_ptr();
                        let _ = v.as_mut_ptr();
                    }
                    4 => {
                        let a = v.as_slice();
                        println!("{}", a.len());
                    }
                    5 => {
                        let b = v.as_mut_slice();
                        if b.len() > 0 {
                            b[0] = b[0].wrapping_add(2);
                            println!("{}", b[0]);
                        }
                    }
                    6 => {
                        let _ = v.try_reserve(_to_usize(GLOBAL_DATA, 72));
                    }
                    7 => {
                        let _ = v.try_reserve_exact(_to_usize(GLOBAL_DATA2, 8));
                    }
                    8 => {
                        v.shrink_to_fit();
                    }
                    9 => {
                        v.dedup();
                    }
                    10 => {
                        let mut flip = _to_bool(GLOBAL_DATA2, 20);
                        v.retain(|e| {
                            flip = !flip;
                            if flip { println!("{}", *e); }
                            flip
                        });
                    }
                    _ => {
                        let mut tick = _to_u8(GLOBAL_DATA2, 21);
                        v.dedup_by(|x, y| {
                            tick = tick.wrapping_add(1);
                            if tick % 9 == 0 { panic!("INTENTIONAL PANIC!"); }
                            *x == *y
                        });
                    }
                }
                let _ = (&*v).is_empty();
            } else {
                let v = &mut v_u8_d;
                match op {
                    0 => {
                        let idx = _to_usize(GLOBAL_DATA, 72);
                        let val = _to_u8(GLOBAL_DATA2, 28);
                        v.insert(idx, val);
                    }
                    1 => {
                        let idx = _to_usize(GLOBAL_DATA2, 36);
                        let _ = v.remove(idx);
                    }
                    2 => {
                        let b: &[u8] = (*v).borrow();
                        println!("{}", b.len());
                        let bm: &mut [u8] = v.borrow_mut();
                        if bm.len() > 0 {
                            bm[0] = bm[0].wrapping_add(1);
                            println!("{}", bm[0]);
                        }
                    }
                    3 => {
                        let _ = v.grow(_to_usize(GLOBAL_DATA2, 44));
                    }
                    4 => {
                        let _ = v.try_grow(_to_usize(GLOBAL_DATA, 80));
                    }
                    5 => {
                        let idx = _to_usize(GLOBAL_DATA, 8);
                        let r = (&*v).index(idx);
                        println!("{}", *r);
                    }
                    6 => {
                        let _ = v.clone();
                    }
                    7 => {
                        let idx = _to_usize(GLOBAL_DATA, 16);
                        let val = _to_u8(GLOBAL_DATA2, 52);
                        v.insert(idx, val);
                    }
                    8 => {
                        let k = (_to_u8(GLOBAL_DATA2, 53) as usize) % 16;
                        v.extend_from_slice(&arr_u8_16[..k]);
                    }
                    9 => {
                        let k = (_to_u8(GLOBAL_DATA2, 54) as usize) % 16;
                        let slice = &arr_u8_16[..k];
                        let _: SmallVec<[u8; 16]> = slice.to_smallvec();
                    }
                    10 => {
                        let _ = v.clone().into_vec();
                    }
                    _ => {
                        let _ = v.clone().into_boxed_slice();
                    }
                }
                let _ = (&*v).is_empty();
            }
        }

        let _ = v_u8_a.eq(&v_u8_c);
        let _ = v_u8_a.partial_cmp(&v_u8_c);
        let _ = v_u8_a.cmp(&v_u8_c);

        let s_d = v_u8_d.deref();
        if s_d.len() > 0 {
            println!("{}", s_d[0]);
        }
        let s_dm = v_u8_d.deref_mut();
        if s_dm.len() > 0 {
            s_dm[0] = s_dm[0].wrapping_add(3);
            println!("{}", s_dm[0]);
        }

        let mut vi = v_u8_a.clone().into_iter();
        let _ = vi.next();
        let _ = vi.next_back();
        let its = vi.as_slice();
        println!("{}", its.len());
        let itm = vi.as_mut_slice();
        if itm.len() > 0 {
            itm[0] = itm[0].wrapping_add(4);
            println!("{}", itm[0]);
        }

        let _ = (&v_i32_b).is_empty();
        let ops_i32 = (_to_u8(GLOBAL_DATA2, 55) % 10) as usize + 1;
        for t in 0..ops_i32 {
            let op = _to_u8(GLOBAL_DATA2, (56 + t) % 80) % 8;
            match op {
                0 => {
                    v_i32_b.push(_to_i32(GLOBAL_DATA2, 12));
                }
                1 => {
                    let idx = _to_usize(GLOBAL_DATA2, 60);
                    let _ = v_i32_b.remove(idx);
                }
                2 => {
                    let idx = _to_usize(GLOBAL_DATA2, 68);
                    let val = _to_i32(GLOBAL_DATA2, 16);
                    v_i32_b.insert(idx, val);
                }
                3 => {
                    let _ = v_i32_b.pop();
                }
                4 => {
                    v_i32_b.truncate(_to_usize(GLOBAL_DATA2, 72));
                }
                5 => {
                    let sl = v_i32_b.as_slice();
                    println!("{}", sl.len());
                }
                6 => {
                    let slm = v_i32_b.as_mut_slice();
                    if slm.len() > 0 {
                        slm[0] = slm[0].wrapping_add(1);
                        println!("{}", slm[0]);
                    }
                }
                _ => {
                    let k = (_to_u8(GLOBAL_DATA2, 18) as usize) % 12;
                    v_i32_b.extend_from_slice(&arr_i32_12[..k]);
                }
            }
            let _ = (&v_i32_b).is_empty();
        }

        let _ = (&v_u8_a).is_empty();
        let _ = (&v_u8_c).is_empty();
        let _ = (&v_u8_d).is_empty();
        let _ = (&v_i32_b).is_empty();
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