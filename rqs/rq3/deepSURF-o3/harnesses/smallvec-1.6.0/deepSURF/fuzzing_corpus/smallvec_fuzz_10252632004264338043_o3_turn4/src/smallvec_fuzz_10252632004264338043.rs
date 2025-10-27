#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::borrow::{Borrow, BorrowMut};
use std::ops::{Deref, DerefMut, Index, IndexMut};
use std::char;

type Arr = [u64; 8];
type SVec = SmallVec<Arr>;

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 1024 {
            return;
        }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let slice_len = (_to_u8(GLOBAL_DATA, 0) % 65) as usize;
        let mut values = Vec::with_capacity(slice_len);
        for i in 0..slice_len {
            let idx = 1 + (i * 8) % (GLOBAL_DATA.len() - 8);
            values.push(_to_u64(GLOBAL_DATA, idx));
        }

        let selector = _to_u8(GLOBAL_DATA, 90) % 5;
        let mut sv: SVec = match selector {
            0 => SmallVec::new(),
            1 => SmallVec::with_capacity(slice_len),
            2 => SmallVec::from_slice(&values),
            3 => SmallVec::from_vec(values.clone()),
            _ => SmallVec::from_elem(_to_u64(GLOBAL_DATA, 98), slice_len),
        };

        let before: &[u64] = sv.borrow();
        println!("{:?}", before.deref());

        let op_count = (_to_u8(GLOBAL_DATA, 106) % 20) as usize;
        for i in 0..op_count {
            let op_sel = _to_u8(GLOBAL_DATA, 107 + i) % 12;
            match op_sel {
                0 => {
                    let idx = (120 + i * 8) % (GLOBAL_DATA.len() - 8);
                    sv.push(_to_u64(GLOBAL_DATA, idx));
                }
                1 => {
                    sv.pop();
                }
                2 => {
                    let idx = (260 + i * 8) % (GLOBAL_DATA.len() - 8);
                    sv.reserve(_to_usize(GLOBAL_DATA, idx));
                }
                3 => {
                    let idx_val = (400 + i * 8) % (GLOBAL_DATA.len() - 8);
                    let val_idx = (500 + i * 8) % (GLOBAL_DATA.len() - 8);
                    let index = _to_usize(GLOBAL_DATA, idx_val) % (sv.len() + 1);
                    sv.insert(index, _to_u64(GLOBAL_DATA, val_idx));
                }
                4 => {
                    if !sv.is_empty() {
                        let idx = _to_usize(GLOBAL_DATA, (600 + i * 4) % (GLOBAL_DATA.len() - 8))
                            % sv.len();
                        sv.remove(idx);
                    }
                }
                5 => {
                    let len = _to_usize(GLOBAL_DATA, (700 + i * 4) % (GLOBAL_DATA.len() - 8)) % 65;
                    sv.truncate(len);
                }
                6 => {
                    if !sv.is_empty() {
                        let idx =
                            _to_usize(GLOBAL_DATA, (800 + i * 4) % (GLOBAL_DATA.len() - 8)) % sv.len();
                        sv.swap_remove(idx);
                    }
                }
                7 => {
                    sv.dedup();
                }
                8 => {
                    let val = _to_u64(GLOBAL_DATA, (900 + i * 8) % (GLOBAL_DATA.len() - 8));
                    let extra = [val];
                    sv.extend_from_slice(&extra);
                }
                9 => {
                    let len = _to_usize(GLOBAL_DATA, (1000 + i * 4) % (GLOBAL_DATA.len() - 8)) % 65;
                    let val = _to_u64(GLOBAL_DATA, (1100 + i * 8) % (GLOBAL_DATA.len() - 8));
                    sv.resize(len, val);
                }
                10 => {
                    let _ = sv.capacity();
                }
                _ => {
                    if sv.is_empty() {
                        let val = _to_u64(GLOBAL_DATA, (1200 + i * 8) % (GLOBAL_DATA.len() - 8));
                        sv.push(val);
                    } else {
                        sv.clear();
                    }
                }
            }

            if !sv.is_empty() {
                let slice_mut: &mut [u64] = sv.borrow_mut();
                slice_mut[0] = slice_mut[0].wrapping_add(1);
                println!("{:?}", slice_mut.deref());
            }
        }

        let after: &[u64] = sv.borrow();
        println!("{:?}", after.deref());

        let sv2: SVec = (&values[..]).to_smallvec();
        let cmp_result = sv.cmp(&sv2);
        println!("{:?}", cmp_result);
        let _ = sv.partial_cmp(&sv2);

        let sv_slice = sv.as_slice();
        println!("{:?}", sv_slice.deref());
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