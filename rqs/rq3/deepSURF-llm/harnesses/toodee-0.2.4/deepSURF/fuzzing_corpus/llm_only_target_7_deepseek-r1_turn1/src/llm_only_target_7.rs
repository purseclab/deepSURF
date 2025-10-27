#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;    
use toodee::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 105 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let t_0 = _to_u8(GLOBAL_DATA, 0);
        let t_5 = _to_usize(GLOBAL_DATA, 31);
        let t_11 = _to_isize(GLOBAL_DATA, 70) as usize;
        let t_14 = _to_char(GLOBAL_DATA, 90);
        let t_15 = _to_bool(GLOBAL_DATA, 94);

        let src_cols = t_5 % 65;
        let src_rows = t_11 % 65;
        let mut src = if t_15 {
            TooDee::init(src_cols, src_rows, t_0)
        } else {
            let data = (0..src_cols*src_rows).map(|i| GLOBAL_DATA[i % GLOBAL_DATA.len()]).collect();
            TooDee::from_vec(src_cols, src_rows, data)
        };

        let dest_cols = _to_usize(GLOBAL_DATA, 15) % 65;
        let dest_rows = _to_usize(GLOBAL_DATA, 23) % 65;
        let mut dest = if t_14.is_alphabetic() {
            TooDee::new(dest_cols, dest_rows)
        } else {
            let data = (0..dest_cols*dest_rows).map(|i| GLOBAL_DATA[(i+50) % GLOBAL_DATA.len()]).collect();
            TooDee::from_vec(dest_cols, dest_rows, data)
        };

        if src_cols > 0 && src_rows > 0 {
            src.swap_rows(0, (src_rows-1) % src_rows);
        }

        let view = src.view((0,0), (src.num_cols(), src.num_rows()));
        dest.copy_from_toodee(&view);

        if dest.num_cols() > 1 {
            dest.flip_cols();
        }

        let ops = _to_u8(GLOBAL_DATA, 50) % 4;
        match ops {
            0 => { dest.push_row(vec![t_0; dest.num_cols()]); },
            1 => { dest.swap_rows(0, dest.num_rows().saturating_sub(1)); },
            2 => { let _ = dest.pop_row(); },
            _ => { dest.fill(&t_0); }
        }

        let mut view_mut = dest.view_mut((0,0), (dest.num_cols(), dest.num_rows()));
        view_mut.copy_from_toodee(&src);

        println!("{:?}", dest[(0,0)]);
        for r in 0..dest.num_rows().min(5) {
            let _ = &dest[r];
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