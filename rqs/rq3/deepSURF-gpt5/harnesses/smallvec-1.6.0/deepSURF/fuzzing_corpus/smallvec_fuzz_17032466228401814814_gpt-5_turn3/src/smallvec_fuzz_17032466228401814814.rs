#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

#[derive(Clone, Debug)]
struct CustomItem(String);

impl core::cmp::PartialEq for CustomItem {
    fn eq(&self, _: &Self) -> bool {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 0);
        let custom_impl_inst_num = self.0.len();
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0 {
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector {
            1 => global_data.first_half,
            _ => global_data.second_half,
        };
        let t_0 = _to_bool(GLOBAL_DATA, 8);
        t_0
    }
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 512 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let fh = global_data.first_half;
        let sh = global_data.second_half;

        let lfh = fh.len();
        let lsh = sh.len();

        let s0 = (_to_u8(fh, 2) as usize) % (lfh - 1);
        let e0 = s0 + ((_to_u8(fh, 3) as usize) % (lfh - s0));
        let base_str0 = _to_str(fh, s0, e0);
        let item0 = CustomItem(String::from(base_str0));

        let s1 = (_to_u8(sh, 2) as usize) % (lsh - 1);
        let e1 = s1 + ((_to_u8(sh, 3) as usize) % (lsh - s1));
        let base_str1 = _to_str(sh, s1, e1);
        let item1 = CustomItem(String::from(base_str1));

        let mut vec_items: Vec<CustomItem> = Vec::new();
        let n_items = (_to_u8(fh, 4) % 65) as usize;
        for i in 0..n_items {
            if i % 2 == 0 {
                vec_items.push(item0.clone());
            } else {
                vec_items.push(item1.clone());
            }
        }

        let mut arr: [CustomItem; 16] = std::array::from_fn(|_| item0.clone());
        arr[0] = item1.clone();
        arr[1] = item0.clone();

        let sel_ctor = _to_u8(fh, 5) % 8;
        let mut sv: SmallVec<[CustomItem; 16]> = match sel_ctor {
            0 => SmallVec::new(),
            1 => SmallVec::with_capacity(_to_usize(fh, 6)),
            2 => SmallVec::from_buf(arr.clone()),
            3 => SmallVec::from_buf_and_len(arr.clone(), _to_usize(fh, 10)),
            4 => SmallVec::from_elem(item0.clone(), _to_usize(fh, 14)),
            5 => SmallVec::from_vec(vec_items.clone()),
            6 => SmallVec::from_iter(vec_items.iter().cloned()),
            _ => SmallVec::from(arr.clone()),
        };

        sv.push(item1.clone());
        let _ = sv.try_reserve(_to_usize(fh, 30));
        sv.reserve(_to_usize(fh, 38));
        sv.reserve_exact(_to_usize(fh, 46));
        let _ = sv.try_reserve_exact(_to_usize(fh, 54));
        sv.grow(_to_usize(fh, 62));
        let _ = sv.try_grow(_to_usize(fh, 70));

        if !sv.is_empty() {
            let r = &sv[0];
            println!("{:?}", r);
        }
        let sref = sv.as_slice();
        println!("{:?}", sref);
        let smut = sv.as_mut_slice();
        println!("{:?}", smut);

        let mut sv2 = sv.clone();

        sv.insert(_to_usize(sh, 0), item0.clone());
        let mut vec2: Vec<CustomItem> = Vec::new();
        let n2 = (_to_u8(sh, 1) % 65) as usize;
        for i in 0..n2 {
            if i % 3 == 0 {
                vec2.push(item1.clone());
            } else {
                vec2.push(item0.clone());
            }
        }
        if !vec2.is_empty() {
            sv.extend(vec2.clone());
            sv.insert_many(_to_usize(sh, 2), vec2.clone());
        }
        sv.insert_many(_to_usize(sh, 4), vec_items.clone());

        sv.retain(|_x| {
            let gd = get_global_data();
            let gsel = if _to_bool(gd.first_half, 9) { gd.first_half } else { gd.second_half };
            if _to_bool(gsel, 8) {
                true
            } else {
                panic!("INTENTIONAL PANIC!");
            }
        });

        sv.resize_with(_to_usize(sh, 10), || item1.clone());

        sv.dedup();

        sv.dedup_by(|a, b| {
            let gd = get_global_data();
            let gsel = if _to_bool(gd.first_half, 11) { gd.first_half } else { gd.second_half };
            if _to_bool(gsel, 12) {
                a == b
            } else {
                panic!("INTENTIONAL PANIC!");
            }
        });

        sv.dedup_by_key(|x| {
            let m = _to_usize(fh, 90).wrapping_add(1);
            x.0.len() % m
        });

        let end_range = _to_usize(fh, 100);
        {
            let mut d = sv.drain(0..end_range);
            let v1 = d.next();
            println!("{:?}", v1);
            let v2 = d.next_back();
            println!("{:?}", v2);
        }

        let mut other = SmallVec::<[CustomItem; 16]>::from_elem(item0.clone(), _to_usize(sh, 12));
        sv.append(&mut other);

        let p = sv.pop();
        println!("{:?}", p);

        sv.truncate(_to_usize(sh, 14));
        sv.shrink_to_fit();

        sv2.push(item0.clone());
        sv2.swap_remove(_to_usize(fh, 104));
        let _ = sv2.capacity();
        let _ = sv2.len();
        let _ = sv2.is_empty();
        let _ = sv2.spilled();
        println!("{:?}", sv2.as_slice());

        let _ = sv.as_ptr();

        let mut ops = (_to_u8(fh, 20) % 16) as usize;
        while ops > 0 {
            let which = _to_u8(sh, 22 + ops) % 10;
            match which {
                0 => { sv.push(item1.clone()); }
                1 => { sv.insert(_to_usize(sh, 24 + ops), item0.clone()); }
                2 => { let _ = sv.remove(_to_usize(fh, 26 + ops)); }
                3 => { let _ = sv.swap_remove(_to_usize(fh, 28 + ops)); }
                4 => {
                    let s = sv.as_slice();
                    println!("{:?}", s);
                }
                5 => {
                    let s = sv.as_mut_slice();
                    println!("{:?}", s);
                }
                6 => {
                    sv.dedup();
                }
                7 => {
                    sv.reserve(_to_usize(fh, 30 + ops));
                }
                8 => {
                    let mut it = sv.clone().into_iter();
                    let _ = it.next();
                    let _ = it.next_back();
                }
                _ => {
                    sv.retain(|_| true);
                }
            }
            ops -= 1;
        }

        sv.dedup();

        let bx = sv2.clone().into_boxed_slice();
        let blen = (*bx).len();
        println!("{}", blen);
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