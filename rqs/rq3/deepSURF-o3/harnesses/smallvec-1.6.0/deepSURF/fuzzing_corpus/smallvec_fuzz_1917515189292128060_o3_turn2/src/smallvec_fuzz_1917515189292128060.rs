#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;

fn build_smallvec() -> SmallVec<[u8; 16]> {
    let global_data = get_global_data();
    let g = global_data.first_half;
    let selector = _to_u8(g, 0) % 5;
    let cap = _to_usize(g, 1);
    let mut buf: [u8; 16] = [0; 16];
    for i in 0..16 {
        buf[i] = _to_u8(g, 9 + i);
    }
    let len_unbounded = _to_usize(g, 25);
    match selector {
        0 => SmallVec::<[u8; 16]>::new(),
        1 => SmallVec::<[u8; 16]>::with_capacity(cap),
        2 => SmallVec::<[u8; 16]>::from_buf_and_len(buf, len_unbounded),
        3 => {
            let slice_len = (_to_u8(g, 33) % 65) as usize;
            let slice = _unwrap_option(g.get(34..34 + slice_len));
            SmallVec::<[u8; 16]>::from_slice(slice)
        }
        _ => {
            let mut v = Vec::new();
            let vec_len = (_to_u8(g, 40) % 65) as usize;
            for i in 0..vec_len {
                v.push(_to_u8(g, 41 + i));
            }
            SmallVec::<[u8; 16]>::from_vec(v)
        }
    }
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 128 {
            return;
        }
        set_global_data(data);
        let global_data = get_global_data();
        let g = global_data.first_half;

        let mut sv1 = build_smallvec();
        let mut sv2 = build_smallvec();

        sv1.push(_to_u8(g, 60));
        let insert_idx = _to_usize(g, 61);
        sv1.insert(insert_idx, _to_u8(g, 69));

        let ext_len = (_to_u8(g, 70) % 65) as usize;
        let ext_slice = _unwrap_option(g.get(71..71 + ext_len));
        sv1.extend_from_slice(ext_slice);

        sv1.reserve(_to_usize(g, 90));

        let _ = sv1.partial_cmp(&sv2);

        let slice_ref = sv1.as_slice();
        if let Some(first) = slice_ref.get(0) {
            println!("{}", *first);
        }

        let start = _to_usize(g, 96);
        let end = _to_usize(g, 100);
        let range = start.min(end)..start.max(end);
        {
            let mut d = sv1.drain(range);
            let _ = d.next();
            let _ = d.next_back();
        }

        let _ = sv1.pop();

        sv1.truncate(_to_usize(g, 104));
        sv1.shrink_to_fit();
        sv1.clear();
        let _ = sv1.into_vec();
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