#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::SmallVec;
use global_data::*;
use std::cmp::Ordering;
use std::ops::Deref;

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 256 {
            return;
        }
        set_global_data(data);
        let global_data = get_global_data();
        let gd = global_data.first_half;

        let mut len1 = _to_u8(gd, 0) as usize % 65;
        if len1 == 0 {
            len1 = 1;
        }
        let mut seed = Vec::with_capacity(len1);
        for i in 0..len1 {
            seed.push(_to_u8(gd, 1 + i));
        }

        let selector = _to_u8(gd, 70);
        let mut sv: SmallVec<[u8; 64]> = match selector % 6 {
            0 => SmallVec::from_slice(&seed),
            1 => SmallVec::from(seed.clone()),
            2 => SmallVec::from_iter(seed.clone()),
            3 => {
                let cap = _to_usize(gd, 90) % 65;
                SmallVec::with_capacity(cap)
            }
            4 => SmallVec::new(),
            _ => {
                let elem = _to_u8(gd, 100);
                let n = (_to_u8(gd, 101) as usize % 10) + 1;
                SmallVec::from_elem(elem, n)
            }
        };

        sv.push(_to_u8(gd, 110));
        let slice_len = _to_u8(gd, 111) as usize % 10;
        let slice_end = 112 + slice_len;
        if slice_end <= gd.len() {
            sv.extend_from_slice(&gd[112..slice_end]);
        }

        let start_raw = _to_u8(gd, 150) as usize;
        let end_raw = _to_u8(gd, 151) as usize;
        let (start, end) = if start_raw <= end_raw {
            (start_raw, end_raw)
        } else {
            (end_raw, start_raw)
        };

        let _cap_before = sv.capacity();
        let _len_before = sv.len();
        let _ = sv.is_empty();

        sv.reserve(_to_usize(gd, 160));

        {
            let mut drain = sv.drain(start..end);

            if _to_bool(gd, 170) {
                let _ = drain.next();
            }
            if _to_bool(gd, 171) {
                let _ = drain.next_back();
            }

            if _to_bool(gd, 172) {
                std::mem::drop(drain);
            } else {
                let _ = &mut drain;
            }
        }

        let slice_ref = sv.as_slice();
        println!("{:?}", slice_ref.deref());

        let _ = sv.len();
        sv.shrink_to_fit();

        let cloned = sv.clone();
        let _ord: Option<Ordering> = sv.partial_cmp(&cloned);

        let mut toggle = true;
        sv.retain(|b| {
            toggle = !toggle;
            if *b == _to_u8(gd, 180) {
                panic!("INTENTIONAL PANIC!");
            }
            toggle
        });

        let boxed = sv.into_boxed_slice();
        println!("{:?}", boxed.len());
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