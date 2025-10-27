#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::{SmallVec, ToSmallVec};
use global_data::*;
use std::cmp::Ordering;
use std::borrow::BorrowMut;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct CustomType1(usize);

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 2048 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let (first_half, second_half) = (global_data.first_half, global_data.second_half);

        let mut elements = Vec::new();
        let elem_count = _to_usize(first_half, 0) % 65;
        for i in 0..elem_count {
            elements.push(CustomType1(_to_usize(first_half, i * 8)));
        }

        let mut sv = match _to_u8(first_half, 100) % 6 {
            0 => SmallVec::from_slice(&elements),
            1 => SmallVec::from_vec(elements.clone()),
            2 => SmallVec::from_elem(CustomType1(_to_usize(first_half, 200)), _to_usize(first_half, 208) % 65),
            3 => SmallVec::with_capacity(_to_usize(first_half, 300) % 65),
            4 => elements.to_smallvec(),
            _ => SmallVec::from_iter(elements.iter().copied()),
        };

        for idx in 0.._to_usize(second_half, 0) % 8 {
            let op = _to_u8(second_half, idx * 4) % 10;
            match op {
                0 => sv.push(CustomType1(_to_usize(second_half, idx * 8))),
                1 => { sv.pop(); }
                2 => sv.truncate(_to_usize(second_half, idx * 8) % 65),
                3 => sv.insert(_to_usize(second_half, idx * 8), CustomType1(_to_usize(second_half, idx * 8 + 1))),
                4 => { let _ = sv.drain(0..sv.len().min(_to_usize(second_half, idx * 8))); }
                5 => sv.extend_from_slice(&elements),
                6 => { sv.retain(|x: &mut CustomType1| x.0 % (_to_usize(second_half, idx * 8) + 1) == 0); },
                7 => { sv.dedup(); },
                8 => { let _ = sv.try_reserve(_to_usize(second_half, idx * 8)); },
                9 => { sv.resize(_to_usize(second_half, idx * 8), CustomType1(_to_usize(second_half, idx * 8 + 1))); },
                _ => (),
            }
        }

        let slice_view = sv.as_slice();
        println!("Slice len: {}", slice_view.len());
        if !slice_view.is_empty() {
            println!("First elem: {:?}", slice_view[0]);
        }

        let _: &mut [CustomType1] = sv.borrow_mut();

        let mut new_sv = SmallVec::<[CustomType1; 64]>::new();
        new_sv.append(&mut sv);
        new_sv.retain(|x: &mut CustomType1| x.0 % 2 == 0);

        let cap = _to_usize(second_half, 500);
        let _ = sv.try_reserve(cap);
        let _ = sv.partial_cmp(&new_sv);
        sv.shrink_to_fit();

        let mut_deref = new_sv.as_mut_slice();
        if !mut_deref.is_empty() {
            mut_deref[0] = CustomType1(_to_usize(second_half, 600));
        }

        let cloned = sv.clone();
        let _ordering = cloned.cmp(&new_sv);
        let _removed = new_sv.swap_remove(0);
        let _capacity = new_sv.capacity();
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