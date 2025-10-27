#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::SmallVec;
use global_data::*;
use std::ops::Deref;

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 3072 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let mut offset = 0;

        let mut sv = match _to_u8(global_data.first_half, offset) % 5 {
            0 => SmallVec::<[usize; 16]>::new(),
            1 => SmallVec::with_capacity(_to_usize(global_data.first_half, offset + 1) % 256),
            2 => SmallVec::from_vec(vec![
                _to_usize(global_data.second_half, 32),
                _to_usize(global_data.second_half, 40),
                _to_usize(global_data.second_half, 48)
            ]),
            3 => SmallVec::from_slice(&[
                _to_usize(global_data.second_half, 56),
                _to_usize(global_data.second_half, 64)
            ]),
            4 => SmallVec::from_elem(_to_usize(global_data.second_half, 72), 4),
            _ => unreachable!()
        };
        offset += 8;

        for _ in 0.._to_usize(global_data.first_half, offset) % 8 {
            match _to_u8(global_data.first_half, offset + 1) % 7 {
                0 => sv.push(_to_usize(global_data.second_half, offset + 2)),
                1 => { sv.pop(); },
                2 => sv.insert(
                    _to_usize(global_data.first_half, offset + 3),
                    _to_usize(global_data.second_half, offset + 11)
                ),
                3 => sv.truncate(_to_usize(global_data.first_half, offset + 19)),
                4 => sv.reserve(_to_usize(global_data.second_half, offset + 27)),
                5 => sv.extend_from_slice(&[
                    _to_usize(global_data.first_half, offset + 35),
                    _to_usize(global_data.first_half, offset + 43)
                ]),
                6 => { let _ = sv.as_mut_slice().get_mut(0).map(|v| *v += 1); },
                _ => unreachable!()
            };
            offset += 4;
        }

        let drained: Vec<_> = sv.drain().collect();
        println!("{:?}", drained);

        let comparison = sv.cmp(&SmallVec::from_slice(&[
            _to_usize(global_data.first_half, 2048),
            _to_usize(global_data.first_half, 2056)
        ]));
        
        let _ordering = comparison;
        let _ = sv.as_slice().get(0).map(|v| println!("{}", v));
        
        let mut sv2 = SmallVec::from_iter([_to_usize(global_data.second_half, 3072), _to_usize(global_data.second_half, 3080)]);
        sv2.clone_from(&sv);
        
        let slice_ref = sv.deref();
        let _ = slice_ref.first().map(|v| println!("{}", v));
        
        sv.resize(
            _to_usize(global_data.first_half, 4096),
            _to_usize(global_data.second_half, 4104)
        );
        
        let partial_cmp = sv2.partial_cmp(&sv);
        println!("{:?}", partial_cmp);
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