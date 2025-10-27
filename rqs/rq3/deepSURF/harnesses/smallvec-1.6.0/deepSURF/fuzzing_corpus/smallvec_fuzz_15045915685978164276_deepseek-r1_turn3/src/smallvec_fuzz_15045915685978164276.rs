#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::SmallVec;
use global_data::*;

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 1024 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GD = global_data.first_half;

        let mut sv = match _to_u8(GD, 0) % 4 {
            0 => SmallVec::new(),
            1 => SmallVec::with_capacity(_to_usize(GD, 1) % 65),
            2 => {
                let mut buf = [0u8; 32];
                let len_start = 33;
                for i in 0..32 {
                    buf[i] = _to_u8(GD, 1 + i);
                }
                SmallVec::from_buf_and_len(buf, _to_usize(GD, len_start) % 32)
            }
            _ => SmallVec::from_slice(&[_to_u8(GD,1), _to_u8(GD,2), _to_u8(GD,3)]),
        };

        let ops = _to_u8(GD, 50) % 6;
        for i in 0..ops {
            match _to_u8(GD, (51 + i*2) as usize) % 5 {
                0 => sv.push(_to_u8(GD, (100 + i) as usize)),
                1 => sv.insert(_to_usize(GD, (120 + i*8) as usize), _to_u8(GD, (160 + i) as usize)),
                2 => sv.extend_from_slice(&[_to_u8(GD, (200 + i) as usize), _to_u8(GD, (201 + i) as usize)]),
                3 => { sv.drain(_to_usize(GD, 250).._to_usize(GD, 258)); },
                4 => sv.truncate(_to_usize(GD, 266)),
                _ => ()
            }
        }

        let mut iter = sv.into_iter();
        let _ = iter.as_slice();
        let _ = iter.as_mut_slice();

        let mut accum = vec![];
        while let Some(item) = iter.next() {
            accum.push(item);
        }
        
        let mut new_sv: SmallVec<[u8; 32]> = SmallVec::from_iter(accum);
        new_sv.reserve(_to_usize(GD, 300));
        new_sv.shrink_to_fit();
        println!("{:?}", new_sv.capacity());
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